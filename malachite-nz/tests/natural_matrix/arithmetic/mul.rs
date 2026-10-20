// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::{natural_matrix_gen, natural_matrix_pair_gen_var_4};
use malachite_nz::test_util::natural_matrix::arithmetic::mul::natural_matrix_mul_naive;

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
}
