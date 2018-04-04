extern crate termcolor;
use std::io::{Write};
use self::termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn get_true_white() -> Color {
	Color::Rgb(255, 255, 255)
}

fn get_true_red() -> Color {
	Color::Rgb(255, 0, 0)
}

fn get_true_blue() -> Color {
	Color::Cyan
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