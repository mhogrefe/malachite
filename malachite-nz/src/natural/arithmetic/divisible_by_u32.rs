use malachite_base::num::{DivisibleBy, DivisibleByPowerOfTwo, Parity, SplitInHalf};
use natural::arithmetic::div_exact_u32::limbs_invert_limb;
use natural::arithmetic::mod_u32::limbs_mod_limb;
use natural::Natural::{self, Large, Small};

/// Interpreting a slice of `u32`s as the limbs of a `Natural` in ascending order, determines
/// whether that `Natural` is divisible by a given limb.
///
/// This function assumes that `limbs` has at least two elements and that `limb` is nonzero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::divisible_by_u32::limbs_divisible_by_limb;
///
/// assert_eq!(limbs_divisible_by_limb(&[333, 333], 3), true);
/// assert_eq!(limbs_divisible_by_limb(&[332, 333], 3), false);
/// ```
pub fn limbs_divisible_by_limb(limbs: &[u32], divisor: u32) -> bool {
    assert!(limbs.len() > 1);
    if divisor.is_even() {
        let twos = divisor.trailing_zeros();
        limbs[0].divisible_by_power_of_two(twos.into())
            && limbs_mod_exact_odd_limb(limbs, divisor >> twos) == 0
    } else {
        limbs_mod_exact_odd_limb(limbs, divisor) == 0
    }
}

// must be >= 1
const BMOD_1_TO_MOD_1_THRESHOLD: usize = 10;

// Benchmarks show that this is never faster than just calling `limbs_divisible_by_limb`.
//
// limbs.len() must be greater than 1; d must be nonzero.
pub fn _combined_limbs_divisible_by_limb(a: &[u32], d: u32) -> bool {
    if a.len() <= BMOD_1_TO_MOD_1_THRESHOLD {
        limbs_divisible_by_limb(a, d)
    } else {
        limbs_mod_limb(a, d) == 0
    }
}

// limbs.len() must be greater than 1; divisor must be odd.
fn limbs_mod_exact_odd_limb(limbs: &[u32], divisor: u32) -> u32 {
    let len = limbs.len();
    let inverse = limbs_invert_limb(divisor);
    let divisor_u64 = u64::from(divisor);
    let last_index = len - 1;
    let mut carry = 0;
    for &limb in &limbs[..last_index] {
        let (l, small_carry) = limb.overflowing_sub(carry);
        carry = (u64::from(l.wrapping_mul(inverse)) * divisor_u64).upper_half();
        if small_carry {
            carry = carry.wrapping_add(1);
        }
    }
    let last = limbs[last_index];
    if last <= divisor {
        if carry >= last {
            carry - last
        } else {
            carry.wrapping_add(divisor - last)
        }
    } else {
        let (l, small_carry) = last.overflowing_sub(carry);
        carry = (u64::from(l.wrapping_mul(inverse)) * divisor_u64).upper_half();
        if small_carry {
            carry = carry.wrapping_add(1);
        }
        carry
    }
}

impl DivisibleBy<u32> for Natural {
    /// Returns whether a `Natural` is divisible by a `u32`; in other worst, whether the `Natural`
    /// is a multiple of the `u32`. THis means that zero is divisible by any number, including zero;
    /// but a nonzero number is never divisible by zero.
    ///
    /// This method is more efficient than finding a remainder and checking whether it's zero.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::{DivisibleBy, Zero};
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::ZERO.divisible_by(&0), true);
    ///     assert_eq!(Natural::from(100u32).divisible_by(&3), false);
    ///     assert_eq!(Natural::from(102u32).divisible_by(&3), true);
    /// }
    /// ```
    fn divisible_by(&self, other: &u32) -> bool {
        match (self, other) {
            (&Small(0), _) => true,
            (_, 0) => false,
            (&Small(small), y) => small.divisible_by(y),
            (&Large(ref limbs), &y) => limbs_divisible_by_limb(limbs, y),
        }
    }
}

impl DivisibleBy<Natural> for u32 {
    /// Returns whether a `u32` is divisible by a `Natural`; in other worst, whether the `u32` is a
    /// multiple of the `Natural`. THis means that zero is divisible by any number, including zero;
    /// but a nonzero number is never divisible by zero.
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
    /// use malachite_base::num::{DivisibleBy, Zero};
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(0.divisible_by(&Natural::ZERO), true);
    ///     assert_eq!(100.divisible_by(&Natural::from(3u32)), false);
    ///     assert_eq!(102.divisible_by(&Natural::from(3u32)), true);
    /// }
    /// ```
    fn divisible_by(&self, other: &Natural) -> bool {
        match (self, other) {
            (&0, _) => true,
            (_, Small(0)) => false,
            (&x, &Small(ref small)) => x.divisible_by(small),
            (_, &Large(_)) => false,
        }
    }
}
