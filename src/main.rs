use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write, prelude::*};
use std::path::PathBuf;
use strum::Display;

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Display)]
enum Commands {
    Create { todo: String },
    Read,
    Update { id: u16, desc: String },
    Delete { id: u16 },
}

impl Commands {
    fn operation(&self, todos: &mut Vec<String>) {
        match self {
            Commands::Create { todo } => Self::create(todo.to_owned(), todos),
            Commands::Read => Self::read(&*todos),
            Commands::Update { id, desc } => Self::update(id.to_owned(), desc.to_owned(), todos),
            Commands::Delete { id } => Self::delete(id.to_owned(), todos),
        }
    }
    fn create(todo: String, todos: &mut Vec<String>) {
        todos.push(todo);
    }

    fn read(todos: &[String]) {
        for (index, todo) in todos.iter().enumerate() {
            println!("[{index}]: {todo}");
        }
    }

    fn update(id: u16, desc: String, todos: &mut [String]) {
        todos[id as usize] = desc;
    }

    fn delete(id: u16, todos: &mut Vec<String>) {
        todos.remove(id as usize);
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let path: PathBuf = home::home_dir()
        .expect("No home directory found")
        .join("Desktop")
        .join("todos.txt");

    let read_file = if path.exists() {
        File::open(&path).with_context(|| format!("could not read file '{}'", path.display()))?
    } else {
        File::create(&path)?
    };

    let reader = BufReader::new(read_file);
    let mut todos: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    args.cmd.operation(&mut todos);

    let mut write_file = BufWriter::new(File::create(path)?);
    for line in todos {
        writeln!(write_file, "{}", line)?;
    }

    Ok(())
}
