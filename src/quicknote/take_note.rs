use colored::Colorize;
use std::{io, io::Error, io::Write};

use crate::quicknote::save_note::save_note;

pub fn take_note(save_location: &str) -> Result<(), Error> {
    println!(
        "\nType your note below. Type '{}' to finish.",
        "quit".red().bold()
    );
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

    Ok(())
}
