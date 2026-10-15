use simple_impl::CompositeShell;

#[derive(CompositeShell)]
#[shell(program = 1)]
#[composite(command = "x")]
#[composite(bogus)]
pub struct Root {
    #[composite(command, surface)]
    both: u8,
}

fn main() {}
