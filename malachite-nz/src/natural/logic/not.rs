use integer::Integer;
use malachite_base::num::logic::traits::NotAssign;
use natural::Natural;
use platform::Limb;
use std::ops::Not;

/// Returns the bitwise not of a slice of limbs.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `xs.len()`
///
/// This is mpn_com from mpn/generic/com.c, GMP 6.1.2, where rp is returned.
///
/// # Examples
/// ```
/// use malachite_nz::natural::logic::not::limbs_not;
/// use std::cmp::Ordering;
///
/// assert_eq!(limbs_not(&[0, 1, 2]), [0xffffffff, 0xfffffffe, 0xfffffffd]);
/// ```
pub fn limbs_not(xs: &[Limb]) -> Vec<Limb> {
    xs.iter().map(|x| !x).collect()
}

/// Writes the bitwise not of a slice of limbs to the lowest `x.len()` limbs of `out`. For this to
/// work, `out` must be at least as long as `xs`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// This is mpn_com from mpn/generic/com.c, GMP 6.1.2, where rp != up.
///
/// # Panics
/// Panics if `out` is shorter than `xs`.
///
/// # Examples
/// ```
/// use malachite_nz::natural::logic::not::limbs_not_to_out;
///
/// let mut out = [0, 1, 2];
/// limbs_not_to_out(&mut out, &[0xffff0000, 0xf0f0f0f0]);
/// assert_eq!(out, [0x0000ffff, 0x0f0f0f0f, 2]);
/// ```
pub fn limbs_not_to_out(out: &mut [Limb], xs: &[Limb]) {
    assert!(out.len() >= xs.len());
    for (x, y) in out.iter_mut().zip(xs.iter()) {
        *x = !y;
    }
}

/// Takes the bitwise not of a slice of limbs in place.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// This is mpn_com from mpn/generic/com.c, GMP 6.1.2, where rp == up.
///
/// # Examples
/// ```
/// use malachite_nz::natural::logic::not::limbs_not_in_place;
/// use std::cmp::Ordering;
///
/// let mut limbs = [0, 1, 2];
/// limbs_not_in_place(&mut limbs);
/// assert_eq!(limbs, [0xffffffff, 0xfffffffe, 0xfffffffd]);
/// ```
pub fn limbs_not_in_place(xs: &mut [Limb]) {
    for x in xs.iter_mut() {
        x.not_assign();
    }
}

impl Not for Natural {
    type Output = Integer;

    /// Returns the bitwise complement of a `Natural`, as if it were represented in two's
    /// complement, taking the `Natural` by value and returning an `Integer`.
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
    fn not(self) -> Integer {
        Integer {
            sign: false,
            abs: self.add_limb(1),
        }
    }
}

impl<'a> Not for &'a Natural {
    type Output = Integer;

    /// Returns the bitwise complement of a `Natural`, as if it were represented in two's
    /// complement, taking the `Natural` by reference and returning an `Integer`.
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
    fn not(self) -> Integer {
        Integer {
            sign: false,
            abs: self.add_limb_ref(1),
        }
    }
}
