use clap::Parser;

#[derive(Parser)]
#[command(version)]
pub struct Args {
    /// Problem file path
    pub file: String,
}

impl Args {
    pub fn build() -> Args {
        Args::parse()
    }
}