use std::io::{stdin, stdout, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::{Duration, Instant};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, cursor, screen};

use crate::event::Event;

const MSEC_PER_FLAME: u64 = 500;
const MSEC_TICKER_RATE: u64 = 1000;

struct TickTimer(Duration);

pub(crate) fn start(duration: Duration) {
    let (ttick, rtick) = mpsc::channel::<TickTimer>();
    let (tplay, rplay) = mpsc::channel::<Event>();
    let latch1 = Arc::new(AtomicBool::new(true));
    let latch2 = latch1.clone();
    let th = thread::spawn(move || {
        let ticker = Ticker::new(duration, ttick, rplay, latch1);
        ticker.run();
    });

    let stdin = stdin();
    let mut screen = screen::AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let rh = thread::spawn(move || {
        for c in stdin.keys() {
            match c {
                Ok(Key::Ctrl('p')) => {
                    if latch2.load(Ordering::Relaxed) {
                        latch2.store(false, Ordering::Relaxed);
                        tplay.send(Event::Pose).unwrap();
                    } else {
                        latch2.store(true, Ordering::Relaxed);
                        tplay.send(Event::Play).unwrap();
                    }
                }
                Ok(Key::Ctrl('s')) | Ok(Key::Ctrl('c')) | Ok(Key::Ctrl('d')) => {
                    latch2.store(false, Ordering::Relaxed);
                    tplay.send(Event::Stop).unwrap();
                    return;
                }
                _ => {}
            }
        }
    });

    let wh = thread::spawn(move || {
        write!(screen, "{}{}", clear::CurrentLine, cursor::Hide).unwrap();
        loop {
            if let Ok(t) = rtick.recv() {
                if t.0 > duration {
                    return;
                }

                write!(screen, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
                write!(
                    screen,
                    "rest: {min:}:{sec:02}",
                    min = t.0.as_secs() / 60,
                    sec = t.0.as_secs() % 60
                )
                .unwrap();
                screen.flush().unwrap();

                thread::sleep(Duration::from_millis(MSEC_PER_FLAME));
            } else {
                write!(screen, "{}{}", clear::CurrentLine, cursor::Show).unwrap();
                return;
            }
        }
    });

    th.join().unwrap();
    wh.join().unwrap();
    rh.join().unwrap();
}

struct Ticker {
    duration: Duration,
    tick: mpsc::Sender<TickTimer>,
    play: mpsc::Receiver<Event>,
    latch: Arc<AtomicBool>,
}

impl Ticker {
    fn new(
        duration: Duration,
        tick: mpsc::Sender<TickTimer>,
        play: mpsc::Receiver<Event>,
        latch: Arc<AtomicBool>,
    ) -> Self {
        Self {
            duration,
            tick,
            play,
            latch,
        }
    }

    fn run(&self) {
        let mut duration = self.duration;
        let mut end = Instant::now() + self.duration;
        loop {
            if !self.latch.load(Ordering::Relaxed) {
                match self.play.recv() {
                    Ok(Event::Play) => {
                        self.latch.store(true, Ordering::Relaxed);
                        end = Instant::now() + duration;
                    }
                    Ok(Event::Pose) => {
                        self.latch.store(false, Ordering::Relaxed);
                        duration = end - Instant::now();
                        continue;
                    }
                    Ok(Event::Stop) => return,
                    _ => continue,
                }
            }

            self.tick.send(TickTimer(end - Instant::now())).unwrap();
            thread::sleep(Duration::from_millis(MSEC_TICKER_RATE));
        }
    }
}
