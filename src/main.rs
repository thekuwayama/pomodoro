use std::process;
use std::time::Duration;

mod event;
mod ticker;
mod timer;

fn main() {
    let duration = Duration::from_secs(25 * 60);
    timer::start(duration).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        process::exit(1);
    });
}
