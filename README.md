# simple_impl

Macros that make a command-line builder short.

Two families, both `macro_rules!`, both exported at the root:

- `sc_*` build the argument vector of a shell command from a struct's fields.
- `builder_*` write the setters of a builder inside an `impl` block.

```rust
use simple_impl::{builder_flag, builder_new_default, builder_opt_into, builder_vec_iter};
use simple_impl::{sc_cmd, sc_each_display, sc_flags, sc_if_some_prefix};

#[derive(Default)]
struct Kill {
    signal: Option<String>,
    pids: Vec<u32>,
    verbose: bool,
}

impl Kill {
    builder_new_default!();
    builder_opt_into!(signal => signal: String);
    builder_vec_iter!(pids => pids: u32);
    builder_flag!(verbose => verbose);

    fn args(&self) -> Vec<String> {
        let mut parts = sc_cmd!("kill");
        sc_flags!(parts, self, verbose => "--verbose");
        sc_if_some_prefix!(parts, self.signal, "-");
        sc_each_display!(parts, &self.pids);
        parts
    }
}

let cmd = Kill::new().signal("TERM").pids([12, 34]).verbose();
assert_eq!(cmd.args(), ["kill", "--verbose", "-TERM", "12", "34"]);
```

## The macros

| macro | does |
|---|---|
| `sc_cmd!("name")` | the vector, started with the command name |
| `sc_flags!(parts, self, field => "--flag", ...)` | a flag for every `bool` that is `true` |
| `sc_flags_into!` | the same with `.into()` |
| `sc_opt_kv!(parts, self, field => "--flag", ...)` | `--flag value` for every `Option` that is `Some` |
| `sc_args!(parts, expr, ...)` | positional arguments |
| `sc_if_some!(parts, opt, \|v\| ...)` | the mapped value of a `Some` |
| `sc_if_some_display!` / `sc_if_some_into!` / `sc_if_some_clone!` | `to_string` / `into` / `clone` of a `Some` |
| `sc_if_some_prefix!(parts, opt, "-")` | `"-value"` for a `Some` |
| `sc_if_some_flag!(parts, opt, "--flag")` | `--flag value` for a `Some` |
| `sc_each!(parts, iter, \|item\| ...)` / `sc_each_display!` | one entry per item |
| `builder_new_default!()` | `new()` from `Default` |
| `builder_opt!(name => field: T)` / `builder_opt_into!` | setters for `Option<T>` |
| `builder_set!(name => field: T)` | a setter for a plain field |
| `builder_vec_into!` / `builder_vec_iter!` / `builder_push!` | setters for `Vec<T>` |
| `builder_flag!(name => field)` | a method that sets a `bool` |

Every macro's documentation is a compiled example.

With the `derive` feature, `simple_impl_derive`'s `#[derive(SimpleBuilder)]`
and `#[derive(SimpleShell)]` generate these shapes from attributes on the
struct, expanding to these macros.

## The toolkit

100 more `macro_rules!` helpers, also exported at the root, for the small
bodies and one-line methods that are left when a derive doesn't fit. They
come from shapes that repeat across the jameshuntt repositories. Each family
below with one example; every macro's documentation has its own.

**Pure checks (10)**: `count_true!`, `all!`, `any!`, `exactly_one!`,
`at_most_one!`, `implies!`, `excludes!`, `in_range!`, `nonempty!`,
`is_one_of!`. They return a value and never return early.

```rust
use simple_impl::{exactly_one, implies};

let (file, url, tls, cert) = (false, true, true, Some("cert.pem"));
assert!(exactly_one!(file, url));
assert!(implies!(tls => cert.is_some()));
```

**Result / guard layer (20)**: `guard!`, `ensure!`, `bail!`, `guard_with!`,
`ensure_with!`, `bail_with!`, `ensure_eq!`, `ensure_ne!`, `ensure_nonempty!`,
`ensure_empty!`, `ensure_len!`, `ensure_min_len!`, `ensure_max_len!`,
`ensure_range!`, `ensure_all!`, `ensure_any!`, `ensure_exactly_one!`,
`ensure_at_most_one!`, `ensure_implies!`, `ensure_excludes!`. `guard!`
returns any value; `ensure!` and the `ensure_*` family return `Err(...)`.

```rust
use simple_impl::{ensure, ensure_range};

fn qos(level: u8, retained: bool) -> Result<u8, &'static str> {
    ensure!(!retained || level > 0, "retained needs qos 1 or 2");
    ensure_range!(level, 0, 2, "qos is 0, 1 or 2");
    Ok(level)
}
assert_eq!(qos(1, true), Ok(1));
assert_eq!(qos(3, false), Err("qos is 0, 1 or 2"));
```

**Extraction / early flow (11)**: `require_some!`, `require_ok!`,
`some_or_return!`, `ok_or_return!`, `return_if!`, `return_unless!`,
`none_if!`, `break_if!`, `continue_if!`, `some_or_continue!`,
`some_or_break!`. Where `?` already says it, use `?`.

```rust
use simple_impl::some_or_continue;

let mut ports = Vec::new();
for text in ["22", "http", "443"] {
    let port = some_or_continue!(text.parse::<u16>().ok());
    ports.push(port);
}
assert_eq!(ports, [22, 443]);
```

**Collection / conditional mutation (10)**: `push_if!`, `push_some!`,
`extend_if!`, `extend_some!`, `insert_if!`, `insert_some!`, `assign_if!`,
`assign_if_some!`, `push_unique!`, `extend_unique!`.

```rust
use simple_impl::{push_if, push_unique};

let mut parts = vec!["rm"];
push_if!(parts, true => "-v");
push_unique!(parts, "-v");
assert_eq!(parts, ["rm", "-v"]);
```

**Whole-method forms (3)**: `simple_method!`, `checked_method!`,
`checked_constructor!`. `simple_method!` is the catch-all: any method whose
body is one expression.

```rust
use simple_impl::{checked_constructor, simple_method};

#[derive(Debug, PartialEq)]
struct Capacity { bytes: usize }

impl Capacity {
    checked_constructor!(
        pub fn new(bytes: usize), &'static str;
        bytes > 0 => "zero";
        => Self { bytes }
    );
    simple_method!(pub fn kib(&self) -> usize => self.bytes / 1024);
}
assert_eq!(Capacity::new(4096).map(|c| c.kib()), Ok(4));
assert_eq!(Capacity::new(0), Err("zero"));
```

**Accessors / delegation (15)**: `getter!`, `getter_ref!`, `getter_mut!`,
`getter_option_ref!`, `getter_option_deref!`, `getter_slice!`,
`len_field!`, `is_empty_field!`, `bool_field!`, `contains_field!`,
`find_by!`, `delegate!`, `delegate_mut!`, `newtype_value!`, `newtype_ref!`.

```rust
use simple_impl::{getter_option_deref, getter_slice, len_field};

struct Policy { scope: Option<String>, roles: Vec<String> }

impl Policy {
    getter_option_deref!(pub fn scope(&self) -> Option<&str> => scope);
    getter_slice!(pub fn roles(&self) -> &[String] => roles);
    len_field!(pub fn role_count(&self) => roles);
}
let policy = Policy { scope: Some("ops".into()), roles: vec!["deployer".into()] };
assert_eq!(policy.scope(), Some("ops"));
assert_eq!(policy.roles(), ["deployer"]);
assert_eq!(policy.role_count(), 1);
```

**Setters / consuming builders (19)**: `setter!`, `setter_into!`,
`setter_some!`, `setter_some_into!`, `clear_collection!`, `take_field!`,
`enable_field!`, `disable_field!`, `toggle_field!`, `with_field!`,
`with_into!`, `with_some!`, `with_some_into!`, `with_push!`,
`with_push_into!`, `with_extend!`, `with_flag!`, `without_flag!`,
`with_unique_push!`. Broader than `builder_*`: mutable setters, nested
fields, state toggles, any configuration object.

```rust
use simple_impl::{with_field, with_some_into, with_unique_push};

#[derive(Default)]
struct Policy { requires_sudo: bool, iam_scope: Option<String>, path_roles: Vec<String> }

impl Policy {
    with_field!(pub fn with_requires_sudo(mut self, value: bool) -> Self => requires_sudo);
    with_some_into!(pub fn with_iam_scope(mut self, scope: String) -> Self => iam_scope);
    with_unique_push!(pub fn with_path_role(mut self, role: String) -> Self => path_roles);
}
let policy = Policy::default()
    .with_requires_sudo(true)
    .with_iam_scope("ops:deploy")
    .with_path_role("deployer".into())
    .with_path_role("deployer".into());
assert_eq!(policy.path_roles, ["deployer"]);
```

**Constructors / trait boilerplate (12)**: `newtype_constructor!`,
`impl_default_via_new!`, `impl_from_newtype!`, `impl_as_ref_newtype!`,
`impl_deref_newtype!`, `impl_deref_mut_newtype!`, `impl_display_via!`,
`impl_debug_redacted!`, `impl_std_error!`, `impl_partial_ord_via_ord!`,
`impl_from_variant!`, `forward_trait_method!`.

```rust
use simple_impl::{impl_debug_redacted, impl_from_newtype, newtype_constructor};

struct Token(String);

impl Token {
    newtype_constructor!(pub fn new(value: String) -> Self);
}
impl_from_newtype!(Token, String);
impl_debug_redacted!(Token, "Token([REDACTED])");

let token: Token = String::from("secret").into();
assert_eq!(format!("{token:?}"), "Token([REDACTED])");
assert_eq!(Token::new("x".into()).0, "x");
```

`all!`, `any!`, `ensure!` and `bail!` are common names. When another crate
in scope has a macro of the same name (`liaise_proc_macro` has an `ensure!`
that records a `syn::Error` and carries on), call this one by path:
`simple_impl::ensure!`.

## License

MIT OR Apache-2.0.

## The `ShellCommand` trait

`simple_impl::ShellCommand` is what `#[derive(SimpleShell)]` implements when
no `trait_path` is given: `build()` returns the whole command line, and
`to_command()` wraps it in `sh -c`.

Two whole-builder macros: `shell_command_builder!(GitBuilder, "git")` for a
program that takes a subcommand and free arguments, and
`define_shell_builder!` for a builder whose fields are constructor arguments
and whose command line is written in a block.
