use git2::Repository;
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(default_value = ".")]
    repo_path: String
}

fn main() {


    let args = Args::parse();

    println!("Opening repository at: {}", args.repo_path);

    let repo = match Repository::open(&args.repo_path) {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("Failed to open repository: {}", e);
            return;
        }
    };

    // Get HEAD commit
    let head = repo.head().unwrap();
    let commit = head.peel_to_commit().unwrap();

    println!("Branch: {}", head.shorthand().unwrap_or("detached"));
    println!("Commit: {}", commit.id());
    println!("Message: {}", commit.message().unwrap_or(""));
    println!("Author: {}", commit.author().name().unwrap_or(""));
}
