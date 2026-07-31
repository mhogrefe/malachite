// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::{OrdAbsDouble, OrdDouble};
use malachite_base::test_util::generators::{signed_pair_gen, unsigned_pair_gen_var_27};
use std::cmp::Ordering::*;

#[test]
fn test_cmp_double() {
    assert_eq!(4u32.cmp_double(&2), Equal);
    assert_eq!(3u32.cmp_double(&2), Less);
    assert_eq!(5u32.cmp_double(&2), Greater);
    assert_eq!(0u32.cmp_double(&0), Equal);
    assert_eq!(0u32.cmp_double(&1), Less);
    assert_eq!(1u32.cmp_double(&0), Greater);
    // - the doubling would overflow, so the doubled value exceeds anything representable
    assert_eq!(u32::MAX.cmp_double(&(1 << 31)), Less);
    assert_eq!(u8::MAX.cmp_double(&128), Less);
    // - just below the overflow boundary
    assert_eq!(u8::MAX.cmp_double(&127), Greater);
    assert_eq!(254u8.cmp_double(&127), Equal);
}

#[test]
fn test_cmp_abs_double() {
    assert_eq!(4i32.cmp_abs_double(&2), Equal);
    // - only the magnitudes matter
    assert_eq!((-4i32).cmp_abs_double(&2), Equal);
    assert_eq!(4i32.cmp_abs_double(&-2), Equal);
    assert_eq!((-4i32).cmp_abs_double(&-2), Equal);
    assert_eq!(3i32.cmp_abs_double(&-2), Less);
    assert_eq!((-5i32).cmp_abs_double(&2), Greater);
    // - the most negative value, whose magnitude is not representable
    assert_eq!(i8::MIN.cmp_abs_double(&(i8::MIN >> 1)), Equal);
    assert_eq!(i8::MIN.cmp_abs_double(&i8::MIN), Less);
    assert_eq!(i8::MAX.cmp_abs_double(&64), Less);
    assert_eq!(i8::MIN.cmp_abs_double(&63), Greater);
}

// A wider type computes the doubled value outright, which is the comparison these avoid.
fn cmp_double_properties_helper_unsigned<T: PrimitiveUnsigned + Into<u128>>() {
    unsigned_pair_gen_var_27::<T>().test_properties(|(x, y)| {
        let c = x.cmp_double(&y);
        assert_eq!(c, x.into().cmp(&(y.into() << 1)));
        // a doubled value is at least the original
        if c == Greater {
            assert_eq!(x.cmp(&y), Greater);
        }
    });
}

fn cmp_double_properties_helper_signed<T: PrimitiveSigned>()
where
    <T as UnsignedAbs>::Output: PrimitiveUnsigned + Into<u128>,
{
    signed_pair_gen::<T>().test_properties(|(x, y)| {
        let c = x.cmp_abs_double(&y);
        let (ax, ay): (u128, u128) = (x.unsigned_abs().into(), y.unsigned_abs().into());
        assert_eq!(c, ax.cmp(&(ay << 1)));
        // sign changes are invisible
        if x != T::MIN {
            assert_eq!((-x).cmp_abs_double(&y), c);
        }
        if y != T::MIN {
            assert_eq!(x.cmp_abs_double(&-y), c);
        }
        assert_eq!(c, x.unsigned_abs().cmp_double(&y.unsigned_abs()));
    });
}

#[test]
fn cmp_double_properties() {
    cmp_double_properties_helper_unsigned::<u8>();
    cmp_double_properties_helper_unsigned::<u16>();
    cmp_double_properties_helper_unsigned::<u32>();
    cmp_double_properties_helper_unsigned::<u64>();
    cmp_double_properties_helper_signed::<i8>();
    cmp_double_properties_helper_signed::<i16>();
    cmp_double_properties_helper_signed::<i32>();
    cmp_double_properties_helper_signed::<i64>();
    // the widest types, where the doubled value has no wider type to be computed in
    unsigned_pair_gen_var_27::<u128>().test_properties(|(x, y)| {
        let c = x.cmp_double(&y);
        assert_eq!(
            c,
            if y.leading_zeros() == 0 {
                Less
            } else {
                x.cmp(&(y << 1))
            }
        );
    });
    signed_pair_gen::<i128>().test_properties(|(x, y)| {
        assert_eq!(
            x.cmp_abs_double(&y),
            x.unsigned_abs().cmp_double(&y.unsigned_abs())
        );
    });
    let _ = i128::MIN.cmp_abs_double(&i128::MIN);
}
