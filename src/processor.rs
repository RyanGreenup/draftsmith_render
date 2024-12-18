//! This module provides functionality for processing markdown-like text
//! with custom admonitions and code blocks.

use regex::Regex;
use rhai::packages::{BasicMathPackage, CorePackage, Package};
use rhai::{Engine, Scope};

pub type CustomFn = Box<dyn Fn(&mut Engine)>;

const ADMONITION_START_PATTERN: &str = r"^\s*:::([\w!\{\}-]+)$";
const ADMONITION_END_PATTERN: &str = r"^\s*(:::)$";
const CODE_START_PATTERN: &str = r"^\s*```\{rhai\}$";
const RHAI_DISPLAY_START_PATTERN: &str = r"^\s*```\{rhai-display\}$";
const CODE_END_PATTERN: &str = r"^\s*```$";
const LAMBDA_PATTERN: &str = r"λ#\(((?s).*?)\)#";
const TABS_START_PATTERN: &str = r"^\s*:::tabs$";

/// A processor for handling custom markdown-like syntax.
pub struct Processor<'a> {
    admonition_start_regex: Regex,
    admonition_end_regex: Regex,
    code_start_regex: Regex,
    rhai_display_start_regex: Regex,
    code_end_regex: Regex,
    lambda_regex: Regex,
    tabs_start_regex: Regex,
    div_stack: Vec<String>,
    eval_stack: bool,
    is_rhai_display: bool,
    contents: Vec<String>,
    rhai_engine: Engine,
    rhai_scope: Scope<'a>,
    in_tabs: bool,
    tab_count: usize,
    tabs_closing: bool, // Add this new field
    current_indent: String,
}

impl<'a> Processor<'a> {
    /// Creates a new Processor with optional Rhai function registrations
    pub fn new(functions: Option<Vec<CustomFn>>) -> Self {
        let mut processor = Self::default();

        // Register any provided functions with the Rhai engine
        if let Some(fns) = functions {
            for register_fn in fns {
                register_fn(&mut processor.rhai_engine);
            }
        }

        processor
    }
}

impl<'a> Default for Processor<'a> {
    fn default() -> Self {
        Self {
            admonition_start_regex: Regex::new(ADMONITION_START_PATTERN)
                .expect("Failed to compile regex"),
            admonition_end_regex: Regex::new(ADMONITION_END_PATTERN)
                .expect("Failed to compile regex"),
            code_start_regex: Regex::new(CODE_START_PATTERN).expect("Failed to compile regex"),
            rhai_display_start_regex: Regex::new(RHAI_DISPLAY_START_PATTERN)
                .expect("Failed to compile regex"),
            code_end_regex: Regex::new(CODE_END_PATTERN).expect("Failed to compile regex"),
            lambda_regex: Regex::new(LAMBDA_PATTERN).expect("Failed to compile regex"),
            tabs_start_regex: Regex::new(TABS_START_PATTERN).expect("Failed to compile regex"),
            div_stack: Vec::new(),
            eval_stack: false,
            is_rhai_display: false,
            contents: Vec::new(),
            rhai_engine: {
                let mut engine = Engine::new_raw();
                // Register the package into the 'Engine'.
                CorePackage::new().register_into_engine(&mut engine);
                BasicMathPackage::new().register_into_engine(&mut engine);
                engine
            },
            rhai_scope: Scope::new(),
            in_tabs: false,
            tab_count: 0,
            tabs_closing: false,
            current_indent: String::new(),
        }
    }
}

impl<'a> Processor<'a> {
    /// Processes the input string and returns the transformed output.
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice that holds the text to be processed.
    ///
    /// # Returns
    ///
    /// A `String` containing the processed text with custom syntax transformed.
    pub fn process(&mut self, input: &str) -> String {
        input
            .lines()
            .map(|line| self.process_line(line))
            .collect::<Vec<String>>()
            .join("")
            .trim_end_matches('\n')
            .to_string()
    }

    /// Evaluates Rhai code and returns a formatted string of the results.
    ///
    /// # Arguments
    ///
    /// * `engine` - A reference to the Rhai Engine.
    /// * `scope` - A mutable reference to the Rhai Scope.
    /// * `captured` - A string slice containing the Rhai code to evaluate.
    ///
    /// # Returns
    ///
    /// A `String` containing the formatted evaluation results.
    fn process_lambda(engine: &Engine, scope: &mut Scope, captured: &str) -> String {
        match engine.eval_with_scope::<rhai::Dynamic>(scope, captured) {
            Ok(result) => format!("{}", result),
            Err(err) => format!("Error: {}", err),
        }
    }

    /// Processes a single line of text.
    ///
    /// # Arguments
    ///
    /// * `line` - A string slice that holds the line to be processed.
    ///
    /// # Returns
    ///
    /// A `String` containing the processed line.
    fn process_line(&mut self, line: &str) -> String {
        if self.tabs_closing && self.admonition_end_regex.is_match(line) {
            self.tabs_closing = false;
            return String::new(); // Ignore the final ":::" when closing a tabs block
        }

        if self.tabs_start_regex.is_match(line) {
            self.handle_tabs_start()
        } else if self.code_start_regex.is_match(line) {
            self.handle_code_start(false)
        } else if self.rhai_display_start_regex.is_match(line) {
            self.handle_code_start(true)
        } else if self.code_end_regex.is_match(line) {
            self.handle_code_end()
        } else if self.admonition_start_regex.is_match(line) {
            if let Some(caps) = self.admonition_start_regex.captures(line) {
                if self.in_tabs && &caps[1] == "tab" {
                    self.handle_tab()
                } else {
                    self.handle_admonition_start(&caps[1])
                }
            } else {
                String::new()
            }
        } else if self.admonition_end_regex.is_match(line) {
            if self.in_tabs {
                self.handle_tab_end()
            } else {
                self.handle_admonition_end()
            }
        } else {
            self.handle_regular_line(line)
        }
    }

    /// Handles the start of an admonition block.
    ///
    /// # Arguments
    ///
    /// * `class` - The class of the admonition.
    ///
    /// # Returns
    ///
    /// A `String` containing the opening HTML div tag for the admonition.
    fn handle_admonition_start(&mut self, class: &str) -> String {
        if class.is_empty() {
            return String::new(); // Return an empty string for empty admonitions
        }
        let html = match class {
            "alert" => "<div role=\"alert\" class=\"alert alert-info\">".to_string(),
            "info" => "<div class=\"admonition note\">".to_string(),
            "success" => "<div class=\"alert alert-success\">".to_string(),
            "warning" => "<div class=\"admonition important\">".to_string(),
            "error" => "<div class=\"admonition warning\">".to_string(),
            "tip" => "<div class=\"admonition tip\">".to_string(),
            "fold" => "<details class=\"my-details\"><summary>📂</summary>".to_string(),
            "summary" => "<summary class=\"my-summary\">".to_string(),
            "col" => "<div class=\"flex w-full flex-col lg:flex-row\">".to_string(),
            "card" => "<div class=\"card bg-base-100 w-96 shadow-xl\">".to_string(),
            _ => format!("<div class=\"{}\">", class),
        };
        self.div_stack.push(class.to_string());
        format!("{}\n", html)
    }

    /// Handles the end of an admonition block.
    ///
    /// # Returns
    ///
    /// A `String` containing the closing HTML tag for the admonition.
    fn handle_admonition_end(&mut self) -> String {
        if self.in_tabs {
            self.handle_tab_end()
        } else {
            self.div_stack.pop().map_or_else(
                || String::from(":::\n"),
                |class| match class.as_str() {
                    "fold" => "</details>\n".to_string(),
                    "summary" => "</summary>\n".to_string(),
                    _ => "</div>\n".to_string(),
                },
            )
        }
    }

    fn handle_tabs_start(&mut self) -> String {
        self.in_tabs = true;
        self.tab_count = 0;
        self.tabs_closing = false; // Reset this flag when starting a new tabs block
        "<div role=\"tablist\" class=\"tabs tabs-lifted\">\n".to_string()
    }

    fn handle_tab(&mut self) -> String {
        self.tab_count += 1;
        let checked = if self.tab_count == 2 {
            " checked=\"checked\""
        } else {
            ""
        };
        format!(
            "  <input type=\"radio\" name=\"my_tabs_2\" role=\"tab\" class=\"tab\" aria-label=\"Tab {}\"{}/>
  <div role=\"tabpanel\" class=\"tab-content bg-base-100 border-base-300 rounded-box p-6\">\n",
            self.tab_count, checked
        )
    }

    fn handle_tab_end(&mut self) -> String {
        if self.tab_count == 3 {
            self.in_tabs = false;
            self.tab_count = 0;
            self.tabs_closing = true; // Set this flag when closing the tabs block
            "  </div>\n</div>\n".to_string()
        } else {
            "  </div>\n".to_string()
        }
    }

    /// Handles the start of a code block.
    ///
    /// # Returns
    ///
    /// An empty `String` as the code block start is not directly output.
    fn handle_code_start(&mut self, is_display: bool) -> String {
        self.eval_stack = true;
        self.is_rhai_display = is_display;
        self.contents.clear();
        // Store the indentation by counting leading spaces
        if let Some(last_line) = self.contents.last() {
            self.current_indent = " ".repeat(last_line.chars().take_while(|c| c.is_whitespace()).count());
        } else {
            // If we're at a new line, count the spaces from the current line
            self.current_indent = " ".repeat(self.contents.len());
        }
        String::new()
    }

    /// Handles the end of a code block.
    ///
    /// # Returns
    ///
    /// A `String` containing the evaluated code output wrapped in HTML.
    fn handle_code_end(&mut self) -> String {
        if self.eval_stack {
            self.eval_stack = false;
            if !self.contents.is_empty() {
                let code = self.contents.join("\n");
                let results = Self::process_lambda(&self.rhai_engine, &mut self.rhai_scope, &code);
                self.contents.clear();

                if self.is_rhai_display {
                    if results.trim().is_empty() {
                        String::new()
                    } else {
                        format!(
                        "<div class=\"rhai-display\">\n\n```rust\n{}\n```\n<div class=\"rhai-out\">\n\n```\n{}\n```\n</div>\n</div>\n",
                        code, results
                    )
                    }
                } else {
                    String::new()
                }
            } else {
                String::new()
            }
        } else {
            format!("{}{}\n", self.current_indent, "```")
        }
    }

    /// Handles a regular line of text.
    ///
    /// # Arguments
    ///
    /// * `line` - A string slice that holds the line to be processed.
    ///
    /// # Returns
    ///
    /// A `String` containing the processed line.
    fn handle_regular_line(&mut self, line: &str) -> String {
        if self.eval_stack {
            self.contents.push(line.to_string());
            String::new()
        } else {
            // Count leading spaces if this is the first line after a code block
            if line.trim_start().starts_with("```") {
                self.current_indent = " ".repeat(line.chars().take_while(|c| c.is_whitespace()).count());
            }
            let mut result = String::new();
            let mut last_end = 0;
            let mut scope = self.rhai_scope.clone();
            let engine = &self.rhai_engine;

            for cap in self.lambda_regex.captures_iter(line) {
                let whole_match = cap.get(0).unwrap();
                let captured = &cap[1];
                result.push_str(&line[last_end..whole_match.start()]);
                result.push_str(&Self::process_lambda(engine, &mut scope, captured));
                last_end = whole_match.end();
            }
            result.push_str(&line[last_end..]);

            // Update the main scope with any changes from the cloned scope
            self.rhai_scope = scope;

            if result.trim().is_empty() {
                "\n".to_string() // Always return a newline for empty lines
            } else {
                format!("{}\n", result) // Add a newline after each non-empty line
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    /*
    The example binary can be helpful here for generating output to test against.
    cargo run --bin md_converter -- \
        -f markdown \
        -i ../../tests/fixtures/input_divs_code_and_inline_code.md \
        -o ../../tests/fixtures/expected_output_divs_code_and_inline_code.md
    */

    #[test]
    fn test_processor_with_custom_functions() {
        // Define test functions
        fn double(x: i64) -> i64 {
            x * 2
        }
        fn concat(a: String, b: String) -> String {
            format!("{}{}", a, b)
        }

        let functions: Vec<CustomFn> = vec![
            Box::new(|engine: &mut Engine| {
                engine.register_fn("double", double);
            }),
            Box::new(|engine: &mut Engine| {
                engine.register_fn("concat", concat);
            }),
        ];

        let mut processor = Processor::new(Some(functions));

        // Test numeric function - note the \n at the end
        let input1 = "Result: λ#(double(21))#";
        let expected1 = "Result: 42"; // New lines are stripped
        let result1 = processor.process(input1);
        assert_eq!(result1, expected1);

        // Test string function
        let input2 = r#"Combined: λ#(concat("Hello ", "World"))#"#;
        let expected2 = "Combined: Hello World"; // New lines are stripped
        let result2 = processor.process(input2);
        assert_eq!(result2, expected2);
    }

    #[test]
    fn test_processor_output() {
        let mut processor = Processor::default();

        let test_string = fs::read_to_string("tests/fixtures/input_divs_code_and_inline_code.md")
            .expect("Failed to read input fixture");

        let expected =
            fs::read_to_string("tests/fixtures/expected_output_divs_code_and_inline_code.md")
                .expect("Failed to read expected output fixture")
                .trim_end_matches('\n')
                .to_string();

        let result = processor.process(&test_string);

        assert_eq!(
            result, expected,
            "Processor output did not match expected output"
        );
    }

    #[test]
    fn test_processor_output_with_codeblocks() {
        let mut processor = Processor::default();

        let test_string = fs::read_to_string("tests/fixtures/code_block.md")
            .expect("Failed to read input fixture");

        let expected = test_string.clone().trim_end_matches('\n').to_string();

        let result = processor.process(&test_string);

        assert_eq!(
            expected, result,
            "Processor output did not match expected output"
        );
    }

    #[test]
    fn test_processor_output_with_custom_rhai() {
        // Register custom functions
        // Define test functions
        fn generate_ascii_diamond(size: i64) -> String {
            if size == 0 {
                println!("Size must be greater than 0.");
                return "".to_string();
            }

            let mut output = String::new();

            // Upper part of the diamond including the middle line
            for i in 0..size {
                let spaces = " ".repeat((size - i) as usize);
                let stars = "*".repeat((2 * i + 1) as usize);
                let line = format!("{spaces}{stars}\n");
                output.push_str(&line);
            }

            // Lower part of the diamond
            for i in (0..size - 1).rev() {
                let spaces = " ".repeat((size - i) as usize);
                let stars = "*".repeat((2 * i + 1) as usize);
                let line = format!("{spaces}{stars}\n");
                output.push_str(&line);
            }
            format!("<pre>\n{}\n</pre>", output)
        }

        fn double(x: i64) -> i64 {
            x * 2
        }
        fn concat(a: String, b: String) -> String {
            format!("{}{}", a, b)
        }
        let functions: Vec<CustomFn> = vec![
            Box::new(|engine: &mut Engine| {
                engine.register_fn("double", double);
            }),
            Box::new(|engine: &mut Engine| {
                engine.register_fn("concat", concat);
            }),
            Box::new(|engine: &mut Engine| {
                engine.register_fn("generate_ascii_diamond", generate_ascii_diamond);
            }),
        ];

        let test_string = fs::read_to_string("tests/fixtures/custom_rhai_functions.md")
            .expect("Failed to read input fixture");

        let expected = fs::read_to_string("tests/fixtures/custom_rhai_functions_expected.md")
            .expect("Failed to read expected output fixture")
            .trim_end_matches('\n')
            .to_string();

        // Processor is Mutable as it stores the rhai environment scope
        let mut processor = Processor::new(Some(functions));

        let result = processor.process(&test_string);

        assert_eq!(
            result, expected,
            "Processor output did not match expected output"
        );
    }

    #[test]
    fn test_tabs_processing() {
        let input = r#":::tabs

:::tab
Tab content 1
:::

:::tab
Tab content 2
:::

:::tab
Tab content 3
:::

:::"#;

        let expected_output = r#"<div role="tablist" class="tabs tabs-lifted">

  <input type="radio" name="my_tabs_2" role="tab" class="tab" aria-label="Tab 1"/>
  <div role="tabpanel" class="tab-content bg-base-100 border-base-300 rounded-box p-6">
Tab content 1
  </div>

  <input type="radio" name="my_tabs_2" role="tab" class="tab" aria-label="Tab 2" checked="checked"/>
  <div role="tabpanel" class="tab-content bg-base-100 border-base-300 rounded-box p-6">
Tab content 2
  </div>

  <input type="radio" name="my_tabs_2" role="tab" class="tab" aria-label="Tab 3"/>
  <div role="tabpanel" class="tab-content bg-base-100 border-base-300 rounded-box p-6">
Tab content 3
  </div>
</div>"#;

        let mut processor = Processor::default();
        let result = processor.process(input);

        assert_eq!(
            result.trim(),
            expected_output.trim(),
            "Tabs processing did not produce the expected output"
        );
    }

    #[test]
    fn test_list_tems() {
        let input = r#"
- item 1
    - item 2
        ```python
        print("Under Item 2")
        ```
    - item 3
        ```python
        print("Under Item 3")
        ```
"#;

        let input = input.trim();
        let expected_output = input.clone();
        let mut processor = Processor::default();
        let result = processor.process(input);

        assert_eq!(
            result.trim(),
            expected_output.trim(),
            "List items did not produce the expected output"
        );
    }
}
