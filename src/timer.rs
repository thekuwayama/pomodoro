use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use console::Term;

pub(crate) fn start(start: &Instant, period: u64) {
    let term = Term::stdout();
    let (tx, rx) = mpsc::channel();
    thread::spawn(|| -> ! {
        let ticker = Ticker::new(tx);
        ticker.run();
    });

    loop {
        let elapsed = rx.recv().unwrap().duration_since(*start);
        if elapsed.as_secs() > period * 60 {
            return;
        }

        let s = format!("start: {:?}, elapsed: {:?}", start, elapsed);
        term.write_line(&s).unwrap();
        term.move_cursor_up(1).unwrap();
        thread::sleep(Duration::from_millis(1000));
        term.clear_line().unwrap();
    }
}

struct Ticker {
    tx: mpsc::Sender<Instant>,
}

impl Ticker {
    fn new(tx: mpsc::Sender<Instant>) -> Self {
        Self { tx }
    }

    fn run(&self) -> ! {
        loop {
            self.tx.send(Instant::now()).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    }
}
