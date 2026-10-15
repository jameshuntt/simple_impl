// Structs that compiled before the collector release must still compile.
use simple_impl::{ShellCommand, SimpleImpl};

#[derive(SimpleImpl, Default)]
#[shell(cmd = "x")]
pub struct Names {
    // a decomposed spelling (e + U+0301): the compiler normalizes it to one letter
    #[builder(method = "e\u{301}")]
    #[shell(kv = "-a")]
    a: String,
    #[builder(method = "r#type")]
    #[shell(kv = "-b")]
    b: String,
    // no setter: kind and method are never used
    #[builder(skip, opt)]
    c: u32,
    #[builder(skip, vec_into)]
    d: u32,
    #[builder(skip, method = "not an ident")]
    #[shell(flag = "-e")]
    e: bool,
    #[builder(init_only, method = "two words")]
    f: u32,
    // a Unicode 16 letter: the compiler decides, not a table in a dependency
    #[builder(method = "x\u{1E5D0}")]
    g: u8,
}

fn main() {
    let n = Names::new().é("1").r#type("2");
    assert_eq!(n.build(), "x -a 1 -b 2");
    let _ = Names::new().x𞗐(3);
}
