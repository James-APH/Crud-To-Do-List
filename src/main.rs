use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write, prelude::*};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
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
        if todos.is_empty() {
            println!("To-do list is empty.");
            return;
        }
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(50)
            .set_header(vec![
                Cell::new("Index:")
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Yellow),
                Cell::new("To-do:")
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Yellow),
            ]);
        for (index, todo) in todos.iter().enumerate() {
            table.add_row(vec![format!("{index}."), format!("{todo}")]);
        }
        println!("{table}");
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
        writeln!(write_file, "{line}")?;
    }

    Ok(())
}
