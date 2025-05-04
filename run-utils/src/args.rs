use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(version)]
pub struct Args {
    /// Problem file path
    pub file: String,

    #[arg(value_enum, default_value_t = Job::Main)]
    pub job: Job,

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

#[derive(Clone, ValueEnum)]
pub enum Job {
    Main,
    TestTabuHyperParams
}