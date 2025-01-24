// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::EqAbs;
use malachite_base::test_util::generators::{
    signed_gen, signed_pair_gen, signed_triple_gen, unsigned_gen, unsigned_pair_gen_var_27,
    unsigned_triple_gen_var_19,
};

#[test]
pub fn test_eq_abs_partial_eq_abs_and_comparators() {
    fn test<T: Copy + EqAbs>(x: T, y: T, eq: bool, ne: bool) {
        assert_eq!(x.eq_abs(&y), eq);
        assert_eq!(y.eq_abs(&x), eq);
        assert_eq!(ne, x.ne_abs(&y));
    }
    test(123u16, 123u16, true, false);
    test(123u16, 456u16, false, true);

    test(123i64, 123i64, true, false);
    test(123i64, 456i64, false, true);

    test(123i64, -123i64, true, false);
    test(123i64, -456i64, false, true);

    test(-123i64, 123i64, true, false);
    test(-123i64, 456i64, false, true);

    test(-123i64, -123i64, true, false);
    test(-123i64, -456i64, false, true);
}

fn properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_27::<T>().test_properties(|(x, y)| {
        let eq = x.eq_abs(&y);
        assert_eq!(x == y, eq);
        assert_eq!(y.eq_abs(&x), eq);
        assert_eq!(y == x, eq);
        assert_eq!(x.ne_abs(&y), !eq);
    });

    unsigned_gen::<T>().test_properties(|x| {
        assert!(x.eq_abs(&x));
        assert!(!x.ne_abs(&x));
    });

    unsigned_triple_gen_var_19::<T>().test_properties(|(x, y, z)| {
        if x == y && y == z {
            assert!(x == z);
        }
    });
}

fn properties_helper_signed<T: PrimitiveSigned>()
where
    <T as UnsignedAbs>::Output: Eq,
{
    signed_pair_gen::<T>().test_properties(|(x, y)| {
        let eq = x.eq_abs(&y);
        if x != T::MIN {
            if y != T::MIN {
                assert_eq!(x.unsigned_abs().eq(&y.unsigned_abs()), eq);
            }
            assert_eq!((-x).eq_abs(&y), eq);
        }
        if y != T::MIN {
            assert_eq!(x.eq_abs(&-y), eq);
        }
        assert_eq!(y.eq_abs(&x), eq);
        assert_eq!(x.ne_abs(&y), !eq);
    });

    signed_gen::<T>().test_properties(|x| {
        assert!(x.eq_abs(&x));
        assert!(!x.ne_abs(&x));
    });

    signed_triple_gen::<T>().test_properties(|(x, y, z)| {
        if x == y && y == z {
            assert!(x == z);
        }
    });
}

#[test]
fn eq_abs_partial_eq_abs_and_comparators_properties() {
    apply_fn_to_unsigneds!(properties_helper_unsigned);
    apply_fn_to_signeds!(properties_helper_signed);
}
