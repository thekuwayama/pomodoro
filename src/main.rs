use std::time::Duration;

mod event;
mod timer;

fn main() {
    let duration = Duration::from_secs(25 * 60);
    timer::start(duration);
}
