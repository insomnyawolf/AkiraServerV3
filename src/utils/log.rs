// Time
extern crate chrono;

use crate::APP_CONFIG;
use core::any::Any;
use std::fmt::Debug;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn do_log_debug<T: Any + Debug>(tag: &str, data: &T, color: Color, intense: bool) {
    // https://en.wikipedia.org/wiki/ANSI_escape_code
    if APP_CONFIG.debug.active {
        if APP_CONFIG.debug.log_to_console {
            let mut colored_stdout: StandardStream = StandardStream::stdout(ColorChoice::Always);
            colored_stdout
                .set_color(
                    ColorSpec::new()
                        .set_fg(Some(Color::Blue))
                        .set_intense(intense),
                )
                .unwrap();
            write!(&mut colored_stdout, "{}\t", tag).unwrap();
            colored_stdout
                .set_color(
                    ColorSpec::new()
                        .set_fg(Some(Color::Green))
                        .set_intense(intense),
                )
                .unwrap();
            writeln!(&mut colored_stdout, "{}", chrono::Local::now()).unwrap();
            colored_stdout
                .set_color(ColorSpec::new().set_fg(Some(color)).set_intense(intense))
                .unwrap();
            writeln!(&mut colored_stdout, "{:#?}", data).unwrap();
            colored_stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::White)))
                .unwrap();
        }
        if APP_CONFIG.debug.log_to_file {
            // Todo
        }
    }
}

pub fn log_custom<T: Any + Debug>(tag: &str, data: &T, color: Color, intense: bool) {
    do_log_debug(tag, data, color, intense);
}

pub fn log_verbose<T: Any + Debug>(data: &T) {
    if APP_CONFIG.debug.verbose {
        do_log_debug(&"Verbose:", data, Color::Cyan, true);
    }
}

pub fn log_warning<T: Any + Debug>(data: &T) {
    if APP_CONFIG.debug.warning {
        do_log_debug(&"Warning:", data, Color::Yellow, true);
    }
}

pub fn log_error<T: Any + Debug>(data: &T) {
    if APP_CONFIG.debug.error {
        do_log_debug(&"Error:", data, Color::Red, true);
    }
}
