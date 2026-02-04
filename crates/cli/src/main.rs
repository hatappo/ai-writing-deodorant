use clap::Parser;
use deo_core::process_text;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "deo")]
#[command(about = "Remove AI-like formatting from text files")]
#[command(version)]
struct Cli {
    /// Input file path (use '-' for stdin)
    #[arg(value_name = "FILE")]
    input: PathBuf,

    /// Remove emoji characters
    #[arg(long)]
    emoji: bool,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let content = if cli.input.to_string_lossy() == "-" {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    } else {
        fs::read_to_string(&cli.input)?
    };

    let result = process_text(&content, cli.emoji);
    print!("{}", result);

    Ok(())
}
