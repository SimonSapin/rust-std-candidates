#[macro_export]
macro_rules! try {
    ($expression: expr) => {
        match ::triable::Triable::try($expression) {
            ::triable::TriableResult::Expression(value) => value,
            ::triable::TriableResult::EarlyReturn(value) => return value,
        }
    };
}


// Work around Servo’s Rust version not having $crate yet.
mod triable {
    pub use super::{Triable, TriableResult};
}


pub enum TriableResult<Expr, Return> {
    Expression(Expr),
    EarlyReturn(Return),
}


pub trait Triable<Expr, Return> {
    fn try(self) -> TriableResult<Expr, Return>;
}


impl<T1, T2, Err1, Err2> Triable<T1, Result<T2, Err2>> for Result<T1, Err1>
where Err2: ::std::error::FromError<Err1> {
    fn try(self) -> TriableResult<T1, Result<T2, Err2>> {
        match self {
            Ok(value) => TriableResult::Expression(value),
            Err(error) => TriableResult::EarlyReturn(Err(::std::error::FromError::from_error(error)))
        }
    }
}


impl<T1, T2> Triable<T1, Option<T2>> for Option<T1> {
    fn try(self) -> TriableResult<T1, Option<T2>> {
        match self {
            Some(value) => TriableResult::Expression(value),
            None => TriableResult::EarlyReturn(None)
        }
    }
}


#[test]
fn result() {
    fn ok() -> Result<i32, ()> {
        Ok(try!(Ok(4)))
    }
    assert_eq!(ok(), Ok(4));

    fn err() -> Result<i32, ()> {
        Ok(try!(Err(())))
    }
    assert_eq!(err(), Err(()));
}

#[test]
fn option() {
    fn some() -> Option<i32> {
        Some(try!(Some(5)))
    }
    assert_eq!(some(), Some(5));

    fn none() -> Option<i32> {
        Some(try!(None))
    }
    assert_eq!(none(), None);
}

