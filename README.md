# Directory Builder

A simple rust based CLI tool to create directory structures from a JSON file.

## Overview

**Directory Builder** is a command-line tool that reads a JSON file representing a directory structure and recreates that structure on your filesystem. This tool is ideal for:

- Setting up project templates.
- Scaffolding directories for new projects.
- Automating the creation of complex file hierarchies.


## How to Run

```bash
cargo run -- --input input.json --output ./output_directory
```


## Input file

You can use the `tree` command to generate the input file in the JSON format. 

```bash
tree -J <DIR>
```

The input file is a JSON file that describes the directory and file structure you want to create. Each node in the JSON represents a file or directory. Directories can contain child nodes, while files are leaf nodes.

```json
{
  "name": "my-project",
  "type": "directory",
  "children": [
    {
      "name": "src",
      "type": "directory",
      "children": [
        {
          "name": "main.rs",
          "type": "file"
        },
        {
          "name": "lib.rs",
          "type": "file"
        }
      ]
    },
    {
      "name": "Cargo.toml",
      "type": "file"
    },
    {
      "name": "README.md",
      "type": "file"
    }
  ]
}
```







