use std::fs::OpenOptions;
use std::{io::Write, io::Error};
use std::{env, io};
use chrono::{self, DateTime, Local};
use colored::Colorize;

fn main() -> Result<(), Error>{
    let args: Vec<String> = env::args().collect();
    

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
                "quit" => {
                    break
                } "" => {
                    println!("{}", "Please enter a valid note.".red().bold());
                } _ => {
                    save_note(input)?;
                }
            }
        }
    } else {
        let message = args[1..].join(" ");
        save_note(&message)?;
    };

    Ok(())
}

fn save_note(message: &str) -> Result<(), Error> {
        let now: DateTime<Local> = Local::now();
        let time = now.format("%Y-%m-%d %H:%M:%S").to_string();

        let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("notes.txt")?;

        writeln!(file, "{}, Created at: {}", message, time)?;
        println!("{}", "Note saved!".green().bold());
        Ok(())
}
