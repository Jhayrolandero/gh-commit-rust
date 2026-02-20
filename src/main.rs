use git2::Repository;

fn main() {

let repo = Repository::open(".").expect("Failed to open repo");

    // Get HEAD commit
    let head = repo.head().unwrap();
    let commit = head.peel_to_commit().unwrap();

    println!("Branch: {}", head.shorthand().unwrap_or("detached"));
    println!("Commit: {}", commit.id());
    println!("Message: {}", commit.message().unwrap_or(""));
    println!("Author: {}", commit.author().name().unwrap_or(""));
}
