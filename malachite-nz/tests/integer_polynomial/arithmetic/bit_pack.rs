// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::num::conversion::traits::PowerOf2Digits;
use malachite_base::polynomial::{BitPack, MulPowerOfX, Polynomial};
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{
    integer_polynomial_pair_gen, integer_polynomial_unsigned_pair_gen_var_1,
    integer_polynomial_unsigned_pair_gen_var_4,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::bit_pack::bit_pack_naive;

#[test]
fn test_bit_pack() {
    let test = |s, bits, out| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let n = (&p).bit_pack(bits);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);
        assert_eq!(p.clone().bit_pack(bits), n);
        assert_eq!(bit_pack_naive(&p, bits), n);
    };
    test("0", 0, "0");
    test("0", 10, "0");
    // With 0 bits, this is p(1).
    test("3*x^2-2*x+1", 0, "2");
    test("5", 8, "5");
    test("-5", 8, "-5");
    test("3*x^2+2*x+1", 8, "197121");
    // A negative coefficient borrows from the field above it.
    test("3*x^2-2*x+1", 8, "196097");
    test("-3*x^2+2*x-1", 8, "-196097");
    // The sign is the sign of the leading coefficient.
    test("-x^2+255*x+255", 8, "-1");
    test("x^2-255*x-255", 8, "1");
    // Coefficients wider than the fields overlap.
    test("1000*x+1000", 8, "257000");
    test(
        "x^3",
        64,
        "6277101735386680763835789423207666416102355444464034512896",
    );
    test(
        "-1000000000000000000000*x+1000000000000000000000",
        70,
        "-1180591620717411303423000000000000000000000",
    );
}

#[test]
fn bit_pack_properties() {
    integer_polynomial_unsigned_pair_gen_var_1().test_properties(|(p, bits)| {
        let n = (&p).bit_pack(bits);
        assert!(n.is_valid());
        assert_eq!(p.clone().bit_pack(bits), n);
        assert_eq!(bit_pack_naive(&p, bits), n);

        // Multiplying by x shifts left by one field.
        assert_eq!((&p).mul_power_of_x(1).bit_pack(bits), &n << bits);
        // It commutes with negation.
        assert_eq!((-&p).bit_pack(bits), -&n);
    });

    integer_polynomial_unsigned_pair_gen_var_4().test_properties(|(p, bits)| {
        // When the coefficients fit in their fields, the sign of the result is the sign of the
        // leading coefficient.
        let n = (&p).bit_pack(bits);
        assert_eq!(n.sign(), p.leading_coefficient().sign());
        // Nonnegative coefficients are the result's digits in base 2^bits.
        if p.coefficients_asc().iter().all(|c| *c >= 0u32) {
            assert_eq!(
                Natural::from_power_of_2_digits_asc(
                    bits,
                    p.coefficients_asc()
                        .iter()
                        .map(|c| c.unsigned_abs_ref().clone())
                ),
                Some(Natural::try_from(n).unwrap())
            );
        }
    });

    integer_polynomial_pair_gen().test_properties(|(p, q)| {
        // Packing is linear.
        for bits in [0, 1, 10, 64, 100] {
            assert_eq!(
                (&p + &q).bit_pack(bits),
                (&p).bit_pack(bits) + (&q).bit_pack(bits)
            );
        }
    });
}
