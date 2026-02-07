use colored::Colorize;
use std::path::PathBuf;
use std::{io, io::Error, io::Write, process::Command};

mod delete_note;
mod read_note;
mod save_note;
mod take_note;

use delete_note::delete_note;
use read_note::read_note;
use take_note::take_note;

fn main() -> Result<(), Error> {
    println!("Where do you want to save the notes? (default: notes.txt)");
    io::stdout().flush()?;
    print!("{} ", ">".blue());
    io::stdout().flush()?;

    let mut input_path = String::new();
    io::stdin().read_line(&mut input_path)?;
    let trimmed_path = input_path.trim();

    let mut path_buf = PathBuf::from(if trimmed_path.is_empty() {
        "notes.txt"
    } else {
        trimmed_path
    });

    if path_buf.extension().is_none() {
        path_buf.set_extension("txt");
    }

    let final_path = path_buf.to_string_lossy().into_owned();
    loop {
        println!(
            "\nWhat do you want to do? (type '{}', '{}', '{}', '{}')",
            "list".yellow(),
            "add".green(),
            "delete".red(),
            "clear/cls".purple()
        );
        print!("{} ", ">".blue());
        io::stdout().flush()?;

        let mut action = String::new();
        io::stdin().read_line(&mut action)?;

        match action.trim().to_lowercase().as_str() {
            "list" => read_note(&final_path)?,
            "add" => take_note(&final_path)?,
            "delete" => delete_note(&final_path)?,
            "clear" => {
                if cfg!(target_os = "windows") {
                    Command::new("cmd").args(["/c, cls"]).status()?;
                } else {
                    Command::new("clear").status()?;
                }
            }
            _ => println!("{}", "Invalid command!".red().bold()),
        }
    }
}
