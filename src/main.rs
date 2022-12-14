use std::{process, time::Duration};

mod bell;
mod cli;
mod event;
mod format;
mod ticker;
mod timer;

fn main() {
    let matches = cli::build().get_matches();
    let working_time = matches
        .get_one::<String>(cli::WORKING_TIME)
        .unwrap()
        .parse::<u64>()
        .unwrap_or_else(|_| {
            eprintln!("failed, <{}> should be integer", cli::WORKING_TIME);
            process::exit(1);
        });
    let break_time = matches
        .get_one::<String>(cli::BREAK_TIME)
        .unwrap()
        .parse::<u64>()
        .unwrap_or_else(|_| {
            eprintln!("failed, <{}> should be integer", cli::BREAK_TIME);
            process::exit(1);
        });
    let cycle = matches
        .get_one::<String>(cli::CYCLE)
        .unwrap()
        .parse::<u64>()
        .unwrap_or_else(|_| {
            eprintln!("failed, <{}> should be integer", cli::CYCLE);
            process::exit(1);
        });

    const SECS_OF_MINUTE: u64 = 60;
    for _ in 0..cycle {
        match timer::run_working(Duration::from_secs(working_time * SECS_OF_MINUTE)) {
            Ok(timer::ExitStatus::Terminated) => return,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
            _ => {}
        }
        match timer::run_break(Duration::from_secs(break_time * SECS_OF_MINUTE)) {
            Ok(timer::ExitStatus::Terminated) => return,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
            _ => {}
        }
    }
}
