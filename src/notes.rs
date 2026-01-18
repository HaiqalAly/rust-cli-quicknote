use std::fs::OpenOptions;
use std::{io, io::Error, io::Write};
use chrono::{self, DateTime, Local};
use colored::Colorize;

pub fn take_note(save_location: &str) -> Result<(), Error>{
    println!("\nType your note below. Type '{}' to finish.", "quit".red().bold());
    loop {
        print!("\n{} ", ">".blue());
        io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();
        match input.to_lowercase().as_str() {
            "quit" => break,
            "" => {
                println!("{}", "Please enter a valid note.".red().bold());
            }
            _ => {
                save_note(input, save_location)?;
            }
        }
    }

    Ok(())
}

pub fn read_note(path: &str) -> Result<(), Error> {
    if !std::path::Path::new(path).exists() {
        println!("{}", "File not found! Please add a note first.".red().bold());
        return Ok(());
    }

    let content = std::fs::read_to_string(path)?;
    println!("\n{}", "Your notes:".bold());

    if content.is_empty() {
        println!("{}", "Notes empty".red().bold());
    } else {
        for line in content.lines() {
            if let Some((message, time)) = line.rsplit_once('[') {
                let time = time.trim_end_matches(']');
                println!("{} {} {}", ">".blue(), format!("[{}]", time).dimmed(), message.trim());
            } else {
                println!("{} {}", ">".blue(), line);
            }
        }
    }

    Ok(())
}

pub fn save_note(message: &str, path: &str) -> Result<(), Error> {
    let now: DateTime<Local> = Local::now();
    let time = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let mut file = OpenOptions::new().create(true).append(true).open(&path)?;

    // TODO: could add tags or categories for organizing notes better
    writeln!(file, "{} [{}]", message, time)?;
    println!("{} {} {}", ">".blue(), "Note saved at".green().bold(), path);
    Ok(())
}