# Draftsmith Markdown Preprocessor

## Overview

<p><img src="./media/logo.png" style="float: left; width: 80px" /></p>

This library preprocess markdown files, implementing custom features used by Draftsmith, a markdown-based knowledge management tool. It extends standard markdown syntax with some `<div>` blocks provided by DaisyUI and allows evaluating Rhai code inline.

## Features

### Summary

- Custom markdown preprocessing for Draftsmith-specific syntax
    - Including tabs, callouts, details, and columns
    - Inline Rhai code evaluation
- Unsanitized HTML output for maximum flexibility

### Inline Code

Inspired by MDX and Rmarkdown, this library allows you to evaluate Rhai code inline in your markdown documents. This can be useful to avoid repetition, generate dynamic content, or perform calculations.

    ## Code Blocks

    ### Hidden

    ```{rhai}
    let s = 0;
    for i in 1..10 {
        s += i;
    }
    ```

    ### Display

    ```{rhai-display}
    let t = "";
    for i in 1..s {
        t += i;
        if i != (s-1) {
            t += " + ";
        }
    }
    t
    ```

### Inline Code

The sum of the first 10 numbers is λ#(s)# all together that is: λ#(t)#

```

```html
<h2>Code Blocks</h2>
<h3>Hidden</h3>
<h3>Display</h3>
<div class="rhai-display">
<pre lang="rust"><code>let t = &quot;&quot;;
for i in 1..s {
    t += i;
    if i != (s-1) {
        t += &quot; + &quot;;
    }
}
t
</code></pre>
<div class="rhai-out">
<pre><code>1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10 + 11 + 12 + 13 + 14 + 15 + 16 + 17 + 18 + 19 + 20 + 21 + 22 + 23 + 24 + 25 + 26 + 27 + 28 + 29 + 30 + 31 + 32 + 33 + 34 + 35 + 36 + 37 + 38 + 39 + 40 + 41 + 42 + 43 + 44
</code></pre>
</div>
</div>
<h3>Inline Code</h3>
<p>The sum of the first 10 numbers is 45 all together that is: 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10 + 11 + 12 + 13 + 14 + 15 + 16 + 17 + 18 + 19 + 20 + 21 + 22 + 23 + 24 + 25 + 26 + 27 + 28 + 29 + 30 + 31 + 32 + 33 + 34 + 35 + 36 + 37 + 38 + 39 + 40 + 41 + 42 + 43 + 44</p>


```

## Usage

This library can be used as a component in markdown-based applications, particularly those requiring custom syntax extensions or specialized preprocessing.

### Basic Example

#### Library

```rust
use draftsmith_md_preprocessor::parse_md_to_html;

fn main() {
    let markdown = "# Hello, Draftsmith!\n\nThis is a **custom** markdown document.";
    let html = parse_md_to_html(markdown);
    println!("{}", html);
}
```

#### Command Line

```bash
cargo run --bin md_converter -- -i tests/fixtures/input_divs_code_and_inline_code.md
```


## Warning

The HTML output is unsanitized by default. This is intentional as this software targets users and aims to motivate experimentation. However, if you're using this in a context where you're rendering untrusted content, pay mind to XSS attacks.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

GPL

