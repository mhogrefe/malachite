use natural::Natural::{self, Large, Small};
use platform::Limb;
use std::ops::{BitXor, BitXorAssign};

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
/// use malachite_nz::natural::logic::xor_limb::limbs_xor_limb;
///
/// assert_eq!(limbs_xor_limb(&[123, 456], 789), &[878, 456]);
/// ```
pub fn limbs_xor_limb(limbs: &[Limb], limb: Limb) -> Vec<Limb> {
    let mut result = limbs.to_vec();
    limbs_xor_limb_in_place(&mut result, limb);
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
/// use malachite_nz::natural::logic::xor_limb::limbs_xor_limb_to_out;
///
/// let mut out = vec![0, 0, 0];
/// limbs_xor_limb_to_out(&mut out, &[123, 456], 789);
/// assert_eq!(out, &[878, 456, 0]);
/// ```
pub fn limbs_xor_limb_to_out(out: &mut [Limb], in_limbs: &[Limb], limb: Limb) {
    out[..in_limbs.len()].copy_from_slice(in_limbs);
    limbs_xor_limb_in_place(out, limb);
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
/// use malachite_nz::natural::logic::xor_limb::limbs_xor_limb_in_place;
///
/// let mut limbs = vec![123, 456];
/// limbs_xor_limb_in_place(&mut limbs, 789);
/// assert_eq!(limbs, &[878, 456]);
/// ```
pub fn limbs_xor_limb_in_place(limbs: &mut [Limb], limb: Limb) {
    limbs[0] ^= limb;
}

/// Takes the bitwise xor of a `Natural` and a `Limb`, taking the `Natural` by value.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::Zero;
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((Natural::ZERO ^ 123).to_string(), "123");
///     assert_eq!((Natural::from(123u32) ^ 0).to_string(), "123");
///     assert_eq!((Natural::from_str("12345678987654321").unwrap() ^ 456).to_string(),
///         "12345678987654521");
/// }
/// ```
impl BitXor<Limb> for Natural {
    type Output = Natural;

    #[inline]
    fn bitxor(mut self, other: Limb) -> Natural {
        self ^= other;
        self
    }
}

#[cfg(feature = "64_bit_limbs")]
impl BitXor<u32> for Natural {
    type Output = Natural;

    #[inline]
    fn bitxor(self, other: u32) -> Natural {
        self ^ Limb::from(other)
    }
}

/// Takes the bitwise xor of a `Natural` and an `Limb`, taking the `Natural` by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::Zero;
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((&Natural::ZERO ^ 123).to_string(), "123");
///     assert_eq!((&Natural::from(123u32) ^ 0).to_string(), "123");
///     assert_eq!((&Natural::from_str("12345678987654321").unwrap() ^ 456).to_string(),
///         "12345678987654521");
/// }
/// ```
impl<'a> BitXor<Limb> for &'a Natural {
    type Output = Natural;

    fn bitxor(self, other: Limb) -> Natural {
        match *self {
            Small(small) => Small(small ^ other),
            Large(ref limbs) => Large(limbs_xor_limb(limbs, other)),
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> BitXor<u32> for &'a Natural {
    type Output = Natural;

    #[inline]
    fn bitxor(self, other: u32) -> Natural {
        self ^ Limb::from(other)
    }
}

/// Takes the bitwise xor of a `Limb` and a `Natural`, taking the `Natural` by value.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::Zero;
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((123 ^ Natural::ZERO).to_string(), "123");
///     assert_eq!((0 ^ Natural::from(123u32)).to_string(), "123");
///     assert_eq!((456 ^ Natural::from_str("12345678987654321").unwrap()).to_string(),
///         "12345678987654521");
/// }
/// ```
impl BitXor<Natural> for Limb {
    type Output = Natural;

    #[inline]
    fn bitxor(self, other: Natural) -> Natural {
        other ^ self
    }
}

#[cfg(feature = "64_bit_limbs")]
impl BitXor<Natural> for u32 {
    type Output = Natural;

    #[inline]
    fn bitxor(self, other: Natural) -> Natural {
        Limb::from(self) ^ other
    }
}

/// Takes the bitwise and of a `Limb` and a `Natural`, taking the `Natural` by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `other.significant_bits()`
///
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::Zero;
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((123 ^ &Natural::ZERO).to_string(), "123");
///     assert_eq!((0 ^ &Natural::from(123u32)).to_string(), "123");
///     assert_eq!((456 ^ &Natural::from_str("12345678987654321").unwrap()).to_string(),
///         "12345678987654521");
/// }
/// ```
impl<'a> BitXor<&'a Natural> for Limb {
    type Output = Natural;

    #[inline]
    fn bitxor(self, other: &'a Natural) -> Natural {
        other ^ self
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> BitXor<&'a Natural> for u32 {
    type Output = Natural;

    #[inline]
    fn bitxor(self, other: &'a Natural) -> Natural {
        Limb::from(self) ^ other
    }
}

/// Bitwise-ors a `Natural` with a `Limb` in place.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::from(0xffff_ffffu32);
///     x ^= 0x0000_000f;
///     x ^= 0x0000_0f00;
///     x ^= 0x000f_0000;
///     x ^= 0x0f00_0000;
///     assert_eq!(x, 0xf0f0_f0f0u32);
/// }
/// ```
impl BitXorAssign<Limb> for Natural {
    fn bitxor_assign(&mut self, other: Limb) {
        match *self {
            Small(ref mut small) => *small ^= other,
            Large(ref mut limbs) => limbs_xor_limb_in_place(limbs, other),
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl BitXorAssign<u32> for Natural {
    #[inline]
    fn bitxor_assign(&mut self, other: u32) {
        *self ^= Limb::from(other);
    }
}
