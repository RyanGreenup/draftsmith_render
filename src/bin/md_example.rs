use draftsmith_render::replace_text;

const DOC: &str = r#"
# Heading

# Admonitions

<div class="tip">

<p>Also maps closer to tailwind css (easier for me)</p>

</div>

# Basic Text

This is my input.\

1. Also [my](#) input.
2. Certainly *my* input.\

```rust
This is some code
```

- Image
    - Normal
        - ![](https://example.com/image.png)
    - Figure
        - ![notrim](https://example.com/image.png)
    - Wiki
        - [[!https://example.com/image.png]]
- Link
    - Normal
        - [Link](https://example.com)
    - Wiki
        - [[Link|https://example.com]]
        - [[Link]]
    - Transclusion
        - [[Link|https://example.com]]

## Math
### Inline
$\sqrt{3x-1}+(1+x)^2$
### Block
$$
\begin{aligned}
\sqrt{3x-1}+(1+x)^2
\end{aligned}
$$

Some markdown:

```markdown
## Math
### Inline
$\sqrt{3x-1}+(1+x)^2$
### Block
$$
\begin{aligned}
\sqrt{3x-1}+(1+x)^2
\end{aligned}
$$

Some markdown:
```

"#;

/* View the output like so
    cargo run --bin md_example | npx prettier --parser html | bat -l html
*/

fn main() {
    let doc = DOC;
    let orig = "my";
    let repl = "your";
    let html = replace_text(&doc, &orig, &repl);

    println!("{}", html);
    // Output:
    //
    // <p>This is your input.</p>
    // <ol>
    // <li>Also <a href="#">your</a> input.</li>
    // <li>Certainly <em>your</em> input.</li>
    // </ol>
}
