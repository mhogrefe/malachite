use malachite_base::num::arithmetic::traits::{
    ArithmeticCheckedShl, NextPowerOfTwo, NextPowerOfTwoAssign,
};
use malachite_base::slices::{slice_set_zero, slice_test_zero};
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// limbs of the smallest integer power of 2 greater than or equal to the `Natural`.
///
/// This function assumes that `xs` is nonempty and the last (most significant) limb is nonzero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `xs` is empty.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::next_power_of_two::limbs_next_power_of_two;
///
/// assert_eq!(limbs_next_power_of_two(&[3]), &[4]);
/// assert_eq!(limbs_next_power_of_two(&[123, 456]), &[0, 512]);
/// assert_eq!(limbs_next_power_of_two(&[123, 456, u32::MAX]), &[0, 0, 0, 1]);
/// ```
pub fn limbs_next_power_of_two(xs: &[Limb]) -> Vec<Limb> {
    let xs_last = xs.last().unwrap();
    let mut out;
    if let Some(x) = xs_last.checked_next_power_of_two() {
        out = vec![0; xs.len() - 1];
        if x == *xs_last && !slice_test_zero(&xs[..xs.len() - 1]) {
            if let Some(x) = x.arithmetic_checked_shl(1) {
                out.push(x)
            } else {
                out.push(0);
                out.push(1);
            }
        } else {
            out.push(x);
        }
    } else {
        out = vec![0; xs.len()];
        out.push(1);
    }
    out
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the smallest integer power of 2 greater than or equal to the `Natural` to the input
/// slice. If the input slice is to small to hold the result, the limbs are all set to zero and the
/// carry bit, `true`, is returned. Otherwise, `false` is returned.
///
/// This function assumes that `xs` is nonempty and the last (most significant) limb is nonzero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `xs` is empty.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::next_power_of_two::*;
///
/// let mut xs = vec![3];
/// assert_eq!(limbs_slice_next_power_of_two_in_place(&mut xs), false);
/// assert_eq!(xs, &[4]);
///
/// let mut xs = vec![123, 456];
/// assert_eq!(limbs_slice_next_power_of_two_in_place(&mut xs), false);
/// assert_eq!(xs, &[0, 512]);
///
/// let mut xs = vec![123, 456, u32::MAX];
/// assert_eq!(limbs_slice_next_power_of_two_in_place(&mut xs), true);
/// assert_eq!(xs, &[0, 0, 0]);
/// ```
pub fn limbs_slice_next_power_of_two_in_place(xs: &mut [Limb]) -> bool {
    let (xs_last, xs_init) = xs.split_last_mut().unwrap();
    if let Some(x) = xs_last.checked_next_power_of_two() {
        if x == *xs_last && !slice_test_zero(xs_init) {
            slice_set_zero(xs_init);
            if let Some(x) = x.arithmetic_checked_shl(1) {
                *xs_last = x;
                false
            } else {
                *xs_last = 0;
                true
            }
        } else {
            slice_set_zero(xs_init);
            *xs_last = x;
            false
        }
    } else {
        slice_set_zero(xs_init);
        *xs_last = 0;
        true
    }
}

/// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the smallest integer power of 2 greater than or equal to the `Natural` to the input
/// `Vec`.
///
/// This function assumes that `xs` is nonempty and the last (most significant) limb is nonzero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `xs` is empty.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::next_power_of_two::limbs_vec_next_power_of_two_in_place;
///
/// let mut xs = vec![3];
/// limbs_vec_next_power_of_two_in_place(&mut xs);
/// assert_eq!(xs, &[4]);
///
/// let mut xs = vec![123, 456];
/// limbs_vec_next_power_of_two_in_place(&mut xs);
/// assert_eq!(xs, &[0, 512]);
///
/// let mut xs = vec![123, 456, u32::MAX];
/// limbs_vec_next_power_of_two_in_place(&mut xs);
/// assert_eq!(xs, &[0, 0, 0, 1]);
/// ```
pub fn limbs_vec_next_power_of_two_in_place(xs: &mut Vec<Limb>) {
    if limbs_slice_next_power_of_two_in_place(xs) {
        xs.push(1);
    }
}

impl NextPowerOfTwo for Natural {
    type Output = Natural;

    /// Returns the smallest integer power of 2 greater than or equal to a `Natural`, taking the
    /// `Natural` by value.
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
    /// use malachite_base::num::arithmetic::traits::NextPowerOfTwo;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.next_power_of_two().to_string(), "1");
    /// assert_eq!(Natural::from(123u32).next_power_of_two().to_string(), "128");
    /// assert_eq!(Natural::trillion().next_power_of_two().to_string(), "1099511627776");
    /// ```
    #[inline]
    fn next_power_of_two(mut self) -> Natural {
        self.next_power_of_two_assign();
        self
    }
}

impl<'a> NextPowerOfTwo for &'a Natural {
    type Output = Natural;

    /// Returns the smallest integer power of 2 greater than or equal to a `Natural`, taking the
    /// `Natural` by reference.
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
    /// use malachite_base::num::arithmetic::traits::NextPowerOfTwo;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::ZERO).next_power_of_two().to_string(), "1");
    /// assert_eq!((&Natural::from(123u32)).next_power_of_two().to_string(), "128");
    /// assert_eq!((&Natural::trillion()).next_power_of_two().to_string(), "1099511627776");
    /// ```
    fn next_power_of_two(self) -> Natural {
        Natural(match *self {
            Natural(Small(small)) => {
                if let Some(result) = small.checked_next_power_of_two() {
                    Small(result)
                } else {
                    Large(vec![0, 1])
                }
            }
            Natural(Large(ref limbs)) => Large(limbs_next_power_of_two(limbs)),
        })
    }
}

impl NextPowerOfTwoAssign for Natural {
    /// Replaces a `Natural` with the smallest integer power of two greater than or equal to it.
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
    /// use malachite_base::num::arithmetic::traits::NextPowerOfTwoAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::ZERO;
    /// x.next_power_of_two_assign();
    /// assert_eq!(x.to_string(), "1");
    ///
    /// let mut x = Natural::from(123u32);
    /// x.next_power_of_two_assign();
    /// assert_eq!(x.to_string(), "128");
    ///
    /// let mut x = Natural::trillion();
    /// x.next_power_of_two_assign();
    /// assert_eq!(x.to_string(), "1099511627776");
    /// ```
    fn next_power_of_two_assign(&mut self) {
        match *self {
            Natural(Small(ref mut small)) => {
                if let Some(pow) = small.checked_next_power_of_two() {
                    *small = pow;
                } else {
                    *self = Natural(Large(vec![0, 1]));
                }
            }
            Natural(Large(ref mut limbs)) => {
                limbs_vec_next_power_of_two_in_place(limbs);
            }
        }
    }
}
