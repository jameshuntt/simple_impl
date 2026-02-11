/// Create the initial `parts` vector with the command name.
///
/// **Replaces:**
/// ```rust
/// let mut parts = vec!["kill".to_string()];
/// // or
/// let mut parts = vec!["file".to_string()];
/// ```
///
/// **Usage:**
/// ```rust
/// let mut parts = sc_cmd!("kill");
/// ```
#[macro_export]
macro_rules! sc_cmd {
    ($cmd:expr) => {
        vec![$cmd.to_string()]
    };
}

/// Push boolean flags onto `parts` when the corresponding struct fields are `true`.
///
/// **Replaces:**
/// ```rust
/// if self.verbose { parts.push("--verbose".to_string()); }
/// if self.force   { parts.push("--force".to_string()); }
/// ```
///
/// **Usage:**
/// ```rust
/// sc_flags!(parts, self,
///     verbose => "--verbose",
///     force   => "--force",
/// );
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

/// Push boolean flags onto `parts` when the corresponding struct fields are `true`.
///
/// **Replaces:**
/// ```rust
/// if self.verbose { parts.push("--verbose".into()); }
/// if self.force   { parts.push("--force".into()); }
/// ```
///
/// **Usage:**
/// ```rust
/// sc_flags_into!(parts, self,
///     verbose => "--verbose",
///     force   => "--force",
/// );
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

/// Push `--flag value` pairs for `Option<T>` fields.
///
/// For each mapping `$field => $flag`, if `self.$field` is `Some(v)`,
/// this pushes `$flag` and then `v.to_string()`.
///
/// **Replaces:**
/// ```rust
/// if let Some(out) = &self.output {
///     parts.push("--output".to_string());
///     parts.push(out.to_string());
/// }
/// ```
///
/// **Usage:**
/// ```rust
/// sc_opt_kv!(parts, self,
///     output => "--output",
///     format => "--format",
/// );
/// ```
#[macro_export]
macro_rules! sc_opt_kv {
    // Option<T>: if Some(v) => push flag then v.to_string()
    ($parts:ident, $this:ident, $( $field:ident => $flag:expr ),* $(,)?) => {{
        $(
            if let Some(v) = &$this.$field {
                $parts.push($flag.to_string());
                $parts.push(v.to_string());
            }
        )*
    }};
}

/// Push positional args onto `parts`.
///
/// Each expression is pushed using `.into()` (commonly `String`).
///
/// **Replaces:**
/// ```rust
/// parts.push(self.filename.clone());
/// parts.push(target.to_string());
/// ```
///
/// **Usage:**
/// ```rust
/// sc_args!(parts,
///     self.filename.clone(),
///     "extra".to_string(),
/// );
/// ```
#[macro_export]
macro_rules! sc_args {
    ($parts:ident, $( $arg:expr ),* $(,)?) => {{
        $(
            $parts.push(($arg).into());
        )*
    }};
}

/// Push a mapped value when an `Option` is `Some`.
///
/// The mapper receives `&T` and must return a `String`.
///
/// **Replaces:**
/// ```rust
/// if let Some(sig) = &self.signal {
///     parts.push(format!("-{}", sig));
/// }
/// ```
///
/// **Usage:**
/// ```rust
/// sc_if_some!(parts, self.signal, |sig| format!("-{}", sig));
/// ```
#[macro_export]
macro_rules! sc_if_some {
    // Push mapped value when Option is Some(..)
    // The mapper takes a reference to the inner value: |v| -> String
    ($parts:ident, $opt:expr, $map:expr $(,)?) => {{
        if let Some(v) = &($opt) {
            $parts.push(($map)(v));
        }
    }};
}

/// Push `v.to_string()` when an `Option` is `Some(v)`.
///
/// **Replaces:**
/// ```rust
/// if let Some(pid) = self.pid {
///     parts.push(pid.to_string());
/// }
/// ```
///
/// **Usage:**
/// ```rust
/// sc_if_some_display!(parts, self.pid);
/// ```
#[macro_export]
macro_rules! sc_if_some_display {
    // Common case: Some(v) => push v.to_string()
    ($parts:ident, $opt:expr $(,)?) => {{
        if let Some(v) = &($opt) {
            $parts.push(v.to_string());
        }
    }};
}

/// Push `v.into()` when an `Option` is `Some(v)`.
///
/// **Replaces:**
/// ```rust
/// if let Some(pid) = self.pid {
///     parts.push(pid.into());
/// }
/// ```
///
/// **Usage:**
/// ```rust
/// sc_if_some_into!(parts, self.pid);
/// ```
#[macro_export]
macro_rules! sc_if_some_into {
    // Common case: Some(v) => push v.into()
    ($parts:ident, $opt:expr $(,)?) => {{
        if let Some(v) = &($opt) {
            $parts.push(v.into());
        }
    }};
}

/// Push `v.clone()` when an `Option` is `Some(v)`.
///
/// **Replaces:**
/// ```rust
/// if let Some(pid) = self.pid {
///     parts.push(pid.clone());
/// }
/// ```
///
/// **Usage:**
/// ```rust
/// sc_if_some_clone!(parts, self.pid);
/// ```
#[macro_export]
macro_rules! sc_if_some_clone {
    // Common case: Some(v) => push v.into()
    ($parts:ident, $opt:expr $(,)?) => {{
        if let Some(v) = &($opt) {
            $parts.push(v.clone());
        }
    }};
}

/// Push `format!("{prefix}{v}")` when an `Option` is `Some(v)`.
///
/// **Replaces:**
/// ```rust
/// if let Some(sig) = &self.signal {
///     parts.push(format!("-{}", sig));
/// }
/// ```
///
/// **Usage:**
/// ```rust
/// sc_if_some_prefix!(parts, self.signal, "-");
/// ```
#[macro_export]
macro_rules! sc_if_some_prefix {
    // Common case: Some(v) => push format!("{prefix}{v}")
    ($parts:ident, $opt:expr, $prefix:expr $(,)?) => {{
        if let Some(v) = &($opt) {
            $parts.push(format!("{}{}", $prefix, v));
        }
    }};
}

/// Push mapped values for each item in an iterator.
///
/// The mapper receives the iterator item and must return a `String`.
///
/// **Replaces:**
/// ```rust
/// for pid in &self.pids {
///     parts.push(pid.to_string());
/// }
/// ```
///
/// **Usage:**
/// ```rust
/// sc_each!(parts, &self.pids, |pid| pid.to_string());
/// ```
#[macro_export]
macro_rules! sc_each {
    // Push mapped value for each item in an iterator
    // mapper takes the iterator item: |item| -> String
    ($parts:ident, $iter:expr, $map:expr $(,)?) => {{
        for item in ($iter) {
            $parts.push(($map)(item));
        }
    }};
}

/// Push `item.to_string()` for each item in an iterator.
///
/// **Replaces:**
/// ```rust
/// for pid in &self.pids {
///     parts.push(pid.to_string());
/// }
/// ```
///
/// **Usage:**
/// ```rust
/// sc_each_display!(parts, &self.pids);
/// ```
#[macro_export]
macro_rules! sc_each_display {
    // Common case: push item.to_string()
    ($parts:ident, $iter:expr $(,)?) => {{
        for item in ($iter) {
            $parts.push(item.to_string());
        }
    }};
}

/// Generate a `new()` constructor that delegates to `Default`.
///
/// **Replaces:**
/// ```rust
/// pub fn new() -> Self {
///     Self::default()
/// }
/// ```
///
/// **Usage:**
/// ```rust
/// impl MyBuilder {
///     builder_new_default!();
/// }
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

/// Generate a builder setter for an `Option<T>` field (no `Into`).
///
/// **Replaces:**
/// ```rust
/// pub fn pid(mut self, pid: u32) -> Self {
///     self.pid = Some(pid);
///     self
/// }
/// ```
///
/// **Usage:**
/// ```rust
/// builder_opt!(pid => pid: u32);
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

/// Generate a builder setter for an `Option<T>` field using `Into<T>`.
///
/// **Replaces:**
/// ```rust
/// pub fn signal(mut self, sig: impl Into<String>) -> Self {
///     self.signal = Some(sig.into());
///     self
/// }
/// ```
///
/// **Usage:**
/// ```rust
/// builder_opt_into!(signal => signal: String);
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

/// Generate a builder setter for a non-`Option` field.
///
/// **Replaces:**
/// ```rust
/// pub fn cwd(mut self, cwd: PathBuf) -> Self {
///     self.cwd = cwd;
///     self
/// }
/// ```
///
/// **Usage:**
/// ```rust
/// builder_set!(cwd => cwd: std::path::PathBuf);
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

/// Generate a builder setter for a `Vec<T>` field from something convertible into `Vec<T>`.
///
/// **Replaces:**
/// ```rust
/// pub fn pids(mut self, list: impl Into<Vec<u32>>) -> Self {
///     self.pids = list.into();
///     self
/// }
/// ```
///
/// **Usage:**
/// ```rust
/// builder_vec_into!(pids => pids: u32);
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

/// Generate a builder setter for a `Vec<T>` field from any `IntoIterator<Item = T>`.
///
/// This accepts `Vec<T>`, arrays, iterators, etc.
///
/// **Replaces:**
/// ```rust
/// pub fn pids<I>(mut self, iter: I) -> Self
/// where
///     I: IntoIterator<Item = u32>,
/// {
///     self.pids = iter.into_iter().collect();
///     self
/// }
/// ```
///
/// **Usage:**
/// ```rust
/// builder_vec_iter!(pids => pids: u32);
/// // then you can do: .pids([1,2,3]) or .pids(vec![1,2,3])
/// ```
#[macro_export]
macro_rules! builder_vec_iter {
    // More flexible than Into<Vec<T>>: accepts Vec, arrays, iterators, etc.
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

/// Generate a builder method that pushes a single element into a `Vec<T>` field.
///
/// **Replaces:**
/// ```rust
/// pub fn push_pid(mut self, pid: u32) -> Self {
///     self.pids.push(pid);
///     self
/// }
/// ```
///
/// **Usage:**
/// ```rust
/// builder_push!(push_pid => pids: u32);
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

/// Generate a builder method that flips a boolean field to `true`.
///
/// **Replaces:**
/// ```rust
/// pub fn verbose(mut self) -> Self {
///     self.verbose = true;
///     self
/// }
/// ```
///
/// **Usage:**
/// ```rust
/// builder_flag!(verbose => verbose);
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

/// Appends a flag and its value if the given `Option<String>` is `Some`.
///
/// This macro is useful for CLI builders where a flag like `--sort-key`
/// is followed by a user-specified string. It avoids writing the common
/// `if let Some(val)` boilerplate.
///
/// ## Parameters
/// - `$vec`: the mutable `Vec<String>` to append to
/// - `$opt`: the `Option<String>` field to check
/// - `$flag`: the flag to push if `$opt` is `Some`
///
/// ## Example
/// ```rust
/// let mut parts = vec![];
/// let sort_key = Some("name".to_string());
///
/// sc_if_some_flag!(parts, sort_key, "--sort-key");
///
/// assert_eq!(parts, vec!["--sort-key", "name"]);
/// ```
///
/// ## Expands To
/// ```rust
/// if let Some(value) = &sort_key {
///     parts.push("--sort-key".to_string());
///     parts.push(value.clone());
/// }
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

