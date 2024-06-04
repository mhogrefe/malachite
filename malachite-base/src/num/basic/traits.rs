// Copyright © 2024 Mikhail Hogrefe
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
