//! The macros. `sc_*` build the argument vector of a shell command from a
//! struct's fields; `builder_*` write the setters of a builder.

/// The argument vector, started with the command name.
///
/// ```
/// use simple_impl::sc_cmd;
///
/// let parts = sc_cmd!("kill");
/// assert_eq!(parts, vec!["kill".to_string()]);
/// ```
#[macro_export]
macro_rules! sc_cmd {
    ($cmd:expr) => {
        vec![$cmd.to_string()]
    };
}

/// Push a flag for every `bool` field that is `true`.
///
/// ```
/// use simple_impl::{sc_cmd, sc_flags};
///
/// struct Rm { force: bool, verbose: bool }
/// let cmd = Rm { force: true, verbose: false };
///
/// let mut parts = sc_cmd!("rm");
/// sc_flags!(parts, cmd, force => "--force", verbose => "--verbose");
/// assert_eq!(parts, ["rm", "--force"]);
/// ```
#[macro_export]
macro_rules! sc_flags {
    ($parts:ident, $this:ident, $( $field:ident => $flag:expr ),* $(,)?) => {{
        $(
            if $this.$field {
                $parts.push($flag.to_string());
            }
        )*
    }};
}

/// [`sc_flags!`] with `.into()` instead of `.to_string()`, for a `parts`
/// whose element type is not `String`.
///
/// ```
/// use simple_impl::sc_flags_into;
///
/// struct Rm { force: bool }
/// let cmd = Rm { force: true };
///
/// let mut parts: Vec<std::ffi::OsString> = Vec::new();
/// sc_flags_into!(parts, cmd, force => "--force");
/// assert_eq!(parts, [std::ffi::OsString::from("--force")]);
/// ```
#[macro_export]
macro_rules! sc_flags_into {
    ($parts:ident, $this:ident, $( $field:ident => $flag:expr ),* $(,)?) => {{
        $(
            if $this.$field {
                $parts.push($flag.into());
            }
        )*
    }};
}

/// Push `flag` then the value for every `Option` field that is `Some`.
///
/// ```
/// use simple_impl::{sc_cmd, sc_opt_kv};
///
/// struct Convert { output: Option<String>, quality: Option<u8> }
/// let cmd = Convert { output: Some("out.png".into()), quality: None };
///
/// let mut parts = sc_cmd!("convert");
/// sc_opt_kv!(parts, cmd, output => "--output", quality => "--quality");
/// assert_eq!(parts, ["convert", "--output", "out.png"]);
/// ```
#[macro_export]
macro_rules! sc_opt_kv {
    ($parts:ident, $this:ident, $( $field:ident => $flag:expr ),* $(,)?) => {{
        $(
            if let Some(v) = &$this.$field {
                $parts.push($flag.to_string());
                $parts.push(v.to_string());
            }
        )*
    }};
}

/// Push positional arguments, each through `.into()`.
///
/// ```
/// use simple_impl::{sc_cmd, sc_args};
///
/// let source = String::from("a.txt");
/// let mut parts = sc_cmd!("cp");
/// sc_args!(parts, source.clone(), "b.txt");
/// assert_eq!(parts, ["cp", "a.txt", "b.txt"]);
/// ```
#[macro_export]
macro_rules! sc_args {
    ($parts:ident, $( $arg:expr ),* $(,)?) => {{
        $(
            $parts.push(($arg).into());
        )*
    }};
}

/// Push the mapped value of an `Option` when it is `Some`. The mapper gets
/// `&T` and returns a `String`.
///
/// ```
/// use simple_impl::{sc_cmd, sc_if_some};
///
/// let signal: Option<u8> = Some(9);
/// let mut parts = sc_cmd!("kill");
/// sc_if_some!(parts, signal, |sig| format!("-{sig}"));
/// assert_eq!(parts, ["kill", "-9"]);
/// ```
#[macro_export]
macro_rules! sc_if_some {
    ($parts:ident, $opt:expr, $map:expr $(,)?) => {{
        if let Some(v) = &($opt) {
            $parts.push(($map)(v));
        }
    }};
}

/// Push `v.to_string()` when an `Option` is `Some(v)`.
///
/// ```
/// use simple_impl::{sc_cmd, sc_if_some_display};
///
/// let pid: Option<u32> = Some(4242);
/// let none: Option<u32> = None;
/// let mut parts = sc_cmd!("kill");
/// sc_if_some_display!(parts, pid);
/// sc_if_some_display!(parts, none);
/// assert_eq!(parts, ["kill", "4242"]);
/// ```
#[macro_export]
macro_rules! sc_if_some_display {
    ($parts:ident, $opt:expr $(,)?) => {{
        if let Some(v) = &($opt) {
            $parts.push(v.to_string());
        }
    }};
}

/// Push `v.into()` when an `Option` is `Some(v)`.
///
/// ```
/// use simple_impl::sc_if_some_into;
///
/// let name: Option<&str> = Some("x");
/// let mut parts: Vec<String> = Vec::new();
/// sc_if_some_into!(parts, name);
/// assert_eq!(parts, ["x"]);
/// ```
#[macro_export]
macro_rules! sc_if_some_into {
    ($parts:ident, $opt:expr $(,)?) => {{
        if let Some(v) = &($opt) {
            $parts.push((*v).into());
        }
    }};
}

/// Push `v.clone()` when an `Option` is `Some(v)`.
///
/// ```
/// use simple_impl::sc_if_some_clone;
///
/// let name: Option<String> = Some("x".into());
/// let mut parts: Vec<String> = Vec::new();
/// sc_if_some_clone!(parts, name);
/// assert_eq!(parts, ["x"]);
/// ```
#[macro_export]
macro_rules! sc_if_some_clone {
    ($parts:ident, $opt:expr $(,)?) => {{
        if let Some(v) = &($opt) {
            $parts.push(v.clone());
        }
    }};
}

/// Push `format!("{prefix}{v}")` when an `Option` is `Some(v)`.
///
/// ```
/// use simple_impl::{sc_cmd, sc_if_some_prefix};
///
/// let signal: Option<&str> = Some("TERM");
/// let mut parts = sc_cmd!("kill");
/// sc_if_some_prefix!(parts, signal, "-");
/// assert_eq!(parts, ["kill", "-TERM"]);
/// ```
#[macro_export]
macro_rules! sc_if_some_prefix {
    ($parts:ident, $opt:expr, $prefix:expr $(,)?) => {{
        if let Some(v) = &($opt) {
            $parts.push(format!("{}{}", $prefix, v));
        }
    }};
}

/// Push the mapped value of every item of an iterator.
///
/// ```
/// use simple_impl::{sc_cmd, sc_each};
///
/// let pids = [1u32, 2, 3];
/// let mut parts = sc_cmd!("kill");
/// sc_each!(parts, &pids, |pid: &u32| format!("{pid}"));
/// assert_eq!(parts, ["kill", "1", "2", "3"]);
/// ```
#[macro_export]
macro_rules! sc_each {
    ($parts:ident, $iter:expr, $map:expr $(,)?) => {{
        for item in ($iter) {
            $parts.push(($map)(item));
        }
    }};
}

/// Push `item.to_string()` for every item of an iterator.
///
/// ```
/// use simple_impl::{sc_cmd, sc_each_display};
///
/// let pids = vec![10u32, 20];
/// let mut parts = sc_cmd!("kill");
/// sc_each_display!(parts, &pids);
/// assert_eq!(parts, ["kill", "10", "20"]);
/// ```
#[macro_export]
macro_rules! sc_each_display {
    ($parts:ident, $iter:expr $(,)?) => {{
        for item in ($iter) {
            $parts.push(item.to_string());
        }
    }};
}

/// Push `flag` then the clone of an `Option<String>` when it is `Some`.
///
/// ```
/// use simple_impl::{sc_cmd, sc_if_some_flag};
///
/// let sort_key = Some("name".to_string());
/// let mut parts = sc_cmd!("ls");
/// sc_if_some_flag!(parts, sort_key, "--sort-key");
/// assert_eq!(parts, ["ls", "--sort-key", "name"]);
/// ```
#[macro_export]
macro_rules! sc_if_some_flag {
    ($vec:expr, $opt:expr, $flag:expr) => {
        if let Some(val) = &$opt {
            $vec.push($flag.to_string());
            $vec.push(val.clone());
        }
    };
}

/// A `new()` that delegates to `Default`, optionally with a visibility.
///
/// ```
/// use simple_impl::builder_new_default;
///
/// #[derive(Default, Debug, PartialEq)]
/// struct Kill { pid: Option<u32> }
/// impl Kill {
///     builder_new_default!();
/// }
/// assert_eq!(Kill::new(), Kill::default());
/// ```
#[macro_export]
macro_rules! builder_new_default {
    () => {
        #[inline]
        pub fn new() -> Self {
            Self::default()
        }
    };
    ($vis:vis) => {
        #[inline]
        $vis fn new() -> Self {
            Self::default()
        }
    };
}

/// A setter for an `Option<T>` field, taking `T`.
///
/// ```
/// use simple_impl::builder_opt;
///
/// #[derive(Default)]
/// struct Kill { pid: Option<u32> }
/// impl Kill {
///     builder_opt!(pid => pid: u32);
/// }
/// assert_eq!(Kill::default().pid(7).pid, Some(7));
/// ```
#[macro_export]
macro_rules! builder_opt {
    ($name:ident => $field:ident : $ty:ty) => {
        #[inline]
        pub fn $name(mut self, value: $ty) -> Self {
            self.$field = Some(value);
            self
        }
    };
}

/// A setter for an `Option<T>` field, taking `impl Into<T>`.
///
/// ```
/// use simple_impl::builder_opt_into;
///
/// #[derive(Default)]
/// struct Kill { signal: Option<String> }
/// impl Kill {
///     builder_opt_into!(signal => signal: String);
/// }
/// assert_eq!(Kill::default().signal("TERM").signal.as_deref(), Some("TERM"));
/// ```
#[macro_export]
macro_rules! builder_opt_into {
    ($name:ident => $field:ident : $ty:ty) => {
        #[inline]
        pub fn $name(mut self, value: impl Into<$ty>) -> Self {
            self.$field = Some(value.into());
            self
        }
    };
}

/// A setter for a plain field, taking the value.
///
/// ```
/// use simple_impl::builder_set;
/// use std::path::PathBuf;
///
/// #[derive(Default)]
/// struct Run { cwd: PathBuf }
/// impl Run {
///     builder_set!(cwd => cwd: PathBuf);
/// }
/// assert_eq!(Run::default().cwd(PathBuf::from("/tmp")).cwd, PathBuf::from("/tmp"));
/// ```
#[macro_export]
macro_rules! builder_set {
    ($name:ident => $field:ident : $ty:ty) => {
        #[inline]
        pub fn $name(mut self, value: $ty) -> Self {
            self.$field = value;
            self
        }
    };
}

/// A setter for a `Vec<T>` field, taking `impl Into<Vec<T>>`.
///
/// ```
/// use simple_impl::builder_vec_into;
///
/// #[derive(Default)]
/// struct Kill { pids: Vec<u32> }
/// impl Kill {
///     builder_vec_into!(pids => pids: u32);
/// }
/// assert_eq!(Kill::default().pids(vec![1, 2]).pids, [1, 2]);
/// ```
#[macro_export]
macro_rules! builder_vec_into {
    ($name:ident => $field:ident : $elem:ty) => {
        #[inline]
        pub fn $name(mut self, list: impl Into<Vec<$elem>>) -> Self {
            self.$field = list.into();
            self
        }
    };
}

/// A setter for a `Vec<T>` field, taking any `IntoIterator<Item = T>`:
/// a `Vec`, an array, an iterator.
///
/// ```
/// use simple_impl::builder_vec_iter;
///
/// #[derive(Default)]
/// struct Kill { pids: Vec<u32> }
/// impl Kill {
///     builder_vec_iter!(pids => pids: u32);
/// }
/// assert_eq!(Kill::default().pids([1, 2, 3]).pids, [1, 2, 3]);
/// assert_eq!(Kill::default().pids((1..=2).map(|n| n * 10)).pids, [10, 20]);
/// ```
#[macro_export]
macro_rules! builder_vec_iter {
    ($name:ident => $field:ident : $elem:ty) => {
        #[inline]
        pub fn $name<I>(mut self, iter: I) -> Self
        where
            I: IntoIterator<Item = $elem>,
        {
            self.$field = iter.into_iter().collect();
            self
        }
    };
}

/// A method that pushes one element onto a `Vec<T>` field.
///
/// ```
/// use simple_impl::builder_push;
///
/// #[derive(Default)]
/// struct Kill { pids: Vec<u32> }
/// impl Kill {
///     builder_push!(push_pid => pids: u32);
/// }
/// assert_eq!(Kill::default().push_pid(1).push_pid(2).pids, [1, 2]);
/// ```
#[macro_export]
macro_rules! builder_push {
    ($name:ident => $field:ident : $elem:ty) => {
        #[inline]
        pub fn $name(mut self, value: $elem) -> Self {
            self.$field.push(value);
            self
        }
    };
}

/// A method that sets a `bool` field to `true`.
///
/// ```
/// use simple_impl::builder_flag;
///
/// #[derive(Default)]
/// struct Rm { force: bool }
/// impl Rm {
///     builder_flag!(force => force);
/// }
/// assert!(Rm::default().force().force);
/// ```
#[macro_export]
macro_rules! builder_flag {
    ($name:ident => $field:ident) => {
        #[inline]
        pub fn $name(mut self) -> Self {
            self.$field = true;
            self
        }
    };
}

/// A builder for a program that takes a subcommand and free arguments:
/// `git clone --mirror url dir`, `cargo new my_crate --lib`.
///
/// ```
/// use simple_impl::{shell_command_builder, ShellCommand};
///
/// shell_command_builder!(GitBuilder, "git");
///
/// let git = GitBuilder::new("clone").args(["--mirror", "https://example.invalid/x.git", "x.git"]);
/// assert_eq!(git.build(), "git clone --mirror https://example.invalid/x.git x.git");
/// assert_eq!(GitBuilder::new("status").arg("--short").build(), "git status --short");
/// ```
#[macro_export]
macro_rules! shell_command_builder {
    ($name:ident, $base_cmd:expr) => {
        #[derive(Debug, Clone, Default)]
        pub struct $name {
            pub subcommand: Option<String>,
            pub args: Vec<String>,
        }

        impl $name {
            pub fn new(subcommand: impl Into<String>) -> Self {
                Self {
                    subcommand: Some(subcommand.into()),
                    ..Default::default()
                }
            }

            pub fn arg(mut self, arg: impl Into<String>) -> Self {
                self.args.push(arg.into());
                self
            }

            pub fn args<I, S>(mut self, args: I) -> Self
            where
                I: IntoIterator<Item = S>,
                S: Into<String>,
            {
                self.args.extend(args.into_iter().map(Into::into));
                self
            }
        }

        impl $crate::ShellCommand for $name {
            fn build(&self) -> String {
                let mut parts = vec![$base_cmd.to_string()];
                if let Some(sub) = &self.subcommand {
                    parts.push(sub.clone());
                }
                parts.extend(self.args.iter().cloned());
                parts.join(" ")
            }
        }
    };
}

/// A builder whose fields are all constructor arguments and whose command
/// line is written out by hand in `build_parts`, a closure-shaped block that
/// names the command and the parts vector.
///
/// ```
/// use simple_impl::{define_shell_builder, ShellCommand};
///
/// define_shell_builder!(
///     Tar,
///     command = "tar",
///     fields = { archive: String, verbose: bool },
///     build_parts = |tar, parts| {
///         parts.push(if tar.verbose { "-cvf" } else { "-cf" }.to_string());
///         parts.push(tar.archive.clone());
///     }
/// );
///
/// assert_eq!(Tar::new("out.tar".into(), true).build(), "tar -cvf out.tar");
/// ```
#[macro_export]
macro_rules! define_shell_builder {
    (
        $builder_name:ident,
        command = $command_str:expr,
        fields = { $( $fname:ident : $ftype:ty ),* $(,)? },
        build_parts = |$this:ident, $parts:ident| $build:block
    ) => {
        #[derive(Debug, Default, Clone)]
        pub struct $builder_name {
            $( pub $fname: $ftype ),*
        }

        impl $builder_name {
            pub fn new($( $fname: $ftype ),*) -> Self {
                Self { $( $fname ),* }
            }
        }

        impl $crate::ShellCommand for $builder_name {
            fn build(&self) -> String {
                let $this = self;
                let mut $parts: Vec<String> = vec![$command_str.to_string()];
                $build
                $parts.join(" ")
            }
        }
    };
}
