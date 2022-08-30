use std::io::stdin;
use std::sync::mpsc;

use termion::event::Key;
use termion::input::TermRead;

use crate::event::Event;

pub(crate) struct Reader {
    play: mpsc::Sender<Event>,
}

impl Reader {
    pub(crate) fn new(play: mpsc::Sender<Event>) -> Self {
        Self { play }
    }

    pub(crate) fn run(&self) {
        let stdin = stdin();
        for c in stdin.keys() {
            match c {
                Ok(Key::Ctrl('p')) => self.play.send(Event::Play).unwrap(),
                Ok(Key::Ctrl('s')) => self.play.send(Event::Stop).unwrap(),
                _ => {}
            }
        }
    }
}
