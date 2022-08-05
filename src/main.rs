use std::fs;
use std::path::Path;

const PROGRAM_VERSION: &str = env!("CARGO_PKG_VERSION");
const IS_DEV: bool = true;

fn validate_path(path: &String) -> Result<(), &'static str> {
  if path.is_empty() {
    return Result::Err("Path is empty");
  } else if !Path::new(path).exists() {
    return Result::Err("File not exists");
  }
  return Result::Ok(());
}

fn read_watermark(password: String, filepath: String) {}

fn write_watermark(watermark: String, password: String) {}

fn main() {
  if IS_DEV {
    terminal::debug(format!("ByteWatermark | DEV v{}", PROGRAM_VERSION).as_str());
  }
  let path_to_file = terminal::input("Enter file path > ");
  terminal::debug(format!("File path: {}", path_to_file).as_str());
  match validate_path(&path_to_file) {
    Err(error) => {
      terminal::error(format!("Problem with validation file path: {}", error).as_str());
      return;
    }
    Ok(()) => (),
  }
  let mode = terminal::input("Enter available mode (read/write) > ");
  let password = terminal::input("Enter watermark password > ");

  match mode.as_str() {
    "read" => {
      terminal::debug("Read");
      read_watermark(password, path_to_file);
    }
    "write" => {
      terminal::debug("Write");
      let watermark = terminal::input("Enter your watermark > ");
      write_watermark(watermark, password);
    }
    _ => {
      terminal::error(format!("Unknown mode: {}", mode).as_str());
    }
  }
}
