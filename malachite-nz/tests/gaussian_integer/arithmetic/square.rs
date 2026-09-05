// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    AbsSquared, CheckedSqrt, Conjugate, PowerOf2, Square, SquareAssign,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_nz::gaussian_integer::{ComparableGaussianIntegerRef, GaussianInteger};
use malachite_nz::integer::Integer;
use malachite_nz::test_util::gaussian_integer::arithmetic::mul::gaussian_integer_mul_naive;
use malachite_nz::test_util::gaussian_integer::arithmetic::square::gaussian_integer_square_naive;
use malachite_nz::test_util::generators::gaussian_integer_gen;
use std::str::FromStr;

#[test]
fn test_square() {
    let test = |s, out: &str| {
        let x = GaussianInteger::from_str(s).unwrap();

        let square = x.clone().square();
        assert!(square.real.is_valid());
        assert!(square.imaginary.is_valid());
        assert_eq!(square.to_string(), out);

        let square = (&x).square();
        assert!(square.real.is_valid());
        assert!(square.imaginary.is_valid());
        assert_eq!(square.to_string(), out);

        let mut square = x;
        square.square_assign();
        assert_eq!(square.to_string(), out);
    };
    test("0", "0");
    test("1", "1");
    test("-1", "1");
    test("i", "-1");
    test("-i", "-1");
    test("1+i", "2i");
    test("1-i", "-2i");
    test("2-3i", "-5-12i");
    test("-2+3i", "-5-12i");
    test("1000000000000", "1000000000000000000000000");
    test("1000000000000i", "-1000000000000000000000000");
}

#[test]
fn square_properties() {
    gaussian_integer_gen().test_properties(|x| {
        let square = x.clone().square();
        assert!(square.real.is_valid());
        assert!(square.imaginary.is_valid());
        assert_eq!((&x).square(), square);
        let mut x_alt = x.clone();
        x_alt.square_assign();
        assert_eq!(x_alt, square);

        assert_eq!(&x * &x, square);
        assert_eq!(gaussian_integer_mul_naive(&x, &x), square);
        assert_eq!(gaussian_integer_square_naive(&x), square);
        assert_eq!((-&x).square(), square);
        assert_eq!((&x).conjugate().square(), (&square).conjugate());
        assert_eq!((&square).abs_squared(), (&x).abs_squared().square());

        // the square root recovers the principal one of x and -x, and both are all the roots
        let principal = if (&x.real, &x.imaginary) >= (&Integer::ZERO, &Integer::ZERO) {
            x.clone()
        } else {
            -&x
        };
        assert_eq!((&square).checked_sqrt(), Some(principal.clone()));
        let roots = square.checked_sqrts();
        if x == 0u32 {
            assert_eq!(roots, vec![GaussianInteger::ZERO]);
        } else {
            let mut expected = vec![-&principal, principal];
            expected.sort_by(|a, b| {
                ComparableGaussianIntegerRef(a).cmp(&ComparableGaussianIntegerRef(b))
            });
            assert_eq!(roots, expected);
        }
    });
}

#[test]
fn square_large_properties() {
    // Large inputs exercise the three-squarings path, which the default configuration rarely
    // reaches.
    let mut config = GenConfig::new();
    config.insert("mean_bits_n", 2048);
    gaussian_integer_gen().test_properties_with_config(&config, |x| {
        let square = (&x).square();
        assert_eq!(gaussian_integer_square_naive(&x), square);
        assert_eq!(x.clone().square(), square);
        // large square roots too
        assert_eq!(
            (&square).checked_sqrt().map(|r| (&r).square()),
            Some(square.clone())
        );
        let mut x_alt = x;
        x_alt.square_assign();
        assert_eq!(x_alt, square);
    });
}

#[test]
fn test_square_branch_coverage() {
    #[allow(clippy::needless_pass_by_value)]
    fn check(x: GaussianInteger) {
        let square = (&x).square();
        assert!(square.real.is_valid());
        assert!(square.imaginary.is_valid());
        assert_eq!(gaussian_integer_square_naive(&x), square);
        assert_eq!(x.clone().square(), square);
        let mut x_alt = x.clone();
        x_alt.square_assign();
        assert_eq!(x_alt, square);
        assert_eq!((&square).abs_squared(), (&x).abs_squared().square());
    }
    let gi = |real, imaginary| GaussianInteger { real, imaginary };
    let big = |bits: u64, tweak: i64| Integer::power_of_2(bits) + Integer::from(tweak);
    // - both parts fit in a signed word
    check(gi(Integer::from(i64::MAX), Integer::from(i64::MIN)));
    // - the value is purely real, and too large for the double-word path
    check(gi(big(100, 3), Integer::from(0)));
    // - the value is purely imaginary, and too large for the double-word path
    check(gi(Integer::from(0), -big(100, 3)));
    // - both parts are large and balanced, engaging the three-squarings path
    check(gi(big(1100, 3), -big(1101, 17)));
    // - the parts are unbalanced, so the general path is used
    check(gi(big(1100, 3), Integer::from(99)));
    // - the real part is too small for the three-squarings path
    check(gi(big(100, 3), -big(101, 17)));
}
