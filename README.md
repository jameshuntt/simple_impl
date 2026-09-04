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
