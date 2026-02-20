use git2::{Repository, DiffFormat};
use clap::Parser;
mod cli;
mod llm;


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

    // Get HEAD commit
    let head = repo.head().unwrap().peel_to_tree().unwrap(); // Fetch tree object 
    
    // Diff HEAD vs current working directory
    let diff = repo.diff_tree_to_workdir_with_index(Some(&head), None).unwrap();

    // Collect diff into a string
    let mut diff_output = String::new();
    diff.print(DiffFormat::Patch, |_delta, _hunk, line| {
        let content = std::str::from_utf8(line.content()).unwrap_or("");
        match line.origin() {
            '+' => diff_output.push_str(&format!("+{}", content)),
            '-' => diff_output.push_str(&format!("-{}", content)),
            _ =>  diff_output.push_str(&format!(" {}", content)),
        }
        true
    }).unwrap();

    llm::claude::review_diff(&diff_output).await?;
    Ok(())
}
