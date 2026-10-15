use simple_impl::SimpleShell;

#[derive(SimpleShell)]
#[shell(cmd = "ls")]
struct Ls {
    #[shell(flag = "-l")]
    long: String,
    #[shell(count_flag = "v")]
    verbose: String,
    #[shell(opt_kv = "--color")]
    color: String,
    #[shell(positional, sep = ",")]
    path: String,
}

fn main() {}
