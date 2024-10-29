use draftsmith_render::replace_text;
use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let orig = "my";
    let repl = "your";
    let input_path = "tests/fixtures/input_divs_code_and_inline_code.md";
    let expected_path = "tests/fixtures/expected_output_divs_code_and_inline_code.md";

    let test_string = fs::read_to_string(input_path)?;
    let expected = fs::read_to_string(expected_path)?.trim_end_matches('\n').to_string();
    
    let html = replace_text(&test_string, orig, repl);

    println!("Generated HTML:\n{}", html);
    
    if html == expected {
        println!("Output matches expected result.");
    } else {
        println!("Output does not match expected result.");
        println!("Expected:\n{}", expected);
    }

    Ok(())
}
