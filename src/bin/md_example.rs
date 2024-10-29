use draftsmith_render::replace_text;

fn main() {
    let orig = "my";
    let repl = "your";
    let test_string = std::fs::read_to_string("tests/fixtures/input_divs_code_and_inline_code.md").unwrap();
    let _expected = std::fs::read_to_string("tests/fixtures/expected_output_divs_code_and_inline_code.md").unwrap().trim_end_matches('\n').to_string();
    let html = replace_text(&test_string, &orig, &repl);

    println!("{}", html);
}
