// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::xx_sub_yy_to_zz::explicit_xx_sub_yy_to_zz;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_pair_gen_var_27, unsigned_quadruple_gen_var_10,
};

#[test]
fn test_xx_sub_yy_to_zz() {
    fn test<T: PrimitiveUnsigned>(x_1: T, x_0: T, y_1: T, y_0: T, z_1: T, z_0: T) {
        assert_eq!(T::xx_sub_yy_to_zz(x_1, x_0, y_1, y_0), (z_1, z_0));
        assert_eq!(explicit_xx_sub_yy_to_zz(x_1, x_0, y_1, y_0), (z_1, z_0));
    }
    test::<u32>(0, 0, 0, 0, 0, 0);
    test::<u64>(0x67, 0x89, 0x33, 0x33, 0x34, 0x56);
    test::<u8>(0x78, 0x9a, 0xbc, 0xde, 0xbb, 0xbc);
    test::<u8>(0, 0, 0, 1, u8::MAX, u8::MAX);
    test(u16::MAX, u16::MAX, u16::MAX, u16::MAX, 0, 0);
}

fn xx_sub_yy_to_zz_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_quadruple_gen_var_10::<T>().test_properties(|(x_1, x_0, y_1, y_0)| {
        let (z_1, z_0) = T::xx_sub_yy_to_zz(x_1, x_0, y_1, y_0);
        assert_eq!(explicit_xx_sub_yy_to_zz(x_1, x_0, y_1, y_0), (z_1, z_0));

        assert_eq!(T::xx_add_yy_to_zz(z_1, z_0, y_1, y_0), (x_1, x_0));
        assert_eq!(
            T::xx_sub_yy_to_zz(z_1, z_0, x_1, x_0),
            T::xx_sub_yy_to_zz(T::ZERO, T::ZERO, y_1, y_0)
        );
        assert_eq!(
            T::xx_sub_yy_to_zz(y_1, y_0, x_1, x_0),
            T::xx_sub_yy_to_zz(T::ZERO, T::ZERO, z_1, z_0)
        );

        let (neg_y_1, neg_y_0) = T::xx_sub_yy_to_zz(T::ZERO, T::ZERO, y_1, y_0);
        assert_eq!(T::xx_add_yy_to_zz(x_1, x_0, neg_y_1, neg_y_0), (z_1, z_0));
    });

    unsigned_pair_gen_var_27::<T>().test_properties(|(x_1, x_0)| {
        assert_eq!(T::xx_sub_yy_to_zz(x_1, x_0, T::ZERO, T::ZERO), (x_1, x_0));
        assert_eq!(T::xx_sub_yy_to_zz(x_1, x_0, x_1, x_0), (T::ZERO, T::ZERO));
    });
}

#[test]
fn xx_sub_yy_to_zz_properties() {
    apply_fn_to_unsigneds!(xx_sub_yy_to_zz_properties_helper);
}
