use chrono::{self, DateTime, Local};
use colored::Colorize;
use std::fs::OpenOptions;
use std::path::PathBuf;
use std::{io, io::Error, io::Write};

fn main() -> Result<(), Error> {
    println!("Where do you want to save the notes? (default: notes.txt)");
    io::stdout().flush()?;
    print!("{} ", ">".blue());
    io::stdout().flush()?;

    let mut input_path = String::new();
    io::stdin().read_line(&mut input_path)?;
    let trimmed_path = input_path.trim();

    let mut path_buf = PathBuf::from(if trimmed_path.is_empty() { "notes.txt" } else { trimmed_path });

    if path_buf.extension().is_none() {
        path_buf.set_extension("txt");
    }

    let final_path = path_buf.to_string_lossy().into_owned();

    println!("\nWhat do you want to do? (type '{}' or '{}')", "list".yellow(), "add".green());
    print!("{} ", ">".blue());
    io::stdout().flush()?;

    let mut action = String::new();
    io::stdin().read_line(&mut action)?;

    match action.trim().to_lowercase().as_str() {
        "list" => read_note(&final_path)?,
        "add" => take_note(&final_path)?,
        _ => println!("{}", "Invalid command!".red().bold()),
    }

    Ok(())
}

fn take_note(save_location: &str) -> Result<(), Error>{
    loop {
        print!("\nAdd Notes:");
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

fn read_note(path: &str) -> Result<(), Error> {
    if !std::path::Path::new(path).exists() {
        println!("{}", "File not found! Please add a note first.".red().bold());
        return Ok(());
    }

    let content = std::fs::read_to_string(path)?;
    println!("\n{}", "Your notes:".bold());

    if content.is_empty() {
        println!("{}", "Notes empty".red().bold());
    } else {
        // TODO: Parse the lines to separate the message from the [timestamp] for a nicer display.
        println!("{}", content);
    }

    Ok(())
}

fn save_note(message: &str, path: &str) -> Result<(), Error> {
    let now: DateTime<Local> = Local::now();
    let time = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let mut file = OpenOptions::new().create(true).append(true).open(&path)?;

    // TODO: could add tags or categories for organizing notes better
    writeln!(file, "{} [{}]", message, time)?;
    println!("{} {} {}", ">".blue(), "Note saved at".green().bold(), path);
    Ok(())
}
