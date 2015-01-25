#[macro_export]
macro_rules! return_if_ok {
    ($expression: expr) => {
        match $expression {
            ::std::result::Result::Ok(value) => {
                return ::std::result::Result::Ok(value)
            }
            ::std::result::Result::Err(error) => error
        }
    }
}


#[test]
fn it_works() {
    fn result_ok() -> Result<i32, ()> {
        Err(return_if_ok!(Ok(4)))
    }
    assert_eq!(result_ok(), Ok(4));

    fn result_err() -> Result<i32, ()> {
        Err(return_if_ok!(Err(())))
    }
    assert_eq!(result_err(), Err(()));
}
