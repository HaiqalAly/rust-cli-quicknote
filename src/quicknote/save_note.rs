use chrono::{self, DateTime, Local};
use std::fs::OpenOptions;
use std::{io::Error, io::Write};

pub fn save_note(message: &str, path: &str) -> Result<(), Error> {
    let now: DateTime<Local> = Local::now();
    let time = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let mut file = OpenOptions::new().create(true).append(true).open(path)?;

    // TODO: could add tags or categories for organizing notes better
    writeln!(file, "{} [{}]", message, time)?;
    Ok(())
}
