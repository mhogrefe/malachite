use num::arithmetic::mod_mul::{_limbs_invert_limb_u32, _limbs_invert_limb_u64};
use num::arithmetic::traits::{ModPow, ModPowAssign, ModPowPrecomputed, ModPowPrecomputedAssign};
use num::basic::integers::PrimitiveInt;
use num::basic::unsigneds::PrimitiveUnsigned;
use num::conversion::traits::WrappingFrom;
use num::conversion::traits::{HasHalf, JoinHalves, SplitInHalf};
use num::logic::traits::{BitIterable, LeadingZeros};

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

/// m.get_highest_bit(), x < m, y < m
///
/// This is n_mulmod_preinv from ulong_extras/mulmod_preinv.c, FLINT Dev 1.
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

/// m.get_highest_bit(), x < m
///
/// This is n_powmod_ui_preinv from ulong_extras/powmod_ui_preinv.c, FLINT Dev 1.
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
        let x = T::power_of_two(shift);
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
            T::power_of_two(shift)
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
        /// Provides functions for computing `self.pow(exp)` mod `m`. Some precomputed data is
        /// provided; this speeds up computations involving several modular exponentiations with the
        /// same modulus. The precomputed data should be obtained using `precompute_mod_pow_data`.
        impl ModPowPrecomputed<u64, $t> for $t {
            type Output = $t;
            type Data = ($t, u64);

            /// Precomputes data for modular exponentiation. See `mod_pow_precomputed` and
            /// `mod_pow_precomputed_assign`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            fn precompute_mod_pow_data(&m: &$t) -> ($t, u64) {
                let leading_zeros = LeadingZeros::leading_zeros(m);
                ($invert_limb(m << leading_zeros), leading_zeros)
            }

            /// Computes `self.pow(exp)` mod `m`. Assumes the input is already reduced mod `m`. Some
            /// precomputed data is provided; this speeds up computations involving several modular
            /// exponentiations with the same modulus. The precomputed data should be obtained using
            /// `precompute_mod_pow_data`.
            ///
            /// TODO complexity
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowPrecomputed;
            ///
            /// let data = u32::precompute_mod_pow_data(&497);
            /// assert_eq!(4u32.mod_pow_precomputed(13, 497, &data), 445);
            /// assert_eq!(5u32.mod_pow_precomputed(3, 497, &data), 125);
            /// assert_eq!(4u32.mod_pow_precomputed(100, 497, &data), 116);
            ///
            /// let data = u64::precompute_mod_pow_data(&30);
            /// assert_eq!(10u64.mod_pow_precomputed(1000, 30, &data), 10);
            /// assert_eq!(4u64.mod_pow_precomputed(9, 30, &data), 4);
            /// assert_eq!(5u64.mod_pow_precomputed(8, 30, &data), 25);
            /// ```
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
        /// Provides functions for computing `self.pow(exp)` mod `m`. Some precomputed data is
        /// provided; this speeds up computations involving several modular exponentiations with the
        /// same modulus. The precomputed data should be obtained using `precompute_mod_pow_data`.
        impl ModPowPrecomputed<u64, $t> for $t {
            type Output = $t;
            type Data = (u32, u64);

            /// Precomputes data for modular exponentiation. See `mod_pow_precomputed` and
            /// `mod_pow_precomputed_assign`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            fn precompute_mod_pow_data(&m: &$t) -> (u32, u64) {
                u32::precompute_mod_pow_data(&u32::from(m))
            }

            /// Computes `self.pow(exp)` mod `m`. Assumes the input is already reduced mod `m`. Some
            /// precomputed data is provided; this speeds up computations involving several modular
            /// exponentiations with the same modulus. The precomputed data should be obtained using
            /// `precompute_mod_pow_data`.
            ///
            /// TODO complexity
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowPrecomputed;
            ///
            /// let data = u16::precompute_mod_pow_data(&497);
            /// assert_eq!(4u16.mod_pow_precomputed(13, 497, &data), 445);
            /// assert_eq!(5u16.mod_pow_precomputed(3, 497, &data), 125);
            /// assert_eq!(4u16.mod_pow_precomputed(100, 497, &data), 116);
            ///
            /// let data = u8::precompute_mod_pow_data(&30);
            /// assert_eq!(10u8.mod_pow_precomputed(1000, 30, &data), 10);
            /// assert_eq!(4u8.mod_pow_precomputed(9, 30, &data), 4);
            /// assert_eq!(5u8.mod_pow_precomputed(8, 30, &data), 25);
            /// ```
            fn mod_pow_precomputed(self, exp: u64, m: $t, data: &(u32, u64)) -> $t {
                $t::wrapping_from(u32::from(self).mod_pow_precomputed(exp, u32::from(m), data))
            }
        }
    };
}
impl_mod_pow_precomputed_promoted!(u8);
impl_mod_pow_precomputed_promoted!(u16);

/// Provides functions for computing `self.pow(exp)` mod `m`. Some precomputed data is provided;
/// this speeds up computations involving several modular exponentiations with the same modulus. The
/// precomputed data should be obtained using `precompute_mod_pow_data`.
impl ModPowPrecomputed<u64, u128> for u128 {
    type Output = u128;
    type Data = ();

    /// Precomputes data for modular exponentiation. See `mod_pow_precomputed` and
    /// `mod_pow_precomputed_assign`.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    fn precompute_mod_pow_data(_m: &u128) {}

    /// Computes `self.pow(exp) mod `m`. Assumes the input is already reduced mod `m`. Some
    /// precomputed data is provided; this speeds up computations involving several modular
    /// exponentiations with the same modulus. The precomputed data should be obtained using
    /// `precompute_mod_pow_data`.
    ///
    /// TODO complexity
    ///
    /// # Example
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowPrecomputed;
    ///
    /// let data = u128::precompute_mod_pow_data(&497);
    /// assert_eq!(4u128.mod_pow_precomputed(13, 497, &data), 445);
    /// assert_eq!(5u128.mod_pow_precomputed(3, 497, &data), 125);
    /// assert_eq!(4u128.mod_pow_precomputed(100, 497, &data), 116);
    ///
    /// let data = u128::precompute_mod_pow_data(&30);
    /// assert_eq!(10u128.mod_pow_precomputed(1000, 30, &data), 10);
    /// assert_eq!(4u128.mod_pow_precomputed(9, 30, &data), 4);
    /// assert_eq!(5u128.mod_pow_precomputed(8, 30, &data), 25);
    /// ```
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
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    fn precompute_mod_pow_data(&m: &usize) -> (usize, u64) {
        if usize::WIDTH == u32::WIDTH {
            let (inverse, shift) = u32::precompute_mod_pow_data(&u32::wrapping_from(m));
            (usize::wrapping_from(inverse), shift)
        } else {
            let (inverse, shift) = u64::precompute_mod_pow_data(&u64::wrapping_from(m));
            (usize::wrapping_from(inverse), shift)
        }
    }

    /// Computes `self.pow(exp)` mod `m`. Assumes the input is already reduced mod `m`. Some
    /// precomputed data is provided; this speeds up computations involving several modular
    /// exponentiations with the same modulus. The precomputed data should be obtained using
    /// `precompute_mod_pow_data`.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    fn mod_pow_precomputed(self, exp: u64, m: usize, data: &(usize, u64)) -> usize {
        if usize::WIDTH == u32::WIDTH {
            let (inverse, shift) = *data;
            usize::wrapping_from(u32::wrapping_from(self).mod_pow_precomputed(
                exp,
                u32::wrapping_from(m),
                &(u32::wrapping_from(inverse), shift),
            ))
        } else {
            let (inverse, shift) = *data;
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
        /// Provides functions for replacing `self` with `self.pow(exp)` mod `m`. Some precomputed
        /// data is provided; this speeds up computations involving several modular exponentiations
        /// with the same modulus. The precomputed data should be obtained using
        /// `precompute_mod_pow_data`.
        impl ModPowPrecomputedAssign<u64, $t> for $t {
            /// Replaces `self` with `self.pow(exp)` mod `m`. Assumes the input is already reduced
            /// mod `m`. Some precomputed data is provided; this speeds up computations involving
            /// several modular exponentiations with the same modulus. The precomputed data should
            /// be obtained using `precompute_mod_pow_data`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::{
            ///     ModPowPrecomputed, ModPowPrecomputedAssign
            /// };
            ///
            /// let data = u32::precompute_mod_pow_data(&497);
            ///
            /// let mut x = 4u32;
            /// x.mod_pow_precomputed_assign(13, 497, &data);
            /// assert_eq!(x, 445);
            ///
            /// let mut x = 5u32;
            /// x.mod_pow_precomputed_assign(3, 497, &data);
            /// assert_eq!(x, 125);
            ///
            /// let mut x = 4u32;
            /// x.mod_pow_precomputed_assign(100, 497, &data);
            /// assert_eq!(x, 116);
            ///
            /// let data = u64::precompute_mod_pow_data(&30);
            ///
            /// let mut x = 10u64;
            /// x.mod_pow_precomputed_assign(1000, 30, &data);
            /// assert_eq!(x, 10);
            ///
            /// let mut x = 4u64;
            /// x.mod_pow_precomputed_assign(9, 30, &data);
            /// assert_eq!(x, 4);
            ///
            /// let mut x = 5u64;
            /// x.mod_pow_precomputed_assign(8, 30, &data);
            /// assert_eq!(x, 25);
            /// ```
            #[inline]
            fn mod_pow_precomputed_assign(&mut self, exp: u64, m: $t, data: &Self::Data) {
                *self = self.mod_pow_precomputed(exp, m, data);
            }
        }

        /// Provides functions for computing `self.pow(exp)` mod `m`.
        impl ModPow<u64> for $t {
            type Output = $t;

            /// Computes `self.pow(exp)` mod `m`. Assumes the input is already reduced mod `m`.
            ///
            /// TODO complexity
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPow;
            ///
            /// assert_eq!(4u16.mod_pow(13, 497), 445);
            /// assert_eq!(10u32.mod_pow(1000, 30), 10);
            /// ```
            #[inline]
            fn mod_pow(self, exp: u64, m: $t) -> $t {
                _simple_binary_mod_pow(self, exp, m)
            }
        }

        /// Provides functions for replacing `self` with `self.pow(exp)` mod `m`.
        impl ModPowAssign<u64> for $t {
            /// Replaces `self` with `self.pow(exp)` mod `m`. Assumes the input is already reduced
            /// mod `m`.
            ///
            /// TODO complexity
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowAssign;
            ///
            /// let mut n = 4u16;
            /// n.mod_pow_assign(13, 497);
            /// assert_eq!(n, 445);
            ///
            /// let mut n = 10u32;
            /// n.mod_pow_assign(1000, 30);
            /// assert_eq!(n, 10);
            /// ```
            #[inline]
            fn mod_pow_assign(&mut self, exp: u64, m: $t) {
                *self = _simple_binary_mod_pow(*self, exp, m);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_pow);
