use chrono::{self, DateTime, Local};
use colored::Colorize;
use std::fs::OpenOptions;
use std::path::PathBuf;
use std::{env, io};
use std::{io::Error, io::Write};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    println!("Where do you want to save the notes? (default: notes.txt)");
    io::stdout().flush()?;
    print!("{} ", ">".blue());
    io::stdout().flush()?;
    let mut save_location = String::new();
    std::io::stdin().read_line(&mut save_location)?;
    let save_location: &str = save_location.trim();
    // TODO: should probably expand the tilde (~) for home directory and validate the path exists
    // or at least check if we can write to that location before starting
    let save_location = if save_location.is_empty() {
        "notes.txt"
    } else {
        save_location
    };

    if args.len() < 2 {
        print!("Enter your note (or '{}' to exit): ", "quit".red().bold());
        io::stdout().flush()?;
        println!(" ");
        loop {
            print!("{} ", ">".blue());
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
    } else {
        let message = args[1..].join(" ");
        save_note(&message, save_location)?;
    };

    Ok(())
}

fn save_note(message: &str, path: &str) -> Result<(), Error> {
    let now: DateTime<Local> = Local::now();
    let time = now.format("%Y-%m-%d %H:%M:%S").to_string();
    // TODO: this will always force .txt extension even if user specified something else - maybe respect their choice?
    let path = PathBuf::from(path).with_extension("txt");

    let mut file = OpenOptions::new().create(true).append(true).open(path)?;

    // TODO: could add tags or categories for organizing notes better
    writeln!(file, "{} [{}]", message, time)?;
    println!("{} {}", ">".blue(), "Note saved!".green().bold());
    Ok(())
}
