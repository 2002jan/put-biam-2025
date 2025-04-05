use clap::Parser;

#[derive(Parser)]
#[command(version)]
pub struct Args {
    /// Problem file path
    pub file: String,

    /// Best solution file path
    #[arg(short, long)]
    pub solution_file: Option<String>,

    #[arg(short, long)]
    pub outputs_folder: Option<String>,

    /// Whether to also calculate similarity scores
    #[arg(long = "ss")]
    pub calculate_similarity: bool
}

impl Args {
    pub fn build() -> Args {
        Args::parse()
    }
}