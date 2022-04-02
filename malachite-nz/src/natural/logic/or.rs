use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;
use std::mem::swap;
use std::ops::{BitOr, BitOrAssign};

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// limbs of the bitwise or of the `Natural` and a `Limb`. `xs` cannot be empty.
//
// Time: worst case O(n)
//
// Additional memory: worst case O(n)
//
// where n = `xs.len()`
//
// # Panics
// Panics if `xs` is empty.
pub fn limbs_or_limb(xs: &[Limb], y: Limb) -> Vec<Limb> {
    let mut result = xs.to_vec();
    limbs_or_limb_in_place(&mut result, y);
    result
}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the bitwise or of the `Natural` and a `Limb` to an output slice. The output slice must
// be at least as long as the input slice. `xs` cannot be empty.
//
// Time: worst case O(n)
//
// Additional memory: worst case O(1)
//
// where n = `xs.len()`
//
// # Panics
// Panics if `out` is shorter than `xs` or if `xs` is empty.
pub_test! {limbs_or_limb_to_out(out: &mut [Limb], xs: &[Limb], y: Limb) {
    out[..xs.len()].copy_from_slice(xs);
    limbs_or_limb_in_place(out, y);
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the bitwise or of the `Natural` and a `Limb` to the input slice. `xs` cannot be empty.
//
// Time: worst case O(1)
//
// Additional memory: worst case O(1)
//
// # Panics
// Panics if `xs` is empty.
pub_test! {limbs_or_limb_in_place(xs: &mut [Limb], y: Limb) {
    xs[0] |= y;
}}

// Interpreting two equal-length slices of `Limb`s as the limbs (in ascending order) of two
// `Natural`s, returns a `Vec` of the limbs of the bitwise or of the `Natural`s. The length of the
// result is the length of one of the input slices.
//
// Time: worst case O(n)
//
// Additional memory: worst case O(n)
//
// where n = `xs.len()` = `ys.len()`
//
// This is mpn_ior_n from gmp-impl.h, GMP 6.2.1, where rp is returned.
//
// # Panics
// Panics if `xs` and `ys` have different lengths.
pub_test! {limbs_or_same_length(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    assert_eq!(xs.len(), ys.len());
    xs.iter().zip(ys.iter()).map(|(x, y)| x | y).collect()
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, returns
// a `Vec` of the limbs of the bitwise or of the `Natural`s. The length of the result is the length
// of the longer input slice.
//
// This is mpz_ior from mpz/ior.c, GMP 6.1.2, where res is returned and both inputs are non-
// negative.
//
// Time: worst case O(n)
//
// Additional memory: worst case O(n)
//
// where n = max(`xs.len()`, `ys.len()`)
pub_test! {limbs_or(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let mut result;
    if xs_len >= ys_len {
        result = limbs_or_same_length(&xs[..ys_len], ys);
        result.extend_from_slice(&xs[ys_len..]);
    } else {
        result = limbs_or_same_length(xs, &ys[..xs_len]);
        result.extend_from_slice(&ys[xs_len..]);
    }
    result
}}

// Interpreting two equal-length slices of `Limb`s as the limbs (in ascending order) of two
// `Natural`s, writes the limbs of the bitwise or of the `Natural`s to an output slice. The output
// must be at least as long as one of the input slices.
//
// Time: worst case O(n)
//
// Additional memory: worst case O(1)
//
// where n = `xs.len()` = `ys.len()`
//
// This is mpn_ior_n from gmp-impl.h, GMP 6.2.1.
//
// # Panics
// Panics if `xs` and `ys` have different lengths or if `out` is too short.
pub_test! {limbs_or_same_length_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let len = xs.len();
    assert_eq!(len, ys.len());
    assert!(out.len() >= len);
    for (out_x, (x, y)) in out.iter_mut().zip(xs.iter().zip(ys.iter())) {
        *out_x = x | y;
    }
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
// the limbs of the bitwise or of the `Natural`s to an output slice. The output must be at least as
// long as the longer input slice.
//
// Time: worst case O(n)
//
// Additional memory: worst case O(1)
//
// where n = max(`xs.len()`, `ys.len()`)
//
// This is mpz_ior from mpz/ior.c, GMP 6.1.2, where both inputs are non-negative.
//
// # Panics
// Panics if `out` is too short.
pub_test! {limbs_or_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
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
}}

// Interpreting two equal-length slices of `Limb`s as the limbs (in ascending order) of two
// `Natural`s, writes the limbs of the bitwise or of the `Natural`s to the first (left) slice.
//
// Time: worst case O(n)
//
// Additional memory: worst case O(1)
//
// where n = `xs.len()` = `ys.len()`
//
// This is mpn_ior_n from gmp-impl.h, GMP 6.2.1, where rp == up.
//
// # Panics
// Panics if `xs` and `ys` have different lengths.
pub_test! {limbs_or_same_length_in_place_left(xs: &mut [Limb], ys: &[Limb]) {
    assert_eq!(xs.len(), ys.len());
    for (x, &y) in xs.iter_mut().zip(ys.iter()) {
        *x |= y;
    }
}}

// Interpreting a `Vec` of `Limb`s and a slice of `Limb`s as the limbs (in ascending order) of two
// `Natural`s, writes the limbs of the bitwise or of the `Natural`s to the `Vec`. If `ys` is longer
// than `xs`, `xs` will be extended.
//
// Time: worst case O(n)
//
// Additional memory: worst case O(n)
//
// where n = `ys.len()`
//
// This is mpz_ior from mpz/ior.c, GMP 6.1.2, where res == op1 and both inputs are non-negative.
pub_test! {limbs_or_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    if xs_len >= ys_len {
        limbs_or_same_length_in_place_left(&mut xs[..ys_len], ys);
    } else {
        limbs_or_same_length_in_place_left(xs, &ys[..xs_len]);
        xs.extend_from_slice(&ys[xs_len..]);
    }
}}

// Interpreting two `Vec`s of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
// the limbs of the bitwise or of the `Natural`s to the longer slice (or the first one, if they are
// equally long). Returns a `bool` which is `false` when the output is to the first slice and
// `true` when it's to the second slice.
//
// Time: worst case O(n)
//
// Additional memory: worst case O(1)
//
// where n = min(`xs.len`, `ys.len()`)
//
// This is mpz_ior from mpz/ior.c, GMP 6.1.2, where both inputs are non-negative and the result is
// written to the longer input slice.
pub_test! {limbs_or_in_place_either(xs: &mut Vec<Limb>, ys: &mut Vec<Limb>) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let right = xs_len < ys_len;
    if right {
        limbs_or_same_length_in_place_left(&mut ys[..xs_len], xs);
    } else {
        limbs_or_same_length_in_place_left(&mut xs[..ys_len], ys);
    }
    right
}}

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

impl BitOr<Natural> for Natural {
    type Output = Natural;

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
    #[inline]
    fn bitor(mut self, other: Natural) -> Natural {
        self |= other;
        self
    }
}

impl<'a> BitOr<&'a Natural> for Natural {
    type Output = Natural;

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
    #[inline]
    fn bitor(mut self, other: &'a Natural) -> Natural {
        self |= other;
        self
    }
}

impl<'a> BitOr<Natural> for &'a Natural {
    type Output = Natural;

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
    #[inline]
    fn bitor(self, mut other: Natural) -> Natural {
        other |= self;
        other
    }
}

impl<'a, 'b> BitOr<&'a Natural> for &'b Natural {
    type Output = Natural;

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
    fn bitor(self, other: &'a Natural) -> Natural {
        match (self, other) {
            (x, &Natural(Small(y))) => x.or_limb_ref(y),
            (&Natural(Small(x)), y) => y.or_limb_ref(x),
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => Natural(Large(limbs_or(xs, ys))),
        }
    }
}

impl BitOrAssign<Natural> for Natural {
    /// Bitwise-ors a `Natural` with another `Natural` in place, taking the `Natural` on the
    /// right-hand side by value.
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
    fn bitor_assign(&mut self, mut other: Natural) {
        match (&mut *self, &mut other) {
            (_, Natural(Small(y))) => self.or_assign_limb(*y),
            (Natural(Small(ref mut x)), _) => *self = other.or_limb(*x),
            (Natural(Large(ref mut xs)), Natural(Large(ref mut ys))) => {
                if limbs_or_in_place_either(xs, ys) {
                    swap(xs, ys);
                }
            }
        }
    }
}

impl<'a> BitOrAssign<&'a Natural> for Natural {
    /// Bitwise-ors a `Natural` with another `Natural` in place, taking the `Natural` on the
    /// right-hand side by reference.
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
    fn bitor_assign(&mut self, other: &'a Natural) {
        match (&mut *self, other) {
            (_, Natural(Small(y))) => self.or_assign_limb(*y),
            (Natural(Small(ref mut x)), _) => *self = other.or_limb_ref(*x),
            (Natural(Large(ref mut xs)), Natural(Large(ref ys))) => {
                limbs_or_in_place_left(xs, ys);
            }
        }
    }
}
