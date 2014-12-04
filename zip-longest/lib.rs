use std::cmp;

pub trait ZipLongestIteratorExt<A>: Iterator<A> {
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
    fn zip_longest<B, U: Iterator<B>>(self, other: U) -> ZipLongest<Self, U> {
        ZipLongest{a: self, b: other}
    }
}


/// An iterator which iterates two other iterators simultaneously
#[deriving(Clone)]
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct ZipLongest<T, U> {
    a: T,
    b: U
}

impl<A, B, T: Iterator<A>, U: Iterator<B>> Iterator<(Option<A>, Option<B>)> for ZipLongest<T, U> {
    #[inline]
    fn next(&mut self) -> Option<(Option<A>, Option<B>)> {
        match (self.a.next(), self.b.next()) {
            (None, None) => None,
            pair_of_options => Some(pair_of_options),
        }
    }

    #[inline]
    fn size_hint(&self) -> (uint, Option<uint>) {
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

impl<A, B, T: ExactSizeIterator<A>, U: ExactSizeIterator<B>> DoubleEndedIterator<(Option<A>, Option<B>)>
for ZipLongest<T, U> {
    #[inline]
    fn next_back(&mut self) -> Option<(Option<A>, Option<B>)> {
        use std::cmp::{Equal, Greater, Less};
        match self.a.len().cmp(&self.b.len()) {
            Equal => match (self.a.next_back(), self.b.next_back()) {
                (None, None) => None,
                pair_of_options => Some(pair_of_options),
            },
            Greater => self.a.next_back().map(|x| (Some(x), None)),
            Less => self.b.next_back().map(|y| (None, Some(y))),
        }
    }
}

impl<A, B, T: RandomAccessIterator<A>, U: RandomAccessIterator<B>>
RandomAccessIterator<(Option<A>, Option<B>)> for ZipLongest<T, U> {
    #[inline]
    fn indexable(&self) -> uint {
        cmp::max(self.a.indexable(), self.b.indexable())
    }

    #[inline]
    fn idx(&mut self, index: uint) -> Option<(Option<A>, Option<B>)> {
        match (self.a.idx(index), self.b.idx(index)) {
            (None, None) => None,
            pair_of_options => Some(pair_of_options),
        }
    }
}

impl<A, B, T: ExactSizeIterator<A>, U: ExactSizeIterator<B>>
ExactSizeIterator<(Option<A>, Option<B>)> for ZipLongest<T, U> {}


impl<A, I> ZipLongestIteratorExt<A> for I where I: Iterator<A> {}


#[test]
fn test_iterator_size_hint() {
    use std::uint;
    use std::iter::count;

    let c = count(0i, 1);
    let v: &[_] = &[0i, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let v2 = &[10i, 11, 12];
    let vi = v.iter();
    assert_eq!(c.zip_longest(vi).size_hint(), (uint::MAX, None));
    assert_eq!(vi.zip_longest(v2.iter()).size_hint(), (10, Some(10)));
}

#[test]
fn test_double_ended() {
    let xs = [1i, 2, 3, 4, 5, 6];
    let ys = [1i, 2, 3, 7];
    let a = xs.iter().map(|&x| x);
    let b = ys.iter().map(|&x| x);
    let mut it = a.zip_longest(b);
    assert_eq!(it.next(), Some((Some(1), Some(1))));
    assert_eq!(it.next(), Some((Some(2), Some(2))));
    assert_eq!(it.next_back(), Some((Some(6), None)));
    assert_eq!(it.next_back(), Some((Some(5), None)));
    assert_eq!(it.next_back(), Some((Some(4), Some(7))));
    assert_eq!(it.next(), Some((Some(3), Some(3))));
    assert_eq!(it.next(), None);
}

#[test]
fn test_random_access() {
    let xs = [1i, 2, 3, 4, 5];
    let ys = [7i, 9, 11];
    check_randacc_iter(xs.iter().zip_longest(ys.iter()),
                       cmp::max(xs.len(), ys.len()));

    fn check_randacc_iter<A, T>(a: T, len: uint)
    where A: PartialEq, T: Clone + RandomAccessIterator<A>
    {
        let mut b = a.clone();
        assert_eq!(len, b.indexable());
        let mut n = 0u;
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
