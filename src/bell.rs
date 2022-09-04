const BEL: char = '\u{07}';

pub(crate) fn beep() {
    print!("{}", BEL);
}
