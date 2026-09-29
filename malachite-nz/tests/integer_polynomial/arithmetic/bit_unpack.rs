// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::polynomial::{BitPack, BitUnpack};
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::generators::{
    integer_polynomial_unsigned_pair_gen_var_4, integer_unsigned_pair_gen_var_6,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::bit_unpack::bit_unpack_naive;

#[test]
fn test_bit_unpack() {
    let test = |s, bits, out| {
        let n = Integer::from_str(s).unwrap();
        let p = IntegerPolynomial::bit_unpack(&n, bits);
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert_eq!(IntegerPolynomial::bit_unpack(n.clone(), bits), p);
        assert_eq!(bit_unpack_naive(&n, bits), p);
    };
    test("0", 1, "0");
    test("0", 8, "0");
    test("197121", 8, "3*x^2+2*x+1");
    // A field whose top bit is set is negative, and borrows from the field above.
    test("196097", 8, "3*x^2-2*x+1");
    test("-196097", 8, "-3*x^2+2*x-1");
    test("127", 8, "127");
    test("128", 8, "x-128");
    test("-128", 8, "-x+128");
    test("255", 8, "x-1");
    test("-1", 8, "-1");
    test("1", 1, "x-1");
    test("2", 1, "x^2-x");
    test(
        "-6277101735386680763835789423207666416102355444464034512896",
        64,
        "-x^3",
    );
}

#[test]
#[should_panic]
fn bit_unpack_fail() {
    IntegerPolynomial::bit_unpack(Integer::from(5), 0);
}

#[test]
fn bit_unpack_properties() {
    integer_unsigned_pair_gen_var_6::<u64>().test_properties(|(n, bits)| {
        let p = IntegerPolynomial::bit_unpack(&n, bits);
        assert!(p.is_valid());
        assert_eq!(IntegerPolynomial::bit_unpack(n.clone(), bits), p);
        assert_eq!(bit_unpack_naive(&n, bits), p);

        // The coefficients lie in [-2^(bits - 1), 2^(bits - 1)], and packing them gives n back.
        let half = Integer::power_of_2(bits - 1);
        for c in p.coefficients_asc() {
            assert!(-&half <= *c && *c <= half);
        }
        assert_eq!((&p).bit_pack(bits), n);
        // Negating n negates the result.
        assert_eq!(IntegerPolynomial::bit_unpack(-&n, bits), -p);
    });

    integer_polynomial_unsigned_pair_gen_var_4().test_properties(|(p, bits)| {
        // Unpacking inverts packing when every coefficient's absolute value is less than 2^(bits -
        // 1).
        assert_eq!(IntegerPolynomial::bit_unpack((&p).bit_pack(bits), bits), p);
    });
}
