// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::DivisibleBy;
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::generators::{
    rational_polynomial_pair_gen, rational_polynomial_pair_gen_var_4,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_polynomial_divisible_by);
    register_demo!(runner, demo_rational_polynomial_divisible_by_divisible);
}

fn demo_rational_polynomial_divisible_by(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in rational_polynomial_pair_gen().get(gm, config).take(limit) {
        println!("(&({p})).divisible_by(&({q})) = {}", (&p).divisible_by(&q));
    }
}

fn demo_rational_polynomial_divisible_by_divisible(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in rational_polynomial_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).divisible_by(&({q})) = {}", (&p).divisible_by(&q));
    }
}
