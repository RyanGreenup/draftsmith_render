//! This module provides functionality for processing markdown-like text
//! with custom admonitions and code blocks.

use regex::Regex;
use rhai::{Engine, Scope};

const ADMONITION_START_PATTERN: &str = r"^\s*:::([\w!\{\}-]+)$";
const ADMONITION_END_PATTERN: &str = r"^\s*(:::)$";
const CODE_START_PATTERN: &str = r"^\s*```\{rhai\}$";
const RHAI_DISPLAY_START_PATTERN: &str = r"^\s*```\{rhai-display\}$";
const CODE_END_PATTERN: &str = r"^\s*```$";
const LAMBDA_PATTERN: &str = r"λ#\(((?s).*?)\)#";

/// A processor for handling custom markdown-like syntax.
pub struct Processor<'a> {
    admonition_start_regex: Regex,
    admonition_end_regex: Regex,
    code_start_regex: Regex,
    rhai_display_start_regex: Regex,
    code_end_regex: Regex,
    lambda_regex: Regex,
    div_stack: Vec<String>,
    eval_stack: bool,
    is_rhai_display: bool,
    contents: Vec<String>,
    rhai_engine: Engine,
    rhai_scope: Scope<'a>,
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
            div_stack: Vec::new(),
            eval_stack: false,
            is_rhai_display: false,
            contents: Vec::new(),
            rhai_engine: Engine::new(),
            rhai_scope: Scope::new(),
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
        if self.code_start_regex.is_match(line) {
            self.handle_code_start(false)
        } else if self.rhai_display_start_regex.is_match(line) {
            self.handle_code_start(true)
        } else if self.code_end_regex.is_match(line) {
            self.handle_code_end()
        } else if self.admonition_start_regex.is_match(line) {
            if let Some(caps) = self.admonition_start_regex.captures(line) {
                self.handle_admonition_start(&caps[1])
            } else {
                String::new()
            }
        } else if self.admonition_end_regex.is_match(line) {
            self.handle_admonition_end()
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
            "alert" => format!("<div role=\"alert\" class=\"alert alert-info\">"),
            "info" => format!("<div class=\"alert alert-info\">"),
            "success" => format!("<div class=\"alert alert-success\">"),
            "warning" => format!("<div class=\"alert alert-warning\">"),
            "error" => format!("<div class=\"alert alert-error\">"),
            "tip" => format!("<div class=\"tip\">"),
            "fold" => format!("<details class=\"my-details\">"),
            "summary" => format!("<summary class=\"my-summary\">"),
            "col" => format!("<div class=\"flex w-full flex-col lg:flex-row\">"),
            "card" => format!("<div class=\"card bg-base-100 w-96 shadow-xl\">"),
            _ => format!("<div class=\"{}\">", class),
        };
        self.div_stack.push(class.to_string());
        format!("{}\n", html)
    }

    /// Handles the end of an admonition block.
    ///
    /// # Returns
    ///
    /// A `String` containing the closing HTML div tag for the admonition.
    fn handle_admonition_end(&mut self) -> String {
        if self.div_stack.pop().is_some() {
            "</div>\n".to_string()
        } else {
            String::from(":::\n")
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
        self.contents.clear(); // Clear any previous contents
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
                        return String::new();
                    } else {
                        format!(
                        "<div class=\"rhai-display\">\n```rust\n{}\n```\n<div class=\"rhai-out\">\n```\n{}\n```\n</div>\n</div>\n",
                        code, results
                    )
                    }
                } else {
                    return String::new();
                }
            } else {
                String::new()
            }
        } else {
            String::new()
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
            String::new() // Return an empty string when in eval_stack mode
        } else {
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
