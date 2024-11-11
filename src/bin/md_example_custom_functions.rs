use draftsmith_render::processor::Processor;
use rhai::Engine;
use std::error::Error;
use std::fs;
pub type CustomFn = Box<dyn Fn(&mut Engine)>;

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = "tests/fixtures/custom_rhai_functions.md";
    let _expected_path = "tests/fixtures/custom_rhai_functions_expected.md";

    let test_string = fs::read_to_string(input_path)?;

    // Register custom functions
    fn double(x: i64) -> i64 {
        x * 2
    }
    fn concat(a: String, b: String) -> String {
        format!("{}{}", a, b)
    }
    let separator = "¶"; // This will be cloned into the closure below
    let sep2 = "$"; // The closure will take an immutable reference to this string
    let functions: Vec<CustomFn> = vec![
        Box::new(|engine: &mut Engine| {
            engine.register_fn("double", double);
        }),
        Box::new(|engine: &mut Engine| {
            engine.register_fn("concat", concat);
        }),
        Box::new(move |engine: &mut Engine| {
            let separator = separator.to_string(); // Clone it here so we can move it into the next closure
            engine.register_fn("generate_ascii_diamond", move |size: i64| -> String {
                if size == 0 {
                    println!("Size must be greater than 0.");
                    return "".to_string();
                }

                let separator = format!("{separator}{sep2}");

                let separator = format!("{separator}{sep2}");

                let mut output = String::new();

                // Upper part of the diamond including the middle line
                for i in 0..size {
                    let spaces = " ".repeat((size - i) as usize);
                    let stars = separator.repeat((2 * i + 1) as usize);
                    let line = format!("{spaces}{stars}\n");
                    output.push_str(&line);
                }

                // Lower part of the diamond
                for i in (0..size - 1).rev() {
                    let spaces = " ".repeat((size - i) as usize);
                    let stars = separator.repeat((2 * i + 1) as usize);
                    let line = format!("{spaces}{stars}\n");
                    output.push_str(&line);
                }
                format!("<pre>\n{}\n</pre>", output)
            });
        }),
    ];

    // Processor is Mutable as it stores the rhai environment scope
    let mut markdown_processor = Processor::new(Some(functions));
    let document = markdown_processor.process(&test_string);

    println!("{document}");

    Ok(())
}
