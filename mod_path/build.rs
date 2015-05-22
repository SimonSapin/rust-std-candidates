use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let dst = PathBuf::from(&env::var("OUT_DIR").unwrap());
    let mut f = File::create(&dst.join("hello.rs")).unwrap();
    f.write_all(b"
        pub const ANSWER: u32 = 40 + 2;
    ").unwrap();
}
