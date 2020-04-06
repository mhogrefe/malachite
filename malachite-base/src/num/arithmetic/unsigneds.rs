use num::arithmetic::traits::{
    CeilingDivAssignNegMod, CeilingDivNegMod, CeilingLogTwo, CheckedLogTwo, CheckedNextPowerOfTwo,
    DivAssignMod, DivMod, DivRound, DivisibleByPowerOfTwo, FloorLogTwo, IsPowerOfTwo, Mod, ModAdd,
    ModAddAssign, ModIsReduced, ModNeg, ModNegAssign, ModPowerOfTwo, ModPowerOfTwoAdd,
    ModPowerOfTwoAddAssign, ModPowerOfTwoAssign, ModPowerOfTwoIsReduced, ModPowerOfTwoNeg,
    ModPowerOfTwoNegAssign, ModPowerOfTwoSub, ModPowerOfTwoSubAssign, ModSub, ModSubAssign, NegMod,
    NegModAssign, NegModPowerOfTwo, NegModPowerOfTwoAssign, NextPowerOfTwo, NextPowerOfTwoAssign,
    Parity, PowerOfTwo, RemPowerOfTwo, RemPowerOfTwoAssign, ShrRound, ShrRoundAssign,
    TrueCheckedShl, TrueCheckedShr, WrappingAddAssign, WrappingNegAssign, WrappingSubAssign,
};
use num::basic::integers::PrimitiveInteger;
use num::conversion::traits::WrappingFrom;
use num::logic::traits::{LeadingZeros, LowMask, SignificantBits, TrailingZeros};
use round::RoundingMode;

macro_rules! impl_arithmetic_traits {
    ($t:ident) => {
        impl ModPowerOfTwoIsReduced for $t {
            /// Returns whether `self` is reduced mod 2<sup>`pow`</sup>; in other words, whether it]
            /// has no more than `pow` significant bits.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoIsReduced;
            ///
            /// assert_eq!(0u8.mod_power_of_two_is_reduced(5), true);
            /// assert_eq!(100u64.mod_power_of_two_is_reduced(5), false);
            /// assert_eq!(100u16.mod_power_of_two_is_reduced(8), true);
            /// ```
            #[inline]
            fn mod_power_of_two_is_reduced(&self, pow: u64) -> bool {
                self.significant_bits() <= pow
            }
        }

        impl ModIsReduced for $t {
            /// Returns whether `self` is reduced mod `m`; in other words whether it is less than
            /// `m`. `m` cannot be zero.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `m` is 0.
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModIsReduced;
            ///
            /// assert_eq!(0u8.mod_is_reduced(&5), true);
            /// assert_eq!(100u64.mod_is_reduced(&100), false);
            /// assert_eq!(100u16.mod_is_reduced(&101), true);
            /// ```
            #[inline]
            fn mod_is_reduced(&self, m: &$t) -> bool {
                assert_ne!(*m, 0);
                self < m
            }
        }

        impl ModPowerOfTwoNeg for $t {
            type Output = $t;

            /// Computes `-self` mod 2<sup>`pow`</sup>. Assumes the input is already reduced mod
            /// 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoNeg;
            ///
            /// assert_eq!(0u8.mod_power_of_two_neg(5), 0);
            /// assert_eq!(10u32.mod_power_of_two_neg(4), 6);
            /// assert_eq!(100u16.mod_power_of_two_neg(8), 156);
            /// ```
            #[inline]
            fn mod_power_of_two_neg(self, pow: u64) -> $t {
                assert!(pow <= $t::WIDTH);
                self.wrapping_neg().mod_power_of_two(pow)
            }
        }

        impl ModPowerOfTwoNegAssign for $t {
            /// Replaces `self` with `-self` mod 2<sup>`pow`</sup>. Assumes the input is already
            /// reduced mod 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoNegAssign;
            ///
            /// let mut n = 0u8;
            /// n.mod_power_of_two_neg_assign(5);
            /// assert_eq!(n, 0);
            ///
            /// let mut n = 10u32;
            /// n.mod_power_of_two_neg_assign(4);
            /// assert_eq!(n, 6);
            ///
            /// let mut n = 100u16;
            /// n.mod_power_of_two_neg_assign(8);
            /// assert_eq!(n, 156);
            /// ```
            #[inline]
            fn mod_power_of_two_neg_assign(&mut self, pow: u64) {
                assert!(pow <= $t::WIDTH);
                self.wrapping_neg_assign();
                self.mod_power_of_two_assign(pow);
            }
        }

        impl ModNeg for $t {
            type Output = $t;

            /// Computes `-self` mod `m`. Assumes the input is already reduced mod `m`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModNeg;
            ///
            /// assert_eq!(0u8.mod_neg(5), 0);
            /// assert_eq!(7u32.mod_neg(10), 3);
            /// assert_eq!(100u16.mod_neg(101), 1);
            /// ```
            ///
            /// This is nmod_neg from nmod_vec.h, FLINT Dev 1.
            #[inline]
            fn mod_neg(self, m: $t) -> $t {
                if self == 0 {
                    0
                } else {
                    m - self
                }
            }
        }

        impl ModNegAssign for $t {
            /// Replaces `self` with `-self` mod `m`. Assumes the input is already reduced mod `m`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModNegAssign;
            ///
            /// let mut n = 0u8;
            /// n.mod_neg_assign(5);
            /// assert_eq!(n, 0);
            ///
            /// let mut n = 7u32;
            /// n.mod_neg_assign(10);
            /// assert_eq!(n, 3);
            ///
            /// let mut n = 100u16;
            /// n.mod_neg_assign(101);
            /// assert_eq!(n, 1);
            /// ```
            ///
            /// This is nmod_neg from nmod_vec.h, FLINT Dev 1, where the output is assign to a.
            #[inline]
            fn mod_neg_assign(&mut self, m: $t) {
                if *self != 0 {
                    *self = m - *self;
                }
            }
        }

        impl ModPowerOfTwoAdd for $t {
            type Output = $t;

            /// Computes `self + rhs` mod 2<sup>`pow`</sup>. Assumes the inputs are already reduced
            /// mod 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoAdd;
            ///
            /// assert_eq!(0u8.mod_power_of_two_add(2, 5), 2);
            /// assert_eq!(10u32.mod_power_of_two_add(14, 4), 8);
            /// ```
            #[inline]
            fn mod_power_of_two_add(self, rhs: $t, pow: u64) -> $t {
                assert!(pow <= $t::WIDTH);
                self.wrapping_add(rhs).mod_power_of_two(pow)
            }
        }

        impl ModPowerOfTwoAddAssign for $t {
            /// Replaces `self` with `self + rhs` mod 2<sup>`pow`</sup>. Assumes the inputs are
            /// already reduced mod 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoAddAssign;
            ///
            /// let mut n = 0u8;
            /// n.mod_power_of_two_add_assign(2, 5);
            /// assert_eq!(n, 2);
            ///
            /// let mut n = 10u32;
            /// n.mod_power_of_two_add_assign(14, 4);
            /// assert_eq!(n, 8);
            /// ```
            #[inline]
            fn mod_power_of_two_add_assign(&mut self, rhs: $t, pow: u64) {
                assert!(pow <= $t::WIDTH);
                self.wrapping_add_assign(rhs);
                self.mod_power_of_two_assign(pow);
            }
        }

        impl ModAdd for $t {
            type Output = $t;

            /// Computes `self + rhs` mod `m`. Assumes the inputs are already reduced mod `m`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModAdd;
            ///
            /// assert_eq!(0u8.mod_add(3, 5), 3);
            /// assert_eq!(7u32.mod_add(5, 10), 2);
            /// ```
            ///
            /// This is nmod_add from nmod_vec.h, FLINT Dev 1.
            #[inline]
            fn mod_add(self, rhs: $t, m: $t) -> $t {
                let neg = m - self;
                if neg > rhs {
                    self + rhs
                } else {
                    rhs - neg
                }
            }
        }

        impl ModAddAssign for $t {
            /// Computes `self + rhs` mod `m`. Assumes the inputs are already reduced mod `m`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModAddAssign;
            ///
            /// let mut n = 0u8;
            /// n.mod_add_assign(3, 5);
            /// assert_eq!(n, 3);
            ///
            /// let mut n = 7u32;
            /// n.mod_add_assign(5, 10);
            /// assert_eq!(n, 2);
            /// ```
            ///
            /// This is nmod_add from nmod_vec.h, FLINT Dev 1, where the result is assigned to a.
            #[inline]
            fn mod_add_assign(&mut self, rhs: $t, m: $t) {
                let neg = m - *self;
                if neg > rhs {
                    *self += rhs;
                } else {
                    *self = rhs - neg;
                }
            }
        }

        impl ModPowerOfTwoSub for $t {
            type Output = $t;

            /// Computes `self - rhs` mod 2<sup>`pow`</sup>. Assumes the inputs are already reduced
            /// mod 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoSub;
            ///
            /// assert_eq!(5u8.mod_power_of_two_sub(2, 5), 3);
            /// assert_eq!(10u32.mod_power_of_two_sub(14, 4), 12);
            /// ```
            #[inline]
            fn mod_power_of_two_sub(self, rhs: $t, pow: u64) -> $t {
                assert!(pow <= $t::WIDTH);
                self.wrapping_sub(rhs).mod_power_of_two(pow)
            }
        }

        impl ModPowerOfTwoSubAssign for $t {
            /// Replaces `self` with `self - rhs` mod 2<sup>`pow`</sup>. Assumes the inputs are
            /// already reduced mod 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoSubAssign;
            ///
            /// let mut n = 5u8;
            /// n.mod_power_of_two_sub_assign(2, 5);
            /// assert_eq!(n, 3);
            ///
            /// let mut n = 10u32;
            /// n.mod_power_of_two_sub_assign(14, 4);
            /// assert_eq!(n, 12);
            /// ```
            #[inline]
            fn mod_power_of_two_sub_assign(&mut self, rhs: $t, pow: u64) {
                assert!(pow <= $t::WIDTH);
                self.wrapping_sub_assign(rhs);
                self.mod_power_of_two_assign(pow);
            }
        }

        impl ModSub for $t {
            type Output = $t;

            /// Computes `self - rhs` mod `m`. Assumes the inputs are already reduced mod `m`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModSub;
            ///
            /// assert_eq!(4u8.mod_sub(3, 5), 1);
            /// assert_eq!(7u32.mod_sub(9, 10), 8);
            /// ```
            ///
            /// This is nmod_sub from nmod_vec.h, FLINT Dev 1.
            #[inline]
            fn mod_sub(self, rhs: $t, m: $t) -> $t {
                let diff = self.wrapping_sub(rhs);
                if self < rhs {
                    m.wrapping_add(diff)
                } else {
                    diff
                }
            }
        }

        impl ModSubAssign for $t {
            /// Computes `self - rhs` mod `m`. Assumes the inputs are already reduced mod `m`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModSubAssign;
            ///
            /// let mut n = 4u8;
            /// n.mod_sub_assign(3, 5);
            /// assert_eq!(n, 1);
            ///
            /// let mut n = 7u32;
            /// n.mod_sub_assign(9, 10);
            /// assert_eq!(n, 8);
            /// ```
            ///
            /// This is nmod_sub from nmod_vec.h, FLINT Dev 1, where the result is assigned to a.
            #[inline]
            fn mod_sub_assign(&mut self, rhs: $t, m: $t) {
                *self = self.mod_sub(rhs, m);
            }
        }

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

        impl PowerOfTwo for $t {
            /// Computes 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `pow` is greater than or equal to the width of `$t`.
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::PowerOfTwo;
            ///
            /// assert_eq!(u16::power_of_two(0), 1);
            /// assert_eq!(u8::power_of_two(3), 8);
            /// assert_eq!(u64::power_of_two(40), 1 << 40);
            /// ```
            #[inline]
            fn power_of_two(pow: u64) -> $t {
                assert!(pow < $t::WIDTH);
                1 << pow
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
