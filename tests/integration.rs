use zero_width_strip::{has_invisible, strip, strip_into};

#[test]
fn detects_zwsp() {
    assert!(has_invisible("a\u{200B}b"));
    assert_eq!(strip("a\u{200B}b"), "ab");
}

#[test]
fn detects_bidi_override() {
    let attack = "safe\u{202E}evil";
    assert!(has_invisible(attack));
    assert_eq!(strip(attack), "safeevil");
}

#[test]
fn detects_bom() {
    assert!(has_invisible("\u{FEFF}hello"));
}

#[test]
fn passes_clean_text() {
    assert!(!has_invisible("hello world"));
    assert_eq!(strip("hello world"), "hello world");
}

#[test]
fn preserves_normal_unicode() {
    let s = "café résumé 日本語 \u{1F600}";
    assert_eq!(strip(s), s);
}

#[test]
fn strip_into_appends() {
    let mut out = "pre:".to_string();
    strip_into("a\u{200B}b", &mut out);
    assert_eq!(out, "pre:ab");
}

#[test]
fn covers_all_bidi_isolates() {
    for c in ['\u{2066}', '\u{2067}', '\u{2068}', '\u{2069}'] {
        assert!(has_invisible(&c.to_string()), "missed {c:?}");
    }
}
