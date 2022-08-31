use std::io::{stdout, Write};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use termion::{clear, cursor};

use crate::event::Event;
use crate::input::Reader;

const MSEC_PER_FLAME: u64 = 500;
const MSEC_TICKER_RATE: u64 = 1000;

struct TickTimer(Duration);

pub(crate) fn start(duration: Duration) {
    let (ttick, rtick) = mpsc::channel::<TickTimer>();
    let (tplay, rplay) = mpsc::channel::<Event>();
    thread::spawn(move || -> ! {
        let ticker = Ticker::new(duration, ttick, rplay);
        ticker.run();
    });

    thread::spawn(|| {
        let reader = Reader::new(tplay);
        reader.run();
    });

    print!("{}{}", clear::CurrentLine, cursor::Hide);
    loop {
        if let Ok(t) = rtick.recv() {
            if t.0 > duration {
                return;
            }

            print!("{}{}", clear::All, cursor::Goto(1, 1));
            print!(
                "rest: {min:}:{sec:02}",
                min = t.0.as_secs() / 60,
                sec = t.0.as_secs() % 60
            );
            stdout().flush().unwrap();

            thread::sleep(Duration::from_millis(MSEC_PER_FLAME));
        }
    }
}

struct Ticker {
    duration: Duration,
    tick: mpsc::Sender<TickTimer>,
    play: mpsc::Receiver<Event>,
}

impl Ticker {
    fn new(duration: Duration, tick: mpsc::Sender<TickTimer>, play: mpsc::Receiver<Event>) -> Self {
        Self {
            duration,
            tick,
            play,
        }
    }

    fn run(&self) -> ! {
        let mut latch = true;
        let mut duration = self.duration;
        let mut end = Instant::now() + self.duration;
        loop {
            if !latch {
                match self.play.recv() {
                    Ok(Event::Play) => {
                        latch = true;
                        end = Instant::now() + duration;
                    }
                    Ok(Event::Stop) => {
                        latch = false;
                        duration = end - Instant::now();
                        continue;
                    }
                    _ => continue,
                }
            }

            self.tick.send(TickTimer(end - Instant::now())).unwrap();
            thread::sleep(Duration::from_millis(MSEC_TICKER_RATE));
        }
    }
}
