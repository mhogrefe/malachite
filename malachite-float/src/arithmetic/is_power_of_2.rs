// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::Finite;
use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_nz::platform::Limb;

const HIGH_BIT: Limb = 1 << (Limb::WIDTH - 1);

impl IsPowerOf2 for Float {
    /// Determines whether a [`Float`] is an integer power of 2.
    ///
    /// $f(x) = (\exists n \in \Z : 2^n = x)$.
    ///
    /// [`Float`]s that are NaN, infinite, or zero are not powers of 2.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::IsPowerOf2;
    /// use malachite_base::num::basic::traits::{NaN, One, OneHalf, Two};
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::NAN.is_power_of_2(), false);
    ///
    /// assert_eq!(Float::ONE.is_power_of_2(), true);
    /// assert_eq!(Float::TWO.is_power_of_2(), true);
    /// assert_eq!(Float::ONE_HALF.is_power_of_2(), true);
    /// assert_eq!(Float::from(1024).is_power_of_2(), true);
    ///
    /// assert_eq!(Float::from(3).is_power_of_2(), false);
    /// assert_eq!(Float::from(1025).is_power_of_2(), false);
    /// assert_eq!(Float::from(0.1f64).is_power_of_2(), false);
    /// ```
    fn is_power_of_2(&self) -> bool {
        match self {
            Float(Finite {
                sign: true,
                significand,
                ..
            }) => {
                let mut first = true;
                for x in significand.limbs().rev() {
                    if first {
                        if x != HIGH_BIT {
                            return false;
                        }
                        first = false;
                    } else if x != 0 {
                        return false;
                    }
                }
                true
            }
            _ => false,
        }
    }
}

pub(crate) fn float_is_signed_min<T: PrimitiveSigned>(f: &Float) -> bool {
    match f {
        Float(Finite {
            sign: false,
            exponent,
            significand,
            ..
        }) => {
            if *exponent != T::WIDTH as i32 {
                return false;
            }
            let mut first = true;
            for x in significand.limbs().rev() {
                if first {
                    if x != HIGH_BIT {
                        return false;
                    }
                    first = false;
                } else if x != 0 {
                    return false;
                }
            }
            true
        }
        _ => false,
    }
}
