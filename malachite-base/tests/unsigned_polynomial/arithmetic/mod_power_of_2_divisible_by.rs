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
fn test_mod_power_of_2_divisible_by() {
    // - the dividend and divisor are 0
    assert!(
        UnsignedPolynomial::<u8>::ZERO.mod_power_of_2_divisible_by(UnsignedPolynomial::ZERO, 3)
    );
    // - the divisor is 0
    assert!(
        !UnsignedPolynomial::<u8>::from_str("x+1")
            .unwrap()
            .mod_power_of_2_divisible_by(UnsignedPolynomial::ZERO, 3)
    );
    // - divisible
    assert!(
        UnsignedPolynomial::<u8>::from_str("x^2+3*x+2")
            .unwrap()
            .mod_power_of_2_divisible_by(UnsignedPolynomial::from_str("x+1").unwrap(), 3)
    );
    // - not divisible
    assert!(
        !UnsignedPolynomial::<u8>::from_str("x^4+7*x+5")
            .unwrap()
            .mod_power_of_2_divisible_by(UnsignedPolynomial::from_str("5*x^2+2").unwrap(), 3)
    );
    // - modulo 1
    assert!(
        UnsignedPolynomial::<u8>::ZERO.mod_power_of_2_divisible_by(UnsignedPolynomial::ZERO, 0)
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_divisible_by_fail() {
    // The divisor's leading coefficient is not invertible.
    UnsignedPolynomial::<u8>::from_str("2*x^2+x")
        .unwrap()
        .mod_power_of_2_divisible_by(UnsignedPolynomial::from_str("2*x+1").unwrap(), 3);
}

fn mod_power_of_2_divisible_by_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_5::<T>().test_properties(
        |(p, q, pow)| {
            let divisible = (&p).mod_power_of_2_divisible_by(&q, pow);
            assert_eq!(
                (&p).checked_mod_power_of_2_div_exact(&q, pow).is_some(),
                divisible
            );
            assert_eq!(
                divisible,
                (&p).mod_power_of_2_rem(&q, pow) == UnsignedPolynomial::ZERO
            );
        },
    );
}

#[test]
fn mod_power_of_2_divisible_by_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_divisible_by_properties_helper);
}
