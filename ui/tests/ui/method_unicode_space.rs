use simple_impl::SimpleBuilder;

#[derive(Default, SimpleBuilder)]
pub struct S {
    #[builder(method = "foo\u{a0}")]
    a: u8,
    #[builder(bogus)]
    b: u8,
}

fn main() {}
