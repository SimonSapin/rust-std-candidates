#![feature(convert)]

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let dst = PathBuf::from(&env::var("OUT_DIR").unwrap());
    let mut f = File::create(&dst.join("hello.rs")).unwrap();
    f.write_all(b"
        pub const SIX_BY_NINE: u32 = 0o42;
    ").unwrap();
}
