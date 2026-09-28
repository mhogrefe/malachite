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
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::{
    natural_polynomial_gen, natural_polynomial_unsigned_pair_gen_var_3,
    natural_unsigned_pair_gen_var_4,
};
use malachite_nz::test_util::natural_polynomial::arithmetic::shl::shl_naive;

fn test_shl_helper<T: PrimitiveUnsigned>()
where
    NaturalPolynomial: Shl<T, Output = NaturalPolynomial> + ShlAssign<T>,
    for<'a> &'a NaturalPolynomial: Shl<T, Output = NaturalPolynomial>,
    u64: ExactFrom<T>,
{
    let test = |s, bits: u8, out| {
        let bits = T::from(bits);
        let p = NaturalPolynomial::from_str(s).unwrap();
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
    test("x^2+3*x+5", 0, "x^2+3*x+5");
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
    NaturalPolynomial: Shl<T, Output = NaturalPolynomial> + ShlAssign<T>,
    for<'a> &'a NaturalPolynomial: Shl<T, Output = NaturalPolynomial>,
    Natural: Shl<T, Output = Natural>,
    for<'a> &'a Natural: Shl<T, Output = Natural>,
    u64: ExactFrom<T>,
{
    natural_polynomial_unsigned_pair_gen_var_3::<T>().test_properties(|(p, bits)| {
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
            <&NaturalPolynomial as Shl<u64>>::shl(&p, u64::exact_from(bits)),
            q
        );
        // Evaluation: q(3) = p(3) << bits.
        let three = Natural::from(3u32);
        assert_eq!((&q).evaluate(&three), (&p).evaluate(&three) << bits);
        // The content is shifted too.
        assert_eq!((&q).content(), (&p).content() << bits);
    });

    natural_polynomial_gen().test_properties(|p| {
        // Shifting by 0 changes nothing.
        assert_eq!(&p << T::ZERO, p);
    });

    natural_unsigned_pair_gen_var_4::<T>().test_properties(|(n, bits)| {
        // Shifting a constant polynomial shifts the constant.
        assert_eq!(
            NaturalPolynomial::from(n.clone()) << bits,
            NaturalPolynomial::from(n << bits)
        );
    });
}

#[test]
fn shl_properties() {
    apply_fn_to_unsigneds!(shl_properties_helper);
}
