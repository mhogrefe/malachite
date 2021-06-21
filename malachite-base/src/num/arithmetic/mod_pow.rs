use num::arithmetic::mod_mul::{_limbs_invert_limb_u32, _limbs_invert_limb_u64};
use num::arithmetic::traits::{ModPow, ModPowAssign, ModPowPrecomputed, ModPowPrecomputedAssign};
use num::basic::integers::PrimitiveInt;
use num::basic::unsigneds::PrimitiveUnsigned;
use num::conversion::traits::WrappingFrom;
use num::conversion::traits::{HasHalf, JoinHalves, SplitInHalf};
use num::logic::traits::{BitIterable, LeadingZeros};

#[doc(hidden)]
pub fn _simple_binary_mod_pow<T: PrimitiveUnsigned>(x: T, exp: u64, m: T) -> T {
    if m == T::ONE {
        return T::ZERO;
    }
    let data = T::precompute_mod_mul_data(&m);
    let mut out = T::ONE;
    for bit in exp.bits().rev() {
        out.mod_mul_precomputed_assign(out, m, &data);
        if bit {
            out.mod_mul_precomputed_assign(x, m, &data);
        }
    }
    out
}

// m.get_highest_bit(), x < m, y < m
//
// This is n_mulmod_preinv from ulong_extras/mulmod_preinv.c, FLINT 2.7.1.
fn mul_mod_helper<
    T: PrimitiveUnsigned,
    DT: From<T> + HasHalf<Half = T> + JoinHalves + PrimitiveUnsigned + SplitInHalf,
>(
    mut x: T,
    y: T,
    m: T,
    inverse: T,
    shift: u64,
) -> T {
    x >>= shift;
    let p = DT::from(x) * DT::from(y);
    let (p_hi, p_lo) = p.split_in_half();
    let (q_1, q_0) = (DT::from(p_hi) * DT::from(inverse))
        .wrapping_add(p)
        .split_in_half();
    let mut r = p_lo.wrapping_sub(q_1.wrapping_add(T::ONE).wrapping_mul(m));
    if r > q_0 {
        r.wrapping_add_assign(m);
    }
    if r < m {
        r
    } else {
        r.wrapping_sub(m)
    }
}

// m.get_highest_bit(), x < m
//
// This is n_powmod_ui_preinv from ulong_extras/powmod_ui_preinv.c, FLINT 2.7.1.
fn _fast_pow_mod<
    T: PrimitiveUnsigned,
    DT: From<T> + HasHalf<Half = T> + JoinHalves + PrimitiveUnsigned + SplitInHalf,
>(
    mut x: T,
    exp: u64,
    m: T,
    inverse: T,
    shift: u64,
) -> T {
    if exp == 0 {
        let x = T::power_of_2(shift);
        if x == m {
            T::ZERO
        } else {
            x
        }
    } else if x == T::ZERO {
        T::ZERO
    } else {
        let mut bits = exp.bits();
        let mut out = if bits.next().unwrap() {
            x
        } else {
            T::power_of_2(shift)
        };
        for bit in bits {
            x = mul_mod_helper::<T, DT>(x, x, m, inverse, shift);
            if bit {
                out = mul_mod_helper::<T, DT>(out, x, m, inverse, shift);
            }
        }
        out
    }
}

macro_rules! impl_mod_pow_precomputed_fast {
    ($t:ident, $dt:ident, $invert_limb:ident) => {
        impl ModPowPrecomputed<u64, $t> for $t {
            type Output = $t;
            type Data = ($t, u64);

            /// Precomputes data for modular exponentiation.
            ///
            /// See `mod_pow_precomputed` and `mod_pow_precomputed_assign`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            fn precompute_mod_pow_data(&m: &$t) -> ($t, u64) {
                let leading_zeros = LeadingZeros::leading_zeros(m);
                ($invert_limb(m << leading_zeros), leading_zeros)
            }

            /// Computes `self.pow(exp)` mod `m`. Assumes the input is already reduced mod `m`.
            ///
            /// Some precomputed data is provided; this speeds up computations involving several
            /// modular exponentiations with the same modulus. The precomputed data should be
            /// obtained using `precompute_mod_pow_data`.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `exp.significant_bits()`.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_pow` module.
            fn mod_pow_precomputed(self, exp: u64, m: $t, data: &($t, u64)) -> $t {
                let (inverse, shift) = *data;
                _fast_pow_mod::<$t, $dt>(self << shift, exp, m << shift, inverse, shift) >> shift
            }
        }
    };
}
impl_mod_pow_precomputed_fast!(u32, u64, _limbs_invert_limb_u32);
impl_mod_pow_precomputed_fast!(u64, u128, _limbs_invert_limb_u64);

macro_rules! impl_mod_pow_precomputed_promoted {
    ($t:ident) => {
        impl ModPowPrecomputed<u64, $t> for $t {
            type Output = $t;
            type Data = (u32, u64);

            /// Precomputes data for modular exponentiation. See `mod_pow_precomputed` and
            /// `mod_pow_precomputed_assign`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            fn precompute_mod_pow_data(&m: &$t) -> (u32, u64) {
                u32::precompute_mod_pow_data(&u32::from(m))
            }

            /// Computes `self.pow(exp)` mod `m`. Assumes the input is already reduced mod `m`.
            ///
            /// Some precomputed data is provided; this speeds up computations involving several
            /// modular exponentiations with the same modulus. The precomputed data should be
            /// obtained using `precompute_mod_pow_data`.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `exp.significant_bits()`.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_pow` module.
            fn mod_pow_precomputed(self, exp: u64, m: $t, data: &(u32, u64)) -> $t {
                $t::wrapping_from(u32::from(self).mod_pow_precomputed(exp, u32::from(m), data))
            }
        }
    };
}
impl_mod_pow_precomputed_promoted!(u8);
impl_mod_pow_precomputed_promoted!(u16);

impl ModPowPrecomputed<u64, u128> for u128 {
    type Output = u128;
    type Data = ();

    /// Precomputes data for modular exponentiation. See `mod_pow_precomputed` and
    /// `mod_pow_precomputed_assign`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    fn precompute_mod_pow_data(_m: &u128) {}

    /// Computes `self.pow(exp) mod `m`. Assumes the input is already reduced mod `m`.
    ///
    /// Some precomputed data is provided; this speeds up computations involving several modular
    /// exponentiations with the same modulus. The precomputed data should be obtained using
    /// `precompute_mod_pow_data`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `exp.significant_bits()`.
    ///
    /// # Examples
    /// See the documentation of the `num::arithmetic::mod_pow` module.
    #[inline]
    fn mod_pow_precomputed(self, exp: u64, m: u128, _data: &()) -> u128 {
        _simple_binary_mod_pow(self, exp, m)
    }
}

impl ModPowPrecomputed<u64, usize> for usize {
    type Output = usize;
    type Data = (usize, u64);

    /// Precomputes data for modular exponentiation. See `mod_pow_precomputed` and
    /// `mod_pow_precomputed_assign`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    fn precompute_mod_pow_data(&m: &usize) -> (usize, u64) {
        if usize::WIDTH == u32::WIDTH {
            let (inverse, shift) = u32::precompute_mod_pow_data(&u32::wrapping_from(m));
            (usize::wrapping_from(inverse), shift)
        } else {
            let (inverse, shift) = u64::precompute_mod_pow_data(&u64::wrapping_from(m));
            (usize::wrapping_from(inverse), shift)
        }
    }

    /// Computes `self.pow(exp)` mod `m`. Assumes the input is already reduced mod `m`.
    ///
    /// Some precomputed data is provided; this speeds up computations involving several modular
    /// exponentiations with the same modulus. The precomputed data should be obtained using
    /// `precompute_mod_pow_data`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `exp.significant_bits()`.
    fn mod_pow_precomputed(self, exp: u64, m: usize, data: &(usize, u64)) -> usize {
        let (inverse, shift) = *data;
        if usize::WIDTH == u32::WIDTH {
            usize::wrapping_from(u32::wrapping_from(self).mod_pow_precomputed(
                exp,
                u32::wrapping_from(m),
                &(u32::wrapping_from(inverse), shift),
            ))
        } else {
            usize::wrapping_from(u64::wrapping_from(self).mod_pow_precomputed(
                exp,
                u64::wrapping_from(m),
                &(u64::wrapping_from(inverse), shift),
            ))
        }
    }
}

macro_rules! impl_mod_pow {
    ($t:ident) => {
        impl ModPowPrecomputedAssign<u64, $t> for $t {
            /// Replaces `self` with `self.pow(exp)` mod `m`.
            ///
            /// Assumes the input is already reduced mod `m`. Some precomputed data is provided;
            /// this speeds up computations involving several modular exponentiations with the same
            /// modulus. The precomputed data should be obtained using `precompute_mod_pow_data`.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `exp.significant_bits()`.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_pow` module.
            #[inline]
            fn mod_pow_precomputed_assign(&mut self, exp: u64, m: $t, data: &Self::Data) {
                *self = self.mod_pow_precomputed(exp, m, data);
            }
        }

        impl ModPow<u64> for $t {
            type Output = $t;

            /// Computes `self.pow(exp)` mod `m`. Assumes the input is already reduced mod `m`.
            ///
            /// $f(x, n, m) = y$, where $x, y < m$ and $x^n \equiv y \mod m$.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `exp.significant_bits()`.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_pow` module.
            #[inline]
            fn mod_pow(self, exp: u64, m: $t) -> $t {
                _simple_binary_mod_pow(self, exp, m)
            }
        }

        impl ModPowAssign<u64> for $t {
            /// Replaces `self` with `self.pow(exp)` mod `m`. Assumes the input is already reduced
            /// mod `m`.
            ///
            /// $x \gets y$, where $x, y < m$ and $x^n \equiv y \mod m$.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `exp.significant_bits()`.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_pow` module.
            #[inline]
            fn mod_pow_assign(&mut self, exp: u64, m: $t) {
                *self = _simple_binary_mod_pow(*self, exp, m);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_pow);
