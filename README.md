## About
This lib create as a helper lib for the [MCQP](https://github.com/mcqp/mcqp) project, It parse and
validate the [Telegram Markdown V1](https://core.telegram.org/bots/api#markdown-style) style, It convert
the markdown to AST and check for errors and get their positions. This lib does not use any other libs it just 
use the native Rust code to parse and validate the markdown.

## Usage
- Add the lib to `Cargo.toml`
```toml
[dependencies]
telemark = { git = "https://github.com/mcqp/telemark.git", tag = "v0.1.0" }
```

- Start using it
```rust
use telemark::parser::mdv1;

fn main() {
    if let Ok(ast) = mdv1::parser("*bold*") {
        println!("The AST: {:#?}", ast);
        println!("The first node value: {:#?}", ast.value());
        println!("The inners nodes length: {}", ast.inner().len());
        println!("The first inner node value: {:#?}", ast.inner()[0].value());
    }

    // This is an error:
    if let Err(err) = mdv1::parser("this is not bold*") {
        println!("The error: {:#?}", err);
        println!("The error type: {:#?}", err.err());
        println!("The error position: {}", err.offset());
    }
}
```

---
> By [Mohaned Sherhan (Mr.x)](https://github.com/Mohaned2023)