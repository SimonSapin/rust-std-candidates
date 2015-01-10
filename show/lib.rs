#[macro_export]
macro_rules! show {
    ($expression: expr) => {
        println!("{:?}", $expression);
    };
    ($expression: expr, $($next: expr),+) => {{
        print!("{:?} ", $expression);
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
    show!(Some(42i32));
    show!(4u8, 'x', ("a", "b"));
    //panic!()  // Uncomment to see test output.
}
