//use std::{io, sync::mpsc, thread};
//use termion::{event::Key, input::TermRead};
//
//use crate::util::App;
//
//pub struct Event {
//    rx: mpsc::Receiver<Key>,
//}
//
//impl Event {
//    fn init(app: &'static mut App) -> Event {
//        let (tx, rx) = mpsc::channel();
//        thread::spawn(move || loop {
//            for c in io::stdin().keys() {
//                match c {
//                    Ok(key) => app.on_key(key),
//                    Err(_) => {}
//                }
//                break;
//            }
//        });
//        Event { rx }
//    }
//}
