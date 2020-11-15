use std::cmp::Ordering;
use std::mem::swap;
use std::ops::{BitAnd, BitAndAssign};

use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::slices::slice_set_zero;

use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// bitwise and of the `Natural` and a `Limb`. The slice cannot be empty.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Panics
/// Panics if `xs` is empty.
///
/// # Examples
/// ```
/// use malachite_nz::natural::logic::and::limbs_and_limb;
///
/// assert_eq!(limbs_and_limb(&[6, 7], 2), 2);
/// assert_eq!(limbs_and_limb(&[100, 101, 102], 10), 0);
/// ```
pub const fn limbs_and_limb(xs: &[Limb], y: Limb) -> Limb {
    xs[0] & y
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, returns
/// a `Vec` of the limbs of the bitwise and of the `Natural`s. The length of the result is the
/// length of the shorter input slice.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = min(`xs.len()`, `ys.len()`)
///
/// This is mpz_and from mpz/and.c, GMP 6.1.2, where res is returned and both inputs are non-
/// negative.
///
/// # Examples
/// ```
/// use malachite_nz::natural::logic::and::limbs_and;
///
/// assert_eq!(limbs_and(&[6, 7], &[1, 2, 3]), &[0, 2]);
/// assert_eq!(limbs_and(&[100, 101, 102], &[102, 101, 100]), &[100, 101, 100]);
/// ```
pub fn limbs_and(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    xs.iter().zip(ys.iter()).map(|(x, y)| x & y).collect()
}

/// Interpreting two equal-length slices of `Limb`s as the limbs (in ascending order) of two
/// `Natural`s, writes the limbs of the bitwise and of the `Natural`s to a specified slice. The
/// output slice must be at least as long as the length of one of the input slices.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` = `ys.len()`
///
/// This is mpn_and_n from gmp-impl.h, GMP 6.1.2.
///
/// # Panics
/// Panics if `xs` and `ys` have different lengths or if `out` is too short.
///
/// # Examples
/// ```
/// use malachite_nz::natural::logic::and::limbs_and_same_length_to_out;
///
/// let mut out = vec![10, 10, 10, 10];
/// limbs_and_same_length_to_out(&mut out, &[6, 7], &[1, 2]);
/// assert_eq!(out, &[0, 2, 10, 10]);
///
/// let mut out = vec![10, 10, 10, 10];
/// limbs_and_same_length_to_out(&mut out, &[100, 101, 102], &[102, 101, 100]);
/// assert_eq!(out, &[100, 101, 100, 10]);
/// ```
pub fn limbs_and_same_length_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let len = xs.len();
    assert_eq!(len, ys.len());
    assert!(out.len() >= len);
    for (out, (&x, &y)) in out.iter_mut().zip(xs.iter().zip(ys.iter())) {
        *out = x & y;
    }
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
/// the limbs of the bitwise and of the `Natural`s to a specified slice. The output slice must be at
/// least as long as the longer input slice.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = max(`xs.len()`, `ys.len()`)
///
/// This is mpz_and from mpz/and.c, GMP 6.1.2, where both inputs are non-negative.
///
/// # Panics
/// Panics if `out` is too short.
///
/// # Examples
/// ```
/// use malachite_nz::natural::logic::and::limbs_and_to_out;
///
/// let mut out = vec![10, 10, 10, 10];
/// limbs_and_to_out(&mut out, &[6, 7], &[1, 2, 3]);
/// assert_eq!(out, &[0, 2, 0, 10]);
///
/// let mut out = vec![10, 10, 10, 10];
/// limbs_and_to_out(&mut out, &[100, 101, 102], &[102, 101, 100]);
/// assert_eq!(out, &[100, 101, 100, 10]);
/// ```
pub fn limbs_and_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    if xs_len >= ys_len {
        assert!(out.len() >= xs_len);
        limbs_and_same_length_to_out(out, &xs[..ys_len], ys);
        slice_set_zero(&mut out[ys_len..xs_len]);
    } else {
        assert!(out.len() >= ys_len);
        limbs_and_same_length_to_out(out, xs, &ys[..xs_len]);
        slice_set_zero(&mut out[xs_len..ys_len]);
    }
}

/// Interpreting two equal-length slices of `Limb`s as the limbs (in ascending order) of two
/// `Natural`s, writes the limbs of the bitwise and of the `Natural`s to the first (left) slice.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` = `ys.len()`
///
/// This is mpn_and_n from gmp-impl.h, GMP 6.1.2, where rp == up.
///
/// # Panics
/// Panics if `xs` and `ys` have different lengths.
///
/// # Examples
/// ```
/// use malachite_nz::natural::logic::and::limbs_slice_and_same_length_in_place_left;
///
/// let mut xs = vec![6, 7];
/// limbs_slice_and_same_length_in_place_left(&mut xs, &[1, 2]);
/// assert_eq!(xs, &[0, 2]);
///
/// let mut xs = vec![100, 101, 102];
/// limbs_slice_and_same_length_in_place_left(&mut xs, &[102, 101, 100]);
/// assert_eq!(xs, &[100, 101, 100]);
/// ```
pub fn limbs_slice_and_same_length_in_place_left(xs: &mut [Limb], ys: &[Limb]) {
    assert_eq!(xs.len(), ys.len());
    for (x, &y) in xs.iter_mut().zip(ys.iter()) {
        *x &= y;
    }
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
/// the limbs of the bitwise and of the `Natural`s to the first (left) slice. If the second slice is
/// shorter than the first, then some of the most-significant bits of the first slice should become
/// zero. Rather than setting them to zero, this function optionally returns the length of the
/// significant part of the slice. The caller can decide whether to zero the rest. If `None` is
/// returned, the entire slice remains significant.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = min(`xs.len()`, `ys.len()`)
///
/// This is mpz_and from mpz/and.c, GMP 6.1.2, where res == op1 and both inputs are non-negative.
///
/// # Examples
/// ```
/// use malachite_nz::natural::logic::and::limbs_slice_and_in_place_left;
///
/// let mut xs = vec![6, 7];
/// assert_eq!(limbs_slice_and_in_place_left(&mut xs, &[1, 2, 3]), None);
/// assert_eq!(xs, &[0, 2]);
///
/// let mut xs = vec![1, 2, 3];
/// assert_eq!(limbs_slice_and_in_place_left(&mut xs, &[6, 7]), Some(2));
/// assert_eq!(xs, &[0, 2, 3]);
///
/// let mut xs = vec![100, 101, 102];
/// assert_eq!(limbs_slice_and_in_place_left(&mut xs, &[102, 101, 100]), None);
/// assert_eq!(xs, &[100, 101, 100]);
/// ```
pub fn limbs_slice_and_in_place_left(xs: &mut [Limb], ys: &[Limb]) -> Option<usize> {
    let xs_len = xs.len();
    let ys_len = ys.len();
    match xs_len.cmp(&ys.len()) {
        Ordering::Equal => {
            limbs_slice_and_same_length_in_place_left(xs, ys);
            None
        }
        Ordering::Greater => {
            limbs_slice_and_same_length_in_place_left(&mut xs[..ys_len], ys);
            Some(ys_len)
        }
        Ordering::Less => {
            limbs_slice_and_same_length_in_place_left(xs, &ys[..xs_len]);
            None
        }
    }
}

/// Interpreting a `Vec` of `Limb`s and a slice of `Limb`s as the limbs (in ascending order) of two
/// `Natural`s, writes the limbs of the bitwise and of the `Natural`s to the `Vec`. If the slice is
/// shorter than the `Vec`, then some of the most-significant bits of the `Vec` should become zero.
/// Rather than setting them to zero, this function truncates the `Vec`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = min(`xs.len()`, `ys.len()`)
///
/// This is mpz_and from mpz/and.c, GMP 6.1.2, where res == op1 and both inputs are non-negative and
/// have the same length, and res is truncated afterwards to remove the max(0, xs.len() - ys.len())
/// trailing zero limbs.
///
/// # Examples
/// ```
/// use malachite_nz::natural::logic::and::limbs_vec_and_in_place_left;
///
/// let mut xs = vec![6, 7];
/// limbs_vec_and_in_place_left(&mut xs, &[1, 2, 3]);
/// assert_eq!(xs, &[0, 2]);
///
/// let mut xs = vec![1, 2, 3];
/// limbs_vec_and_in_place_left(&mut xs, &[6, 7]);
/// assert_eq!(xs, &[0, 2]);
///
/// let mut xs = vec![100, 101, 102];
/// limbs_vec_and_in_place_left(&mut xs, &[102, 101, 100]);
/// assert_eq!(xs, &[100, 101, 100]);
/// ```
pub fn limbs_vec_and_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb]) {
    if let Some(truncate_size) = limbs_slice_and_in_place_left(xs, ys) {
        xs.truncate(truncate_size);
    }
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, takes
/// the limbs of the bitwise and of the `Natural`s and writes them to the shorter slice (or the
/// first one, if they are equally long). If the function writes to the first slice, it returns
/// `false`; otherwise, it returns `true`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = min(`xs.len()`, `ys.len()`)
///
/// This is mpz_and from mpz/and.c, GMP 6.1.2, where both inputs are non-negative and the result is
/// written to the shorter input slice.
///
/// # Examples
/// ```
/// use malachite_nz::natural::logic::and::limbs_and_in_place_either;
///
/// let mut xs = vec![6, 7];
/// let mut ys = vec![1, 2, 3];
/// assert_eq!(limbs_and_in_place_either(&mut xs, &mut ys), false);
/// assert_eq!(xs, &[0, 2]);
/// assert_eq!(ys, &[1, 2, 3]);
///
/// let mut xs = vec![1, 2, 3];
/// let mut ys = vec![6, 7];
/// assert_eq!(limbs_and_in_place_either(&mut xs, &mut ys), true);
/// assert_eq!(xs, &[1, 2, 3]);
/// assert_eq!(ys, &[0, 2]);
///
/// let mut xs = vec![100, 101, 102];
/// let mut ys = vec![102, 101, 100];
/// assert_eq!(limbs_and_in_place_either(&mut xs, &mut ys), false);
/// assert_eq!(xs, &[100, 101, 100]);
/// assert_eq!(ys, &[102, 101, 100]);
/// ```
pub fn limbs_and_in_place_either(xs: &mut [Limb], ys: &mut [Limb]) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    match xs_len.cmp(&ys_len) {
        Ordering::Equal => {
            limbs_slice_and_same_length_in_place_left(xs, ys);
            false
        }
        Ordering::Less => {
            limbs_slice_and_same_length_in_place_left(xs, &ys[..xs_len]);
            false
        }
        Ordering::Greater => {
            limbs_slice_and_same_length_in_place_left(ys, &xs[..ys_len]);
            true
        }
    }
}

impl Natural {
    fn and_limb(self, other: Limb) -> Limb {
        Limb::wrapping_from(self) & other
    }

    fn and_limb_ref(&self, other: Limb) -> Limb {
        Limb::wrapping_from(self) & other
    }

    fn and_assign_limb(&mut self, other: Limb) {
        *self = Natural(Small(self.and_limb_ref(other)));
    }
}

/// Takes the bitwise and of two `Natural`s, taking both by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `min(self.significant_bits(), other.significant_bits)`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::basic::traits::One;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!((Natural::from(123u32) & Natural::from(456u32)).to_string(), "72");
/// assert_eq!((Natural::trillion() & (Natural::trillion() - Natural::ONE)).to_string(),
///     "999999995904");
/// ```
impl BitAnd<Natural> for Natural {
    type Output = Natural;

    #[inline]
    fn bitand(mut self, other: Natural) -> Natural {
        self &= other;
        self
    }
}

/// Takes the bitwise and of two `Natural`s, taking the left `Natural` by value and the right
/// `Natural` by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `other.significant_bits`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::basic::traits::One;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!((Natural::from(123u32) & &Natural::from(456u32)).to_string(), "72");
/// assert_eq!((Natural::trillion() & &(Natural::trillion() - Natural::ONE)).to_string(),
///     "999999995904");
/// ```
impl<'a> BitAnd<&'a Natural> for Natural {
    type Output = Natural;

    #[inline]
    fn bitand(mut self, other: &'a Natural) -> Natural {
        self &= other;
        self
    }
}

/// Takes the bitwise and of two `Natural`s, taking the left `Natural` by reference and the right
/// `Natural` by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::basic::traits::One;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!((&Natural::from(123u32) & Natural::from(456u32)).to_string(), "72");
/// assert_eq!((&Natural::trillion() & (Natural::trillion() - Natural::ONE)).to_string(),
///     "999999995904");
/// ```
impl<'a> BitAnd<Natural> for &'a Natural {
    type Output = Natural;

    #[inline]
    fn bitand(self, mut other: Natural) -> Natural {
        other &= self;
        other
    }
}

/// Takes the bitwise and of two `Natural`s, taking both `Natural`s by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `min(self.significant_bits(), other.significant_bits)`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::basic::traits::One;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!((&Natural::from(123u32) & &Natural::from(456u32)).to_string(), "72");
/// assert_eq!((&Natural::trillion() & &(Natural::trillion() - Natural::ONE)).to_string(),
///     "999999995904");
/// ```
impl<'a, 'b> BitAnd<&'a Natural> for &'b Natural {
    type Output = Natural;

    fn bitand(self, other: &'a Natural) -> Natural {
        match (self, other) {
            (x, &Natural(Small(y))) => Natural(Small(x.and_limb_ref(y))),
            (&Natural(Small(x)), y) => Natural(Small(y.and_limb_ref(x))),
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => {
                Natural::from_owned_limbs_asc(limbs_and(xs, ys))
            }
        }
    }
}

/// Bitwise-ands a `Natural` with another `Natural` in place, taking the `Natural` on the RHS by
/// value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `min(self.significant_bits(), other.significant_bits)`
///
/// # Examples
/// ```
/// use malachite_nz::natural::Natural;
///
/// let mut x = Natural::from(u32::MAX);
/// x &= Natural::from(0xf0ffffffu32);
/// x &= Natural::from(0xfff0_ffffu32);
/// x &= Natural::from(0xfffff0ffu32);
/// x &= Natural::from(0xfffffff0u32);
/// assert_eq!(x, 0xf0f0_f0f0u32);
/// ```
impl BitAndAssign<Natural> for Natural {
    fn bitand_assign(&mut self, mut other: Natural) {
        match (&mut *self, &mut other) {
            (_, Natural(Small(y))) => self.and_assign_limb(*y),
            (Natural(Small(ref mut x)), _) => *x = other.and_limb(*x),
            (Natural(Large(ref mut xs)), Natural(Large(ref mut ys))) => {
                if limbs_and_in_place_either(xs, ys) {
                    swap(xs, ys);
                }
                self.trim();
            }
        }
    }
}

/// Bitwise-ands a `Natural` with another `Natural` in place, taking the `Natural` on the RHS by
/// reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `other.significant_bits`
///
/// # Examples
/// ```
/// use malachite_nz::natural::Natural;
///
/// let mut x = Natural::from(u32::MAX);
/// x &= &Natural::from(0xf0ffffffu32);
/// x &= &Natural::from(0xfff0_ffffu32);
/// x &= &Natural::from(0xfffff0ffu32);
/// x &= &Natural::from(0xfffffff0u32);
/// assert_eq!(x, 0xf0f0_f0f0u32);
/// ```
impl<'a> BitAndAssign<&'a Natural> for Natural {
    fn bitand_assign(&mut self, other: &'a Natural) {
        match (&mut *self, other) {
            (_, Natural(Small(y))) => self.and_assign_limb(*y),
            (Natural(Small(ref mut x)), _) => *x = other.and_limb_ref(*x),
            (Natural(Large(ref mut xs)), Natural(Large(ref ys))) => {
                limbs_vec_and_in_place_left(xs, ys);
                self.trim();
            }
        }
    }
}
