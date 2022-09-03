use std::time::Duration;

const TOMATO: char = '\u{1F345}';
const COFFEE: char = '\u{2615}';

pub(crate) fn working_format(rest: Duration) -> String {
    do_format(TOMATO, rest)
}

pub(crate) fn break_format(rest: Duration) -> String {
    do_format(COFFEE, rest)
}

fn do_format(icon: char, rest: Duration) -> String {
    format!(
        "{icon:} rest: {min:}:{sec:02}",
        icon = icon,
        min = rest.as_secs() / 60,
        sec = rest.as_secs() % 60
    )
}
