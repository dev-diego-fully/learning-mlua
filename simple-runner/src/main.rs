use clap::{Parser, command};
use mlua::prelude::*;
use std::fs;

fn main() {
    let args = Args::parse();
    if let Err(msg) = file_content(&args.file).map(run_script) {
        println!("{}", msg);
    };
}

fn file_content(path: &str) -> Result<String, String> {
    match fs::read_to_string(path) {
        Ok(v) => Ok(v),
        Err(_) => Err(format!("Failed to load file: {}", path)),
    }
}

fn run_script(script: String) -> Result<(), String> {
    match Lua::new().load(script).exec() {
        Err(_) => Err("Failed to execute script content".to_owned()),
        _ => Ok(()),
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(help = "Path of lua script.")]
    file: String,
}
