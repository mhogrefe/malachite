// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::Zero;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{integer_gen, natural_gen};

#[test]
fn test_from() {
    let test = |g: GaussianInteger, out| {
        assert_eq!(g.to_string(), out);
        assert_eq!(g.imaginary, 0);
    };
    test(GaussianInteger::from(0u8), "0");
    test(GaussianInteger::from(123u8), "123");
    test(GaussianInteger::from(-123i64), "-123");
    test(
        GaussianInteger::from(u128::MAX),
        "340282366920938463463374607431768211455",
    );
    test(GaussianInteger::from(false), "0");
    test(GaussianInteger::from(true), "1");
    test(GaussianInteger::from(Natural::from(123u32)), "123");
    test(GaussianInteger::from(&Natural::from(123u32)), "123");
    test(GaussianInteger::from(Integer::from(-123)), "-123");
}

#[test]
fn from_properties() {
    integer_gen().test_properties(|x| {
        let g = GaussianInteger::from(x.clone());
        assert_eq!(g.real, x);
        assert_eq!(g.imaginary, 0);
        assert_eq!(g.to_string(), x.to_string());
    });

    natural_gen().test_properties(|x| {
        let g = GaussianInteger::from(x.clone());
        assert_eq!(g.real, x);
        assert_eq!(g.imaginary, Integer::ZERO);
    });
}
