// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{BellNumber, CheckedBellNumber};

#[test]
fn test_bell_number() {
    // - small values, identical across widths
    assert_eq!(u8::bell_number(0), 1);
    assert_eq!(u8::bell_number(4), 15);
    assert_eq!(u16::bell_number(9), 21147);
    assert_eq!(u32::bell_number(15), 1382958545);
    assert_eq!(u64::bell_number(25), 4638590332229999353);
    assert_eq!(
        u128::bell_number(42),
        35742549198872617291353508656626642567
    );
}

#[test]
fn checked_bell_number_properties() {
    // Each width's table ends exactly where the next value stops fitting: the last Some entry
    // agrees with the u128 table, and the next index is None.
    fn check<T: CheckedBellNumber + Copy + Into<u128>>(last: u64) {
        let x = T::checked_bell_number(last).unwrap();
        assert_eq!(x.into(), u128::checked_bell_number(last).unwrap());
        assert!(T::checked_bell_number(last + 1).is_none());
    }
    check::<u8>(6);
    check::<u16>(9);
    check::<u32>(15);
    check::<u64>(25);
    // - the u128 table's own edge, and far-out-of-range inputs
    assert!(u128::checked_bell_number(43).is_none());
    assert!(u8::checked_bell_number(1 << 40).is_none());
}

#[test]
#[should_panic]
fn bell_number_fail() {
    u8::bell_number(7);
}
