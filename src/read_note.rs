use colored::Colorize;
use std::io::Error;

pub fn read_note(path: &str) -> Result<(), Error> {
    if !std::path::Path::new(path).exists() {
        println!(
            "{}",
            "File not found! Please add a note first.".red().bold()
        );
        return Ok(());
    }

    let content = std::fs::read_to_string(path)?;
    println!("\n{}", "Your notes:".bold());

    if content.is_empty() {
        println!("{}", "Notes empty".red().bold());
    } else {
        for (index, line) in content.lines().enumerate() {
            let id = index + 1;

            if let Some((message, time)) = line.rsplit_once('[') {
                let time = time.trim_end_matches(']');
                println!(
                    "{} {} {} {}",
                    id.to_string().cyan(),
                    ">".blue(),
                    format!("[{}]", time).dimmed(),
                    message.trim()
                );
            } else {
                println!("{} {} {}", id.to_string().cyan(), ">".blue(), line);
            }
        }
    }

    Ok(())
}
