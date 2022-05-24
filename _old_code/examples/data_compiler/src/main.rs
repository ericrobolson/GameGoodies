use ansi_term::Colour;
use std::error::Error;
use std::{env, path::PathBuf};
use walkdir::WalkDir;

mod character_info;
use character_info::CharacterInfo;
mod entity;
use entity::Entity;

#[derive(PartialEq, Debug, Eq, PartialOrd, Ord)]
enum DataType {
    CharacterSheet,
    Entity,
}

impl DataType {
    fn parse_path(f_name: &std::borrow::Cow<str>) -> Option<Self> {
        if f_name.ends_with(CharacterInfo::file_extension()) {
            Some(DataType::CharacterSheet)
        } else if f_name.ends_with(Entity::file_extension()) {
            Some(DataType::Entity)
        } else {
            None
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Expected path to data directory!");
    }

    let data_path = args[1].clone();

    let mut ctx = Ctx::new();
    for (data_type, path) in source_data(&data_path) {
        if validate_data(&path, data_type, &mut ctx) {}
    }

    ctx.output();
}

fn source_data(dir: &str) -> Vec<(DataType, PathBuf)> {
    let mut data = vec![];

    for entry in WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        let path = entry.path().to_path_buf();

        // Check if it should be added
        if let Some(data_type) = DataType::parse_path(&f_name) {
            data.push((data_type, path.clone()));
        }
    }

    data.sort();
    data
}

fn validate_data(path: &PathBuf, data_type: DataType, ctx: &mut Ctx) -> bool {
    match data_type {
        DataType::CharacterSheet => CharacterInfo::validate(&path, ctx),
        DataType::Entity => Entity::validate(path, ctx),
    }
}

/// A data resource that can be executed.
pub trait DataResource {
    fn file_extension() -> &'static str;
    fn validate(path: &PathBuf, ctx: &mut Ctx) -> bool;
}

pub struct ResourceState {
    location: PathBuf,
    resource: String,
    err_msgs: Vec<String>,
}

impl ResourceState {
    pub fn new(location: PathBuf, resource: String) -> Self {
        Self {
            location,
            resource,
            err_msgs: vec![],
        }
    }

    pub fn add_err(&mut self, err: String) {
        self.err_msgs.push(err);
    }

    pub fn is_valid(&self) -> bool {
        self.err_msgs.is_empty()
    }
}

pub struct Ctx {
    resources: Vec<ResourceState>,
}

impl Ctx {
    pub fn new() -> Self {
        Self { resources: vec![] }
    }

    pub fn log_resource(&mut self, resource: ResourceState) {
        self.resources.push(resource);
    }

    pub fn output(&mut self) {
        self.resources
            .sort_by(|a, b| a.err_msgs.len().partial_cmp(&b.err_msgs.len()).unwrap());

        for resource in &self.resources {
            if resource.is_valid() {
                print_ok(&resource.location);
                continue;
            }

            println!(
                "{} - {}",
                Colour::Red.paint("ERROR"),
                Colour::Red.paint(&format!("{:?}", resource.location)),
            );

            for msg in &resource.err_msgs {
                println!("- {}", msg);
            }
        }
    }
}

pub fn print_error(path: &PathBuf, msg: &Box<Error>) {
    let err = Colour::Red.paint("ERROR");
    println!("{} on {:?}: {:?}", err, path, msg);
}

pub fn print_ok(path: &PathBuf) {
    let green = Colour::Green.paint("VALID");
    println!("{} - {:?}", green, path);
}
