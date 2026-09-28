// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::ops::{Shl, ShlAssign};
use core::str::FromStr;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{Content, Evaluate, Polynomial};
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{
    integer_polynomial_gen, integer_polynomial_unsigned_pair_gen_var_3,
    integer_unsigned_pair_gen_var_2,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::shl::shl_naive;

fn test_shl_helper<T: PrimitiveUnsigned>()
where
    IntegerPolynomial: Shl<T, Output = IntegerPolynomial> + ShlAssign<T>,
    for<'a> &'a IntegerPolynomial: Shl<T, Output = IntegerPolynomial>,
    u64: ExactFrom<T>,
{
    let test = |s, bits: u8, out| {
        let bits = T::from(bits);
        let p = IntegerPolynomial::from_str(s).unwrap();
        let q = &p << bits;
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
        assert_eq!(p.clone() << bits, q);
        let mut r = p.clone();
        r <<= bits;
        assert!(r.is_valid());
        assert_eq!(r, q);
        assert_eq!(shl_naive(&p, bits), q);
    };
    // The zero polynomial stays zero.
    test("0", 0, "0");
    test("0", 10, "0");
    // Shifting by 0 changes nothing.
    test("x^2-3*x+5", 0, "x^2-3*x+5");
    // Every coefficient is shifted.
    test("x^2+3*x+5", 2, "4*x^2+12*x+20");
    test("x", 1, "2*x");
    test(
        "7*x^3+1",
        100,
        "8873554201597605810476922437632*x^3+1267650600228229401496703205376",
    );
    test(
        "1000000000000000000000*x+1",
        64,
        "18446744073709551616000000000000000000000*x+18446744073709551616",
    );
}

#[test]
fn test_shl() {
    apply_fn_to_unsigneds!(test_shl_helper);
}

fn shl_properties_helper<T: PrimitiveUnsigned>()
where
    IntegerPolynomial: Shl<T, Output = IntegerPolynomial> + ShlAssign<T>,
    for<'a> &'a IntegerPolynomial: Shl<T, Output = IntegerPolynomial>,
    Integer: Shl<T, Output = Integer>,
    for<'a> &'a Integer: Shl<T, Output = Integer>,
    Natural: Shl<T, Output = Natural>,
    u64: ExactFrom<T>,
{
    integer_polynomial_unsigned_pair_gen_var_3::<T>().test_properties(|(p, bits)| {
        let q = &p << bits;
        assert!(q.is_valid());
        // The forms agree.
        assert_eq!(p.clone() << bits, q);
        let mut r = p.clone();
        r <<= bits;
        assert!(r.is_valid());
        assert_eq!(r, q);
        assert_eq!(shl_naive(&p, bits), q);

        // Every coefficient is shifted, and the degree is unchanged.
        assert_eq!(q.degree(), p.degree());
        for i in 0..p.len() {
            assert_eq!(*q.coefficient(i), p.coefficient(i) << bits);
        }
        // Shifting by the same amount of another type gives the same result.
        assert_eq!(
            <&IntegerPolynomial as Shl<u64>>::shl(&p, u64::exact_from(bits)),
            q
        );
        // Evaluation: q(-3) = p(-3) << bits.
        let x = Integer::from(-3i32);
        assert_eq!((&q).evaluate(&x), (&p).evaluate(&x) << bits);
        // The content is shifted too.
        assert_eq!((&q).content(), (&p).content() << bits);
        // It commutes with negation.
        assert_eq!(-&p << bits, -&q);
    });

    integer_polynomial_gen().test_properties(|p| {
        // Shifting by 0 changes nothing.
        assert_eq!(&p << T::ZERO, p);
    });

    integer_unsigned_pair_gen_var_2::<T>().test_properties(|(n, bits)| {
        // Shifting a constant polynomial shifts the constant.
        assert_eq!(
            IntegerPolynomial::from(n.clone()) << bits,
            IntegerPolynomial::from(n << bits)
        );
    });
}

#[test]
fn shl_properties() {
    apply_fn_to_unsigneds!(shl_properties_helper);
}
