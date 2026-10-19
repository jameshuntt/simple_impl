use simple_impl::SimpleSubCommand;
use std::ffi::OsString;

#[derive(SimpleSubCommand)]
#[subcommand(segment = "s", no_segment)]
#[exit_status(code = 3, word = "a")]
#[exit_status(code = 3, word = "b")]
#[exit_status(code = 3000000000, word = "big")]
pub struct S {
    #[builder(init_required)]
    #[arg(positional)]
    list: Vec<OsString>,
    #[arg(positional, min = 2)]
    one: OsString,
    #[arg(flag = "-f", each = check)]
    f: bool,
}

fn main() {}
