// Copyright © 2025 William Youmans
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2009 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL as published by the Free Software Foundation; either version
// 3 of the License, or (at your option any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{FloorSqrt, Square};
use crate::num::conversion::traits::ExactFrom;
use crate::num::factorization::traits::IsSquare;

const IS_SQUARE_MOD64: [bool; 64] = [
    true, true, false, false, true, false, false, false, false, true, false, false, false, false,
    false, false, true, true, false, false, false, false, false, false, false, true, false, false,
    false, false, false, false, false, true, false, false, true, false, false, false, false, true,
    false, false, false, false, false, false, false, true, false, false, false, false, false,
    false, false, true, false, false, false, false, false, false,
];

const IS_SQUARE_MOD65: [bool; 65] = [
    true, true, false, false, true, false, false, false, false, true, true, false, false, false,
    true, false, true, false, false, false, false, false, false, false, false, true, true, false,
    false, true, true, false, false, false, false, true, true, false, false, true, true, false,
    false, false, false, false, false, false, false, true, false, true, false, false, false, true,
    true, false, false, false, false, true, false, false, true,
];

const IS_SQUARE_MOD63: [bool; 63] = [
    true, true, false, false, true, false, false, true, false, true, false, false, false, false,
    true, false, true, false, true, false, false, false, true, false, false, true, false, false,
    true, false, false, false, false, false, false, true, true, true, false, false, false, false,
    false, true, false, false, true, false, false, true, false, false, false, false, false, false,
    true, false, true, false, false, false, false,
];

// This is n_is_square when FLINT64 is false, from ulong_extras/is_square.c, FLINT 3.1.2.
fn is_square_u64(x: u64) -> bool {
    IS_SQUARE_MOD64[(x % 64) as usize]
        && IS_SQUARE_MOD63[(x % 63) as usize]
        && IS_SQUARE_MOD65[(x % 65) as usize]
        && x.floor_sqrt().square() == x
}

macro_rules! impl_unsigned {
    ($t: ident) => {
        impl IsSquare for $t {
            #[inline]
            fn is_square(&self) -> bool {
                is_square_u64(u64::exact_from(*self))
            }
        }
    };
}
apply_to_unsigneds!(impl_unsigned);

macro_rules! impl_signed {
    ($t: ident) => {
        impl IsSquare for $t {
            #[inline]
            fn is_square(&self) -> bool {
                false
            }
        }
    };
}
apply_to_signeds!(impl_signed);
