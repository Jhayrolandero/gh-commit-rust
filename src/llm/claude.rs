use claude_sdk::{ClaudeClient, MessagesRequest, Message};


const INSTRUCTION: &str = "You are a helpful assistant that reviews git diffs and generates meaningful\n
                        commit messages. Please analyze the following git diff and provide a concise commit\n
                        message that summarizes the changes made. Conform to these formatting rules:\n

                        - The commit message should be brief and to the point, ideally under 50 characters.\n
                        - Use the following prefixes to categorize the type of change:\n
                            test: Update test/* files
                            dist: Changes to submodules, version bumps, updates to package.json
                            minor: Small changes
                            doc: Updates to documentation
                            fix: Bug fixes
                            bin: Update binary scripts associated with the project
                            refactor: Refactor of existing code
                            nit: Small code review changes mainly around style or syntax
                            feat: New features

                        - After this generate it into a ready format {git commit -m `Title` -m `Brief additional Description` }for copy-pasting into a git commit message. Do not include any additional commentary or explanation, just the commit message itself.
";

pub async fn review_diff(diff: &str) -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let api_key = std::env::var("ANTHROPIC_API_KEY").expect("API_KEY must be set");

    let client = ClaudeClient::anthropic(api_key);

    let prompt = format!("{}\n\n{}", INSTRUCTION, diff);
    let request = MessagesRequest::new(
        "claude-haiku-4-5-20251001",
        1024,
        vec![Message::user(&prompt)],
    );

    let response = client.send_message(request).await?;
    for block in &response.content {
        if let claude_sdk::ContentBlock::Text { text, .. } = block {
            println!("{}", text);
        }
    }

    Ok(())
}
