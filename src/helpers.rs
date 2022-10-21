use std::io::{stdin, stdout, Write};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub(crate) enum DividerType {
    Single,
    Double,
    Star,
}

pub struct TerminalDimensions {
    pub width: u16,
    pub height: u16,
}

#[macro_export]
macro_rules! clear {
    // Some weird magic characters that clear the terminal and place the cursor back on the 1st line
    () => {print!("\x1B[2J\x1B[1;1H");};
}

pub(crate) fn print_div(div_type: DividerType, term_dim: &TerminalDimensions) {
    let div_char: char;
    match div_type {
        DividerType::Single => div_char = '-',
        DividerType::Double => div_char = '=',
        DividerType::Star => div_char = '*',
    }

    let mut divider = String::new();
    for _ in 0..term_dim.width {
        divider.push(div_char);
    }

    println!("{}", divider);
}

pub(crate) fn print_centre(text: &str, term_dim: &TerminalDimensions) {
    let message_width = text.chars().count() as u16;
    let space = (term_dim.width as u16 - message_width as u16) / 2 as u16;
    let mut final_message = String::new();
    for _ in 0..space {
        final_message.push_str(" ");
    }
    final_message.push_str(text);
    println!("{}", final_message);
}

pub(crate) fn pause() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "Press any key to continue...\r\n").unwrap();
    stdout.flush().unwrap();
    stdin().keys().next();
}

