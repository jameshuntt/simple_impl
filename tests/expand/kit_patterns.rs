// The code tests/expansions.rs expands with rustc: each macro the kit's patterns
// reading asked for, called in `<macro>_<form>_by_macro`, next to the same code
// written out by hand in `<macro>_<form>_by_hand`. The toolkit comes in as its own
// source file, so this is the macro as it ships. Not a test target of its own.
#![allow(dead_code, unused_mut, unused_variables, clippy::all)]

#[macro_use]
#[path = "../../src/macros/toolkit.rs"]
mod toolkit;

#[derive(Debug)]
struct Error(String);

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Error(e.to_string())
    }
}

struct Form(u8);

impl Form {
    fn validate(&self) -> Result<(), String> {
        if self.0 == 0 { Err(String::new()) } else { Ok(()) }
    }
}

enum Signal { Term, Kill, Hup, Int }

fn parse(text: &str) -> Result<u16, std::num::ParseIntError> {
    text.parse::<u16>()
}

fn wrap(e: std::num::ParseIntError) -> Error {
    Error(e.to_string())
}

// require_ok_into: a.map_err(f)? bound by let, let mut and a typed let, and as a statement

fn require_ok_into_let_by_macro(text: &str) -> Result<u16, Error> {
    let port = require_ok_into!(parse(text), wrap);
    Ok(port)
}
fn require_ok_into_let_by_hand(text: &str) -> Result<u16, Error> {
    let port = parse(text).map_err(wrap)?;
    Ok(port)
}

fn require_ok_into_let_mut_by_macro(text: &str) -> Result<u16, Error> {
    let mut port = require_ok_into!(parse(text), |e| Error(e.to_string()));
    Ok(port)
}
fn require_ok_into_let_mut_by_hand(text: &str) -> Result<u16, Error> {
    let mut port = parse(text).map_err(|e| Error(e.to_string()))?;
    Ok(port)
}

fn require_ok_into_let_typed_by_macro(text: &str) -> Result<u16, Error> {
    let port: u16 = require_ok_into!(text.parse::<u16>(), wrap);
    Ok(port)
}
fn require_ok_into_let_typed_by_hand(text: &str) -> Result<u16, Error> {
    let port: u16 = text.parse::<u16>().map_err(wrap)?;
    Ok(port)
}

fn require_ok_into_statement_by_macro(text: &str) -> Result<(), Error> {
    require_ok_into!(parse(text), wrap);
    Ok(())
}
fn require_ok_into_statement_by_hand(text: &str) -> Result<(), Error> {
    parse(text).map_err(wrap)?;
    Ok(())
}

// require_some_into: o.ok_or(e)? and o.ok_or_else(|| e)?

fn require_some_into_ok_or_by_macro(args: &[u16]) -> Result<u16, Error> {
    let first = require_some_into!(args.first().copied(), Error(String::new()));
    Ok(first)
}
fn require_some_into_ok_or_by_hand(args: &[u16]) -> Result<u16, Error> {
    let first = args.first().copied().ok_or(Error(String::new()))?;
    Ok(first)
}

fn require_some_into_ok_or_else_by_macro(args: &[u16]) -> Result<u16, Error> {
    let mut first: u16 = require_some_into!(args.first().copied(), || Error(String::new()));
    Ok(first)
}
fn require_some_into_ok_or_else_by_hand(args: &[u16]) -> Result<u16, Error> {
    let mut first: u16 = args.first().copied().ok_or_else(|| Error(String::new()))?;
    Ok(first)
}

// ensure_valid: x.validate()? and x.validate().map_err(f)?

fn ensure_valid_statement_by_macro(form: Form) -> Result<u8, String> {
    ensure_valid!(form);
    Ok(form.0)
}
fn ensure_valid_statement_by_hand(form: Form) -> Result<u8, String> {
    form.validate()?;
    Ok(form.0)
}

fn ensure_valid_map_err_by_macro(form: Form) -> Result<u8, Error> {
    ensure_valid!(form, Error);
    Ok(form.0)
}
fn ensure_valid_map_err_by_hand(form: Form) -> Result<u8, Error> {
    form.validate().map_err(Error)?;
    Ok(form.0)
}

// ok_or_else_return: let x = match r { Ok(v) => v, Err(e) => return y };

fn ok_or_else_return_let_by_macro(text: &str) -> usize {
    let port = ok_or_else_return!(parse(text), refused => refused.to_string().len());
    port as usize
}
fn ok_or_else_return_let_by_hand(text: &str) -> usize {
    let port = match parse(text) {
        Ok(value) => value,
        Err(refused) => return refused.to_string().len(),
    };
    port as usize
}

// return_if_err: if let Err(e) = a { return y; }

fn return_if_err_statement_by_macro(text: &str) -> usize {
    return_if_err!(parse(text), refused => refused.to_string().len());
    0
}
fn return_if_err_statement_by_hand(text: &str) -> usize {
    if let Err(refused) = parse(text) { return refused.to_string().len(); }
    0
}

// if_else: bound by let, and as a function's value

fn if_else_let_by_macro(ok: bool) -> u16 {
    let code = if_else!(ok => 200, 400);
    code
}
fn if_else_let_by_hand(ok: bool) -> u16 {
    let code = if ok { 200 } else { 400 };
    code
}

fn if_else_value_by_macro(ok: bool) -> &'static str {
    if_else!(ok && true => "ok", "error")
}
fn if_else_value_by_hand(ok: bool) -> &'static str {
    if ok && true { "ok" } else { "error" }
}

// table: match self with every arm a value, three and four arms, and joined patterns

impl Signal {
    fn table_three_by_macro(&self) -> u8 {
        table!(self; Signal::Term => 15, Signal::Kill => 9, Signal::Hup | Signal::Int => 1)
    }
    fn table_three_by_hand(&self) -> u8 {
        match self { Signal::Term => 15, Signal::Kill => 9, Signal::Hup | Signal::Int => 1, }
    }

    fn table_four_by_macro(&self) -> &'static str {
        table!(self; Self::Term => "term", Self::Kill => "kill", Self::Hup => "hup", Self::Int => "int")
    }
    fn table_four_by_hand(&self) -> &'static str {
        match self { Self::Term => "term", Self::Kill => "kill", Self::Hup => "hup", Self::Int => "int", }
    }
}
