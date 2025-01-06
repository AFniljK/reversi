use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    // set server host address
    #[arg(long, short, default_value = "127.0.0.1")]
    address: String,
    // set server host port
    #[arg(long, short, default_value_t = 3000)]
    port: u16,
    // set playing piece to white (default=black) if possible
    #[arg(long, short)]
    white_piece: bool,
}

impl Args {
    pub fn addr(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }

    pub fn wants_black(&self) -> bool {
        !self.white_piece
    }
}
