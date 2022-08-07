use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

const PROGRAM_VERSION: &str = env!("CARGO_PKG_VERSION");
const IS_DEV: bool = true;

fn resolve_path(raw_path: &String) -> PathBuf {
  let path = Path::new(raw_path);
  if path.is_absolute() {
    return path.to_path_buf();
  }
  let mut cwd = env::current_dir().unwrap();
  cwd.push(path);
  cwd
}

fn validate_path(path: &PathBuf) -> Result<(), &'static str> {
  if path.to_str().is_none() {
    return Result::Err("Path is empty");
  } else if path.exists() {
    return Result::Err("File not exists");
  }
  return Result::Ok(());
}

fn read_file(filepath: PathBuf) -> Vec<u8> {
  println!("Reading file: {}", filepath.display());
  let mut file = File::open(filepath.display().to_string().trim()).unwrap();
  let mut buffer: Vec<u8> = Vec::new();
  file.read_to_end(&mut buffer).expect("Failed to read file");
  buffer
}

fn xor(raw: &Vec<u8>, password: &[u8]) -> Vec<u8> {
  let mut result: Vec<u8> = Vec::new();
  for i in 0..raw.len() {
    result.push(raw[i] ^ password[i % password.len()]);
  }
  result
}

fn generate_junk(junk_length: u8) -> Vec<u8> {
  let signature: [u8; 6] = [0x0, 0x0, 0xaf, 0xf, 0xaf, 0xF];
  let mut result: Vec<u8> = Vec::new();
  result.extend_from_slice(&signature);
  for _ in 0..junk_length {
    result.push(rand::random::<u8>());
  }
  result
}

fn read_watermark(filepath: PathBuf, password: String) {
  let mut raw = read_file(filepath);
}

fn write_watermark(filepath: PathBuf, watermark: String, password: String) {
  let mut raw = read_file(filepath);
  let mut watermark = watermark.as_bytes().to_vec();
  let password = password.as_bytes();
  watermark = xor(&watermark, password);
  let mut junk = generate_junk(8);
  raw.append(&mut junk);
  raw.append(&mut watermark);

  let watermarked_name = String::from("watermarked.bin");
  let path_watermarked = resolve_path(&watermarked_name);
  let mut watermarked = File::create(path_watermarked).unwrap();
  watermarked
    .write_all(&raw)
    .expect("Failed to write watermark");
  terminal::success("Watermarked!")
}

fn bootstrap() -> Result<(), &'static str> {
  if IS_DEV {
    terminal::debug(format!("ByteWatermark | DEV v{}", PROGRAM_VERSION).as_str());
  }
  let path_to_file = terminal::input("Enter file path > ");
  terminal::debug(format!("File path: {}", path_to_file).as_str());
  let path_to_file = resolve_path(&path_to_file);
  println!("Path: {}", path_to_file.display());

  match validate_path(&path_to_file) {
    Err(error) => {
      terminal::error(format!("Problem with validation file path: {}", error).as_str());
      return Result::Err("Validation error");
    }
    Ok(()) => (),
  }
  let mode = terminal::input("Enter available mode (read/write) > ");
  let password = terminal::input("Enter watermark password > ");

  match mode.as_str().trim() {
    "read" => {
      terminal::debug("Read");
      read_watermark(path_to_file, password);
    }
    "write" => {
      terminal::debug("Write");
      let watermark = terminal::input("Enter your watermark > ");
      write_watermark(path_to_file, watermark, password);
    }
    _ => {
      terminal::error(format!("Unknown mode: {}", mode).as_str());
    }
  }
  return Result::Ok(());
}

fn main() {
  bootstrap().expect("Failed bootstrap");
}
