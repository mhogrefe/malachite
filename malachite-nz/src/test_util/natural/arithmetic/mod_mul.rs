// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::platform::{DoubleLimb, Limb};
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::{ExactFrom, JoinHalves, SplitInHalf};

pub fn limbs_precompute_mod_mul_two_limbs_alt(m_1: Limb, m_0: Limb) -> (Limb, Limb, Limb) {
    let out_limbs = (Natural::power_of_2(Limb::WIDTH << 2)
        / Natural::from(DoubleLimb::join_halves(m_1, m_0)))
    .into_limbs_asc();
    assert_eq!(out_limbs.len(), 3);
    (out_limbs[2], out_limbs[1], out_limbs[0])
}

/// m_1 cannot be zero, and we cannot have m_1 == 1 and m_0 == 0. Both [x_0, x_1] and [y_0, y_1]
/// must be less than [m_0, m_1].
pub fn limbs_mod_mul_two_limbs_naive(
    x_1: Limb,
    x_0: Limb,
    y_1: Limb,
    y_0: Limb,
    m_1: Limb,
    m_0: Limb,
) -> (Limb, Limb) {
    DoubleLimb::exact_from(
        &(Natural::from(DoubleLimb::join_halves(x_1, x_0))
            * Natural::from(DoubleLimb::join_halves(y_1, y_0))
            % Natural::from(DoubleLimb::join_halves(m_1, m_0))),
    )
    .split_in_half()
}
