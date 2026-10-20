// the macros the kit's patterns reading asked for compile where they stand in a
// function and give what the code written by hand gives
use simple_impl::{ensure_valid, if_else, ok_or_else_return, require_ok_into, require_some_into, return_if_err, table};

#[derive(Debug, PartialEq)]
struct Error(String);

impl From<String> for Error {
    fn from(text: String) -> Self {
        Error(text)
    }
}

struct Form {
    name: String,
}

impl Form {
    fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() { Err("no name".to_string()) } else { Ok(()) }
    }
}

enum Mode {
    Read,
    Write,
    Append,
    Truncate,
}

impl Mode {
    fn flag(&self) -> &'static str {
        table!(self; Mode::Read => "r", Mode::Write => "w", Mode::Append => "a", Mode::Truncate => "w+")
    }
    fn writes(&self) -> bool {
        table!(self; Self::Read => false, Self::Write | Self::Truncate => true, Self::Append => true)
    }
}

fn port(text: &str) -> Result<u16, Error> {
    let port = require_ok_into!(text.parse::<u16>(), |e| format!("port: {e}"));
    let mut low: u16 = require_ok_into!("1024".parse::<u16>(), |e| e.to_string());
    low = if_else!(port < low => 0, low);
    Ok(port + low)
}

fn byte(text: &str) -> Result<(), Error> {
    require_ok_into!(text.parse::<u8>(), |_| "wider than a byte".to_string());
    Ok(())
}

fn named(forms: &[Form]) -> Result<String, Error> {
    let first = require_some_into!(forms.first(), Error("no forms".to_string()));
    ensure_valid!(first);
    let last = require_some_into!(forms.last(), || Error("unreachable".to_string()));
    ensure_valid!(last, |e| Error(format!("last: {e}")));
    Ok(if_else!(forms.len() == 1 => first.name.clone(), format!("{}..{}", first.name, last.name)))
}

fn answer(text: &str) -> String {
    let code = ok_or_else_return!(text.parse::<u8>(), refused => format!("refused: {refused}"));
    return_if_err!(u16::try_from(code).map(|_| ()).map_err(|_| "never"), never => never.to_string());
    format!("{code}")
}

fn main() {
    assert_eq!(port("80"), Ok(80));
    assert_eq!(port("2048"), Ok(3072));
    assert_eq!(port("x"), Err(Error("port: invalid digit found in string".to_string())));
    assert_eq!(byte("7"), Ok(()));
    assert_eq!(byte("300"), Err(Error("wider than a byte".to_string())));

    let one = Form { name: "a".to_string() };
    let empty = Form { name: String::new() };
    assert_eq!(named(&[]), Err(Error("no forms".to_string())));
    assert_eq!(named(&[Form { name: "a".to_string() }]), Ok("a".to_string()));
    assert_eq!(named(&[one, Form { name: "b".to_string() }]), Ok("a..b".to_string()));
    assert_eq!(named(&[empty]), Err(Error("no name".to_string())));
    assert_eq!(named(&[Form { name: "a".to_string() }, Form { name: String::new() }]), Err(Error("last: no name".to_string())));

    assert_eq!(answer("7"), "7");
    assert_eq!(answer("x"), "refused: invalid digit found in string");

    assert_eq!(Mode::Truncate.flag(), "w+");
    assert!(!Mode::Read.writes());
    assert!(Mode::Append.writes());
    assert!(Mode::Write.writes());
}
