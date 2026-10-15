// expressions written as code and order written as keys build the same command
use simple_impl::{ShellCommand, SimpleImpl};

#[derive(SimpleImpl, Default)]
#[shell(cmd_expr = if self.sudo { "sudo ls" } else { "ls" })]
pub struct Ls {
    sudo: bool,
    #[shell(flag = "-a", order(after = long))]
    all: bool,
    #[shell(flag = "-l", order(first))]
    long: bool,
    #[shell(arg_expr = self.depth + 1)]
    #[builder(skip)]
    shown: u8,
    #[builder(default_expr = 2)]
    depth: u8,
    // code that starts with a number, and a comparison followed by more keys
    #[shell(arg_expr = 10 * self.depth, order = 100)]
    #[builder(skip)]
    scaled: u8,
    #[shell(arg_expr = self.depth < 5, order = 101)]
    #[builder(skip)]
    small: u8,
}

fn main() {
    let ls = Ls::new().sudo().all().long();
    // order = 100 and 101 are absolute keys: before the flag band -a sits in
    assert_eq!(ls.build(), "sudo ls -l 20 true -a 3");
}
