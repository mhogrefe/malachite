// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    DivisibleBy, Mod, ModIsReduced, ModPowerOf2, PowerOf2, UnsignedAbs,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::{
    integer_matrix_gen, integer_matrix_integer_pair_gen_var_1,
    integer_matrix_natural_pair_gen_var_1, integer_matrix_unsigned_pair_gen_var_1,
};

fn neg(a: &IntegerMatrix) -> IntegerMatrix {
    IntegerMatrix::from_owned_elements(
        a.height(),
        a.width(),
        a.elements.iter().map(|x| -x).collect(),
    )
}

#[test]
fn test_mod_op() {
    let test = |s, m, out| {
        let a = IntegerMatrix::from_str(s).unwrap();
        let m = Natural::from_str(m).unwrap();
        // All four combinations of value and reference.
        let b = (&a).mod_op(&m);
        assert_eq!(b.to_string(), out);
        assert_eq!((&a).mod_op(m.clone()), b);
        assert_eq!(a.clone().mod_op(&m), b);
        assert_eq!(a.mod_op(m), b);
    };
    test("[]_{0,0}", "1", "[]_{0,0}");
    test("[]_{3,0}", "1", "[]_{3,0}");
    test("[]_{0,4}", "1", "[]_{0,4}");
    test("[5, -4; -1, 9]", "3", "[2, 2; 2, 0]");
    test("[-4; 3]", "4", "[0; 3]");
    test("[]_{0,0}", "7", "[]_{0,0}");
    // Modulo 1 every element is zero, but the shape is unchanged.
    test("[1, -4, -5]", "1", "[0, 0, 0]");
    // Negative elements become non-negative.
    test("[1, -4, -5]", "3", "[1, 2, 1]");
    test("[-1]", "10", "[9]");
    test("[-1]", "18446744073709551616", "[18446744073709551615]");
    // Non-negative matrices behave as NaturalMatrixs do.
    test("[1, 4, 5]", "3", "[1, 1, 2]");
    // A negative multiple of the modulus becomes zero, not the modulus, and stays.
    test("[-6, 3]", "3", "[0, 0]");
    // Elements and divisor of many limbs.
    test(
        "[-1000000000000000000000000, 1]",
        "1234567890987",
        "[704498996588, 1]",
    );
    // A divisor larger than every element leaves the non-negative ones alone.
    test(
        "[1, -4, 5]",
        "1000000000000000000000000",
        "[1, 999999999999999999999996, 5]",
    );
}

#[test]
#[should_panic]
fn mod_op_fail() {
    let _ = IntegerMatrix::from_str("[1]")
        .unwrap()
        .mod_op(Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_op_val_ref_fail() {
    let _ = IntegerMatrix::from_str("[1]")
        .unwrap()
        .mod_op(&Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_op_ref_val_fail() {
    let _ = (&IntegerMatrix::from_str("[1]").unwrap()).mod_op(Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_op_ref_ref_fail() {
    let _ = (&IntegerMatrix::from_str("[1]").unwrap()).mod_op(&Natural::ZERO);
}

// The empty matrix has no elements, so a zero divisor is never reached by the elementwise loop;
// only the explicit check makes these panic rather than quietly returning the empty matrix.
#[test]
#[should_panic]
fn mod_op_empty_matrix_fail() {
    let _ = IntegerMatrix::from_str("[]_{0,0}")
        .unwrap()
        .mod_op(Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_op_ref_empty_matrix_fail() {
    let _ = (&IntegerMatrix::from_str("[]_{0,0}").unwrap()).mod_op(&Natural::ZERO);
}

#[test]
fn mod_op_properties() {
    integer_matrix_natural_pair_gen_var_1().test_properties(|(a, m)| {
        let b = (&a).mod_op(&m);
        // The forms agree.
        assert_eq!((&a).mod_op(m.clone()), b);
        assert_eq!(a.clone().mod_op(&m), b);
        assert_eq!(a.clone().mod_op(m.clone()), b);

        // The result is reduced, and reducing it again changes nothing.
        assert!(b.mod_is_reduced(&m));
        assert_eq!(&b % &m, b);

        // The shape is unchanged.
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));

        // Element by element, this is the `Integer` operation, and each result is congruent to the
        // original element.
        let m_i = Integer::from(&m);
        for (x, y) in a.elements.iter().zip(&b.elements) {
            assert_eq!(Integer::from(y), x.mod_op(&m_i));
            assert!((Integer::from(y) - x).divisible_by(&m_i));
        }

        // Negating the matrix gives the negated residues.
        let neg_w = neg(&a).mod_op(&m);
        for (x, y) in b.elements.iter().zip(&neg_w.elements) {
            assert!((Integer::from(x) + Integer::from(y)).divisible_by(&m_i));
        }

        // On a matrix with no negative elements, this is the `NaturalMatrix` operation.
        if let Ok(n) = NaturalMatrix::try_from(&a) {
            assert_eq!(n % &m, b);
        }
    });

    integer_matrix_gen().test_properties(|a| {
        // Modulo 1 every element vanishes, but the shape is unchanged.
        assert_eq!(
            (&a).mod_op(Natural::ONE),
            NaturalMatrix::from_owned_elements(
                a.height(),
                a.width(),
                vec![Natural::ZERO; a.elements.len()]
            )
        );
        // For a power-of-2 divisor this is `mod_power_of_2`.
        for pow in [1, 7, 64, 100] {
            assert_eq!(
                (&a).mod_op(Natural::power_of_2(pow)),
                (&a).mod_power_of_2(pow)
            );
        }
    });
}

#[test]
fn test_mod_op_unsigned() {
    fn test<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(s: &str, m: T, out: &str)
    where
        Natural: From<T>,
    {
        let a = IntegerMatrix::from_str(s).unwrap();
        let b: UnsignedMatrix<T> = (&a).mod_op(m);
        assert_eq!(b.to_string(), out);
        assert_eq!(a.mod_op(m), b);
    }
    test("[]_{0,0}", 1u8, "[]_{0,0}");
    test("[]_{3,0}", 1u8, "[]_{3,0}");
    test("[]_{0,4}", 1u8, "[]_{0,4}");
    test("[5, -4; -1, 9]", 3u8, "[2, 2; 2, 0]");
    test("[-4; 3]", 4u8, "[0; 3]");
    test("[]_{0,0}", 7u64, "[]_{0,0}");
    test("[1, -4, -5]", 3u32, "[1, 2, 1]");
    test("[1, -4, -5]", 1u16, "[0, 0, 0]");
    // Negative elements become non-negative, and stay within the modulus type.
    test("[-1]", 255u8, "[254]");
    test("[-1]", u64::MAX, "[18446744073709551614]");
    test(
        "[-1]",
        u128::MAX,
        "[340282366920938463463374607431768211454]",
    );
    // Elements far larger than the modulus type are reduced into it.
    test("[-1000000000001, 2000000000003, -5]", 7u8, "[5, 5, 2]");
    test(
        "[-100000000000000000000, 1]",
        u64::MAX,
        "[10680464442257309690, 1]",
    );
    // An element that reduces to zero stays, so the shape is unchanged.
    test("[-1024, -3]", 4usize, "[0, 1]");
}

#[test]
#[should_panic]
fn mod_op_unsigned_fail() {
    let _: UnsignedMatrix<u8> = IntegerMatrix::from_str("[1]").unwrap().mod_op(0u8);
}

#[test]
#[should_panic]
fn mod_op_unsigned_ref_fail() {
    let _: UnsignedMatrix<u64> = (&IntegerMatrix::from_str("[-1]").unwrap()).mod_op(0u64);
}

#[test]
#[should_panic]
fn mod_op_unsigned_empty_matrix_fail() {
    let _: UnsignedMatrix<u32> = (&IntegerMatrix::from_str("[]_{0,0}").unwrap()).mod_op(0u32);
}

fn mod_op_unsigned_properties_helper<
    T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural> + for<'a> TryFrom<&'a Natural>,
>()
where
    Natural: From<T>,
{
    integer_matrix_unsigned_pair_gen_var_1::<T>().test_properties(|(a, m)| {
        if m == T::ZERO {
            return;
        }
        let b: UnsignedMatrix<T> = (&a).mod_op(m);
        assert_eq!(a.clone().mod_op(m), b);

        // The result is reduced, and reducing it again changes nothing.
        assert!(b.mod_is_reduced(&m));
        assert_eq!((&b).mod_op(m), b);

        // Apart from its type, the result is the reduction modulo the Natural with the same value.
        let r = (&a).mod_op(Natural::from(m));
        assert_eq!(UnsignedMatrix::<T>::try_from(&r), Ok(b.clone()));

        // On a matrix with no negative elements, this is the `NaturalMatrix` operation.
        if let Ok(n) = NaturalMatrix::try_from(&a) {
            assert_eq!(n % m, b);
        }
    });
}

#[test]
fn mod_op_unsigned_properties() {
    apply_fn_to_unsigneds!(mod_op_unsigned_properties_helper);
}

#[test]
fn test_rem() {
    let test = |s, m, out| {
        let a = IntegerMatrix::from_str(s).unwrap();
        let m = Integer::from_str(m).unwrap();
        // All four combinations of value and reference, and in place with both.
        let b = &a % &m;
        assert_eq!(b.to_string(), out);
        assert_eq!(&a % m.clone(), b);
        assert_eq!(a.clone() % &m, b);
        assert_eq!(a.clone() % m.clone(), b);
        let mut x = a.clone();
        x %= &m;
        assert_eq!(x, b);
        let mut x = a;
        x %= m;
        assert_eq!(x, b);
    };
    test("[]_{0,0}", "1", "[]_{0,0}");
    test("[]_{3,0}", "1", "[]_{3,0}");
    test("[]_{0,4}", "1", "[]_{0,4}");
    test("[5, -4; -1, 9]", "3", "[2, -1; -1, 0]");
    test("[-4; 3]", "-4", "[0; 3]");
    test("[]_{0,0}", "-7", "[]_{0,0}");
    // Modulo 1 every element is zero, but the shape is unchanged.
    test("[1, -4, -5]", "1", "[0, 0, 0]");
    // Every element keeps its sign, and the sign of the modulus makes no difference.
    test("[1, -4, -5]", "3", "[1, -1, -2]");
    test("[1, -4, -5]", "-3", "[1, -1, -2]");
    // Multiples of the modulus become zero and stay.
    test("[-6, 3, -1]", "-3", "[0, 0, -1]");
    // Elements and divisor of many limbs.
    test(
        "[-1000000000000000000000000, 1]",
        "1234567890987",
        "[-530068894399, 1]",
    );
}

#[test]
#[should_panic]
fn rem_fail() {
    let _ = IntegerMatrix::from_str("[1]").unwrap() % Integer::ZERO;
}

#[test]
#[should_panic]
fn rem_ref_ref_fail() {
    let _ = &IntegerMatrix::from_str("[1]").unwrap() % &Integer::ZERO;
}

#[test]
#[should_panic]
fn rem_assign_fail() {
    let mut a = IntegerMatrix::from_str("[1]").unwrap();
    a %= Integer::ZERO;
}

// The empty matrix has no elements, so a zero divisor is never reached by the elementwise loop;
// only the explicit check makes these panic rather than quietly returning the empty matrix.
#[test]
#[should_panic]
fn rem_empty_matrix_fail() {
    let _ = &IntegerMatrix::from_str("[]_{0,0}").unwrap() % &Integer::ZERO;
}

#[test]
#[should_panic]
fn rem_assign_empty_matrix_fail() {
    let mut a = IntegerMatrix::from_str("[]_{0,0}").unwrap();
    a %= &Integer::ZERO;
}

#[test]
fn rem_properties() {
    integer_matrix_integer_pair_gen_var_1().test_properties(|(a, m)| {
        let b = &a % &m;
        // The forms agree.
        assert_eq!(&a % m.clone(), b);
        assert_eq!(a.clone() % &m, b);
        assert_eq!(a.clone() % m.clone(), b);
        let mut x = a.clone();
        x %= &m;
        assert_eq!(x, b);
        let mut x = a.clone();
        x %= m.clone();
        assert_eq!(x, b);

        // The sign of the modulus makes no difference, the result commutes with negation, and
        // reducing again changes nothing.
        assert_eq!(&a % -&m, b);
        assert_eq!(neg(&a) % &m, neg(&b));
        assert_eq!(&b % &m, b);
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));

        // Element by element, this is the `Integer` operation.
        for (x, y) in a.elements.iter().zip(&b.elements) {
            assert_eq!(*y, x % &m);
        }

        // Reducing into [0, |m|) gives the same result as from the original matrix.
        let abs_m = (&m).unsigned_abs();
        assert_eq!((&b).mod_op(&abs_m), (&a).mod_op(&abs_m));
    });
}
