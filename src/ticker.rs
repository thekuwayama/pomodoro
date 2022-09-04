use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::{Duration, Instant};

use anyhow::Result;

use crate::event::Event;

pub(crate) struct TickTimer(Duration);

impl TickTimer {
    pub(crate) fn rest(&self) -> Duration {
        self.0
    }
}

pub(crate) struct Ticker {
    duration: Duration,
    tick_rate: u64,
    ttick: mpsc::Sender<TickTimer>,
    rplay: mpsc::Receiver<Event>,
    latch: Arc<AtomicBool>,
}

impl Ticker {
    pub(crate) fn new(
        duration: Duration,
        tick_rate: u64,
        ttick: mpsc::Sender<TickTimer>,
        rplay: mpsc::Receiver<Event>,
        latch: Arc<AtomicBool>,
    ) -> Self {
        Self {
            duration,
            tick_rate,
            ttick,
            rplay,
            latch,
        }
    }

    pub(crate) fn run(&self) -> Result<()> {
        let mut duration = self.duration;
        let mut end = Instant::now() + self.duration;
        loop {
            if !self.latch.load(Ordering::Relaxed) {
                match self.rplay.recv() {
                    Ok(Event::Play) => {
                        self.latch.store(true, Ordering::Relaxed);
                        end = Instant::now() + duration;
                    }
                    Ok(Event::Pose) => {
                        self.latch.store(false, Ordering::Relaxed);
                        duration = end - Instant::now();
                        continue;
                    }
                    Ok(Event::Stop) => return Ok(()),
                    _ => continue,
                }
            }

            self.ttick.send(TickTimer(end - Instant::now()))?;
            if end < Instant::now() {
                return Ok(());
            }

            thread::sleep(Duration::from_millis(self.tick_rate));
        }
    }
}
