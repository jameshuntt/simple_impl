use simple_impl::SimpleSubCommand;

// only the shape is wrong: the variant's own fields are not reported as unknown
#[derive(SimpleSubCommand)]
#[subcommand(segment = "add")]
#[one_of("a", "b")]
pub enum Add {
    V {
        #[arg(flag = "-a")]
        a: bool,
        #[arg(kv = "-b")]
        #[requires("a")]
        b: Option<String>,
    },
}

fn main() {}
