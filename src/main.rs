use git2::{Repository};
use clap::Parser;
mod cli;
mod llm;
mod git;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = cli::Args::parse();

    let repo = match Repository::open(&args.repo_path) {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("Failed to open repository: {}", e);
            return Ok(());
        }
    };

    let git = git::GIT { repository: repo };
    let diff_output = git.fetch_diff();
    let branch_name = git.fetch_branch_name();
    
    llm::claude::review_diff(&diff_output, &branch_name).await?;
    Ok(())
}
