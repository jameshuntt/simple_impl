//! Macros that make a command-line builder short.
//!
//! Two families, both `macro_rules!`, both `#[macro_export]`ed at the root:
//!
//! * `sc_*` build the argument vector of a shell command from a struct's
//!   fields: [`sc_cmd!`] starts it, [`sc_flags!`] pushes the flags whose
//!   `bool` is set, [`sc_opt_kv!`] pushes `--flag value` for `Some` fields,
//!   [`sc_args!`] pushes positionals, and the `sc_if_some_*` and `sc_each*`
//!   forms cover the single-value and iterator cases.
//! * `builder_*` write the setters of a builder inside an `impl` block:
//!   [`builder_new_default!`], [`builder_opt!`] / [`builder_opt_into!`] for
//!   `Option` fields, [`builder_set!`] for plain ones, [`builder_vec_into!`]
//!   / [`builder_vec_iter!`] / [`builder_push!`] for `Vec` fields, and
//!   [`builder_flag!`] for a `bool`.
//!
//! ```
//! use simple_impl::{builder_flag, builder_new_default, builder_opt_into, builder_vec_iter};
//! use simple_impl::{sc_cmd, sc_each_display, sc_flags, sc_if_some_prefix};
//!
//! #[derive(Default)]
//! struct Kill {
//!     signal: Option<String>,
//!     pids: Vec<u32>,
//!     verbose: bool,
//! }
//!
//! impl Kill {
//!     builder_new_default!();
//!     builder_opt_into!(signal => signal: String);
//!     builder_vec_iter!(pids => pids: u32);
//!     builder_flag!(verbose => verbose);
//!
//!     fn args(&self) -> Vec<String> {
//!         let mut parts = sc_cmd!("kill");
//!         sc_flags!(parts, self, verbose => "--verbose");
//!         sc_if_some_prefix!(parts, self.signal, "-");
//!         sc_each_display!(parts, &self.pids);
//!         parts
//!     }
//! }
//!
//! let cmd = Kill::new().signal("TERM").pids([12, 34]).verbose();
//! assert_eq!(cmd.args(), ["kill", "--verbose", "-TERM", "12", "34"]);
//! ```
//!
//! [`ShellCommand`] is the trait `#[derive(SimpleShell)]` (from
//! `simple_impl_derive`, re-exported here under the `derive` feature)
//! implements by default: `build()` gives the whole command line.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

#[macro_use]
pub mod macros;

mod command;
pub use command::ShellCommand;

#[cfg(feature = "derive")]
pub use simple_impl_derive::*;

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme_doctests {}
