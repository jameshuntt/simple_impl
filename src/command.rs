//! The trait a `SimpleShell` command implements.

use std::process::Command;

/// A command that renders itself as one shell string.
///
/// `#[derive(SimpleShell)]` implements this for a struct when no `trait_path`
/// is given; `build()` is the whole command line, program first, joined by
/// spaces. Any trait with a `fn build(&self) -> String` can be named instead.
pub trait ShellCommand {
    /// The whole command line, program first, joined by single spaces.
    fn build(&self) -> String;

    /// `sh -c <build()>`.
    ///
    /// The string goes through a shell, so every value in it is shell syntax.
    /// For untrusted values use an argv command (`xccute_contract`) instead.
    fn to_command(&self) -> Command {
        let mut command = Command::new("sh");
        command.arg("-c").arg(self.build());
        command
    }
}
