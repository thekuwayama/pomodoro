use clap::{crate_description, crate_name, crate_version, Arg, Command};

pub(crate) const WORKING_TIME: &str = "WORKING_TIME";
pub(crate) const BREAK_TIME: &str = "BREAK_TIME";

pub(crate) fn build() -> Command<'static> {
    Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::new(WORKING_TIME)
                .help("working time (minutes)")
                .default_value("25")
                .required(false),
        )
        .arg(
            Arg::new(BREAK_TIME)
                .help("break time (minutes)")
                .default_value("5")
                .required(false),
        )
}
