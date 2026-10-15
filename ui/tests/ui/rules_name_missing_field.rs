use simple_impl::SimpleSubCommand;

#[derive(SimpleSubCommand)]
#[subcommand(segment = "add")]
#[one_of("a", "k")]
pub struct Add {
    #[arg(kv = "-k")]
    k: String,
    #[arg(flag = "-r")]
    #[requires("a")]
    r: bool,
}

fn main() {}
