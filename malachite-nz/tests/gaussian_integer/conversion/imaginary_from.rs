// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{ImaginaryFrom, ImaginaryInto};
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{integer_gen, natural_gen};

#[test]
fn test_imaginary_from() {
    let test = |g: GaussianInteger, out| {
        assert_eq!(g.to_string(), out);
        assert_eq!(g.real, 0);
    };
    test(GaussianInteger::imaginary_from(0u8), "0");
    test(GaussianInteger::imaginary_from(1u8), "i");
    test(GaussianInteger::imaginary_from(-1i8), "-i");
    test(GaussianInteger::imaginary_from(123u8), "123i");
    test(GaussianInteger::imaginary_from(-123i64), "-123i");
    test(GaussianInteger::imaginary_from(false), "0");
    test(GaussianInteger::imaginary_from(true), "i");
    test(
        GaussianInteger::imaginary_from(Natural::from(123u32)),
        "123i",
    );
    test(
        GaussianInteger::imaginary_from(&Natural::from(123u32)),
        "123i",
    );
    test(
        GaussianInteger::imaginary_from(Integer::from(-123)),
        "-123i",
    );
    let g: GaussianInteger = 9u8.imaginary_into();
    assert_eq!(g.to_string(), "9i");
}

#[test]
fn imaginary_from_properties() {
    integer_gen().test_properties(|x| {
        let g = GaussianInteger::imaginary_from(x.clone());
        assert_eq!(g.real, Integer::ZERO);
        assert_eq!(g.imaginary, x);
    });

    natural_gen().test_properties(|x| {
        let g = GaussianInteger::imaginary_from(x.clone());
        assert_eq!(g.real, Integer::ZERO);
        assert_eq!(g.imaginary, x);
    });
}
