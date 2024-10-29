# Draftsmith Markdown Preprocessor

## Overview

This library is designed to preprocess markdown files, implementing custom features used by Draftsmith, a markdown-based knowledge management tool. It extends standard markdown syntax with additional capabilities and finally parses the output to HTML.

## Features

- Custom markdown preprocessing for Draftsmith-specific syntax
- Flexible and extensible design for experimentation
- Outputs unsanitized HTML by default for maximum flexibility
- Designed to be integrated into future projects

## Usage

This library can be used as a component in markdown-based applications, particularly those requiring custom syntax extensions or specialized preprocessing.

### Basic Example

```rust
use draftsmith_md_preprocessor::parse_md_to_html;

fn main() {
    let markdown = "# Hello, Draftsmith!\n\nThis is a **custom** markdown document.";
    let html = parse_md_to_html(markdown);
    println!("{}", html);
}
```

## Warning

The HTML output is unsanitized by default. This is intentional to allow for maximum flexibility and experimentation. However, if you're using this in a context where you're rendering user-supplied content, make sure to sanitize the output before displaying it to prevent XSS attacks.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

[Insert your chosen license here]

## Disclaimer

This tool is designed for flexibility and experimentation. Use it responsibly and be aware of potential security implications when working with unsanitized HTML output.
