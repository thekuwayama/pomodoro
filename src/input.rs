use std::io::stdin;
use std::sync::mpsc;

use termion::event::Key;
use termion::input::TermRead;

use crate::event::Event;

pub(crate) struct Reader {
    tx: mpsc::Sender<Event>,
}

impl Reader {
    pub(crate) fn new(tx: mpsc::Sender<Event>) -> Self {
        Self { tx }
    }

    pub(crate) fn run(&self) {
        loop {
            match stdin().keys().next().unwrap() {
                Ok(Key::Char('d')) => self.tx.send(Event::StopTimer).unwrap(),
                _ => continue,
            }
        }
    }
}
