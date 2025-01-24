// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::{
    natural_bit_bucketer, pair_1_natural_bit_bucketer,
};
use malachite_nz::test_util::generators::{natural_gen, natural_unsigned_pair_gen_var_4};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_to_limbs_asc);
    register_demo!(runner, demo_natural_to_limbs_desc);
    register_demo!(runner, demo_natural_into_limbs_asc);
    register_demo!(runner, demo_natural_into_limbs_desc);
    register_demo!(runner, demo_natural_limbs);
    register_demo!(runner, demo_natural_limbs_rev);
    register_demo!(runner, demo_natural_limbs_size_hint);
    register_demo!(runner, demo_natural_limbs_index);
    register_demo!(runner, demo_natural_limb_count);

    register_bench!(runner, benchmark_natural_limbs_evaluation_strategy);
    register_bench!(runner, benchmark_natural_limbs_rev_evaluation_strategy);
    register_bench!(runner, benchmark_natural_limbs_size_hint);
    register_bench!(runner, benchmark_natural_limbs_index_algorithms);
    register_bench!(runner, benchmark_natural_limb_count);
}

fn demo_natural_to_limbs_asc(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        println!("to_limbs_asc({}) = {:?}", n, n.to_limbs_asc());
    }
}

fn demo_natural_to_limbs_desc(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        println!("to_limbs_desc({}) = {:?}", n, n.to_limbs_desc());
    }
}

fn demo_natural_into_limbs_asc(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        println!("into_limbs_asc({}) = {:?}", n, n.clone().into_limbs_asc());
    }
}

fn demo_natural_into_limbs_desc(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        println!("into_limbs_desc({}) = {:?}", n, n.clone().into_limbs_desc());
    }
}

fn demo_natural_limbs(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        println!("limbs({}) = {:?}", n, n.limbs().collect_vec());
    }
}

fn demo_natural_limbs_rev(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        println!("limbs({}).rev() = {:?}", n, n.limbs().rev().collect_vec());
    }
}

fn demo_natural_limbs_size_hint(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        println!("limbs({}).size_hint() = {:?}", n, n.limbs().size_hint());
    }
}

fn demo_natural_limbs_index(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, i) in natural_unsigned_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!("limbs({})[{}] = {:?}", n, i, n.limbs()[i]);
    }
}

fn demo_natural_limb_count(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        println!("limb_count({}) = {}", n, n.limb_count());
    }
}

fn benchmark_natural_limbs_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.limbs()",
        BenchmarkType::EvaluationStrategy,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [
            ("Natural.to_limbs_asc()", &mut |n| no_out!(n.to_limbs_asc())),
            ("Natural.into_limbs_asc()", &mut |n| {
                no_out!(n.into_limbs_asc())
            }),
            ("Natural.limbs().collect_vec()", &mut |n| {
                no_out!(n.limbs().collect_vec())
            }),
        ],
    );
}

fn benchmark_natural_limbs_rev_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.limbs().rev()",
        BenchmarkType::EvaluationStrategy,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [
            ("Natural.to_limbs_desc()", &mut |n| {
                no_out!(n.to_limbs_desc())
            }),
            ("Natural.into_limbs_desc()", &mut |n| {
                no_out!(n.into_limbs_desc())
            }),
            ("Natural.limbs().rev().collect_vec()", &mut |n| {
                no_out!(n.limbs().rev().collect_vec())
            }),
        ],
    );
}

fn benchmark_natural_limbs_size_hint(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.limbs().size_hint()",
        BenchmarkType::Single,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [("Natural.limbs().size_hint()", &mut |n| {
            no_out!(n.limbs().size_hint())
        })],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_natural_limbs_index_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.limbs()[usize]",
        BenchmarkType::Algorithms,
        natural_unsigned_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [
            ("Natural.limbs()[u]", &mut |(n, u)| no_out!(n.limbs()[u])),
            ("Natural.into_limbs_asc()[u]", &mut |(n, u)| {
                let limbs = n.into_limbs_asc();
                if u >= limbs.len() {
                    0
                } else {
                    limbs[u]
                };
            }),
        ],
    );
}

fn benchmark_natural_limb_count(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Natural.limb_count()",
        BenchmarkType::Single,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [("Malachite", &mut |n| no_out!(n.limb_count()))],
    );
}
