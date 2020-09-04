use std::ops::{BitXor, BitXorAssign};

use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// limbs of the bitwise xor of the `Natural` and a `Limb`. `limbs` cannot be empty.
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
/// # Example
/// ```
/// use malachite_nz::natural::logic::xor::limbs_xor_limb;
///
/// assert_eq!(limbs_xor_limb(&[123, 456], 789), &[878, 456]);
/// ```
pub fn limbs_xor_limb(xs: &[Limb], y: Limb) -> Vec<Limb> {
    let mut result = xs.to_vec();
    limbs_xor_limb_in_place(&mut result, y);
    result
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the bitwise xor of the `Natural` and a `Limb` to an output slice. The output slice must
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
/// # Example
/// ```
/// use malachite_nz::natural::logic::xor::limbs_xor_limb_to_out;
///
/// let mut out = vec![0, 0, 0];
/// limbs_xor_limb_to_out(&mut out, &[123, 456], 789);
/// assert_eq!(out, &[878, 456, 0]);
/// ```
pub fn limbs_xor_limb_to_out(out: &mut [Limb], xs: &[Limb], y: Limb) {
    out[..xs.len()].copy_from_slice(xs);
    limbs_xor_limb_in_place(out, y);
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the bitwise xor of the `Natural` and a `Limb` to the input slice. `limbs` cannot be
/// empty.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Panics
/// Panics if `in_limbs` is empty.
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::xor::limbs_xor_limb_in_place;
///
/// let mut limbs = vec![123, 456];
/// limbs_xor_limb_in_place(&mut limbs, 789);
/// assert_eq!(limbs, &[878, 456]);
/// ```
pub fn limbs_xor_limb_in_place(xs: &mut [Limb], y: Limb) {
    xs[0] ^= y;
}

/// Interpreting two equal-length slices of `Limb`s as the limbs (in ascending order) of two
/// `Natural`s, returns a `Vec` of the limbs of the bitwise xor of the `Natural`s. The length of the
/// result is the length of one of the input slices.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `xs.len()` = `ys.len()`
///
/// This is mpn_xor_n from gmp-impl.h, GMP 6.1.2, where rp is returned.
///
/// # Panics
/// Panics if `xs` and `ys` have different lengths.
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::xor::limbs_xor_same_length;
///
/// assert_eq!(limbs_xor_same_length(&[6, 7], &[1, 2]), &[7, 5]);
/// assert_eq!(limbs_xor_same_length(&[100, 101, 102], &[102, 101, 100]), &[2, 0, 2]);
/// ```
pub fn limbs_xor_same_length(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    assert_eq!(xs.len(), ys.len());
    xs.iter().zip(ys.iter()).map(|(x, y)| x ^ y).collect()
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, returns
/// a `Vec` of the limbs of the bitwise xor of the `Natural`s. The length of the result is the
/// length of the longer input slice.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = max(`xs.len()`, `ys.len()`)
///
/// This is mpz_xor from mpz/xor.c, GMP 6.1.2, where res is returned and both inputs are non-
/// negative.
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::xor::limbs_xor;
///
/// assert_eq!(limbs_xor(&[6, 7], &[1, 2, 3]), &[7, 5, 3]);
/// assert_eq!(limbs_xor(&[100, 101, 102], &[102, 101, 100]), &[2, 0, 2]);
/// ```
pub fn limbs_xor(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    let xs_len = xs.len();
    let ys_len = ys.len();
    if xs_len >= ys_len {
        let mut result = limbs_xor_same_length(&xs[..ys_len], ys);
        result.extend_from_slice(&xs[ys_len..]);
        result
    } else {
        let mut result = limbs_xor_same_length(xs, &ys[..xs_len]);
        result.extend_from_slice(&ys[xs_len..]);
        result
    }
}

/// Interpreting two equal-length slices of `Limb`s as the limbs (in ascending order) of two
/// `Natural`s, writes the limbs of the bitwise xor of the `Natural`s to an output slice. The output
/// must be at least as long as one of the input slices.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` = `ys.len()`
///
/// # Panics
/// Panics if `xs` and `ys` have different lengths or if `out` is too short.
///
/// This is mpn_xor_n from gmp-impl.h, GMP 6.1.2.
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::xor::limbs_xor_same_length_to_out;
///
/// let limbs = &mut [10, 10, 10, 10];
/// limbs_xor_same_length_to_out(limbs, &[6, 7], &[1, 2]);
/// assert_eq!(limbs, &[7, 5, 10, 10]);
///
/// let limbs = &mut [10, 10, 10, 10];
/// limbs_xor_same_length_to_out(limbs, &[100, 101, 102], &[102, 101, 100]);
/// assert_eq!(limbs, &[2, 0, 2, 10]);
/// ```
pub fn limbs_xor_same_length_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let len = xs.len();
    assert_eq!(len, ys.len());
    assert!(out.len() >= len);
    for i in 0..xs.len() {
        out[i] = xs[i] ^ ys[i];
    }
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
/// the limbs of the bitwise xor of the `Natural`s to an output slice. The output must be at least
/// as long as the longer input slice.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = max(`xs.len()`, `ys.len()`)
///
/// # Panics
/// Panics if `out` is too short.
///
/// This is mpz_xor from mpz/xor.c, GMP 6.1.2, where both inputs are non-negative.
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::xor::limbs_xor_to_out;
///
/// let limbs = &mut [10, 10, 10, 10];
/// limbs_xor_to_out(limbs, &[6, 7], &[1, 2, 3]);
/// assert_eq!(limbs, &[7, 5, 3, 10]);
///
/// let limbs = &mut [10, 10, 10, 10];
/// limbs_xor_to_out(limbs, &[100, 101, 102], &[102, 101, 100]);
/// assert_eq!(limbs, &[2, 0, 2, 10]);
/// ```
pub fn limbs_xor_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    if xs_len >= ys_len {
        assert!(out.len() >= xs_len);
        limbs_xor_same_length_to_out(out, &xs[..ys_len], ys);
        out[ys_len..xs_len].copy_from_slice(&xs[ys_len..]);
    } else {
        assert!(out.len() >= ys_len);
        limbs_xor_same_length_to_out(out, xs, &ys[..xs_len]);
        out[xs_len..ys_len].copy_from_slice(&ys[xs_len..]);
    }
}

/// Interpreting two equal-length slices of `Limb`s as the limbs (in ascending order) of two
/// `Natural`s, writes the limbs of the bitwise xor of the `Natural`s to the first (left) slice.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` = `ys.len()`
///
/// This is mpn_xor_n from gmp-impl.h, GMP 6.1.2, where rp == up.
///
/// # Panics
/// Panics if `xs` and `ys` have different lengths.
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::xor::limbs_xor_same_length_in_place_left;
///
/// let xs = &mut [6, 7];
/// limbs_xor_same_length_in_place_left(xs, &[1, 2]);
/// assert_eq!(xs, &[7, 5]);
///
/// let xs = &mut [100, 101, 102];
/// limbs_xor_same_length_in_place_left(xs, &[102, 101, 100]);
/// assert_eq!(xs, &[2, 0, 2]);
/// ```
pub fn limbs_xor_same_length_in_place_left(xs: &mut [Limb], ys: &[Limb]) {
    assert_eq!(xs.len(), ys.len());
    for i in 0..xs.len() {
        xs[i] ^= ys[i];
    }
}

/// Interpreting a `Vec` of `Limb`s and a slice of `Limb`s as the limbs (in ascending order) of two
/// `Natural`s, writes the limbs of the bitwise xor of the `Natural`s to the `Vec`. If `ys` is
/// longer than `xs`, `xs` will be extended.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `ys.len()`
///
/// This is mpz_xor from mpz/xor.c, GMP 6.1.2, where res == op1 and both inputs are non-negative.
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::xor::limbs_xor_in_place_left;
///
/// let mut xs = vec![6, 7];
/// limbs_xor_in_place_left(&mut xs, &[1, 2, 3]);
/// assert_eq!(xs, &[7, 5, 3]);
///
/// let mut xs = vec![1, 2, 3];
/// limbs_xor_in_place_left(&mut xs, &[6, 7]);
/// assert_eq!(xs, &[7, 5, 3]);
///
/// let mut xs = vec![100, 101, 102];
/// limbs_xor_in_place_left(&mut xs, &[102, 101, 100]);
/// assert_eq!(xs, &[2, 0, 2]);
/// ```
pub fn limbs_xor_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    if xs_len >= ys_len {
        limbs_xor_same_length_in_place_left(&mut xs[..ys_len], ys);
    } else {
        limbs_xor_same_length_in_place_left(xs, &ys[..xs_len]);
        xs.extend_from_slice(&ys[xs_len..]);
    }
}

/// Interpreting two `Vec`s of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
/// the limbs of the bitwise xor of the `Natural`s to the longer slice (or the first one, if they
/// are equally long). Returns a `bool` which is `false` when the output is to the first slice and
/// `true` when it's to the second slice.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = min(`xs.len`, `ys.len()`)
///
/// This is mpz_xor from mpz/xor.c, GMP 6.1.2, where both inputs are non-negative and the result is
/// written to the longer input slice.
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::xor::limbs_xor_in_place_either;
///
/// let mut xs = vec![6, 7];
/// let mut ys = vec![1, 2, 3];
/// assert_eq!(limbs_xor_in_place_either(&mut xs, &mut ys), true);
/// assert_eq!(xs, &[6, 7]);
/// assert_eq!(ys, &[7, 5, 3]);
///
/// let mut xs = vec![1, 2, 3];
/// let mut ys = vec![6, 7];
/// assert_eq!(limbs_xor_in_place_either(&mut xs, &mut ys), false);
/// assert_eq!(xs, &[7, 5, 3]);
/// assert_eq!(ys, &[6, 7]);
///
/// let mut xs = vec![100, 101, 102];
/// let mut ys = vec![102, 101, 100];
/// assert_eq!(limbs_xor_in_place_either(&mut xs, &mut ys), false);
/// assert_eq!(xs, &[2, 0, 2]);
/// assert_eq!(ys, &[102, 101, 100]);
/// ```
pub fn limbs_xor_in_place_either(xs: &mut Vec<Limb>, ys: &mut Vec<Limb>) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    if xs_len >= ys_len {
        limbs_xor_same_length_in_place_left(&mut xs[..ys_len], ys);
        false
    } else {
        limbs_xor_same_length_in_place_left(&mut ys[..xs_len], xs);
        true
    }
}

impl Natural {
    #[inline]
    fn xor_limb(mut self, other: Limb) -> Natural {
        self.xor_assign_limb(other);
        self
    }

    fn xor_limb_ref(&self, other: Limb) -> Natural {
        Natural(match *self {
            Natural(Small(small)) => Small(small ^ other),
            Natural(Large(ref limbs)) => Large(limbs_xor_limb(limbs, other)),
        })
    }

    fn xor_assign_limb(&mut self, other: Limb) {
        match *self {
            Natural(Small(ref mut small)) => *small ^= other,
            Natural(Large(ref mut limbs)) => limbs_xor_limb_in_place(limbs, other),
        }
    }
}

/// Takes the bitwise xor of two `Natural`s, taking both by value.
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
/// assert_eq!((Natural::from(123u32) ^ Natural::from(456u32)).to_string(), "435");
/// assert_eq!((Natural::trillion() ^ (Natural::trillion() - Natural::ONE)).to_string(), "8191");
/// ```
impl BitXor<Natural> for Natural {
    type Output = Natural;

    #[inline]
    fn bitxor(mut self, other: Natural) -> Natural {
        self ^= other;
        self
    }
}

/// Takes the bitwise xor of two `Natural`s, taking the left `Natural` by value and the right
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
/// assert_eq!((Natural::from(123u32) ^ &Natural::from(456u32)).to_string(), "435");
/// assert_eq!((Natural::trillion() ^ &(Natural::trillion() - Natural::ONE)).to_string(), "8191");
/// ```
impl<'a> BitXor<&'a Natural> for Natural {
    type Output = Natural;

    #[inline]
    fn bitxor(mut self, other: &'a Natural) -> Natural {
        self ^= other;
        self
    }
}

/// Takes the bitwise xor of two `Natural`s, taking the left `Natural` by reference and the right
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
/// assert_eq!((&Natural::from(123u32) ^ Natural::from(456u32)).to_string(), "435");
/// assert_eq!((&Natural::trillion() ^ (Natural::trillion() - Natural::ONE)).to_string(), "8191");
/// ```
impl<'a> BitXor<Natural> for &'a Natural {
    type Output = Natural;

    #[inline]
    fn bitxor(self, mut other: Natural) -> Natural {
        other ^= self;
        other
    }
}

/// Takes the bitwise xor of two `Natural`s, taking both `Natural`s by reference.
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
/// assert_eq!((&Natural::from(123u32) ^ &Natural::from(456u32)).to_string(), "435");
/// assert_eq!((&Natural::trillion() ^ &(Natural::trillion() - Natural::ONE)).to_string(), "8191");
/// ```
impl<'a, 'b> BitXor<&'a Natural> for &'b Natural {
    type Output = Natural;

    fn bitxor(self, other: &'a Natural) -> Natural {
        match (self, other) {
            (x, &Natural(Small(y))) => x.xor_limb_ref(y),
            (&Natural(Small(x)), y) => y.xor_limb_ref(x),
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => {
                Natural::from_owned_limbs_asc(limbs_xor(xs, ys))
            }
        }
    }
}

/// Bitwise-xors a `Natural` with another `Natural` in place, taking the `Natural` on the RHS by
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
/// x ^= Natural::from(0x0000_000fu32);
/// x ^= Natural::from(0x0000_0f00u32);
/// x ^= Natural::from(0x000f_0000u32);
/// x ^= Natural::from(0x0f00_0000u32);
/// assert_eq!(x, 0x0f0f_0f0f);
/// ```
impl BitXorAssign<Natural> for Natural {
    fn bitxor_assign(&mut self, other: Natural) {
        if let Natural(Small(y)) = other {
            self.xor_assign_limb(y);
        } else if let Natural(Small(x)) = *self {
            *self = other.xor_limb(x);
        } else if let Natural(Large(mut ys)) = other {
            if let Natural(Large(ref mut xs)) = *self {
                if limbs_xor_in_place_either(xs, &mut ys) {
                    *xs = ys;
                }
            }
            self.trim();
        }
    }
}

/// Bitwise-xors a `Natural` with another `Natural` in place, taking the `Natural` on the RHS by
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
/// x |= Natural::from(0x0000_000fu32);
/// x |= Natural::from(0x0000_0f00u32);
/// x |= Natural::from(0x000f_0000u32);
/// x |= Natural::from(0x0f00_0000u32);
/// assert_eq!(x, 0x0f0f_0f0f);
/// ```
impl<'a> BitXorAssign<&'a Natural> for Natural {
    fn bitxor_assign(&mut self, other: &'a Natural) {
        if let Natural(Small(y)) = *other {
            self.xor_assign_limb(y);
        } else if let Natural(Small(x)) = *self {
            *self = other.xor_limb_ref(x);
        } else if let Natural(Large(ref ys)) = *other {
            if let Natural(Large(ref mut xs)) = *self {
                limbs_xor_in_place_left(xs, ys);
            }
            self.trim();
        }
    }
}
