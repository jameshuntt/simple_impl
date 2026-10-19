// A surface whose leaf is a surface, built and run: the shapes custodian had to
// flatten into one leaf with an action word (`job start|close`, `count report`,
// `mcp serve`), and a surface three down. Each leaf keeps its setters, its exit
// statuses and its help line at every depth.
#![allow(clippy::duplicated_attributes)]
use std::ffi::OsString;

use simple_impl::{CompositeShell, CompositeSubCommand, SimpleSubCommand};
use xccute_contract::{CommandHelp, CompositeShellCommand, ValidatedCommand};

#[derive(CompositeShell, Debug, Clone, Copy, Default)]
#[shell(program = "custodian")]
#[composite(surface = "job", ty = Job)]
#[composite(surface = "count", ty = Count)]
#[composite(surface = "mcp", ty = Mcp)]
pub struct Custodian;

#[derive(CompositeSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "job")]
#[composite(command = "start", ty = JobStart, init = "name")]
#[composite(command = "close", ty = JobClose)]
pub struct Job;

/// Starts a job.
#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "start")]
#[exit_status(code = 0, word = "done")]
#[exit_status(code = 1, word = "findings", accepted)]
#[exit_status(code = 2, word = "refused")]
pub struct JobStart {
    #[builder(init_required, into)]
    #[arg(positional)]
    name: OsString,
    #[builder(into)]
    #[arg(kv = "--kind")]
    kind: Option<OsString>,
    #[arg(flag = "--again")]
    again: bool,
}

/// Closes the running job, or the one named.
#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "close")]
#[exit_status(code = 0, word = "done")]
#[exit_status(code = 2, word = "refused")]
pub struct JobClose {
    #[builder(into)]
    #[arg(positional)]
    id: Option<OsString>,
}

#[derive(CompositeSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "count")]
#[composite(command = "report", ty = CountReport, init = "a,b")]
pub struct Count;

#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "report")]
pub struct CountReport {
    #[builder(init_required, into)]
    #[arg(positional)]
    a: OsString,
    #[builder(init_required, into)]
    #[arg(positional)]
    b: OsString,
}

#[derive(CompositeSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "mcp")]
pub struct Mcp {
    #[composite(command)]
    serve: McpServe,
    #[composite(surface)]
    tools: Tools,
}

#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "serve")]
pub struct McpServe {
    #[builder(into)]
    #[arg(kv = "--root")]
    root: Option<OsString>,
}

#[derive(CompositeSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "tools")]
#[composite(surface = "schema", ty = Schema)]
pub struct Tools;

#[derive(CompositeSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "schema")]
#[composite(command = "show", ty = SchemaShow, init = "tool")]
pub struct Schema;

#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "show")]
#[exit_status(code = 0, word = "shown")]
pub struct SchemaShow {
    #[builder(init_required, into)]
    #[arg(positional)]
    tool: OsString,
    #[arg(flag = "--json")]
    json: bool,
}

fn main() {
    let start = Custodian::job().start("triage").kind("crate").again();
    assert_eq!(start.build_display(), "custodian job start triage --kind crate --again");
    assert_eq!(start.ok_codes(), &[0, 1]);
    assert_eq!(start.exit_status_of(Some(2)).map(|s| s.word), Some("refused"));
    start.validate().unwrap();

    assert_eq!(Custodian::job().close().id("7").build_display(), "custodian job close 7");
    assert_eq!(Custodian::count().report("a", "b").build_display(), "custodian count report a b");
    assert_eq!(Custodian::mcp().serve().root("/w").build_display(), "custodian mcp serve --root /w");

    let show = Custodian::mcp().tools().schema().show("run").json();
    assert_eq!(show.build_display(), "custodian mcp tools schema show run --json");
    assert_eq!(show.exit_status_of(Some(0)).map(|s| s.word), Some("shown"));

    let lines: Vec<String> = Custodian::help_lines().into_iter().map(|line| line.words).collect();
    assert_eq!(
        lines,
        [
            "custodian job start NAME [--kind KIND] [--again]",
            "custodian job close [ID]",
            "custodian count report A B",
            "custodian mcp serve [--root ROOT]",
            "custodian mcp tools schema show TOOL [--json]",
        ]
    );
    assert_eq!(Custodian::help_lines()[0].about, "Starts a job.");
}
