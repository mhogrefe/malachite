use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Shl, ModPowerOf2ShlAssign, ModPowerOf2Shr, ModPowerOf2ShrAssign, UnsignedAbs,
};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use std::ops::{Shr, ShrAssign};

fn mod_power_of_2_shr_ref<'a, U, S: PrimitiveSigned + UnsignedAbs<Output = U>>(
    x: &'a Natural,
    bits: S,
    pow: u64,
) -> Natural
where
    &'a Natural: ModPowerOf2Shl<U, Output = Natural> + Shr<U, Output = Natural>,
{
    if bits >= S::ZERO {
        x >> bits.unsigned_abs()
    } else {
        x.mod_power_of_2_shl(bits.unsigned_abs(), pow)
    }
}

fn mod_power_of_2_shr_assign<U, S: PrimitiveSigned + UnsignedAbs<Output = U>>(
    x: &mut Natural,
    bits: S,
    pow: u64,
) where
    Natural: ModPowerOf2ShlAssign<U> + ShrAssign<U>,
{
    if bits >= S::ZERO {
        *x >>= bits.unsigned_abs();
    } else {
        x.mod_power_of_2_shl_assign(bits.unsigned_abs(), pow);
    }
}

macro_rules! impl_mod_power_of_2_shr_signed {
    ($t:ident) => {
        impl ModPowerOf2Shr<$t> for Natural {
            type Output = Natural;

            /// Right-shifts a [`Natural`] (divides it by a power of 2) modulo $2^k$. Assumes the
            /// input is already reduced modulo $2^k$. The [`Natural`] is taken by value.
            ///
            /// $f(x, n, k) = y$, where $x, y < 2^k$ and
            /// $\lfloor 2^{-n}x \rfloor \equiv y \mod 2^k$.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_shr#mod_power_of_2_shr).
            #[inline]
            fn mod_power_of_2_shr(mut self, bits: $t, pow: u64) -> Natural {
                self.mod_power_of_2_shr_assign(bits, pow);
                self
            }
        }

        impl<'a> ModPowerOf2Shr<$t> for &'a Natural {
            type Output = Natural;

            /// Right-shifts a [`Natural`] (divides it by a power of 2) modulo $2^k$. Assumes the
            /// input is already reduced modulo $2^k$. The [`Natural`] is taken by reference.
            ///
            /// $f(x, n, k) = y$, where $x, y < 2^k$ and
            /// $\lfloor 2^{-n}x \rfloor \equiv y \mod 2^k$.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_shr#mod_power_of_2_shr).
            #[inline]
            fn mod_power_of_2_shr(self, bits: $t, pow: u64) -> Natural {
                mod_power_of_2_shr_ref(self, bits, pow)
            }
        }

        impl ModPowerOf2ShrAssign<$t> for Natural {
            /// Right-shifts a [`Natural`] (divides it by a power of 2) modulo $2^k$, in place.
            /// Assumes the input is already reduced modulo $2^k$.
            ///
            /// $x \gets y$, where $x, y < 2^k$ and $\lfloor 2^{-n}x \rfloor \equiv y \mod 2^k$.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_shr#mod_power_of_2_shr_assign).
            #[inline]
            fn mod_power_of_2_shr_assign(&mut self, bits: $t, pow: u64) {
                mod_power_of_2_shr_assign(self, bits, pow);
            }
        }
    };
}
apply_to_signeds!(impl_mod_power_of_2_shr_signed);
