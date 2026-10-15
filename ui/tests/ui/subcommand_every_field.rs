use simple_impl::SimpleSubCommand;

#[derive(SimpleSubCommand)]
#[subcommand(segment = "add")]
#[xccute_policy(bad)]
pub struct Add {
    #[arg(flag = 1)]
    a: bool,
    b: u8,
    #[arg(flag = "-c")]
    #[builder(nope)]
    c: bool,
}

fn main() {}
