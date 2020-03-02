use std::ops::Not;

use malachite_base::num::logic::traits::NotAssign;

use integer::Integer;
use natural::Natural;
use platform::Limb;

/// Returns the bitwise not of a slice of limbs.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `limbs.len()`
///
/// This is mpn_com from mpn/generic/com.c, GMP 6.1.2, where rp is returned.
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::not::limbs_not;
/// use std::cmp::Ordering;
///
/// assert_eq!(limbs_not(&[0, 1, 2]), [0xffffffff, 0xfffffffe, 0xfffffffd]);
/// ```
pub fn limbs_not(limbs: &[Limb]) -> Vec<Limb> {
    limbs.iter().map(|limb| !limb).collect()
}

/// Writes the bitwise not of a slice of limbs to the lowest `in_limbs.len()` limbs of `out`.
/// For this to work, `out` must be at least as long as `in_limbs`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `in_limbs.len()`
///
/// This is mpn_com from mpn/generic/com.c, GMP 6.1.2, where rp != up.
///
/// # Panics
/// Panics if `out` is shorter than `in_limbs`.
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::not::limbs_not_to_out;
///
/// let mut out = [0, 1, 2];
/// limbs_not_to_out(&mut out, &[0xffff0000, 0xf0f0f0f0]);
/// assert_eq!(out, [0x0000ffff, 0x0f0f0f0f, 2]);
/// ```
pub fn limbs_not_to_out(out: &mut [Limb], in_limbs: &[Limb]) {
    assert!(out.len() >= in_limbs.len());
    for (x, y) in out.iter_mut().zip(in_limbs.iter()) {
        *x = !y;
    }
}

/// Takes the bitwise not of a slice of limbs in place.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// This is mpn_com from mpn/generic/com.c, GMP 6.1.2, where rp == up.
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::not::limbs_not_in_place;
/// use std::cmp::Ordering;
///
/// let mut limbs = [0, 1, 2];
/// limbs_not_in_place(&mut limbs);
/// assert_eq!(limbs, [0xffffffff, 0xfffffffe, 0xfffffffd]);
/// ```
pub fn limbs_not_in_place(limbs: &mut [Limb]) {
    for limb in limbs.iter_mut() {
        limb.not_assign();
    }
}

/// Returns the bitwise complement of a `Natural`, as if it were represented in two's complement,
/// taking the `Natural` by value and returning an `Integer`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!((!Natural::ZERO).to_string(), "-1");
/// assert_eq!((!Natural::from(123u32)).to_string(), "-124");
/// ```
impl Not for Natural {
    type Output = Integer;

    fn not(self) -> Integer {
        Integer {
            sign: false,
            abs: self.add_limb(1),
        }
    }
}

/// Returns the bitwise complement of a `Natural`, as if it were represented in two's complement,
/// taking the `Natural` by reference and returning an `Integer`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!((!&Natural::ZERO).to_string(), "-1");
/// assert_eq!((!&Natural::from(123u32)).to_string(), "-124");
/// ```
impl<'a> Not for &'a Natural {
    type Output = Integer;

    fn not(self) -> Integer {
        Integer {
            sign: false,
            abs: self.add_limb_ref(1),
        }
    }
}
