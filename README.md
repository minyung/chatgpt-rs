# chatgpt-rs
[![CI](https://github.com/minyung/chatgpt-rs/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/minyung/chatgpt-rs/actions/workflows/ci.yml)

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
