use std::iter::FusedIterator;

use malachite::natural::conversion::to_limbs::LimbIterator;

pub struct U32Digits<'a> {
    iter: LimbIterator<'a>,
    next_hi: Option<u32>,
    last_hi_is_zero: bool,
    len: usize,
}

impl<'a> U32Digits<'a> {
    #[inline]
    pub(crate) fn new(iter: LimbIterator<'a>) -> Self {
        let iter_len = iter.len();
        let last_hi_is_zero = iter_len != 0 && (iter[iter_len - 1] >> 32) == 0;
        let len = iter_len * 2 - usize::from(last_hi_is_zero);
        Self {
            iter,
            next_hi: None,
            last_hi_is_zero,
            len,
        }
    }
}

impl Iterator for U32Digits<'_> {
    type Item = u32;

    #[inline]
    fn next(&mut self) -> Option<u32> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;

        self.next_hi.take().or_else(|| {
            let limb = self.iter.next()?;
            let hi = (limb >> 32) as u32;
            let lo = limb as u32;

            self.next_hi = Some(hi);
            Some(lo)
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        self.iter.last().map(|limb| {
            if self.last_hi_is_zero {
                limb as u32
            } else {
                (limb >> 32) as u32
            }
        })
    }
}

impl ExactSizeIterator for U32Digits<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.len
    }
}

// TODO: DoubleEndedIterator

impl FusedIterator for U32Digits<'_> {}

#[test]
fn test_iter_u32_digits() {
    let n = super::BigUint::from(5u8);
    let mut it = n.iter_u32_digits();
    assert_eq!(it.len(), 1);
    assert_eq!(it.next(), Some(5));
    assert_eq!(it.len(), 0);
    assert_eq!(it.next(), None);
    assert_eq!(it.len(), 0);
    assert_eq!(it.next(), None);

    let n = super::BigUint::from(112500000000u64);
    let mut it = n.iter_u32_digits();
    assert_eq!(it.len(), 2);
    assert_eq!(it.next(), Some(830850304));
    assert_eq!(it.len(), 1);
    assert_eq!(it.next(), Some(26));
    assert_eq!(it.len(), 0);
    assert_eq!(it.next(), None);
}
