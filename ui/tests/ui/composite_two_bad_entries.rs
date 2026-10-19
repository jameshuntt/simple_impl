use simple_impl::CompositeSubCommand;

pub struct Add;
pub struct Remove;

#[derive(CompositeSubCommand)]
#[subcommand(segment = "remote")]
#[composite(surface = "add", ty = Add, init = "name")]
#[composite(surface = "remove", ty = Remove, init = "name")]
pub struct Remote;

fn main() {}
