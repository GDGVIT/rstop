use std::{io, sync::mpsc, thread, time::Duration};
use termion::{event::Key, input::TermRead};

pub enum Event {
    Input(Key),
    Tick,
}

#[derive(Debug)]
pub struct Events {
    rx: mpsc::Receiver<Event>,
}

pub struct Config {
    pub exit_key: Key,
    pub tick_rate: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            exit_key: Key::Char('q'),
            tick_rate: Duration::from_millis(250),
        }
    }
}

impl Events {
    pub fn with_config(config: Config) -> Events {
        let (tx, rx) = mpsc::channel();
        let tx_clone = tx.clone();
        thread::spawn(move || loop {
            for c in io::stdin().keys() {
                match c {
                    Ok(key) => {
                        if let Err(err) = tx_clone.send(Event::Input(key)) {
                            eprintln!("{}", err);
                        }
                    }
                    Err(_) => {}
                }
                break;
            }
        });

        let tx = tx.clone();
        thread::spawn(move || loop {
            if tx.send(Event::Tick).is_err() {
                break;
            }
            thread::sleep(config.tick_rate);
        });
        Events { rx }
    }

    pub fn next_event(&self) -> Result<Event, mpsc::RecvError> {
        self.rx.recv()
    }
}

impl Default for Events {
    fn default() -> Self {
        Events::with_config(Config::default())
    }
}
