use git2::{Repository, DiffFormat};

pub struct GIT {
    pub repository: Repository,
}

impl GIT {
    pub fn fetch_diff(&self) -> String {

         // Get HEAD commit
    let head = self.repository.head().unwrap().peel_to_tree().unwrap(); // Fetch tree object 
    
    // Diff HEAD vs current working directory
    let diff = self.repository.diff_tree_to_workdir_with_index(Some(&head), None).unwrap();

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

    diff_output
    }


    pub fn fetch_branch_name(&self) -> String {
        let head = self.repository.head().unwrap();
        let branch_name = head.shorthand().unwrap_or("unknown").to_string();
        branch_name
    }
}