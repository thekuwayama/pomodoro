use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use console::Term;

use crate::event::Event;
use crate::input::Reader;

const MSEC_PER_FLAME: u64 = 500;
const MSEC_TICKER_RATE: u64 = 1000;

pub(crate) fn start(start: &Instant, period: u64) {
    let term = Term::stdout();
    let (tx1, rx): (mpsc::Sender<Event>, mpsc::Receiver<Event>) = mpsc::channel();
    let tx2 = tx1.clone();
    thread::spawn(|| -> ! {
        let ticker = Ticker::new(tx1);
        ticker.run();
    });

    thread::spawn(|| {
        let reader = Reader::new(tx2);
        reader.run();
    });

    loop {
        match rx.recv().unwrap() {
            Event::StopTimer => continue,
            Event::TickTimer(t) => {
                let elapsed = t.duration_since(*start);
                if elapsed.as_secs() > period * 60 {
                    return;
                }

                term.clear_line().unwrap();
                let s = format!("start: {:?}, elapsed: {:?}", start, elapsed);
                term.write_line(&s).unwrap();
                term.move_cursor_up(1).unwrap();

                thread::sleep(Duration::from_millis(MSEC_PER_FLAME));
            }
        }
    }
}

struct Ticker {
    tx: mpsc::Sender<Event>,
}

impl Ticker {
    fn new(tx: mpsc::Sender<Event>) -> Self {
        Self { tx }
    }

    fn run(&self) -> ! {
        loop {
            self.tx.send(Event::TickTimer(Instant::now())).unwrap();
            thread::sleep(Duration::from_millis(MSEC_TICKER_RATE));
        }
    }
}
