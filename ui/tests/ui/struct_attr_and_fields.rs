use simple_impl::SimpleShell;

#[derive(SimpleShell)]
#[shell(cmd = "ls", requre_order)]
struct Ls {
    #[shell(flag = "-l")]
    long: String,
}

fn main() {}
