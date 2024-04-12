// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{JoinHalves, SplitInHalf};
use malachite_base::test_util::generators::unsigned_gen;

fn lower_half_test_helper<T: PrimitiveUnsigned + SplitInHalf>(n: T, out: T::Half)
where
    T::Half: PrimitiveUnsigned,
{
    assert_eq!(n.lower_half(), out);
}

#[test]
pub fn test_lower_half() {
    lower_half_test_helper(0u64, 0u32);
    lower_half_test_helper(1u64, 1u32);
    lower_half_test_helper(u16::from(u8::MAX), u8::MAX);
    lower_half_test_helper(u16::from(u8::MAX) + 1, 0);
    lower_half_test_helper(u16::MAX, u8::MAX);
    lower_half_test_helper(258u16, 2u8);
    lower_half_test_helper(0xabcd1234u32, 0x1234);
}

fn lower_half_properties_helper<T: JoinHalves + PrimitiveUnsigned + SplitInHalf>() {
    unsigned_gen::<T>().test_properties(|n| {
        let lower = n.lower_half();
        assert_eq!(T::join_halves(n.upper_half(), lower), n);
    });
}

#[test]
fn lower_half_properties() {
    lower_half_properties_helper::<u16>();
    lower_half_properties_helper::<u32>();
    lower_half_properties_helper::<u64>();
    lower_half_properties_helper::<u128>();
}
