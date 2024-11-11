pub mod processor;

use comrak::nodes::NodeValue;
// https://raw.githubusercontent.com/kivikakk/comrak/f4853af61978e90d73f3b8c9a63be186d85c1e5c/examples/syntect.rs
use comrak::plugins::syntect::SyntectAdapterBuilder;
use comrak::{format_html_with_plugins, parse_document, Arena, Options, Plugins};
use processor::{CustomFn, Processor};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Configures the Comrak options for Markdown parsing and rendering.
///
/// This function sets various extension and rendering options to customize
/// the behavior of the Markdown parser and HTML renderer.
///
/// # Arguments
///
/// * `options` - A mutable reference to a Comrak `Options` struct.
fn config_opts(options: &mut Options) {
    options.extension.autolink = true;
    options.extension.footnotes = true;
    options.extension.greentext = true;
    options.extension.math_dollars = true;
    options.extension.spoiler = true;
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.tagfilter = false;
    options.extension.tasklist = true;
    options.extension.underline = true;
    options.extension.wikilinks_title_after_pipe = true;
    options.render.figure_with_caption = true;
    options.render.full_info_string = true;
    options.render.github_pre_lang = true;
    options.render.unsafe_ = true;
}

pub fn process_md(document: &str, functions: Option<Vec<CustomFn>>) -> String {
    let mut processor = Processor::new(functions);
    processor.process(document)
}

/// Parses a Markdown document and converts it to HTML.
///
/// This function takes a Markdown string as input, processes it using custom rules,
/// and then converts it to HTML using the Comrak library with customized options.
///
/// # Arguments
///
/// * `document` - A string slice that holds the Markdown content to be parsed.
///
/// # Returns
///
/// A `String` containing the parsed and formatted HTML output.
///
/// # Process
///
/// 1. Creates an Arena for memory management.
/// 2. Configures Comrak options using `config_opts`.
/// 3. Preprocesses the document using a custom `Processor`.
/// 4. Parses the preprocessed document into an AST.
/// 5. Iterates over the AST, applying custom transformations (e.g., for math elements).
/// 6. Formats the modified AST into HTML.
///
/// # Note
///
/// This function uses unsafe Rust features through the Comrak library's options.
pub fn parse_md_to_html(document: &str, functions: Option<Vec<CustomFn>>) -> String {
    // Set up plugins for syntax highlighting
    // let adapter = builder.build();
    let _options = Options::default();
    let mut plugins = Plugins::default();

    // The returned nodes are created in the supplied Arena, and are bound by its lifetime.
    let arena = Arena::new();

    // Configure the options
    let mut options = Options::default();
    config_opts(&mut options);

    // Preprocess the document
    let document = process_md(document, functions);
    let document = document.as_str();

    // get the AST
    let root = parse_document(&arena, document, &options);

    // Iterate over all the descendants of root.
    for node in root.descendants() {
        /*
        // Left in for reference
        if let NodeValue::Text(ref mut text) = node.data.borrow_mut().value {
            // If the node is a text node, perform the string replacement.
            *text = text.replace(orig_string, replacement);
        }
        */

        // handle math (efficiently)
        if let NodeValue::Math(ref mut math) = node.data.borrow_mut().value {
            // Capture the current math literal
            let math_literal = &mut math.literal;

            // Determine the appropriate prefix and calculate the total length upfront
            let prefix = if math.display_math { "$$" } else { "$" };
            let prefix_len = prefix.len();
            let total_len = prefix_len + math_literal.len() + prefix_len;

            // Reserve the capacity to avoid multiple allocations
            math_literal.reserve_exact(total_len - math_literal.len());

            // Use `insert_str` to prepend and append the prefix directly to the literal
            math_literal.insert_str(0, prefix);
            math_literal.push_str(prefix);
        }
    }

    // println!("{:#?}", root);

    let mut html = vec![];
    // format_html(root, &options, &mut html).unwrap();
    let builder = SyntectAdapterBuilder::new().theme("base16-ocean.dark");
    let adapter = builder.build();
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    format_html_with_plugins(root, &options, &mut html, &plugins).expect("Failed to format HTML");
    String::from_utf8(html).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_processor_output() {
        let test_string = fs::read_to_string("tests/fixtures/input_divs_code_and_inline_code.md")
            .expect("Failed to read input fixture");

        let expected = fs::read_to_string(
            "tests/fixtures/expected_output_divs_code_and_inline_code_html.html",
        )
        .expect("Failed to read expected output fixture")
        .trim_end_matches('\n')
        .to_string();

        // Create the HTML
        let result = parse_md_to_html(&test_string, Option::None)
            .trim_end_matches('\n')
            .to_string();

        // save the file
        std::fs::write("/tmp/f.html", result.clone()).expect("Unable to write file");

        assert_eq!(
            result, expected,
            "Processor output did not match expected output"
        );
    }
}
