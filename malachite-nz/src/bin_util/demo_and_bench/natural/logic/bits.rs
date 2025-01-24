// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitConvertible, BitIterable};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::{
    natural_bit_bucketer, pair_1_natural_bit_bucketer,
};
use malachite_nz::test_util::generators::{natural_gen, natural_unsigned_pair_gen_var_4};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_bits);
    register_demo!(runner, demo_natural_bits_rev);
    register_demo!(runner, demo_natural_bits_size_hint);

    register_bench!(runner, benchmark_natural_bits_size_hint);
    register_bench!(runner, benchmark_natural_bits_get_algorithms);
}

fn demo_natural_bits(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        println!("bits({}) = {:?}", n, n.bits().collect_vec());
    }
}

fn demo_natural_bits_rev(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        println!("bits({}).rev() = {:?}", n, n.bits().rev().collect_vec());
    }
}

fn demo_natural_bits_size_hint(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        println!("bits({}).size_hint() = {:?}", n, n.bits().size_hint());
    }
}

fn benchmark_natural_bits_size_hint(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.bits().size_hint()",
        BenchmarkType::Single,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [("Natural.bits().size_hint()", &mut |n| {
            no_out!(n.bits().size_hint())
        })],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_natural_bits_get_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.bits()[u64]",
        BenchmarkType::Algorithms,
        natural_unsigned_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [
            ("Natural.bits()[u]", &mut |(n, u)| no_out!(n.bits()[u])),
            ("Natural.to_bits_asc()[u]", &mut |(n, u)| {
                let bits = n.to_bits_asc();
                let u = usize::exact_from(u);
                if u >= bits.len() {
                    n < 0
                } else {
                    bits[u]
                };
            }),
        ],
    );
}
