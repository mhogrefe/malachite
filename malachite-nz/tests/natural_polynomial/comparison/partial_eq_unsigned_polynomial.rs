// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::natural_polynomial_unsigned_polynomial_pair_gen;

#[test]
fn test_partial_eq_unsigned_polynomial() {
    fn test<T: PrimitiveUnsigned>(s: &str, t: &str, out: bool)
    where
        Natural: PartialEq<T>,
    {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let q = UnsignedPolynomial::<T>::from_str(t).unwrap();
        assert_eq!(p == q, out);
        assert_eq!(q == p, out);
    }
    test::<u8>("0", "0", true);
    test::<u64>("0", "1", false);
    test::<u32>("1", "0", false);
    test::<u16>("1", "1", true);
    test::<u8>("x", "x", true);
    test::<u8>("x", "x+1", false);
    test::<u128>("x+1", "x", false);
    test::<u8>("3*x^2+255", "3*x^2+255", true);
    test::<u8>("3*x^2+254", "3*x^2+255", false);
    test::<u8>("2*x^2+3", "3*x^2+2", false);
    // A coefficient too large for the coefficient type matches nothing, including the value it
    // would wrap to.
    test::<u8>("3*x^2+256", "3*x^2", false);
    test::<u8>("256*x", "0", false);
    test::<u64>(
        "18446744073709551615*x^2+18446744073709551615",
        "18446744073709551615*x^2+18446744073709551615",
        true,
    );
    test::<u64>("18446744073709551616*x", "x", false);
    // The same polynomial of different degrees: lengths differ.
    test::<usize>("x^3+x", "x^2+x", false);
}

// Comparing with a converted polynomial is the reference the direct comparison is checked against.
#[allow(clippy::cmp_owned, clippy::op_ref)]
fn partial_eq_unsigned_polynomial_properties_helper<T: PrimitiveUnsigned>()
where
    Natural: From<T> + PartialEq<T>,
{
    natural_polynomial_unsigned_polynomial_pair_gen::<T>().test_properties(|(p, q)| {
        let eq = p == q;
        assert_eq!(q == p, eq);
        // Extra refs for type inference: with `NaturalPolynomial: PartialEq<UnsignedPolynomial<T>>`
        // in scope, `p == q` would look for that impl.
        assert_eq!(&p == &NaturalPolynomial::from(q.clone()), eq);
        if eq {
            assert_eq!(p.degree(), q.degree());
        }

        let q_p = NaturalPolynomial::from(q.clone());
        assert!(q_p == q);
        assert!(q == q_p);
    });

    assert!(NaturalPolynomial::ZERO == UnsignedPolynomial::<T>::ZERO);
    assert!(NaturalPolynomial::one() == UnsignedPolynomial::<T>::one());
    assert!(NaturalPolynomial::x() == UnsignedPolynomial::<T>::x());
    assert!(NaturalPolynomial::x() != UnsignedPolynomial::<T>::one());
}

#[test]
fn partial_eq_unsigned_polynomial_properties() {
    apply_fn_to_unsigneds!(partial_eq_unsigned_polynomial_properties_helper);
}
