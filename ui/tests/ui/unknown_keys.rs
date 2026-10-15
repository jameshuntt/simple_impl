use simple_impl::SimpleShell;

#[derive(SimpleShell)]
#[shell(cmd = "x")]
struct X {
    #[shell(flagg = "-a", ordr = 1)]
    a: bool,
}

fn main() {}
