use malachite_base::num::arithmetic::traits::{ModPowerOf2Assign, ShrRound};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::LowMask;
use malachite_base::rounding_modes::RoundingMode;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

/// Returns the limbs of a `Natural`, where the lowest `bits` bits are set.
///
/// Time: worst case O(`bits`)
///
/// Additional memory: worst case O(`bits`)
#[doc(hidden)]
pub fn limbs_low_mask(bits: u64) -> Vec<Limb> {
    let len = bits.shr_round(Limb::LOG_WIDTH, RoundingMode::Ceiling);
    let remaining_bits = bits & Limb::WIDTH_MASK;
    let mut xs = vec![Limb::MAX; usize::exact_from(len)];
    if remaining_bits != 0 {
        xs.last_mut().unwrap().mod_power_of_2_assign(remaining_bits);
    }
    xs
}

impl LowMask for Natural {
    /// Returns a `Natural` with the least significant `bits` bits on and the remaining bits off.
    ///
    /// Time: worst case O(`bits`)
    ///
    /// Additional memory: worst case O(`bits`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::logic::traits::LowMask;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::low_mask(0).to_string(), "0");
    /// assert_eq!(Natural::low_mask(3).to_string(), "7");
    /// assert_eq!(Natural::low_mask(100).to_string(), "1267650600228229401496703205375");
    /// ```
    fn low_mask(bits: u64) -> Natural {
        if bits <= Limb::WIDTH {
            Natural(Small(Limb::low_mask(bits)))
        } else {
            Natural(Large(limbs_low_mask(bits)))
        }
    }
}
