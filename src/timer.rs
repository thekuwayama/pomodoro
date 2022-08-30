use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use console::Term;

use crate::event::Event;
use crate::input::Reader;

const MSEC_PER_FLAME: u64 = 500;
const MSEC_TICKER_RATE: u64 = 1000;

struct TickTimer(Instant);

pub(crate) fn start(start: &Instant, period: u64) {
    let term = Term::stdout();
    let (ttick, rtick): (mpsc::Sender<TickTimer>, mpsc::Receiver<TickTimer>) = mpsc::channel();
    let (tplay, rplay): (mpsc::Sender<Event>, mpsc::Receiver<Event>) = mpsc::channel();
    thread::spawn(move || -> ! {
        let ticker = Ticker::new(ttick, rplay);
        ticker.run();
    });

    thread::spawn(|| {
        let reader = Reader::new(tplay);
        reader.run();
    });

    loop {
        if let Ok(t) = rtick.recv() {
            let elapsed = t.0.duration_since(*start);
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

struct Ticker {
    tick: mpsc::Sender<TickTimer>,
    play: mpsc::Receiver<Event>,
}

impl Ticker {
    fn new(tick: mpsc::Sender<TickTimer>, play: mpsc::Receiver<Event>) -> Self {
        Self { tick, play }
    }

    fn run(&self) -> ! {
        let mut latch = true;
        loop {
            if !latch {
                match self.play.recv() {
                    Ok(Event::Play) => latch = true,
                    Ok(Event::Stop) => latch = false,
                    _ => {}
                }

                continue;
            }

            self.tick.send(TickTimer(Instant::now())).unwrap();
            thread::sleep(Duration::from_millis(MSEC_TICKER_RATE));
        }
    }
}
