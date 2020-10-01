use num::arithmetic::traits::{ModMul, ModMulAssign, ModPow, ModShl, ModShlAssign, UnsignedAbs};
use num::basic::integers::PrimitiveInt;
use num::basic::traits::Zero;
use num::conversion::traits::{ExactFrom, WrappingFrom};
use std::ops::{Shr, ShrAssign};

fn _mod_shl_unsigned<T: ModMul<T, Output = T> + ModPow<u64, T, Output = T> + PrimitiveInt, U>(
    x: T,
    other: U,
    m: T,
) -> T
where
    u64: ExactFrom<U>,
{
    if m == T::ONE {
        T::ZERO
    } else {
        x.mod_mul(T::TWO.mod_pow(u64::exact_from(other), m), m)
    }
}

fn _mod_shl_assign_unsigned<T: ModMulAssign<T> + ModPow<u64, T, Output = T> + PrimitiveInt, U>(
    x: &mut T,
    other: U,
    m: T,
) where
    u64: ExactFrom<U>,
{
    if m == T::ONE {
        *x = T::ZERO;
    } else {
        x.mod_mul_assign(T::TWO.mod_pow(u64::exact_from(other), m), m);
    }
}

macro_rules! impl_mod_shl_unsigned {
    ($t:ident) => {
        macro_rules! impl_mod_shl_unsigned_inner {
            ($u:ident) => {
                impl ModShl<$u, $t> for $t {
                    type Output = $t;

                    /// Computes `self << other` mod `m`. Assumes the input is already reduced mod
                    /// `m`.
                    ///
                    /// Time: worst case O(1)
                    ///
                    /// Additional memory: worst case O(1)
                    ///
                    /// # Example
                    /// ```
                    /// use malachite_base::num::arithmetic::traits::ModShl;
                    ///
                    /// assert_eq!(8u32.mod_shl(2u8, 10), 2);
                    /// assert_eq!(10u8.mod_shl(100u64, 17), 7);
                    /// ```
                    #[inline]
                    fn mod_shl(self, other: $u, m: $t) -> $t {
                        _mod_shl_unsigned(self, other, m)
                    }
                }

                impl ModShlAssign<$u, $t> for $t {
                    /// Replaces `self` with `self << other` mod `m`. Assumes the input is already
                    /// reduced mod `m`.
                    ///
                    /// Time: worst case O(1)
                    ///
                    /// Additional memory: worst case O(1)
                    ///
                    /// # Example
                    /// ```
                    /// use malachite_base::num::arithmetic::traits::ModShlAssign;
                    ///
                    /// let mut n = 8u32;
                    /// n.mod_shl_assign(2u8, 10);
                    /// assert_eq!(n, 2);
                    ///
                    /// let mut n = 10u8;
                    /// n.mod_shl_assign(100u64, 17);
                    /// assert_eq!(n, 7);
                    /// ```
                    #[inline]
                    fn mod_shl_assign(&mut self, other: $u, m: $t) {
                        _mod_shl_assign_unsigned(self, other, m);
                    }
                }
            };
        }
        apply_to_unsigneds!(impl_mod_shl_unsigned_inner);
    };
}
apply_to_unsigneds!(impl_mod_shl_unsigned);

fn _mod_shl_signed<
    T: ModShl<U, T, Output = T> + PrimitiveInt + Shr<U, Output = T>,
    U: Copy + Eq + Ord + WrappingFrom<u64> + Zero,
    S: Copy + Ord + UnsignedAbs<Output = U> + Zero,
>(
    x: T,
    other: S,
    m: T,
) -> T {
    let other_abs = other.unsigned_abs();
    if other >= S::ZERO {
        x.mod_shl(other_abs, m)
    } else {
        let width = U::wrapping_from(T::WIDTH);
        if width != U::ZERO && other_abs >= width {
            T::ZERO
        } else {
            x >> other_abs
        }
    }
}

fn _mod_shl_assign_signed<
    T: ModShlAssign<U, T> + PrimitiveInt + ShrAssign<U>,
    U: Copy + Eq + Ord + WrappingFrom<u64> + Zero,
    S: Copy + Ord + UnsignedAbs<Output = U> + Zero,
>(
    x: &mut T,
    other: S,
    m: T,
) {
    let other_abs = other.unsigned_abs();
    if other >= S::ZERO {
        x.mod_shl_assign(other_abs, m);
    } else {
        let width = U::wrapping_from(T::WIDTH);
        if width != U::ZERO && other_abs >= width {
            *x = T::ZERO;
        } else {
            *x >>= other_abs;
        }
    }
}

macro_rules! impl_mod_shl_signed {
    ($t:ident) => {
        macro_rules! impl_mod_shl_signed_inner {
            ($u:ident) => {
                impl ModShl<$u, $t> for $t {
                    type Output = $t;

                    /// Computes `self << other` mod `m`. Assumes the input is already reduced mod
                    /// `m`.
                    ///
                    /// Time: worst case O(1)
                    ///
                    /// Additional memory: worst case O(1)
                    ///
                    /// # Example
                    /// ```
                    /// use malachite_base::num::arithmetic::traits::ModShl;
                    ///
                    /// assert_eq!(8u32.mod_shl(2i8, 10), 2);
                    /// assert_eq!(10u8.mod_shl(-2i64, 15), 2);
                    /// ```
                    #[inline]
                    fn mod_shl(self, other: $u, m: $t) -> $t {
                        _mod_shl_signed(self, other, m)
                    }
                }

                impl ModShlAssign<$u, $t> for $t {
                    /// Replaces `self` with `self << other` mod `m`. Assumes the input is already
                    /// reduced mod `m`.
                    ///
                    /// Time: worst case O(1)
                    ///
                    /// Additional memory: worst case O(1)
                    ///
                    /// # Example
                    /// ```
                    /// use malachite_base::num::arithmetic::traits::ModShlAssign;
                    ///
                    /// let mut n = 8u32;
                    /// n.mod_shl_assign(2i8, 10);
                    /// assert_eq!(n, 2);
                    ///
                    /// let mut n = 10u8;
                    /// n.mod_shl_assign(-2i64, 15);
                    /// assert_eq!(n, 2);
                    /// ```
                    #[inline]
                    fn mod_shl_assign(&mut self, other: $u, m: $t) {
                        _mod_shl_assign_signed(self, other, m);
                    }
                }
            };
        }
        apply_to_signeds!(impl_mod_shl_signed_inner);
    };
}
apply_to_unsigneds!(impl_mod_shl_signed);
