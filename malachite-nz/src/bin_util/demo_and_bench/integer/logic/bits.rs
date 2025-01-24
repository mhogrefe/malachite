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
use malachite_nz::test_util::bench::bucketers::pair_1_integer_bit_bucketer;
use malachite_nz::test_util::generators::{integer_gen, integer_unsigned_pair_gen_var_2};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_bits);
    register_demo!(runner, demo_integer_bits_rev);
    register_demo!(runner, demo_integer_bits_index);

    register_bench!(runner, benchmark_integer_bits_get_algorithms);
}

fn demo_integer_bits(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in integer_gen().get(gm, config).take(limit) {
        println!("bits({}) = {:?}", n, n.bits().collect_vec());
    }
}

fn demo_integer_bits_rev(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in integer_gen().get(gm, config).take(limit) {
        println!("bits({}).rev() = {:?}", n, n.bits().rev().collect_vec());
    }
}

fn demo_integer_bits_index(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, i) in integer_unsigned_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!("bits({})[{}] = {:?}", n, i, n.bits()[i]);
    }
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_integer_bits_get_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.bits()[u64]",
        BenchmarkType::Algorithms,
        integer_unsigned_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Integer.bits()[u]", &mut |(n, u)| no_out!(n.bits()[u])),
            ("Integer.to_bits_asc()[u]", &mut |(n, u)| {
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
