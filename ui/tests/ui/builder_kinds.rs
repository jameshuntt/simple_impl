use simple_impl::SimpleImpl;

#[derive(SimpleImpl)]
#[shell(cmd = "x")]
struct X {
    #[builder(opt)]
    a: u32,
    #[builder(vec_into)]
    b: Option<u8>,
    #[builder(method = "set name")]
    c: u8,
    #[builder(push)]
    #[shell(flag = "-d")]
    d: String,
}

fn main() {}
