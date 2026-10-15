use simple_impl::SimpleShell;

// order(before = name): a name that is no field is reported on the name
#[derive(SimpleShell)]
#[shell(cmd = "x")]
struct X {
    #[shell(flag = "-a", order(before = nope))]
    a: bool,
    #[shell(flag = "-b", order = "before:gone")]
    b: bool,
}

fn main() {}
