// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Abs, PowerOf2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::test_util::generators::signed_gen;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_nz::test_util::generators::{integer_gen, natural_gen};
use num::BigInt;
use rug;
use std::str::FromStr;

#[test]
fn test_significant_bits() {
    let test = |n, out| {
        assert_eq!(Integer::from_str(n).unwrap().significant_bits(), out);
        assert_eq!(u64::wrapping_from(BigInt::from_str(n).unwrap().bits()), out);
        assert_eq!(
            u64::from(rug::Integer::from_str(n).unwrap().significant_bits()),
            out
        );
    };
    test("0", 0);
    test("100", 7);
    test("-100", 7);
    test("1000000000000", 40);
    test("-1000000000000", 40);
}

#[test]
fn significant_bits_properties() {
    integer_gen().test_properties(|x| {
        let bits = x.significant_bits();
        assert_eq!(u64::wrapping_from(BigInt::from(&x).bits()), bits);
        assert_eq!(u64::from(rug::Integer::from(&x).significant_bits()), bits);
        assert_eq!((-&x).significant_bits(), bits);
        let x_abs = x.abs();
        assert_eq!(x_abs <= Limb::MAX, bits <= Limb::WIDTH);
        if x_abs != 0 {
            assert!(Natural::power_of_2(bits - 1) <= x_abs);
            assert!(x_abs < Natural::power_of_2(bits));
        }
    });

    natural_gen().test_properties(|n| {
        assert_eq!(Integer::from(&n).significant_bits(), n.significant_bits());
    });

    signed_gen::<SignedLimb>().test_properties(|i| {
        assert_eq!(Integer::from(i).significant_bits(), i.significant_bits());
    });
}
