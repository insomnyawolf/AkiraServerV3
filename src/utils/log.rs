use std::any::Any;
use std::fmt::Debug;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn log<T: Any + Debug>(data: &T, color: Color, intense: bool, to_console: bool, to_file: bool) {
    if to_console {
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
                    .set_fg(Some(Color::Cyan))
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
    if to_file {
        // Todo
    }
}
