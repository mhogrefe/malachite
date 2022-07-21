use crate::num::arithmetic::traits::{ModShl, ModShlAssign, ModShr, ModShrAssign, UnsignedAbs};
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use std::ops::{Shr, ShrAssign};

fn mod_shr_signed<
    T: ModShl<U, T, Output = T> + PrimitiveUnsigned + Shr<U, Output = T>,
    U: PrimitiveUnsigned,
    S: PrimitiveSigned + UnsignedAbs<Output = U>,
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

fn mod_shr_assign_signed<
    T: ModShlAssign<U, T> + PrimitiveUnsigned + ShrAssign<U>,
    U: PrimitiveUnsigned,
    S: PrimitiveSigned + UnsignedAbs<Output = U>,
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

                    /// Right-shifts a number (divides it by a power of 2) modulo a number $m$.
                    /// Assumes the input is already reduced modulo $m$.
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
                    /// See [here](super::mod_shr#mod_shr).
                    #[inline]
                    fn mod_shr(self, other: $u, m: $t) -> $t {
                        mod_shr_signed(self, other, m)
                    }
                }

                impl ModShrAssign<$u, $t> for $t {
                    /// Right-shifts a number (divides it by a power of 2) modulo a number $m$, in
                    /// place. Assumes the input is already reduced modulo $m$.
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
                    /// See [here](super::mod_shr#mod_shr).
                    #[inline]
                    fn mod_shr_assign(&mut self, other: $u, m: $t) {
                        mod_shr_assign_signed(self, other, m)
                    }
                }
            };
        }
        apply_to_signeds!(impl_mod_shr_signed_inner);
    };
}
apply_to_unsigneds!(impl_mod_shr_signed);
