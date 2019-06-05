use std::cmp::Ordering;
use std::fmt::Display;

use malachite_base::num::arithmetic::traits::{CheckedSubMul, SubMul, SubMulAssign};

use natural::arithmetic::mul::limbs_mul;
use natural::arithmetic::sub::limbs_sub_in_place_left;
use natural::comparison::ord::limbs_cmp;
use natural::Natural;
use platform::Limb;

/// Given the limbs `xs`, `ys` and `zs` of three `Natural`s a, b, and c, returns the limbs of
/// a - b * c. If a < b * c, `None` is returned. `ys` and `zs` should have length at least 2, and
/// the length of `xs` should be at least `ys.len()` + `zs.len()` - 1 (if the latter condition is
/// false, the result would be `None` and there's no point in calling this function). None of the
/// slices should have any trailing zeros. The result, if it exists, will have no trailing zeros.
///
/// Time: O(n * log(n) * log(log(n)))
///
/// Additional memory: O(n * log(n))
///
/// where n = max(`ys.len()`, `zs.len()`)
///
/// # Panics
/// Panics if `ys` or `zs` have fewer than two elements each, or if `xs.len()` < `ys.len()` +
/// `zs.len()` - 1.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::sub_mul::limbs_sub_mul;
///
/// assert_eq!(limbs_sub_mul(&[123, 456, 789], &[123, 789], &[321, 654]), None);
/// assert_eq!(limbs_sub_mul(&[123, 456, 789, 1], &[123, 789], &[321, 654]),
///         Some(vec![4294927936, 4294634040, 4294452078, 0]));
/// ```
///
/// This is mpz_aorsmul from mpz/aorsmul.c, where w, x, and y are positive, sub is negative,
/// negative results are converted to `None`, and w is returned instead of overwriting the first
/// input.
pub fn limbs_sub_mul(xs: &[Limb], ys: &[Limb], zs: &[Limb]) -> Option<Vec<Limb>> {
    let mut xs = xs.to_vec();
    if limbs_sub_mul_in_place_left(&mut xs, ys, zs) {
        None
    } else {
        Some(xs)
    }
}

/// Given the limbs `xs`, `ys` and `zs` of three `Natural`s a, b, and c, computes a - b * c. The
/// limbs of the result are written to `xs`. Returns whether a borrow (overflow) occurred: if a <
/// b * c, `true` is returned and the value of `xs` should be ignored. `ys` and `zs` should have
/// length at least 2, and the length of `xs` should be at least `ys.len()` + `zs.len()` - 1 (if the
/// latter condition is false, the result would be negative and there would be no point in calling
/// this function). None of the slices should have any trailing zeros. The result, if it exists,
/// will have no trailing zeros.
///
/// Time: O(n * log(n) * log(log(n)))
///
/// Additional memory: O(n * log(n))
///
/// where n = max(`ys.len()`, `zs.len()`)
///
/// # Panics
/// Panics if `ys` or `zs` have fewer than two elements each, or if `xs.len()` < `ys.len()` +
/// `zs.len()` - 1.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::sub_mul::limbs_sub_mul_in_place_left;
///
/// let mut xs = vec![123, 456, 789];
/// assert_eq!(limbs_sub_mul_in_place_left(&mut xs, &[123, 789], &[321, 654]), true);
///
/// let mut xs = vec![123, 456, 789, 1];
/// assert_eq!(limbs_sub_mul_in_place_left(&mut xs, &[123, 789], &[321, 654]), false);
/// assert_eq!(xs, &[4294927936, 4294634040, 4294452078, 0]);
/// ```
///
/// This is mpz_aorsmul from mpz/aorsmul.c, where w, x, and y are positive, sub is negative and
/// negative results are discarded.
pub fn limbs_sub_mul_in_place_left(xs: &mut [Limb], ys: &[Limb], zs: &[Limb]) -> bool {
    assert!(ys.len() > 1);
    assert!(zs.len() > 1);
    let mut scratch = limbs_mul(ys, zs);
    assert!(xs.len() >= scratch.len() - 1);
    if *scratch.last().unwrap() == 0 {
        scratch.pop();
    }
    let borrow = limbs_cmp(&xs, &scratch) == Ordering::Less;
    if !borrow {
        assert!(!limbs_sub_in_place_left(xs, &scratch));
    }
    borrow
}

pub(crate) fn sub_mul_panic<S: Display, T: Display, U: Display>(a: S, b: T, c: U) -> ! {
    panic!("Cannot perform sub_mul. a: {}, b: {}, c: {}", a, b, c);
}

impl SubMul<Natural, Natural> for Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), taking
    /// `self`, b, and c by value.
    ///
    /// Time: O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: O(n * log(n))
    ///
    /// where n = max(`b.significant_bits()`, `zs.significant_bits()`)
    ///
    /// # Panics
    /// Panics if `b * c` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::SubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(
    ///         Natural::from(20u32).sub_mul(Natural::from(3u32), Natural::from(4u32))
    ///         .to_string(), "8");
    ///     assert_eq!(Natural::trillion().sub_mul(
    ///         Natural::from(0x1_0000u32), Natural::from(0x1_0000u32)).to_string(),
    ///         "995705032704");
    /// }
    /// ```
    fn sub_mul(self, b: Natural, c: Natural) -> Natural {
        self.checked_sub_mul(b, c)
            .expect("Natural sub_mul_assign cannot have a negative result")
    }
}

impl<'a> SubMul<Natural, &'a Natural> for Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), taking
    /// `self` and b by value and c by reference.
    ///
    /// Time: O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: O(n * log(n))
    ///
    /// where n = max(`b.significant_bits()`, `zs.significant_bits()`)
    ///
    /// # Panics
    /// Panics if `b * c` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::SubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(
    ///         Natural::from(20u32).sub_mul(Natural::from(3u32), &Natural::from(4u32))
    ///         .to_string(), "8");
    ///     assert_eq!(Natural::trillion().sub_mul(
    ///         Natural::from(0x1_0000u32), &Natural::from(0x1_0000u32)).to_string(),
    ///         "995705032704");
    /// }
    /// ```
    fn sub_mul(self, b: Natural, c: &'a Natural) -> Natural {
        self.checked_sub_mul(b, c)
            .expect("Natural sub_mul_assign cannot have a negative result")
    }
}

impl<'a> SubMul<&'a Natural, Natural> for Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), taking
    /// `self` and c value and b by reference.
    ///
    /// Time: O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: O(n * log(n))
    ///
    /// where n = max(`b.significant_bits()`, `zs.significant_bits()`)
    ///
    /// # Panics
    /// Panics if `b * c` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::SubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(
    ///         Natural::from(20u32).sub_mul(&Natural::from(3u32), Natural::from(4u32))
    ///         .to_string(), "8");
    ///     assert_eq!(Natural::trillion().sub_mul(
    ///         &Natural::from(0x1_0000u32), Natural::from(0x1_0000u32)).to_string(),
    ///         "995705032704");
    /// }
    /// ```
    fn sub_mul(self, b: &'a Natural, c: Natural) -> Natural {
        self.checked_sub_mul(b, c)
            .expect("Natural sub_mul_assign cannot have a negative result")
    }
}

impl<'a, 'b> SubMul<&'a Natural, &'b Natural> for Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), taking
    /// `self` by value and b and c by reference.
    ///
    /// Time: O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: O(n * log(n))
    ///
    /// where n = max(`b.significant_bits()`, `zs.significant_bits()`)
    ///
    /// # Panics
    /// Panics if `b * c` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::SubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(
    ///         Natural::from(20u32).sub_mul(&Natural::from(3u32), &Natural::from(4u32))
    ///         .to_string(), "8");
    ///     assert_eq!(Natural::trillion().sub_mul(
    ///         &Natural::from(0x1_0000u32), &Natural::from(0x1_0000u32)).to_string(),
    ///         "995705032704");
    /// }
    /// ```
    fn sub_mul(self, b: &'a Natural, c: &'b Natural) -> Natural {
        self.checked_sub_mul(b, c)
            .expect("Natural sub_mul_assign cannot have a negative result")
    }
}

impl<'a, 'b, 'c> SubMul<&'a Natural, &'b Natural> for &'c Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), taking
    /// `self`, b, and c by reference.
    ///
    /// Time: O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: O(n * log(n))
    ///
    /// where n = max(`b.significant_bits()`, `zs.significant_bits()`)
    ///
    /// # Panics
    /// Panics if `b * c` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::SubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(
    ///         (&Natural::from(20u32)).sub_mul(&Natural::from(3u32), &Natural::from(4u32))
    ///         .to_string(), "8");
    ///     assert_eq!((&Natural::trillion()).sub_mul(
    ///         &Natural::from(0x1_0000u32), &Natural::from(0x1_0000u32)).to_string(),
    ///         "995705032704");
    /// }
    /// ```
    fn sub_mul(self, b: &'a Natural, c: &'b Natural) -> Natural {
        self.checked_sub_mul(b, c).unwrap_or_else(|| {
            sub_mul_panic(self, b, c);
        })
    }
}

impl SubMulAssign<Natural, Natural> for Natural {
    /// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), in
    /// place, taking b and c by value.
    ///
    /// Time: O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: O(n * log(n))
    ///
    /// where n = max(`b.significant_bits()`, `zs.significant_bits()`)
    ///
    /// # Panics
    /// Panics if `b * c` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::SubMulAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     let mut x = Natural::from(20u32);
    ///     x.sub_mul_assign(Natural::from(3u32), Natural::from(4u32));
    ///     assert_eq!(x, 8);
    ///
    ///     let mut x = Natural::trillion();
    ///     x.sub_mul_assign(Natural::from(0x1_0000u32), Natural::from(0x1_0000u32));
    ///     assert_eq!(x.to_string(), "995705032704");
    /// }
    /// ```
    fn sub_mul_assign(&mut self, b: Natural, c: Natural) {
        if self.sub_mul_assign_no_panic(b, c) {
            panic!("Natural sub_mul_assign cannot have a negative result");
        }
    }
}

impl<'a> SubMulAssign<Natural, &'a Natural> for Natural {
    /// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), in
    /// place, taking b by value and c by reference.
    ///
    /// Time: O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: O(n * log(n))
    ///
    /// where n = max(`b.significant_bits()`, `zs.significant_bits()`)
    ///
    /// # Panics
    /// Panics if `b * c` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::SubMulAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     let mut x = Natural::from(20u32);
    ///     x.sub_mul_assign(Natural::from(3u32), &Natural::from(4u32));
    ///     assert_eq!(x, 8);
    ///
    ///     let mut x = Natural::trillion();
    ///     x.sub_mul_assign(Natural::from(0x1_0000u32), &Natural::from(0x1_0000u32));
    ///     assert_eq!(x.to_string(), "995705032704");
    /// }
    /// ```
    fn sub_mul_assign(&mut self, b: Natural, c: &'a Natural) {
        if self.sub_mul_assign_val_ref_no_panic(b, c) {
            panic!("Natural sub_mul_assign cannot have a negative result");
        }
    }
}

impl<'a> SubMulAssign<&'a Natural, Natural> for Natural {
    /// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), in
    /// place, taking b by reference and c by value.
    ///
    /// Time: O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: O(n * log(n))
    ///
    /// where n = max(`b.significant_bits()`, `zs.significant_bits()`)
    ///
    /// # Panics
    /// Panics if `b * c` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::SubMulAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     let mut x = Natural::from(20u32);
    ///     x.sub_mul_assign(&Natural::from(3u32), Natural::from(4u32));
    ///     assert_eq!(x, 8);
    ///
    ///     let mut x = Natural::trillion();
    ///     x.sub_mul_assign(&Natural::from(0x1_0000u32), Natural::from(0x1_0000u32));
    ///     assert_eq!(x.to_string(), "995705032704");
    /// }
    /// ```
    fn sub_mul_assign(&mut self, b: &'a Natural, c: Natural) {
        if self.sub_mul_assign_ref_val_no_panic(b, c) {
            panic!("Natural sub_mul_assign cannot have a negative result");
        }
    }
}

impl<'a, 'b> SubMulAssign<&'a Natural, &'b Natural> for Natural {
    /// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), in
    /// place, taking b and c by reference.
    ///
    /// Time: O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: O(n * log(n))
    ///
    /// where n = max(`b.significant_bits()`, `zs.significant_bits()`)
    ///
    /// # Panics
    /// Panics if `b * c` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::SubMulAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     let mut x = Natural::from(20u32);
    ///     x.sub_mul_assign(&Natural::from(3u32), &Natural::from(4u32));
    ///     assert_eq!(x, 8);
    ///
    ///     let mut x = Natural::trillion();
    ///     x.sub_mul_assign(&Natural::from(0x1_0000u32), &Natural::from(0x1_0000u32));
    ///     assert_eq!(x.to_string(), "995705032704");
    /// }
    /// ```
    fn sub_mul_assign(&mut self, b: &'a Natural, c: &'b Natural) {
        if self.sub_mul_assign_ref_ref_no_panic(b, c) {
            panic!("Natural sub_mul_assign cannot have a negative result");
        }
    }
}
