// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{signed_gen, unsigned_gen};

#[test]
pub fn test_get_highest_bit() {
    assert_eq!(0u8.get_highest_bit(), false);
    assert_eq!(123u32.get_highest_bit(), false);
    assert_eq!(4000000000u32.get_highest_bit(), true);
    assert_eq!(2000000000i32.get_highest_bit(), false);
    assert_eq!((-2000000000i32).get_highest_bit(), true);
}

fn get_highest_bit_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|u| {
        assert_eq!(u.get_highest_bit(), u >= T::power_of_2(T::WIDTH - 1));
    });
}

fn get_highest_bit_properties_helper_signed<T: PrimitiveSigned>() {
    signed_gen::<T>().test_properties(|i| {
        assert_eq!(i.get_highest_bit(), i < T::ZERO);
    });
}

#[test]
fn get_highest_bit_properties() {
    apply_fn_to_unsigneds!(get_highest_bit_properties_helper_unsigned);
    apply_fn_to_signeds!(get_highest_bit_properties_helper_signed);
}
