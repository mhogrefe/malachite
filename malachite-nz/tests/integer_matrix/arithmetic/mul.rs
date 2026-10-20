// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{Parity, PowerOf2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_matrix::arithmetic::mul::ProductShape;
use malachite_nz::integer_matrix::arithmetic::mul::classical::{
    matrix_mul_classical, matrix_mul_classical_assign,
};
use malachite_nz::integer_matrix::arithmetic::mul::double_word::matrix_mul_double_word;
use malachite_nz::integer_matrix::arithmetic::mul::small::matrix_mul_small;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    integer_matrix_gen, integer_matrix_pair_gen_var_4, integer_matrix_pair_gen_var_5,
    integer_matrix_pair_gen_var_6,
};
use malachite_nz::test_util::integer_matrix::arithmetic::mul::{
    integer_matrix_mul_naive, integer_matrix_mul_with, matrix_mul_small_unblocked,
    matrix_mul_small_unblocked_column_reads, matrix_mul_strassen_any, matrix_mul_waksman_any,
};

#[test]
fn test_mul() {
    let test = |s, t, out| {
        let a = IntegerMatrix::from_str(s).unwrap();
        let b = IntegerMatrix::from_str(t).unwrap();

        let c = a.clone() * b.clone();
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);

        let c = a.clone() * &b;
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);

        let c = &a * b.clone();
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);

        let c = &a * &b;
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);

        let mut c = a.clone();
        c *= b.clone();
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);

        let mut c = a.clone();
        c *= &b;
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);

        assert_eq!(integer_matrix_mul_naive(&a, &b).to_string(), out);
    };
    test("[]_{0,0}", "[]_{0,0}", "[]_{0,0}");
    test("[]_{0,3}", "[1, 2; 3, 4; 5, 6]", "[]_{0,2}");
    test("[]_{2,0}", "[]_{0,3}", "[0, 0, 0; 0, 0, 0]");
    test("[1, 2, 3; 4, 5, 6]", "[]_{3,0}", "[]_{2,0}");
    test("[5]", "[-7]", "[-35]");
    test("[1; 2; 3]", "[4, 5]", "[4, 5; 8, 10; 12, 15]");
    test("[1, -2]", "[3; 4]", "[-5]");
    test("[1, 2; 3, 4]", "[5, 6; 7, 8]", "[19, 22; 43, 50]");
    test(
        "[1, 2; 3, 4; 5, 6]",
        "[-1, 0, 2; 3, -4, 1]",
        "[5, -8, 4; 9, -16, 10; 13, -24, 16]",
    );
    test(
        "[1, 2, 3; 4, 5, 6]",
        "[7, 8; 9, 10; 11, 12]",
        "[58, 64; 139, 154]",
    );
    test("[1, 2, 3]", "[-1; 2; -3]", "[-6]");
    test(
        "[1, 0, 0; 0, 1, 0; 0, 0, 1]",
        "[2, -3, 5; 7, 11, -13; 17, 19, 23]",
        "[2, -3, 5; 7, 11, -13; 17, 19, 23]",
    );
    test(
        "[18446744073709551615, 18446744073709551616; -18446744073709551616, 1]",
        "[18446744073709551616, 3; -1, 18446744073709551617]",
        "[340282366920938463426481119284349108224, 340282366920938463537161583726606417917; -340282\
        366920938463463374607431768211457, -36893488147419103231]",
    );
    test(
        "[18446744073709551615, 2, -340282366920938463463374607431768211456; 1, -1, 1]",
        "[18446744073709551616; -18446744073709551615; 340282366920938463463374607431768211463]",
        "[-115792089237316195423570985008687907855311678867166194820293171884724867563518; 34028236\
        6920938463500268095579187314694]",
    );
    test(
        "[2, 0, 0, 0; 0, 3, 0, 0]",
        "[1, 2; 3, 4; 5, 6; 7, 8]",
        "[2, 4; 9, 12]",
    );
}

// A matrix whose elements are built from a fixed pseudorandom sequence: each has a pseudorandom
// number of significant bits, at most `bits`, and is negative about half of the time if `signed`.
fn pseudorandom_matrix(
    height: u64,
    width: u64,
    bits: u64,
    signed: bool,
    seed: u64,
) -> IntegerMatrix {
    let mut state = seed.wrapping_mul(0x9e37_79b9_7f4a_7c15) | 1;
    let mut next = || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    let mut elements = Vec::new();
    for _ in 0..height * width {
        let len = next() % (bits + 1);
        let mut x = Integer::ZERO;
        let mut remaining = len;
        while remaining != 0 {
            let chunk = remaining.min(32);
            x <<= chunk;
            x += Integer::from(next() >> (64 - chunk));
            remaining -= chunk;
        }
        if len != 0 {
            // Make the element have exactly `len` significant bits.
            x |= Integer::power_of_2(len - 1);
        }
        let state = next();
        let x = if signed && (state >> 7).odd() { -x } else { x };
        elements.push(x);
    }
    IntegerMatrix::from_owned_elements(height, width, elements)
}

// Checks every algorithm that applies to the product against the naive one.
fn check_kernels(a: &IntegerMatrix, b: &IntegerMatrix, bits: u64, signed: bool) {
    let expected = integer_matrix_mul_naive(a, b);
    assert_eq!(a * b, expected);
    let mut c = a.clone();
    c *= b;
    assert_eq!(c, expected);
    assert_eq!(
        integer_matrix_mul_with(a, b, matrix_mul_classical),
        expected
    );
    assert_eq!(
        integer_matrix_mul_with(a, b, matrix_mul_waksman_any),
        expected
    );
    assert_eq!(
        integer_matrix_mul_with(a, b, matrix_mul_strassen_any),
        expected
    );
    if bits <= Limb::WIDTH - 2 {
        assert_eq!(integer_matrix_mul_with(a, b, matrix_mul_small), expected);
        assert_eq!(
            integer_matrix_mul_with(a, b, matrix_mul_small_unblocked),
            expected
        );
        assert_eq!(
            integer_matrix_mul_with(a, b, matrix_mul_small_unblocked_column_reads),
            expected
        );
    }
    if bits + u64::from(signed) <= Limb::WIDTH << 1 {
        assert_eq!(
            integer_matrix_mul_with(a, b, matrix_mul_double_word),
            expected
        );
    }
}

#[test]
fn test_mul_kernels() {
    let test = |height, inner, width, bits, signed| {
        for seed in 0..4 {
            let a = pseudorandom_matrix(height, inner, bits, signed, seed);
            let b = pseudorandom_matrix(inner, width, bits, signed, seed + 100);
            check_kernels(&a, &b, bits, signed);
        }
    };
    for signed in [false, true] {
        // Small dimensions: the products fit in one, two, and three words.
        test(5, 7, 6, 10, signed);
        test(5, 7, 6, 40, signed);
        test(5, 7, 6, 62, signed);
        // Larger dimensions, with at most 128 columns in the first matrix, so no blocking.
        test(10, 20, 7, 10, signed);
        test(10, 20, 7, 40, signed);
        test(10, 20, 7, 62, signed);
        // More than 128 columns in the first matrix: blocks of 16 rows, with a partial last block,
        // and blocks of 128 columns, with a last block of even and of odd length.
        test(20, 300, 5, 10, signed);
        test(20, 300, 5, 40, signed);
        test(17, 301, 3, 62, signed);
        test(33, 257, 2, 30, signed);
        // Double-word elements, with fewer than 16 columns in the first matrix and with more, and
        // products that fit in four and in five words.
        test(3, 5, 4, 100, signed);
        test(3, 5, 4, 127, signed);
        test(3, 20, 4, 100, signed);
        test(3, 20, 4, 127, signed);
        test(12, 9, 11, 120, signed);
        // Elements that are too large for the word algorithms.
        test(4, 5, 3, 200, signed);
        // Large elements: Waksman's algorithm for dimensions 2, 3, at least 4, and at least 12, and
        // Strassen's for dimensions of at least 8, with odd dimensions in each position.
        test(2, 3, 2, 6000, signed);
        test(3, 4, 3, 3500, signed);
        test(5, 5, 5, 1200, signed);
        test(12, 13, 12, 600, signed);
        test(8, 8, 8, 600, signed);
        test(9, 11, 7, 600, signed);
        test(20, 21, 23, 520, signed);
    }
    // Non-negative double-word elements up to 128 bits.
    test(3, 5, 4, 128, false);
    test(3, 20, 4, 128, false);
}

#[test]
fn test_mul_classical_empty() {
    // The classical kernels handle every empty shape themselves, although the dispatcher handles
    // empty shapes before reaching them.
    let test = |ar: usize, br: usize, bc: usize| {
        let xs = vec![Integer::from(3u32); ar * br];
        let ys = vec![Integer::from(5u32); br * bc];
        let expected = vec![Integer::ZERO; ar * bc];
        let shape = ProductShape { ar, br, bc };
        assert_eq!(matrix_mul_classical(&xs, &ys, shape), expected);
        let mut zs = xs.clone();
        matrix_mul_classical_assign(&mut zs, &ys, shape);
        assert_eq!(zs, expected);
    };
    test(0, 0, 0);
    test(0, 3, 2);
    test(2, 3, 0);
    test(2, 0, 3);
    test(0, 0, 3);
}

#[test]
fn test_mul_strassen_small_elements() {
    // Every dimension is above 1300 and every element of the product fits in a small `fmpz`, so the
    // product uses Strassen's algorithm. The naive product would be too slow, so the result is
    // compared with that of the blocked algorithm for small elements.
    let a = pseudorandom_matrix(1301, 1301, 10, true, 1);
    let b = pseudorandom_matrix(1301, 1301, 10, true, 2);
    assert_eq!(&a * &b, integer_matrix_mul_with(&a, &b, matrix_mul_small));
}

#[test]
#[should_panic]
fn mul_fail_1() {
    let _ = IntegerMatrix::from_str("[1, 2]").unwrap() * IntegerMatrix::from_str("[1, 2]").unwrap();
}

#[test]
#[should_panic]
fn mul_fail_2() {
    let _ = &IntegerMatrix::from_str("[]_{0,2}").unwrap()
        * &IntegerMatrix::from_str("[]_{3,0}").unwrap();
}

#[test]
#[should_panic]
fn mul_assign_fail() {
    let mut a = IntegerMatrix::from_str("[1; 2]").unwrap();
    a *= IntegerMatrix::from_str("[1; 2]").unwrap();
}

#[test]
fn mul_properties() {
    integer_matrix_pair_gen_var_4().test_properties(|(a, b)| {
        let c = &a * &b;
        assert!(c.is_valid());
        assert_eq!((c.height(), c.width()), (a.height(), b.width()));
        assert_eq!(a.clone() * b.clone(), c);
        assert_eq!(a.clone() * &b, c);
        assert_eq!(&a * b.clone(), c);
        let mut d = a.clone();
        d *= b.clone();
        assert_eq!(d, c);
        let mut d = a.clone();
        d *= &b;
        assert_eq!(d, c);
        assert_eq!(integer_matrix_mul_naive(&a, &b), c);
        // Multiplying in place reuses the storage when the product is no wider than the first
        // factor, which includes every product by a square matrix.
        if b.width() <= b.height() && !a.elements.is_empty() && !c.elements.is_empty() {
            let mut d = a.clone();
            let pointer = d.elements.as_ptr();
            d *= &b;
            assert_eq!(d.elements.as_ptr(), pointer);
        }

        // The transpose of a product is the product of the transposes in the other order.
        assert_eq!(b.transpose() * a.transpose(), c.transpose());
        // Each column of the product is the first matrix times that column of the second.
        for j in 0..b.width() {
            assert_eq!(&a * b.column(j), c.column(j));
        }
        // Negating either factor negates the product.
        assert_eq!(-&a * &b, -&c);
        assert_eq!(&a * -&b, -c);
    });

    integer_matrix_gen().test_properties(|a| {
        let (h, w) = (a.height(), a.width());
        // Identity matrices of the right sizes leave a matrix unchanged.
        assert_eq!(IntegerMatrix::identity(h) * &a, a);
        assert_eq!(&a * IntegerMatrix::identity(w), a);
        // Zero matrices give zero products.
        assert_eq!(&a * IntegerMatrix::zero(w, 3), IntegerMatrix::zero(h, 3));
        assert_eq!(IntegerMatrix::zero(3, h) * &a, IntegerMatrix::zero(3, w));
        // A matrix times its transpose is symmetric.
        assert!((&a * a.transpose()).is_symmetric());
    });

    integer_matrix_pair_gen_var_4().test_properties(|(a, b)| {
        let c = &a * &b;
        assert_eq!(integer_matrix_mul_with(&a, &b, matrix_mul_classical), c);
        assert_eq!(integer_matrix_mul_with(&a, &b, matrix_mul_waksman_any), c);
        assert_eq!(integer_matrix_mul_with(&a, &b, matrix_mul_strassen_any), c);
    });

    integer_matrix_pair_gen_var_5().test_properties(|(a, b)| {
        let c = integer_matrix_mul_naive(&a, &b);
        assert_eq!(&a * &b, c);
        assert_eq!(integer_matrix_mul_with(&a, &b, matrix_mul_small), c);
        assert_eq!(
            integer_matrix_mul_with(&a, &b, matrix_mul_small_unblocked),
            c
        );
        assert_eq!(
            integer_matrix_mul_with(&a, &b, matrix_mul_small_unblocked_column_reads),
            c
        );
        let mut d = a.clone();
        d *= &b;
        assert_eq!(d, c);
    });

    integer_matrix_pair_gen_var_6().test_properties(|(a, b)| {
        let c = integer_matrix_mul_naive(&a, &b);
        assert_eq!(&a * &b, c);
        assert_eq!(integer_matrix_mul_with(&a, &b, matrix_mul_double_word), c);
        let mut d = a.clone();
        d *= &b;
        assert_eq!(d, c);
    });
}
