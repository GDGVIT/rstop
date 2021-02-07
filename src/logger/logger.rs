use std::fs::{self, OpenOptions};
use std::io::prelude::*;

pub struct Logger {
    file: fs::File,
}

impl Logger {
    pub fn init() -> Logger {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("logging.log")
            .unwrap();

        if let Err(err) = writeln!(file, "----Logs----\n") {
            eprintln!("Couldnt write to file: {}", err);
        }

        Logger { file }
    }

    pub fn add_log<T>(&mut self, log: T) -> Result<(), Box<dyn std::error::Error>>
    where
        T: Into<String> + std::fmt::Display,
    {
        writeln!(self.file, "{}", log)?;
        Ok(())
    }
}
