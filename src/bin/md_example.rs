use draftsmith_render::replace_text;

const DOC: &str = r#"
<div class="ugh">
this is inline html
</div>

:::foo


this is an admonition block


:::



# Heading

# Admonitions



:::tip
This works

    :::foo
    Also maps closer to tailwind css (easier for me)
    :::

```rust
fn main() {
    println!("Hello, world!");
}
```

:::

This is some code:

```markdown
:::tip
This works

:::
Also maps closer to tailwind css (easier for me)
:::

:::

```

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
    let test_string = std::fs::read_to_string("tests/fixtures/input_divs_code_and_inline_code.md").unwrap();
    let expected = std::fs::read_to_string("tests/fixtures/expected_output_divs_code_and_inline_code.md").unwrap().trim_end_matches('\n').to_string();
    let html = replace_text(&doc, &orig, &repl);

    // println!("{}", html);
}
