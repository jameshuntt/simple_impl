use simple_impl::SimpleShell;

// an expression written as code: the compiler's error lands on `nope` itself;
// the same mistake as a string can only point at the derive
#[derive(SimpleShell)]
#[shell(cmd = "x")]
struct X {
    #[shell(arg_expr = self.nope + 1)]
    a: u8,
    #[shell(arg_expr = "self.gone + 1")]
    b: u8,
}

fn main() {}
