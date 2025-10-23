// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::logic::traits::BitBlockAccess;
use malachite_base::test_util::bench::bucketers::quadruple_3_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::large_type_gen_var_4;
use malachite_base::test_util::num::logic::bit_block_access::assign_bits_naive;
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz::integer::logic::bit_block_access::limbs_neg_assign_bits;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::integer_unsigned_unsigned_natural_quadruple_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_neg_assign_bits);
    register_demo!(runner, demo_integer_assign_bits);
    register_bench!(runner, benchmark_limbs_neg_assign_bits);
    register_bench!(runner, benchmark_integer_assign_bits_algorithms);
}

fn demo_limbs_neg_assign_bits(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, start, end, bits) in large_type_gen_var_4().get(gm, config).take(limit) {
        let old_xs = xs.clone();
        limbs_neg_assign_bits(&mut xs, start, end, &bits);
        println!(
            "xs := {old_xs:?}; \
            limbs_neg_assign_bits(&mut xs, {start}, {end}, &{bits:?}); limbs = {xs:?}",
        );
    }
}

fn demo_integer_assign_bits(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut n, start, end, bits) in integer_unsigned_unsigned_natural_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let old_n = n.clone();
        n.assign_bits(start, end, &bits);
        println!("n := {old_n}; n.assign_bits({start}, {end}, &{bits}); n = {n}");
    }
}

fn benchmark_limbs_neg_assign_bits(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_neg_assign_bits(&mut Vec<Limb>, u64, u64, &[Limb])",
        BenchmarkType::Single,
        large_type_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_bucketer("end"),
        &mut [("limbs_neg_assign_bits", &mut |(
            ref mut limbs,
            start,
            end,
            ref bits,
        )| {
            limbs_neg_assign_bits(limbs, start, end, bits);
        })],
    );
}

fn benchmark_integer_assign_bits_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.assign_bits(u64, u64, &Natural)",
        BenchmarkType::Algorithms,
        integer_unsigned_unsigned_natural_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_bucketer("end"),
        &mut [
            ("default", &mut |(mut n, start, end, bits)| {
                n.assign_bits(start, end, &bits);
            }),
            ("naive", &mut |(mut n, start, end, bits)| {
                assign_bits_naive::<Integer, Natural>(&mut n, start, end, &bits);
            }),
        ],
    );
}
