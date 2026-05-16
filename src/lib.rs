//! # zero-width-strip
//!
//! Strip zero-width and bidi-control Unicode characters from text.
//!
//! Zero-width characters (U+200B–U+200F, U+2060, U+FEFF, etc.) and bidi
//! overrides (U+202A–U+202E) are invisible in most renderers but are
//! preserved by most tokenizers, which makes them a clean payload
//! channel for prompt-injection attacks ("invisible instructions"
//! hidden inside otherwise plain text).
//!
//! This crate strips them.
//!
//! ## Example
//!
//! ```
//! use zero_width_strip::{strip, has_invisible};
//! let dirty = "hello\u{200B}\u{202E}world";
//! assert!(has_invisible(dirty));
//! assert_eq!(strip(dirty), "helloworld");
//! ```

#![deny(missing_docs)]

/// True when the input contains any zero-width or bidi-override char.
pub fn has_invisible(s: &str) -> bool {
    s.chars().any(is_invisible)
}

/// Return a copy of `s` with every zero-width / bidi-override char
/// removed.
pub fn strip(s: &str) -> String {
    s.chars().filter(|c| !is_invisible(*c)).collect()
}

/// Strip into a caller-provided buffer (avoids an allocation).
pub fn strip_into(s: &str, out: &mut String) {
    out.reserve(s.len());
    for c in s.chars() {
        if !is_invisible(c) {
            out.push(c);
        }
    }
}

/// Per-char test. The list covers:
/// - U+200B…U+200F (zero-width space, ZWNJ, ZWJ, LRM, RLM)
/// - U+202A…U+202E (LRE, RLE, PDF, LRO, RLO — bidi controls)
/// - U+2060…U+2064 (word joiner + invisible math operators)
/// - U+2066…U+2069 (LRI, RLI, FSI, PDI — bidi isolates)
/// - U+FEFF (BOM / zero-width no-break space)
/// - U+180E (Mongolian vowel separator)
fn is_invisible(c: char) -> bool {
    matches!(c as u32,
        0x200B..=0x200F
        | 0x202A..=0x202E
        | 0x2060..=0x2064
        | 0x2066..=0x2069
        | 0x180E
        | 0xFEFF
    )
}
