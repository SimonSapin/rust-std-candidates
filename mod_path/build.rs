#![feature(os, path, io)]

use std::os;
use std::old_io::File;

fn main() {
    let dst = Path::new(os::getenv("OUT_DIR").unwrap());
    let mut f = File::create(&dst.join("hello.rs")).unwrap();
    f.write_str("
        pub const SIX_BY_NINE: u32 = 0o42;
    ").unwrap();
}
