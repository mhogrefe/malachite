// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::{BitPack, BitUnpack, Polynomial};
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::arithmetic::bit_unpack::{
    limbs_unpack_coefficients, limbs_unpack_coefficients_unsigned,
};
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    integer_polynomial_unsigned_pair_gen_var_4, integer_unsigned_pair_gen_var_6,
    natural_unsigned_pair_gen_var_7,
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

// The limbs of `n`, padded with zero limbs to cover every field that holds any of its bits, and the
// number of those fields.
fn limbs_and_fields(n: &Natural, bits: u64) -> (Vec<Limb>, usize) {
    let nhi = usize::exact_from(n.significant_bits().div_ceil(bits));
    let mut xs = n.to_limbs_asc();
    xs.resize(
        usize::exact_from((u64::exact_from(nhi) * bits).div_ceil(Limb::WIDTH)) + 1,
        0,
    );
    (xs, nhi)
}

// The expected limbs are written for 64-bit limbs.
#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_limbs_unpack_coefficients() {
    let test = |xs: &[Limb], nlo, nhi, bits, negate, out: &[&str], borrow_out| {
        let mut unpacked = vec![Integer::ZERO; nhi - nlo];
        let borrow = limbs_unpack_coefficients(&mut unpacked, nlo, nhi, xs, bits, negate);
        let out: Vec<Integer> = out.iter().map(|x| Integer::from_str(x).unwrap()).collect();
        assert_eq!(unpacked, out);
        assert_eq!(borrow, borrow_out);
    };
    // - rem_bits != 0
    // - bits <= SMALL_FMPZ_BITCOUNT_MAX && sign
    // - negate
    // - bits <= SMALL_FMPZ_BITCOUNT_MAX && limbs + usize::from(rem_bits != 0) <= 1
    test(&[0x1, 0x0], 0, 1, 1, true, &["1"], true);
    // - rem_bits == 0
    // - bits > SMALL_FMPZ_BITCOUNT_MAX
    test(&[0x2, 0x0], 0, 1, 64, true, &["-2"], false);
    // - bits <= SMALL_FMPZ_BITCOUNT_MAX && limbs + usize::from(rem_bits != 0) > 1
    test(
        &[0x0, 0x2, 0x0],
        0,
        8,
        9,
        true,
        &["0", "0", "0", "0", "0", "0", "0", "-4"],
        false,
    );
    // - bits <= SMALL_FMPZ_BITCOUNT_MAX && borrow
    test(&[0x3, 0x0], 0, 2, 1, false, &["-1", "0"], true);
    // - nlo != 0
    test(&[0x2, 0x0], 1, 2, 1, false, &["-1"], true);
    // - bits > SMALL_FMPZ_BITCOUNT_MAX && b != 0
    // - bits > SMALL_FMPZ_BITCOUNT_MAX && shift == 0
    test(&[0x2, 0x0], 0, 1, 63, false, &["2"], false);
    // - bits > SMALL_FMPZ_BITCOUNT_MAX && shift != 0
    // - bits > SMALL_FMPZ_BITCOUNT_MAX && limbs + usize::from(rem_bits != 0) > l
    test(&[0x0, 0x20, 0x0], 0, 2, 63, false, &["0", "64"], false);
    // - bits > SMALL_FMPZ_BITCOUNT_MAX && sign && b != 0
    // - bits > SMALL_FMPZ_BITCOUNT_MAX && sign && !borrow
    test(
        &[0x0, 0x100, 0x0],
        0,
        1,
        73,
        false,
        &["-4722366482869645213696"],
        true,
    );
    // - bits > SMALL_FMPZ_BITCOUNT_MAX && !sign && borrow
    test(
        &[0x0, 0x1f, 0x0, 0x0],
        0,
        2,
        68,
        false,
        &["-18446744073709551616", "2"],
        false,
    );
    // - bits > SMALL_FMPZ_BITCOUNT_MAX && sign && borrow
    test(
        &[0xffffffffffffffff, 0xffffffffffffffff, 0x0],
        0,
        2,
        63,
        false,
        &["-1", "0"],
        true,
    );
    // More examples, with fields of 8 bits that are easy to read in hexadecimal.
    test(&[0x30201, 0], 0, 3, 8, false, &["1", "2", "3"], false);
    test(&[0x2fe01, 0], 0, 3, 8, false, &["1", "-2", "3"], false);
    test(&[0x2fe01, 0], 0, 3, 8, true, &["-1", "2", "-3"], false);
    // The borrow into the first field unpacked comes from the field below it.
    test(&[0x2fe01, 0], 2, 3, 8, false, &["3"], false);
    test(&[0x2fe01, 0], 1, 2, 8, false, &["-2"], true);
    test(&[0xfd01ff, 0], 0, 3, 8, false, &["-1", "2", "-3"], true);
    test(
        &[1, 0xffffffffff0000, 0],
        0,
        3,
        40,
        false,
        &["1", "0", "-1"],
        true,
    );
}

// The same cases as the 64-bit version, with each limb split in two; the fields, and so the
// results, are the same.
#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_unpack_coefficients_32() {
    let test = |xs: &[Limb], nlo, nhi, bits, negate, out: &[&str], borrow_out| {
        let mut unpacked = vec![Integer::ZERO; nhi - nlo];
        let borrow = limbs_unpack_coefficients(&mut unpacked, nlo, nhi, xs, bits, negate);
        let out: Vec<Integer> = out.iter().map(|x| Integer::from_str(x).unwrap()).collect();
        assert_eq!(unpacked, out);
        assert_eq!(borrow, borrow_out);
    };
    test(&[0x1, 0x0, 0x0, 0x0], 0, 1, 1, true, &["1"], true);
    test(&[0x2, 0x0, 0x0, 0x0], 0, 1, 64, true, &["-2"], false);
    test(
        &[0x0, 0x0, 0x2, 0x0, 0x0, 0x0],
        0,
        8,
        9,
        true,
        &["0", "0", "0", "0", "0", "0", "0", "-4"],
        false,
    );
    test(&[0x3, 0x0, 0x0, 0x0], 0, 2, 1, false, &["-1", "0"], true);
    test(&[0x2, 0x0, 0x0, 0x0], 1, 2, 1, false, &["-1"], true);
    test(&[0x2, 0x0, 0x0, 0x0], 0, 1, 63, false, &["2"], false);
    test(
        &[0x0, 0x0, 0x20, 0x0, 0x0, 0x0],
        0,
        2,
        63,
        false,
        &["0", "64"],
        false,
    );
    test(
        &[0x0, 0x0, 0x100, 0x0, 0x0, 0x0],
        0,
        1,
        73,
        false,
        &["-4722366482869645213696"],
        true,
    );
    test(
        &[0x0, 0x0, 0x1f, 0x0, 0x0, 0x0, 0x0, 0x0],
        0,
        2,
        68,
        false,
        &["-18446744073709551616", "2"],
        false,
    );
    test(
        &[0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0x0, 0x0],
        0,
        2,
        63,
        false,
        &["-1", "0"],
        true,
    );
    // More examples, with fields of 8 bits that are easy to read in hexadecimal.
    test(
        &[0x30201, 0x0, 0x0, 0x0],
        0,
        3,
        8,
        false,
        &["1", "2", "3"],
        false,
    );
    test(
        &[0x2fe01, 0x0, 0x0, 0x0],
        0,
        3,
        8,
        false,
        &["1", "-2", "3"],
        false,
    );
    test(
        &[0x2fe01, 0x0, 0x0, 0x0],
        0,
        3,
        8,
        true,
        &["-1", "2", "-3"],
        false,
    );
    // The borrow into the first field unpacked comes from the field below it.
    test(&[0x2fe01, 0x0, 0x0, 0x0], 2, 3, 8, false, &["3"], false);
    test(&[0x2fe01, 0x0, 0x0, 0x0], 1, 2, 8, false, &["-2"], true);
    test(
        &[0xfd01ff, 0x0, 0x0, 0x0],
        0,
        3,
        8,
        false,
        &["-1", "2", "-3"],
        true,
    );
    test(
        &[0x1, 0x0, 0xffff0000, 0xffffff, 0x0, 0x0],
        0,
        3,
        40,
        false,
        &["1", "0", "-1"],
        true,
    );
}

// The expected limbs are written for 64-bit limbs.
#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_limbs_unpack_coefficients_unsigned() {
    let test = |xs: &[Limb], nlo, nhi, bits, out: &[&str]| {
        let mut unpacked = vec![Integer::ZERO; nhi - nlo];
        limbs_unpack_coefficients_unsigned(&mut unpacked, nlo, nhi, xs, bits);
        let out: Vec<Integer> = out.iter().map(|x| Integer::from_str(x).unwrap()).collect();
        assert_eq!(unpacked, out);
    };
    // - nlo != 0
    // - bits <= SMALL_FMPZ_BITCOUNT_MAX && limbs + usize::from(rem_bits != 0) <= 1
    test(&[0x2, 0x0], 1, 2, 1, &["1"]);
    // - bits <= SMALL_FMPZ_BITCOUNT_MAX && limbs + usize::from(rem_bits != 0) > 1
    test(
        &[0x0, 0x2, 0x0],
        0,
        8,
        9,
        &["0", "0", "0", "0", "0", "0", "0", "4"],
    );
    // - bits > SMALL_FMPZ_BITCOUNT_MAX && b != 0
    // - bits > SMALL_FMPZ_BITCOUNT_MAX && shift == 0
    test(&[0x2, 0x0], 0, 1, 63, &["2"]);
    // - bits > SMALL_FMPZ_BITCOUNT_MAX && shift != 0
    // - bits > SMALL_FMPZ_BITCOUNT_MAX && limbs + usize::from(rem_bits != 0) > l
    test(&[0x0, 0x20, 0x0], 0, 2, 63, &["0", "64"]);
    // More examples, with fields of 8 bits that are easy to read in hexadecimal.
    test(&[0x30201, 0], 0, 3, 8, &["1", "2", "3"]);
    test(&[0x2fe01, 0], 0, 3, 8, &["1", "254", "2"]);
    test(&[0x2fe01, 0], 1, 3, 8, &["254", "2"]);
    test(&[u64::MAX, 1, 0], 0, 2, 65, &["36893488147419103231", "0"]);
}

// The same cases as the 64-bit version, with each limb split in two; the fields, and so the
// results, are the same.
#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_unpack_coefficients_unsigned_32() {
    let test = |xs: &[Limb], nlo, nhi, bits, out: &[&str]| {
        let mut unpacked = vec![Integer::ZERO; nhi - nlo];
        limbs_unpack_coefficients_unsigned(&mut unpacked, nlo, nhi, xs, bits);
        let out: Vec<Integer> = out.iter().map(|x| Integer::from_str(x).unwrap()).collect();
        assert_eq!(unpacked, out);
    };
    test(&[0x2, 0x0, 0x0, 0x0], 1, 2, 1, &["1"]);
    test(
        &[0x0, 0x0, 0x2, 0x0, 0x0, 0x0],
        0,
        8,
        9,
        &["0", "0", "0", "0", "0", "0", "0", "4"],
    );
    test(&[0x2, 0x0, 0x0, 0x0], 0, 1, 63, &["2"]);
    test(&[0x0, 0x0, 0x20, 0x0, 0x0, 0x0], 0, 2, 63, &["0", "64"]);
    // More examples, with fields of 8 bits that are easy to read in hexadecimal.
    test(&[0x30201, 0x0, 0x0, 0x0], 0, 3, 8, &["1", "2", "3"]);
    test(&[0x2fe01, 0x0, 0x0, 0x0], 0, 3, 8, &["1", "254", "2"]);
    test(&[0x2fe01, 0x0, 0x0, 0x0], 1, 3, 8, &["254", "2"]);
    test(
        &[0xffffffff, 0xffffffff, 0x1, 0x0, 0x0, 0x0],
        0,
        2,
        65,
        &["36893488147419103231", "0"],
    );
}

#[test]
fn limbs_unpack_coefficients_properties() {
    natural_unsigned_pair_gen_var_7::<u64>().test_properties(|(n, bits)| {
        let (xs, nhi) = limbs_and_fields(&n, bits);
        let mut full = vec![Integer::ZERO; nhi];
        let borrow = limbs_unpack_coefficients(&mut full, 0, nhi, &xs, bits, false);
        // It agrees with the public signed unpacking, whose leading 1 is the final borrow.
        let mut expected = full.clone();
        if borrow {
            expected.push(Integer::ONE);
        }
        assert_eq!(
            IntegerPolynomial::from_coefficients_asc(expected),
            IntegerPolynomial::bit_unpack(Integer::from(&n), bits)
        );
        // Negating negates every coefficient, and leaves the borrows alone.
        let mut negated = vec![Integer::ZERO; nhi];
        assert_eq!(
            limbs_unpack_coefficients(&mut negated, 0, nhi, &xs, bits, true),
            borrow
        );
        assert_eq!(negated, full.iter().map(|c| -c).collect::<Vec<_>>());
        // A window agrees with the same part of the whole.
        for nlo in 0..nhi {
            let mut window = vec![Integer::ZERO; nhi - nlo];
            assert_eq!(
                limbs_unpack_coefficients(&mut window, nlo, nhi, &xs, bits, false),
                borrow
            );
            assert_eq!(window, &full[nlo..]);
        }

        let mut unsigned = vec![Integer::ZERO; nhi];
        limbs_unpack_coefficients_unsigned(&mut unsigned, 0, nhi, &xs, bits);
        // The unsigned unpacking agrees with NaturalPolynomial's.
        assert_eq!(
            NaturalPolynomial::from_coefficients_asc(
                unsigned.iter().map(Natural::exact_from).collect()
            ),
            NaturalPolynomial::bit_unpack(&n, bits)
        );
        for nlo in 0..nhi {
            let mut window = vec![Integer::ZERO; nhi - nlo];
            limbs_unpack_coefficients_unsigned(&mut window, nlo, nhi, &xs, bits);
            assert_eq!(window, &unsigned[nlo..]);
        }
    });
}
