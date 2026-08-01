// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{signed_quadruple_gen, unsigned_quadruple_gen};

#[test]
fn test_mul_add_mul() {
    fn test<T: PrimitiveInt>(x: T, y: T, z: T, w: T, out: T) {
        assert_eq!(x.mul_add_mul(y, z, w), out);
        assert_eq!(x.wrapping_mul_add_mul(y, z, w), out);

        let mut x_alt = x;
        x_alt.mul_add_mul_assign(y, z, w);
        assert_eq!(x_alt, out);

        let mut x_alt = x;
        x_alt.wrapping_mul_add_mul_assign(y, z, w);
        assert_eq!(x_alt, out);
    }
    test::<u8>(0, 0, 0, 0, 0);
    test::<u32>(7, 5, 10, 3, 65);
    test::<u64>(123, 456, 789, 12, 65556);
    test::<i32>(123, -456, 789, 12, -46620);
    test::<i128>(-123, 456, 789, -12, -65556);
    test::<u8>(200, 200, 100, 100, 80);
    test::<i8>(100, 100, 100, 100, 32);
}

// The wide helper must agree with exact arithmetic done another way, so the checked, saturating,
// wrapping, and overflowing variants are all cross-checked against each other and against the value
// computed in a type twice as wide.
fn mul_add_mul_properties_helper_int<T: PrimitiveInt, W: PrimitiveInt>(x: T, y: T, z: T, w: T)
where
    W: From<T> + TryInto<T>,
{
    let wrapped = x.mul_add_mul(y, z, w);
    assert_eq!(x.wrapping_mul_add_mul(y, z, w), wrapped);

    let mut x_alt = x;
    x_alt.wrapping_mul_add_mul_assign(y, z, w);
    assert_eq!(x_alt, wrapped);

    // The exact value, computed in a wider type.
    let exact = W::from(x) * W::from(y) + W::from(z) * W::from(w);
    let fits: Option<T> = exact.try_into().ok();

    assert_eq!(x.checked_mul_add_mul(y, z, w), fits);

    let (result, overflow) = x.overflowing_mul_add_mul(y, z, w);
    assert_eq!(result, wrapped);
    assert_eq!(overflow, fits.is_none());

    let mut x_alt = x;
    assert_eq!(x_alt.overflowing_mul_add_mul_assign(y, z, w), overflow);
    assert_eq!(x_alt, wrapped);

    let saturated = x.saturating_mul_add_mul(y, z, w);
    match fits {
        Some(v) => assert_eq!(saturated, v),
        None => assert_eq!(
            saturated,
            if exact > W::from(T::MAX) {
                T::MAX
            } else {
                T::MIN
            }
        ),
    }

    let mut x_alt = x;
    x_alt.saturating_mul_add_mul_assign(y, z, w);
    assert_eq!(x_alt, saturated);

    // Each product is symmetric in its own factors.
    assert_eq!(y.mul_add_mul(x, z, w), wrapped);
    assert_eq!(x.mul_add_mul(y, w, z), wrapped);
    // Negating one factor of the second product turns one operation into the other.
    assert_eq!(x.mul_sub_mul(y, z.wrapping_neg(), w), wrapped);
}

fn mul_add_mul_properties_helper_unsigned<T: PrimitiveUnsigned, W: PrimitiveInt>()
where
    W: From<T> + TryInto<T>,
{
    unsigned_quadruple_gen::<T>().test_properties(|(x, y, z, w)| {
        mul_add_mul_properties_helper_int::<T, W>(x, y, z, w);
    });
}

fn mul_add_mul_properties_helper_signed<T: PrimitiveSigned, W: PrimitiveInt>()
where
    W: From<T> + TryInto<T>,
{
    signed_quadruple_gen::<T>().test_properties(|(x, y, z, w)| {
        mul_add_mul_properties_helper_int::<T, W>(x, y, z, w);
    });
}

#[test]
fn mul_add_mul_properties() {
    mul_add_mul_properties_helper_unsigned::<u8, i64>();
    mul_add_mul_properties_helper_unsigned::<u16, i64>();
    mul_add_mul_properties_helper_unsigned::<u32, i128>();
    mul_add_mul_properties_helper_signed::<i8, i64>();
    mul_add_mul_properties_helper_signed::<i16, i64>();
    mul_add_mul_properties_helper_signed::<i32, i128>();
}
