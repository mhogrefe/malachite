// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{BellNumber, CheckedBellNumber};

// The Bell numbers grow superexponentially, so for every primitive width the entire range of
// representable values is a short hardcoded table, checked against OEIS A000110.
const BELL_NUMBERS_U8: [u8; 7] = [1, 1, 2, 5, 15, 52, 203];
const BELL_NUMBERS_U16: [u16; 10] = [1, 1, 2, 5, 15, 52, 203, 877, 4140, 21147];
const BELL_NUMBERS_U32: [u32; 16] = [
    1, 1, 2, 5, 15, 52, 203, 877, 4140, 21147, 115975, 678570, 4213597, 27644437, 190899322,
    1382958545,
];
const BELL_NUMBERS_U64: [u64; 26] = [
    1,
    1,
    2,
    5,
    15,
    52,
    203,
    877,
    4140,
    21147,
    115975,
    678570,
    4213597,
    27644437,
    190899322,
    1382958545,
    10480142147,
    82864869804,
    682076806159,
    5832742205057,
    51724158235372,
    474869816156751,
    4506715738447323,
    44152005855084346,
    445958869294805289,
    4638590332229999353,
];
const BELL_NUMBERS_U128: [u128; 43] = [
    1,
    1,
    2,
    5,
    15,
    52,
    203,
    877,
    4140,
    21147,
    115975,
    678570,
    4213597,
    27644437,
    190899322,
    1382958545,
    10480142147,
    82864869804,
    682076806159,
    5832742205057,
    51724158235372,
    474869816156751,
    4506715738447323,
    44152005855084346,
    445958869294805289,
    4638590332229999353,
    49631246523618756274,
    545717047936059989389,
    6160539404599934652455,
    71339801938860275191172,
    846749014511809332450147,
    10293358946226376485095653,
    128064670049908713818925644,
    1629595892846007606764728147,
    21195039388640360462388656799,
    281600203019560266563340426570,
    3819714729894818339975525681317,
    52868366208550447901945575624941,
    746289892095625330523099540639146,
    10738823330774692832768857986425209,
    157450588391204931289324344702531067,
    2351152507740617628200694077243788988,
    35742549198872617291353508656626642567,
];

macro_rules! impl_bell_numbers {
    ($t:ident, $bs:ident) => {
        impl CheckedBellNumber for $t {
            /// Computes the $n$th Bell number: the number of ways to partition a set of $n$
            /// elements.
            ///
            /// If the result is too large to be represented, the function returns `None`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::bell_number#checked_bell_number).
            #[inline]
            fn checked_bell_number(n: u64) -> Option<$t> {
                $bs.get(usize::try_from(n).ok()?).copied()
            }
        }

        impl BellNumber for $t {
            /// Computes the $n$th Bell number: the number of ways to partition a set of $n$
            /// elements.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if the result is too large to be represented.
            ///
            /// # Examples
            /// See [here](super::bell_number#bell_number).
            #[inline]
            fn bell_number(n: u64) -> $t {
                $t::checked_bell_number(n).unwrap()
            }
        }
    };
}
impl_bell_numbers!(u8, BELL_NUMBERS_U8);
impl_bell_numbers!(u16, BELL_NUMBERS_U16);
impl_bell_numbers!(u32, BELL_NUMBERS_U32);
impl_bell_numbers!(u64, BELL_NUMBERS_U64);
impl_bell_numbers!(u128, BELL_NUMBERS_U128);

impl CheckedBellNumber for usize {
    /// Computes the $n$th Bell number: the number of ways to partition a set of $n$ elements.
    ///
    /// If the result is too large to be represented, the function returns `None`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::bell_number#checked_bell_number).
    #[inline]
    fn checked_bell_number(n: u64) -> Option<Self> {
        BELL_NUMBERS_U64
            .get(Self::try_from(n).ok()?)
            .and_then(|&b| Self::try_from(b).ok())
    }
}

impl BellNumber for usize {
    /// Computes the $n$th Bell number: the number of ways to partition a set of $n$ elements.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if the result is too large to be represented.
    ///
    /// # Examples
    /// See [here](super::bell_number#bell_number).
    #[inline]
    fn bell_number(n: u64) -> Self {
        Self::checked_bell_number(n).unwrap()
    }
}
