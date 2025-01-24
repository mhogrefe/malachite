// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Abs, CheckedSqrt, Square, SquareAssign};
use malachite_base::test_util::generators::signed_gen_var_10;
use malachite_nz::integer::Integer;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_nz::test_util::generators::{integer_gen, integer_pair_gen, natural_gen};
use std::str::FromStr;

#[test]
fn test_square() {
    let test = |x, out| {
        let u = Integer::from_str(x).unwrap();

        assert_eq!(u.clone().square().to_string(), out);
        assert_eq!((&u).square().to_string(), out);

        let mut x = u;
        x.square_assign();
        assert_eq!(x.to_string(), out);
    };
    test("0", "0");
    test("1", "1");
    test("2", "4");
    test("3", "9");
    test("10", "100");
    test("123", "15129");
    test("1000", "1000000");
    test("123456789", "15241578750190521");

    test("-1", "1");
    test("-2", "4");
    test("-3", "9");
    test("-10", "100");
    test("-123", "15129");
    test("-1000", "1000000");
    test("-123456789", "15241578750190521");
}

#[test]
fn square_properties() {
    integer_gen().test_properties(|x| {
        let square = (&x).square();
        assert!(square.is_valid());

        let mut mut_x = x.clone();
        mut_x.square_assign();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, square);

        assert_eq!(&x * &x, square);
        assert_eq!((-&x).square(), square);
        assert!(square >= 0);
        assert!(square >= x);

        assert_eq!(square.checked_sqrt(), Some(x.abs()));
    });

    integer_pair_gen().test_properties(|(x, y)| {
        let x_squared = (&x).square();
        let y_squared = (&y).square();
        let xy = &x * &y;
        assert_eq!((&x + &y).square(), &x_squared + &y_squared + (&xy << 1));
        assert_eq!((&x - &y).square(), &x_squared + &y_squared - (&xy << 1));
        assert_eq!(xy.square(), x_squared * y_squared);
    });

    natural_gen().test_properties(|x| {
        assert_eq!((&x).square(), Integer::from(x).square());
    });

    signed_gen_var_10::<Limb, SignedLimb>().test_properties(|x| {
        assert_eq!(x.square(), Integer::from(x).square());
    });
}
