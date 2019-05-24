use malachite_base::num::traits::{DivisibleBy, DivisibleByPowerOfTwo, Parity};
use natural::arithmetic::eq_limb_mod_limb::limbs_mod_exact_odd_limb;
use natural::arithmetic::mod_limb::limbs_mod_limb;
use natural::Natural::{self, Large, Small};
use platform::Limb;

// must be >= 1
//TODO tune
const BMOD_1_TO_MOD_1_THRESHOLD: usize = 29;

/// Benchmarks show that this is never faster than just calling `limbs_divisible_by_limb`.
///
/// limbs.len() must be greater than 1; divisor must be nonzero.
///
/// This is mpz_divisible_ui_p from mpz/divis_ui.c, where a is non-negative.
pub fn _combined_limbs_divisible_by_limb(limbs: &[Limb], divisor: Limb) -> bool {
    if limbs.len() <= BMOD_1_TO_MOD_1_THRESHOLD {
        limbs_divisible_by_limb(limbs, divisor)
    } else {
        limbs_mod_limb(limbs, divisor) == 0
    }
}

/// Interpreting a slice of `Limb`s as the limbs of a `Natural` in ascending order, determines
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
/// use malachite_nz::natural::arithmetic::divisible_by_limb::limbs_divisible_by_limb;
///
/// assert_eq!(limbs_divisible_by_limb(&[333, 333], 3), true);
/// assert_eq!(limbs_divisible_by_limb(&[332, 333], 3), false);
/// ```
///
/// This is mpz_divisible_ui_p from mpz/divis_ui.c, where a is non-negative and the ABOVE_THRESHOLD
/// branch is excluded.
pub fn limbs_divisible_by_limb(limbs: &[Limb], divisor: Limb) -> bool {
    assert!(limbs.len() > 1);
    if divisor.even() {
        let twos = divisor.trailing_zeros();
        limbs[0].divisible_by_power_of_two(twos.into())
            && limbs_mod_exact_odd_limb(limbs, divisor >> twos, 0) == 0
    } else {
        limbs_mod_exact_odd_limb(limbs, divisor, 0) == 0
    }
}

impl<'a> DivisibleBy<Limb> for &'a Natural {
    /// Returns whether a `Natural` is divisible by a `Limb`; in other words, whether the `Natural`
    /// is a multiple of the `Limb`. This means that zero is divisible by any number, including
    /// zero; but a nonzero number is never divisible by zero.
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
    /// use malachite_base::num::traits::{DivisibleBy, Zero};
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::ZERO.divisible_by(0), true);
    ///     assert_eq!(Natural::from(100u32).divisible_by(3), false);
    ///     assert_eq!(Natural::from(102u32).divisible_by(3), true);
    /// }
    /// ```
    fn divisible_by(self, other: Limb) -> bool {
        match (self, other) {
            (&Small(0), _) => true,
            (_, 0) => false,
            (&Small(small), y) => small.divisible_by(y),
            (&Large(ref limbs), y) => limbs_divisible_by_limb(limbs, y),
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> DivisibleBy<u32> for &'a Natural {
    #[inline]
    fn divisible_by(self, other: u32) -> bool {
        self.divisible_by(Limb::from(other))
    }
}

impl<'a> DivisibleBy<&'a Natural> for Limb {
    /// Returns whether a `Limb` is divisible by a `Natural`; in other words, whether the `Limb` is
    /// a multiple of the `Natural`. This means that zero is divisible by any number, including
    /// zero; but a nonzero number is never divisible by zero.
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
    /// use malachite_base::num::traits::{DivisibleBy, Zero};
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(0.divisible_by(&Natural::ZERO), true);
    ///     assert_eq!(100.divisible_by(&Natural::from(3u32)), false);
    ///     assert_eq!(102.divisible_by(&Natural::from(3u32)), true);
    /// }
    /// ```
    fn divisible_by(self, other: &'a Natural) -> bool {
        match (self, other) {
            (0, _) => true,
            (_, Small(0)) => false,
            (x, &Small(small)) => x.divisible_by(small),
            (_, &Large(_)) => false,
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> DivisibleBy<&'a Natural> for u32 {
    #[inline]
    fn divisible_by(self, other: &'a Natural) -> bool {
        Limb::from(self).divisible_by(other)
    }
}
