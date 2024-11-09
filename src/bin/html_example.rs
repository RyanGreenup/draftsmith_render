use draftsmith_render::parse_md_to_html;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    // Get command-line arguments
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        return Err("Please provide a filepath as the first argument".into());
    }

    // Use the second argument (index 1) as the file path
    let input_path = &args[1];

    // Read the file content into a string
    let test_string = fs::read_to_string(input_path)?;

    // Create the HTML from the markdown string
    let html = parse_md_to_html(&test_string);

    println!("{}", html);

    Ok(())
}
