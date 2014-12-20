use std::fmt::{mod, FormatWriter};
use std::str;

#[deriving(Copy, Show)]
pub struct Error;


pub type Result = ::std::result::Result<(), Error>;


pub trait TextWriter {
    fn write_str(&mut self, s: &str) -> Result;
    fn write_char(&mut self, c: char) -> Result;

    fn write_fmt(&mut self, args: &fmt::Arguments) -> Result {
        struct Adaptor<'a, W: 'a> {
            text_writer: &'a mut W,
        }
        impl<'a, W> FormatWriter for Adaptor<'a, W> where W: TextWriter {
            fn write(&mut self, bytes: &[u8]) -> fmt::Result {
                match str::from_utf8(bytes) {
                    Some(s) => self.text_writer.write_str(s).map_err(|_| fmt::WriteError),
                    None => Err(fmt::WriteError),
                }
            }
        }
        Adaptor { text_writer: self }.write_fmt(args).map_err(|_| Error)
    }
}


impl TextWriter for String {
    #[inline]
    fn write_str(&mut self, s: &str) -> Result {
        self.push_str(s);
        Ok(())
    }

    #[inline]
    fn write_char(&mut self, c: char) -> Result {
        self.push(c);
        Ok(())
    }
}


impl<'a> TextWriter for fmt::Formatter<'a> {
    #[inline]
    fn write_str(&mut self, s: &str) -> Result {
        self.write(s.as_bytes()).map_err(|_| Error)
    }

    #[inline]
    fn write_char(&mut self, c: char) -> Result {
        (write!(self, "{}", c)).map_err(|_| Error)
    }
}


#[cfg(test)]
fn write_to<W: TextWriter>(dest: &mut W) -> Result {
    try!(dest.write_str("fo"));
    try!(dest.write_char('ô'));
    try!(write!(dest, "{}", 42u));
    Ok(())
}

#[test]
fn test_string() {

    let mut s = String::new();
    write_to(&mut s).unwrap();
    assert_eq!(s.as_slice(), "foô42");
}

#[test]
fn test_show() {
    struct Foo;
    impl fmt::Show for Foo {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write_to(formatter).unwrap();
            Ok(())
        }
    }
    assert_eq!(Foo.to_string().as_slice(), "foô42");
}

#[test]
fn test_ucs4() {
    struct Ucs4 {
        chars: Vec<char>,
    }

    impl TextWriter for Ucs4 {
        #[inline]
        fn write_str(&mut self, s: &str) -> Result {
            self.chars.extend(s.chars());
            Ok(())
        }

        #[inline]
        fn write_char(&mut self, c: char) -> Result {
            self.chars.push(c);
            Ok(())
        }
    }

    let mut s = Ucs4 { chars: vec![] };
    write_to(&mut s).unwrap();
    assert_eq!(s.chars.as_slice(), ['f', 'o', 'ô', '4', '2'].as_slice());
}
