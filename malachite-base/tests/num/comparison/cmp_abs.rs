// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::comparison::traits::OrdAbs;
use malachite_base::test_util::generators::{signed_gen, signed_pair_gen, signed_triple_gen};
use std::cmp::Ordering::{self, *};

#[test]
pub fn test_cmp_abs() {
    fn test<T: Copy + OrdAbs>(x: T, y: T, cmp: Ordering) {
        assert_eq!(x.cmp_abs(&y), cmp);
        assert_eq!(x.partial_cmp_abs(&y), Some(cmp));
        assert_eq!(x.lt_abs(&y), cmp == Less);
        assert_eq!(x.gt_abs(&y), cmp == Greater);
        assert_eq!(x.le_abs(&y), cmp != Greater);
        assert_eq!(x.ge_abs(&y), cmp != Less);
    }
    test(123i64, 123i64, Equal);
    test(123i64, 456i64, Less);
    test(456i64, 123i64, Greater);

    test(123i64, -123i64, Equal);
    test(123i64, -456i64, Less);
    test(456i64, -123i64, Greater);

    test(-123i64, 123i64, Equal);
    test(-123i64, 456i64, Less);
    test(-456i64, 123i64, Greater);

    test(-123i64, -123i64, Equal);
    test(-123i64, -456i64, Less);
    test(-456i64, -123i64, Greater);
}

fn properties_helper_signed<T: PrimitiveSigned>()
where
    <T as UnsignedAbs>::Output: Ord,
{
    signed_pair_gen::<T>().test_properties(|(x, y)| {
        let cmp = x.cmp_abs(&y);
        assert_eq!(x.unsigned_abs().cmp(&y.unsigned_abs()), cmp);
        if x != T::MIN {
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
        if x.lt_abs(&y) && y.lt_abs(&z) {
            assert!(x.lt_abs(&z));
        } else if x.gt_abs(&y) && y.gt_abs(&z) {
            assert!(x.gt_abs(&z));
        }
    });
}

#[test]
fn ord_abs_properties() {
    apply_fn_to_signeds!(properties_helper_signed);
}
