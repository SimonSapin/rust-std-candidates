use std::fmt;

#[deriving(Copy, Show)]
pub struct Error;


pub type Result = ::std::result::Result<(), Error>;


pub trait TextWriter {
    fn write_str(&mut self, s: &str) -> Result;
    fn write_char(&mut self, c: char) -> Result;
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
fn write_to<W: TextWriter>(writer: &mut W) -> Result {
    try!(writer.write_str("fo"));
    writer.write_char('ô')
}

#[test]
fn test_string() {

    let mut s = String::new();
    write_to(&mut s).unwrap();
    assert_eq!(s.as_slice(), "foô");
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
    assert_eq!(Foo.to_string().as_slice(), "foô");
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
    assert_eq!(s.chars.as_slice(), ['f', 'o', 'ô'].as_slice());
}
