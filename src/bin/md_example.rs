use draftsmith_render::replace_text;
use draftsmith_render::processor::Processor;
use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let orig = "my";
    let repl = "your";
    let input_path = "tests/fixtures/input_divs_code_and_inline_code.md";
    let _expected_path = "tests/fixtures/expected_output_divs_code_and_inline_code.md";

    let test_string = fs::read_to_string(input_path)?;

    // Mutable as it stores the rhai environment scope
    let mut markdown_processor = Processor::default();
    let document = markdown_processor.process(&test_string);

    println!("{document}");

    Ok(())
}

