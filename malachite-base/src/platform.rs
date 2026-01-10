// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

#[cfg(not(feature = "std"))]
#[macro_export]
macro_rules! fma {
    ($a: expr, $b: expr, $c: expr) => {{ libm::fma($a, $b, $c) }};
}

#[cfg(feature = "std")]
#[macro_export]
macro_rules! fma {
    ($a: expr, $b: expr, $c: expr) => {{ $a.mul_add($b, $c) }};
}

pub use fma;

#[cfg(not(feature = "std"))]
#[macro_export]
macro_rules! round_even {
    ($a: expr) => {{ libm::roundeven($a) }};
}

#[cfg(feature = "std")]
#[macro_export]
macro_rules! round_even {
    ($a: expr) => {{ $a.round_ties_even() }};
}

pub use round_even;
