use std::time::Instant;

mod timer;

fn main() {
    let now = Instant::now();
    timer::start(&now, 25)
}
