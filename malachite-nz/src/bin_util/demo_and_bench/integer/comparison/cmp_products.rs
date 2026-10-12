// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::comparison::traits::{OrdProducts, OrdProductsNeg};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::bench::bucketers::quadruple_integer_max_bit_bucketer;
use malachite_nz::test_util::generators::integer_quadruple_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_cmp_products);
    register_demo!(runner, demo_integer_cmp_products_neg);
    register_bench!(runner, benchmark_integer_cmp_products_algorithms);
    register_bench!(runner, benchmark_integer_cmp_products_neg_algorithms);
}

fn demo_integer_cmp_products(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d) in integer_quadruple_gen().get(gm, config).take(limit) {
        println!(
            "Integer::cmp_products(&{a}, &{b}, &{c}, &{d}) = {:?}",
            Integer::cmp_products(&a, &b, &c, &d)
        );
    }
}

fn demo_integer_cmp_products_neg(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d) in integer_quadruple_gen().get(gm, config).take(limit) {
        println!(
            "Integer::cmp_products_neg(&{a}, &{b}, &{c}, &{d}) = {:?}",
            Integer::cmp_products_neg(&a, &b, &c, &d)
        );
    }
}

fn benchmark_integer_cmp_products_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer::cmp_products(&Integer, &Integer, &Integer, &Integer)",
        BenchmarkType::Algorithms,
        integer_quadruple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_integer_max_bit_bucketer("a", "b", "c", "d"),
        &mut [
            ("default", &mut |(a, b, c, d)| {
                no_out!(Integer::cmp_products(&a, &b, &c, &d));
            }),
            ("multiplying first", &mut |(a, b, c, d)| {
                no_out!((&a * &b).cmp(&(&c * &d)));
            }),
        ],
    );
}

fn benchmark_integer_cmp_products_neg_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer::cmp_products_neg(&Integer, &Integer, &Integer, &Integer)",
        BenchmarkType::Algorithms,
        integer_quadruple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_integer_max_bit_bucketer("a", "b", "c", "d"),
        &mut [
            ("default", &mut |(a, b, c, d)| {
                no_out!(Integer::cmp_products_neg(&a, &b, &c, &d));
            }),
            ("multiplying and negating first", &mut |(a, b, c, d)| {
                no_out!((&a * &b).cmp(&-(&c * &d)));
            }),
        ],
    );
}
