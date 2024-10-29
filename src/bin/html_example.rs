use draftsmith_render::parse_md_to_html;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = "tests/fixtures/input_divs_code_and_inline_code.md";
    let _expected_path = "tests/fixtures/expected_output_divs_code_and_inline_code.md";

    let test_string = fs::read_to_string(input_path)?;

    // Create the html
    let html = parse_md_to_html(&test_string);

    println!("{html}");

    Ok(())
}
