# Changelog

A simple list of changes to my QuickNote CLI project.

## January 14, 2026
- **New Feature: Custom Save Paths:**
    - Users can now specify a custom file path for saving notes (defaults to `notes.txt`).
    - Added prompt for save location on startup.
- **Code Review & Documentation:**
    - Added TODO comments throughout the codebase identifying areas for future improvements.
    - Improved note format from "message, Created at: timestamp" to "message [timestamp]" for better readability.

## January 13, 2026
- **Code Refactoring:**
    - Refactored input validation to use Rust pattern matching (`match`) instead of nested if-else statements for cleaner, more idiomatic code.
    - Moved `save_note` function outside of `main` to improve code organization and modularity.
- **Code Quality:** Improved code readability and maintainability following Rust best practices.

## January 12, 2026
- **Added Interactive Mode:** The program now enters a live input loop if no command-line arguments are provided.
- **Improved CLI Arguments:** Used argument joining to allow multi-word notes without needing quotation marks.
- **Enhanced Time Accuracy:** Switched from UTC to Local timezone for more relevant note timestamps.
- **Input Validation:** Added logic to ignore empty entries and prevent cluttered log files.
- **Interactive UI Improvements:**
    - Added a persistent `>` blue prompt for better user feedback.
    - Added a `quit` command to exit the interactive session.
- **Styling:**
    - Added colored output using the `colored` library.
    - Success messages now display in green bold text.
    - Error and exit hints display in red.
- **Cleanup:** Removed the welcome banner and updated the README roadmap.

## January 11, 2026
- Added timestamps to notes using `chrono` library.
- Each note now includes date and time.
- Improved error handling in the code.

## January 10, 2026
- Created README with project description.
- Added features list and roadmap.

## January 9, 2026
- Initial project setup.
- Basic note-taking functionality (saving to `notes.txt`).
- Added command-line argument parsing.