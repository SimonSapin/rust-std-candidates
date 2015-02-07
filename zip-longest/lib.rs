#![feature(core)]

use std::cmp;
use std::iter::RandomAccessIterator;


pub trait ZipLongestIteratorExt: Iterator + Sized {
    /// Creates an iterator which iterates over both this and the specified
    /// iterators simultaneously, yielding pairs of two optional elements.
    /// When both iterators return None, all further invocations of next() will
    /// return None.
    ///
    /// # Example
    ///
    /// ```rust
    /// let a = [0i];
    /// let b = [1i, 2i];
    /// let mut it = a.iter().zip(b.iter());
    /// let (x0, x1, x2) = (0i, 1i, 2i);
    /// assert_eq!(it.next().unwrap(), (Some(&x0), Some(&x1)));
    /// assert_eq!(it.next().unwrap(), (None, Some(&x2)));
    /// assert!(it.next().is_none());
    /// ```
    #[inline]
    fn zip_longest<U: Iterator>(self, other: U) -> ZipLongest<Self, U> {
        ZipLongest{a: self, b: other}
    }
}


/// An iterator which iterates two other iterators simultaneously
#[derive(Clone)]
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct ZipLongest<T, U> {
    a: T,
    b: U
}

impl<A, B, T: Iterator<Item = A>, U: Iterator<Item = B>> Iterator for ZipLongest<T, U> {
    type Item = EitherOrBoth<A, B>;

    #[inline]
    fn next(&mut self) -> Option<EitherOrBoth<A, B>> {
        match (self.a.next(), self.b.next()) {
            (None, None) => None,
            (Some(a), None) => Some(EitherOrBoth::Left(a)),
            (None, Some(b)) => Some(EitherOrBoth::Right(b)),
            (Some(a), Some(b)) => Some(EitherOrBoth::Both(a, b)),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (a_lower, a_upper) = self.a.size_hint();
        let (b_lower, b_upper) = self.b.size_hint();

        let lower = cmp::max(a_lower, b_lower);

        let upper = match (a_upper, b_upper) {
            (Some(x), Some(y)) => Some(cmp::max(x,y)),
            _ => None
        };

        (lower, upper)
    }
}

impl<T, U> DoubleEndedIterator for ZipLongest<T, U>
where T: DoubleEndedIterator + ExactSizeIterator, U: DoubleEndedIterator + ExactSizeIterator {
    #[inline]
    fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
        use std::cmp::Ordering::{Equal, Greater, Less};
        match self.a.len().cmp(&self.b.len()) {
            Equal => match (self.a.next_back(), self.b.next_back()) {
                (None, None) => None,
                (Some(a), Some(b)) => Some(EitherOrBoth::Both(a, b)),
                // XXX these can only happen if .len() is inconsistent with .next_back()
                (Some(a), None) => Some(EitherOrBoth::Left(a)),
                (None, Some(b)) => Some(EitherOrBoth::Right(b)),
            },
            Greater => self.a.next_back().map(EitherOrBoth::Left),
            Less => self.b.next_back().map(EitherOrBoth::Right),
        }
    }
}

impl<T: RandomAccessIterator, U: RandomAccessIterator> RandomAccessIterator for ZipLongest<T, U> {
    #[inline]
    fn indexable(&self) -> usize {
        cmp::max(self.a.indexable(), self.b.indexable())
    }

    #[inline]
    fn idx(&mut self, index: usize) -> Option<<Self as Iterator>::Item> {
        match (self.a.idx(index), self.b.idx(index)) {
            (None, None) => None,
            (Some(a), None) => Some(EitherOrBoth::Left(a)),
            (None, Some(b)) => Some(EitherOrBoth::Right(b)),
            (Some(a), Some(b)) => Some(EitherOrBoth::Both(a, b)),
        }
    }
}

impl<T: ExactSizeIterator, U: ExactSizeIterator> ExactSizeIterator for ZipLongest<T, U> {}


impl<I> ZipLongestIteratorExt for I where I: Iterator {}


/// A value yielded by `ZipLongest`.
/// Contains one or two values,
/// depending on which of the input iterators are exhausted.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum EitherOrBoth<A, B> {
    /// Neither input iterator is exhausted yet, yielding two values.
    Both(A, B),
    /// The parameter iterator of `.zip_longest()` is exhausted,
    /// only yielding a value from the `self` iterator.
    Left(A),
    /// The `self` iterator of `.zip_longest()` is exhausted,
    /// only yielding a value from the parameter iterator.
    Right(B),
}


#[test]
fn test_iterator_size_hint() {
    use std::uint;
    use std::iter::count;

    let c = count(0i32, 1);
    let v: &[_] = &[0i32, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let v2 = &[10i32, 11, 12];
    let vi = v.iter();
    assert_eq!(c.zip_longest(vi.clone()).size_hint(), (uint::MAX, None));
    assert_eq!(vi.zip_longest(v2.iter()).size_hint(), (10, Some(10)));
}

#[test]
fn test_double_ended() {
    let xs = [1i32, 2, 3, 4, 5, 6];
    let ys = [1i32, 2, 3, 7];
    let a = xs.iter().map(|&x| x);
    let b = ys.iter().map(|&x| x);
    let mut it = a.zip_longest(b);
    assert_eq!(it.next(), Some(EitherOrBoth::Both(1, 1)));
    assert_eq!(it.next(), Some(EitherOrBoth::Both(2, 2)));
    assert_eq!(it.next_back(), Some(EitherOrBoth::Left(6)));
    assert_eq!(it.next_back(), Some(EitherOrBoth::Left(5)));
    assert_eq!(it.next_back(), Some(EitherOrBoth::Both(4, 7)));
    assert_eq!(it.next(), Some(EitherOrBoth::Both(3, 3)));
    assert_eq!(it.next(), None);
}

#[test]
fn test_random_access() {
    use std::slice::Iter;

    let xs = [1i32, 2, 3, 4, 5];
    let ys = [7i32, 9, 11];
    check_randacc_iter(xs.iter().zip_longest(ys.iter()),
                       cmp::max(xs.len(), ys.len()));

    fn check_randacc_iter(a: ZipLongest<Iter<i32>, Iter<i32>>, len: usize) {
        let mut b = a.clone();
        assert_eq!(len, b.indexable());
        let mut n = 0us;
        for (i, elt) in a.enumerate() {
            assert!(Some(elt) == b.idx(i));
            n += 1;
        }
        assert_eq!(n, len);
        assert!(None == b.idx(n));
        // call recursively to check after picking off an element
        if len > 0 {
            b.next();
            check_randacc_iter(b, len-1);
        }
    }
}
