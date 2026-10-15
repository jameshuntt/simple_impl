use simple_impl::CompositeSubCommand;

pub struct Add;
pub struct Remove;

#[derive(CompositeSubCommand)]
#[subcommand(segment = "remote")]
#[composite(surface = "add", ty = Add)]
#[composite(surface = "remove", ty = Remove)]
pub struct Remote;

fn main() {}
