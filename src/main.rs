pub mod class_file;

use class_file::error::{Result, error};
use class_file::reader;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let str = match run() {
        Ok(str) => str,
        Err(e) => e.message
    };
    println!("{}", str)
}

fn run() -> Result<String> {
    let args: Vec<String> = env::args().collect();
    let file_name = match args.len() {
        2 => Ok(args[1].to_owned()),
        _ => error("usage: cargo run YourClass.class".to_string()),
    }?;
    let path = Path::new(&file_name);
    let mut file = File::open(&path).or_else(|e| error(e.to_string()))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).or_else(|e| error(e.to_string()))?;
    let class_file = reader::read_class_file(buffer)?;
    Ok(format!("{}", class_file))
}