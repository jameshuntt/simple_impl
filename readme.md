# `# make your impls simple`
###
### heres a real use case
    the original source of the example file can be found here
    
    this was before pushing the version that introduced simple_impl

`https://github.com/jameshuntt/xccute/blob/main/src/proc/kill.rs`

```rust
use simple_impl::{
    builder_new_default,
    builder_opt,
    builder_opt_into,
    builder_vec_into,
    sc_cmd,
    sc_each_display,
    sc_if_some,
    sc_if_some_display,
    sc_if_some_prefix,
};

use crate::command::ShellCommand;

#[derive(Default)]
pub struct KillBuilder {
    pub signal: Option<String>,
    pub pid: Option<u32>,
    pub pids: Vec<u32>,
}

impl KillBuilder {
    // pub fn new() -> Self {
    //     Self::default()
    // }
    builder_new_default!();

    // pub fn signal(mut self, sig: impl Into<String>) -> Self {
    //     self.signal = Some(sig.into());
    //     self
    // }
    builder_opt_into!(signal => signal : String);

    // pub fn pid(mut self, pid: u32) -> Self {
    //     self.pid = Some(pid);
    //     self
    // }
    builder_opt!(pid => pid : u32);

    // pub fn pids(mut self, list: impl Into<Vec<u32>>) -> Self {
    //     self.pids = list.into();
    //     self
    // }
    builder_vec_into!(pids => pids : u32);
}

impl ShellCommand for KillBuilder {
    fn build(&self) -> String {
        // let mut parts = vec!["kill".to_string()];
        let mut parts = sc_cmd!("kill");

        // if let Some(sig) = &self.signal {
        //     parts.push(format!("-{}", sig));
        // }
        sc_if_some_prefix!(parts, self.signal, "-");

        // if let Some(pid) = self.pid {
        //     parts.push(pid.to_string());
        // }
        sc_if_some_display!(parts, self.pid);

        // for pid in &self.pids {
        //     parts.push(pid.to_string());
        // }
        sc_each_display!(parts, &self.pids);

        parts.join(" ")
    }
}

// let kill_cmd = KillBuilder::new()
//     .signal("9")
//     .pid(1234)
//     .build();
// // => "kill -9 1234"
```