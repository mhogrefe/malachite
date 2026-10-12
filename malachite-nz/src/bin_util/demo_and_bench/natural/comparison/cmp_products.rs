// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::comparison::traits::OrdProducts;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::quadruple_natural_max_bit_bucketer;
use malachite_nz::test_util::generators::{natural_quadruple_gen, natural_triple_gen};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_cmp_products);
    register_bench!(runner, benchmark_natural_cmp_products_algorithms);
    register_bench!(runner, benchmark_natural_cmp_products_near_algorithms);
}

fn demo_natural_cmp_products(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d) in natural_quadruple_gen().get(gm, config).take(limit) {
        println!(
            "Natural::cmp_products(&{a}, &{b}, &{c}, &{d}) = {:?}",
            Natural::cmp_products(&a, &b, &c, &d)
        );
    }
}

fn benchmark_natural_cmp_products_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::cmp_products(&Natural, &Natural, &Natural, &Natural)",
        BenchmarkType::Algorithms,
        natural_quadruple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_natural_max_bit_bucketer("a", "b", "c", "d"),
        &mut [
            ("default", &mut |(a, b, c, d)| {
                no_out!(Natural::cmp_products(&a, &b, &c, &d));
            }),
            ("multiplying first", &mut |(a, b, c, d)| {
                no_out!((&a * &b).cmp(&(&c * &d)));
            }),
        ],
    );
}

// Quadruples `(a, b, c, floor(a * b / c))`, whose products differ by less than `c`, so that the
// screens rarely decide.
fn benchmark_natural_cmp_products_near_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::cmp_products(..), near the product",
        BenchmarkType::Algorithms,
        natural_triple_gen()
            .get(gm, config)
            .filter_map(|(a, b, c)| {
                if c == 0u32 {
                    return None;
                }
                let d = &a * &b / &c;
                Some((a, b, c, d))
            }),
        gm.name(),
        limit,
        file_name,
        &quadruple_natural_max_bit_bucketer("a", "b", "c", "d"),
        &mut [
            ("default", &mut |(a, b, c, d)| {
                no_out!(Natural::cmp_products(&a, &b, &c, &d));
            }),
            ("multiplying first", &mut |(a, b, c, d)| {
                no_out!((&a * &b).cmp(&(&c * &d)));
            }),
        ],
    );
}
