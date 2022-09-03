use std::io::{stdin, stdout, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Duration;

use anyhow::{anyhow, Result};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, cursor, screen};

use crate::event::Event;
use crate::format;
use crate::ticker::{TickTimer, Ticker};

const MSEC_PER_FLAME: u64 = 500;
const MSEC_TICKER_RATE: u64 = 1000;

pub(crate) enum ExitStatus {
    Completed,
    Terminated,
}

pub(crate) fn run_working(duration: Duration) -> Result<ExitStatus> {
    run(format::working_format, duration)
}

pub(crate) fn run_break(duration: Duration) -> Result<ExitStatus> {
    run(format::break_format, duration)
}

fn run<F: Fn(Duration) -> String + std::marker::Send + 'static>(
    f: F,
    duration: Duration,
) -> Result<ExitStatus> {
    let (ttick, rtick) = mpsc::channel::<TickTimer>();
    let (tplay, rplay) = mpsc::channel::<Event>();
    let latch1 = Arc::new(AtomicBool::new(true));
    let latch2 = latch1.clone();
    let th = thread::spawn(move || -> Result<()> {
        let ticker = Ticker::new(duration, MSEC_TICKER_RATE, ttick, rplay, latch1);
        ticker.run()?;
        Ok(())
    });

    let stdin = stdin();
    let mut screen = screen::AlternateScreen::from(stdout().into_raw_mode()?);
    let wh = thread::spawn(move || -> Result<()> {
        write!(screen, "{}{}", clear::CurrentLine, cursor::Hide)?;
        loop {
            if let Ok(t) = rtick.recv() {
                if t.rest() > duration {
                    return Ok(());
                }

                write!(screen, "{}{}", clear::All, cursor::Goto(1, 1))?;
                write!(screen, "{}", f(t.rest()))?;
                screen.flush()?;

                thread::sleep(Duration::from_millis(MSEC_PER_FLAME));
            } else {
                write!(screen, "{}{}", clear::CurrentLine, cursor::Show)?;
                return Ok(());
            }
        }
    });

    let mut exit_status = ExitStatus::Completed;
    for c in stdin.keys() {
        match c {
            Ok(Key::Ctrl('p')) => {
                if latch2.load(Ordering::Relaxed) {
                    latch2.store(false, Ordering::Relaxed);
                    tplay.send(Event::Pose)?;
                } else {
                    latch2.store(true, Ordering::Relaxed);
                    tplay.send(Event::Play)?;
                }
            }
            Ok(Key::Ctrl('s')) | Ok(Key::Ctrl('c')) | Ok(Key::Ctrl('d')) => {
                latch2.store(false, Ordering::Relaxed);
                tplay.send(Event::Stop)?;
                exit_status = ExitStatus::Terminated;
                break;
            }
            _ => {}
        }
    }

    match th.join() {
        Err(_) => return Err(anyhow!("failed to join ticker handler")),
        Ok(Err(msg)) => return Err(msg),
        _ => {}
    }

    match wh.join() {
        Err(_) => return Err(anyhow!("failed to join writer handler")),
        Ok(Err(msg)) => return Err(msg),
        _ => {}
    }

    Ok(exit_status)
}
