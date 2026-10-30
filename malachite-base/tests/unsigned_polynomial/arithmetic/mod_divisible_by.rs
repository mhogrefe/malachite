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
use malachite_base::polynomial::*;
use malachite_base::test_util::generators::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

#[test]
fn test_mod_divisible_by() {
    // - the dividend and divisor are 0
    assert!(UnsignedPolynomial::<u8>::ZERO.mod_divisible_by(UnsignedPolynomial::ZERO, 7));
    // - the divisor is 0
    assert!(
        !UnsignedPolynomial::<u8>::from_str("x+1")
            .unwrap()
            .mod_divisible_by(UnsignedPolynomial::ZERO, 7)
    );
    // - divisible
    assert!(
        UnsignedPolynomial::<u8>::from_str("x^2+3*x+2")
            .unwrap()
            .mod_divisible_by(UnsignedPolynomial::from_str("x+1").unwrap(), 7)
    );
    // - not divisible
    assert!(
        !UnsignedPolynomial::<u8>::from_str("x^3+2*x+1")
            .unwrap()
            .mod_divisible_by(UnsignedPolynomial::from_str("x^2+3").unwrap(), 7)
    );
    // - modulo 1
    assert!(UnsignedPolynomial::<u8>::ZERO.mod_divisible_by(UnsignedPolynomial::ZERO, 1));
}

#[test]
#[should_panic]
fn mod_divisible_by_fail() {
    // The divisor's leading coefficient is not invertible.
    UnsignedPolynomial::<u8>::from_str("2*x^2+x")
        .unwrap()
        .mod_divisible_by(UnsignedPolynomial::from_str("2*x+1").unwrap(), 6);
}

fn mod_divisible_by_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_4::<T>().test_properties(
        |(p, q, m)| {
            let divisible = (&p).mod_divisible_by(&q, m);
            assert_eq!((&p).checked_mod_div_exact(&q, m).is_some(), divisible);
            assert_eq!(divisible, (&p).mod_rem(&q, m) == UnsignedPolynomial::ZERO);
        },
    );
}

#[test]
fn mod_divisible_by_properties() {
    apply_fn_to_unsigneds!(mod_divisible_by_properties_helper);
}
