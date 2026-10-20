//! The toolkit: small `macro_rules!` helpers for the bodies and one-line
//! methods that are left when a derive doesn't fit. Pure checks, guards
//! that return early, early flow out of a function or a loop, conditional
//! pushes and inserts, [`simple_method!`](crate::simple_method!) for any one-expression method,
//! accessors, setters and consuming `with_*` builders, constructors and
//! one-line trait impls, and values chosen by a condition or a pattern. All
//! of them are exported at the crate root.

// -----------------------------------------------------------------------------
// Pure boolean / validation expressions
// -----------------------------------------------------------------------------

/// How many of the conditions are `true`, as a `usize`. Every condition is
/// evaluated.
///
/// ```
/// use simple_impl::count_true;
///
/// let (force, verbose, quiet) = (true, false, true);
/// assert_eq!(count_true!(force, verbose, quiet), 2);
/// assert_eq!(count_true!(), 0);
/// ```
#[macro_export]
macro_rules! count_true {
    ($($condition:expr),* $(,)?) => {
        0usize $(+ if $condition { 1usize } else { 0usize })*
    };
}

/// `true` when every condition is `true`. The conditions are joined with
/// `&&`, so they are checked left to right and stop at the first `false`.
///
/// ```
/// use simple_impl::all;
///
/// let port = 8080u16;
/// assert!(all!(port > 1024, port != 8443));
/// assert!(!all!(port > 1024, port == 22));
/// ```
#[macro_export]
macro_rules! all {
    ($($condition:expr),+ $(,)?) => { true $(&& ($condition))+ };
}

/// `true` when at least one condition is `true`. The conditions are joined
/// with `||`, so they are checked left to right and stop at the first `true`.
///
/// ```
/// use simple_impl::any;
///
/// let (file, url) = (None::<&str>, Some("https://example.invalid"));
/// assert!(any!(file.is_some(), url.is_some()));
/// assert!(!any!(false, file.is_some()));
/// ```
#[macro_export]
macro_rules! any {
    ($($condition:expr),+ $(,)?) => { false $(|| ($condition))+ };
}

/// `true` when exactly one of the conditions is `true`, through
/// [`count_true!`](crate::count_true!), so every condition is evaluated.
///
/// ```
/// use simple_impl::exactly_one;
///
/// let (file, url, stdin) = (false, true, false);
/// assert!(exactly_one!(file, url, stdin));
/// assert!(!exactly_one!(true, true, false));
/// assert!(!exactly_one!(false, false));
/// ```
#[macro_export]
macro_rules! exactly_one {
    ($($condition:expr),+ $(,)?) => { $crate::count_true!($($condition),+) == 1 };
}

/// `true` when none or one of the conditions is `true`, through
/// [`count_true!`](crate::count_true!), so every condition is evaluated.
///
/// ```
/// use simple_impl::at_most_one;
///
/// assert!(at_most_one!(false, false));
/// assert!(at_most_one!(false, true));
/// assert!(!at_most_one!(true, true));
/// ```
#[macro_export]
macro_rules! at_most_one {
    ($($condition:expr),+ $(,)?) => { $crate::count_true!($($condition),+) <= 1 };
}

/// `true` unless the left side holds and the right side does not: when
/// `left` is `true`, `right` has to be.
///
/// ```
/// use simple_impl::implies;
///
/// let (tls, cert) = (true, Some("cert.pem"));
/// assert!(implies!(tls => cert.is_some()));
/// assert!(implies!(false => false));
/// assert!(!implies!(true => false));
/// ```
#[macro_export]
macro_rules! implies {
    ($left:expr => $right:expr $(,)?) => { !($left) || ($right) };
}

/// `true` unless both sides hold: the two can't be on together.
///
/// ```
/// use simple_impl::excludes;
///
/// let (quiet, verbose) = (true, false);
/// assert!(excludes!(quiet, verbose));
/// assert!(!excludes!(true, true));
/// ```
#[macro_export]
macro_rules! excludes {
    ($left:expr, $right:expr $(,)?) => { !(($left) && ($right)) };
}

/// `true` when the value lies in `min..=max`, both ends included. The value
/// is borrowed, not moved.
///
/// ```
/// use simple_impl::in_range;
///
/// assert!(in_range!(3, 1, 4));
/// assert!(in_range!(4, 1, 4));
/// assert!(!in_range!(5, 1, 4));
/// ```
#[macro_export]
macro_rules! in_range {
    ($value:expr, $min:expr, $max:expr $(,)?) => {{
        let value = &$value;
        ($min..=$max).contains(value)
    }};
}

/// `true` when `value.is_empty()` is `false`.
///
/// ```
/// use simple_impl::nonempty;
///
/// let name = String::from("kill");
/// assert!(nonempty!(name));
/// assert!(!nonempty!(Vec::<u8>::new()));
/// ```
#[macro_export]
macro_rules! nonempty {
    ($value:expr $(,)?) => { !($value).is_empty() };
}

/// `true` when the value matches one of the patterns, through `matches!`.
/// The value comes first, then `;`, then the patterns joined with `|`.
///
/// ```
/// use simple_impl::is_one_of;
///
/// let signal = "TERM";
/// assert!(is_one_of!(signal; "TERM" | "INT" | "HUP"));
/// assert!(!is_one_of!(signal; "KILL"));
/// assert!(is_one_of!(7u8; 1..=9));
/// ```
#[macro_export]
macro_rules! is_one_of {
    ($value:expr; $($candidate:pat_param)|+ $(,)?) => {
        ::core::matches!($value, $($candidate)|+)
    };
}

// -----------------------------------------------------------------------------
// Function-body guards and Result validation
// -----------------------------------------------------------------------------

/// Return `value` from the enclosing function when the condition is `false`.
/// The return type can be anything; [`ensure!`](crate::ensure!) is the form that returns
/// `Err`.
///
/// ```
/// use simple_impl::guard;
///
/// fn port_or_default(port: u16) -> u16 {
///     guard!(port != 0 => 8080);
///     port
/// }
/// assert_eq!(port_or_default(0), 8080);
/// assert_eq!(port_or_default(22), 22);
/// ```
#[macro_export]
macro_rules! guard {
    ($condition:expr => $return_value:expr $(,)?) => {
        if !($condition) {
            return $return_value;
        }
    };
}

/// Return `Err(error)` from the enclosing function when the condition is
/// `false`.
///
/// ```
/// use simple_impl::ensure;
///
/// fn pid(pid: u32) -> Result<u32, &'static str> {
///     ensure!(pid != 0, "pid must not be 0");
///     Ok(pid)
/// }
/// assert_eq!(pid(4242), Ok(4242));
/// assert_eq!(pid(0), Err("pid must not be 0"));
/// ```
#[macro_export]
macro_rules! ensure {
    ($condition:expr, $error:expr $(,)?) => {
        if !($condition) {
            return ::core::result::Result::Err($error);
        }
    };
}

/// Return `Err(error)` from the enclosing function, unconditionally.
///
/// ```
/// use simple_impl::bail;
///
/// fn signal(name: &str) -> Result<u8, String> {
///     match name {
///         "TERM" => Ok(15),
///         "KILL" => Ok(9),
///         other => bail!(format!("unknown signal {other}")),
///     }
/// }
/// assert_eq!(signal("KILL"), Ok(9));
/// assert_eq!(signal("HUP"), Err("unknown signal HUP".to_string()));
/// ```
#[macro_export]
macro_rules! bail {
    ($error:expr $(,)?) => {
        return ::core::result::Result::Err($error)
    };
}

/// [`guard!`](crate::guard!) that runs a cleanup block before it returns.
///
/// ```
/// use simple_impl::guard_with;
///
/// fn drain(queue: &mut Vec<u32>, open: bool) -> usize {
///     guard_with!(open, { queue.clear(); } => 0);
///     queue.len()
/// }
/// let mut queue = vec![1, 2];
/// assert_eq!(drain(&mut queue, true), 2);
/// assert_eq!(drain(&mut queue, false), 0);
/// assert!(queue.is_empty());
/// ```
#[macro_export]
macro_rules! guard_with {
    ($condition:expr, $cleanup:block => $return_value:expr $(,)?) => {
        if !($condition) {
            $cleanup
            return $return_value;
        }
    };
}

/// [`ensure!`](crate::ensure!) that runs a cleanup block before it returns the error.
///
/// ```
/// use simple_impl::ensure_with;
///
/// fn stage(parts: &mut Vec<String>, ok: bool) -> Result<usize, &'static str> {
///     parts.push("--dry-run".into());
///     ensure_with!(ok, { parts.clear(); }, "not staged");
///     Ok(parts.len())
/// }
/// let mut parts = Vec::new();
/// assert_eq!(stage(&mut parts, true), Ok(1));
/// assert_eq!(stage(&mut parts, false), Err("not staged"));
/// assert!(parts.is_empty());
/// ```
#[macro_export]
macro_rules! ensure_with {
    ($condition:expr, $cleanup:block, $error:expr $(,)?) => {
        if !($condition) {
            $cleanup
            return ::core::result::Result::Err($error);
        }
    };
}

/// [`bail!`](crate::bail!) that runs a cleanup block first.
///
/// ```
/// use simple_impl::bail_with;
///
/// fn open(lock: &mut Option<u32>, path: &str) -> Result<(), String> {
///     *lock = Some(1);
///     if path.is_empty() {
///         bail_with!({ *lock = None; }, "no path".to_string());
///     }
///     Ok(())
/// }
/// let mut lock = None;
/// assert_eq!(open(&mut lock, ""), Err("no path".to_string()));
/// assert_eq!(lock, None);
/// assert_eq!(open(&mut lock, "/tmp/x"), Ok(()));
/// assert_eq!(lock, Some(1));
/// ```
#[macro_export]
macro_rules! bail_with {
    ($cleanup:block, $error:expr $(,)?) => {{
        $cleanup
        return ::core::result::Result::Err($error);
    }};
}

/// [`ensure!`](crate::ensure!) that `left == right`.
///
/// ```
/// use simple_impl::ensure_eq;
///
/// fn same_len(a: &[u8], b: &[u8]) -> Result<usize, &'static str> {
///     ensure_eq!(a.len(), b.len(), "lengths differ");
///     Ok(a.len())
/// }
/// assert_eq!(same_len(&[1, 2], &[3, 4]), Ok(2));
/// assert_eq!(same_len(&[1], &[]), Err("lengths differ"));
/// ```
#[macro_export]
macro_rules! ensure_eq {
    ($left:expr, $right:expr, $error:expr $(,)?) => {
        $crate::ensure!(($left) == ($right), $error);
    };
}

/// [`ensure!`](crate::ensure!) that `left != right`.
///
/// ```
/// use simple_impl::ensure_ne;
///
/// fn port(port: u16) -> Result<u16, &'static str> {
///     ensure_ne!(port, 0, "port 0");
///     Ok(port)
/// }
/// assert_eq!(port(22), Ok(22));
/// assert_eq!(port(0), Err("port 0"));
/// ```
#[macro_export]
macro_rules! ensure_ne {
    ($left:expr, $right:expr, $error:expr $(,)?) => {
        $crate::ensure!(($left) != ($right), $error);
    };
}

/// [`ensure!`](crate::ensure!) that the value is not empty: `!value.is_empty()`.
///
/// ```
/// use simple_impl::ensure_nonempty;
///
/// fn program(name: &str) -> Result<String, &'static str> {
///     ensure_nonempty!(name, "no program");
///     Ok(name.to_string())
/// }
/// assert_eq!(program("ls"), Ok("ls".to_string()));
/// assert_eq!(program(""), Err("no program"));
/// ```
#[macro_export]
macro_rules! ensure_nonempty {
    ($value:expr, $error:expr $(,)?) => {
        $crate::ensure!(!($value).is_empty(), $error);
    };
}

/// [`ensure!`](crate::ensure!) that the value is empty: `value.is_empty()`.
///
/// ```
/// use simple_impl::ensure_empty;
///
/// fn no_leftovers(rest: &[&str]) -> Result<(), &'static str> {
///     ensure_empty!(rest, "unexpected arguments");
///     Ok(())
/// }
/// assert_eq!(no_leftovers(&[]), Ok(()));
/// assert_eq!(no_leftovers(&["--x"]), Err("unexpected arguments"));
/// ```
#[macro_export]
macro_rules! ensure_empty {
    ($value:expr, $error:expr $(,)?) => {
        $crate::ensure!(($value).is_empty(), $error);
    };
}

/// [`ensure!`](crate::ensure!) that `value.len()` equals `expected`.
///
/// ```
/// use simple_impl::ensure_len;
///
/// fn key(bytes: &[u8]) -> Result<[u8; 4], &'static str> {
///     ensure_len!(bytes, 4, "a key is 4 bytes");
///     Ok([bytes[0], bytes[1], bytes[2], bytes[3]])
/// }
/// assert_eq!(key(&[1, 2, 3, 4]), Ok([1, 2, 3, 4]));
/// assert_eq!(key(&[1, 2]), Err("a key is 4 bytes"));
/// ```
#[macro_export]
macro_rules! ensure_len {
    ($value:expr, $expected:expr, $error:expr $(,)?) => {
        $crate::ensure!(($value).len() == ($expected), $error);
    };
}

/// [`ensure!`](crate::ensure!) that `value.len()` is at least `minimum`.
///
/// ```
/// use simple_impl::ensure_min_len;
///
/// fn password(text: &str) -> Result<&str, &'static str> {
///     ensure_min_len!(text, 8, "too short");
///     Ok(text)
/// }
/// assert_eq!(password("correct horse"), Ok("correct horse"));
/// assert_eq!(password("abc"), Err("too short"));
/// ```
#[macro_export]
macro_rules! ensure_min_len {
    ($value:expr, $minimum:expr, $error:expr $(,)?) => {
        $crate::ensure!(($value).len() >= ($minimum), $error);
    };
}

/// [`ensure!`](crate::ensure!) that `value.len()` is at most `maximum`.
///
/// ```
/// use simple_impl::ensure_max_len;
///
/// fn label(text: &str) -> Result<&str, &'static str> {
///     ensure_max_len!(text, 63, "a label is at most 63 bytes");
///     Ok(text)
/// }
/// assert_eq!(label("kit1"), Ok("kit1"));
/// assert!(label(&"x".repeat(64)).is_err());
/// ```
#[macro_export]
macro_rules! ensure_max_len {
    ($value:expr, $maximum:expr, $error:expr $(,)?) => {
        $crate::ensure!(($value).len() <= ($maximum), $error);
    };
}

/// [`ensure!`](crate::ensure!) that the value lies in `min..=max`, through [`in_range!`](crate::in_range!).
///
/// ```
/// use simple_impl::ensure_range;
///
/// fn qos(level: u8) -> Result<u8, &'static str> {
///     ensure_range!(level, 0, 2, "qos is 0, 1 or 2");
///     Ok(level)
/// }
/// assert_eq!(qos(1), Ok(1));
/// assert_eq!(qos(3), Err("qos is 0, 1 or 2"));
/// ```
#[macro_export]
macro_rules! ensure_range {
    ($value:expr, $minimum:expr, $maximum:expr, $error:expr $(,)?) => {
        $crate::ensure!($crate::in_range!($value, $minimum, $maximum), $error);
    };
}

/// One [`ensure!`](crate::ensure!) per `condition => error` pair, in order: the first
/// condition that is `false` returns its error.
///
/// ```
/// use simple_impl::ensure_all;
///
/// fn endpoint(host: &str, port: u16) -> Result<String, &'static str> {
///     ensure_all!(
///         !host.is_empty() => "no host",
///         port != 0 => "no port",
///     );
///     Ok(format!("{host}:{port}"))
/// }
/// assert_eq!(endpoint("kit1", 22), Ok("kit1:22".to_string()));
/// assert_eq!(endpoint("", 0), Err("no host"));
/// assert_eq!(endpoint("kit1", 0), Err("no port"));
/// ```
#[macro_export]
macro_rules! ensure_all {
    ($($condition:expr => $error:expr),+ $(,)?) => {
        $( $crate::ensure!($condition, $error); )+
    };
}

/// [`ensure!`](crate::ensure!) that at least one condition holds, through [`any!`](crate::any!). The
/// error comes first, then `;`, then the conditions.
///
/// ```
/// use simple_impl::ensure_any;
///
/// fn source(file: Option<&str>, url: Option<&str>) -> Result<(), &'static str> {
///     ensure_any!("give a file or a url"; file.is_some(), url.is_some());
///     Ok(())
/// }
/// assert_eq!(source(Some("a.txt"), None), Ok(()));
/// assert_eq!(source(None, None), Err("give a file or a url"));
/// ```
#[macro_export]
macro_rules! ensure_any {
    ($error:expr; $($condition:expr),+ $(,)?) => {
        $crate::ensure!($crate::any!($($condition),+), $error);
    };
}

/// [`ensure!`](crate::ensure!) that exactly one condition holds, through [`exactly_one!`](crate::exactly_one!).
/// The error comes first, then `;`, then the conditions.
///
/// ```
/// use simple_impl::ensure_exactly_one;
///
/// fn source(file: Option<&str>, url: Option<&str>) -> Result<(), &'static str> {
///     ensure_exactly_one!("give a file or a url, not both"; file.is_some(), url.is_some());
///     Ok(())
/// }
/// assert_eq!(source(None, Some("https://example.invalid")), Ok(()));
/// assert!(source(Some("a.txt"), Some("https://example.invalid")).is_err());
/// assert!(source(None, None).is_err());
/// ```
#[macro_export]
macro_rules! ensure_exactly_one {
    ($error:expr; $($condition:expr),+ $(,)?) => {
        $crate::ensure!($crate::exactly_one!($($condition),+), $error);
    };
}

/// [`ensure!`](crate::ensure!) that none or one of the conditions holds, through
/// [`at_most_one!`](crate::at_most_one!). The error comes first, then `;`, then the conditions.
///
/// ```
/// use simple_impl::ensure_at_most_one;
///
/// fn output(quiet: bool, verbose: bool) -> Result<(), &'static str> {
///     ensure_at_most_one!("quiet or verbose, not both"; quiet, verbose);
///     Ok(())
/// }
/// assert_eq!(output(false, false), Ok(()));
/// assert_eq!(output(true, false), Ok(()));
/// assert_eq!(output(true, true), Err("quiet or verbose, not both"));
/// ```
#[macro_export]
macro_rules! ensure_at_most_one {
    ($error:expr; $($condition:expr),+ $(,)?) => {
        $crate::ensure!($crate::at_most_one!($($condition),+), $error);
    };
}

/// [`ensure!`](crate::ensure!) on [`implies!`](crate::implies!): when the left side holds, the right side
/// has to.
///
/// ```
/// use simple_impl::ensure_implies;
///
/// fn tls(on: bool, cert: Option<&str>) -> Result<(), &'static str> {
///     ensure_implies!(on => cert.is_some(), "tls needs a cert");
///     Ok(())
/// }
/// assert_eq!(tls(false, None), Ok(()));
/// assert_eq!(tls(true, Some("cert.pem")), Ok(()));
/// assert_eq!(tls(true, None), Err("tls needs a cert"));
/// ```
#[macro_export]
macro_rules! ensure_implies {
    ($left:expr => $right:expr, $error:expr $(,)?) => {
        $crate::ensure!($crate::implies!($left => $right), $error);
    };
}

/// [`ensure!`](crate::ensure!) on [`excludes!`](crate::excludes!): the two sides can't both hold.
///
/// ```
/// use simple_impl::ensure_excludes;
///
/// fn output(quiet: bool, verbose: bool) -> Result<(), &'static str> {
///     ensure_excludes!(quiet, verbose, "quiet or verbose, not both");
///     Ok(())
/// }
/// assert_eq!(output(true, false), Ok(()));
/// assert_eq!(output(true, true), Err("quiet or verbose, not both"));
/// ```
#[macro_export]
macro_rules! ensure_excludes {
    ($left:expr, $right:expr, $error:expr $(,)?) => {
        $crate::ensure!($crate::excludes!($left, $right), $error);
    };
}

/// Call the value's `validate()` and, on `Err`, return the error from the
/// enclosing function through `?`, which converts it with `From` into the
/// function's error type. With a second argument the error goes through it
/// first, the way [`require_ok!`](crate::require_ok!)'s `map_error` does.
/// `ensure_valid!(value);` writes `value.validate()?;`, and
/// `ensure_valid!(value, map_error);` writes
/// `value.validate().map_err(map_error)?;`.
///
/// The kit's patterns reading found `x.validate()?;` written out 776 times
/// in 53 crates, and `x.validate().map_err(f)?;` 512 times in 30.
///
/// ```
/// use simple_impl::ensure_valid;
///
/// struct Port(u16);
///
/// impl Port {
///     fn validate(&self) -> Result<(), String> {
///         if self.0 == 0 { Err("port 0 is not a port".to_string()) } else { Ok(()) }
///     }
/// }
///
/// fn listen(port: Port) -> Result<u16, String> {
///     ensure_valid!(port);
///     Ok(port.0)
/// }
///
/// fn listen_on(port: Port) -> Result<u16, String> {
///     ensure_valid!(port, |error| format!("listen: {error}"));
///     Ok(port.0)
/// }
///
/// assert_eq!(listen(Port(22)), Ok(22));
/// assert_eq!(listen(Port(0)), Err("port 0 is not a port".to_string()));
/// assert_eq!(listen_on(Port(0)), Err("listen: port 0 is not a port".to_string()));
/// ```
#[macro_export]
macro_rules! ensure_valid {
    ($value:expr $(,)?) => {
        $value.validate()?
    };
    ($value:expr, $map_error:expr $(,)?) => {
        $value.validate().map_err($map_error)?
    };
}

// -----------------------------------------------------------------------------
// Extraction and early control flow
// -----------------------------------------------------------------------------

/// The value inside a `Some`, or return `Err(error)` from the enclosing
/// function on `None`.
///
/// ```
/// use simple_impl::require_some;
///
/// fn first_arg(args: &[&str]) -> Result<String, &'static str> {
///     let first = require_some!(args.first(), "no arguments");
///     Ok(first.to_string())
/// }
/// assert_eq!(first_arg(&["ls", "-l"]), Ok("ls".to_string()));
/// assert_eq!(first_arg(&[]), Err("no arguments"));
/// ```
#[macro_export]
macro_rules! require_some {
    ($option:expr, $error:expr $(,)?) => {
        match $option {
            ::core::option::Option::Some(value) => value,
            ::core::option::Option::None => return ::core::result::Result::Err($error),
        }
    };
}

/// The value inside an `Ok`, or return `Err(map_error(error))` from the
/// enclosing function. `map_error` is anything that can be called with the
/// error: a closure, a function, a tuple variant.
///
/// ```
/// use simple_impl::require_ok;
///
/// #[derive(Debug, PartialEq)]
/// enum Error { BadPort(std::num::ParseIntError) }
///
/// fn port(text: &str) -> Result<u16, Error> {
///     let port = require_ok!(text.parse::<u16>(), Error::BadPort);
///     Ok(port)
/// }
/// assert_eq!(port("8080"), Ok(8080));
/// assert!(matches!(port("http"), Err(Error::BadPort(_))));
/// ```
#[macro_export]
macro_rules! require_ok {
    ($result:expr, $map_error:expr $(,)?) => {
        match $result {
            ::core::result::Result::Ok(value) => value,
            ::core::result::Result::Err(error) => {
                return ::core::result::Result::Err(($map_error)(error));
            }
        }
    };
}

/// The value inside a `Some`, or return `value` from the enclosing function
/// on `None`. The return type can be anything.
///
/// ```
/// use simple_impl::some_or_return;
///
/// fn width(columns: Option<usize>) -> usize {
///     let columns = some_or_return!(columns => 80);
///     columns.min(200)
/// }
/// assert_eq!(width(None), 80);
/// assert_eq!(width(Some(120)), 120);
/// ```
#[macro_export]
macro_rules! some_or_return {
    ($option:expr => $return_value:expr $(,)?) => {
        match $option {
            ::core::option::Option::Some(value) => value,
            ::core::option::Option::None => return $return_value,
        }
    };
}

/// The value inside an `Ok`, or return `value` from the enclosing function
/// on `Err`. The error is dropped.
///
/// ```
/// use simple_impl::ok_or_return;
///
/// fn port(text: &str) -> u16 {
///     let port = ok_or_return!(text.parse::<u16>() => 8080);
///     port
/// }
/// assert_eq!(port("22"), 22);
/// assert_eq!(port("http"), 8080);
/// ```
#[macro_export]
macro_rules! ok_or_return {
    ($result:expr => $return_value:expr $(,)?) => {
        match $result {
            ::core::result::Result::Ok(value) => value,
            ::core::result::Result::Err(_) => return $return_value,
        }
    };
}

/// Return `value` from the enclosing function when the condition is `true`.
///
/// ```
/// use simple_impl::return_if;
///
/// fn columns(text: &str) -> usize {
///     return_if!(text.is_empty() => 0);
///     text.split(',').count()
/// }
/// assert_eq!(columns(""), 0);
/// assert_eq!(columns("a,b,c"), 3);
/// ```
#[macro_export]
macro_rules! return_if {
    ($condition:expr => $return_value:expr $(,)?) => {
        if $condition {
            return $return_value;
        }
    };
}

/// Return `value` from the enclosing function when the condition is
/// `false`. The same expansion as [`guard!`](crate::guard!).
///
/// ```
/// use simple_impl::return_unless;
///
/// fn first_word(text: &str) -> &str {
///     return_unless!(text.contains(' ') => text);
///     text.split(' ').next().unwrap_or(text)
/// }
/// assert_eq!(first_word("kill"), "kill");
/// assert_eq!(first_word("kill -9"), "kill");
/// ```
#[macro_export]
macro_rules! return_unless {
    ($condition:expr => $return_value:expr $(,)?) => {
        if !($condition) {
            return $return_value;
        }
    };
}

/// Return `None` from the enclosing function when the condition is `true`.
///
/// ```
/// use simple_impl::none_if;
///
/// fn half(n: u32) -> Option<u32> {
///     none_if!(n % 2 == 1);
///     Some(n / 2)
/// }
/// assert_eq!(half(8), Some(4));
/// assert_eq!(half(7), None);
/// ```
#[macro_export]
macro_rules! none_if {
    ($condition:expr $(,)?) => {
        if $condition {
            return ::core::option::Option::None;
        }
    };
}

/// `break` out of the enclosing loop when the condition is `true`. With
/// `=> value`, `break value` out of a `loop`.
///
/// ```
/// use simple_impl::break_if;
///
/// let mut seen = Vec::new();
/// for line in ["a", "b", "--", "c"] {
///     break_if!(line == "--");
///     seen.push(line);
/// }
/// assert_eq!(seen, ["a", "b"]);
///
/// let mut n = 1;
/// let first_over = loop {
///     n *= 3;
///     break_if!(n > 50 => n);
/// };
/// assert_eq!(first_over, 81);
/// ```
#[macro_export]
macro_rules! break_if {
    ($condition:expr $(,)?) => {
        if $condition {
            break;
        }
    };
    ($condition:expr => $value:expr $(,)?) => {
        if $condition {
            break $value;
        }
    };
}

/// `continue` the enclosing loop when the condition is `true`.
///
/// ```
/// use simple_impl::continue_if;
///
/// let mut kept = Vec::new();
/// for line in ["# comment", "a", "", "b"] {
///     continue_if!(line.is_empty() || line.starts_with('#'));
///     kept.push(line);
/// }
/// assert_eq!(kept, ["a", "b"]);
/// ```
#[macro_export]
macro_rules! continue_if {
    ($condition:expr $(,)?) => {
        if $condition {
            continue;
        }
    };
}

/// The value inside a `Some`, or `continue` the enclosing loop on `None`.
///
/// ```
/// use simple_impl::some_or_continue;
///
/// let mut ports = Vec::new();
/// for text in ["22", "http", "443"] {
///     let port = some_or_continue!(text.parse::<u16>().ok());
///     ports.push(port);
/// }
/// assert_eq!(ports, [22, 443]);
/// ```
#[macro_export]
macro_rules! some_or_continue {
    ($option:expr $(,)?) => {
        match $option {
            ::core::option::Option::Some(value) => value,
            ::core::option::Option::None => continue,
        }
    };
}

/// The value inside a `Some`, or `break` out of the enclosing loop on
/// `None`.
///
/// ```
/// use simple_impl::some_or_break;
///
/// let mut lines = ["a", "b"].into_iter();
/// let mut seen = Vec::new();
/// loop {
///     let line = some_or_break!(lines.next());
///     seen.push(line);
/// }
/// assert_eq!(seen, ["a", "b"]);
/// ```
#[macro_export]
macro_rules! some_or_break {
    ($option:expr $(,)?) => {
        match $option {
            ::core::option::Option::Some(value) => value,
            ::core::option::Option::None => break,
        }
    };
}

/// The value inside an `Ok`, or return `Err(map_error(error))` from the
/// enclosing function through `?`. The same as
/// [`require_ok!`](crate::require_ok!), except that `?` converts the mapped
/// error with `From` into the function's error type. It writes
/// `result.map_err(map_error)?`, so `let x = require_ok_into!(a, f);` is
/// `let x = a.map_err(f)?;`, `let mut` and a type on the binding stay
/// outside the call as they are, and the statement
/// `require_ok_into!(a, f);` is `a.map_err(f)?;`.
///
/// The kit's patterns reading found `let x = a.map_err(f)?;` written out
/// 1,756 times in 182 crates, with `let mut` 214 times in 79, with a type on
/// the binding 321 times in 78, and `a.map_err(f)?;` as a statement 1,040
/// times in 136.
///
/// ```
/// use simple_impl::require_ok_into;
/// use std::error::Error;
///
/// fn port(text: &str) -> Result<u16, Box<dyn Error>> {
///     let port: u16 = require_ok_into!(text.parse::<u16>(), |e| format!("port {text}: {e}"));
///     Ok(port)
/// }
///
/// fn check(text: &str) -> Result<(), String> {
///     require_ok_into!(text.parse::<u16>(), |e| e.to_string());
///     Ok(())
/// }
///
/// assert_eq!(port("22").unwrap(), 22);
/// assert_eq!(port("http").unwrap_err().to_string(), "port http: invalid digit found in string");
/// assert_eq!(check("http"), Err("invalid digit found in string".to_string()));
/// ```
#[macro_export]
macro_rules! require_ok_into {
    ($result:expr, $map_error:expr $(,)?) => {
        $result.map_err($map_error)?
    };
}

/// The value inside a `Some`, or return `Err(error)` from the enclosing
/// function through `?`. The same as
/// [`require_some!`](crate::require_some!), except that `?` converts the
/// error with `From` into the function's error type. Two forms:
/// `require_some_into!(option, error)` writes `option.ok_or(error)?`, which
/// builds the error before it looks at the option, as `ok_or` does, and
/// `require_some_into!(option, || error)` writes
/// `option.ok_or_else(|| error)?`, which builds it only on `None`.
///
/// The kit's patterns reading found `let x = o.ok_or_else(f)?;` written out
/// 937 times in 131 crates, and `let x = o.ok_or(e)?;` 303 times in 74.
///
/// ```
/// use simple_impl::require_some_into;
/// use std::error::Error;
///
/// fn first_arg(args: &[&str]) -> Result<String, Box<dyn Error>> {
///     let first = require_some_into!(args.first(), "no arguments");
///     Ok(first.to_string())
/// }
///
/// fn second_arg(args: &[&str]) -> Result<String, String> {
///     let second = require_some_into!(args.get(1), || format!("{} has no argument", args[0]));
///     Ok(second.to_string())
/// }
///
/// assert_eq!(first_arg(&["ls", "-l"]).unwrap(), "ls");
/// assert_eq!(first_arg(&[]).unwrap_err().to_string(), "no arguments");
/// assert_eq!(second_arg(&["ls", "-l"]), Ok("-l".to_string()));
/// assert_eq!(second_arg(&["ls"]), Err("ls has no argument".to_string()));
/// ```
#[macro_export]
macro_rules! require_some_into {
    ($option:expr, || $error:expr $(,)?) => {
        $option.ok_or_else(|| $error)?
    };
    ($option:expr, $error:expr $(,)?) => {
        $option.ok_or($error)?
    };
}

/// The value inside an `Ok`, or return `value` from the enclosing function
/// on `Err`, with the error bound under the name before `=>` so `value` can
/// use it. [`ok_or_return!`](crate::ok_or_return!) is the form that drops
/// the error. It writes
/// `match result { Ok(value) => value, Err(error) => return value, }`.
///
/// The kit's patterns reading found
/// `let x = match r { Ok(v) => v, Err(e) => return y };` written out 222
/// times in 30 crates.
///
/// ```
/// use simple_impl::ok_or_else_return;
///
/// fn status(text: &str) -> String {
///     let code = ok_or_else_return!(text.parse::<u16>(), error => format!("400 {error}"));
///     format!("{code} ok")
/// }
///
/// assert_eq!(status("200"), "200 ok");
/// assert_eq!(status("x"), "400 invalid digit found in string");
/// ```
#[macro_export]
macro_rules! ok_or_else_return {
    ($result:expr, $error:ident => $return_value:expr $(,)?) => {
        match $result {
            ::core::result::Result::Ok(value) => value,
            ::core::result::Result::Err($error) => return $return_value,
        }
    };
}

/// Return `value` from the enclosing function when the result is an `Err`,
/// with the error bound under the name before `=>` so `value` can use it.
/// The `Ok` value is dropped. It writes
/// `if let Err(error) = result { return value; }`.
///
/// The kit's patterns reading found `if let Err(e) = a { return y; }`
/// written out 144 times in 18 crates.
///
/// ```
/// use simple_impl::return_if_err;
///
/// fn answer(ask: &str) -> String {
///     return_if_err!(ask.parse::<u8>(), refused => format!("refused: {refused}"));
///     "admitted".to_string()
/// }
///
/// assert_eq!(answer("7"), "admitted");
/// assert_eq!(answer("700"), "refused: number too large to fit in target type");
/// ```
#[macro_export]
macro_rules! return_if_err {
    ($result:expr, $error:ident => $return_value:expr $(,)?) => {
        if let ::core::result::Result::Err($error) = $result {
            return $return_value;
        }
    };
}

// -----------------------------------------------------------------------------
// Collections and conditional mutation
// -----------------------------------------------------------------------------

/// Push `value` onto the collection when the condition is `true`. The value
/// is only evaluated then.
///
/// ```
/// use simple_impl::push_if;
///
/// let (verbose, force) = (true, false);
/// let mut parts = vec!["rm"];
/// push_if!(parts, verbose => "-v");
/// push_if!(parts, force => "-f");
/// assert_eq!(parts, ["rm", "-v"]);
/// ```
#[macro_export]
macro_rules! push_if {
    ($collection:expr, $condition:expr => $value:expr $(,)?) => {
        if $condition {
            ($collection).push($value);
        }
    };
}

/// Push the value inside a `Some` onto the collection; nothing on `None`.
///
/// ```
/// use simple_impl::push_some;
///
/// let mut parts = vec!["kill".to_string()];
/// push_some!(parts, Some("-9".to_string()));
/// push_some!(parts, None::<String>);
/// assert_eq!(parts, ["kill", "-9"]);
/// ```
#[macro_export]
macro_rules! push_some {
    ($collection:expr, $option:expr $(,)?) => {
        if let ::core::option::Option::Some(value) = $option {
            ($collection).push(value);
        }
    };
}

/// Extend the collection with `values` when the condition is `true`.
///
/// ```
/// use simple_impl::extend_if;
///
/// let mut parts = vec!["tar"];
/// extend_if!(parts, true => ["-c", "-f"]);
/// extend_if!(parts, false => ["-v"]);
/// assert_eq!(parts, ["tar", "-c", "-f"]);
/// ```
#[macro_export]
macro_rules! extend_if {
    ($collection:expr, $condition:expr => $values:expr $(,)?) => {
        if $condition {
            ($collection).extend($values);
        }
    };
}

/// Extend the collection with the values inside a `Some`; nothing on `None`.
///
/// ```
/// use simple_impl::extend_some;
///
/// let mut pids = vec![1u32];
/// extend_some!(pids, Some([2, 3]));
/// extend_some!(pids, None::<Vec<u32>>);
/// assert_eq!(pids, [1, 2, 3]);
/// ```
#[macro_export]
macro_rules! extend_some {
    ($collection:expr, $option:expr $(,)?) => {
        if let ::core::option::Option::Some(values) = $option {
            ($collection).extend(values);
        }
    };
}

/// `map.insert(key, value)` when the condition is `true`.
///
/// ```
/// use simple_impl::insert_if;
/// use std::collections::BTreeMap;
///
/// let mut env = BTreeMap::new();
/// insert_if!(env, true => "RUST_LOG", "debug");
/// insert_if!(env, false => "NO_COLOR", "1");
/// assert_eq!(env.len(), 1);
/// assert_eq!(env["RUST_LOG"], "debug");
/// ```
#[macro_export]
macro_rules! insert_if {
    ($map:expr, $condition:expr => $key:expr, $value:expr $(,)?) => {
        if $condition {
            ($map).insert($key, $value);
        }
    };
}

/// `map.insert(key, value)` for the value inside a `Some`; nothing on
/// `None`.
///
/// ```
/// use simple_impl::insert_some;
/// use std::collections::BTreeMap;
///
/// let mut env = BTreeMap::new();
/// insert_some!(env, "HOME" => Some("/home/james"));
/// insert_some!(env, "SHELL" => None);
/// assert_eq!(env.len(), 1);
/// assert_eq!(env["HOME"], "/home/james");
/// ```
#[macro_export]
macro_rules! insert_some {
    ($map:expr, $key:expr => $option:expr $(,)?) => {
        if let ::core::option::Option::Some(value) = $option {
            ($map).insert($key, value);
        }
    };
}

/// Assign `value` to a place when the condition is `true`. The condition
/// comes first, then `;`, then `place => value`.
///
/// ```
/// use simple_impl::assign_if;
///
/// let mut port = 80u16;
/// assign_if!(true; port => 443);
/// assign_if!(false; port => 22);
/// assert_eq!(port, 443);
/// ```
#[macro_export]
macro_rules! assign_if {
    ($condition:expr; $place:expr => $value:expr $(,)?) => {
        if $condition {
            $place = $value;
        }
    };
}

/// Assign the value inside a `Some` to a place; leave the place as it is on
/// `None`.
///
/// ```
/// use simple_impl::assign_if_some;
///
/// let mut port = 80u16;
/// assign_if_some!(port, "443".parse::<u16>().ok());
/// assign_if_some!(port, "http".parse::<u16>().ok());
/// assert_eq!(port, 443);
/// ```
#[macro_export]
macro_rules! assign_if_some {
    ($place:expr, $option:expr $(,)?) => {
        if let ::core::option::Option::Some(value) = $option {
            $place = value;
        }
    };
}

/// Push `value` unless the collection already holds an equal one. The check
/// is a scan of the collection with `==`.
///
/// ```
/// use simple_impl::push_unique;
///
/// let mut roles = vec!["deployer"];
/// push_unique!(roles, "deployer");
/// push_unique!(roles, "reader");
/// assert_eq!(roles, ["deployer", "reader"]);
/// ```
#[macro_export]
macro_rules! push_unique {
    ($collection:expr, $value:expr $(,)?) => {{
        let collection = &mut $collection;
        let value = $value;
        if !collection.iter().any(|existing| existing == &value) {
            collection.push(value);
        }
    }};
}

/// [`push_unique!`](crate::push_unique!) for every value of an iterator, in order, so a value
/// repeated inside `values` is only pushed once too.
///
/// ```
/// use simple_impl::extend_unique;
///
/// let mut roles = vec!["deployer"];
/// extend_unique!(roles, ["reader", "deployer", "reader"]);
/// assert_eq!(roles, ["deployer", "reader"]);
/// ```
#[macro_export]
macro_rules! extend_unique {
    ($collection:expr, $values:expr $(,)?) => {{
        for value in $values {
            $crate::push_unique!($collection, value);
        }
    }};
}

// -----------------------------------------------------------------------------
// Generic one-expression methods: the catch-all for trivial impl bodies
// -----------------------------------------------------------------------------

/// A method (or function) whose whole body is one expression: the
/// signature, `=>`, the expression. Without `-> T` it returns `()`.
/// `const fn` works, and attributes, doc comments included, go onto the
/// generated method.
///
/// ```
/// use simple_impl::simple_method;
///
/// struct Queue { items: Vec<u32> }
///
/// impl Queue {
///     simple_method!(pub fn len(&self) -> usize => self.items.len());
///     simple_method!(pub fn is_empty(&self) -> bool => self.items.is_empty());
///     simple_method!(
///         /// Drops every item.
///         pub fn clear(&mut self) => self.items.clear()
///     );
/// }
///
/// struct Gf256(u8);
///
/// impl Gf256 {
///     simple_method!(pub const fn is_zero(&self) -> bool => self.0 == 0);
/// }
///
/// let mut queue = Queue { items: vec![1, 2] };
/// assert_eq!(queue.len(), 2);
/// queue.clear();
/// assert!(queue.is_empty());
/// assert!(Gf256(0).is_zero());
/// ```
#[macro_export]
macro_rules! simple_method {
    ($(#[$meta:meta])* $vis:vis fn $name:ident($($args:tt)*) -> $ret:ty => $body:expr $(;)?) => {
        $(#[$meta])*
        $vis fn $name($($args)*) -> $ret {
            $body
        }
    };
    ($(#[$meta:meta])* $vis:vis fn $name:ident($($args:tt)*) => $body:expr $(;)?) => {
        $(#[$meta])*
        $vis fn $name($($args)*) {
            $body
        }
    };
    ($(#[$meta:meta])* $vis:vis const fn $name:ident($($args:tt)*) -> $ret:ty => $body:expr $(;)?) => {
        $(#[$meta])*
        $vis const fn $name($($args)*) -> $ret {
            $body
        }
    };
    ($(#[$meta:meta])* $vis:vis const fn $name:ident($($args:tt)*) => $body:expr $(;)?) => {
        $(#[$meta])*
        $vis const fn $name($($args)*) {
            $body
        }
    };
}

/// A method that returns `Result`, written as its checks and its value: the
/// signature ending in `-> T, E;`, one `condition => error;` per check
/// (each an [`ensure!`](crate::ensure!), in order), then `=> value`, which is returned as
/// `Ok(value)`.
///
/// ```
/// use simple_impl::checked_method;
///
/// struct Pool { size: usize, used: usize }
///
/// impl Pool {
///     checked_method!(
///         pub fn free(&self) -> usize, &'static str;
///         self.size > 0 => "empty pool";
///         self.used <= self.size => "over capacity";
///         => self.size - self.used
///     );
/// }
///
/// assert_eq!(Pool { size: 4, used: 1 }.free(), Ok(3));
/// assert_eq!(Pool { size: 0, used: 0 }.free(), Err("empty pool"));
/// assert_eq!(Pool { size: 2, used: 3 }.free(), Err("over capacity"));
/// ```
#[macro_export]
macro_rules! checked_method {
    (
        $(#[$meta:meta])* $vis:vis fn $name:ident($($args:tt)*) -> $ok:ty, $error_ty:ty;
        $($condition:expr => $error:expr;)*
        => $value:expr $(;)?
    ) => {
        $(#[$meta])*
        $vis fn $name($($args)*) -> ::core::result::Result<$ok, $error_ty> {
            $( $crate::ensure!($condition, $error); )*
            ::core::result::Result::Ok($value)
        }
    };
}

/// [`checked_method!`](crate::checked_method!) for a constructor: the signature has no return type,
/// only `, E;`, and the value at the end is a `Self`, returned as
/// `Ok(Self)`.
///
/// ```
/// use simple_impl::checked_constructor;
///
/// #[derive(Debug, PartialEq)]
/// struct Capacity { bytes: usize }
///
/// impl Capacity {
///     checked_constructor!(
///         pub fn new(bytes: usize), &'static str;
///         bytes > 0 => "zero";
///         bytes <= 4096 => "too large";
///         => Self { bytes }
///     );
/// }
///
/// assert_eq!(Capacity::new(64), Ok(Capacity { bytes: 64 }));
/// assert_eq!(Capacity::new(0), Err("zero"));
/// assert_eq!(Capacity::new(8192), Err("too large"));
/// ```
#[macro_export]
macro_rules! checked_constructor {
    (
        $(#[$meta:meta])* $vis:vis fn $name:ident($($args:tt)*), $error_ty:ty;
        $($condition:expr => $error:expr;)*
        => $value:expr $(;)?
    ) => {
        $(#[$meta])*
        $vis fn $name($($args)*) -> ::core::result::Result<Self, $error_ty> {
            $( $crate::ensure!($condition, $error); )*
            ::core::result::Result::Ok($value)
        }
    };
}

// -----------------------------------------------------------------------------
// Accessor / delegation method generators
// -----------------------------------------------------------------------------

/// A `&self` method that returns a field by value, so the field is `Copy`.
/// The field can be a path through nested fields, `inner.qos`, and
/// `const fn` works too.
///
/// ```
/// use simple_impl::getter;
///
/// struct Inner { qos: u8 }
/// struct Session { port: u16, inner: Inner }
///
/// impl Session {
///     getter!(pub fn port(&self) -> u16 => port);
///     getter!(pub const fn qos(&self) -> u8 => inner.qos);
/// }
///
/// let session = Session { port: 1883, inner: Inner { qos: 1 } };
/// assert_eq!(session.port(), 1883);
/// assert_eq!(session.qos(), 1);
/// ```
#[macro_export]
macro_rules! getter {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&self) -> $ret:ty => $first:ident $(.$rest:ident)* $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&self) -> $ret {
            self.$first$(.$rest)*
        }
    };
    ($(#[$meta:meta])* $vis:vis const fn $name:ident(&self) -> $ret:ty => $first:ident $(.$rest:ident)* $(;)?) => {
        $(#[$meta])*
        $vis const fn $name(&self) -> $ret {
            self.$first$(.$rest)*
        }
    };
}

/// A `&self` method that returns `&field`. The field can be a path through
/// nested fields.
///
/// ```
/// use simple_impl::getter_ref;
///
/// struct Node { name: String }
///
/// impl Node {
///     getter_ref!(pub fn name(&self) -> &String => name);
/// }
///
/// let node = Node { name: "kit1".into() };
/// assert_eq!(node.name(), "kit1");
/// ```
#[macro_export]
macro_rules! getter_ref {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&self) -> &$ret:ty => $first:ident $(.$rest:ident)* $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&self) -> &$ret {
            &self.$first$(.$rest)*
        }
    };
}

/// A `&mut self` method that returns `&mut field`. The field can be a path
/// through nested fields.
///
/// ```
/// use simple_impl::getter_mut;
///
/// struct Node { name: String }
///
/// impl Node {
///     getter_mut!(pub fn name_mut(&mut self) -> &mut String => name);
/// }
///
/// let mut node = Node { name: "kit".into() };
/// node.name_mut().push('1');
/// assert_eq!(node.name, "kit1");
/// ```
#[macro_export]
macro_rules! getter_mut {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&mut self) -> &mut $ret:ty => $first:ident $(.$rest:ident)* $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&mut self) -> &mut $ret {
            &mut self.$first$(.$rest)*
        }
    };
}

/// A `&self` method that returns an `Option<T>` field as `Option<&T>`,
/// through `as_ref()`.
///
/// ```
/// use simple_impl::getter_option_ref;
///
/// struct Policy { scope: Option<String> }
///
/// impl Policy {
///     getter_option_ref!(pub fn scope(&self) -> Option<&String> => scope);
/// }
///
/// let policy = Policy { scope: Some("ops".into()) };
/// assert_eq!(policy.scope(), Some(&"ops".to_string()));
/// assert_eq!(Policy { scope: None }.scope(), None);
/// ```
#[macro_export]
macro_rules! getter_option_ref {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&self) -> Option<&$ret:ty> => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&self) -> ::core::option::Option<&$ret> {
            self.$field.as_ref()
        }
    };
}

/// A `&self` method that returns an `Option<T>` field through `as_deref()`:
/// an `Option<String>` comes out as `Option<&str>`, an `Option<Vec<T>>` as
/// `Option<&[T]>`.
///
/// ```
/// use simple_impl::getter_option_deref;
///
/// struct Policy { scope: Option<String> }
///
/// impl Policy {
///     getter_option_deref!(pub fn scope(&self) -> Option<&str> => scope);
/// }
///
/// let policy = Policy { scope: Some("ops:deploy".into()) };
/// assert_eq!(policy.scope(), Some("ops:deploy"));
/// ```
#[macro_export]
macro_rules! getter_option_deref {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&self) -> Option<&$ret:ty> => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&self) -> ::core::option::Option<&$ret> {
            self.$field.as_deref()
        }
    };
}

/// A `&self` method that returns a `Vec<T>` field, or anything else that
/// derefs to a slice, as `&[T]`.
///
/// ```
/// use simple_impl::getter_slice;
///
/// struct Kill { pids: Vec<u32> }
///
/// impl Kill {
///     getter_slice!(pub fn pids(&self) -> &[u32] => pids);
/// }
///
/// let kill = Kill { pids: vec![12, 34] };
/// assert_eq!(kill.pids(), &[12, 34]);
/// ```
#[macro_export]
macro_rules! getter_slice {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&self) -> &[$elem:ty] => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&self) -> &[$elem] {
            &self.$field
        }
    };
}

/// A `&self` method that returns `field.len()`.
///
/// ```
/// use simple_impl::len_field;
///
/// struct Queue { items: Vec<u32> }
///
/// impl Queue {
///     len_field!(pub fn len(&self) => items);
/// }
///
/// assert_eq!(Queue { items: vec![1, 2, 3] }.len(), 3);
/// ```
#[macro_export]
macro_rules! len_field {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&self) => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&self) -> usize {
            self.$field.len()
        }
    };
}

/// A `&self` method that returns `field.is_empty()`.
///
/// ```
/// use simple_impl::is_empty_field;
///
/// struct Queue { items: Vec<u32> }
///
/// impl Queue {
///     is_empty_field!(pub fn is_empty(&self) => items);
/// }
///
/// assert!(Queue { items: vec![] }.is_empty());
/// assert!(!Queue { items: vec![1] }.is_empty());
/// ```
#[macro_export]
macro_rules! is_empty_field {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&self) => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&self) -> bool {
            self.$field.is_empty()
        }
    };
}

/// A `&self` method that returns a `bool` field.
///
/// ```
/// use simple_impl::bool_field;
///
/// struct Policy { requires_sudo: bool }
///
/// impl Policy {
///     bool_field!(pub fn requires_sudo(&self) => requires_sudo);
/// }
///
/// assert!(Policy { requires_sudo: true }.requires_sudo());
/// ```
#[macro_export]
macro_rules! bool_field {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&self) => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&self) -> bool {
            self.$field
        }
    };
}

/// A `&self` method that returns `field.contains(arg)`; for a `Vec<T>` the
/// argument is a `&T`.
///
/// ```
/// use simple_impl::contains_field;
///
/// struct Kill { pids: Vec<u32> }
///
/// impl Kill {
///     contains_field!(pub fn targets(&self, pid: &u32) => pids);
/// }
///
/// let kill = Kill { pids: vec![12, 34] };
/// assert!(kill.targets(&12));
/// assert!(!kill.targets(&56));
/// ```
#[macro_export]
macro_rules! contains_field {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&self, $arg:ident: $ty:ty) => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&self, $arg: $ty) -> bool {
            self.$field.contains($arg)
        }
    };
}

/// A `&self` method that returns the first item of a collection field the
/// condition holds for, as `Option<&T>`. After `=>` come the field, the name
/// the condition calls the item by (a `&&T`, as `iter().find` hands it
/// over), `=>`, and the condition.
///
/// ```
/// use simple_impl::find_by;
///
/// struct Member { name: String, role: u8 }
/// struct Team { members: Vec<Member> }
///
/// impl Team {
///     find_by!(pub fn by_name(&self, name: &str) -> Option<&Member> => members, member => member.name == name);
/// }
///
/// let team = Team {
///     members: vec![
///         Member { name: "ana".into(), role: 1 },
///         Member { name: "bo".into(), role: 2 },
///     ],
/// };
/// assert_eq!(team.by_name("bo").map(|m| m.role), Some(2));
/// assert!(team.by_name("cy").is_none());
/// ```
#[macro_export]
macro_rules! find_by {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&self, $arg:ident: $arg_ty:ty) -> Option<&$item_ty:ty> => $field:ident, $item:ident => $condition:expr $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&self, $arg: $arg_ty) -> ::core::option::Option<&$item_ty> {
            self.$field.iter().find(|$item| $condition)
        }
    };
}

/// A `&self` method that calls one method on a field and returns what it
/// returns. With one argument, the argument is passed through.
///
/// ```
/// use simple_impl::delegate;
/// use std::collections::HashMap;
///
/// struct Registry { names: HashMap<String, u32> }
///
/// impl Registry {
///     delegate!(pub fn len(&self) -> usize => names.len());
///     delegate!(pub fn has(&self, name: &str) -> bool => names.contains_key(name));
/// }
///
/// let registry = Registry { names: HashMap::from([("kit1".to_string(), 1)]) };
/// assert_eq!(registry.len(), 1);
/// assert!(registry.has("kit1"));
/// assert!(!registry.has("kit2"));
/// ```
#[macro_export]
macro_rules! delegate {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&self) -> $ret:ty => $field:ident.$method:ident() $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&self) -> $ret {
            self.$field.$method()
        }
    };
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&self, $arg:ident: $ty:ty) -> $ret:ty => $field:ident.$method:ident($pass:ident) $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&self, $arg: $ty) -> $ret {
            self.$field.$method($pass)
        }
    };
}

/// [`delegate!`](crate::delegate!) with `&mut self`.
///
/// ```
/// use simple_impl::delegate_mut;
///
/// struct Stack { items: Vec<u32> }
///
/// impl Stack {
///     delegate_mut!(pub fn pop(&mut self) -> Option<u32> => items.pop());
///     delegate_mut!(pub fn push(&mut self, item: u32) -> () => items.push(item));
/// }
///
/// let mut stack = Stack { items: vec![1] };
/// stack.push(2);
/// assert_eq!(stack.pop(), Some(2));
/// assert_eq!(stack.pop(), Some(1));
/// assert_eq!(stack.pop(), None);
/// ```
#[macro_export]
macro_rules! delegate_mut {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&mut self) -> $ret:ty => $field:ident.$method:ident() $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&mut self) -> $ret {
            self.$field.$method()
        }
    };
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&mut self, $arg:ident: $ty:ty) -> $ret:ty => $field:ident.$method:ident($pass:ident) $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&mut self, $arg: $ty) -> $ret {
            self.$field.$method($pass)
        }
    };
}

/// A `self` method on a tuple newtype that returns the inner value,
/// `self.0`. `const fn` works too.
///
/// ```
/// use simple_impl::newtype_value;
///
/// #[derive(Clone, Copy)]
/// struct Gf256(u8);
///
/// impl Gf256 {
///     newtype_value!(pub const fn value(self) -> u8);
/// }
///
/// assert_eq!(Gf256(7).value(), 7);
/// ```
#[macro_export]
macro_rules! newtype_value {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(self) -> $ret:ty $(;)?) => {
        $(#[$meta])*
        $vis fn $name(self) -> $ret { self.0 }
    };
    ($(#[$meta:meta])* $vis:vis const fn $name:ident(self) -> $ret:ty $(;)?) => {
        $(#[$meta])*
        $vis const fn $name(self) -> $ret { self.0 }
    };
}

/// A `&self` method on a tuple newtype that returns `&self.0`.
///
/// ```
/// use simple_impl::newtype_ref;
///
/// struct Digest(Vec<u8>);
///
/// impl Digest {
///     newtype_ref!(pub fn bytes(&self) -> &Vec<u8>);
/// }
///
/// assert_eq!(Digest(vec![1, 2]).bytes(), &[1, 2]);
/// ```
#[macro_export]
macro_rules! newtype_ref {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&self) -> &$ret:ty $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&self) -> &$ret { &self.0 }
    };
}

// -----------------------------------------------------------------------------
// Setters and consuming builders
// -----------------------------------------------------------------------------

/// A `&mut self` setter that assigns the argument to a field. The field can
/// be a path through nested fields.
///
/// ```
/// use simple_impl::setter;
///
/// struct Inner { qos: u8 }
/// struct Session { inner: Inner }
///
/// impl Session {
///     setter!(pub fn set_qos(&mut self, qos: u8) => inner.qos);
/// }
///
/// let mut session = Session { inner: Inner { qos: 0 } };
/// session.set_qos(2);
/// assert_eq!(session.inner.qos, 2);
/// ```
#[macro_export]
macro_rules! setter {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&mut self, $arg:ident: $ty:ty) => $first:ident $(.$rest:ident)* $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&mut self, $arg: $ty) {
            self.$first$(.$rest)* = $arg;
        }
    };
}

/// [`setter!`](crate::setter!) taking `impl Into<T>`: the type written in the signature is
/// the field's, and the method takes anything that converts into it.
///
/// ```
/// use simple_impl::setter_into;
///
/// struct Node { name: String }
///
/// impl Node {
///     setter_into!(pub fn set_name(&mut self, name: String) => name);
/// }
///
/// let mut node = Node { name: String::new() };
/// node.set_name("kit1");
/// assert_eq!(node.name, "kit1");
/// ```
#[macro_export]
macro_rules! setter_into {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&mut self, $arg:ident: $ty:ty) => $first:ident $(.$rest:ident)* $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&mut self, $arg: impl ::core::convert::Into<$ty>) {
            self.$first$(.$rest)* = $arg.into();
        }
    };
}

/// A `&mut self` setter that stores `Some(arg)` in an `Option<T>` field.
///
/// ```
/// use simple_impl::setter_some;
///
/// struct Kill { signal: Option<u8> }
///
/// impl Kill {
///     setter_some!(pub fn set_signal(&mut self, signal: u8) => signal);
/// }
///
/// let mut kill = Kill { signal: None };
/// kill.set_signal(9);
/// assert_eq!(kill.signal, Some(9));
/// ```
#[macro_export]
macro_rules! setter_some {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&mut self, $arg:ident: $ty:ty) => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&mut self, $arg: $ty) {
            self.$field = ::core::option::Option::Some($arg);
        }
    };
}

/// [`setter_some!`](crate::setter_some!) taking `impl Into<T>`.
///
/// ```
/// use simple_impl::setter_some_into;
///
/// struct Policy { scope: Option<String> }
///
/// impl Policy {
///     setter_some_into!(pub fn set_scope(&mut self, scope: String) => scope);
/// }
///
/// let mut policy = Policy { scope: None };
/// policy.set_scope("ops");
/// assert_eq!(policy.scope.as_deref(), Some("ops"));
/// ```
#[macro_export]
macro_rules! setter_some_into {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&mut self, $arg:ident: $ty:ty) => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&mut self, $arg: impl ::core::convert::Into<$ty>) {
            self.$field = ::core::option::Option::Some($arg.into());
        }
    };
}

/// A `&mut self` method that calls `clear()` on a field.
///
/// ```
/// use simple_impl::clear_collection;
///
/// struct Queue { items: Vec<u32> }
///
/// impl Queue {
///     clear_collection!(pub fn clear(&mut self) => items);
/// }
///
/// let mut queue = Queue { items: vec![1, 2] };
/// queue.clear();
/// assert!(queue.items.is_empty());
/// ```
#[macro_export]
macro_rules! clear_collection {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&mut self) => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&mut self) {
            self.$field.clear();
        }
    };
}

/// A `&mut self` method that takes an `Option<T>` field out, leaving
/// `None`.
///
/// ```
/// use simple_impl::take_field;
///
/// struct Slot { job: Option<String> }
///
/// impl Slot {
///     take_field!(pub fn take_job(&mut self) -> Option<String> => job);
/// }
///
/// let mut slot = Slot { job: Some("build".into()) };
/// assert_eq!(slot.take_job().as_deref(), Some("build"));
/// assert_eq!(slot.take_job(), None);
/// ```
#[macro_export]
macro_rules! take_field {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&mut self) -> Option<$ty:ty> => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&mut self) -> ::core::option::Option<$ty> {
            self.$field.take()
        }
    };
}

/// A `&mut self` method that sets a `bool` field to `true`.
///
/// ```
/// use simple_impl::enable_field;
///
/// struct Node { serving: bool }
///
/// impl Node {
///     enable_field!(pub fn serve(&mut self) => serving);
/// }
///
/// let mut node = Node { serving: false };
/// node.serve();
/// assert!(node.serving);
/// ```
#[macro_export]
macro_rules! enable_field {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&mut self) => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&mut self) {
            self.$field = true;
        }
    };
}

/// A `&mut self` method that sets a `bool` field to `false`.
///
/// ```
/// use simple_impl::disable_field;
///
/// struct Node { serving: bool }
///
/// impl Node {
///     disable_field!(pub fn stop(&mut self) => serving);
/// }
///
/// let mut node = Node { serving: true };
/// node.stop();
/// assert!(!node.serving);
/// ```
#[macro_export]
macro_rules! disable_field {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&mut self) => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&mut self) {
            self.$field = false;
        }
    };
}

/// A `&mut self` method that flips a `bool` field.
///
/// ```
/// use simple_impl::toggle_field;
///
/// struct Node { serving: bool }
///
/// impl Node {
///     toggle_field!(pub fn toggle(&mut self) => serving);
/// }
///
/// let mut node = Node { serving: false };
/// node.toggle();
/// assert!(node.serving);
/// node.toggle();
/// assert!(!node.serving);
/// ```
#[macro_export]
macro_rules! toggle_field {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(&mut self) => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(&mut self) {
            self.$field = !self.$field;
        }
    };
}

/// A consuming `with_*` setter: it takes `mut self` and the value, assigns
/// the value to the field and returns `self`. The field can be a path
/// through nested fields, and `const fn` works too.
///
/// ```
/// use simple_impl::with_field;
///
/// #[derive(Default)]
/// struct Inner { qos: u8 }
///
/// #[derive(Default)]
/// struct Session { port: u16, inner: Inner }
///
/// impl Session {
///     with_field!(pub fn with_port(mut self, port: u16) -> Self => port);
///     with_field!(pub fn with_qos(mut self, qos: u8) -> Self => inner.qos);
/// }
///
/// #[derive(Clone, Copy)]
/// struct Limits { max: u32 }
///
/// impl Limits {
///     with_field!(pub const fn with_max(mut self, max: u32) -> Self => max);
/// }
///
/// let session = Session::default().with_port(1883).with_qos(1);
/// assert_eq!((session.port, session.inner.qos), (1883, 1));
///
/// const LIMITS: Limits = Limits { max: 0 }.with_max(64);
/// assert_eq!(LIMITS.max, 64);
/// ```
#[macro_export]
macro_rules! with_field {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(mut self, $arg:ident: $ty:ty) -> Self => $first:ident $(.$rest:ident)* $(;)?) => {
        $(#[$meta])*
        $vis fn $name(mut self, $arg: $ty) -> Self {
            self.$first$(.$rest)* = $arg;
            self
        }
    };
    ($(#[$meta:meta])* $vis:vis const fn $name:ident(mut self, $arg:ident: $ty:ty) -> Self => $first:ident $(.$rest:ident)* $(;)?) => {
        $(#[$meta])*
        $vis const fn $name(mut self, $arg: $ty) -> Self {
            self.$first$(.$rest)* = $arg;
            self
        }
    };
}

/// [`with_field!`](crate::with_field!) taking `impl Into<T>`.
///
/// ```
/// use simple_impl::with_into;
///
/// #[derive(Default)]
/// struct Node { name: String }
///
/// impl Node {
///     with_into!(pub fn with_name(mut self, name: String) -> Self => name);
/// }
///
/// assert_eq!(Node::default().with_name("kit1").name, "kit1");
/// ```
#[macro_export]
macro_rules! with_into {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(mut self, $arg:ident: $ty:ty) -> Self => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(mut self, $arg: impl ::core::convert::Into<$ty>) -> Self {
            self.$field = $arg.into();
            self
        }
    };
}

/// A consuming setter that stores `Some(arg)` in an `Option<T>` field and
/// returns `self`.
///
/// ```
/// use simple_impl::with_some;
///
/// #[derive(Default)]
/// struct Kill { signal: Option<u8> }
///
/// impl Kill {
///     with_some!(pub fn with_signal(mut self, signal: u8) -> Self => signal);
/// }
///
/// assert_eq!(Kill::default().with_signal(9).signal, Some(9));
/// ```
#[macro_export]
macro_rules! with_some {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(mut self, $arg:ident: $ty:ty) -> Self => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(mut self, $arg: $ty) -> Self {
            self.$field = ::core::option::Option::Some($arg);
            self
        }
    };
}

/// [`with_some!`](crate::with_some!) taking `impl Into<T>`.
///
/// ```
/// use simple_impl::with_some_into;
///
/// #[derive(Default)]
/// struct Policy { iam_scope: Option<String> }
///
/// impl Policy {
///     with_some_into!(pub fn with_iam_scope(mut self, scope: String) -> Self => iam_scope);
/// }
///
/// let policy = Policy::default().with_iam_scope("ops:deploy");
/// assert_eq!(policy.iam_scope.as_deref(), Some("ops:deploy"));
/// ```
#[macro_export]
macro_rules! with_some_into {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(mut self, $arg:ident: $ty:ty) -> Self => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(mut self, $arg: impl ::core::convert::Into<$ty>) -> Self {
            self.$field = ::core::option::Option::Some($arg.into());
            self
        }
    };
}

/// A consuming method that pushes the argument onto a `Vec` field and
/// returns `self`.
///
/// ```
/// use simple_impl::with_push;
///
/// #[derive(Default)]
/// struct Kill { pids: Vec<u32> }
///
/// impl Kill {
///     with_push!(pub fn with_pid(mut self, pid: u32) -> Self => pids);
/// }
///
/// assert_eq!(Kill::default().with_pid(12).with_pid(34).pids, [12, 34]);
/// ```
#[macro_export]
macro_rules! with_push {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(mut self, $arg:ident: $ty:ty) -> Self => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(mut self, $arg: $ty) -> Self {
            self.$field.push($arg);
            self
        }
    };
}

/// [`with_push!`](crate::with_push!) taking `impl Into<T>`.
///
/// ```
/// use simple_impl::with_push_into;
///
/// #[derive(Default)]
/// struct Command { args: Vec<String> }
///
/// impl Command {
///     with_push_into!(pub fn with_arg(mut self, arg: String) -> Self => args);
/// }
///
/// assert_eq!(Command::default().with_arg("-l").with_arg("-a").args, ["-l", "-a"]);
/// ```
#[macro_export]
macro_rules! with_push_into {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(mut self, $arg:ident: $ty:ty) -> Self => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(mut self, $arg: impl ::core::convert::Into<$ty>) -> Self {
            self.$field.push($arg.into());
            self
        }
    };
}

/// A consuming method that extends a collection field with any
/// `IntoIterator<Item = T>` and returns `self`. The argument is named
/// without a type; the element type comes after the field: `=> field: T`.
///
/// ```
/// use simple_impl::with_extend;
///
/// #[derive(Default)]
/// struct Kill { pids: Vec<u32> }
///
/// impl Kill {
///     with_extend!(pub fn with_pids(mut self, pids) -> Self => pids: u32);
/// }
///
/// let kill = Kill::default().with_pids([1, 2]).with_pids((3..=4).map(|n| n * 10));
/// assert_eq!(kill.pids, [1, 2, 30, 40]);
/// ```
#[macro_export]
macro_rules! with_extend {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(mut self, $arg:ident) -> Self => $field:ident: $elem:ty $(;)?) => {
        $(#[$meta])*
        $vis fn $name<I>(mut self, $arg: I) -> Self
        where
            I: ::core::iter::IntoIterator<Item = $elem>,
        {
            self.$field.extend($arg);
            self
        }
    };
}

/// A consuming method that sets a `bool` field to `true` and returns
/// `self`.
///
/// ```
/// use simple_impl::with_flag;
///
/// #[derive(Default)]
/// struct Rm { force: bool }
///
/// impl Rm {
///     with_flag!(pub fn force(mut self) -> Self => force);
/// }
///
/// assert!(Rm::default().force().force);
/// ```
#[macro_export]
macro_rules! with_flag {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(mut self) -> Self => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(mut self) -> Self {
            self.$field = true;
            self
        }
    };
}

/// A consuming method that sets a `bool` field to `false` and returns
/// `self`.
///
/// ```
/// use simple_impl::without_flag;
///
/// struct Ls { color: bool }
///
/// impl Ls {
///     without_flag!(pub fn no_color(mut self) -> Self => color);
/// }
///
/// assert!(!Ls { color: true }.no_color().color);
/// ```
#[macro_export]
macro_rules! without_flag {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(mut self) -> Self => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(mut self) -> Self {
            self.$field = false;
            self
        }
    };
}

/// A consuming method that pushes the argument onto a `Vec` field through
/// [`push_unique!`](crate::push_unique!), so a value already there is not added twice, and
/// returns `self`.
///
/// ```
/// use simple_impl::with_unique_push;
///
/// #[derive(Default)]
/// struct Policy { path_roles: Vec<String> }
///
/// impl Policy {
///     with_unique_push!(pub fn with_path_role(mut self, role: String) -> Self => path_roles);
/// }
///
/// let policy = Policy::default()
///     .with_path_role("deployer".into())
///     .with_path_role("deployer".into())
///     .with_path_role("reader".into());
/// assert_eq!(policy.path_roles, ["deployer", "reader"]);
/// ```
#[macro_export]
macro_rules! with_unique_push {
    ($(#[$meta:meta])* $vis:vis fn $name:ident(mut self, $arg:ident: $ty:ty) -> Self => $field:ident $(;)?) => {
        $(#[$meta])*
        $vis fn $name(mut self, $arg: $ty) -> Self {
            $crate::push_unique!(self.$field, $arg);
            self
        }
    };
}

// -----------------------------------------------------------------------------
// Constructors and trait boilerplate
// -----------------------------------------------------------------------------

/// A constructor for a tuple newtype: it takes the inner value and returns
/// `Self(value)`. `const fn` works too.
///
/// ```
/// use simple_impl::newtype_constructor;
///
/// struct Port(u16);
///
/// impl Port {
///     newtype_constructor!(pub const fn new(port: u16) -> Self);
/// }
///
/// const SSH: Port = Port::new(22);
/// assert_eq!(SSH.0, 22);
/// ```
#[macro_export]
macro_rules! newtype_constructor {
    ($(#[$meta:meta])* $vis:vis fn $name:ident($arg:ident: $ty:ty) -> Self $(;)?) => {
        $(#[$meta])*
        $vis fn $name($arg: $ty) -> Self { Self($arg) }
    };
    ($(#[$meta:meta])* $vis:vis const fn $name:ident($arg:ident: $ty:ty) -> Self $(;)?) => {
        $(#[$meta])*
        $vis const fn $name($arg: $ty) -> Self { Self($arg) }
    };
}

/// `impl Default` that calls `Self::new()`.
///
/// ```
/// use simple_impl::impl_default_via_new;
///
/// struct Pool { size: usize }
///
/// impl Pool {
///     fn new() -> Self { Self { size: 8 } }
/// }
///
/// impl_default_via_new!(Pool);
///
/// assert_eq!(Pool::default().size, 8);
/// ```
#[macro_export]
macro_rules! impl_default_via_new {
    ($ty:ty $(;)?) => {
        impl ::core::default::Default for $ty {
            fn default() -> Self { Self::new() }
        }
    };
}

/// `impl From<Inner> for Wrapper` that wraps the value: `Self(value)`.
///
/// ```
/// use simple_impl::impl_from_newtype;
///
/// struct Port(u16);
///
/// impl_from_newtype!(Port, u16);
///
/// let port: Port = 22.into();
/// assert_eq!(port.0, 22);
/// ```
#[macro_export]
macro_rules! impl_from_newtype {
    ($wrapper:ty, $inner:ty $(;)?) => {
        impl ::core::convert::From<$inner> for $wrapper {
            fn from(value: $inner) -> Self { Self(value) }
        }
    };
}

/// `impl AsRef<Inner>` for a tuple newtype, returning `&self.0`.
///
/// ```
/// use simple_impl::impl_as_ref_newtype;
///
/// struct Name(String);
///
/// impl_as_ref_newtype!(Name, String);
///
/// let name = Name("kit1".into());
/// assert_eq!(name.as_ref(), "kit1");
/// ```
#[macro_export]
macro_rules! impl_as_ref_newtype {
    ($wrapper:ty, $inner:ty $(;)?) => {
        impl ::core::convert::AsRef<$inner> for $wrapper {
            fn as_ref(&self) -> &$inner { &self.0 }
        }
    };
}

/// `impl Deref<Target = T>` for a tuple newtype, returning `&self.0`.
///
/// ```
/// use simple_impl::impl_deref_newtype;
///
/// struct Roles(Vec<String>);
///
/// impl_deref_newtype!(Roles, Vec<String>);
///
/// let roles = Roles(vec!["deployer".into()]);
/// assert_eq!(roles.len(), 1);
/// assert_eq!(roles[0], "deployer");
/// ```
#[macro_export]
macro_rules! impl_deref_newtype {
    ($wrapper:ty, $target:ty $(;)?) => {
        impl ::core::ops::Deref for $wrapper {
            type Target = $target;
            fn deref(&self) -> &Self::Target { &self.0 }
        }
    };
}

/// `impl DerefMut` for a tuple newtype, returning `&mut self.0`. The type
/// needs its `Deref` first, from [`impl_deref_newtype!`](crate::impl_deref_newtype!) or by hand.
///
/// ```
/// use simple_impl::{impl_deref_mut_newtype, impl_deref_newtype};
///
/// struct Roles(Vec<String>);
///
/// impl_deref_newtype!(Roles, Vec<String>);
/// impl_deref_mut_newtype!(Roles);
///
/// let mut roles = Roles(Vec::new());
/// roles.push("reader".into());
/// assert_eq!(roles.0, ["reader"]);
/// ```
#[macro_export]
macro_rules! impl_deref_mut_newtype {
    ($wrapper:ty $(;)?) => {
        impl ::core::ops::DerefMut for $wrapper {
            fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
        }
    };
}

/// `impl Display` that writes what one `&self` method returns, through
/// `{}`.
///
/// ```
/// use simple_impl::impl_display_via;
///
/// struct Endpoint { host: String, port: u16 }
///
/// impl Endpoint {
///     fn address(&self) -> String { format!("{}:{}", self.host, self.port) }
/// }
///
/// impl_display_via!(Endpoint, address);
///
/// let endpoint = Endpoint { host: "kit1".into(), port: 22 };
/// assert_eq!(endpoint.to_string(), "kit1:22");
/// ```
#[macro_export]
macro_rules! impl_display_via {
    ($ty:ty, $method:ident $(;)?) => {
        impl ::core::fmt::Display for $ty {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "{}", self.$method())
            }
        }
    };
}

/// `impl Debug` that writes a fixed text and nothing of the value, for a
/// type that holds a secret.
///
/// ```
/// use simple_impl::impl_debug_redacted;
///
/// struct Secret(String);
///
/// impl_debug_redacted!(Secret, "Secret([REDACTED])");
///
/// let secret = Secret("hunter2".into());
/// assert_eq!(format!("{secret:?}"), "Secret([REDACTED])");
/// assert_eq!(secret.0.len(), 7);
/// ```
#[macro_export]
macro_rules! impl_debug_redacted {
    ($ty:ty, $text:expr $(;)?) => {
        impl ::core::fmt::Debug for $ty {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str($text)
            }
        }
    };
}

/// An empty `impl std::error::Error`; the type needs `Debug` and `Display`.
///
/// ```
/// use simple_impl::impl_std_error;
/// use std::fmt;
///
/// #[derive(Debug)]
/// struct NoHost;
///
/// impl fmt::Display for NoHost {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str("no host") }
/// }
///
/// impl_std_error!(NoHost);
///
/// let error: Box<dyn std::error::Error> = Box::new(NoHost);
/// assert_eq!(error.to_string(), "no host");
/// ```
#[macro_export]
macro_rules! impl_std_error {
    ($ty:ty $(;)?) => {
        impl ::std::error::Error for $ty {}
    };
}

/// `impl PartialOrd` that returns `Some(self.cmp(other))`, for a type whose
/// `Ord` is written by hand.
///
/// ```
/// use simple_impl::impl_partial_ord_via_ord;
/// use std::cmp::Ordering;
///
/// #[derive(PartialEq, Eq)]
/// struct Version { major: u32, minor: u32 }
///
/// impl Ord for Version {
///     fn cmp(&self, other: &Self) -> Ordering {
///         (self.major, self.minor).cmp(&(other.major, other.minor))
///     }
/// }
///
/// impl_partial_ord_via_ord!(Version);
///
/// assert!(Version { major: 0, minor: 2 } < Version { major: 1, minor: 0 });
/// ```
#[macro_export]
macro_rules! impl_partial_ord_via_ord {
    ($ty:ty $(;)?) => {
        impl ::core::cmp::PartialOrd for $ty {
            fn partial_cmp(&self, other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::option::Option::Some(self.cmp(other))
            }
        }
    };
}

/// `impl From<Source> for Target` that wraps the value in one tuple variant,
/// `Self::Variant(value)`, so `?` lifts the source error into the target.
///
/// ```
/// use simple_impl::impl_from_variant;
/// use std::num::ParseIntError;
///
/// #[derive(Debug)]
/// enum Error { Port(ParseIntError) }
///
/// impl_from_variant!(ParseIntError => Error, Port);
///
/// fn port(text: &str) -> Result<u16, Error> {
///     Ok(text.parse::<u16>()?)
/// }
/// assert_eq!(port("22").unwrap(), 22);
/// assert!(matches!(port("http"), Err(Error::Port(_))));
/// ```
#[macro_export]
macro_rules! impl_from_variant {
    ($source:ty => $target:ty, $variant:ident $(;)?) => {
        impl ::core::convert::From<$source> for $target {
            fn from(error: $source) -> Self {
                Self::$variant(error)
            }
        }
    };
}

/// A trait method whose body calls a path with the receiver, and the one
/// argument when there is one. Four receivers: `self`, `self` and an
/// argument, `&self`, `&self` and an argument.
///
/// ```
/// use simple_impl::forward_trait_method;
/// use std::ops::Add;
///
/// #[derive(Clone, Copy, Debug, PartialEq)]
/// struct Gf256(u8);
///
/// impl Gf256 {
///     fn add_impl(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
///     fn describe(&self) -> String { format!("gf({})", self.0) }
/// }
///
/// trait Describe { fn describe(&self) -> String; }
///
/// impl Add for Gf256 {
///     type Output = Self;
///     forward_trait_method!(fn add(self, rhs: Self) -> Self => Gf256::add_impl);
/// }
///
/// impl Describe for Gf256 {
///     forward_trait_method!(fn describe(&self) -> String => Gf256::describe);
/// }
///
/// assert_eq!(Gf256(2) + Gf256(3), Gf256(1));
/// assert_eq!(Describe::describe(&Gf256(5)), "gf(5)");
/// ```
#[macro_export]
macro_rules! forward_trait_method {
    ($(#[$meta:meta])* fn $name:ident(self) -> $ret:ty => $target:path $(;)?) => {
        $(#[$meta])*
        fn $name(self) -> $ret { $target(self) }
    };
    ($(#[$meta:meta])* fn $name:ident(self, $arg:ident: $ty:ty) -> $ret:ty => $target:path $(;)?) => {
        $(#[$meta])*
        fn $name(self, $arg: $ty) -> $ret { $target(self, $arg) }
    };
    ($(#[$meta:meta])* fn $name:ident(&self) -> $ret:ty => $target:path $(;)?) => {
        $(#[$meta])*
        fn $name(&self) -> $ret { $target(self) }
    };
    ($(#[$meta:meta])* fn $name:ident(&self, $arg:ident: $ty:ty) -> $ret:ty => $target:path $(;)?) => {
        $(#[$meta])*
        fn $name(&self, $arg: $ty) -> $ret { $target(self, $arg) }
    };
}

// -----------------------------------------------------------------------------
// Values chosen by a condition or by a pattern
// -----------------------------------------------------------------------------

/// `then` when the condition is `true`, `otherwise` when it is `false`.
/// Only the one chosen is evaluated. It writes
/// `if condition { then } else { otherwise }`.
///
/// The kit's patterns reading found `let x = if c { a } else { b };`
/// written out 688 times in 111 crates, and `if c { a } else { b }` as a
/// function's value 332 times in 80.
///
/// ```
/// use simple_impl::if_else;
///
/// fn status(ok: bool) -> &'static str {
///     if_else!(ok => "ok", "error")
/// }
///
/// let width = if_else!(status(true).len() > 4 => 80, 120);
/// assert_eq!(status(false), "error");
/// assert_eq!(width, 120);
/// ```
#[macro_export]
macro_rules! if_else {
    ($condition:expr => $then:expr, $otherwise:expr $(,)?) => {
        if $condition { $then } else { $otherwise }
    };
}

/// A table from patterns to values: the value of the first entry whose
/// pattern matches. The value comes first, then `;`, then
/// `pattern => entry` pairs; one entry can take several patterns joined
/// with `|`, as in a `match`. It writes
/// `match value { pattern => entry, ... }`.
///
/// The kit's patterns reading found `match self` with every arm a value
/// written out with three arms 134 times in 47 crates, and with four arms
/// 115 times in 40.
///
/// ```
/// use simple_impl::table;
///
/// enum Signal { Term, Kill, Hup, Int }
///
/// impl Signal {
///     fn number(&self) -> u8 {
///         table!(self; Signal::Term => 15, Signal::Kill => 9, Signal::Hup => 1, Signal::Int => 2)
///     }
///     fn stops(&self) -> bool {
///         table!(self; Signal::Term | Signal::Kill | Signal::Int => true, Signal::Hup => false)
///     }
/// }
///
/// assert_eq!(Signal::Kill.number(), 9);
/// assert!(!Signal::Hup.stops());
/// assert!(Signal::Term.stops());
/// ```
#[macro_export]
macro_rules! table {
    ($value:expr; $($($pattern:pat_param)|+ => $entry:expr),+ $(,)?) => {
        match $value {
            $($($pattern)|+ => $entry,)+
        }
    };
}
