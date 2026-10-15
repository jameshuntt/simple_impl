use simple_impl::SimpleShell;

// a syntax error inside code is reported where it is, not at the derive
#[derive(SimpleShell)]
#[shell(cmd = "x")]
struct X {
    #[shell(arg_expr = self.n +, order = 1)]
    a: u8,
    n: u8,
}

fn main() {}
