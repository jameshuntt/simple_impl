//! The same struct derived by the previous release (0.2.3) and by this one; every
//! value must build the same command.

macro_rules! parity {
    ($test:ident: $(#[$a:meta])* struct S { $($f:tt)* } => [$($v:expr),+ $(,)?]) => {
        mod $test {
            #[allow(dead_code)]
            mod old {
                use simple_impl::ShellCommand;
                #[derive(old_derive::SimpleShell)]
                $(#[$a])* pub struct S { $($f)* }
                pub fn run() -> Vec<String> { vec![$({ let s: S = $v; s.build() }),+] }
            }
            #[allow(dead_code)]
            mod new {
                use simple_impl::ShellCommand;
                #[derive(simple_impl_derive::SimpleShell)]
                $(#[$a])* pub struct S { $($f)* }
                pub fn run() -> Vec<String> { vec![$({ let s: S = $v; s.build() }),+] }
            }
            #[test]
            fn same_commands() {
                let (old, new) = (old::run(), new::run());
                assert_eq!(old, new);
                println!("{new:?}");
            }
        }
    };
}

parity!(flags:
    #[shell(cmd = "ls")]
    struct S {
        #[shell(flag = "-l")] pub long: bool,
        #[shell(flag_off = "--no-color")] pub color: bool,
        #[shell(flag = "--on", flag_off = "--off")] pub toggle: bool,
        #[shell(count_flag = "v")] pub verbose: u8,
    } => [
        S { long: true, color: true, toggle: true, verbose: 0 },
        S { long: false, color: false, toggle: false, verbose: 3 },
    ]);

parity!(options:
    #[shell(cmd = "x")]
    struct S {
        #[shell(opt_kv = "--name")] pub name: Option<String>,
        #[shell(opt_prefix = "-O")] pub opt: Option<u32>,
        #[shell(opt_eq = "--mode")] pub mode: Option<String>,
        #[shell(kv = "-k")] pub k: String,
        #[shell(kv = "-m")] pub m: Option<u16>,
        #[shell(prefix = "-p")] pub p: String,
        #[shell(eq = "--e")] pub e: u8,
    } => [
        S { name: Some("a b".into()), opt: Some(2), mode: Some("fast".into()), k: "kk".into(), m: Some(9), p: "pp".into(), e: 1 },
        S { name: None, opt: None, mode: None, k: String::new(), m: None, p: String::new(), e: 0 },
    ]);

parity!(multi:
    #[shell(cmd = "x")]
    struct S {
        #[shell(multi_opt_kv = "-e")] pub envs: Vec<String>,
        #[shell(multi_opt_kv = "-f")] pub files: Option<Vec<String>>,
        #[shell(multi_opt_kv = "-j", join = ",")] pub joined: Vec<String>,
        #[shell(multi_opt_prefix = "-I")] pub incs: Vec<String>,
        #[shell(multi_arg_flag = "--files")] pub many: Vec<String>,
        #[shell(multi_arg_flag = "--j", join = ":")] pub mj: Option<Vec<String>>,
    } => [
        S { envs: vec!["A=1".into(), "B=2".into()], files: Some(vec!["f".into()]), joined: vec!["x".into(), "y".into()],
            incs: vec!["inc".into()], many: vec!["m1".into(), "m2".into()], mj: Some(vec!["p".into(), "q".into()]) },
        S { envs: vec![], files: None, joined: vec![], incs: vec![], many: vec![], mj: Some(vec![]) },
    ]);

parity!(positional:
    #[shell(cmd = "cp")]
    struct S {
        #[shell(positional)] pub src: String,
        #[shell(positional)] pub dst: Option<String>,
        #[shell(positional)] pub rest: Vec<String>,
        #[shell(positional, join = ",")] pub csv: Vec<u32>,
        #[shell(arg_join_opt = "port", sep = ":")] pub host: String,
        pub port: Option<u16>,
        #[shell(kv = "-H", fmt = "'{}'")] pub header: String,
    } => [
        S { src: "a".into(), dst: Some("b".into()), rest: vec!["c".into()], csv: vec![1, 2], host: "h".into(), port: Some(22), header: "v".into() },
        S { src: "a".into(), dst: None, rest: vec![], csv: vec![], host: "h".into(), port: None, header: String::new() },
    ]);

parity!(order:
    #[shell(cmd = "x", require_order)]
    struct S {
        #[shell(flag = "-a", order = 5)] pub a: bool,
        #[shell(flag = "-b", order = "first")] pub b: bool,
        #[shell(flag = "-c", order = "last")] pub c: bool,
        #[shell(flag = "-d", order = "before:A")] pub d: bool,
        #[shell(flag = "-e", order = "after:b")] pub e: bool,
        #[shell(positional, order = -3)] pub f: String,
    } => [
        S { a: true, b: true, c: true, d: true, e: true, f: "pos".into() },
    ]);

parity!(subcommand:
    #[shell(cmd = "git")]
    struct S {
        #[shell(subcommand)] pub sub: String,
        #[shell(flag = "-q")] pub quiet: bool,
    } => [
        S { sub: "status".into(), quiet: true },
    ]);
