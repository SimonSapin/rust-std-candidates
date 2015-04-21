These are candidates for addition to
the [Rust](http://rust-lang.org/) standard library.
Each crate is published separately on [crates.io](https://crates.io/)

# The `matches!` macro

[RFC #163](https://github.com/rust-lang/rfcs/pull/163)

A macro to evaluate, as a boolean, whether an expression matches a pattern.

To use it, add to your `Cargo.toml`:

```toml
[dependencies]
matches = "0.1"
```

Example:

```rust
#[macro_use] extern crate matches;

fn is_slash(input: &str, pos: uint) -> bool {
    matches!(input.char_at(pos), '/' | '\\')
}
```

```rust
#[macro_use] extern crate matches;
extern crate serialize;
use serialize::json::Json;

fn object_has_key(json: &Json, key: &str) -> bool {
    matches!(json, &Json::Object(ref obj) if obj.contains_key(key))
}
```


# The `zip_longest` iterator adaptor

[PR #19283](https://github.com/rust-lang/rust/pull/19283)

The standard library has an [`iterator.zip(other_iterator)`](
http://doc.rust-lang.org/std/iter/trait.IteratorExt.html#tymethod.zip) method
that returns a new iterator that yields pairs,
and stops when one of the input iterators does.

`zip_longest` is similar,
but instead continues until both iterators are exhausted.
Instead of a pair of values `(A, B)`,
it yield an `EitherOrBoth` enum
that contains `Both(A, B)`, `Left(A)`, or `Right(B)`
depending on which of the input iterators if any is exhausted.

To use it, add to your `Cargo.toml`:

```toml
[dependencies]
zip_longest = "0.1"
```

Example:

```rust
#[macro_use] extern crate matches;
extern crate zip_longest;
use zip_longest::{ZipLongestIteratorExt, EitherOrBoth};

fn iter_eq<I, J, T>(i: I, j: J) -> bool
where I: Iterator<T>, J: Iterator<T>, T: Eq {
    i.zip_longest(j).all(|x| matches!(x, EitherOrBoth::Both(a, b) if a == b))
}
```


# The `show!` debugging macro

[Issue #12015](https://github.com/rust-lang/rust/issues/12015)

The minimal code (with only the standard library) for writing something to stdout
is currently `println!("{}", foo)`.
The `"{}", ` part does not provide any information other than "do the default thing"
which, you know, could be implicitly the default.
It gets slitghly tedious to type when it is added and removed a lot,
for doing “print-style” debugging.

The `show!` macro is a shortcut for calling `println!` with any any number of expressions,
that are printed space-separated with the default `{}` / `Show` formatting.

**Note:** Although debugging is the primary motivation for `show!`,
there is nothing wrong with using it in “normal” code if it does what you need.

To use it, add to your `Cargo.toml`:

```toml
[dependencies]
show = "0.1"
```

Example:

```rust
#[macro_use] extern crate show;

use std::os::getenv;

fn defibrillate_flux_capacitor(gigowatts: float) {
    // Temporarily added during debugging:
    show!(gigowatts, getenv("EPOCH"));
    // Equivalent to:
    println!("{} {}", gigowatts, getenv("EPOCH"));

    // ... do complicated stuff here.
}
```


# The `TextWriter` trait

Depreacted. Use [`std::fmt::Write`](http://doc.rust-lang.org/nightly/std/fmt/trait.Write.html) instead.

For historical interest, see the [previous README entry](
https://github.com/SimonSapin/rust-std-candidates/tree/0a0a9ccb3b3#the-textwriter-trait)
and [previous code](
https://github.com/SimonSapin/rust-std-candidates/blob/0a0a9ccb3b/text_writer/lib.rs)


# The `return_if_ok!` macro

[Discuss topic #1416](http://discuss.rust-lang.org/t/a-macro-that-is-to-result-or-else-what-try-is-to-result-and-then/1416).

The `return_if_ok` macro takes a `Result`,
then makes the function return early for `Ok(_)` values
or unwraps `Err(_)` values:

```rust
macro_rules! return_if_ok {
    ($expr:expr) => (match $expr {
        Ok(val) => return Ok(val),
        Err(err) => err
    })
}
```

Compare with the `try!` macro which takes a `Result`,
then makes the funciton return early for `Err(_)` values
or unwraps `Ok(_)` values:

```rust
macro_rules! try {
    ($expr:expr) => (match $expr {
        Ok(val) => val,
        Err(err) => return Err(FromError::from_error(err))
    })
}
```

If we ignore [the `FromError` conversion](http://doc.rust-lang.org/std/error/#the-fromerror-trait),
`return_if_ok!` and `try!` (which could be named `return_if_err!`)
are [dual][https://en.wikipedia.org/wiki/Duality_%28mathematics%29]
in the same way that [`Result::or_else`](http://doc.rust-lang.org/std/result/enum.Result.html#method.or_else)
and  [`Result::and_then`](http://doc.rust-lang.org/std/result/enum.Result.html#method.and_then) are dual,
and that [`||` and `&&` are dual](https://en.wikipedia.org/wiki/De_Morgan%27s_laws).

To use it, add to your `Cargo.toml`:

```toml
[dependencies]
return_if_ok = "0.1"
```

Example:

I’ve been using `Result` heavily
in [Servo’s CSS parsing rewrite](https://github.com/servo/servo/pull/4689).
Grammar production rules map (roughly) to functions that return `Result`,
concatenation maps to `try!` (or `Result::and_then`),
and alternation maps to `Result::or_else` but would look nicer with `return_if_ok!` instead:

```rust
#[macro_use] extern crate return_if_ok;

/// <'width'> = <length> | <percentage> | "auto"
fn parse_width(input: &mut Parser) -> Result<LengthOrPercentageOrAuto, ()> {
    return_if_ok!(parse_length(input).map(LengthOrPercentageOrAuto::Length));
    return_if_ok!(parse_percentage(input).map(LengthOrPercentageOrAuto::Percentage));
    parse_keyword(input, "auto").map(|()| LengthOrPercentageOrAuto::Auto)
}

/// <'border-spacing'> = <length> <length>?
/// The second length defaults to the first
fn parse_border_spacing(input: &mut Parser) -> Result<(Length, Length), ()> {
    let first = try!(parse_length(input));
    let second = parse_length(input).unwrap_or(first);
    Ok((first, second))
}
```
