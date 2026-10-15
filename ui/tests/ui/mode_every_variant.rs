use simple_impl::SimpleShellMode;

#[derive(SimpleShellMode)]
pub enum Mode {
    Json,
    Raw(u8),
    #[shell(flag = 1)]
    Xml,
    Csv { x: u8 },
}

fn main() {}
