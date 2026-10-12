// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::comparison::traits::OrdProduct;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::triple_natural_max_bit_bucketer;
use malachite_nz::test_util::generators::{natural_pair_gen, natural_triple_gen};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_cmp_product);
    register_bench!(runner, benchmark_natural_cmp_product_algorithms);
    register_bench!(runner, benchmark_natural_cmp_product_near_algorithms);
}

fn demo_natural_cmp_product(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c) in natural_triple_gen().get(gm, config).take(limit) {
        println!(
            "Natural::cmp_product(&{a}, &{b}, &{c}) = {:?}",
            Natural::cmp_product(&a, &b, &c)
        );
    }
}

fn benchmark_natural_cmp_product_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::cmp_product(&Natural, &Natural, &Natural)",
        BenchmarkType::Algorithms,
        natural_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("a", "b", "c"),
        &mut [
            ("default", &mut |(a, b, c)| {
                no_out!(Natural::cmp_product(&a, &b, &c));
            }),
            ("multiplying first", &mut |(a, b, c)| {
                no_out!((&a * &b).cmp(&c));
            }),
        ],
    );
}

// Triples `(a, b, a * b)`, where every screen fails and the product must be formed.
fn benchmark_natural_cmp_product_near_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::cmp_product(..), near the product",
        BenchmarkType::Algorithms,
        natural_pair_gen().get(gm, config).map(|(a, b)| {
            let c = &a * &b;
            (a, b, c)
        }),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("a", "b", "c"),
        &mut [
            ("default", &mut |(a, b, c)| {
                no_out!(Natural::cmp_product(&a, &b, &c));
            }),
            ("multiplying first", &mut |(a, b, c)| {
                no_out!((&a * &b).cmp(&c));
            }),
        ],
    );
}
