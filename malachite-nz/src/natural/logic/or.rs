use std::ops::{BitOr, BitOrAssign};

use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// limbs of the bitwise or of the `Natural` and a `Limb`. `limbs` cannot be empty.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `in_limbs` is empty.
///
/// # Examples
/// ```
/// use malachite_nz::natural::logic::or::limbs_or_limb;
///
/// assert_eq!(limbs_or_limb(&[123, 456], 789), &[895, 456]);
/// ```
pub fn limbs_or_limb(xs: &[Limb], y: Limb) -> Vec<Limb> {
    let mut result = xs.to_vec();
    limbs_or_limb_in_place(&mut result, y);
    result
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the bitwise or of the `Natural` and a `Limb` to an output slice. The output slice must
/// be at least as long as the input slice. `in_limbs` cannot be empty.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `out` is shorter than `in_limbs` or if `in_limbs` is empty.
///
/// # Examples
/// ```
/// use malachite_nz::natural::logic::or::limbs_or_limb_to_out;
///
/// let mut out = vec![0, 0, 0];
/// limbs_or_limb_to_out(&mut out, &[123, 456], 789);
/// assert_eq!(out, &[895, 456, 0]);
/// ```
pub fn limbs_or_limb_to_out(out: &mut [Limb], xs: &[Limb], y: Limb) {
    out[..xs.len()].copy_from_slice(xs);
    limbs_or_limb_in_place(out, y);
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the bitwise or of the `Natural` and a `Limb` to the input slice. `limbs` cannot be
/// empty.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Panics
/// Panics if `limbs` is empty.
///
/// # Examples
/// ```
/// use malachite_nz::natural::logic::or::limbs_or_limb_in_place;
///
/// let mut limbs = vec![123, 456];
/// limbs_or_limb_in_place(&mut limbs, 789);
/// assert_eq!(limbs, &[895, 456]);
/// ```
pub fn limbs_or_limb_in_place(xs: &mut [Limb], y: Limb) {
    xs[0] |= y;
}

/// Interpreting two equal-length slices of `Limb`s as the limbs (in ascending order) of two
/// `Natural`s, returns a `Vec` of the limbs of the bitwise or of the `Natural`s. The length of the
/// result is the length of one of the input slices.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `xs.len()` = `ys.len()`
///
/// This is mpn_ior_n from gmp-impl.h, GMP 6.1.2, where rp is returned.
///
/// # Panics
/// Panics if `xs` and `ys` have different lengths.
///
/// # Examples
/// ```
/// use malachite_nz::natural::logic::or::limbs_or_same_length;
///
/// assert_eq!(limbs_or_same_length(&[6, 7], &[1, 2]), &[7, 7]);
/// assert_eq!(limbs_or_same_length(&[100, 101, 102], &[102, 101, 100]), &[102, 101, 102]);
/// ```
pub fn limbs_or_same_length(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    assert_eq!(xs.len(), ys.len());
    xs.iter().zip(ys.iter()).map(|(x, y)| x | y).collect()
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, returns
/// a `Vec` of the limbs of the bitwise or of the `Natural`s. The length of the result is the length
/// of the longer input slice.
///
/// This is mpz_ior from mpz/ior.c, GMP 6.1.2, where res is returned and both inputs are non-
/// negative.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = max(`xs.len()`, `ys.len()`)
///
/// # Examples
/// ```
/// use malachite_nz::natural::logic::or::limbs_or;
///
/// assert_eq!(limbs_or(&[6, 7], &[1, 2, 3]), &[7, 7, 3]);
/// assert_eq!(limbs_or(&[100, 101, 102], &[102, 101, 100]), &[102, 101, 102]);
/// ```
pub fn limbs_or(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    let xs_len = xs.len();
    let ys_len = ys.len();
    if xs_len >= ys_len {
        let mut result = limbs_or_same_length(&xs[..ys_len], ys);
        result.extend_from_slice(&xs[ys_len..]);
        result
    } else {
        let mut result = limbs_or_same_length(xs, &ys[..xs_len]);
        result.extend_from_slice(&ys[xs_len..]);
        result
    }
}

/// Interpreting two equal-length slices of `Limb`s as the limbs (in ascending order) of two
/// `Natural`s, writes the limbs of the bitwise or of the `Natural`s to an output slice. The output
/// must be at least as long as one of the input slices.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` = `ys.len()`
///
/// This is mpn_ior_n from gmp-impl.h, GMP 6.1.2.
///
/// # Panics
/// Panics if `xs` and `ys` have different lengths or if `out` is too short.
///
/// # Examples
/// ```
/// use malachite_nz::natural::logic::or::limbs_or_same_length_to_out;
///
/// let limbs = &mut [10, 10, 10, 10];
/// limbs_or_same_length_to_out(limbs, &[6, 7], &[1, 2]);
/// assert_eq!(limbs, &[7, 7, 10, 10]);
///
/// let limbs = &mut [10, 10, 10, 10];
/// limbs_or_same_length_to_out(limbs, &[100, 101, 102], &[102, 101, 100]);
/// assert_eq!(limbs, &[102, 101, 102, 10]);
/// ```
pub fn limbs_or_same_length_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let len = xs.len();
    assert_eq!(len, ys.len());
    assert!(out.len() >= len);
    for i in 0..xs.len() {
        out[i] = xs[i] | ys[i];
    }
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
/// the limbs of the bitwise or of the `Natural`s to an output slice. The output must be at least as
/// long as the longer input slice.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = max(`xs.len()`, `ys.len()`)
///
/// This is mpz_ior from mpz/ior.c, GMP 6.1.2, where both inputs are non-negative.
///
/// # Panics
/// Panics if `out` is too short.
///
/// # Examples
/// ```
/// use malachite_nz::natural::logic::or::limbs_or_to_out;
///
/// let limbs = &mut [10, 10, 10, 10];
/// limbs_or_to_out(limbs, &[6, 7], &[1, 2, 3]);
/// assert_eq!(limbs, &[7, 7, 3, 10]);
///
/// let limbs = &mut [10, 10, 10, 10];
/// limbs_or_to_out(limbs, &[100, 101, 102], &[102, 101, 100]);
/// assert_eq!(limbs, &[102, 101, 102, 10]);
/// ```
pub fn limbs_or_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    if xs_len >= ys_len {
        assert!(out.len() >= xs_len);
        limbs_or_same_length_to_out(out, &xs[..ys_len], ys);
        out[ys_len..xs_len].copy_from_slice(&xs[ys_len..]);
    } else {
        assert!(out.len() >= ys_len);
        limbs_or_same_length_to_out(out, xs, &ys[..xs_len]);
        out[xs_len..ys_len].copy_from_slice(&ys[xs_len..]);
    }
}

/// Interpreting two equal-length slices of `Limb`s as the limbs (in ascending order) of two
/// `Natural`s, writes the limbs of the bitwise or of the `Natural`s to the first (left) slice.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` = `ys.len()`
///
/// This is mpn_ior_n from gmp-impl.h, GMP 6.1.2, where rp == up.
///
/// # Panics
/// Panics if `xs` and `ys` have different lengths.
///
/// # Examples
/// ```
/// use malachite_nz::natural::logic::or::limbs_or_same_length_in_place_left;
///
/// let xs = &mut [6, 7];
/// limbs_or_same_length_in_place_left(xs, &[1, 2]);
/// assert_eq!(xs, &[7, 7]);
///
/// let xs = &mut [100, 101, 102];
/// limbs_or_same_length_in_place_left(xs, &[102, 101, 100]);
/// assert_eq!(xs, &[102, 101, 102]);
/// ```
pub fn limbs_or_same_length_in_place_left(xs: &mut [Limb], ys: &[Limb]) {
    assert_eq!(xs.len(), ys.len());
    for i in 0..xs.len() {
        xs[i] |= ys[i];
    }
}

/// Interpreting a `Vec` of `Limb`s and a slice of `Limb`s as the limbs (in ascending order) of two
/// `Natural`s, writes the limbs of the bitwise or of the `Natural`s to the `Vec`. If `ys` is longer
/// than `xs`, `xs` will be extended.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `ys.len()`
///
/// This is mpz_ior from mpz/ior.c, GMP 6.1.2, where res == op1 and both inputs are non-negative.
///
/// # Examples
/// ```
/// use malachite_nz::natural::logic::or::limbs_or_in_place_left;
///
/// let mut xs = vec![6, 7];
/// limbs_or_in_place_left(&mut xs, &[1, 2, 3]);
/// assert_eq!(xs, &[7, 7, 3]);
///
/// let mut xs = vec![1, 2, 3];
/// limbs_or_in_place_left(&mut xs, &[6, 7]);
/// assert_eq!(xs, &[7, 7, 3]);
///
/// let mut xs = vec![100, 101, 102];
/// limbs_or_in_place_left(&mut xs, &[102, 101, 100]);
/// assert_eq!(xs, &[102, 101, 102]);
/// ```
pub fn limbs_or_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    if xs_len >= ys_len {
        limbs_or_same_length_in_place_left(&mut xs[..ys_len], ys);
    } else {
        limbs_or_same_length_in_place_left(xs, &ys[..xs_len]);
        xs.extend_from_slice(&ys[xs_len..]);
    }
}

/// Interpreting two `Vec`s of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
/// the limbs of the bitwise or of the `Natural`s to the longer slice (or the first one, if they are
/// equally long). Returns a `bool` which is `false` when the output is to the first slice and
/// `true` when it's to the second slice.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = min(`xs.len`, `ys.len()`)
///
/// This is mpz_ior from mpz/ior.c, GMP 6.1.2, where both inputs are non-negative and the result is
/// written to the longer input slice.
///
/// # Examples
/// ```
/// use malachite_nz::natural::logic::or::limbs_or_in_place_either;
///
/// let mut xs = vec![6, 7];
/// let mut ys = vec![1, 2, 3];
/// assert_eq!(limbs_or_in_place_either(&mut xs, &mut ys), true);
/// assert_eq!(xs, &[6, 7]);
/// assert_eq!(ys, &[7, 7, 3]);
///
/// let mut xs = vec![1, 2, 3];
/// let mut ys = vec![6, 7];
/// assert_eq!(limbs_or_in_place_either(&mut xs, &mut ys), false);
/// assert_eq!(xs, &[7, 7, 3]);
/// assert_eq!(ys, &[6, 7]);
///
/// let mut xs = vec![100, 101, 102];
/// let mut ys = vec![102, 101, 100];
/// assert_eq!(limbs_or_in_place_either(&mut xs, &mut ys), false);
/// assert_eq!(xs, &[102, 101, 102]);
/// assert_eq!(ys, &[102, 101, 100]);
/// ```
pub fn limbs_or_in_place_either(xs: &mut Vec<Limb>, ys: &mut Vec<Limb>) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    if xs_len >= ys_len {
        limbs_or_same_length_in_place_left(&mut xs[..ys_len], ys);
        false
    } else {
        limbs_or_same_length_in_place_left(&mut ys[..xs_len], xs);
        true
    }
}

impl Natural {
    #[inline]
    fn or_limb(mut self, other: Limb) -> Natural {
        self.or_assign_limb(other);
        self
    }

    fn or_limb_ref(&self, other: Limb) -> Natural {
        Natural(match *self {
            Natural(Small(small)) => Small(small | other),
            Natural(Large(ref limbs)) => Large(limbs_or_limb(limbs, other)),
        })
    }

    fn or_assign_limb(&mut self, other: Limb) {
        match *self {
            Natural(Small(ref mut small)) => *small |= other,
            Natural(Large(ref mut limbs)) => limbs_or_limb_in_place(limbs, other),
        }
    }
}

/// Takes the bitwise or of two `Natural`s, taking both by value.
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
/// assert_eq!((Natural::from(123u32) | Natural::from(456u32)).to_string(), "507");
/// assert_eq!((Natural::trillion() | (Natural::trillion() - Natural::ONE)).to_string(),
///     "1000000004095");
/// ```
impl BitOr<Natural> for Natural {
    type Output = Natural;

    #[inline]
    fn bitor(mut self, other: Natural) -> Natural {
        self |= other;
        self
    }
}

/// Takes the bitwise or of two `Natural`s, taking the left `Natural` by value and the right
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
/// assert_eq!((Natural::from(123u32) | &Natural::from(456u32)).to_string(), "507");
/// assert_eq!((Natural::trillion() | &(Natural::trillion() - Natural::ONE)).to_string(),
///     "1000000004095");
/// ```
impl<'a> BitOr<&'a Natural> for Natural {
    type Output = Natural;

    #[inline]
    fn bitor(mut self, other: &'a Natural) -> Natural {
        self |= other;
        self
    }
}

/// Takes the bitwise or of two `Natural`s, taking the left `Natural` by reference and the right
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
/// assert_eq!((&Natural::from(123u32) | Natural::from(456u32)).to_string(), "507");
/// assert_eq!((&Natural::trillion() | (Natural::trillion() - Natural::ONE)).to_string(),
///     "1000000004095");
/// ```
impl<'a> BitOr<Natural> for &'a Natural {
    type Output = Natural;

    #[inline]
    fn bitor(self, mut other: Natural) -> Natural {
        other |= self;
        other
    }
}

/// Takes the bitwise or of two `Natural`s, taking both `Natural`s by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `max(self.significant_bits(), other.significant_bits)`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::basic::traits::One;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!((&Natural::from(123u32) | &Natural::from(456u32)).to_string(), "507");
/// assert_eq!((&Natural::trillion() | &(Natural::trillion() - Natural::ONE)).to_string(),
///     "1000000004095");
/// ```
impl<'a, 'b> BitOr<&'a Natural> for &'b Natural {
    type Output = Natural;

    fn bitor(self, other: &'a Natural) -> Natural {
        match (self, other) {
            (x, &Natural(Small(y))) => x.or_limb_ref(y),
            (&Natural(Small(x)), y) => y.or_limb_ref(x),
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => Natural(Large(limbs_or(xs, ys))),
        }
    }
}

/// Bitwise-ors a `Natural` with another `Natural` in place, taking the `Natural` on the RHS by
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
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::natural::Natural;
///
/// let mut x = Natural::ZERO;
/// x |= Natural::from(0x0000000fu32);
/// x |= Natural::from(0x00000f00u32);
/// x |= Natural::from(0x000f_0000u32);
/// x |= Natural::from(0x0f000000u32);
/// assert_eq!(x, 0x0f0f_0f0f);
/// ```
impl BitOrAssign<Natural> for Natural {
    fn bitor_assign(&mut self, other: Natural) {
        if let Natural(Small(y)) = other {
            self.or_assign_limb(y);
        } else if let Natural(Small(x)) = *self {
            *self = other.or_limb(x);
        } else if let Natural(Large(mut ys)) = other {
            if let Natural(Large(ref mut xs)) = *self {
                if limbs_or_in_place_either(xs, &mut ys) {
                    *xs = ys;
                }
            }
        }
    }
}

/// Bitwise-ors a `Natural` with another `Natural` in place, taking the `Natural` on the RHS by
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
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::natural::Natural;
///
/// let mut x = Natural::ZERO;
/// x |= &Natural::from(0x0000000fu32);
/// x |= &Natural::from(0x00000f00u32);
/// x |= &Natural::from(0x000f_0000u32);
/// x |= &Natural::from(0x0f000000u32);
/// assert_eq!(x, 0x0f0f_0f0f);
/// ```
impl<'a> BitOrAssign<&'a Natural> for Natural {
    fn bitor_assign(&mut self, other: &'a Natural) {
        if let Natural(Small(y)) = *other {
            self.or_assign_limb(y);
        } else if let Natural(Small(x)) = *self {
            *self = other.or_limb_ref(x);
        } else if let Natural(Large(ref ys)) = *other {
            if let Natural(Large(ref mut xs)) = *self {
                limbs_or_in_place_left(xs, ys);
            }
        }
    }
}
