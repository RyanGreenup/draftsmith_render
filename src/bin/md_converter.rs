use clap::Parser;
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Input Markdown file
    #[clap(short, long)]
    input: Option<PathBuf>,

    /// Output HTML file
    #[clap(short, long)]
    output: Option<PathBuf>,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    // Read input from file or stdin
    let input = if let Some(input_path) = cli.input {
        fs::read_to_string(input_path)?
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    };

    // Convert Markdown to HTML
    let html_output = draftsmith_render::parse_md_to_html(&input);

    // Write output to file or stdout
    if let Some(output_path) = cli.output {
        fs::write(output_path, html_output)?;
    } else {
        io::stdout().write_all(html_output.as_bytes())?;
    }

    Ok(())
}
