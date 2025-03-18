use qap_utils::problem_loader::load_from_file;
use run_utils::args::Args;

fn main() {
    let args = Args::build();

    load_from_file(&args.file)
}
