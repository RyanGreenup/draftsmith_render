pub mod processor;

use comrak::nodes::NodeValue;
use comrak::{format_html, parse_document, Arena, Options};
use comrak::{ComrakOptions, ExtensionOptions, ParseOptions, RenderOptions};
use processor::Processor;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

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

pub fn replace_text(document: &str, orig_string: &str, replacement: &str) -> String {
    // The returned nodes are created in the supplied Arena, and are bound by its lifetime.
    let arena = Arena::new();

    // Configure the options
    let mut options = Options::default();
    config_opts(&mut options);

    // Preprocess the document
    let mut processor = Processor::default();
    let document = processor.process(document);
    let document = document.as_str();


    // get the AST
    let root = parse_document(&arena, document, &options);

    // Iterate over all the descendants of root.
    for node in root.descendants() {
        if let NodeValue::Text(ref mut text) = node.data.borrow_mut().value {
            // If the node is a text node, perform the string replacement.
            *text = text.replace(orig_string, replacement);
        }

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
    format_html(root, &options, &mut html).unwrap();

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

        let expected = fs::read_to_string("tests/fixtures/expected_output_divs_code_and_inline_code_html.html")
            .expect("Failed to read expected output fixture")
            .trim_end_matches('\n')
            .to_string();


        // Create the HTML
        let result = replace_text(&test_string, "", "").trim_end_matches('\n').to_string();

        // save the file
        std::fs::write("/tmp/f.html", result.clone()).expect("Unable to write file");

        assert_eq!(result, expected, "Processor output did not match expected output");
    }
}

