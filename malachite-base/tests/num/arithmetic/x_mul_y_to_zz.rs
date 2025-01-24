// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::x_mul_y_to_zz::explicit_x_mul_y_to_zz;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_gen, unsigned_pair_gen_var_27, unsigned_triple_gen_var_19,
};

#[test]
fn test_x_mul_y_to_zz() {
    fn test<T: PrimitiveUnsigned>(x: T, y: T, z_1: T, z_0: T) {
        assert_eq!(T::x_mul_y_to_zz(x, y), (z_1, z_0));
        assert_eq!(explicit_x_mul_y_to_zz(x, y), (z_1, z_0));
    }
    test::<u32>(0, 0, 0, 0);
    test::<u64>(15, 3, 0, 45);
    test::<u8>(0x78, 0x9a, 0x48, 0x30);
    test::<u8>(u8::MAX, 0, 0, 0);
    test::<u8>(u8::MAX, 1, 0, u8::MAX);
    test(u16::MAX, u16::MAX, u16::MAX - 1, 1);
}

fn x_mul_y_to_zz_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_27::<T>().test_properties(|(x, y)| {
        let p = T::x_mul_y_to_zz(x, y);
        assert_eq!(explicit_x_mul_y_to_zz(x, y), p);
        assert_eq!(T::x_mul_y_to_zz(y, x), p);
    });

    unsigned_gen::<T>().test_properties(|x| {
        assert_eq!(T::x_mul_y_to_zz(x, T::ZERO), (T::ZERO, T::ZERO));
        assert_eq!(T::x_mul_y_to_zz(T::ZERO, x), (T::ZERO, T::ZERO));
        assert_eq!(T::x_mul_y_to_zz(x, T::ONE), (T::ZERO, x));
        assert_eq!(T::x_mul_y_to_zz(T::ONE, x), (T::ZERO, x));
    });

    unsigned_triple_gen_var_19::<T>().test_properties(|(x, y, z)| {
        let product_1 = T::x_mul_y_to_zz(x, y).1;
        let product_2 = T::x_mul_y_to_zz(y, z).1;
        assert_eq!(product_1.wrapping_mul(z), x.wrapping_mul(product_2));
    });
}

#[test]
fn x_mul_y_to_zz_properties() {
    apply_fn_to_unsigneds!(x_mul_y_to_zz_properties_helper);
}
