// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::OrdAbs;
use malachite_base::test_util::generators::{
    signed_gen, signed_pair_gen, signed_triple_gen, unsigned_gen, unsigned_pair_gen_var_27,
    unsigned_triple_gen_var_19,
};
use std::cmp::Ordering::{self, *};

#[test]
pub fn test_cmp_abs_partial_cmp_abs_and_comparators() {
    fn test<T: Copy + OrdAbs>(x: T, y: T, cmp: Ordering, lt: bool, gt: bool, le: bool, ge: bool) {
        assert_eq!(x.cmp_abs(&y), cmp);
        assert_eq!(x.partial_cmp_abs(&y), Some(cmp));
        assert_eq!(lt, x.lt_abs(&y));
        assert_eq!(gt, x.gt_abs(&y));
        assert_eq!(le, x.le_abs(&y));
        assert_eq!(ge, x.ge_abs(&y));
    }
    test(123u16, 123u16, Equal, false, false, true, true);
    test(123u16, 456u16, Less, true, false, true, false);
    test(456u16, 123u16, Greater, false, true, false, true);

    test(123i64, 123i64, Equal, false, false, true, true);
    test(123i64, 456i64, Less, true, false, true, false);
    test(456i64, 123i64, Greater, false, true, false, true);

    test(123i64, -123i64, Equal, false, false, true, true);
    test(123i64, -456i64, Less, true, false, true, false);
    test(456i64, -123i64, Greater, false, true, false, true);

    test(-123i64, 123i64, Equal, false, false, true, true);
    test(-123i64, 456i64, Less, true, false, true, false);
    test(-456i64, 123i64, Greater, false, true, false, true);

    test(-123i64, -123i64, Equal, false, false, true, true);
    test(-123i64, -456i64, Less, true, false, true, false);
    test(-456i64, -123i64, Greater, false, true, false, true);
}

fn properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_27::<T>().test_properties(|(x, y)| {
        let cmp = x.cmp_abs(&y);
        assert_eq!(x.cmp(&y), cmp);
        assert_eq!(x.partial_cmp_abs(&y), Some(cmp));
        assert_eq!(x.lt_abs(&y), cmp == Less);
        assert_eq!(x.gt_abs(&y), cmp == Greater);
        assert_eq!(x.le_abs(&y), cmp != Greater);
        assert_eq!(x.ge_abs(&y), cmp != Less);
        assert_eq!(y.cmp_abs(&x), cmp.reverse());
    });

    unsigned_gen::<T>().test_properties(|x| {
        assert_eq!(x.cmp_abs(&x), Equal);
        assert!(x.le_abs(&x));
        assert!(x.ge_abs(&x));
        assert!(!x.lt_abs(&x));
        assert!(!x.gt_abs(&x));
        assert!(x.le_abs(&T::MAX));
        assert!(x.ge_abs(&T::ZERO));
    });

    unsigned_triple_gen_var_19::<T>().test_properties(|(x, y, z)| {
        if x < y && y < z {
            assert!(x < z);
        } else if x > y && y > z {
            assert!(x > z);
        }
    });
}

fn properties_helper_signed<T: PrimitiveSigned>()
where
    <T as UnsignedAbs>::Output: Ord,
{
    signed_pair_gen::<T>().test_properties(|(x, y)| {
        let cmp = x.cmp_abs(&y);
        if x != T::MIN {
            if y != T::MIN {
                assert_eq!(x.unsigned_abs().cmp(&y.unsigned_abs()), cmp);
            }
            assert_eq!((-x).cmp_abs(&y), cmp);
        }
        if y != T::MIN {
            assert_eq!(x.cmp_abs(&-y), cmp);
        }
        assert_eq!(x.partial_cmp_abs(&y), Some(cmp));
        assert_eq!(x.lt_abs(&y), cmp == Less);
        assert_eq!(x.gt_abs(&y), cmp == Greater);
        assert_eq!(x.le_abs(&y), cmp != Greater);
        assert_eq!(x.ge_abs(&y), cmp != Less);
        assert_eq!(y.cmp_abs(&x), cmp.reverse());
    });

    signed_gen::<T>().test_properties(|x| {
        assert_eq!(x.cmp_abs(&x), Equal);
        assert!(x.le_abs(&x));
        assert!(x.ge_abs(&x));
        assert!(!x.lt_abs(&x));
        assert!(!x.gt_abs(&x));
        assert!(x.le_abs(&T::MIN));
        assert!(x.ge_abs(&T::ZERO));
    });

    signed_triple_gen::<T>().test_properties(|(x, y, z)| {
        if x < y && y < z {
            assert!(x < z);
        } else if x > y && y > z {
            assert!(x > z);
        }
    });
}

#[test]
fn ord_abs_partial_ord_abs_and_comparators_properties() {
    apply_fn_to_unsigneds!(properties_helper_unsigned);
    apply_fn_to_signeds!(properties_helper_signed);
}
