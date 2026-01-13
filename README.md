# QuickNote CLI
A lightning-fast command-line tool for capturing thoughts before they vanish. Written in Rust as a learning project.

## How it Works
QuickNote takes your input and appends it to a local notes.txt file with timestamps. You can use it in two ways:

**Usage**:
```bash
# Clone the repo
git clone https://github.com/HaiqalAly/rust-cli-quicknote.git
cd rust-cli-quicknote

# Quick note mode (single note)
cargo run -- "Pick up milk on the way home"

# Interactive mode (multiple notes)
cargo run
# Then type notes at the prompt. Type 'quit' to exit.
```

## Features
- **Interactive Mode**: Enter a persistent session to quickly add multiple notes without restarting
- **Automatic Timestamps**: Each note is prefixed with [YYYY-MM-DD HH:MM:SS]
- **Colored Output**: Visual feedback with green success messages and colored prompts
- **Persistent Storage**: Automatically creates and appends to notes.txt
- **Zero Overhead**: No database required, just plain text
- **Rust Powered**: Fast, safe, and memory-efficient

See [CHANGELOG.md](CHANGELOG.md) for project updates.

## Roadmap
[ ] **Custom File Paths**: Allow users to specify where their notes are saved<br>
[ ] **List/Delete**: Add commands to view or clear the notes list<br>
