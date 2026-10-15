use simple_impl::SimpleImpl;

#[derive(SimpleImpl)]
#[shell(cmd = "x")]
struct X {
    #[builder(init_required, default_expr = "1")]
    a: u8,
    #[builder(push)]
    b: String,
    #[builder(init_only, flag)]
    c: bool,
}

fn main() {}
