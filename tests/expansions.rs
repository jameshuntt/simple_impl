//! Each macro the kit's patterns reading asked for writes exactly the code it
//! stands for. rustc expands tests/expand/kit_patterns.rs, where every form is
//! called in `<macro>_<form>_by_macro` and written out by hand in
//! `<macro>_<form>_by_hand`, and the two bodies have to print the same.
//!
//! Two differences are written off first, on both sides, and nothing else: the
//! prelude names the toolkit writes in full (`::core::result::Result::Err` for
//! `Err`, `::core::option::Option::Some` for `Some`), and the `;` a statement
//! call leaves after an `if` (`return_if_err!(a, e => y);` is `if let Err(e) = a
//! { return y; };`). Line breaks are the printer's, so runs of whitespace count
//! as one space.
//!
//! The printer is rustc's own `-Zunpretty=expanded`. It is a nightly flag, so
//! the test runs it with RUSTC_BOOTSTRAP=1 on whatever rustc cargo runs; it
//! prints the expansion and stops, nothing is built or written. A second test
//! compiles the fixture, so both sides are code that type-checks.

use std::process::Command;

const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/expand/kit_patterns.rs");

/// Every macro and form in the fixture.
const FORMS: &[(&str, &str)] = &[
    ("require_ok_into", "let"),
    ("require_ok_into", "let_mut"),
    ("require_ok_into", "let_typed"),
    ("require_ok_into", "statement"),
    ("require_some_into", "ok_or"),
    ("require_some_into", "ok_or_else"),
    ("ensure_valid", "statement"),
    ("ensure_valid", "map_err"),
    ("ok_or_else_return", "let"),
    ("return_if_err", "statement"),
    ("if_else", "let"),
    ("if_else", "value"),
    ("table", "three"),
    ("table", "four"),
];

fn expanded() -> String {
    let rustc = std::env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string());
    let out = Command::new(rustc)
        .env("RUSTC_BOOTSTRAP", "1")
        .args(["--edition", "2021", "--crate-type", "lib", "--crate-name", "kit_patterns", "-Zunpretty=expanded", FIXTURE])
        .output()
        .expect("rustc runs");
    assert!(out.status.success(), "rustc could not expand the fixture:\n{}", String::from_utf8_lossy(&out.stderr));
    String::from_utf8(out.stdout).expect("the expansion is UTF-8")
}

/// The body of `fn name`, braces included.
fn body<'a>(text: &'a str, name: &str) -> &'a str {
    let start = text.find(&format!("fn {name}(")).unwrap_or_else(|| panic!("no fn {name}"));
    let open = start + text[start..].find('{').expect("a body");
    let mut depth = 0;
    for (i, c) in text[open..].char_indices() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return &text[open..=open + i];
                }
            }
            _ => {}
        }
    }
    panic!("fn {name}'s body does not close")
}

/// The form the two sides are compared in.
fn normal(body: &str) -> String {
    let short = body.replace("::core::result::Result::", "").replace("::core::option::Option::", "");
    let spaced = short.split_whitespace().collect::<Vec<_>>().join(" ");
    drop_semicolon_after_if(&spaced)
}

/// `if ... { ... };` as a statement is `if ... { ... }`.
fn drop_semicolon_after_if(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut out = String::new();
    let mut i = 0;
    while i < bytes.len() {
        let at_statement = i == 0 || text[..i].ends_with("{ ") || text[..i].ends_with("; ") || text[..i].ends_with("} ");
        if at_statement && text[i..].starts_with("if ") {
            let mut depth = 0;
            let mut j = i;
            let end = loop {
                match bytes[j] {
                    b'{' => depth += 1,
                    b'}' => {
                        depth -= 1;
                        if depth == 0 && !text[j + 1..].starts_with(" else") {
                            break j + 1;
                        }
                    }
                    _ => {}
                }
                j += 1;
            };
            out.push_str(&text[i..end]);
            i = if text[end..].starts_with(';') { end + 1 } else { end };
            continue;
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    out
}

#[test]
fn every_form_expands_to_the_code_written_by_hand() {
    let source = std::fs::read_to_string(FIXTURE).expect("the fixture reads");
    let text = expanded();
    for (piece, form) in FORMS {
        let by_macro = format!("{piece}_{form}_by_macro");
        let by_hand = format!("{piece}_{form}_by_hand");
        assert!(body(&source, &by_macro).contains(&format!("{piece}!(")), "{by_macro} does not call {piece}!");
        assert!(!body(&source, &by_hand).contains('!'), "{by_hand} calls a macro");
        let (m, h) = (normal(body(&text, &by_macro)), normal(body(&text, &by_hand)));
        assert_eq!(m, h, "{piece}! ({form}) does not write the code written by hand");
        println!("{piece}! ({form}): {m}");
    }
}

#[test]
fn the_fixture_type_checks() {
    // the two sides are real code: both compile, with the toolkit as it ships
    let rustc = std::env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string());
    let out = Command::new(rustc)
        .args(["--edition", "2021", "--crate-type", "lib", "--crate-name", "kit_patterns", "--emit=metadata", "-o"])
        .arg(std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join("kit_patterns.rmeta"))
        .arg(FIXTURE)
        .output()
        .expect("rustc runs");
    assert!(out.status.success(), "the fixture does not compile:\n{}", String::from_utf8_lossy(&out.stderr));
}

#[test]
fn the_normal_form_drops_only_what_it_says() {
    assert_eq!(normal("{ if let ::core::result::Result::Err(e) = a { return y; }; 0 }"), "{ if let Err(e) = a { return y; } 0 }");
    assert_eq!(normal("{ let x = if c { a } else { b }; x }"), "{ let x = if c { a } else { b }; x }");
    assert_eq!(normal("{ if c { a } else { b }; 0 }"), "{ if c { a } else { b } 0 }");
    assert_ne!(normal("{ a.map_err(f)?; }"), normal("{ a.map_err(f); }"));
}
