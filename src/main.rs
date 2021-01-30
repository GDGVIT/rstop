mod util;

use std::{error::Error, io};
use sysinfo::SystemExt;
use termion::{input::TermRead, raw::IntoRawMode};
use tui::{backend::TermionBackend, Terminal};

use crate::util::{ui, App};

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    //let stdout = MouseTerminal::from(stdout);
    //let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new("rstop");

    loop {
        let mut system = sysinfo::System::new_all();
        let stdin = io::stdin();
        App::refresh(&mut app, &mut system);
        terminal.draw(|f| ui::draw(f, &mut app))?;

        for c in stdin.keys() {
            match c {
                Ok(key) => app.on_key(key),
                Err(_) => {}
            }
            break;
        }

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
