// Copyright © 2026 Mikhail Hogrefe
//
// Implementations of traits for NonZero* types by b4D8.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::num::*;

/// Provides the constant 0.
#[allow(clippy::declare_interior_mutable_const)]
pub trait Zero {
    const ZERO: Self;
}

/// Provides the constant 1.
#[allow(clippy::declare_interior_mutable_const)]
pub trait One {
    const ONE: Self;
}

/// Provides the constant 2.
#[allow(clippy::declare_interior_mutable_const)]
pub trait Two {
    const TWO: Self;
}

/// Provides the constant -1.
#[allow(clippy::declare_interior_mutable_const)]
pub trait NegativeOne {
    const NEGATIVE_ONE: Self;
}

/// Provides the constant 1/2.
#[allow(clippy::declare_interior_mutable_const)]
pub trait OneHalf {
    const ONE_HALF: Self;
}

/// Provides the constant -0.
#[allow(clippy::declare_interior_mutable_const)]
pub trait NegativeZero {
    const NEGATIVE_ZERO: Self;
}

/// Provides the constant (positive) Infinity.
#[allow(clippy::declare_interior_mutable_const)]
pub trait Infinity {
    const INFINITY: Self;
}

/// Provides the constant -Infinity.
#[allow(clippy::declare_interior_mutable_const)]
pub trait NegativeInfinity {
    const NEGATIVE_INFINITY: Self;
}

/// Provides the constant NaN.
#[allow(clippy::declare_interior_mutable_const)]
pub trait NaN {
    const NAN: Self;
}

/// Provides the Prouhet-Thue-Morse constant, whose bits are the Thue-Morse sequence.
pub trait ProuhetThueMorseConstant {
    const PROUHET_THUE_MORSE_CONSTANT: Self;
}

/// Provides the prime constant, whose $n$th bit (starting from $n=1$) is true if and only if $n$ is
/// prime.
pub trait PrimeConstant {
    const PRIME_CONSTANT: Self;
}

/// Provides $\ln 2$.
pub trait Ln2 {
    const LN_2: Self;
}

/// Provides $\log_2 e$.
pub trait Log2E {
    const LOG_2_E: Self;
}

/// Provides $\sqrt{2}$.
pub trait Sqrt2 {
    const SQRT_2: Self;
}

/// Provides $\sqrt{3}$.
pub trait Sqrt3 {
    const SQRT_3: Self;
}

/// Provides $\sqrt{2}/2=\sqrt{1/2}=1/\sqrt{2}$.
pub trait Sqrt2Over2 {
    const SQRT_2_OVER_2: Self;
}

/// Provides $\sqrt{3}/3=\sqrt{1/3}=1/\sqrt{3}$.
pub trait Sqrt3Over3 {
    const SQRT_3_OVER_3: Self;
}

/// Provides $\varphi$, the golden ratio.
pub trait Phi {
    const PHI: Self;
}

/// Provides $\pi$.
pub trait Pi {
    const PI: Self;
}

/// Provides $\tau=2\pi$.
pub trait Tau {
    const TAU: Self;
}

/// Provides $\pi/2$.
pub trait PiOver2 {
    const PI_OVER_2: Self;
}

/// Provides $\pi/3$.
pub trait PiOver3 {
    const PI_OVER_3: Self;
}

/// Provides $\pi/4$.
pub trait PiOver4 {
    const PI_OVER_4: Self;
}

/// Provides $\pi/6$.
pub trait PiOver6 {
    const PI_OVER_6: Self;
}

/// Provides $\pi/8$.
pub trait PiOver8 {
    const PI_OVER_8: Self;
}

/// Provides $1/\pi$.
pub trait OneOverPi {
    const ONE_OVER_PI: Self;
}

/// Provides $\sqrt{\pi}$.
pub trait SqrtPi {
    const SQRT_PI: Self;
}

/// Provides $1/\sqrt{\pi}$.
pub trait OneOverSqrtPi {
    const ONE_OVER_SQRT_PI: Self;
}

/// Provides $1/\sqrt{\tau}=1/\sqrt{2\pi}$.
pub trait OneOverSqrtTau {
    const ONE_OVER_SQRT_TAU: Self;
}

/// Provides $2/\pi$.
pub trait TwoOverPi {
    const TWO_OVER_PI: Self;
}

/// Provides $2/\sqrt{\pi}$.
pub trait TwoOverSqrtPi {
    const TWO_OVER_SQRT_PI: Self;
}

// Implementation for `NonZero*` types:
// - `One` and `Two` for both signed and unsigned variants
// - `NegativeOne` for the signed variant
macro_rules! impl_non_zero {
    ($($t:ident),+) => {
        $(
            impl One for $t {
                const ONE: Self = match Self::new(1) {
                    Some(v) => v,
                    None => unreachable!() // 1 is a valid nonzero value
                };
            }

            impl Two for $t {
                const TWO: Self = match Self::new(2) {
                    Some(v) => v,
                    None => unreachable!() // 2 is a valid nonzero value
                };
            }
        )+
    };
    ($($u:ident && $i:ident),+) => {
        $(
            impl_non_zero!($u, $i);

            impl NegativeOne for $i {
                const NEGATIVE_ONE: Self = match Self::new(-1) {
                    Some(v) => v,
                    None => unreachable!() // -1 is a valid non zero value
                };
            }
        )+
    }
}

impl_non_zero!(
    NonZeroUsize && NonZeroIsize,
    NonZeroU128 && NonZeroI128,
    NonZeroU64 && NonZeroI64,
    NonZeroU32 && NonZeroI32,
    NonZeroU16 && NonZeroI16,
    NonZeroU8 && NonZeroI8
);
