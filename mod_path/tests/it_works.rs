#![feature(plugin)]
#![plugin(mod_path)]

mod_path! foo (concat!(env!("OUT_DIR"), "/hello.rs"));

#[test]
fn it_works() {
    assert_eq!(foo::ANSWER, 42);
}
