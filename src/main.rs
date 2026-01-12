use std::fs::OpenOptions;
use std::{io::Write, io::Error};
use std::env;
use chrono::{self, DateTime, Utc};
use colored::Colorize;

fn main() -> Result<(), Error>{
    // TODO:
    //       1. Make it actually wait for user input like in Python with interactive style.
    //          Currently it uses argument e.g cargo run -- "Remember to take a break" to work.
    //          Maybe I'll just keep it like this?
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("{}", "Usage: 'Your Notes'".red());
        return Ok(());
    }

    let message = &args[1];
    let now: DateTime<Utc> = Utc::now();
    let time = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("notes.txt")?;

    writeln!(file, "{} | Created at: {}", message, time)?;

    println!("{}", "Note saved!".green().bold());

    Ok(())
}
