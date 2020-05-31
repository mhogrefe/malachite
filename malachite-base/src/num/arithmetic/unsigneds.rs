use num::arithmetic::traits::{
    CeilingLogTwo, CheckedLogTwo, CheckedNextPowerOfTwo, DivRound, FloorLogTwo, IsPowerOfTwo, Mod,
    NegMod, NegModAssign, NextPowerOfTwo, NextPowerOfTwoAssign, Parity,
};
use num::basic::integers::PrimitiveInteger;
use num::basic::unsigneds::PrimitiveUnsigned;
use num::logic::traits::{LeadingZeros, SignificantBits, TrailingZeros};
use round::RoundingMode;

macro_rules! impl_arithmetic_traits {
    ($t:ident) => {
        impl IsPowerOfTwo for $t {
            #[inline]
            fn is_power_of_two(&self) -> bool {
                $t::is_power_of_two(*self)
            }
        }

        impl NextPowerOfTwo for $t {
            type Output = $t;

            #[inline]
            fn next_power_of_two(self) -> $t {
                $t::next_power_of_two(self)
            }
        }

        impl CheckedNextPowerOfTwo for $t {
            type Output = $t;

            #[inline]
            fn checked_next_power_of_two(self) -> Option<$t> {
                $t::checked_next_power_of_two(self)
            }
        }

        impl NextPowerOfTwoAssign for $t {
            #[inline]
            fn next_power_of_two_assign(&mut self) {
                *self = $t::next_power_of_two(*self)
            }
        }

        impl CheckedLogTwo for $t {
            #[inline]
            fn checked_log_two(self) -> Option<u64> {
                if self == 0 {
                    panic!("Cannot take the base-2 logarithm of 0.");
                }
                let leading_zeros = LeadingZeros::leading_zeros(self);
                let trailing_zeros = TrailingZeros::trailing_zeros(self);
                if leading_zeros + trailing_zeros == $t::WIDTH - 1 {
                    Some(trailing_zeros)
                } else {
                    None
                }
            }
        }

        impl FloorLogTwo for $t {
            /// Returns the floor of the base-2 logarithm of a positive primitive unsigned integer.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `self` is 0.
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::FloorLogTwo;
            ///
            /// assert_eq!(1u8.floor_log_two(), 0);
            /// assert_eq!(100u64.floor_log_two(), 6);
            /// ```
            #[inline]
            fn floor_log_two(self) -> u64 {
                if self == 0 {
                    panic!("Cannot take the base-2 logarithm of 0.");
                }
                self.significant_bits() - 1
            }
        }

        impl CeilingLogTwo for $t {
            /// Returns the ceiling of the base-2 logarithm of a positive primitive unsigned
            /// integer.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `self` is 0.
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::CeilingLogTwo;
            ///
            /// assert_eq!(1u8.ceiling_log_two(), 0);
            /// assert_eq!(100u64.ceiling_log_two(), 7);
            /// ```
            #[inline]
            fn ceiling_log_two(self) -> u64 {
                let floor_log_two = self.floor_log_two();
                if self.is_power_of_two() {
                    floor_log_two
                } else {
                    floor_log_two + 1
                }
            }
        }

        impl Mod for $t {
            type Output = $t;

            #[inline]
            fn mod_op(self, other: $t) -> $t {
                self % other
            }
        }

        impl NegMod for $t {
            type Output = $t;

            #[inline]
            fn neg_mod(self, other: $t) -> $t {
                let rem = self % other;
                if rem == 0 {
                    0
                } else {
                    other - rem
                }
            }
        }

        impl NegModAssign for $t {
            #[inline]
            fn neg_mod_assign(&mut self, other: $t) {
                *self = self.neg_mod(other);
            }
        }

        impl DivRound for $t {
            type Output = $t;

            fn div_round(self, other: $t, rm: RoundingMode) -> $t {
                let quotient = self / other;
                if rm == RoundingMode::Down || rm == RoundingMode::Floor {
                    quotient
                } else {
                    let remainder = self % other;
                    match rm {
                        _ if remainder == 0 => quotient,
                        RoundingMode::Up | RoundingMode::Ceiling => quotient + 1,
                        RoundingMode::Nearest => {
                            let shifted_other = other >> 1;
                            if remainder > shifted_other
                                || remainder == shifted_other && other.even() && quotient.odd()
                            {
                                quotient + 1
                            } else {
                                quotient
                            }
                        }
                        RoundingMode::Exact => {
                            panic!("Division is not exact: {} / {}", self, other);
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
    };
}

impl_arithmetic_traits!(u8);
impl_arithmetic_traits!(u16);
impl_arithmetic_traits!(u32);
impl_arithmetic_traits!(u64);
impl_arithmetic_traits!(u128);
impl_arithmetic_traits!(usize);

#[inline]
pub(crate) fn wide_lower_half<T: PrimitiveUnsigned>(x: T) -> T {
    x.mod_power_of_two(T::WIDTH >> 1)
}

#[inline]
pub(crate) fn wide_upper_half<T: PrimitiveUnsigned>(x: T) -> T {
    x >> (T::WIDTH >> 1)
}

#[inline]
pub(crate) fn wide_split_in_half<T: PrimitiveUnsigned>(x: T) -> (T, T) {
    (wide_upper_half(x), wide_lower_half(x))
}

#[inline]
pub(crate) fn wide_join_halves<T: PrimitiveUnsigned>(hi: T, lo: T) -> T {
    (hi << (T::WIDTH >> 1)) | lo
}
