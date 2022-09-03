use std::time::Duration;

const TOMATO: char = '\u{1F345}';

pub(crate) fn format(rest: Duration) -> String {
    format!(
        "{tomato:} rest: {min:}:{sec:02}",
        tomato = TOMATO,
        min = rest.as_secs() / 60,
        sec = rest.as_secs() % 60
    )
}
