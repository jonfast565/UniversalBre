extern crate termcolor;

use self::termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::io::Write;

fn get_true_white() -> Color {
    Color::Rgb(255, 255, 255)
}

fn get_true_red() -> Color {
    Color::Rgb(255, 0, 0)
}

fn get_true_blue() -> Color {
    Color::Cyan
}

fn get_true_green() -> Color {
    Color::Rgb(0, 255, 0)
}

#[allow(unused_must_use)]
pub fn log_info(info_message: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(get_true_white())));
    writeln!(&mut stdout, "{}", info_message);
    stdout.set_color(ColorSpec::new().set_fg(Some(get_true_white())));
}

#[allow(unused_must_use)]
pub fn log_debug(debug_message: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(get_true_blue())));
    writeln!(&mut stdout, "{}", debug_message);
    stdout.set_color(ColorSpec::new().set_fg(Some(get_true_white())));
}

#[allow(unused_must_use)]
pub fn log_error(error_message: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(get_true_red())));
    writeln!(&mut stdout, "{}", error_message);
    stdout.set_color(ColorSpec::new().set_fg(Some(get_true_white())));
}

#[allow(unused_must_use)]
pub fn log_success(success_message: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(get_true_green())));
    writeln!(&mut stdout, "{}", success_message);
    stdout.set_color(ColorSpec::new().set_fg(Some(get_true_white())));
}
