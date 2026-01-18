# QuickNote CLI
A lightning-fast command-line tool for capturing thoughts before they vanish. Written in Rust as a learning project.

## Project Status: Complete (Learning Milestone)
This was my first project in Rust, focused on mastering CLI basics, module system and error handling. I have achieved my primary learning goals and am moving on to more complex projects.

### Few Key Learning Outcomes
- Memory Safety: Practiced string ownership and borrowing across modules.
- Crates: Integrated chrono for time and colorize for terminal UI.
- Idiomatic Rust: Refactored from if/else to match patterns.
- STD API: Used fs::OpenOptions for file manipulation, PathBuf for path handling, and io::stdin()/stdout() for user interaction.
- String Parsing & Manipulation: Used rsplit_once() for timestamp extraction and trim() variations for input sanitization.

## How it Works
QuickNote takes your input and appends it to a file with timestamps. You can specify a custom file path or press Enter to use the default `notes.txt`.
Now includes menu to either add new notes or view existing ones.

**Usage**:
```bash
# Clone the repo
git clone https://github.com/HaiqalAly/rust-cli-quicknote.git
cd rust-cli-quicknote

# Interactive mode (Recommended)
cargo run
# 1. Enter save location (or press Enter for default)
# 2. Choose action: 'list' (view notes) or 'add' (new notes)
# 3. If 'add': Type notes at the '>' prompt (type 'quit' to exit)
```

## Features
- **View Notes**: Read your saved notes directly from the terminal
- **Interactive Menu**: Choose between listing or adding notes
- **Custom Save Location**: Choose where to store your notes on startup
- **Interactive Mode**: Enter a persistent session to quickly add multiple notes without restarting
- **Automatic Timestamps**: Each note is prefixed with [YYYY-MM-DD HH:MM:SS]
- **Colored Output**: Visual feedback with green success messages and colored prompts
- **Persistent Storage**: Automatically creates and appends to notes.txt
- **Zero Overhead**: No database required, just plain text
- **Rust Powered**: Fast, safe, and memory-efficient

See [CHANGELOG.md](CHANGELOG.md) for project updates.

## Future Ideas (If i return to this)
**Tags & Categories**: Organize notes with labels for better filtering<br>
**Path Expansion**: Support for `~` (home directory) in file paths<br>
