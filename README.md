# chatgpt-rs

## Usage
```toml
[dependencies]
chatgpt = { git = "https://github.com/minyung/chatgpt-rs.git" }
```

```rust
fn main {
    let mut gpt = Gpt35TurboChat::new(
        "YOUR_OPENAI_API_KEY"
    );
    print!("{}", gpt.run("...").unwrap());
}
```

## Supported chat model
- gpt-3.5-turbo
