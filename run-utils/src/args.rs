use clap::Parser;

#[derive(Parser)]
#[command(version)]
pub struct Args {
    /// Problem file path
    pub file: String,

    /// Best solution file path
    #[arg(short, long)]
    pub solution_file: Option<String>
}

impl Args {
    pub fn build() -> Args {
        Args::parse()
    }
}