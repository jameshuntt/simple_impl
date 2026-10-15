use simple_impl::SimpleImpl;

#[derive(SimpleImpl)]
#[shell(cmd = "x")]
struct X {
    #[builder(push)]
    a: String,
    #[shell(flag = "-b")]
    b: u8,
    #[shell(flag = "-c", order = "before:nobody")]
    c: bool,
    #[shell(flag = "-d", bogus)]
    d: bool,
    #[shell(flag = "-e", order = "after:d")]
    e: bool,
}

fn main() {}
