use std::convert::From;


#[macro_export]
macro_rules! try {
    ($expression: expr) => {
        match $crate::Triable::try($expression) {
            $crate::TriableResult::Expression(value) => value,
            $crate::TriableResult::EarlyReturn(value) => return value,
        }
    };
}


pub enum TriableResult<Expr, Return> {
    Expression(Expr),
    EarlyReturn(Return),
}


pub trait Triable<Expr, Return> {
    fn try(self) -> TriableResult<Expr, Return>;
}


impl<T1, T2, Err1, Err2> Triable<T1, Result<T2, Err2>> for Result<T1, Err1>
where Err2: From<Err1> {
    fn try(self) -> TriableResult<T1, Result<T2, Err2>> {
        match self {
            Ok(value) => TriableResult::Expression(value),
            Err(error) => TriableResult::EarlyReturn(Err(From::from(error)))
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


impl<T1, T2> Triable<T1, Result<T2, ()>> for Option<T1> {
    fn try(self) -> TriableResult<T1, Result<T2, ()>> {
        match self {
            Some(value) => TriableResult::Expression(value),
            None => TriableResult::EarlyReturn(Err(()))
        }
    }
}


impl<T1, T2> Triable<T1, Option<T2>> for Result<T1, ()> {
    fn try(self) -> TriableResult<T1, Option<T2>> {
        match self {
            Ok(value) => TriableResult::Expression(value),
            Err(()) => TriableResult::EarlyReturn(None)
        }
    }
}


impl Triable<(), bool> for bool {
    fn try(self) -> TriableResult<(), bool> {
        if self {
            TriableResult::Expression(())
        } else {
            TriableResult::EarlyReturn(false)
        }
    }
}

impl<T> Triable<T, bool> for Result<T, ()> {
    fn try(self) -> TriableResult<T, bool> {
        match self {
            Ok(value) => TriableResult::Expression(value),
            Err(()) => TriableResult::EarlyReturn(false)
        }
    }
}

impl<T> Triable<T, bool> for Option<T> {
    fn try(self) -> TriableResult<T, bool> {
        match self {
            Some(value) => TriableResult::Expression(value),
            None => TriableResult::EarlyReturn(false)
        }
    }
}

impl<T> Triable<(), Result<T, ()>> for bool {
    fn try(self) -> TriableResult<(), Result<T, ()>> {
        if self {
            TriableResult::Expression(())
        } else {
            TriableResult::EarlyReturn(Err(()))
        }
    }
}


impl<T> Triable<(), Option<T>> for bool {
    fn try(self) -> TriableResult<(), Option<T>> {
        if self {
            TriableResult::Expression(())
        } else {
            TriableResult::EarlyReturn(None)
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

#[test]
fn option_to_result() {
    fn ok() -> Result<i32, ()> {
        Ok(try!(Some(4)))
    }
    assert_eq!(ok(), Ok(4));

    fn err() -> Result<i32, ()> {
        Ok(try!(None))
    }
    assert_eq!(err(), Err(()));
}

#[test]
fn result_to_option() {
    fn some() -> Option<i32> {
        Some(try!(Ok(5)))
    }
    assert_eq!(some(), Some(5));

    fn none() -> Option<i32> {
        Some(try!(Err(())))
    }
    assert_eq!(none(), None);
}

#[test]
fn bool() {
    fn true_() -> bool {
        try!(true);
        true
    }
    assert_eq!(true_(), true);

    fn false_() -> bool {
        try!(false);
        true
    }
    assert_eq!(false_(), false);
}

#[test]
fn option_to_bool() {
    fn true_() -> bool {
        try!(Some(5));
        true
    }
    assert_eq!(true_(), true);

    fn false_() -> bool {
        try!(None);
        true
    }
    assert_eq!(false_(), false);
}

#[test]
fn result_to_bool() {
    fn true_() -> bool {
        try!(Ok(5));
        true
    }
    assert_eq!(true_(), true);

    fn false_() -> bool {
        try!(Err(()));
        true
    }
    assert_eq!(false_(), false);
}

#[test]
fn bool_to_result() {
    fn ok() -> Result<(), ()> {
        Ok(try!(true))
    }
    assert_eq!(ok(), Ok(()));

    fn err() -> Result<(), ()> {
        Ok(try!(false))
    }
    assert_eq!(err(), Err(()));
}

#[test]
fn bool_to_option() {
    fn some() -> Option<()> {
        Some(try!(true))
    }
    assert_eq!(some(), Some(()));

    fn none() -> Option<()> {
        Some(try!(false))
    }
    assert_eq!(none(), None);
}
