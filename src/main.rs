use std::time::Instant;

mod timer;

fn main() {
    let now = Instant::now();
    timer::print_loop(&now, 25, 1000)
}
