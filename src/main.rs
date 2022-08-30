use std::thread;
use std::time::{Duration, Instant};

use console::Term;

fn print_loop(start: &Instant, interval: u64) -> ! {
    let term = Term::stdout();
    loop {
        let s = format!("start: {:?}, elapsed: {:?}", start, start.elapsed());
        term.write_line(&s).unwrap();
        thread::sleep(Duration::from_millis(interval));
        term.move_cursor_up(1).unwrap();
        term.clear_line().unwrap();
    }
}

fn main() {
    let now = Instant::now();
    print_loop(&now, 1000)
}
