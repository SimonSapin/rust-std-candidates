These are candidates for addition to
the [Rust](http://rust-lang.org/) standard library.
Each crate is published separately on [crate.io](https://crates.io/)

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
#![feature(phase)]
#[phase(plugin)] extern crate matches;

fn is_slash(input: &str, pos: uint) -> bool {
    matches!(input.char_at(pos), '/' | '\\')
}
```

```rust
#![feature(phase)]
#[phase(plugin)] extern crate matches;
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
#![feature(phase)]
#[phase(plugin)] extern crate matches;
extern crate "zip-longest" as zip_longest;
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

```toml
[dependencies]
show = "0.1"
```

Example:

```rust
#![feature(phase)]
#[phase(plugin)] extern crate show;

use std::os::getenv;

fn defilibrate_flux_capacitor(gigowatts: float) {
    // Temporarily added during debugging:
    show!(gigowatts, getenv("EPOCH"));
    // Equivalent to:
    println!("{} {}", gigowatts, getenv("EPOCH"));
}
```
