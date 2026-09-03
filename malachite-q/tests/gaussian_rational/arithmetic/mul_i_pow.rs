// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    AbsSquared, CanonicalUnitIPow, CanonicalizeUnit, DivI, ModPowerOf2, MulI, MulIPow,
    MulIPowAssign,
};
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::generators::{
    gaussian_rational_gen, gaussian_rational_unsigned_pair_gen,
};
use std::str::FromStr;

#[test]
fn test_mul_i_pow() {
    let test = |s, k, out| {
        let x = GaussianRational::from_str(s).unwrap();

        let mut mut_x = x.clone();
        mut_x.mul_i_pow_assign(k);
        assert_eq!(mut_x.to_string(), out);
        assert!(mut_x.real.is_valid());
        assert!(mut_x.imaginary.is_valid());

        assert_eq!(x.clone().mul_i_pow(k).to_string(), out);
        assert_eq!((&x).mul_i_pow(k).to_string(), out);
    };
    test("0", 0, "0");
    test("0", 1, "0");
    test("0", 3, "0");
    test("1", 0, "1");
    test("1", 1, "i");
    test("1", 2, "-1");
    test("1", 3, "-i");
    test("1", 4, "1");
    test("1", 5, "i");
    test("1", 100, "1");
    test("1", 101, "i");
    test("1", 102, "-1");
    test("1", 103, "-i");
    test("2+3i", 0, "2+3i");
    test("2+3i", 1, "-3+2i");
    test("2+3i", 2, "-2-3i");
    test("2+3i", 3, "3-2i");
    test("2+3i", 4, "2+3i");
    test("2+3i", 5, "-3+2i");
    test("2+3i", 1000000000001, "-3+2i");
    test("2+3i", u64::MAX, "3-2i");
    test("-5-7i", 1, "7-5i");
    test("-5-7i", 3, "-7+5i");
}

// What `mul_i_pow` should do for an exponent already reduced modulo 4.
fn expected(x: &GaussianRational, k: u64) -> GaussianRational {
    match k {
        0 => x.clone(),
        1 => x.mul_i(),
        2 => -x,
        _ => x.div_i(),
    }
}

#[test]
fn mul_i_pow_properties() {
    gaussian_rational_unsigned_pair_gen::<u64>().test_properties(|(x, k)| {
        let mut mut_x = x.clone();
        mut_x.mul_i_pow_assign(k);
        assert!(mut_x.real.is_valid());
        assert!(mut_x.imaginary.is_valid());
        let y = mut_x;

        let y_alt = (&x).mul_i_pow(k);
        assert!(y_alt.real.is_valid());
        assert!(y_alt.imaginary.is_valid());
        assert_eq!(y_alt, y);
        assert_eq!(x.clone().mul_i_pow(k), y);

        assert_eq!(y, expected(&x, k.mod_power_of_2(2)));
        assert_eq!((&y).abs_squared(), (&x).abs_squared());
        // only the exponent modulo 4 matters (wrapping preserves the residue)
        assert_eq!((&x).mul_i_pow(k.wrapping_add(4)), y);
        // i^(-k) = i^(3k) undoes i^k
        assert_eq!((&y).mul_i_pow(k.wrapping_mul(3)), x);
        assert_eq!((&y).mul_i_pow(k).mul_i_pow(k).mul_i_pow(k), x);
    });

    gaussian_rational_gen().test_properties(|x| {
        assert_eq!((&x).mul_i_pow(0), x);
        assert_eq!((&x).mul_i_pow(1), (&x).mul_i());
        assert_eq!((&x).mul_i_pow(2), -&x);
        assert_eq!((&x).mul_i_pow(3), (&x).div_i());
        // FLINT's definition of the canonical unit form
        let k = x.canonical_unit_i_pow();
        assert_eq!((&x).mul_i_pow(k), (&x).canonicalize_unit());
    });
}
