use clap::Parser;
#[derive(Parser)]
pub struct Args {
    #[arg(default_value = ".")]
    pub repo_path: String
}