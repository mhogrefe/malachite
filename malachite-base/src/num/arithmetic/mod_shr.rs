use num::arithmetic::traits::{ModShl, ModShlAssign, ModShr, ModShrAssign, UnsignedAbs};
use num::basic::integers::PrimitiveInt;
use num::basic::traits::Zero;
use num::conversion::traits::WrappingFrom;
use std::ops::{Shr, ShrAssign};

fn _mod_shr_signed<
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
        let width = U::wrapping_from(T::WIDTH);
        if width != U::ZERO && other_abs >= width {
            T::ZERO
        } else {
            x >> other_abs
        }
    } else {
        x.mod_shl(other_abs, m)
    }
}

fn _mod_shr_assign_signed<
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
        let width = U::wrapping_from(T::WIDTH);
        if width != U::ZERO && other_abs >= width {
            *x = T::ZERO;
        } else {
            *x >>= other_abs;
        }
    } else {
        x.mod_shl_assign(other_abs, m);
    }
}

macro_rules! impl_mod_shr_signed {
    ($t:ident) => {
        macro_rules! impl_mod_shr_signed_inner {
            ($u:ident) => {
                impl ModShr<$u, $t> for $t {
                    type Output = $t;

                    /// Computes `self >> other` mod `m`. Assumes the input is already reduced mod
                    /// `m`.
                    ///
                    /// $f(x, n, m) = y$, where $x, y < m$ and
                    /// $\lfloor 2^{-n}x \rfloor \equiv y \mod m$.
                    ///
                    /// # Worst-case complexity
                    /// $T(n) = O(n)$
                    ///
                    /// $M(n) = O(1)$
                    ///
                    /// where $T$ is time, $M$ is additional memory, and $n$ is
                    /// `other.significant_bits()`.
                    ///
                    /// # Examples
                    /// See the documentation of the `num::arithmetic::mod_shr` module.
                    #[inline]
                    fn mod_shr(self, other: $u, m: $t) -> $t {
                        _mod_shr_signed(self, other, m)
                    }
                }

                impl ModShrAssign<$u, $t> for $t {
                    /// Replaces `self` with `self >> other` mod `m`. Assumes the input is already
                    /// reduced mod `m`.
                    ///
                    /// $x \gets y$, where $x, y < m$ and
                    /// $\lfloor 2^{-n}x \rfloor \equiv y \mod m$.
                    ///
                    /// # Worst-case complexity
                    /// $T(n) = O(n)$
                    ///
                    /// $M(n) = O(1)$
                    ///
                    /// where $T$ is time, $M$ is additional memory, and $n$ is
                    /// `other.significant_bits()`.
                    ///
                    /// # Examples
                    /// See the documentation of the `num::arithmetic::mod_shr` module.
                    #[inline]
                    fn mod_shr_assign(&mut self, other: $u, m: $t) {
                        _mod_shr_assign_signed(self, other, m)
                    }
                }
            };
        }
        apply_to_signeds!(impl_mod_shr_signed_inner);
    };
}
apply_to_unsigneds!(impl_mod_shr_signed);
