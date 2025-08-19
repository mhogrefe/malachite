// Copyright © 2025 William Youmans
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL as published by the Free Software Foundation; either version
// 3 of the License, or (at your option any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::factorization::traits::IsSquare;
use malachite_base::test_util::generators::{signed_gen_var_4, unsigned_pair_gen_var_6};

#[test]
fn is_square_properties() {
    // test negative signed integers are non square and positive signed
    // integer squares are squares.
    signed_gen_var_4::<i32>().test_properties(|x| {
        let x = i64::from(x);
        assert!(!x.is_square());
        assert!((x * x).is_square());
    });

    // 1 < x < 2^32 avoids overflow and consecutive squares (0, 1).
    unsigned_pair_gen_var_6::<u32, u64>().test_properties(|(y, x)| {
        // test unsigned squares
        let sqr = x * x;
        assert!(sqr.is_square());

        // test non squares in interval (x^2, (x+1)^2)
        let non_sqr = sqr + (u64::from(y) % (2 * x)) + 1;
        assert!(!non_sqr.is_square());
    });
}

#[test]
fn test_is_square() {
    assert!(0u8.is_square());
    assert!(0u16.is_square());
    assert!(0u32.is_square());
    assert!(0u64.is_square());

    assert!(1u64.is_square());
    assert!(4u64.is_square());
    assert!(9u64.is_square());
    assert!(16u64.is_square());
    assert!(25u64.is_square());

    assert!(0i8.is_square());
    assert!(0i16.is_square());
    assert!(0i32.is_square());
    assert!(0i64.is_square());

    assert!(1i64.is_square());
    assert!(4i64.is_square());
    assert!(9i64.is_square());
    assert!(16i64.is_square());
    assert!(25i64.is_square());

    assert!(!(-1i64).is_square());
    assert!(!(-4i64).is_square());
    assert!(!(-9i64).is_square());
    assert!(!(-16i64).is_square());
    assert!(!(-25i64).is_square());
}
