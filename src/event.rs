use std::time::Instant;

pub(crate) enum Event {
    StopTimer,
    TickTimer(Instant),
}
