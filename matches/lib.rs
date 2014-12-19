#![feature(macro_rules)]

#[macro_export]
macro_rules! matches(
    ($expression: expr, $($pattern:pat)|+) => (
        matches!($expression, $($pattern)|+ if true)
    );
    ($expression: expr, $($pattern:pat)|+ if $guard: expr) => (
        match $expression {
            $($pattern)|+ => $guard,
            _ => false
        }
    );
);

#[test]
fn it_works() {
    let foo = Some("-12");
    assert!(matches!(foo, Some(bar) if
        matches!(bar.char_at(0), '+' | '-') &&
        matches!(bar.char_at(1), '0'...'9')
    ));
}
