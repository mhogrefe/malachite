// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{EqModPowerOf2, ModPowerOf2};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::bucketers::{
    triple_1_2_vec_max_len_bucketer, triple_1_vec_len_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_vec_unsigned_unsigned_triple_gen_var_13,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_9,
};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::eq_mod_power_of_2::{
    limbs_eq_limb_mod_power_of_2, limbs_eq_mod_power_of_2,
};
use malachite_nz::test_util::bench::bucketers::{
    pair_2_triple_1_2_natural_max_bit_bucketer, triple_1_2_natural_max_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    natural_natural_unsigned_triple_gen_var_1, natural_natural_unsigned_triple_gen_var_1_rm,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_eq_limb_mod_power_of_2);
    register_demo!(runner, demo_limbs_eq_mod_power_of_2);
    register_demo!(runner, demo_natural_eq_mod_power_of_2);

    register_bench!(runner, benchmark_limbs_eq_limb_mod_power_of_2);
    register_bench!(runner, benchmark_limbs_eq_mod_power_of_2);
    register_bench!(
        runner,
        benchmark_natural_eq_mod_power_of_2_library_comparison
    );
    register_bench!(runner, benchmark_natural_eq_mod_power_of_2_algorithms);
}

fn demo_limbs_eq_limb_mod_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, y, pow) in unsigned_vec_unsigned_unsigned_triple_gen_var_13()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_eq_limb_mod_power_of_2({:?}, {}, {}) = {:?}",
            xs,
            y,
            pow,
            limbs_eq_limb_mod_power_of_2(&xs, y, pow)
        );
    }
}

fn demo_limbs_eq_mod_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, pow) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_eq_mod_power_of_2({:?}, {:?}, {}) = {:?}",
            xs,
            ys,
            pow,
            limbs_eq_mod_power_of_2(&xs, &ys, pow)
        );
    }
}

fn demo_natural_eq_mod_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, pow) in natural_natural_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        if x.eq_mod_power_of_2(&y, pow) {
            println!("{x} is equal to {y} mod 2^{pow}");
        } else {
            println!("{x} is not equal to {y} mod 2^{pow}");
        }
    }
}

fn benchmark_limbs_eq_limb_mod_power_of_2(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_eq_limb_mod_power_of_2(&[Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_unsigned_triple_gen_var_13().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(ref xs, y, pow)| {
            no_out!(limbs_eq_limb_mod_power_of_2(xs, y, pow))
        })],
    );
}

fn benchmark_limbs_eq_mod_power_of_2(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_eq_mod_power_of_2(&[Limb], &[Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_9().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_vec_max_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(ref xs, ref ys, pow)| {
            no_out!(limbs_eq_mod_power_of_2(xs, ys, pow))
        })],
    );
}

fn benchmark_natural_eq_mod_power_of_2_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.eq_mod_power_of_2(&Natural, u64)",
        BenchmarkType::LibraryComparison,
        natural_natural_unsigned_triple_gen_var_1_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_1_2_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (ref n, ref u, pow))| {
                no_out!(n.eq_mod_power_of_2(u, pow))
            }),
            ("rug", &mut |((ref n, ref u, pow), _)| {
                no_out!(n.is_congruent_2pow(u, u32::exact_from(pow)))
            }),
        ],
    );
}

#[allow(clippy::unnecessary_operation, unused_must_use)]
fn benchmark_natural_eq_mod_power_of_2_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.eq_mod_power_of_2(&Natural, u64)",
        BenchmarkType::Algorithms,
        natural_natural_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_max_bit_bucketer("x", "y"),
        &mut [
            (
                "Natural.eq_mod_power_of_2(&Natural, u64)",
                &mut |(ref x, ref y, pow)| no_out!(x.eq_mod_power_of_2(y, pow)),
            ),
            (
                "Natural.mod_power_of_2(u64) == Natural.mod_power_of_2(u64)",
                &mut |(ref x, ref y, pow)| no_out!(x.mod_power_of_2(pow) == y.mod_power_of_2(pow)),
            ),
        ],
    );
}
