use std::fmt;
use std::io::Write;
use std::mem::transmute;
use std::ops;
use std::ptr;
use std::str;

/// Like `String`, but with a fixed capacity and a generic backing bytes storage.
///
/// Use e.g. `StringWrapper<[u8; 4]>` to have a string without heap memory allocation.
#[derive(Clone)]
pub struct StringWrapper<T> where T: AsRef<[u8]> + AsMut<[u8]> {
    len: usize,
    buffer: T,
}

impl<T> StringWrapper<T> where T: AsRef<[u8]> + AsMut<[u8]> {
    pub fn new(buffer: T) -> Self {
        StringWrapper {
            len: 0,
            buffer: buffer,
        }
    }

    /// Users must ensure that:
    ///
    /// * The buffer length is at least `len`
    /// * The first `len` bytes of `buffer` are well-formed UTF-8.
    pub unsafe fn from_raw_parts(buffer: T, len: usize) -> Self {
        StringWrapper {
            len: len,
            buffer: buffer,
        }
    }

    pub fn into_buffer(self) -> T {
        self.buffer
    }

    pub fn buffer(&self) -> &[u8] {
        self.buffer.as_ref()
    }

    /// Users must ensure that the string up to `self.len()` remains well-formed UTF-8.
    pub unsafe fn buffer_mut(&mut self) -> &mut [u8] {
        self.buffer.as_mut()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// Users must ensure that the string remains well-formed UTF-8.
    pub unsafe fn set_len(&mut self, new_len: usize) {
        self.len = new_len
    }

    pub fn truncate(&mut self, new_len: usize) {
        if new_len < self.len {
            assert!(starts_well_formed_utf8_sequence(self.buffer.as_ref()[new_len]));
            self.len = new_len;
        }
    }

    pub fn capacity(&self) -> usize {
        self.buffer.as_ref().len()
    }

    pub fn extra_capacity(&self) -> usize {
        self.capacity() - self.len
    }

    /// Return the slice of unused bytes after the string
    pub fn extra_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.buffer.as_mut()[self.len..]
    }

    pub fn push(&mut self, c: char) -> Result<(), ()> {
        let new_len = self.len + c.len_utf8();
        if new_len <= self.capacity() {
            // FIXME: use `c.encode_utf8` once it’s stable.
            write!(self.extra_bytes_mut(), "{}", c).unwrap();
            self.len = new_len;
            Ok(())
        } else {
            Err(())
        }
    }

    /// Panics if `s.len() <= self.extra_capacity()`
    pub fn push_str(&mut self, s: &str) {
        copy_memory(s.as_bytes(), self.extra_bytes_mut());
        self.len += s.len();
    }

    /// Append as much as possible of the given string
    /// (within 3 bytes of `self.extra_capacity()`)
    /// Return `Ok(())` if the capacity was sufficient,
    /// or `Err(number_of_bytes_written)`.
    pub fn push_partial_str(&mut self, s: &str) -> Result<(), usize> {
        let mut i = self.extra_capacity();
        let (s, result) = if i < s.len() {
            // As long as `self` is well-formed,
            // this loop does as most 3 iterations and `i` does not underflow.
            while !starts_well_formed_utf8_sequence(s.as_bytes()[i]) {
                i -= 1
            }
            (&s[..i], Err(i))
        } else {
            (s, Ok(()))
        };
        self.push_str(s);
        result
    }
}
fn starts_well_formed_utf8_sequence(byte: u8) -> bool {
    // ASCII byte or "leading" byte
    byte < 128 || byte >= 192
}

// FIXME: Use `std::slice::bytes::copy_memory` instead when it’s stable.
/// Copies data from `src` to `dst`
///
/// Panics if the length of `dst` is less than the length of `src`.
fn copy_memory(src: &[u8], dst: &mut [u8]) {
    let len_src = src.len();
    assert!(dst.len() >= len_src);
    // `dst` is unaliasable, so we know statically it doesn't overlap
    // with `src`.
    unsafe {
        ptr::copy_nonoverlapping(src.as_ptr(),
                                 dst.as_mut_ptr(),
                                 len_src);
    }
}


impl<T> ops::Deref for StringWrapper<T> where T: AsRef<[u8]> + AsMut<[u8]> {
    type Target = str;

    fn deref(&self) -> &str {
        unsafe {
            str::from_utf8_unchecked(&self.buffer.as_ref()[..self.len])
        }
    }
}

impl<T> ops::DerefMut for StringWrapper<T> where T: AsRef<[u8]> + AsMut<[u8]> {
    fn deref_mut(&mut self) -> &mut str {
        unsafe {
            transmute::<&mut [u8], &mut str>(&mut self.buffer.as_mut()[..self.len])
        }
    }
}

impl<T> fmt::Display for StringWrapper<T> where T: AsRef<[u8]> + AsMut<[u8]> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl<T> fmt::Debug for StringWrapper<T> where T: AsRef<[u8]> + AsMut<[u8]> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

#[test]
fn it_works() {
    let mut s = StringWrapper::new([0; 10]);
    assert_eq!(&*s, "");
    assert_eq!(s.len(), 0);
    assert_eq!(s.capacity(), 10);
    assert_eq!(s.extra_capacity(), 10);

    assert_eq!(&*s, "");
    assert_eq!(s.len(), 0);
    assert_eq!(s.capacity(), 10);
    assert_eq!(s.extra_capacity(), 10);

    s.push_str("a");
    assert_eq!(&*s, "a");
    assert_eq!(s.len(), 1);
    assert_eq!(s.capacity(), 10);
    assert_eq!(s.extra_capacity(), 9);

    assert_eq!(s.push('é'), Ok(()));
    assert_eq!(&*s, "aé");
    assert_eq!(s.len(), 3);
    assert_eq!(s.capacity(), 10);
    assert_eq!(s.extra_capacity(), 7);

    assert_eq!(s.push_partial_str("~~~"), Ok(()));
    assert_eq!(&*s, "aé~~~");
    assert_eq!(s.len(), 6);
    assert_eq!(s.capacity(), 10);
    assert_eq!(s.extra_capacity(), 4);

    assert_eq!(s.push_partial_str("hello"), Err(4));
    assert_eq!(&*s, "aé~~~hell");
    assert_eq!(s.len(), 10);
    assert_eq!(s.capacity(), 10);
    assert_eq!(s.extra_capacity(), 0);

    s.truncate(6);
    assert_eq!(&*s, "aé~~~");
    assert_eq!(s.len(), 6);
    assert_eq!(s.capacity(), 10);
    assert_eq!(s.extra_capacity(), 4);

    s.truncate(8);
    assert_eq!(&*s, "aé~~~");
    assert_eq!(s.len(), 6);
    assert_eq!(s.capacity(), 10);
    assert_eq!(s.extra_capacity(), 4);

    assert_eq!(s.push_partial_str("_🌠"), Err(1));
    assert_eq!(&*s, "aé~~~_");
    assert_eq!(s.len(), 7);
    assert_eq!(s.capacity(), 10);
    assert_eq!(s.extra_capacity(), 3);

    assert_eq!(s.push('🌠'), Err(()));
    assert_eq!(&*s, "aé~~~_");
    assert_eq!(s.len(), 7);
    assert_eq!(s.capacity(), 10);
    assert_eq!(s.extra_capacity(), 3);


    let buffer: [u8; 10] = s.clone().into_buffer();
    assert_eq!(&buffer, b"a\xC3\xA9~~~_ell");
    assert_eq!(format!("{}", s), "aé~~~_");
    assert_eq!(format!("{:?}", s), r#""a\u{e9}~~~_""#);

    assert_eq!(s.push_partial_str("ô!?"), Err(3));
    assert_eq!(&*s, "aé~~~_ô!");
    assert_eq!(s.len(), 10);
    assert_eq!(s.capacity(), 10);
    assert_eq!(s.extra_capacity(), 0);
}

