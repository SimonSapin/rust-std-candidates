/// Check if an expression matches a refutable pattern.
///
/// Syntax: `matches!(` *expression* `,` *pattern* `)`
///
/// Return a boolean, true if the expression matches the pattern, false otherwise.
///
/// # Examples
///
/// ```
/// use matches::matches;
///
/// pub enum Foo<T> {
///     A,
///     B(T),
/// }
///
/// impl<T> Foo<T> {
///     pub fn is_a(&self) -> bool {
///         matches!(*self, Foo::A)
///     }
///
///     pub fn is_b(&self) -> bool {
///         matches!(*self, Foo::B(_))
///     }
/// }
///
/// # fn main() { }
/// ```
#[macro_export]
macro_rules! matches {
    ($expression:expr, $($pattern:tt)+) => {
        match $expression {
            $($pattern)+ => true,
            _ => false
        }
    }
}

/// Assert that an expression matches a refutable pattern. Optionally, you
/// can add a follow-up block, which runs with the matched pattern if the
/// assertion succeeded.
///
/// Syntax: `assert_matches!(` *expression* `,` *pattern (* `=>` *block )?* `)`
///
/// Panic with a message that shows the expression if it does not match the
/// pattern.
///
/// # Examples
///
/// ```
/// use matches::assert_matches;
///
/// let data = [1, 2, 3];
/// assert_matches!(data.get(1), Some(_));
///
/// assert_matches!(data.get(1), Some(x) => {
///     assert_eq!(x, &2);
/// })
/// ```
#[macro_export]
macro_rules! assert_matches {
    ($expression:expr, $pattern:pat $(if $guard:expr)? $(=> $block:expr)?) => {
        match $expression {
            $pattern $(if $guard)? => { $($block)? },
            ref e => panic!("assertion failed: `value` does not match `pattern`
   value: {:?},
 pattern: {}", e, stringify!($pattern $(if $guard)?)),
        }
    };
}

/// Assert that an expression matches a refutable pattern using debug assertions.
///
/// Syntax: `debug_assert_matches!(` *expression* `,` *pattern* `)`
///
/// If debug assertions are enabled, panic with a message that shows the
/// expression if it does not match the pattern.
///
/// When debug assertions are not enabled, this macro does nothing.
///
/// # Examples
///
/// ```
/// use matches::debug_assert_matches;
///
/// fn main() {
///     let data = [1, 2, 3];
///     debug_assert_matches!(data.get(1), Some(_));
/// }
/// ```
#[macro_export]
macro_rules! debug_assert_matches {
    ($expression:expr, $pattern:pat $(if $guard:expr)? $(=> $block:expr)?) => {
        if cfg!(debug_assertions) {
            $crate::assert_matches!($expression, $pattern $(if $guard)? $(=> $block)?)
        }
    }
}

#[test]
fn matches_works() {
    let foo = Some("-12");
    assert!(matches!(foo, Some(bar) if
        matches!(bar.as_bytes()[0], b'+' | b'-') &&
        matches!(bar.as_bytes()[1], b'0'..=b'9')
    ));
}

#[test]
fn assert_matches_works() {
    let foo = Some("-12");
    assert_matches!(foo, Some(bar) if
        matches!(bar.as_bytes()[0], b'+' | b'-') &&
        matches!(bar.as_bytes()[1], b'0'..=b'9')
    );
}

#[test]
#[should_panic(expected = "assertion failed: `value` does not match `pattern`")]
fn assert_matches_panics() {
    let foo = Some("-AB");
    assert_matches!(foo, Some(bar) if
        matches!(bar.as_bytes()[0], b'+' | b'-') &&
        matches!(bar.as_bytes()[1], b'0'..=b'9')
    );
}

#[test]
#[should_panic(expected = "assertion failed: `value` does not match `pattern`")]
fn assert_matches_panics_on_mismatch() {
    let foo: Result<&str, &str> = Ok("-AB");
    assert_matches!(foo, Err(_));
}

#[test]
fn assert_matches_block() {
    let foo = Some(10);

    assert_matches!(foo, Some(x) => {
        assert_eq!(x, 10);
    })
}

#[test]
#[should_panic(expected = "assertion failed: x != 5")]
fn assert_matches_block_fails() {
    let foo = Some(10);

    assert_matches!(foo, Some(x) => {
        // Use a panic instead of assert because the specific error message
        // of std::assert_eq!() might change in future releases
        if x != 5 {
            panic!("assertion failed: x != 5");
        }
    })
}
