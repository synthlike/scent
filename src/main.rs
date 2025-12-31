use std::{fs, path::PathBuf};

use clap::Parser;
use scent::{loader::Program, selectors::load_selectors, view::View};

#[derive(Parser)]
struct Cli {
    #[arg(value_name = "PATH")]
    path: PathBuf,

    /// Raw bytecode input
    #[arg(long)]
    raw: bool,

    /// Runtime bytecode input
    #[arg(long)]
    runtime: bool,

    /// Decorate push data and labels
    #[arg(long)]
    decorated: bool,

    /// Selectors list as JSON (implies --decorated)
    #[arg(long)]
    selectors: Option<PathBuf>,
}

fn read_hex_file(path: &PathBuf) -> Result<Vec<u8>, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("failed to read {}: {}", path.display(), e))?;

    let content = content.trim().strip_prefix("0x").unwrap_or(content.trim());

    hex::decode(content).map_err(|e| format!("failed to parse hex: {}", e))
}

fn main() {
    let cli = Cli::parse();

    let bytes = read_hex_file(&cli.path).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });

    let program = Program::load(&bytes, cli.raw, cli.runtime);
    let decorated = cli.decorated || cli.selectors.is_some();
    let selectors = cli
        .selectors
        .map(|path| load_selectors(path))
        .unwrap_or_default();
    let view = View::from_program(&program, decorated, selectors);
    print!("{}", view);
}
