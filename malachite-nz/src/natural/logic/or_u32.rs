use natural::Natural::{self, Large, Small};
use std::ops::{BitOr, BitOrAssign};

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of a `Natural`, returns the
/// limbs of the bitwise or of the `Natural` and a `u32`. `limbs` cannot be empty.
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
/// use malachite_nz::natural::logic::or_u32::limbs_or_limb;
///
/// assert_eq!(limbs_or_limb(&[123, 456], 789), &[895, 456]);
/// ```
pub fn limbs_or_limb(limbs: &[u32], limb: u32) -> Vec<u32> {
    let mut result = limbs.to_vec();
    limbs_or_limb_in_place(&mut result, limb);
    result
}

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the bitwise or of the `Natural` and a `u32` to an output slice. The output slice must
/// be at least as long as the input slice. `in_limbs` cannot be empty.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `out_limbs` is shorter than `in_limbs` or if `in_limbs` is empty.
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::or_u32::limbs_or_limb_to_out;
///
/// let mut out_limbs = vec![0, 0, 0];
/// limbs_or_limb_to_out(&mut out_limbs, &[123, 456], 789);
/// assert_eq!(out_limbs, &[895, 456, 0]);
/// ```
pub fn limbs_or_limb_to_out(out_limbs: &mut [u32], in_limbs: &[u32], limb: u32) {
    out_limbs[0..in_limbs.len()].copy_from_slice(in_limbs);
    limbs_or_limb_in_place(out_limbs, limb);
}

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the bitwise or of the `Natural` and a `u32` to the input slice. `limbs` cannot be
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
/// use malachite_nz::natural::logic::or_u32::limbs_or_limb_in_place;
///
/// let mut limbs = vec![123, 456];
/// limbs_or_limb_in_place(&mut limbs, 789);
/// assert_eq!(limbs, &[895, 456]);
/// ```
pub fn limbs_or_limb_in_place(limbs: &mut [u32], limb: u32) {
    limbs[0] |= limb;
}

/// Takes the bitwise or of a `Natural` and a `u32`, taking the `Natural` by value.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::Zero;
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((Natural::ZERO | 123).to_string(), "123");
///     assert_eq!((Natural::from(123u32) | 0).to_string(), "123");
///     assert_eq!((Natural::from_str("12345678987654321").unwrap() | 456).to_string(),
///         "12345678987654649");
/// }
/// ```
impl BitOr<u32> for Natural {
    type Output = Natural;

    fn bitor(mut self, other: u32) -> Natural {
        self |= other;
        self
    }
}

/// Takes the bitwise or of a `Natural` and an `u32`, taking the `Natural` by reference.
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
/// use malachite_base::num::Zero;
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((&Natural::ZERO | 123).to_string(), "123");
///     assert_eq!((&Natural::from(123u32) | 0).to_string(), "123");
///     assert_eq!((&Natural::from_str("12345678987654321").unwrap() | 456).to_string(),
///         "12345678987654649");
/// }
/// ```
impl<'a> BitOr<u32> for &'a Natural {
    type Output = Natural;

    fn bitor(self, other: u32) -> Natural {
        match *self {
            Small(small) => Small(small | other),
            Large(ref limbs) => Large(limbs_or_limb(limbs, other)),
        }
    }
}

/// Takes the bitwise or of a `u32` and a `Natural`, taking the `Natural` by value.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::Zero;
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((123 | Natural::ZERO).to_string(), "123");
///     assert_eq!((0 | Natural::from(123u32)).to_string(), "123");
///     assert_eq!((456 | Natural::from_str("12345678987654321").unwrap()).to_string(),
///         "12345678987654649");
/// }
/// ```
impl BitOr<Natural> for u32 {
    type Output = Natural;

    fn bitor(self, other: Natural) -> Natural {
        other | self
    }
}

/// Takes the bitwise and of a `u32` and a `Natural`, taking the `Natural` by reference.
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
/// use malachite_base::num::Zero;
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((123 | &Natural::ZERO).to_string(), "123");
///     assert_eq!((0 | &Natural::from(123u32)).to_string(), "123");
///     assert_eq!((456 | &Natural::from_str("12345678987654321").unwrap()).to_string(),
///         "12345678987654649");
/// }
/// ```
impl<'a> BitOr<&'a Natural> for u32 {
    type Output = Natural;

    fn bitor(self, other: &'a Natural) -> Natural {
        other | self
    }
}

/// Bitwise-ors a `Natural` with a `u32` in place.
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
/// use malachite_base::num::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::ZERO;
///     x |= 0x0000_000f;
///     x |= 0x0000_0f00;
///     x |= 0x000f_0000;
///     x |= 0x0f00_0000;
///     assert_eq!(x, 0x0f0f_0f0f);
/// }
/// ```
impl BitOrAssign<u32> for Natural {
    fn bitor_assign(&mut self, other: u32) {
        match *self {
            Small(ref mut small) => *small |= other,
            Large(ref mut limbs) => limbs_or_limb_in_place(limbs, other),
        }
    }
}
