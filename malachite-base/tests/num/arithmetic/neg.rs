// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::generators::{primitive_float_gen, signed_gen_var_1};

fn neg_assign_helper_signed<T: PrimitiveSigned>() {
    let test = |n: T, out| {
        let mut n = n;
        n.neg_assign();
        assert_eq!(n, out);
    };
    test(T::ZERO, T::ZERO);
    test(T::ONE, T::NEGATIVE_ONE);
    test(T::exact_from(100), T::exact_from(-100));
    test(T::NEGATIVE_ONE, T::ONE);
    test(T::exact_from(-100), T::exact_from(100));
}

fn neg_assign_helper_primitive_float<T: PrimitiveFloat>() {
    let test = |x: T, out| {
        let mut x = x;
        x.neg_assign();
        assert_eq!(NiceFloat(x), NiceFloat(out));
    };
    test(T::ZERO, T::NEGATIVE_ZERO);
    test(T::NEGATIVE_ZERO, T::ZERO);
    test(T::INFINITY, T::NEGATIVE_INFINITY);
    test(T::NEGATIVE_INFINITY, T::INFINITY);
    test(T::NAN, T::NAN);

    test(T::ONE, T::NEGATIVE_ONE);
    test(T::from(100.0), T::from(-100.0));
    test(T::NEGATIVE_ONE, T::ONE);
    test(T::from(-100.0), T::from(100.0));
}

#[test]
fn test_neg_assign() {
    apply_fn_to_signeds!(neg_assign_helper_signed);
    apply_fn_to_primitive_floats!(neg_assign_helper_primitive_float);
}

fn neg_assign_properties_helper_signed<T: PrimitiveSigned>() {
    signed_gen_var_1::<T>().test_properties(|n| {
        let mut neg = n;
        neg.neg_assign();
        assert_eq!(neg, -n);
        assert_eq!(-neg, n);
        assert_eq!(neg == n, n == T::ZERO);
        assert_eq!(n + neg, T::ZERO);
    });
}

fn neg_assign_properties_helper_primitive_float<T: PrimitiveFloat>() {
    primitive_float_gen::<T>().test_properties(|x| {
        let mut neg = x;
        neg.neg_assign();
        assert_eq!(NiceFloat(neg), NiceFloat(-x));
        assert_eq!(NiceFloat(-neg), NiceFloat(x));
        assert_eq!(NiceFloat(neg) == NiceFloat(x), x.is_nan());
        if x.is_finite() {
            assert_eq!(x + neg, T::ZERO);
        }
    });
}

#[test]
fn neg_assign_properties() {
    apply_fn_to_signeds!(neg_assign_properties_helper_signed);
    apply_fn_to_primitive_floats!(neg_assign_properties_helper_primitive_float);
}
