use std::time::Instant;

mod event;
mod input;
mod timer;

fn main() {
    let now = Instant::now();
    timer::start(&now, 25)
}
