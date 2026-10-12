// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::comparison::traits::{OrdProduct, OrdProductNeg};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::bench::bucketers::triple_integer_max_bit_bucketer;
use malachite_nz::test_util::generators::integer_triple_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_cmp_product);
    register_demo!(runner, demo_integer_cmp_product_neg);
    register_bench!(runner, benchmark_integer_cmp_product_algorithms);
    register_bench!(runner, benchmark_integer_cmp_product_neg_algorithms);
}

fn demo_integer_cmp_product(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c) in integer_triple_gen().get(gm, config).take(limit) {
        println!(
            "Integer::cmp_product(&{a}, &{b}, &{c}) = {:?}",
            Integer::cmp_product(&a, &b, &c)
        );
    }
}

fn demo_integer_cmp_product_neg(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c) in integer_triple_gen().get(gm, config).take(limit) {
        println!(
            "Integer::cmp_product_neg(&{a}, &{b}, &{c}) = {:?}",
            Integer::cmp_product_neg(&a, &b, &c)
        );
    }
}

fn benchmark_integer_cmp_product_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer::cmp_product(&Integer, &Integer, &Integer)",
        BenchmarkType::Algorithms,
        integer_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_integer_max_bit_bucketer("a", "b", "c"),
        &mut [
            ("default", &mut |(a, b, c)| {
                no_out!(Integer::cmp_product(&a, &b, &c));
            }),
            ("multiplying first", &mut |(a, b, c)| {
                no_out!((&a * &b).cmp(&c));
            }),
        ],
    );
}

fn benchmark_integer_cmp_product_neg_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer::cmp_product_neg(&Integer, &Integer, &Integer)",
        BenchmarkType::Algorithms,
        integer_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_integer_max_bit_bucketer("a", "b", "c"),
        &mut [
            ("default", &mut |(a, b, c)| {
                no_out!(Integer::cmp_product_neg(&a, &b, &c));
            }),
            ("negating, then multiplying", &mut |(a, b, c)| {
                no_out!((&a * &b).cmp(&-c));
            }),
        ],
    );
}
