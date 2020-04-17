use num::arithmetic::traits::{
    CeilingDivAssignNegMod, CeilingDivNegMod, CeilingLogTwo, CheckedLogTwo, CheckedNextPowerOfTwo,
    DivAssignMod, DivMod, DivRound, DivisibleByPowerOfTwo, FloorLogTwo, IsPowerOfTwo, Mod,
    ModPowerOfTwo, ModPowerOfTwoAssign, NegMod, NegModAssign, NegModPowerOfTwo,
    NegModPowerOfTwoAssign, NextPowerOfTwo, NextPowerOfTwoAssign, Parity, PowerOfTwo,
    RemPowerOfTwo, RemPowerOfTwoAssign, ShrRound, ShrRoundAssign, TrueCheckedShl, TrueCheckedShr,
};
use num::basic::integers::PrimitiveInteger;
use num::basic::unsigneds::PrimitiveUnsigned;
use num::conversion::traits::WrappingFrom;
use num::logic::traits::{LeadingZeros, LowMask, SignificantBits, TrailingZeros};
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

        impl DivisibleByPowerOfTwo for $t {
            #[inline]
            fn divisible_by_power_of_two(self, pow: u64) -> bool {
                self.mod_power_of_two(pow) == 0
            }
        }

        impl ModPowerOfTwo for $t {
            type Output = $t;

            #[inline]
            fn mod_power_of_two(self, pow: u64) -> $t {
                if self == 0 || pow >= $t::WIDTH {
                    self
                } else {
                    self & $t::low_mask(pow)
                }
            }
        }

        impl ModPowerOfTwoAssign for $t {
            #[inline]
            fn mod_power_of_two_assign(&mut self, pow: u64) {
                if *self != 0 && pow < $t::WIDTH {
                    *self &= $t::low_mask(pow)
                }
            }
        }

        impl NegModPowerOfTwo for $t {
            type Output = $t;

            #[inline]
            fn neg_mod_power_of_two(self, pow: u64) -> $t {
                self.wrapping_neg().mod_power_of_two(pow)
            }
        }

        impl NegModPowerOfTwoAssign for $t {
            #[inline]
            fn neg_mod_power_of_two_assign(&mut self, pow: u64) {
                *self = self.neg_mod_power_of_two(pow)
            }
        }

        impl RemPowerOfTwo for $t {
            type Output = $t;

            #[inline]
            fn rem_power_of_two(self, pow: u64) -> $t {
                self.mod_power_of_two(pow)
            }
        }

        impl RemPowerOfTwoAssign for $t {
            #[inline]
            fn rem_power_of_two_assign(&mut self, pow: u64) {
                self.mod_power_of_two_assign(pow)
            }
        }

        impl DivMod for $t {
            type DivOutput = $t;
            type ModOutput = $t;

            #[inline]
            fn div_mod(self, rhs: $t) -> ($t, $t) {
                (self / rhs, self % rhs)
            }
        }

        impl DivAssignMod for $t {
            type ModOutput = $t;

            #[inline]
            fn div_assign_mod(&mut self, rhs: $t) -> $t {
                let rem = *self % rhs;
                *self /= rhs;
                rem
            }
        }

        impl Mod for $t {
            type Output = $t;

            #[inline]
            fn mod_op(self, rhs: $t) -> $t {
                self % rhs
            }
        }

        impl NegMod for $t {
            type Output = $t;

            #[inline]
            fn neg_mod(self, rhs: $t) -> $t {
                let rem = self % rhs;
                if rem == 0 {
                    0
                } else {
                    rhs - rem
                }
            }
        }

        impl NegModAssign for $t {
            #[inline]
            fn neg_mod_assign(&mut self, rhs: $t) {
                *self = self.neg_mod(rhs);
            }
        }

        impl DivRound for $t {
            type Output = $t;

            fn div_round(self, rhs: $t, rm: RoundingMode) -> $t {
                let quotient = self / rhs;
                if rm == RoundingMode::Down || rm == RoundingMode::Floor {
                    quotient
                } else {
                    let remainder = self % rhs;
                    match rm {
                        _ if remainder == 0 => quotient,
                        RoundingMode::Up | RoundingMode::Ceiling => quotient + 1,
                        RoundingMode::Nearest => {
                            let shifted_rhs = rhs >> 1;
                            if remainder > shifted_rhs
                                || remainder == shifted_rhs && rhs.even() && quotient.odd()
                            {
                                quotient + 1
                            } else {
                                quotient
                            }
                        }
                        RoundingMode::Exact => {
                            panic!("Division is not exact: {} / {}", self, rhs);
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }

        impl CeilingDivNegMod for $t {
            type DivOutput = $t;
            type ModOutput = $t;

            #[inline]
            fn ceiling_div_neg_mod(self, rhs: $t) -> ($t, $t) {
                let quotient = self / rhs;
                let remainder = self % rhs;
                if remainder == 0 {
                    (quotient, 0)
                } else {
                    (quotient + 1, rhs - remainder)
                }
            }
        }

        impl CeilingDivAssignNegMod for $t {
            type ModOutput = $t;

            #[inline]
            fn ceiling_div_assign_neg_mod(&mut self, rhs: $t) -> $t {
                let remainder = *self % rhs;
                *self /= rhs;
                if remainder == 0 {
                    0
                } else {
                    *self += 1;
                    rhs - remainder
                }
            }
        }

        impl TrueCheckedShl for $t {
            type Output = $t;

            fn true_checked_shl(self, rhs: u64) -> Option<$t> {
                if self == 0 {
                    Some(self)
                } else if rhs >= $t::WIDTH {
                    None
                } else {
                    let result = self << rhs;
                    if result >> rhs == self {
                        Some(result)
                    } else {
                        None
                    }
                }
            }
        }

        impl TrueCheckedShr for $t {
            type Output = $t;

            fn true_checked_shr(self, _rhs: u64) -> Option<$t> {
                unimplemented!();
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

macro_rules! round_shift_unsigned_unsigned {
    ($t:ident, $u:ident) => {
        impl ShrRound<$u> for $t {
            type Output = $t;

            fn shr_round(self, other: $u, rm: RoundingMode) -> $t {
                if other == 0 || self == 0 {
                    return self;
                }
                let width = $u::wrapping_from($t::WIDTH);
                match rm {
                    RoundingMode::Down | RoundingMode::Floor if other >= width => 0,
                    RoundingMode::Down | RoundingMode::Floor => self >> other,
                    RoundingMode::Up | RoundingMode::Ceiling if other >= width => 1,
                    RoundingMode::Up | RoundingMode::Ceiling => {
                        let shifted = self >> other;
                        if shifted << other == self {
                            shifted
                        } else {
                            shifted + 1
                        }
                    }
                    RoundingMode::Nearest
                        if other == width && self > $t::power_of_two($t::WIDTH - 1) =>
                    {
                        1
                    }
                    RoundingMode::Nearest if other >= width => 0,
                    RoundingMode::Nearest => {
                        let mostly_shifted = self >> (other - 1);
                        if mostly_shifted.even() {
                            // round down
                            mostly_shifted >> 1
                        } else if mostly_shifted << (other - 1) != self {
                            // round up
                            (mostly_shifted >> 1) + 1
                        } else {
                            // result is half-integer; round to even
                            let shifted = mostly_shifted >> 1;
                            if shifted.even() {
                                shifted
                            } else {
                                shifted + 1
                            }
                        }
                    }
                    RoundingMode::Exact if other >= width => {
                        panic!("Right shift is not exact: {} >> {}", self, other);
                    }
                    RoundingMode::Exact => {
                        let shifted = self >> other;
                        if shifted << other != self {
                            panic!("Right shift is not exact: {} >> {}", self, other);
                        }
                        shifted
                    }
                }
            }
        }

        impl ShrRoundAssign<$u> for $t {
            fn shr_round_assign(&mut self, other: $u, rm: RoundingMode) {
                if other == 0 || *self == 0 {
                    return;
                }
                let width = $u::wrapping_from($t::WIDTH);
                match rm {
                    RoundingMode::Down | RoundingMode::Floor if other >= width => *self = 0,
                    RoundingMode::Down | RoundingMode::Floor => *self >>= other,
                    RoundingMode::Up | RoundingMode::Ceiling if other >= width => *self = 1,
                    RoundingMode::Up | RoundingMode::Ceiling => {
                        let original = *self;
                        *self >>= other;
                        if *self << other != original {
                            *self += 1;
                        }
                    }
                    RoundingMode::Nearest
                        if other == width && *self > $t::power_of_two($t::WIDTH - 1) =>
                    {
                        *self = 1;
                    }
                    RoundingMode::Nearest if other >= width => *self = 0,
                    RoundingMode::Nearest => {
                        let original = *self;
                        *self >>= other - 1;
                        if self.even() {
                            // round down
                            *self >>= 1;
                        } else if *self << (other - 1) != original {
                            // round up
                            *self >>= 1;
                            *self += 1;
                        } else {
                            // result is half-integer; round to even
                            *self >>= 1;
                            if self.odd() {
                                *self += 1;
                            }
                        }
                    }
                    RoundingMode::Exact if other >= width => {
                        panic!("Right shift is not exact: {} >>= {}", *self, other);
                    }
                    RoundingMode::Exact => {
                        let original = *self;
                        *self >>= other;
                        if *self << other != original {
                            panic!("Right shift is not exact: {} >>= {}", original, other);
                        }
                    }
                }
            }
        }
    };
}
round_shift_unsigned_unsigned!(u8, u8);
round_shift_unsigned_unsigned!(u8, u16);
round_shift_unsigned_unsigned!(u8, u32);
round_shift_unsigned_unsigned!(u8, u64);
round_shift_unsigned_unsigned!(u8, u128);
round_shift_unsigned_unsigned!(u8, usize);
round_shift_unsigned_unsigned!(u16, u8);
round_shift_unsigned_unsigned!(u16, u16);
round_shift_unsigned_unsigned!(u16, u32);
round_shift_unsigned_unsigned!(u16, u64);
round_shift_unsigned_unsigned!(u16, u128);
round_shift_unsigned_unsigned!(u16, usize);
round_shift_unsigned_unsigned!(u32, u8);
round_shift_unsigned_unsigned!(u32, u16);
round_shift_unsigned_unsigned!(u32, u32);
round_shift_unsigned_unsigned!(u32, u64);
round_shift_unsigned_unsigned!(u32, u128);
round_shift_unsigned_unsigned!(u32, usize);
round_shift_unsigned_unsigned!(u64, u8);
round_shift_unsigned_unsigned!(u64, u16);
round_shift_unsigned_unsigned!(u64, u32);
round_shift_unsigned_unsigned!(u64, u64);
round_shift_unsigned_unsigned!(u64, u128);
round_shift_unsigned_unsigned!(u64, usize);
round_shift_unsigned_unsigned!(u128, u8);
round_shift_unsigned_unsigned!(u128, u16);
round_shift_unsigned_unsigned!(u128, u32);
round_shift_unsigned_unsigned!(u128, u64);
round_shift_unsigned_unsigned!(u128, u128);
round_shift_unsigned_unsigned!(u128, usize);
round_shift_unsigned_unsigned!(usize, u8);
round_shift_unsigned_unsigned!(usize, u16);
round_shift_unsigned_unsigned!(usize, u32);
round_shift_unsigned_unsigned!(usize, u64);
round_shift_unsigned_unsigned!(usize, u128);
round_shift_unsigned_unsigned!(usize, usize);
