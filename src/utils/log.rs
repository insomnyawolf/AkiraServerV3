use crate::APP_CONFIG;
use core::any::Any;
use std::fmt::Debug;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use crate::settings::debug::Debug as deb;

pub fn log<T: Any + Debug>(data: &T, color: Color) {
    let debug: &deb = &APP_CONFIG.debug;
    let intense: bool = true;
    // https://en.wikipedia.org/wiki/ANSI_escape_code
    if debug.active {
        if debug.log_to_console {
            let mut colored_stdout: StandardStream = StandardStream::stdout(ColorChoice::Always);
            colored_stdout
                .set_color(
                    ColorSpec::new()
                        .set_fg(Some(Color::Blue))
                        .set_intense(intense),
                )
                .unwrap();
            write!(&mut colored_stdout, "Debug:\t").unwrap();
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
        if debug.log_to_file {
            // Todo
        }
    }
}
