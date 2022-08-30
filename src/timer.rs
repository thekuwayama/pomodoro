use std::thread;
use std::time::{Duration, Instant};

use console::Term;

pub(crate) fn print_loop(start: &Instant, period: u64, interval: u64) {
    let term = Term::stdout();
    loop {
        let elapsed = start.elapsed();
        if elapsed.as_secs() > period * 60 {
            return;
        }
        let s = format!("start: {:?}, elapsed: {:?}", start, elapsed);
        term.write_line(&s).unwrap();
        thread::sleep(Duration::from_millis(interval));
        term.move_cursor_up(1).unwrap();
        term.clear_line().unwrap();
    }
}
