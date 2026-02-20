use git2::{Repository, DiffFormat};
use claude_sdk::{ClaudeClient, MessagesRequest, Message};
use clap::Parser;
use std::env;
mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let api_key = std::env::var("ANTHROPIC_API_KEY").expect("API_KEY must be set");
    
    let client = ClaudeClient::anthropic(api_key);

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

    let prompt = format!("Can you review this git diff and create a meaningful git commit message with brief note:\n\n{}", diff_output);
    let request = MessagesRequest::new(
        "claude-haiku-4-5-20251001",
        1024,
        vec![Message::user(&prompt)],
    );

    let response = client.send_message(request).await?;
    for block in &response.content {
        println!("{:#?}", block);
    }
    Ok(())
    
   
}
