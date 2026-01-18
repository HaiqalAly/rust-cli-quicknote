use colored::Colorize;
use std::path::PathBuf;
use std::{io, io::Error, io::Write};

mod notes;
use notes::{read_note, take_note}; 

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


