//! # strip-codeblocks
//!
//! A Rust library to strip markdown fenced code blocks from text while preserving
//! the inner content and leaving inline code blocks untouched.
//!
//! ## Features
//!
//! - Removes fenced code blocks (triple backticks: \`\`\`)
//! - Preserves the content inside code blocks
//! - Keeps inline code blocks (single backticks: \`) intact
//! - Handles code blocks with or without language identifiers
//!
//! ## Usage
//!
//! ```rust
//! use strip_codeblocks::strip_codeblocks;
//!
//! let markdown = "Here is some text.\n\n```rust\nfn main() {\n    println!(\"Hello, world!\");\n}\n```\n\nMore text with `inline code` here.";
//!
//! let result = strip_codeblocks(markdown);
//! // Result: "Here is some text.\n\nfn main() {\n    println!(\"Hello, world!\");\n}\n\nMore text with `inline code` here."
//! ```
//!
//! ## Examples
//!
//! ### Basic Usage
//!
//! ```rust
//! use strip_codeblocks::strip_codeblocks;
//!
//! let input = "```python\nprint('hello')\n```";
//! let output = strip_codeblocks(input);
//! assert_eq!(output, "print('hello')\n");
//! ```
//!
//! ### Preserving Inline Code
//!
//! ```rust
//! use strip_codeblocks::strip_codeblocks;
//!
//! let input = "This has `inline code` and ```\ncode block\n```";
//! let output = strip_codeblocks(input);
//! assert_eq!(output, "This has `inline code` and code block\n");
//! ```

use regex::Regex;

/// Strips fenced code blocks from markdown text while preserving the inner content.
///
/// This function removes markdown fenced code blocks (triple backticks) but keeps
/// the content inside them. Inline code blocks (single backticks) are left untouched.
///
/// # Arguments
///
/// * `text` - The markdown text containing code blocks to strip
///
/// # Returns
///
/// A new string with fenced code blocks removed, but their content preserved.
///
/// # Examples
///
/// ```
/// use strip_codeblocks::strip_codeblocks;
///
/// let markdown = "Some text before.\n\n```rust\nfn example() {\n    println!(\"Hello\");\n}\n```\n\nSome text after with `inline code`.";
///
/// let result = strip_codeblocks(markdown);
/// // The fenced code block is removed, but its content remains
/// //Inline code is preserved
/// ```
pub fn strip_codeblocks(text: &str) -> String {
    // Match fenced code blocks: ```optional_lang\n...content...\n```
    // This regex matches:
    // - Three backticks (```)
    // - Optional language identifier (any characters except newline and backtick)
    // - Newline
    // - Content (non-greedy, including newlines)
    // - Three backticks (```)
    // The (?s) flag makes . match newlines
    let re = Regex::new(r"(?s)```[^\n`]*\n(.*?)```").unwrap();

    re.replace_all(text, |caps: &regex::Captures| {
        // Extract the content (first capture group)
        caps.get(1)
            .map_or(String::new(), |m| m.as_str().to_string())
    })
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_codeblock() {
        let input = "```rust\nfn main() {}\n```";
        let output = strip_codeblocks(input);
        assert_eq!(output, "fn main() {}\n");
    }

    #[test]
    fn test_codeblock_with_language() {
        let input = "```python\nprint('hello')\n```";
        let output = strip_codeblocks(input);
        assert_eq!(output, "print('hello')\n");
    }

    #[test]
    fn test_codeblock_without_language() {
        let input = "```\njust code\n```";
        let output = strip_codeblocks(input);
        assert_eq!(output, "just code\n");
    }

    #[test]
    fn test_preserves_inline_code() {
        let input = "This has `inline code` in it.";
        let output = strip_codeblocks(input);
        assert_eq!(output, "This has `inline code` in it.");
    }

    #[test]
    fn test_multiple_codeblocks() {
        let input = "```rust\nfn a() {}\n```\n```python\nprint('b')\n```";
        let output = strip_codeblocks(input);
        assert_eq!(output, "fn a() {}\n\nprint('b')\n");
    }

    #[test]
    fn test_codeblock_with_text_around() {
        let input = "Before\n```rust\ncode here\n```\nAfter";
        let output = strip_codeblocks(input);
        assert_eq!(output, "Before\ncode here\n\nAfter");
    }

    #[test]
    fn test_codeblock_with_inline_code() {
        let input = "Text with `inline` and ```\nblock code\n```";
        let output = strip_codeblocks(input);
        assert_eq!(output, "Text with `inline` and block code\n");
    }

    #[test]
    fn test_empty_codeblock() {
        let input = "```\n\n```";
        let output = strip_codeblocks(input);
        assert_eq!(output, "\n");
    }

    #[test]
    fn test_codeblock_with_multiline_content() {
        let input = "```python\ndef hello():\n    print('hi')\n    return True\n```";
        let output = strip_codeblocks(input);
        assert_eq!(output, "def hello():\n    print('hi')\n    return True\n");
    }

    #[test]
    fn test_no_codeblocks() {
        let input = "Just regular text with `inline code`.";
        let output = strip_codeblocks(input);
        assert_eq!(output, "Just regular text with `inline code`.");
    }

    #[test]
    fn test_codeblock_with_special_chars_in_language() {
        let input = "```c++\nint x = 0;\n```";
        let output = strip_codeblocks(input);
        assert_eq!(output, "int x = 0;\n");
    }

    #[test]
    fn test_codeblock_with_backticks_inside() {
        // Code blocks can contain backticks, but we should still match the closing ```
        let input = "```\nThis has `backticks` inside\n```";
        let output = strip_codeblocks(input);
        assert_eq!(output, "This has `backticks` inside\n");
    }

    #[test]
    fn test_complex_markdown() {
        let input = r#"# Title

Some paragraph with `inline code`.

```rust
fn main() {
    println!("Hello");
}
```

More text with ``double backticks`` inline.

```python
x = 1
y = 2
```
"#;
        let output = strip_codeblocks(input);
        assert!(output.contains("fn main()"));
        assert!(output.contains("`inline code`"));
        assert!(output.contains("``double backticks``"));
        assert!(output.contains("x = 1"));
        assert!(!output.contains("```rust"));
        assert!(!output.contains("```python"));
    }
}
