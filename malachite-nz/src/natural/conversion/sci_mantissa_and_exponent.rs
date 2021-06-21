use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2Assign, Parity, PowerOf2, Sign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::{ExactFrom, FromOtherTypeSlice, WrappingFrom};
use malachite_base::num::float::PrimitiveFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::slices::{slice_set_zero, slice_test_zero};
use natural::arithmetic::shl::limbs_slice_shl_in_place;
use natural::arithmetic::shr::limbs_slice_shr_in_place;
use natural::logic::bit_access::limbs_get_bit;
use natural::logic::bit_scan::limbs_index_of_next_true_bit;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;
use std::cmp::Ordering;

impl Natural {
    pub fn sci_mantissa_and_exponent_with_rounding<T: PrimitiveFloat>(
        &self,
        rm: RoundingMode,
    ) -> Option<(T, u64)> {
        assert_ne!(*self, 0);
        // Worst case: 32-bit limbs, 64-bit float output, most-significant limb is 1. In this
        // case, the 3 most significant limbs are needed.
        let mut most_significant_limbs = [0; 3];
        let mut exponent = T::MANTISSA_WIDTH;
        let significant_bits;
        let mut exact = true;
        let mut half_compare = Ordering::Less; // (mantissa - floor(mantissa)).cmp(&0.5)
        let care_about_exactness = rm != RoundingMode::Floor && rm != RoundingMode::Down;
        let mut highest_discarded_limb = 0;
        match self {
            Natural(Small(x)) => {
                most_significant_limbs[0] = *x;
                significant_bits = x.significant_bits();
            }
            Natural(Large(ref xs)) => {
                let len = xs.len();
                if len == 2 {
                    most_significant_limbs[0] = xs[0];
                    most_significant_limbs[1] = xs[1];
                    significant_bits = xs[1].significant_bits() + Limb::WIDTH;
                } else {
                    most_significant_limbs[2] = xs[len - 1];
                    most_significant_limbs[1] = xs[len - 2];
                    most_significant_limbs[0] = xs[len - 3];
                    exponent += u64::exact_from(len - 3) << Limb::LOG_WIDTH;
                    if care_about_exactness && !slice_test_zero(&xs[..len - 3]) {
                        if rm == RoundingMode::Exact {
                            return None;
                        }
                        exact = false;
                        highest_discarded_limb = xs[len - 4];
                    }
                    significant_bits =
                        most_significant_limbs[2].significant_bits() + (Limb::WIDTH << 1);
                }
            }
        }
        let shift =
            i128::wrapping_from(T::MANTISSA_WIDTH + 1) - i128::wrapping_from(significant_bits);
        match shift.sign() {
            Ordering::Greater => {
                let mut shift = u64::exact_from(shift);
                exponent -= shift;
                let limbs_to_shift = shift >> Limb::LOG_WIDTH;
                if limbs_to_shift != 0 {
                    shift.mod_power_of_2_assign(Limb::LOG_WIDTH);
                    let limbs_to_shift = usize::wrapping_from(limbs_to_shift);
                    most_significant_limbs.copy_within(..3 - limbs_to_shift, limbs_to_shift);
                    slice_set_zero(&mut most_significant_limbs[..limbs_to_shift])
                }
                if shift != 0 {
                    limbs_slice_shl_in_place(&mut most_significant_limbs, shift);
                }
            }
            Ordering::Less => {
                let mut shift = u64::exact_from(-shift);
                if care_about_exactness {
                    let one_index =
                        limbs_index_of_next_true_bit(&most_significant_limbs, 0).unwrap();
                    if one_index < shift {
                        if rm == RoundingMode::Exact {
                            return None;
                        }
                        if rm == RoundingMode::Nearest {
                            // If `exact` is true here, that means all lower limbs are 0
                            half_compare = if exact && one_index == shift - 1 {
                                Ordering::Equal
                            } else if limbs_get_bit(&most_significant_limbs, shift - 1) {
                                Ordering::Greater
                            } else {
                                Ordering::Less
                            };
                        }
                        exact = false;
                    }
                }
                exponent += shift;
                let limbs_to_shift = shift >> Limb::LOG_WIDTH;
                if limbs_to_shift != 0 {
                    shift.mod_power_of_2_assign(Limb::LOG_WIDTH);
                    most_significant_limbs.copy_within(usize::wrapping_from(limbs_to_shift).., 0);
                }
                if shift != 0 {
                    limbs_slice_shr_in_place(&mut most_significant_limbs, shift);
                }
            }
            Ordering::Equal => {
                if !exact && rm == RoundingMode::Nearest {
                    // len is at least 4, since the only way `exact` is false at this point is if
                    // xs[..len - 3] is nonzero
                    half_compare = highest_discarded_limb.cmp(&Limb::power_of_2(Limb::WIDTH - 1));
                }
            }
        }
        let raw_mantissa =
            u64::from_other_type_slice(&most_significant_limbs).mod_power_of_2(T::MANTISSA_WIDTH);
        let mantissa =
            T::from_raw_mantissa_and_exponent(raw_mantissa, u64::wrapping_from(T::MAX_EXPONENT));
        let increment = !exact
            && (rm == RoundingMode::Up
                || rm == RoundingMode::Ceiling
                || rm == RoundingMode::Nearest
                    && (half_compare == Ordering::Greater
                        || half_compare == Ordering::Equal && raw_mantissa.odd()));
        if increment {
            let next_mantissa = mantissa.next_higher();
            if next_mantissa == T::TWO {
                Some((T::ONE, exponent + 1))
            } else {
                Some((next_mantissa, exponent))
            }
        } else {
            Some((mantissa, exponent))
        }
    }

    #[inline]
    pub fn sci_mantissa_and_exponent<T: PrimitiveFloat>(&self) -> (T, u64) {
        self.sci_mantissa_and_exponent_with_rounding(RoundingMode::Nearest)
            .unwrap()
    }
}
