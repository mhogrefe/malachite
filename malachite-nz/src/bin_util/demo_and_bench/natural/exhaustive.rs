// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::natural::exhaustive::{
    exhaustive_natural_inclusive_range, exhaustive_natural_range,
    exhaustive_natural_range_to_infinity, exhaustive_naturals, exhaustive_positive_naturals,
};
use malachite_nz::test_util::generators::{natural_gen, natural_pair_gen};

// The number of elements of a range generator to print.
const PREFIX_LENGTH: usize = 20;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_exhaustive_naturals);
    register_demo!(runner, demo_exhaustive_positive_naturals);
    register_demo!(runner, demo_exhaustive_natural_range);
    register_demo!(runner, demo_exhaustive_natural_inclusive_range);
    register_demo!(runner, demo_exhaustive_natural_range_to_infinity);
}

// The two endpoints of a range, in increasing order.
fn ordered(x: Natural, y: Natural) -> (Natural, Natural) {
    if x <= y { (x, y) } else { (y, x) }
}

fn demo_exhaustive_naturals(_gm: GenMode, _config: &GenConfig, limit: usize) {
    for (i, n) in exhaustive_naturals().take(limit).enumerate() {
        println!("exhaustive_naturals()[{i}] = {n}");
    }
}

fn demo_exhaustive_positive_naturals(_gm: GenMode, _config: &GenConfig, limit: usize) {
    for (i, n) in exhaustive_positive_naturals().take(limit).enumerate() {
        println!("exhaustive_positive_naturals()[{i}] = {n}");
    }
}

fn demo_exhaustive_natural_range(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let (a, b) = ordered(x, y);
        println!(
            "exhaustive_natural_range({a}, {b}) = {:?}",
            exhaustive_natural_range(a.clone(), b.clone())
                .take(PREFIX_LENGTH)
                .collect_vec()
        );
    }
}

fn demo_exhaustive_natural_inclusive_range(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let (a, b) = ordered(x, y);
        println!(
            "exhaustive_natural_inclusive_range({a}, {b}) = {:?}",
            exhaustive_natural_inclusive_range(a.clone(), b.clone())
                .take(PREFIX_LENGTH)
                .collect_vec()
        );
    }
}

fn demo_exhaustive_natural_range_to_infinity(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in natural_gen().get(gm, config).take(limit) {
        println!(
            "exhaustive_natural_range_to_infinity({a}) = {:?}",
            exhaustive_natural_range_to_infinity(a.clone())
                .take(PREFIX_LENGTH)
                .collect_vec()
        );
    }
}
