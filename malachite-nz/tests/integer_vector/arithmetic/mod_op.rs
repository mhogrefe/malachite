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
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::{
    integer_vector_gen, integer_vector_integer_pair_gen_var_1,
    integer_vector_natural_pair_gen_var_1, integer_vector_unsigned_pair_gen,
};

#[test]
fn test_mod_op() {
    let test = |s, m, out| {
        let v = IntegerVector::from_str(s).unwrap();
        let m = Natural::from_str(m).unwrap();
        // All four combinations of value and reference.
        let w = (&v).mod_op(&m);
        assert_eq!(w.to_string(), out);
        assert_eq!((&v).mod_op(m.clone()), w);
        assert_eq!(v.clone().mod_op(&m), w);
        assert_eq!(v.mod_op(m), w);
    };
    test("()", "1", "()");
    test("()", "7", "()");
    // Modulo 1 every element is zero, but the dimension is unchanged.
    test("(1, -4, -5)", "1", "(0, 0, 0)");
    // Negative elements become non-negative.
    test("(1, -4, -5)", "3", "(1, 2, 1)");
    test("(-1)", "10", "(9)");
    test("(-1)", "18446744073709551616", "(18446744073709551615)");
    // Non-negative vectors behave as NaturalVectors do.
    test("(1, 4, 5)", "3", "(1, 1, 2)");
    // A negative multiple of the modulus becomes zero, not the modulus, and stays.
    test("(-6, 3)", "3", "(0, 0)");
    // Elements and divisor of many limbs.
    test(
        "(-1000000000000000000000000, 1)",
        "1234567890987",
        "(704498996588, 1)",
    );
    // A divisor larger than every element leaves the non-negative ones alone.
    test(
        "(1, -4, 5)",
        "1000000000000000000000000",
        "(1, 999999999999999999999996, 5)",
    );
}

#[test]
#[should_panic]
fn mod_op_fail() {
    let _ = IntegerVector::from_str("(1)")
        .unwrap()
        .mod_op(Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_op_val_ref_fail() {
    let _ = IntegerVector::from_str("(1)")
        .unwrap()
        .mod_op(&Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_op_ref_val_fail() {
    let _ = (&IntegerVector::from_str("(1)").unwrap()).mod_op(Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_op_ref_ref_fail() {
    let _ = (&IntegerVector::from_str("(1)").unwrap()).mod_op(&Natural::ZERO);
}

// The empty vector has no elements, so a zero divisor is never reached by the elementwise loop;
// only the explicit check makes these panic rather than quietly returning the empty vector.
#[test]
#[should_panic]
fn mod_op_empty_vector_fail() {
    let _ = IntegerVector::from_str("()").unwrap().mod_op(Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_op_ref_empty_vector_fail() {
    let _ = (&IntegerVector::from_str("()").unwrap()).mod_op(&Natural::ZERO);
}

#[test]
fn mod_op_properties() {
    integer_vector_natural_pair_gen_var_1().test_properties(|(v, m)| {
        let w = (&v).mod_op(&m);
        // The forms agree.
        assert_eq!((&v).mod_op(m.clone()), w);
        assert_eq!(v.clone().mod_op(&m), w);
        assert_eq!(v.clone().mod_op(m.clone()), w);

        // The result is reduced, and reducing it again changes nothing.
        assert!(w.mod_is_reduced(&m));
        assert_eq!(&w % &m, w);

        // The dimension is unchanged.
        assert_eq!(w.dimension(), v.dimension());

        // Element by element, this is the `Integer` operation, and each result is congruent to the
        // original element.
        let m_i = Integer::from(&m);
        for (x, y) in v.elements.iter().zip(&w.elements) {
            assert_eq!(Integer::from(y), x.mod_op(&m_i));
            assert!((Integer::from(y) - x).divisible_by(&m_i));
        }

        // Negating the vector gives the negated residues.
        let neg_w = (-&v).mod_op(&m);
        for (x, y) in w.elements.iter().zip(&neg_w.elements) {
            assert!((Integer::from(x) + Integer::from(y)).divisible_by(&m_i));
        }

        // On a vector with no negative elements, this is the `NaturalVector` operation.
        if let Ok(n) = NaturalVector::try_from(&v) {
            assert_eq!(n % &m, w);
        }
    });

    integer_vector_gen().test_properties(|v| {
        // Modulo 1 every element vanishes, but the dimension is unchanged.
        assert_eq!(
            (&v).mod_op(Natural::ONE),
            NaturalVector::zero(v.dimension())
        );
        // For a power-of-2 divisor this is `mod_power_of_2`.
        for pow in [1, 7, 64, 100] {
            assert_eq!(
                (&v).mod_op(Natural::power_of_2(pow)),
                (&v).mod_power_of_2(pow)
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
        let v = IntegerVector::from_str(s).unwrap();
        let w: UnsignedVector<T> = (&v).mod_op(m);
        assert_eq!(w.to_string(), out);
        assert_eq!(v.mod_op(m), w);
    }
    test("()", 1u8, "()");
    test("()", 7u64, "()");
    test("(1, -4, -5)", 3u32, "(1, 2, 1)");
    test("(1, -4, -5)", 1u16, "(0, 0, 0)");
    // Negative elements become non-negative, and stay within the modulus type.
    test("(-1)", 255u8, "(254)");
    test("(-1)", u64::MAX, "(18446744073709551614)");
    test(
        "(-1)",
        u128::MAX,
        "(340282366920938463463374607431768211454)",
    );
    // Elements far larger than the modulus type are reduced into it.
    test("(-1000000000001, 2000000000003, -5)", 7u8, "(5, 5, 2)");
    test(
        "(-100000000000000000000, 1)",
        u64::MAX,
        "(10680464442257309690, 1)",
    );
    // An element that reduces to zero stays, so the dimension is unchanged.
    test("(-1024, -3)", 4usize, "(0, 1)");
}

#[test]
#[should_panic]
fn mod_op_unsigned_fail() {
    let _: UnsignedVector<u8> = IntegerVector::from_str("(1)").unwrap().mod_op(0u8);
}

#[test]
#[should_panic]
fn mod_op_unsigned_ref_fail() {
    let _: UnsignedVector<u64> = (&IntegerVector::from_str("(-1)").unwrap()).mod_op(0u64);
}

#[test]
#[should_panic]
fn mod_op_unsigned_empty_vector_fail() {
    let _: UnsignedVector<u32> = (&IntegerVector::from_str("()").unwrap()).mod_op(0u32);
}

fn mod_op_unsigned_properties_helper<
    T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural> + for<'a> TryFrom<&'a Natural>,
>()
where
    Natural: From<T>,
{
    integer_vector_unsigned_pair_gen::<T>().test_properties(|(v, m)| {
        if m == T::ZERO {
            return;
        }
        let w: UnsignedVector<T> = (&v).mod_op(m);
        assert_eq!(v.clone().mod_op(m), w);

        // The result is reduced, and reducing it again changes nothing.
        assert!(w.mod_is_reduced(&m));
        assert_eq!((&w).mod_op(m), w);

        // Apart from its type, the result is the reduction modulo the Natural with the same value.
        let r = (&v).mod_op(Natural::from(m));
        assert_eq!(UnsignedVector::<T>::try_from(&r), Ok(w.clone()));

        // On a vector with no negative elements, this is the `NaturalVector` operation.
        if let Ok(n) = NaturalVector::try_from(&v) {
            assert_eq!(n % m, w);
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
        let v = IntegerVector::from_str(s).unwrap();
        let m = Integer::from_str(m).unwrap();
        // All four combinations of value and reference, and in place with both.
        let w = &v % &m;
        assert_eq!(w.to_string(), out);
        assert_eq!(&v % m.clone(), w);
        assert_eq!(v.clone() % &m, w);
        assert_eq!(v.clone() % m.clone(), w);
        let mut x = v.clone();
        x %= &m;
        assert_eq!(x, w);
        let mut x = v;
        x %= m;
        assert_eq!(x, w);
    };
    test("()", "1", "()");
    test("()", "-7", "()");
    // Modulo 1 every element is zero, but the dimension is unchanged.
    test("(1, -4, -5)", "1", "(0, 0, 0)");
    // Every element keeps its sign, and the sign of the modulus makes no difference.
    test("(1, -4, -5)", "3", "(1, -1, -2)");
    test("(1, -4, -5)", "-3", "(1, -1, -2)");
    // Multiples of the modulus become zero and stay.
    test("(-6, 3, -1)", "-3", "(0, 0, -1)");
    // Elements and divisor of many limbs.
    test(
        "(-1000000000000000000000000, 1)",
        "1234567890987",
        "(-530068894399, 1)",
    );
}

#[test]
#[should_panic]
fn rem_fail() {
    let _ = IntegerVector::from_str("(1)").unwrap() % Integer::ZERO;
}

#[test]
#[should_panic]
fn rem_ref_ref_fail() {
    let _ = &IntegerVector::from_str("(1)").unwrap() % &Integer::ZERO;
}

#[test]
#[should_panic]
fn rem_assign_fail() {
    let mut v = IntegerVector::from_str("(1)").unwrap();
    v %= Integer::ZERO;
}

// The empty vector has no elements, so a zero divisor is never reached by the elementwise loop;
// only the explicit check makes these panic rather than quietly returning the empty vector.
#[test]
#[should_panic]
fn rem_empty_vector_fail() {
    let _ = &IntegerVector::from_str("()").unwrap() % &Integer::ZERO;
}

#[test]
#[should_panic]
fn rem_assign_empty_vector_fail() {
    let mut v = IntegerVector::from_str("()").unwrap();
    v %= &Integer::ZERO;
}

#[test]
fn rem_properties() {
    integer_vector_integer_pair_gen_var_1().test_properties(|(v, m)| {
        let w = &v % &m;
        // The forms agree.
        assert_eq!(&v % m.clone(), w);
        assert_eq!(v.clone() % &m, w);
        assert_eq!(v.clone() % m.clone(), w);
        let mut x = v.clone();
        x %= &m;
        assert_eq!(x, w);
        let mut x = v.clone();
        x %= m.clone();
        assert_eq!(x, w);

        // The sign of the modulus makes no difference, the result commutes with negation, and
        // reducing again changes nothing.
        assert_eq!(&v % -&m, w);
        assert_eq!(-&v % &m, -&w);
        assert_eq!(&w % &m, w);
        assert_eq!(w.dimension(), v.dimension());

        // Element by element, this is the `Integer` operation.
        for (x, y) in v.elements.iter().zip(&w.elements) {
            assert_eq!(*y, x % &m);
        }

        // Reducing into [0, |m|) gives the same result as from the original vector.
        let abs_m = (&m).unsigned_abs();
        assert_eq!((&w).mod_op(&abs_m), (&v).mod_op(&abs_m));
    });
}
