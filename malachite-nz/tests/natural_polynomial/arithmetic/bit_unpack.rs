// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::ModPowerOf2IsReduced;
use malachite_base::polynomial::{BitPack, BitUnpack};
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::{
    natural_polynomial_unsigned_pair_gen_var_4, natural_unsigned_pair_gen_var_7,
};
use malachite_nz::test_util::natural_polynomial::arithmetic::bit_unpack::bit_unpack_naive;

#[test]
fn test_bit_unpack() {
    let test = |s, bits, out| {
        let n = Natural::from_str(s).unwrap();
        let p = NaturalPolynomial::bit_unpack(&n, bits);
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert_eq!(NaturalPolynomial::bit_unpack(n.clone(), bits), p);
        assert_eq!(bit_unpack_naive(&n, bits), p);
    };
    test("0", 1, "0");
    test("0", 8, "0");
    test("1", 100, "1");
    test("197121", 8, "3*x^2+2*x+1");
    test("65535", 8, "255*x+255");
    // A packing with overlapping fields is not recovered.
    test("257000", 8, "3*x^2+235*x+232");
    test("5", 1, "x^2+1");
    test(
        "6277101735386680763835789423207666416102355444464034512896",
        64,
        "x^3",
    );
}

#[test]
#[should_panic]
fn bit_unpack_fail() {
    NaturalPolynomial::bit_unpack(Natural::from(5u32), 0);
}

#[test]
fn bit_unpack_properties() {
    natural_unsigned_pair_gen_var_7::<u64>().test_properties(|(n, bits)| {
        let p = NaturalPolynomial::bit_unpack(&n, bits);
        assert!(p.is_valid());
        assert_eq!(NaturalPolynomial::bit_unpack(n.clone(), bits), p);
        assert_eq!(bit_unpack_naive(&n, bits), p);

        // The coefficients fit in their fields, and packing them gives n back.
        assert!(p.mod_power_of_2_is_reduced(bits));
        assert_eq!(p.bit_pack(bits), n);
    });

    natural_polynomial_unsigned_pair_gen_var_4().test_properties(|(p, bits)| {
        // Unpacking inverts packing when the coefficients fit in their fields.
        assert_eq!(NaturalPolynomial::bit_unpack((&p).bit_pack(bits), bits), p);
    });
}
