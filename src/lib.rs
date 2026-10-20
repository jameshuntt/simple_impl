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
//! The toolkit adds 107 more `macro_rules!` helpers at the root, for the
//! small bodies and one-line methods a derive doesn't cover: pure checks
//! ([`all!`], [`exactly_one!`], [`implies!`]), guards that return early
//! ([`guard!`], [`ensure!`] and the `ensure_*` family), early flow
//! ([`require_some!`], [`some_or_continue!`]), conditional mutation
//! ([`push_if!`], [`push_unique!`]), [`simple_method!`] for any
//! one-expression method, accessors ([`getter!`], [`getter_option_deref!`]),
//! setters and consuming builders ([`setter!`], [`with_field!`],
//! [`with_unique_push!`]), and constructors and trait boilerplate
//! ([`newtype_constructor!`], [`impl_from_variant!`],
//! [`forward_trait_method!`]). Seven come from a reading of the kit:
//! [`ensure_valid!`], [`require_ok_into!`], [`require_some_into!`],
//! [`ok_or_else_return!`], [`return_if_err!`], and values chosen by a
//! condition or a pattern, [`if_else!`] and [`table!`].
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
