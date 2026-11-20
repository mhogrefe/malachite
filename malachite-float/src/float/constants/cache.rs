// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// With the `std` feature, a cache, kept per thread, of the most accurate approximations of pi and
// ln 2 computed so far. Each approximation is stored unrounded, with the number of its bits that
// are known to be correct, which is what Ziv's loop of the constant decided to round it on; a later
// request is answered from it whenever the same rounding test passes for the requested precision
// and rounding mode, and the result is then the same as that of a fresh computation. The logarithm,
// the exponential, and the trigonometric functions take these constants on every call, so at a low
// precision the cache saves about half of the time of a logarithm.
//
// Without `std` there is no thread-local storage, and every request is computed.

use crate::Float;
#[cfg(feature = "std")]
use crate::big_exponent_float::BigExponentFloat;
#[cfg(feature = "std")]
use crate::big_exponent_float::exponential::exp::finite_can_round;
#[cfg(feature = "std")]
use core::cell::RefCell;
#[cfg(feature = "std")]
use core::cmp::Ordering;
#[cfg(feature = "std")]
use malachite_base::rounding_modes::RoundingMode;
#[cfg(feature = "std")]
use malachite_nz::natural::arithmetic::float::round::float_can_round;

// An unrounded approximation of pi, and the number of its bits known to be correct.
#[cfg(feature = "std")]
type PiApproximation = (Float, u64);

// An unrounded approximation of ln 2 that was computed with working precision `wp`, and the number
// of its bits known to be correct: (approximation, correct bits, `wp`).
#[cfg(feature = "std")]
type Ln2Approximation = (BigExponentFloat, u64, u64);

#[cfg(feature = "std")]
std::thread_local! {
    static PI: RefCell<Option<PiApproximation>> = const { RefCell::new(None) };
    static LN_2: RefCell<Option<Ln2Approximation>> = const { RefCell::new(None) };
}

// pi rounded to `prec` bits with `rm`, if the cached approximation decides the rounding.
#[cfg(feature = "std")]
pub(crate) fn pi_from_cache(prec: u64, rm: RoundingMode) -> Option<(Float, Ordering)> {
    // `try_with` fails only while the thread is being torn down; then nothing is cached.
    PI.try_with(|cache| {
        let cache = cache.borrow();
        let (pi, err_prec) = cache.as_ref()?;
        float_can_round(pi.significand_ref().unwrap(), *err_prec, prec, rm)
            .then(|| Float::from_float_prec_round_ref(pi, prec, rm))
    })
    .ok()
    .flatten()
}

// Keeps an approximation of pi with `err_prec` correct bits, unless a more accurate one is cached.
#[cfg(feature = "std")]
pub(crate) fn pi_to_cache(pi: Float, err_prec: u64) {
    let _ = PI.try_with(|cache| {
        let mut cache = cache.borrow_mut();
        if cache
            .as_ref()
            .is_none_or(|&(_, cached_err_prec)| cached_err_prec < err_prec)
        {
            *cache = Some((pi, err_prec));
        }
    });
}

// ln 2 rounded to `prec` bits with `rm`, if the cached approximation decides the rounding.
#[cfg(feature = "std")]
pub(crate) fn ln_2_from_cache(prec: u64, rm: RoundingMode) -> Option<(Float, Ordering)> {
    LN_2.try_with(|cache| {
        let cache = cache.borrow();
        let (ln_2, err_prec, working_prec) = cache.as_ref()?;
        finite_can_round(ln_2, *err_prec, *working_prec, prec, rm)
            .then(|| Float::from_big_exponent_float_prec_round_ref(ln_2, prec, rm))
    })
    .ok()
    .flatten()
}

// Keeps an approximation of ln 2 with `err_prec` correct bits that was computed with working
// precision `working_prec`, unless a more accurate one is cached.
#[cfg(feature = "std")]
pub(crate) fn ln_2_to_cache(ln_2: BigExponentFloat, err_prec: u64, working_prec: u64) {
    let _ = LN_2.try_with(|cache| {
        let mut cache = cache.borrow_mut();
        if cache
            .as_ref()
            .is_none_or(|&(_, cached_err_prec, _)| cached_err_prec < err_prec)
        {
            *cache = Some((ln_2, err_prec, working_prec));
        }
    });
}

crate_test_fn! {
    // The numbers of correct bits of the cached approximations of pi and ln 2 in this thread, or
    // `None` for a constant that is not cached.
    cached_constant_precisions() -> (Option<u64>, Option<u64>) {
        #[cfg(feature = "std")]
        {
            (
                PI.try_with(|c| c.borrow().as_ref().map(|&(_, e)| e)).ok().flatten(),
                LN_2.try_with(|c| c.borrow().as_ref().map(|&(_, e, _)| e)).ok().flatten(),
            )
        }
        #[cfg(not(feature = "std"))]
        {
            (None, None)
        }
    }
}

impl Float {
    /// Frees the memory that this thread uses to remember approximations of $\pi$ and $\ln 2$.
    ///
    /// With the `std` feature, [`Float::pi_prec_round`] and [`Float::ln_2_prec_round`] keep, for
    /// each thread, the most accurate approximation of their constant that they have computed, and
    /// answer later requests at the same or a lower precision from it. The many functions that use
    /// these constants, such as the logarithm and the trigonometric functions, then avoid
    /// recomputing them on every call. The memory that this takes is about that of the most precise
    /// constant requested so far; this function frees it. Results do not depend on the cache.
    ///
    /// Without the `std` feature, nothing is remembered and this function does nothing.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let pi = Float::pi_prec(1000).0;
    /// Float::clear_constant_cache();
    /// assert_eq!(Float::pi_prec(1000).0, pi);
    /// ```
    pub fn clear_constant_cache() {
        #[cfg(feature = "std")]
        {
            let _ = PI.try_with(|cache| cache.borrow_mut().take());
            let _ = LN_2.try_with(|cache| cache.borrow_mut().take());
        }
    }
}
