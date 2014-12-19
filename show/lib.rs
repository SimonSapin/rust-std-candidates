#![feature(macro_rules)]

#[macro_export]
macro_rules! show {
    ($expression: expr) => {
        println!("{}", $expression);
    };
    ($expression: expr, $($next: expr),+) => {{
        print!("{} ", $expression);
        show!($($next),+)
    }};
    // Ignore a trailing comma:
    ($($expression: expr),+,) => {
        show!($($expression),+)
    };
}

#[test]
fn it_works() {
    show!("foo",);
    show!(Some(42i));
    show!(4u, 'x', ("a", "b"));
    //panic!()  // Uncomment to see test output.
}
