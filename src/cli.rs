use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, default_value = "mongodb://localhost:27017")]
    pub uri: String,
}

pub fn args() -> Args {
    Args::parse()
}
