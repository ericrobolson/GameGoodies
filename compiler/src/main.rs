use ansi_term::Colour;
use std::error::Error;
use std::{env, path::PathBuf};
use walkdir::WalkDir;

mod environment;
mod ir_generation;
mod lexer;
mod parser;
mod syntax_analysis;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Expected path to data directory!");
    }

    let data_path = args[1].clone();
    let files = source_data(&data_path);

    let mut env = environment::Environment::new();
    env.compile(files);
}

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub path: PathBuf,
    pub contents: String,
}

fn source_data(dir: &str) -> Vec<File> {
    let mut data = vec![];

    for entry in WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        let path = entry.path().to_path_buf();

        // Check if it should be added
        if f_name.ends_with(".blood") {
            data.push(path.clone());
        }
    }

    data.sort();

    let data = data
        .iter()
        .map(|p| File {
            name: format!("{:?}", p.file_name().unwrap_or_default()),
            path: p.clone(),
            contents: std::fs::read_to_string(p).unwrap(),
        })
        .collect();

    data
}
