use std::process;
use std::time::Duration;

mod cli;
mod event;
mod format;
mod ticker;
mod timer;

fn main() {
    let matches = cli::build().get_matches();
    let working_time = matches
        .value_of(cli::WORKING_TIME)
        .unwrap()
        .parse::<u64>()
        .unwrap_or_else(|_| {
            eprintln!("failed, <{}> should be integer", cli::WORKING_TIME);
            process::exit(1);
        });
    let break_time = matches
        .value_of(cli::BREAK_TIME)
        .unwrap()
        .parse::<u64>()
        .unwrap_or_else(|_| {
            eprintln!("failed, <{}> should be integer", cli::BREAK_TIME);
            process::exit(1);
        });

    match timer::run_working(Duration::from_secs(working_time * 60)) {
        Ok(timer::ExitStatus::Terminated) => return,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
        _ => {}
    }
    timer::run_break(Duration::from_secs(break_time * 60)).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1);
    });
}
