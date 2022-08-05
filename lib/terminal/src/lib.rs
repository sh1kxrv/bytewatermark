use std::io;

use colored::Colorize;

pub fn input(message: &str) -> String {
  let mut line = String::new();
  print!("{}", message);
  io::Write::flush(&mut io::stdout()).expect("Flush Error");
  io::stdin().read_line(&mut line).expect("Wtf");
  line
}

pub fn warn(message: &str) {
  println!("{} {}", "[WARN]".bright_yellow(), message.bright_yellow());
}

pub fn success(message: &str) {
  println!("{} {}", "[SUCCESS]".bright_green(), message.bright_green());
}

pub fn debug(message: &str) {
  println!("{} {}", "[DEBUG]".bright_white(), message.bright_black());
}

pub fn error(message: &str) {
  println!("{} {}", "[ERROR]".bright_red(), message.bright_red());
}
