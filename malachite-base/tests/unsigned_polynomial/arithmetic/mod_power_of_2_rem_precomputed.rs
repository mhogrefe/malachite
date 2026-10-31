// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::*;
use malachite_base::test_util::generators::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

fn verify<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> UnsignedPolynomial<T> {
    let data = UnsignedPolynomial::precompute_mod_power_of_2_rem_data(q, pow);
    let r = p.mod_power_of_2_rem_precomputed(q, pow, &data);
    assert!(r.is_valid());
    assert_eq!(
        p.clone()
            .mod_power_of_2_rem_precomputed(q.clone(), pow, &data),
        r
    );
    assert_eq!(p.clone().mod_power_of_2_rem_precomputed(q, pow, &data), r);
    assert_eq!(p.mod_power_of_2_rem_precomputed(q.clone(), pow, &data), r);
    let mut s = p.clone();
    s.mod_power_of_2_rem_precomputed_assign(q, pow, &data);
    assert_eq!(s, r);
    let mut s = p.clone();
    s.mod_power_of_2_rem_precomputed_assign(q.clone(), pow, &data);
    assert_eq!(s, r);
    assert_eq!(p.mod_power_of_2_rem(q, pow), r);
    r
}

fn test_mod_power_of_2_rem_precomputed_helper<T: PrimitiveUnsigned>() {
    let test = |p, q, pow, out| {
        let p = UnsignedPolynomial::<T>::from_str(p).unwrap();
        let q = UnsignedPolynomial::<T>::from_str(q).unwrap();
        assert_eq!(verify(&p, &q, pow).to_string(), out);
    };
    test("0", "0", 0, "0");
    test("5", "3*x+2", 3, "5");
    test("6*x^2+5", "3", 3, "0");
    test("3*x+5", "x+1", 3, "2");
    test("x^3+2*x+1", "x^2+3", 3, "7*x+1");
    test("x^5+3*x^4+2*x+1", "3*x^3+x+1", 3, "4*x^2+2*x+2");
    test("x^5+3*x^4+2*x+1", "3*x^2+x+1", 3, "6");
}

#[test]
fn test_mod_power_of_2_rem_precomputed() {
    apply_fn_to_unsigneds!(test_mod_power_of_2_rem_precomputed_helper);
}

fn mod_power_of_2_rem_precomputed_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_5::<T>().test_properties(
        |(p, q, pow)| {
            verify(&p, &q, pow);
        },
    );
}

#[test]
fn mod_power_of_2_rem_precomputed_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_rem_precomputed_properties_helper);
}
