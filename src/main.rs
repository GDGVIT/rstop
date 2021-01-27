mod util;

use std::{error::Error, io};
use sysinfo::SystemExt;
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{backend::TermionBackend, Terminal};

use crate::util::{ui, App};

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new("rstop");
    let mut system = sysinfo::System::new_all();

    App::refresh(&mut app, &mut system);
    terminal.draw(|f| ui::draw(f, &mut app))?;

    Ok(())
}
