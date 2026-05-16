# zero-width-strip

[![crates.io](https://img.shields.io/crates/v/zero-width-strip.svg)](https://crates.io/crates/zero-width-strip)

Strip zero-width and bidi-control Unicode chars from text. Closes the
"invisible payload" prompt-injection channel.

```rust
use zero_width_strip::{strip, has_invisible};
let dirty = "hello\u{200B}\u{202E}world";
assert!(has_invisible(dirty));
assert_eq!(strip(dirty), "helloworld");
```

Covers U+200B–U+200F, U+202A–U+202E, U+2060–U+2064, U+2066–U+2069,
U+180E, U+FEFF. Zero deps. MIT or Apache-2.0.
