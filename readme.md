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

# `# craaaazy advancement:`
```rust
use simple_impl_derive::{SimpleImpl};
use xccute::ShellCommand;

#[derive(SimpleImpl, Default)]
#[shell(cmd = "top", trait_path = "ShellCommand")]
pub struct TopBuilder {
    #[shell(flag = "-b")]
    pub batch_mode: bool,

    #[shell(opt_kv = "-d")]
    pub delay: Option<f32>,

    #[shell(opt_kv = "-n")]
    pub iterations: Option<u32>,

    #[builder(into)]
    #[shell(opt_kv = "-u")]
    pub user: Option<String>,

    #[shell(opt_kv = "-p")]
    pub pid: Option<u32>,
}
```
### `# produces`
```rust
    impl TopBuilder {
        #[inline]
        pub fn new() -> Self {
            ::core::default::Default::default()
        }
        #[inline]
        pub fn batch_mode(mut self) -> Self {
            self.batch_mode = true;
            self
        }
        #[inline]
        pub fn delay(mut self, value: f32) -> Self {
            self.delay = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub fn iterations(mut self, value: u32) -> Self {
            self.iterations = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub fn user(mut self, value: impl ::core::convert::Into<String>) -> Self {
            self.user = ::core::option::Option::Some(value.into());
            self
        }
        #[inline]
        pub fn pid(mut self, value: u32) -> Self {
            self.pid = ::core::option::Option::Some(value);
            self
        }
    }
    impl ShellCommand for TopBuilder {
        fn build(&self) -> ::std::string::String {
            let mut parts: ::std::vec::Vec<::std::string::String> = ::std::vec![
                "top".to_string()
            ];
            if self.batch_mode {
                parts.push("-b".to_string());
            }
            if let ::core::option::Option::Some(v) = &self.delay {
                parts.push("-d".to_string());
                parts.push(v.to_string());
            }
            if let ::core::option::Option::Some(v) = &self.iterations {
                parts.push("-n".to_string());
                parts.push(v.to_string());
            }
            if let ::core::option::Option::Some(v) = &self.user {
                parts.push("-u".to_string());
                parts.push(v.to_string());
            }
            if let ::core::option::Option::Some(v) = &self.pid {
                parts.push("-p".to_string());
                parts.push(v.to_string());
            }
            parts.join(" ")
        }
    }
```
### `# when compares to the previous version`
```rust
use crate::command::ShellCommand;

#[derive(Default)]
pub struct TopBuilder {
    pub batch_mode: bool,
    pub delay: Option<f32>,
    pub iterations: Option<u32>,
    pub user: Option<String>,
    pub pid: Option<u32>,
}

impl TopBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn batch_mode(mut self) -> Self {
        self.batch_mode = true;
        self
    }

    pub fn delay(mut self, seconds: f32) -> Self {
        self.delay = Some(seconds);
        self
    }

    pub fn iterations(mut self, count: u32) -> Self {
        self.iterations = Some(count);
        self
    }

    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }

    pub fn pid(mut self, pid: u32) -> Self {
        self.pid = Some(pid);
        self
    }
}

impl ShellCommand for TopBuilder {
    fn build(&self) -> String {
        let mut parts = vec!["top".to_string()];

        if self.batch_mode {
            parts.push("-b".into());
        }

        if let Some(delay) = self.delay {
            parts.push("-d".into());
            parts.push(delay.to_string());
        }

        if let Some(iters) = self.iterations {
            parts.push("-n".into());
            parts.push(iters.to_string());
        }

        if let Some(user) = &self.user {
            parts.push("-u".into());
            parts.push(user.clone());
        }

        if let Some(pid) = self.pid {
            parts.push("-p".into());
            parts.push(pid.to_string());
        }

        parts.join(" ")
    }
}

// let top_cmd = TopBuilder::new()
//     .batch_mode()
//     .delay(1.5)
//     .iterations(5)
//     .user("james")
//     .build();
// // => "top -b -d 1.5 -n 5 -u james"
```

### `# its identical`
