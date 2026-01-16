# QuickNote CLI
A lightning-fast command-line tool for capturing thoughts before they vanish. Written in Rust as a learning project.

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

# Single note mode
cargo run -- "Pick up milk on the way home"
# (You will be prompted for save location first)
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

## Planned Features
[x] **Custom File Paths**: Users can now specify where their notes are saved<br>
[ ] **List/Search Commands**: Add commands to view recent notes or search through them(List command have been implemented but it's still need improvements)<br>
[ ] **Tags & Categories**: Organize notes with labels for better filtering<br>
[ ] **Path Expansion**: Support for `~` (home directory) in file paths<br>
