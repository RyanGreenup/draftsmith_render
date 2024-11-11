use clap::Parser;
use draftsmith_render::processor::Processor;
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Input Markdown file
    #[clap(short, long)]
    input: Option<PathBuf>,

    /// Output file
    #[clap(short, long)]
    output: Option<PathBuf>,

    /// Output format (html or markdown)
    #[clap(short, long, default_value = "html")]
    format: String,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    // Read input from file or stdin
    let input = if let Some(input_path) = &cli.input {
        fs::read_to_string(input_path)?
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    };

    let output_content = match cli.format.as_str() {
        "html" => {
            // Convert Markdown to HTML
            draftsmith_render::parse_md_to_html(&input, Option::None)
        }
        "markdown" | "md" => {
            // Assuming `Processor` can process and convert input to markdown if necessary.
            let mut processor = Processor::default();
            processor.process(&input)
        }
        _ => {
            eprintln!("Unsupported format: {}", cli.format);
            return Ok(());
        }
    };

    // Write output to file or stdout
    if let Some(output_path) = &cli.output {
        fs::write(output_path, &output_content)?;
    } else {
        io::stdout().write_all(output_content.as_bytes())?;
    }

    Ok(())
}
