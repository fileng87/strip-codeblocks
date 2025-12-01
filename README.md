# strip-codeblocks

A Rust library to strip markdown fenced code blocks from text while preserving the inner content and leaving inline code blocks untouched.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
strip-codeblocks = "0.1.0"
```

## Usage

```rust
use strip_codeblocks::strip_codeblocks;

let markdown = "Some text.\n\n```rust\nfn main() {}\n```\n\nMore text with `inline code`.";

let result = strip_codeblocks(markdown);
// Result: "Some text.\n\nfn main() {}\n\nMore text with `inline code`."
```

## Features

- Removes fenced code blocks (triple backticks: ```)
- Preserves the content inside code blocks
- Keeps inline code blocks (single backticks: `) intact
- Handles code blocks with or without language identifiers

## License

MIT - see [LICENSE](LICENSE) file for details
