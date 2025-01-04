use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    // set server host address
    #[arg(long, short, default_value = "127.0.0.1")]
    address: String,
    // set server host port
    #[arg(long, short, default_value_t = 3000)]
    port: u16,
}

impl Args {
    pub fn addr(&self) -> String {
        format!("ws://{}:{}", self.address, self.port)
    }
}
