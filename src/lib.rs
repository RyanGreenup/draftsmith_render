use comrak::nodes::NodeValue;
use comrak::{format_html, parse_document, Arena, Options};
use comrak::{ComrakOptions, ExtensionOptions, ParseOptions, RenderOptions};

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

    println!("{:#?}", root);

    let mut html = vec![];
    format_html(root, &Options::default(), &mut html).unwrap();

    String::from_utf8(html).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
