// Structs that must compile.
use simple_impl::{CompositeSubCommand, SimpleSubCommand};

#[derive(SimpleSubCommand, Clone, Debug, Default)]
#[subcommand(segment = "status")]
#[one_of()]
pub struct Status {
    #[arg(flag = "--short")]
    short: bool,
}

#[derive(SimpleSubCommand, Clone, Debug, Default)]
#[subcommand(segment = "type")]
pub struct Kind {
    #[arg(flag = "-v")]
    v: bool,
}

// a raw-identifier command crashed quote_kit 0.3.1; it now builds
#[derive(CompositeSubCommand, Clone, Debug, Default)]
#[subcommand(segment = "tools")]
pub struct Tools {
    #[composite(command)]
    r#type: Kind,
}

fn main() {}
