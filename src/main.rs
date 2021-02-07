mod logger;
mod util;

use std::{error::Error, io, sync::mpsc, thread};
use sysinfo::SystemExt;
use termion::{input::TermRead, raw::IntoRawMode};
use tui::{backend::TermionBackend, Terminal};

use crate::logger::Logger;
use crate::util::{ui, App};

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    //let stdout = MouseTerminal::from(stdout);
    //let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new("rstop", 100);
    let mut app_clone = app.clone();
    let mut logger = Logger::init();

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || loop {
        for c in io::stdin().keys() {
            match c {
                Ok(key) => {
                    match tx.send(key) {
                        Ok(_) => {}
                        Err(err) => {
                            eprintln!("{}", err);
                        }
                    }
                    app_clone.on_key(key);
                }
                Err(_) => {}
            }
            break;
        }
    });

    loop {
        let mut system = sysinfo::System::new_all();
        app.refresh(&mut system, &mut logger);
        terminal.draw(|f| ui::draw(f, &mut app))?;

        match rx.recv() {
            Ok(key) => app.on_key(key),
            Err(_) => {}
        }

        //for c in stdin.keys() {
        //    match c {
        //        Ok(key) => app.on_key(key),
        //        Err(_) => {}
        //    }
        //    break;
        //}

        if app.should_quit {
            break;
        }
    }

    Ok(())

    //terminal.draw(|f| {
    //    let size = f.size();
    //    let block = Block::default().title("Block").borders(Borders::ALL);
    //    f.render_widget(block, size);
    //})?;
}
