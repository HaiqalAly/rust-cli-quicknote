use colored::Colorize;
use std::io::{self, Error, Write};

use crate::quicknote::read_note::read_note;

pub fn delete_note(path: &str) -> Result<(), Error> {
    read_note(path)?;

    let content = std::fs::read_to_string(path)?;
    let mut lines: Vec<String> = content.lines().map(String::from).collect();

    if content.trim().is_empty() {
        return Ok(());
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let id: usize = match input.trim().parse() {
        Ok(num) if num > 0 => num,
        _ => {
            println!("{}", "Invalid ID. Please enter a number.".red());
            return Ok(());
        }
    };

    print!("\n{}", "Enter the ID to delete: ".yellow().bold());
    io::stdout().flush()?;

    if id > lines.len() {
        println!("{}", "ID out of range!".red());
        return Ok(());
    }

    lines.remove(id - 1);

    let after_delete = lines.join("\n");

    if !after_delete.is_empty() {
        std::fs::write(path, after_delete + "\n")?;
    } else {
        std::fs::write(path, "")?;
    }
    Ok(())
}
