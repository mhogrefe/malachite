// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_nz::integer_matrix::arithmetic::mul::classical::matrix_mul_classical;
use malachite_nz::integer_matrix::arithmetic::mul::double_word::matrix_mul_double_word;
use malachite_nz::integer_matrix::arithmetic::mul::multi_mod::matrix_mul_multi_mod_any;
use malachite_nz::integer_matrix::arithmetic::mul::small::matrix_mul_small;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_matrix_gen, natural_matrix_pair_gen_var_4, natural_matrix_pair_gen_var_5,
    natural_matrix_pair_gen_var_6,
};
use malachite_nz::test_util::integer_matrix::arithmetic::mul::{
    matrix_mul_small_unblocked, matrix_mul_small_unblocked_column_reads, matrix_mul_strassen_any,
    matrix_mul_waksman_any,
};
use malachite_nz::test_util::natural_matrix::arithmetic::mul::{
    natural_matrix_mul_naive, natural_matrix_mul_with,
};

#[test]
fn test_mul() {
    let test = |s, t, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let b = NaturalMatrix::from_str(t).unwrap();

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

        assert_eq!(natural_matrix_mul_naive(&a, &b).to_string(), out);
    };
    test("[]_{0,0}", "[]_{0,0}", "[]_{0,0}");
    test("[]_{0,3}", "[1, 2; 3, 4; 5, 6]", "[]_{0,2}");
    test("[]_{2,0}", "[]_{0,3}", "[0, 0, 0; 0, 0, 0]");
    test("[1, 2, 3; 4, 5, 6]", "[]_{3,0}", "[]_{2,0}");
    test("[5]", "[7]", "[35]");
    test("[1; 2; 3]", "[4, 5]", "[4, 5; 8, 10; 12, 15]");
    test("[1, 2]", "[3; 4]", "[11]");
    test("[1, 2; 3, 4]", "[5, 6; 7, 8]", "[19, 22; 43, 50]");
    test(
        "[1, 2; 3, 4; 5, 6]",
        "[1, 0, 2; 3, 4, 1]",
        "[7, 8, 4; 15, 16, 10; 23, 24, 16]",
    );
    test(
        "[1, 2, 3; 4, 5, 6]",
        "[7, 8; 9, 10; 11, 12]",
        "[58, 64; 139, 154]",
    );
    test(
        "[1, 0, 0; 0, 1, 0; 0, 0, 1]",
        "[2, 3, 5; 7, 11, 13; 17, 19, 23]",
        "[2, 3, 5; 7, 11, 13; 17, 19, 23]",
    );
    test(
        "[18446744073709551615, 18446744073709551616; 18446744073709551616, 1]",
        "[18446744073709551616, 3; 1, 18446744073709551617]",
        "[340282366920938463463374607431768211456, 340282366920938463537161583726606417917; 3402823\
        66920938463463374607431768211457, 73786976294838206465]",
    );
    test(
        "[18446744073709551615, 2, 340282366920938463463374607431768211456; 1, 1, 1]",
        "[18446744073709551616; 18446744073709551615; 340282366920938463463374607431768211463]",
        "[115792089237316195423570985008687907855992243601008071747183027611440984883198; 340282366\
        920938463500268095579187314694]",
    );
    test(
        "[2, 0, 0, 0; 0, 3, 0, 0]",
        "[1, 2; 3, 4; 5, 6; 7, 8]",
        "[2, 4; 9, 12]",
    );
}

// A matrix whose elements are built from a fixed pseudorandom sequence: each has a pseudorandom
// number of significant bits, at most `bits`.
fn pseudorandom_matrix(
    height: u64,
    width: u64,
    bits: u64,
    _signed: bool,
    seed: u64,
) -> NaturalMatrix {
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
        let mut x = Natural::ZERO;
        let mut remaining = len;
        while remaining != 0 {
            let chunk = remaining.min(32);
            x <<= chunk;
            x += Natural::from(next() >> (64 - chunk));
            remaining -= chunk;
        }
        if len != 0 {
            // Make the element have exactly `len` significant bits.
            x |= Natural::power_of_2(len - 1);
        }
        elements.push(x);
    }
    NaturalMatrix::from_owned_elements(height, width, elements)
}

// Checks every algorithm that applies to the product against the naive one.
fn check_kernels(a: &NaturalMatrix, b: &NaturalMatrix, bits: u64, signed: bool) {
    let expected = natural_matrix_mul_naive(a, b);
    assert_eq!(a * b, expected);
    let mut c = a.clone();
    c *= b;
    assert_eq!(c, expected);
    assert_eq!(
        natural_matrix_mul_with(a, b, matrix_mul_classical),
        expected
    );
    assert_eq!(
        natural_matrix_mul_with(a, b, matrix_mul_waksman_any),
        expected
    );
    assert_eq!(
        natural_matrix_mul_with(a, b, matrix_mul_multi_mod_any),
        expected
    );
    assert_eq!(
        natural_matrix_mul_with(a, b, matrix_mul_strassen_any),
        expected
    );
    if bits <= Limb::WIDTH - 2 {
        assert_eq!(natural_matrix_mul_with(a, b, matrix_mul_small), expected);
        assert_eq!(
            natural_matrix_mul_with(a, b, matrix_mul_small_unblocked),
            expected
        );
        assert_eq!(
            natural_matrix_mul_with(a, b, matrix_mul_small_unblocked_column_reads),
            expected
        );
    }
    if bits + u64::from(signed) <= Limb::WIDTH << 1 {
        assert_eq!(
            natural_matrix_mul_with(a, b, matrix_mul_double_word),
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
    let signed = false;
    // Small dimensions: the products fit in one, two, and three words.
    test(5, 7, 6, 10, signed);
    test(5, 7, 6, 40, signed);
    test(5, 7, 6, 62, signed);
    // Larger dimensions, with at most 128 columns in the first matrix, so no blocking.
    test(10, 20, 7, 10, signed);
    test(10, 20, 7, 40, signed);
    test(10, 20, 7, 62, signed);
    // More than 128 columns in the first matrix: blocks of 16 rows, with a partial last block, and
    // blocks of 128 columns, with a last block of even and of odd length.
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
    // Non-negative double-word elements up to 128 bits.
    test(3, 5, 4, 128, false);
    test(3, 20, 4, 128, false);
}

#[test]
fn test_mul_multi_mod_dispatch() {
    // At these sizes the product uses the multimodular algorithm. The results are compared with
    // those of algorithms that the product does not choose.
    let test = |height, inner, width, bits| {
        let a = pseudorandom_matrix(height, inner, bits, false, 1);
        let b = pseudorandom_matrix(inner, width, bits, false, 2);
        let expected = if bits <= 127 {
            natural_matrix_mul_with(&a, &b, matrix_mul_double_word)
        } else {
            natural_matrix_mul_with(&a, &b, matrix_mul_classical)
        };
        assert_eq!(&a * &b, expected);
        let mut c = a.clone();
        c *= &b;
        assert_eq!(c, expected);
        if height == inner && inner == width {
            // A matrix times itself is reduced only once.
            assert_eq!(
                &a * &a,
                natural_matrix_mul_with(&a, &a, matrix_mul_classical)
            );
        }
    };
    // Large elements at a dimension of at least 3 times the bit length of the product's bit count.
    test(33, 33, 33, 600);
    // Non-negative double-word elements; the dimension, 310, exceeds 300 by more than the limit,
    // ((148 - 128) / 8)^2 = 4.
    test(310, 310, 310, 70);
}

#[test]
#[should_panic]
fn mul_fail_1() {
    let _ = NaturalMatrix::from_str("[1, 2]").unwrap() * NaturalMatrix::from_str("[1, 2]").unwrap();
}

#[test]
#[should_panic]
fn mul_fail_2() {
    let _ = &NaturalMatrix::from_str("[]_{0,2}").unwrap()
        * &NaturalMatrix::from_str("[]_{3,0}").unwrap();
}

#[test]
#[should_panic]
fn mul_assign_fail() {
    let mut a = NaturalMatrix::from_str("[1; 2]").unwrap();
    a *= NaturalMatrix::from_str("[1; 2]").unwrap();
}

#[test]
fn mul_properties() {
    natural_matrix_pair_gen_var_4().test_properties(|(a, b)| {
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
        assert_eq!(natural_matrix_mul_naive(&a, &b), c);
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
    });

    natural_matrix_gen().test_properties(|a| {
        let (h, w) = (a.height(), a.width());
        // Identity matrices of the right sizes leave a matrix unchanged.
        assert_eq!(NaturalMatrix::identity(h) * &a, a);
        assert_eq!(&a * NaturalMatrix::identity(w), a);
        // Zero matrices give zero products.
        assert_eq!(&a * NaturalMatrix::zero(w, 3), NaturalMatrix::zero(h, 3));
        assert_eq!(NaturalMatrix::zero(3, h) * &a, NaturalMatrix::zero(3, w));
        // A matrix times its transpose is symmetric.
        assert!((&a * a.transpose()).is_symmetric());
    });

    natural_matrix_pair_gen_var_4().test_properties(|(a, b)| {
        let c = &a * &b;
        assert_eq!(natural_matrix_mul_with(&a, &b, matrix_mul_classical), c);
        assert_eq!(natural_matrix_mul_with(&a, &b, matrix_mul_waksman_any), c);
        assert_eq!(natural_matrix_mul_with(&a, &b, matrix_mul_multi_mod_any), c);
        assert_eq!(natural_matrix_mul_with(&a, &b, matrix_mul_strassen_any), c);
    });

    natural_matrix_pair_gen_var_5().test_properties(|(a, b)| {
        let c = natural_matrix_mul_naive(&a, &b);
        assert_eq!(&a * &b, c);
        assert_eq!(natural_matrix_mul_with(&a, &b, matrix_mul_small), c);
        assert_eq!(
            natural_matrix_mul_with(&a, &b, matrix_mul_small_unblocked),
            c
        );
        assert_eq!(
            natural_matrix_mul_with(&a, &b, matrix_mul_small_unblocked_column_reads),
            c
        );
        let mut d = a.clone();
        d *= &b;
        assert_eq!(d, c);
    });

    natural_matrix_pair_gen_var_6().test_properties(|(a, b)| {
        let c = natural_matrix_mul_naive(&a, &b);
        assert_eq!(&a * &b, c);
        assert_eq!(natural_matrix_mul_with(&a, &b, matrix_mul_double_word), c);
        let mut d = a.clone();
        d *= &b;
        assert_eq!(d, c);
    });
}
