use comparison::traits::Min;
use num::arithmetic::traits::{ShrRound, ShrRoundAssign, UnsignedAbs};
use num::basic::integers::PrimitiveInt;
use num::basic::traits::{One, Zero};
use num::conversion::traits::WrappingFrom;
use rounding_modes::RoundingMode;
use std::fmt::Display;
use std::ops::{Neg, Shl, ShlAssign, Shr, ShrAssign, Sub};

fn _shr_round_unsigned_unsigned<
    T: PrimitiveInt + Shl<U, Output = T> + Shr<U, Output = T>,
    U: Copy + Display + Eq + One + Ord + Sub<U, Output = U> + WrappingFrom<u64> + Zero,
>(
    x: T,
    bits: U,
    rm: RoundingMode,
) -> T {
    if bits == U::ZERO || x == T::ZERO {
        return x;
    }
    let width = U::wrapping_from(T::WIDTH);
    match rm {
        RoundingMode::Down | RoundingMode::Floor if bits >= width => T::ZERO,
        RoundingMode::Down | RoundingMode::Floor => x >> bits,
        RoundingMode::Up | RoundingMode::Ceiling if bits >= width => T::ONE,
        RoundingMode::Up | RoundingMode::Ceiling => {
            let shifted = x >> bits;
            if shifted << bits == x {
                shifted
            } else {
                shifted + T::ONE
            }
        }
        RoundingMode::Nearest if bits == width && x > T::power_of_2(T::WIDTH - 1) => T::ONE,
        RoundingMode::Nearest if bits >= width => T::ZERO,
        RoundingMode::Nearest => {
            let mostly_shifted = x >> (bits - U::ONE);
            if mostly_shifted.even() {
                // round down
                mostly_shifted >> 1
            } else if mostly_shifted << (bits - U::ONE) != x {
                // round up
                (mostly_shifted >> 1) + T::ONE
            } else {
                // result is half-integer; round to even
                let shifted: T = mostly_shifted >> 1;
                if shifted.even() {
                    shifted
                } else {
                    shifted + T::ONE
                }
            }
        }
        RoundingMode::Exact if bits >= width => {
            panic!("Right shift is not exact: {} >> {}", x, bits);
        }
        RoundingMode::Exact => {
            let shifted = x >> bits;
            if shifted << bits != x {
                panic!("Right shift is not exact: {} >> {}", x, bits);
            }
            shifted
        }
    }
}

fn _shr_round_assign_unsigned_unsigned<
    T: PrimitiveInt + Shl<U, Output = T> + ShrAssign<U>,
    U: Copy + Display + Eq + One + Ord + Sub<U, Output = U> + WrappingFrom<u64> + Zero,
>(
    x: &mut T,
    bits: U,
    rm: RoundingMode,
) {
    if bits == U::ZERO || *x == T::ZERO {
        return;
    }
    let width = U::wrapping_from(T::WIDTH);
    match rm {
        RoundingMode::Down | RoundingMode::Floor if bits >= width => *x = T::ZERO,
        RoundingMode::Down | RoundingMode::Floor => *x >>= bits,
        RoundingMode::Up | RoundingMode::Ceiling if bits >= width => *x = T::ONE,
        RoundingMode::Up | RoundingMode::Ceiling => {
            let original = *x;
            *x >>= bits;
            if *x << bits != original {
                *x += T::ONE;
            }
        }
        RoundingMode::Nearest if bits == width && *x > T::power_of_2(T::WIDTH - 1) => {
            *x = T::ONE;
        }
        RoundingMode::Nearest if bits >= width => *x = T::ZERO,
        RoundingMode::Nearest => {
            let original = *x;
            *x >>= bits - U::ONE;
            let old_x = *x;
            *x >>= 1;
            if old_x.even() {
                // round down
            } else if old_x << (bits - U::ONE) != original {
                // round up
                *x += T::ONE;
            } else {
                // result is half-integer; round to even
                if x.odd() {
                    *x += T::ONE;
                }
            }
        }
        RoundingMode::Exact if bits >= width => {
            panic!("Right shift is not exact: {} >>= {}", *x, bits);
        }
        RoundingMode::Exact => {
            let original = *x;
            *x >>= bits;
            if *x << bits != original {
                panic!("Right shift is not exact: {} >>= {}", original, bits);
            }
        }
    }
}

macro_rules! impl_shr_round_unsigned_unsigned {
    ($t:ident) => {
        macro_rules! impl_shr_round_unsigned_unsigned_inner {
            ($u:ident) => {
                impl ShrRound<$u> for $t {
                    type Output = $t;

                    /// Shifts `self` right (divides it by a power of 2) and rounds according to the
                    /// specified rounding mode. Passing `RoundingMode::Floor` or
                    /// `RoundingMode::Down` is equivalent to using `>>`. To test whether
                    /// `RoundingMode::Exact` can be passed, use
                    /// `self.divisible_by_power_of_2(bits)`.
                    ///
                    /// Time: worst case O(1)
                    ///
                    /// Additional memory: worst case O(1)
                    ///
                    /// # Panics
                    /// Panics if `rm` is `RoundingMode::Exact` but `self` is not divisible by
                    /// 2<sup>`bits`</sup>.
                    ///
                    /// # Examples
                    /// ```
                    /// use malachite_base::rounding_modes::RoundingMode;
                    /// use malachite_base::num::arithmetic::traits::ShrRound;
                    ///
                    /// assert_eq!(0x101u32.shr_round(8u8, RoundingMode::Down), 1);
                    /// assert_eq!(0x101u16.shr_round(8u16, RoundingMode::Up), 2);
                    ///
                    /// assert_eq!(0x101u64.shr_round(9u32, RoundingMode::Down), 0);
                    /// assert_eq!(0x101u32.shr_round(9u64, RoundingMode::Up), 1);
                    /// assert_eq!(0x101u16.shr_round(9u8, RoundingMode::Nearest), 1);
                    /// assert_eq!(0xffu8.shr_round(9u16, RoundingMode::Nearest), 0);
                    /// assert_eq!(0x100u32.shr_round(9u32, RoundingMode::Nearest), 0);
                    ///
                    /// assert_eq!(0x100u32.shr_round(8u64, RoundingMode::Exact), 1);
                    /// ```
                    #[inline]
                    fn shr_round(self, bits: $u, rm: RoundingMode) -> $t {
                        _shr_round_unsigned_unsigned(self, bits, rm)
                    }
                }

                impl ShrRoundAssign<$u> for $t {
                    /// Shifts `self` right (divides it by a power of 2) and rounds according to the
                    /// specified rounding mode, in place. Passing `RoundingMode::Floor` or
                    /// `RoundingMode::Down` is equivalent to using `>>`. To test whether
                    /// `RoundingMode::Exact` can be passed, use
                    /// `self.divisible_by_power_of_2(bits)`.
                    ///
                    /// Time: worst case O(1)
                    ///
                    /// Additional memory: worst case O(1)
                    ///
                    /// # Panics
                    /// Panics if `rm` is `RoundingMode::Exact` but `self` is not divisible by
                    /// 2<sup>`bits`</sup>.
                    ///
                    /// # Examples
                    /// ```
                    /// use malachite_base::rounding_modes::RoundingMode;
                    /// use malachite_base::num::arithmetic::traits::ShrRoundAssign;
                    ///
                    /// let mut x = 0x101u32;
                    /// x.shr_round_assign(8u8, RoundingMode::Down);
                    /// assert_eq!(x, 1);
                    ///
                    /// let mut x = 0x101u16;
                    /// x.shr_round_assign(8u16, RoundingMode::Up);
                    /// assert_eq!(x, 2);
                    ///
                    /// let mut x = 0x101u64;
                    /// x.shr_round_assign(9u32, RoundingMode::Down);
                    /// assert_eq!(x, 0);
                    ///
                    /// let mut x = 0x101u32;
                    /// x.shr_round_assign(9u64, RoundingMode::Up);
                    /// assert_eq!(x, 1);
                    ///
                    /// let mut x = 0x101u16;
                    /// x.shr_round_assign(9u8, RoundingMode::Nearest);
                    /// assert_eq!(x, 1);
                    ///
                    /// let mut x = 0xffu8;
                    /// x.shr_round_assign(9u16, RoundingMode::Nearest);
                    /// assert_eq!(x, 0);
                    ///
                    /// let mut x = 0x100u32;
                    /// x.shr_round_assign(9u32, RoundingMode::Nearest);
                    /// assert_eq!(x, 0);
                    ///
                    /// let mut x = 0x100u32;
                    /// x.shr_round_assign(8u64, RoundingMode::Exact);
                    /// assert_eq!(x, 1);
                    /// ```
                    #[inline]
                    fn shr_round_assign(&mut self, bits: $u, rm: RoundingMode) {
                        _shr_round_assign_unsigned_unsigned(self, bits, rm);
                    }
                }
            };
        }
        apply_to_unsigneds!(impl_shr_round_unsigned_unsigned_inner);
    };
}
apply_to_unsigneds!(impl_shr_round_unsigned_unsigned);

fn _shr_round_signed_unsigned<
    U: Copy + Eq + ShrRound<B, Output = U> + Zero,
    S: Copy + Eq + Min + Neg<Output = S> + Ord + UnsignedAbs<Output = U> + WrappingFrom<U> + Zero,
    B,
>(
    x: S,
    bits: B,
    rm: RoundingMode,
) -> S {
    let abs = x.unsigned_abs();
    if x >= S::ZERO {
        S::wrapping_from(abs.shr_round(bits, rm))
    } else {
        let abs_shifted = abs.shr_round(bits, -rm);
        if abs_shifted == U::ZERO {
            S::ZERO
        } else if abs_shifted == S::MIN.unsigned_abs() {
            S::MIN
        } else {
            -S::wrapping_from(abs_shifted)
        }
    }
}

macro_rules! impl_shr_round_signed_unsigned {
    ($t:ident) => {
        macro_rules! impl_shr_round_signed_unsigned_inner {
            ($u:ident) => {
                impl ShrRound<$u> for $t {
                    type Output = $t;

                    /// Shifts `self` right (divides it by a power of 2) and rounds according to the
                    /// specified rounding mode. Passing `RoundingMode::Floor` or
                    /// `RoundingMode::Down` is equivalent to using `>>`. To test whether
                    /// `RoundingMode::Exact` can be passed, use
                    /// `self.divisible_by_power_of_2(bits)`.
                    ///
                    /// Time: worst case O(1)
                    ///
                    /// Additional memory: worst case O(1)
                    ///
                    /// # Panics
                    /// Panics if `rm` is `RoundingMode::Exact` but `self` is not divisible by
                    /// 2<sup>`bits`</sup>.
                    ///
                    /// # Examples
                    /// ```
                    /// use malachite_base::rounding_modes::RoundingMode;
                    /// use malachite_base::num::arithmetic::traits::ShrRound;
                    ///
                    /// assert_eq!(0x101i32.shr_round(8u8, RoundingMode::Down), 1);
                    /// assert_eq!(0x101i16.shr_round(8u16, RoundingMode::Up), 2);
                    ///
                    /// assert_eq!((-0x101i32).shr_round(9u32, RoundingMode::Down), 0);
                    /// assert_eq!((-0x101i64).shr_round(9u64, RoundingMode::Up), -1);
                    /// assert_eq!((-0x101i16).shr_round(9u8, RoundingMode::Nearest), -1);
                    /// assert_eq!((-0xffi32).shr_round(9u16, RoundingMode::Nearest), 0);
                    /// assert_eq!((-0x100i64).shr_round(9u32, RoundingMode::Nearest), 0);
                    ///
                    /// assert_eq!(0x100i32.shr_round(8u64, RoundingMode::Exact), 1);
                    /// ```
                    #[inline]
                    fn shr_round(self, bits: $u, rm: RoundingMode) -> $t {
                        _shr_round_signed_unsigned(self, bits, rm)
                    }
                }

                impl ShrRoundAssign<$u> for $t {
                    /// Shifts `self` right (divides it by a power of 2) and rounds according to the
                    /// specified rounding mode, in place. Passing `RoundingMode::Floor` or
                    /// `RoundingMode::Down` is equivalent to using `>>`. To test whether
                    /// `RoundingMode::Exact` can be passed, use
                    /// `self.divisible_by_power_of_2(bits)`.
                    ///
                    /// Time: worst case O(1)
                    ///
                    /// Additional memory: worst case O(1)
                    ///
                    /// # Panics
                    /// Panics if `rm` is `RoundingMode::Exact` but `self` is not divisible by
                    /// 2<sup>`bits`</sup>.
                    ///
                    /// # Examples
                    /// ```
                    /// use malachite_base::rounding_modes::RoundingMode;
                    /// use malachite_base::num::arithmetic::traits::ShrRoundAssign;
                    ///
                    /// let mut x = 0x101i32;
                    /// x.shr_round_assign(8u8, RoundingMode::Down);
                    /// assert_eq!(x, 1);
                    ///
                    /// let mut x = 0x101i16;
                    /// x.shr_round_assign(8u16, RoundingMode::Up);
                    /// assert_eq!(x, 2);
                    ///
                    /// let mut x = -0x101i32;
                    /// x.shr_round_assign(9u32, RoundingMode::Down);
                    /// assert_eq!(x, 0);
                    ///
                    /// let mut x = -0x101i64;
                    /// x.shr_round_assign(9u64, RoundingMode::Up);
                    /// assert_eq!(x, -1);
                    ///
                    /// let mut x = -0x101i16;
                    /// x.shr_round_assign(9u8, RoundingMode::Nearest);
                    /// assert_eq!(x, -1);
                    ///
                    /// let mut x = -0xffi32;
                    /// x.shr_round_assign(9u16, RoundingMode::Nearest);
                    /// assert_eq!(x, 0);
                    ///
                    /// let mut x = -0x100i64;
                    /// x.shr_round_assign(9u32, RoundingMode::Nearest);
                    /// assert_eq!(x, 0);
                    ///
                    /// let mut x = 0x100u32;
                    /// x.shr_round_assign(8i64, RoundingMode::Exact);
                    /// assert_eq!(x, 1);
                    /// ```
                    #[inline]
                    fn shr_round_assign(&mut self, bits: $u, rm: RoundingMode) {
                        *self = self.shr_round(bits, rm);
                    }
                }
            };
        }
        apply_to_unsigneds!(impl_shr_round_signed_unsigned_inner);
    };
}
apply_to_signeds!(impl_shr_round_signed_unsigned);

fn _shr_round_primitive_signed<
    T: PrimitiveInt + Shl<U, Output = T> + ShrRound<U, Output = T>,
    U: Ord + WrappingFrom<u64>,
    S: Copy + Ord + UnsignedAbs<Output = U> + Zero,
>(
    x: T,
    bits: S,
    rm: RoundingMode,
) -> T {
    if bits >= S::ZERO {
        x.shr_round(bits.unsigned_abs(), rm)
    } else {
        let abs = bits.unsigned_abs();
        if abs >= U::wrapping_from(T::WIDTH) {
            T::ZERO
        } else {
            x << bits.unsigned_abs()
        }
    }
}

fn _shr_round_assign_primitive_signed<
    T: PrimitiveInt + ShlAssign<U> + ShrRoundAssign<U>,
    U: Ord + WrappingFrom<u64>,
    S: Copy + Ord + UnsignedAbs<Output = U> + Zero,
>(
    x: &mut T,
    bits: S,
    rm: RoundingMode,
) {
    if bits >= S::ZERO {
        x.shr_round_assign(bits.unsigned_abs(), rm);
    } else {
        let abs = bits.unsigned_abs();
        if abs >= U::wrapping_from(T::WIDTH) {
            *x = T::ZERO;
        } else {
            *x <<= bits.unsigned_abs();
        }
    }
}

macro_rules! impl_shr_round_primitive_signed {
    ($t:ident) => {
        macro_rules! impl_shr_round_primitive_signed_inner {
            ($u:ident) => {
                impl ShrRound<$u> for $t {
                    type Output = $t;

                    /// Shifts `self` right (divides it by a power of 2) and rounds according to the
                    /// specified rounding mode. Passing `RoundingMode::Floor` or
                    /// `RoundingMode::Down` is equivalent to using `>>`. To test whether
                    /// `RoundingMode::Exact` can be passed, use
                    /// `self.divisible_by_power_of_2(bits)`. Rounding might only be necessary if
                    /// `bits` is non-negative.
                    ///
                    /// Time: worst case O(1)
                    ///
                    /// Additional memory: worst case O(1)
                    ///
                    /// # Panics
                    /// Panics if `bits` is positive and `rm` is `RoundingMode::Exact` but `self` is
                    /// not divisible by 2<sup>`bits`</sup>.
                    ///
                    /// # Examples
                    /// ```
                    /// use malachite_base::rounding_modes::RoundingMode;
                    /// use malachite_base::num::arithmetic::traits::ShrRound;
                    ///
                    /// assert_eq!(0x101u32.shr_round(8i8, RoundingMode::Down), 1);
                    /// assert_eq!(0x101u16.shr_round(8i16, RoundingMode::Up), 2);
                    ///
                    /// assert_eq!((-0x101i32).shr_round(9i32, RoundingMode::Down), 0);
                    /// assert_eq!((-0x101i64).shr_round(9i64, RoundingMode::Up), -1);
                    /// assert_eq!((-0x101i16).shr_round(9i8, RoundingMode::Nearest), -1);
                    /// assert_eq!((-0xffi32).shr_round(9i16, RoundingMode::Nearest), 0);
                    /// assert_eq!((-0x100i64).shr_round(9i32, RoundingMode::Nearest), 0);
                    ///
                    /// assert_eq!(0x100u32.shr_round(8i64, RoundingMode::Exact), 1);
                    /// ```
                    #[inline]
                    fn shr_round(self, bits: $u, rm: RoundingMode) -> $t {
                        _shr_round_primitive_signed(self, bits, rm)
                    }
                }

                impl ShrRoundAssign<$u> for $t {
                    /// Shifts `self` right (divides it by a power of 2) and rounds according to the
                    /// specified rounding mode, in place. Passing `RoundingMode::Floor` or
                    /// `RoundingMode::Down` is equivalent to using `>>`. To test whether
                    /// `RoundingMode::Exact` can be passed, use
                    /// `self.divisible_by_power_of_2(bits)`. Rounding might only be necessary if
                    /// `bits` is non-negative.
                    ///
                    /// Time: worst case O(1)
                    ///
                    /// Additional memory: worst case O(1)
                    ///
                    /// # Panics
                    /// Panics if `bits` is positive and `rm` is `RoundingMode::Exact` but `self` is
                    /// not divisible by 2<sup>`bits`</sup>.
                    ///
                    /// # Examples
                    /// ```
                    /// use malachite_base::rounding_modes::RoundingMode;
                    /// use malachite_base::num::arithmetic::traits::ShrRoundAssign;
                    ///
                    /// let mut x = 0x101u32;
                    /// x.shr_round_assign(8i8, RoundingMode::Down);
                    /// assert_eq!(x, 1);
                    ///
                    /// let mut x = 0x101u16;
                    /// x.shr_round_assign(8i16, RoundingMode::Up);
                    /// assert_eq!(x, 2);
                    ///
                    /// let mut x = -0x101i32;
                    /// x.shr_round_assign(9i32, RoundingMode::Down);
                    /// assert_eq!(x, 0);
                    ///
                    /// let mut x = -0x101i64;
                    /// x.shr_round_assign(9i64, RoundingMode::Up);
                    /// assert_eq!(x, -1);
                    ///
                    /// let mut x = -0x101i16;
                    /// x.shr_round_assign(9i8, RoundingMode::Nearest);
                    /// assert_eq!(x, -1);
                    ///
                    /// let mut x = -0xffi32;
                    /// x.shr_round_assign(9i16, RoundingMode::Nearest);
                    /// assert_eq!(x, 0);
                    ///
                    /// let mut x = -0x100i64;
                    /// x.shr_round_assign(9i32, RoundingMode::Nearest);
                    /// assert_eq!(x, 0);
                    ///
                    /// let mut x = 0x100u32;
                    /// x.shr_round_assign(8i64, RoundingMode::Exact);
                    /// assert_eq!(x, 1);
                    /// ```
                    #[inline]
                    fn shr_round_assign(&mut self, bits: $u, rm: RoundingMode) {
                        _shr_round_assign_primitive_signed(self, bits, rm)
                    }
                }
            };
        }
        apply_to_signeds!(impl_shr_round_primitive_signed_inner);
    };
}
apply_to_primitive_ints!(impl_shr_round_primitive_signed);
