mod logger;
mod util;

use std::{error::Error, io};
use sysinfo::SystemExt;
use termion::raw::IntoRawMode;
use tui::{backend::TermionBackend, Terminal};

use crate::logger::Logger;
use crate::util::{event, ui, App};

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    //let stdout = MouseTerminal::from(stdout);
    //let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new("rstop", 100);
    let mut logger = Logger::init();

    let events = event::Events::default();

    loop {
        let mut system = sysinfo::System::new_all();
        terminal.draw(|f| ui::draw(f, &mut app))?;

        match &events.next_event()? {
            event::Event::Input(key) => app.on_key(&key),
            event::Event::Tick => app.refresh(&mut system, &mut logger),
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
