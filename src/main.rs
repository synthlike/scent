use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};
use scent::{
    analysis::Analysis,
    parser::{parse_bytecode, print_instruction},
};

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
        Commands::Disasm { path } => {
            let bytes = read_hex_file(&path).unwrap_or_else(|e| {
                eprintln!("{}", e);
                std::process::exit(1);
            });

            let instructions = parse_bytecode(bytes);
            for inst in instructions {
                print_instruction(&inst);
            }
        }
        Commands::Funcs { path } => {
            let bytes = read_hex_file(&path).unwrap_or_else(|e| {
                eprintln!("{}", e);
                std::process::exit(1);
            });

            let instructions = parse_bytecode(bytes);
            let analysis = Analysis::from_instructions(&instructions);

            for func in analysis.functions {
                println!("{:?}", &func) // TODO: proper print
            }
        }
    }
}
