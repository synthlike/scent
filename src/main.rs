use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};
use scent::{loader::Program, view::View};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Disasm {
        #[arg(value_name = "PATH")]
        path: PathBuf,

        /// Raw bytecode input
        #[arg(long)]
        raw: bool,

        /// Runtime bytecode input
        #[arg(long)]
        runtime: bool,

        /// Decorate push data
        #[arg(long)]
        decorated: bool,
    },
    Funcs {
        #[arg(value_name = "PATH")]
        path: PathBuf,
    },
}

fn read_hex_file(path: &PathBuf) -> Result<Vec<u8>, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("failed to read {}: {}", path.display(), e))?;

    let content = content.trim().strip_prefix("0x").unwrap_or(content.trim());

    hex::decode(content).map_err(|e| format!("failed to parse hex: {}", e))
}

fn main() {
    let cli = Cli::parse();

    match cli.cmd {
        Commands::Disasm {
            path,
            raw,
            runtime,
            decorated,
        } => {
            let bytes = read_hex_file(&path).unwrap_or_else(|e| {
                eprintln!("{}", e);
                std::process::exit(1);
            });

            let program = Program::load(&bytes, raw, runtime);
            let view = View::from_program(&program, decorated);
            print!("{}", view);
        }
        Commands::Funcs { .. } => {
            // XXX: removed until disasm is in decent shape
            todo!()
        }
    }
}
