use simple_impl::SimpleShell;

#[derive(SimpleShell)]
#[shell(cmd = "x")]
struct X {
    #[shell(flag = "-a", order = "before:nope")]
    a: bool,
    #[shell(flag = "-b", order = "after:gone")]
    b: bool,
}

fn main() {}
