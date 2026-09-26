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
use malachite_base::polynomial::{MakeMonicMod, MakeMonicModAssign, Polynomial};
use malachite_base::test_util::generators::unsigned_polynomial_unsigned_unsigned_triple_gen_var_2;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

#[test]
fn test_make_monic_mod() {
    fn test<T: PrimitiveUnsigned>(s: &str, m: T, out: Result<&str, T>) {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        let q = (&p).make_monic_mod(m);
        assert_eq!(
            q.as_ref().map(ToString::to_string).map_err(|&e| e),
            out.map(ToString::to_string)
        );
        assert_eq!(p.clone().make_monic_mod(m), q);
        let mut r = p.clone();
        let result = r.make_monic_mod_assign(m);
        match q {
            Ok(q) => {
                assert_eq!(result, Ok(()));
                assert_eq!(r, q);
            }
            Err(g) => {
                assert_eq!(result, Err(g));
                assert_eq!(r, p);
            }
        }
    }
    test::<u8>("0", 7, Ok("0"));
    test::<u8>("3*x^2+x+2", 7, Ok("x^2+5*x+3"));
    test::<u8>("x+1", 7, Ok("x+1"));
    test::<u8>("5", 7, Ok("1"));
    test::<u8>("2*x+1", 4, Err(2));
    test::<u8>("6*x^2+1", 9, Err(3));
    test::<u8>("0", 1, Ok("0"));
    test::<u64>(
        "3*x^3+7",
        18446744073709551557,
        Ok("x^3+6148914691236517188"),
    );
    test::<u128>(
        "2*x+1",
        170141183460469231731687303715884105727,
        Ok("x+85070591730234615865843651857942052864"),
    );
}

#[test]
#[should_panic]
fn make_monic_mod_fail_1() {
    // m is 0.
    let _ = (&UnsignedPolynomial::<u8>::ZERO).make_monic_mod(0);
}

#[test]
#[should_panic]
fn make_monic_mod_fail_2() {
    // A coefficient is not reduced.
    let _ = (&UnsignedPolynomial::<u8>::from_str("7*x+1").unwrap()).make_monic_mod(7);
}

#[test]
#[should_panic]
fn make_monic_mod_assign_fail() {
    // A coefficient is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("7*x+1").unwrap();
    let _ = p.make_monic_mod_assign(7);
}

fn make_monic_mod_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<T>().test_properties(|(p, _, m)| {
        let q = (&p).make_monic_mod(m);
        assert_eq!(p.clone().make_monic_mod(m), q);
        let mut r = p.clone();
        let result = r.make_monic_mod_assign(m);
        let leading = p.leading_coefficient();
        match q {
            Ok(q) => {
                assert!(q.is_valid());
                assert_eq!(result, Ok(()));
                assert_eq!(r, q);
                assert_eq!(q == UnsignedPolynomial::ZERO, p == UnsignedPolynomial::ZERO);
                if p != UnsignedPolynomial::ZERO {
                    assert!(q.is_monic());
                    // q is p times the inverse of its leading coefficient.
                    let inverse = leading.mod_inverse(m).unwrap();
                    for (&c, &d) in p.coefficients_asc().iter().zip(q.coefficients_asc()) {
                        assert_eq!(c.mod_mul(inverse, m), d);
                    }
                    // Making it monic again changes nothing.
                    assert_eq!((&q).make_monic_mod(m), Ok(q));
                }
            }
            Err(g) => {
                assert_eq!(result, Err(g));
                assert_eq!(r, p);
                assert!(leading.mod_inverse(m).is_none());
                assert_eq!(g, leading.gcd(m));
                assert!(g > T::ONE && g < m);
                assert!(m.divisible_by(g));
            }
        }
    });
}

#[test]
fn make_monic_mod_properties() {
    apply_fn_to_unsigneds!(make_monic_mod_properties_helper);
}
