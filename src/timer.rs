use std::{
    io::{stdin, stdout, Write},
    marker::Send,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc,
    },
    thread,
    time::Duration,
};

use anyhow::{anyhow, Result};
use termion::{
    clear, color, cursor, event::Key, input::TermRead, raw::IntoRawMode, screen, terminal_size,
};

use crate::bell;
use crate::event::Event;
use crate::format;
use crate::ticker::{TickTimer, Ticker};

const MSEC_PER_FLAME: u64 = 500;
const MSEC_TICKER_RATE: u64 = 1000;
const MENU_BAR: &str = "Pomodoro Timer | Ctrl-c: Exit | Ctrl-s: Exit | Ctrl-p: Pose/Play";

#[derive(PartialEq)]
pub(crate) enum ExitStatus {
    Completed,
    Terminated,
}

pub(crate) fn run_working(duration: Duration) -> Result<ExitStatus> {
    run(format::working_format, duration, color::Bg(color::LightRed))
}

pub(crate) fn run_break(duration: Duration) -> Result<ExitStatus> {
    run(format::break_format, duration, color::Bg(color::LightCyan))
}

fn run<F: Fn(Duration) -> String + Send + 'static, C: color::Color + Send + 'static>(
    f: F,
    duration: Duration,
    bg: color::Bg<C>,
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

    let mut screen = screen::AlternateScreen::from(stdout().into_raw_mode()?);
    let wh = thread::spawn(move || -> Result<()> {
        write!(
            screen,
            "{}{}{}",
            clear::CurrentLine,
            cursor::Hide,
            color::Fg(color::LightWhite)
        )?;
        loop {
            if let Ok(t) = rtick.recv() {
                if t.rest() > duration {
                    return Ok(());
                }

                show(&mut screen, &t, &f, &bg)?;
                thread::sleep(Duration::from_millis(MSEC_PER_FLAME));
            } else {
                write!(
                    screen,
                    "{}{}{}",
                    clear::CurrentLine,
                    cursor::Show,
                    color::Fg(color::Reset)
                )?;
                return Ok(());
            }
        }
    });

    let terminated1 = Arc::new(AtomicBool::new(false));
    let terminated2 = terminated1.clone();
    thread::spawn(move || -> Result<()> {
        for c in stdin().keys() {
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
                    terminated2.store(true, Ordering::Relaxed);
                    break;
                }
                _ => {}
            }
        }
        Ok(())
    });

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

    if terminated1.load(Ordering::Relaxed) {
        return Ok(ExitStatus::Terminated);
    }

    bell::beep();
    Ok(ExitStatus::Completed)
}

fn show<W: Write, F: Fn(Duration) -> String + Send + 'static, C: color::Color + 'static>(
    screen: &mut W,
    t: &TickTimer,
    f: &F,
    bg: &color::Bg<C>,
) -> Result<()> {
    let (w, _) = terminal_size()?;
    let menu_bar = if MENU_BAR.len() > w as usize {
        &MENU_BAR[..w as usize]
    } else {
        MENU_BAR
    };
    write!(screen, "{}{}", clear::All, cursor::Goto(1, 1))?;
    write!(
        screen,
        "{}{:width$}{}",
        bg,
        menu_bar,
        color::Bg(color::Reset),
        width = w as usize,
    )?;
    write!(screen, "{}", cursor::Goto(1, 3))?;
    write!(screen, "{}", f(t.rest()))?;
    screen.flush()?;

    Ok(())
}
