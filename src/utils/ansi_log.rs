extern crate ansi_term;

use ansi_term::{Colour, Style};

pub(crate) fn ansi_print(text: &str, color: Colour) {
    let text = color.paint(text);
    println!("{}", text);
}

pub(crate) fn warning(message: &str) {
    let warning_message = "⚠ Warning:";
    let styled_warning = Style::new()
        .on(Colour::Yellow)
        .bold()
        .paint(warning_message);
    println!(
        "{} {}",
        styled_warning,
        ansi_term::Colour::Yellow.paint(message)
    );
}

pub(crate) fn error(message: &str) {
    let prefix = "⛔ Error:";
    let styled_error = Style::new().on(Colour::Red).bold().paint(prefix);
    println!("{} {}", styled_error, ansi_term::Colour::Red.paint(message));
}

pub(crate) fn success(message: &str) {
    let prefix = "✔ Success:";
    let styled_error = Style::new().on(Colour::Green).bold().paint(prefix);
    println!(
        "{} {}",
        styled_error,
        ansi_term::Colour::Green.paint(message)
    );
}

pub(crate) fn debug(message: &str) {
    let prefix = "ℹ Debug:";
    let styled_error = Style::new().on(Colour::White).bold().paint(prefix);
    println!(
        "{} {}",
        styled_error,
        ansi_term::Colour::Black.paint(message)
    );
}
