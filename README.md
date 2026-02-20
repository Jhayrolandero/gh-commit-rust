# git-rust

A CLI tool that reads your git diff and uses Claude AI to generate a meaningful commit message.

## Requirements

- [Rust](https://rustup.rs/)
- An [Anthropic API key](https://console.anthropic.com/)

## Setup

1. Clone the repo and navigate to it:
   ```bash
   git clone <repo-url>
   cd git-rust
   ```

2. Create a `.env` file in the project root:
   ```
   ANTHROPIC_API_KEY=sk-ant-...
   ```

3. Build the release binary:
   ```bash
   cargo build --release
   ```

4. (Optional) Install globally:
   ```bash
   cargo install --path .
   ```

## Usage

Run inside any git repository:

```bash
# Use current directory
git-rust

# Or specify a repo path
git-rust /path/to/repo
```

The tool will diff your current working directory against HEAD and output a ready-to-use commit message like:

```
git commit -m "feat: add user authentication" -m "Adds JWT-based login and session handling"
```

## Environment Variables

| Variable | Description |
|---|---|
| `ANTHROPIC_API_KEY` | Your Anthropic API key (required) |
