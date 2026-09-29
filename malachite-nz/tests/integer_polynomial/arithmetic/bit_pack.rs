// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModPowerOf2, Sign};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{ExactFrom, PowerOf2Digits};
use malachite_base::polynomial::{BitPack, MulPowerOfX, Polynomial};
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::arithmetic::bit_pack::limbs_pack_coefficients;
use malachite_nz::integer_polynomial::arithmetic::bit_unpack::limbs_unpack_coefficients;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
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

// Packs `xs` into a zeroed buffer with a spare limb, and returns the buffer.
fn pack(xs: &[Integer], bits: u64, negate: bool) -> Vec<Limb> {
    let total = u64::exact_from(xs.len()) * bits;
    let mut out = vec![0; usize::exact_from(total.div_ceil(Limb::WIDTH)) + 1];
    limbs_pack_coefficients(&mut out, xs, bits, negate);
    out
}

#[test]
fn test_limbs_pack_coefficients() {
    let test = |s, bits, negate, out: &[Limb]| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        assert_eq!(pack(p.coefficients_asc(), bits, negate), out);
    };
    // - *x == 0
    // - !x.sign ^ negate && is_small(x)
    // - !x.sign ^ negate && limbs < size
    // - *x == 0 && !borrow
    test("x", 2, true, &[0xc, 0x0]);
    // - *x == 0 && borrow
    // - *x == 0 && borrow && limbs == 0
    test("x^2+1", 2, true, &[0x2f, 0x0]);
    // - *x == 0 && borrow && limbs > 1
    // - *x == 0 && borrow && limbs != 0
    // - !x.sign ^ negate && is_small(x) && limbs != 0
    // - !x.sign ^ negate && is_small(x) && limbs != 0 && shift == 0
    // - !x.sign ^ negate && !is_small(x)
    // - !x.sign ^ negate && !is_small(x) && shift != 0
    test(
        "18238344937144572188124859526*x^2+1",
        97,
        true,
        &[
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0x525276d6419acde7,
            0x7144660ef,
            0x0,
        ],
    );
    test("x^2+1", 32, true, &[0xffffffffffffffff, 0xfffffffe, 0x0]);
    test("1", 2, true, &[0x3, 0x0]);
    // - !x.sign ^ negate && is_small(x) && limbs != 0 && shift != 0
    test("x", 32, true, &[0xffffffff00000000, 0x0]);
    test("653838635", 64, true, &[0xffffffffd90736d5, 0x0]);
    // - !x.sign ^ negate && !is_small(x) && !borrow
    // - !x.sign ^ negate && limbs >= size
    test(
        "5546202786897723575",
        70,
        true,
        &[0xb307edf34aadff49, 0x3f, 0x0],
    );
    // - limbs + usize::from(rem_bits != 0) > s
    test(
        "8634607594055949012*x",
        68,
        true,
        &[0x0, 0x82bb6242d67a52c0, 0xf8, 0x0],
    );
    // - !x.sign ^ negate && limbs > size
    test(
        "-4*x-4343195683468647380487427071193215737625973",
        160,
        false,
        &[
            0x2b3303ceb75e828b,
            0x7eb9e32ee886a7f0,
            0xfffffffbffffce24,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0x0,
        ],
    );
    // - x.sign ^ negate && is_small(x)
    test("1", 2, false, &[0x1, 0x0]);
    // - x.sign ^ negate && is_small(x) && limbs + usize::from(rem_bits != 0) > 1 && shift != 0
    test("x", 33, false, &[0x200000000, 0x0, 0x0]);
    // - x.sign ^ negate && !is_small(x) && shift != 0
    // - x.sign ^ negate && !is_small(x) && carry != 0
    test(
        "8634607594055949012*x",
        68,
        false,
        &[0x0, 0x7d449dbd2985ad40, 0x7, 0x0],
    );
    // - x.sign ^ negate && !is_small(x) && shift == 0
    test(
        "5546202786897723575",
        70,
        false,
        &[0x4cf8120cb55200b7, 0x0, 0x0],
    );
    // - x.sign ^ negate && !is_small(x) && borrow
    test(
        "-6820794238993809956*x+5",
        68,
        true,
        &[0xfffffffffffffffb, 0xea85438e541c223f, 0x5, 0x0],
    );
    // More examples, with fields of 8 bits that are easy to read in hexadecimal.
    test("0", 8, false, &[0]);
    test("3*x^2+2*x+1", 8, false, &[0x30201, 0]);
    // A negative coefficient borrows from the field above.
    test("3*x^2-2*x+1", 8, false, &[0x2fe01, 0]);
    // A negative leading coefficient borrows past the last field, whose bits are all set.
    test("-3*x^2+2*x-1", 8, false, &[0xfd01ff, 0]);
    test("-3*x^2+2*x-1", 8, true, &[0x2fe01, 0]);
    // Fields that cross limbs.
    test("x^2-1", 40, false, &[u64::MAX, 0xffff, 0]);
    test("-x^2+1", 40, false, &[1, 0xffffffffff0000, 0]);
    test("-18446744073709551616*x+1", 70, false, &[1, 0, 0xfc0, 0]);
}

#[test]
fn limbs_pack_coefficients_properties() {
    integer_polynomial_unsigned_pair_gen_var_4().test_properties(|(p, bits)| {
        let xs = p.coefficients_asc();
        let total = u64::exact_from(xs.len()) * bits;
        for negate in [false, true] {
            let out = pack(xs, bits, negate);
            let packed = Natural::from_limbs_asc(&out);
            // The packing, as a number, is the value at 2^bits of the polynomial or its negation,
            // modulo 2^(len * bits).
            let value = if negate {
                (-&p).bit_pack(bits)
            } else {
                (&p).bit_pack(bits)
            };
            assert_eq!(packed, value.mod_power_of_2(total));
            // Unpacking with the same negation gives the coefficients back, and the borrow out of
            // the last field is set exactly when the packed leading coefficient is negative.
            let mut unpacked = vec![Integer::ZERO; xs.len()];
            let borrow = limbs_unpack_coefficients(&mut unpacked, 0, xs.len(), &out, bits, negate);
            assert_eq!(unpacked, xs);
            if let Some(leading) = xs.last() {
                assert_eq!(borrow, (*leading < 0u32) != negate);
            }
        }
    });
}
