// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::sub::limbs_sub_limb_in_place;
use crate::natural::logic::not::limbs_not_in_place;
use crate::platform::Limb;
use malachite_base::num::arithmetic::traits::WrappingNegAssign;

pub fn limbs_twos_complement_in_place_alt_1(limbs: &mut [Limb]) -> bool {
    let i = limbs.iter().copied().take_while(|&x| x == 0).count();
    let len = limbs.len();
    if i == len {
        return true;
    }
    limbs[i].wrapping_neg_assign();
    let j = i + 1;
    if j != len {
        limbs_not_in_place(&mut limbs[j..]);
    }
    false
}

pub fn limbs_twos_complement_in_place_alt_2(limbs: &mut [Limb]) -> bool {
    let carry = limbs_sub_limb_in_place(limbs, 1);
    limbs_not_in_place(limbs);
    carry
}
