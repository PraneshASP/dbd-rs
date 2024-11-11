use serde::Deserialize;
use std::env::args;
use std::fs::{create_dir_all, read_to_string, File};
use std::io::Result;
use std::os::unix::process;
use std::path::PathBuf;
use std::process::exit;
use clap::{Command, Parser};


#[derive(Parser, Debug)]
#[command(name = "Directory Builder")]
#[command(about = "A tool to create directory structures from JSON file", long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,

    #[arg(short, long, value_name = "DIR", default_value = ".")]
    output: PathBuf,
}


#[derive(Deserialize)]
struct Node {
    name: String,
    #[serde(rename = "type")]
    node_type: String,
    children: Option<Vec<Node>>
}

fn create_structure(base_path: &PathBuf, node: &Node) -> Result<()> {
    let mut current_path = base_path.clone();
    current_path.push(&node.name);

    if node.node_type == "directory" {
        create_dir_all(&current_path)?;
        if let Some(children) = &node.children {
            for child in children {
                create_structure(&current_path, child)?;
            }
        }
    } else if node.node_type == "file" {
        if let Some(parent) = current_path.parent() {
            create_dir_all(parent)?;
        }

        if !current_path.exists() {
            File::create(&current_path)?;
        }
    } else {
        eprintln!("Unknown node type: {}", node.node_type);
    }
    println!("Directory structure created successfully.");
    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();

    if !args.input.exists() {
        eprint!("Input file doesn't exist");
        exit(1);
    }
    let input_json_str = read_to_string(&args.input)?;
    let root_node: Node = match serde_json::from_str(&input_json_str) {
        Ok(node) => node,
        Err(e) => {
            eprintln!("Unable to parse input json");
            eprintln!("{:?}", e);
            exit(1);
        }
    };

    create_structure(&args.output, &root_node)
}