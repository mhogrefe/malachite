// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::Finite;
use alloc::vec;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{
    DivisibleByPowerOf2, NegModPowerOf2, PowerOf2, ShrRound,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

const HIGH_BIT: Limb = 1 << (Limb::WIDTH - 1);

impl Float {
    /// Returns an approximation of a real number, given the number's bits. To avoid troublesome
    /// edge cases, the number should not be a dyadic rational (and the iterator of bits should
    /// therefore be infinite, and not eventually 0 or 1). Given this assumption, the rounding mode
    /// `Exact` should never be passed.
    ///
    /// The approximation has precision `prec` and is rounded according to the provided rounding
    /// mode.
    ///
    /// This function reads `prec + z` bits, or `prec + z + 1` bits if `rm` is `Nearest`, where `z`
    /// is the number of leading false bits in `bits`.
    ///
    /// This function always produces a value in the interval $[1/2,1]$. In particular, it never
    /// overflows or underflows.
    ///
    /// $$
    /// f((x_k),p,m) = C+\varepsilon,
    /// $$
    /// where
    /// $$
    /// C=\sum_{k=0}^\infty x_k 2^{-(k+1)}.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 C\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 C\rfloor-p}$.
    ///
    /// The output has precision `prec`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or `rm` is `Exact`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// // Produces 10100100010000100000...
    /// struct Bits {
    ///     b: bool,
    ///     k: usize,
    ///     j: usize,
    /// }
    ///
    /// impl Iterator for Bits {
    ///     type Item = bool;
    ///
    ///     fn next(&mut self) -> Option<bool> {
    ///         Some(if self.b {
    ///             self.b = false;
    ///             self.j = self.k;
    ///             true
    ///         } else {
    ///             self.j -= 1;
    ///             if self.j == 0 {
    ///                 self.k += 1;
    ///                 self.b = true;
    ///             }
    ///             false
    ///         })
    ///     }
    /// }
    ///
    /// impl Bits {
    ///     fn new() -> Bits {
    ///         Bits {
    ///             b: true,
    ///             k: 1,
    ///             j: 1,
    ///         }
    ///     }
    /// }
    ///
    /// let (c, o) = Float::non_dyadic_from_bits_prec_round(Bits::new(), 100, Floor);
    /// assert_eq!(c.to_string(), "0.6416325606551538662938427702254");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::non_dyadic_from_bits_prec_round(Bits::new(), 100, Ceiling);
    /// assert_eq!(c.to_string(), "0.641632560655153866293842770226");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn non_dyadic_from_bits_prec_round<I: Iterator<Item = bool>>(
        mut bits: I,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        assert_ne!(prec, 0);
        assert_ne!(rm, Exact);
        let len = usize::exact_from(prec.shr_round(Limb::LOG_WIDTH, Ceiling).0);
        let mut limbs = vec![0; len];
        let mut limbs_it = limbs.iter_mut().rev();
        let mut x = limbs_it.next().unwrap();
        let mut mask = HIGH_BIT;
        let mut seen_one = false;
        let mut exponent: i32 = 0;
        let mut remaining = prec;
        for b in &mut bits {
            if !seen_one {
                if b {
                    seen_one = true;
                } else {
                    exponent = exponent.checked_sub(1).unwrap();
                    continue;
                }
            }
            if b {
                *x |= mask;
            }
            remaining -= 1;
            if remaining == 0 {
                break;
            }
            if mask == 1 {
                x = limbs_it.next().unwrap();
                mask = HIGH_BIT;
            } else {
                mask >>= 1;
            }
        }
        let mut significand = Natural::from_owned_limbs_asc(limbs);
        let increment = rm == Up || rm == Ceiling || (rm == Nearest && bits.next() == Some(true));
        if increment {
            significand +=
                Natural::from(Limb::power_of_2(prec.neg_mod_power_of_2(Limb::LOG_WIDTH)));
            if !significand
                .significant_bits()
                .divisible_by_power_of_2(Limb::LOG_WIDTH)
            {
                significand >>= 1;
                exponent += 1;
            }
        }
        (
            Float(Finite {
                sign: true,
                exponent,
                precision: prec,
                significand,
            }),
            if increment { Greater } else { Less },
        )
    }

    /// Returns an approximation of a real number, given the number's bits. To avoid troublesome
    /// edge cases, the number should not be a dyadic rational (and the iterator of bits should
    /// therefore be infinite, and not eventually 0 or 1).
    ///
    /// The approximation has precision `prec` and is rounded according to the `Nearest` rounding
    /// mode.
    ///
    /// This function reads `prec + z + 1` bits, where `z` is the number of leading false bits in
    /// `bits`.
    ///
    /// This function always produces a value in the interval $[1/2,1]$. In particular, it never
    /// overflows or underflows.
    ///
    /// $$
    /// f((x_k),p,m) = C+\varepsilon,
    /// $$
    /// where
    /// $$
    /// C=\sum_{k=0}^\infty x_k 2^{-(k+1)}
    /// $$
    /// and $|\varepsilon| < 2^{\lfloor\log_2 C\rfloor-p}$.
    ///
    /// The output has precision `prec`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// // Produces 10100100010000100000...
    /// struct Bits {
    ///     b: bool,
    ///     k: usize,
    ///     j: usize,
    /// }
    ///
    /// impl Iterator for Bits {
    ///     type Item = bool;
    ///
    ///     fn next(&mut self) -> Option<bool> {
    ///         Some(if self.b {
    ///             self.b = false;
    ///             self.j = self.k;
    ///             true
    ///         } else {
    ///             self.j -= 1;
    ///             if self.j == 0 {
    ///                 self.k += 1;
    ///                 self.b = true;
    ///             }
    ///             false
    ///         })
    ///     }
    /// }
    ///
    /// impl Bits {
    ///     fn new() -> Bits {
    ///         Bits {
    ///             b: true,
    ///             k: 1,
    ///             j: 1,
    ///         }
    ///     }
    /// }
    ///
    /// let (c, o) = Float::non_dyadic_from_bits_prec(Bits::new(), 1);
    /// assert_eq!(c.to_string(), "0.5");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::non_dyadic_from_bits_prec(Bits::new(), 10);
    /// assert_eq!(c.to_string(), "0.642");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::non_dyadic_from_bits_prec(Bits::new(), 100);
    /// assert_eq!(c.to_string(), "0.6416325606551538662938427702254");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn non_dyadic_from_bits_prec<I: Iterator<Item = bool>>(
        bits: I,
        prec: u64,
    ) -> (Float, Ordering) {
        Float::non_dyadic_from_bits_prec_round(bits, prec, Nearest)
    }
}
