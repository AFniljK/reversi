use std::collections::HashMap;

pub mod cli;
pub mod gui;

pub const GENERAL_EDGE: u64 = 35604928818740736;
pub const TOP_EDGE: u64 = 18374686479671623680;
pub const BOTTOM_EDGE: u64 = 255;
pub const RIGHT_EDGE: u64 = 72340172838076673;
pub const LEFT_EDGE: u64 = 9259542123273814144;

pub const RIGHT_TOP_MESH: u64 = 144959613005987840;
pub const LEFT_TOP_MESH: u64 = 4665729213955833856;
pub const RIGHT_BOTTOM_MESH: u64 = 770;
pub const LEFT_BOTTOM_MESH: u64 = 49216;
pub const RIGHT_MESH: u64 = 197123;
pub const LEFT_MESH: u64 = 12599488;
pub const TOP_MESH: u64 = 362258295026614272;
pub const BOTTOM_MESH: u64 = 1797;
pub const GENERAL_MESH: u64 = 460039;

// Converts row and columns into a bitboard position
pub fn bitboard_position(row: u8, column: u8) -> u64 {
    let mut bitboard: u64 = 1;
    bitboard <<= ((7 - row) * 8) + (7 - column);
    return bitboard;
}

// Converts bitboard position into rows and columns
pub fn bitboard_rowcol(position: u64) -> (u8, u8) {
    let mut row: u8 = 0;
    let mut column: u8 = 0;
    'running: for i in 0..64 {
        let mut pointer: u64 = 1;
        pointer <<= 63 - i;
        if pointer & position != 0 {
            row = i / 8;
            column = i % 8;
            break 'running;
        }
    }
    (row, column)
}

enum Position {
    Inner6x6(u64),
    LeftEdge(u64),
    RightEdge(u64),
    TopEdge(u64),
    BottomEdge(u64),
    LeftTop,
    RightTop,
    LeftBottom,
    RightBottom,
}

impl Position {
    fn new(position: u64) -> Position {
        let is_inner_6x6 = position & GENERAL_EDGE != 0;
        let is_edge_right = position & RIGHT_EDGE != 0;
        let is_edge_left = position & LEFT_EDGE != 0;
        let is_edge_top = position & TOP_EDGE != 0;
        let is_edge_bottom = position & BOTTOM_EDGE != 0;
        let is_lefttop = is_edge_left && is_edge_top;
        let is_righttop = is_edge_right && is_edge_top;
        let is_leftbottom = is_edge_left && is_edge_bottom;
        let is_rightbottom = is_edge_right && is_edge_bottom;

        if is_inner_6x6 {
            return Position::Inner6x6(position);
        } else if is_righttop {
            return Position::RightTop;
        } else if is_lefttop {
            return Position::LeftTop;
        } else if is_rightbottom {
            return Position::RightBottom;
        } else if is_leftbottom {
            return Position::LeftBottom;
        } else if is_edge_top {
            return Position::TopEdge(position);
        } else if is_edge_left {
            return Position::LeftEdge(position);
        } else if is_edge_right {
            return Position::RightEdge(position);
        } else {
            return Position::BottomEdge(position);
        }
    }

    // Checks surrounding 3x3 grid with position as center
    // for neighbouring pieces
    fn placable_mesh(&self) -> u64 {
        match self {
            Self::Inner6x6(position) => {
                let (row, column) = bitboard_rowcol(*position);
                GENERAL_MESH << (8 * (6 - row) + (6 - column))
            }
            Self::LeftEdge(position) => {
                let (row, _) = bitboard_rowcol(*position);
                LEFT_MESH << (8 * (6 - row))
            }
            Self::RightEdge(position) => {
                let (row, _) = bitboard_rowcol(*position);
                RIGHT_MESH << (8 * (6 - row))
            }
            Self::TopEdge(position) => {
                let (_, column) = bitboard_rowcol(*position);
                TOP_MESH << (6 - column)
            }
            Self::BottomEdge(position) => {
                let (_, column) = bitboard_rowcol(*position);
                BOTTOM_MESH << (6 - column)
            }
            Self::LeftTop => LEFT_TOP_MESH,
            Self::RightTop => RIGHT_TOP_MESH,
            Self::LeftBottom => LEFT_BOTTOM_MESH,
            Self::RightBottom => RIGHT_BOTTOM_MESH,
        }
    }
}

// Returns the position for pieces on the board in Vec form
pub fn piece_positions(board: u64) -> Option<Vec<u64>> {
    let mut positions = Vec::new();
    for i in 0..64 {
        let position: u64 = 1 << i;
        if board & position != 0 {
            positions.push(position);
        }
    }
    if positions.is_empty() {
        return None;
    }
    return Some(positions);
}

// Mesh for surrounding adjacent squares
fn all_placable(board: u64) -> Option<u64> {
    let mut possible_moves: u64 = 0;
    let positions = piece_positions(board)?;
    for position in positions {
        if board & position != 0 {
            let mesh = Position::new(position).placable_mesh();
            possible_moves |= mesh & !board;
        }
    }
    if possible_moves == 0 {
        return None;
    }
    return Some(possible_moves);
}

// Mesh searching in "shifted" direction for encapsulating piece
fn directional_mesh(
    position: u64,
    ally: u64,
    foe: u64,
    shift: impl Fn(u64, u64) -> u64,
    is_border: impl Fn(u64) -> bool,
) -> Option<u64> {
    let mut count = 1;
    let mut mesh = 0;
    loop {
        let shifted = shift(position, count);
        if (shifted & foe) != 0 {
            mesh |= shifted;
        } else if (shifted & ally) != 0 {
            return Some(mesh);
        } else {
            return None;
        }
        if is_border(shifted) {
            return None;
        }
        count += 1;
    }
}

// Mesh searching all 8 directions for encapsulating piece
pub fn available_captures(ally: u64, foe: u64) -> Option<HashMap<u64, u64>> {
    let psuedo_legal = all_placable(ally | foe)?;
    let mut legal_moves = HashMap::new();
    let positions = piece_positions(psuedo_legal)?;

    for position in positions {
        let mut all_star: u64 = 0;
        let (row, col) = bitboard_rowcol(position);

        // East
        if col < 7 {
            if let Some(mesh) = directional_mesh(
                position,
                ally,
                foe,
                |position, shifted| position >> shifted,
                |position| {
                    let (_row, column) = bitboard_rowcol(position);
                    column == 7
                },
            ) {
                all_star |= mesh;
            }
        }

        // West
        if col > 0 {
            if let Some(mesh) = directional_mesh(
                position,
                ally,
                foe,
                |position, shifted| position << shifted,
                |position| {
                    let (_row, column) = bitboard_rowcol(position);
                    column == 0
                },
            ) {
                all_star |= mesh;
            }
        }

        // North
        if row > 0 {
            if let Some(mesh) = directional_mesh(
                position,
                ally,
                foe,
                |position, shifted| position << shifted * 8,
                |position| {
                    let (row, _column) = bitboard_rowcol(position);
                    row == 0
                },
            ) {
                all_star |= mesh;
            }
        }

        // South
        if row < 7 {
            if let Some(mesh) = directional_mesh(
                position,
                ally,
                foe,
                |position, shifted| position >> shifted * 8,
                |position| {
                    let (row, _column) = bitboard_rowcol(position);
                    row == 7
                },
            ) {
                all_star |= mesh;
            }
        }

        // NorthEast
        if row > 0 && col < 7 {
            if let Some(mesh) = directional_mesh(
                position,
                ally,
                foe,
                |position, shifted| (position << shifted * 8) >> shifted * 1,
                |position| {
                    let (row, column) = bitboard_rowcol(position);
                    row == 0 || column == 7
                },
            ) {
                all_star |= mesh;
            }
        }

        // SouthEast
        if row < 7 && col < 7 {
            if let Some(mesh) = directional_mesh(
                position,
                ally,
                foe,
                |position, shifted| (position >> shifted * 8) >> shifted * 1,
                |position| {
                    let (row, column) = bitboard_rowcol(position);
                    row == 7 || column == 7
                },
            ) {
                all_star |= mesh;
            }
        }

        // NorthWest
        if row > 0 && col > 0 {
            if let Some(mesh) = directional_mesh(
                position,
                ally,
                foe,
                |position, shifted| (position << shifted * 8) << shifted * 1,
                |position| {
                    let (row, column) = bitboard_rowcol(position);
                    row == 0 || column == 0
                },
            ) {
                all_star |= mesh;
            }
        }

        // SouthWest
        if row < 7 && col > 0 {
            if let Some(mesh) = directional_mesh(
                position,
                ally,
                foe,
                |position, shifted| (position >> shifted * 8) << shifted * 1,
                |position| {
                    let (row, column) = bitboard_rowcol(position);
                    row == 7 || column == 0
                },
            ) {
                all_star |= mesh;
            }
        }

        if all_star != 0 {
            legal_moves.insert(position, all_star);
        }
    }

    if legal_moves.is_empty() {
        return None;
    }
    return Some(legal_moves);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitboard_position() {
        let row = 7;
        let col = 7;
        assert_eq!(bitboard_position(row, col), 1);
    }

    #[test]
    fn test_biboard_rowcol() {
        let position = 1 << 7;
        assert_eq!(bitboard_rowcol(position), (7, 0));
    }

    #[test]
    fn test_position_placement() {
        let position = 137438953472;
        assert_eq!(Position::new(position).placable_mesh(), 123490778742784);
    }

    #[test]
    fn test_all_placable() {
        let board = 33252697899776;
        assert_eq!(all_placable(board), Some(17839856411507719));
    }

    #[test]
    fn text_all_star_mesh() {
        let ally = 68987912192;
        let foe = 34493956096;
        let mut all_star = 0;
        for mesh in available_captures(ally, foe).unwrap().keys() {
            all_star |= mesh;
        }
        assert_eq!(all_star, 4415293751296);
    }
}
