// The four line shapes a system command library needs, each built and run:
// a list (`systemctl stop U...`), a line with no word of its own
// (`chown OWNER PATH`), a program alone (`hostname`), and the exit statuses a
// line names.
use std::ffi::{OsStr, OsString};

use simple_impl::{CompositeShell, SimpleSubCommand};
use xccute_contract::{CommandValidationError, CommandValidationResult, CompositeShellCommand, ExitStatusWord, ValidatedCommand};

fn plain(field: &str, word: &OsStr) -> CommandValidationResult {
    let text = word.to_string_lossy();
    if text.is_empty() || text.starts_with('-') || text.chars().any(char::is_whitespace) {
        return Err(CommandValidationError::structural(format!("{text:?} is not a plain {field}")).with_field(field));
    }
    Ok(())
}

#[derive(CompositeShell, Debug, Clone, Copy, Default)]
#[shell(program = "systemctl")]
#[composite(command = "stop", ty = Stop)]
#[composite(command = "show", method = "show", ty = Show, init = "unit")]
#[composite(command = "is-active", method = "is_active", ty = IsActive, init = "unit")]
#[composite(command = "daemon-reload", method = "daemon_reload", ty = DaemonReload)]
pub struct Systemctl;

#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "stop")]
pub struct Stop {
    #[arg(positional, each = plain, min = 1)]
    units: Vec<OsString>,
}

#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "show")]
pub struct Show {
    #[arg(flag = "--timestamp=unix")]
    unix: bool,
    #[arg(kv = "-p", each = "plain")]
    properties: Vec<OsString>,
    #[builder(init_required)]
    #[arg(positional, each = plain)]
    unit: OsString,
}

#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "is-active")]
#[exit_status(code = 0, word = "active")]
#[exit_status(code = 3, word = "inactive")]
pub struct IsActive {
    #[arg(flag = "--quiet")]
    quiet: bool,
    #[builder(init_required)]
    #[arg(positional, each = plain)]
    unit: OsString,
}

#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "daemon-reload")]
pub struct DaemonReload;

#[derive(CompositeShell, Debug, Clone, Copy, Default)]
#[shell(program = "chown")]
#[composite(command = "-R", method = "recursive", ty = ChownTree, init = "owner,path")]
#[composite(command = "", method = "to", ty = ChownTo, init = "owner,path")]
pub struct Chown;

#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "-R")]
pub struct ChownTree {
    #[builder(init_required)]
    #[arg(positional, each = plain)]
    owner: OsString,
    #[builder(init_required)]
    #[arg(positional)]
    path: OsString,
}

#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(no_segment)]
pub struct ChownTo {
    #[builder(init_required)]
    #[arg(positional, each = plain)]
    owner: OsString,
    #[builder(init_required)]
    #[arg(positional)]
    path: OsString,
}

#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(program = "hostname")]
pub struct Hostname;

#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(program = "kill", segment = "-TERM")]
pub struct Kill {
    #[arg(positional, min = 1)]
    pids: Vec<OsString>,
}

#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(program = "ssh")]
#[exit_status(code = 255, word = "the login or the connection failed")]
pub struct Login {
    #[arg(kv = "-o")]
    options: Vec<OsString>,
    #[arg(kv = "-i")]
    identity: Option<OsString>,
    #[builder(init_required)]
    #[arg(positional)]
    to: OsString,
}

fn words(command: &impl CompositeShellCommand) -> Vec<String> {
    let std = command.to_std_command();
    std::iter::once(std.get_program()).chain(std.get_args()).map(|w| w.to_string_lossy().into_owned()).collect()
}

fn main() {
    // a list: one word per value, a kv list `-p value` per value, the setter adding to what is there
    let stop = Systemctl::stop().units(["a.service"]).units(vec![String::from("b.service")]);
    assert_eq!(words(&stop), ["systemctl", "stop", "a.service", "b.service"]);
    stop.validate().unwrap();
    assert_eq!(Systemctl::stop().validate().unwrap_err().field(), Some("units"));
    assert!(Systemctl::stop().units(["--all"]).validate().unwrap_err().message().contains("not a plain units"));
    let show = Systemctl::show("u.service").unix().properties(["A", "B"]);
    assert_eq!(words(&show), ["systemctl", "show", "--timestamp=unix", "-p", "A", "-p", "B", "u.service"]);
    assert!(Systemctl::show("u.service").properties(["-H"]).validate().is_err());

    // statuses named on the line: 0 goes on, 3 does not; a status it does not name is none
    let check = Systemctl::is_active("u.service").quiet();
    assert_eq!(words(&check), ["systemctl", "is-active", "--quiet", "u.service"]);
    assert_eq!(check.ok_codes(), &[0]);
    assert_eq!(check.exit_statuses(), [ExitStatusWord::accepted(0, "active"), ExitStatusWord::rejected(3, "inactive")]);
    assert_eq!(check.exit_status_of(Some(3)).map(|s| s.word), Some("inactive"));
    assert_eq!(check.exit_status_of(Some(1)), None);
    assert!(Systemctl::is_active("a b").validate().is_err());

    // a unit struct: no arguments
    assert_eq!(words(&Systemctl::daemon_reload()), ["systemctl", "daemon-reload"]);

    // no word of its own: the owner leads, under the same root as -R
    assert_eq!(words(&Chown::to("a:b", "/x")), ["chown", "a:b", "/x"]);
    assert_eq!(words(&Chown::recursive("a:b", "/x")), ["chown", "-R", "a:b", "/x"]);
    assert!(Chown::to("-R", "/x").validate().unwrap_err().message().contains("not a plain owner"));

    // its own root: the program alone, or with its segment and its own setters
    assert_eq!(words(&Hostname), ["hostname"]);
    assert_eq!(words(&Kill::default().pids(["12", "13"])), ["kill", "-TERM", "12", "13"]);
    assert!(Kill::default().validate().is_err());
    let login = Login::new("root@h").options(["BatchMode=yes"]).identity("/k");
    assert_eq!(words(&login), ["ssh", "-o", "BatchMode=yes", "-i", "/k", "root@h"]);
    assert_eq!(login.exit_status_of(Some(255)).map(|s| s.accepted), Some(false));
    assert_eq!(login.ok_codes(), &[0]);
    // a leaf with its own program still goes under a root as any leaf does
    assert_eq!(words(&Login::new("root@h").rooted::<Login>()), ["ssh", "root@h"]);
}
