// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::bucketers::vec_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{unsigned_vec_gen, unsigned_vec_gen_var_2};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::conversion::to_twos_complement_limbs::{
    limbs_maybe_sign_extend_non_negative_in_place, limbs_twos_complement,
    limbs_twos_complement_and_maybe_sign_extend_negative_in_place, limbs_twos_complement_in_place,
};
use malachite_nz::platform::Limb;
use malachite_nz::test_util::bench::bucketers::{
    integer_bit_bucketer, pair_1_integer_bit_bucketer,
};
use malachite_nz::test_util::generators::{integer_gen, integer_unsigned_pair_gen_var_2};
use malachite_nz::test_util::integer::conversion::to_twos_complement_limbs::{
    limbs_twos_complement_in_place_alt_1, limbs_twos_complement_in_place_alt_2,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_twos_complement);
    register_demo!(runner, demo_limbs_maybe_sign_extend_non_negative_in_place);
    register_demo!(runner, demo_limbs_twos_complement_in_place);
    register_demo!(
        runner,
        demo_limbs_twos_complement_and_maybe_sign_extend_negative_in_place
    );
    register_demo!(runner, demo_integer_to_twos_complement_limbs_asc);
    register_demo!(runner, demo_integer_to_twos_complement_limbs_desc);
    register_demo!(runner, demo_integer_into_twos_complement_limbs_asc);
    register_demo!(runner, demo_integer_into_twos_complement_limbs_desc);
    register_demo!(runner, demo_integer_twos_complement_limbs);
    register_demo!(runner, demo_integer_twos_complement_limbs_rev);
    register_demo!(runner, demo_integer_twos_complement_limbs_get);
    register_demo!(runner, demo_integer_twos_complement_limb_count);

    register_bench!(runner, benchmark_limbs_twos_complement);
    register_bench!(
        runner,
        benchmark_limbs_maybe_sign_extend_non_negative_in_place
    );
    register_bench!(runner, benchmark_limbs_twos_complement_in_place_algorithms);
    register_bench!(
        runner,
        benchmark_limbs_twos_complement_and_maybe_sign_extend_negative_in_place
    );
    register_bench!(
        runner,
        benchmark_integer_to_twos_complement_limbs_asc_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_integer_to_twos_complement_limbs_desc_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_integer_twos_complement_limbs_get_algorithms
    );
    register_bench!(runner, benchmark_integer_twos_complement_limb_count);
}

fn demo_limbs_twos_complement(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_vec_gen_var_2().get(gm, config).take(limit) {
        println!(
            "limbs_twos_complement({:?}) = {:?}",
            xs,
            limbs_twos_complement(&xs)
        );
    }
}

fn demo_limbs_maybe_sign_extend_non_negative_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for xs in unsigned_vec_gen().get(gm, config).take(limit) {
        let mut mut_xs = xs.clone();
        limbs_maybe_sign_extend_non_negative_in_place(&mut mut_xs);
        println!(
            "xs := {xs:?}; limbs_maybe_sign_extend_non_negative_in_place(&mut xs); xs = {mut_xs:?}",
        );
    }
}

fn demo_limbs_twos_complement_in_place(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_vec_gen().get(gm, config).take(limit) {
        let mut mut_xs = xs.clone();
        let carry = limbs_twos_complement_in_place(&mut mut_xs);
        println!(
            "xs := {xs:?}; limbs_twos_complement_in_place(&mut xs) = {carry}; xs = {mut_xs:?}",
        );
    }
}

fn demo_limbs_twos_complement_and_maybe_sign_extend_negative_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for xs in unsigned_vec_gen_var_2().get(gm, config).take(limit) {
        let mut mut_xs = xs.clone();
        limbs_twos_complement_and_maybe_sign_extend_negative_in_place(&mut mut_xs);
        println!(
            "xs := {xs:?}; limbs_twos_complement_and_maybe_sign_extend_negative_in_place(&mut xs); \
            xs = {mut_xs:?}",
        );
    }
}

fn demo_integer_to_twos_complement_limbs_asc(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in integer_gen().get(gm, config).take(limit) {
        println!(
            "to_twos_complement_limbs_asc({}) = {:?}",
            n,
            n.to_twos_complement_limbs_asc()
        );
    }
}

fn demo_integer_to_twos_complement_limbs_desc(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in integer_gen().get(gm, config).take(limit) {
        println!(
            "to_twos_complement_limbs_desc({}) = {:?}",
            n,
            n.to_twos_complement_limbs_desc()
        );
    }
}

fn demo_integer_into_twos_complement_limbs_asc(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in integer_gen().get(gm, config).take(limit) {
        println!(
            "into_twos_complement_limbs_asc({}) = {:?}",
            n,
            n.clone().into_twos_complement_limbs_asc()
        );
    }
}

fn demo_integer_into_twos_complement_limbs_desc(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in integer_gen().get(gm, config).take(limit) {
        println!(
            "into_twos_complement_limbs_desc({}) = {:?}",
            n,
            n.clone().into_twos_complement_limbs_desc()
        );
    }
}

fn demo_integer_twos_complement_limbs(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in integer_gen().get(gm, config).take(limit) {
        println!(
            "twos_complement_limbs({}) = {:?}",
            n,
            n.twos_complement_limbs().collect_vec()
        );
    }
}

fn demo_integer_twos_complement_limbs_rev(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in integer_gen().get(gm, config).take(limit) {
        println!(
            "twos_complement_limbs({}).rev() = {:?}",
            n,
            n.twos_complement_limbs().rev().collect_vec()
        );
    }
}

fn demo_integer_twos_complement_limbs_get(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, i) in integer_unsigned_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "twos_complement_limbs({}).get({}) = {:?}",
            n,
            i,
            n.twos_complement_limbs().get(i)
        );
    }
}

fn demo_integer_twos_complement_limb_count(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in integer_gen().get(gm, config).take(limit) {
        println!(
            "twos_complement_limb_count({}) = {}",
            n,
            n.twos_complement_limb_count()
        );
    }
}

fn benchmark_limbs_twos_complement(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_twos_complement(&[Limb])",
        BenchmarkType::Single,
        unsigned_vec_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [("Malachite", &mut |xs| no_out!(limbs_twos_complement(&xs)))],
    );
}

fn benchmark_limbs_maybe_sign_extend_non_negative_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_maybe_sign_extend_non_negative_in_place(&mut [Limb])",
        BenchmarkType::Single,
        unsigned_vec_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [("Malachite", &mut |ref mut xs| {
            limbs_maybe_sign_extend_non_negative_in_place(xs)
        })],
    );
}

fn benchmark_limbs_twos_complement_in_place_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_twos_complement_in_place(&mut [Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [
            ("default", &mut |ref mut xs| {
                no_out!(limbs_twos_complement_in_place(xs))
            }),
            ("integrated", &mut |ref mut xs| {
                no_out!(limbs_twos_complement_in_place_alt_1(xs))
            }),
            ("sub 1 and not", &mut |ref mut xs| {
                no_out!(limbs_twos_complement_in_place_alt_2(xs))
            }),
        ],
    );
}

fn benchmark_limbs_twos_complement_and_maybe_sign_extend_negative_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_twos_complement_and_maybe_sign_extend_negative_in_place(&mut [Limb])",
        BenchmarkType::Single,
        unsigned_vec_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [("Malachite", &mut |ref mut xs| {
            limbs_twos_complement_and_maybe_sign_extend_negative_in_place(xs)
        })],
    );
}

fn benchmark_integer_to_twos_complement_limbs_asc_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.to_twos_complement_limbs_asc()",
        BenchmarkType::EvaluationStrategy,
        integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("n"),
        &mut [
            ("Integer.to_twos_complement_limbs_asc()", &mut |n| {
                no_out!(n.to_twos_complement_limbs_asc())
            }),
            ("Integer.into_twos_complement_limbs_asc()", &mut |n| {
                no_out!(n.into_twos_complement_limbs_asc())
            }),
            ("Integer.twos_complement_limbs().collect_vec()", &mut |n| {
                no_out!(n.twos_complement_limbs().collect_vec())
            }),
        ],
    );
}

fn benchmark_integer_to_twos_complement_limbs_desc_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.to_twos_complement_limbs_desc()",
        BenchmarkType::EvaluationStrategy,
        integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("n"),
        &mut [
            ("Integer.to_twos_complement_limbs_desc()", &mut |n| {
                no_out!(n.to_twos_complement_limbs_desc())
            }),
            ("Integer.into_twos_complement_limbs_desc()", &mut |n| {
                no_out!(n.into_twos_complement_limbs_desc())
            }),
            (
                "Integer.twos_complement_limbs().rev().collect_vec()",
                &mut |n| no_out!(n.twos_complement_limbs().collect_vec()),
            ),
        ],
    );
}

fn benchmark_integer_twos_complement_limbs_get_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.twos_complement_limbs().get()",
        BenchmarkType::Algorithms,
        integer_unsigned_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("n"),
        &mut [
            ("Integer.twos_complement_limbs().get(u)", &mut |(n, u)| {
                no_out!(n.twos_complement_limbs().get(u))
            }),
            (
                "Integer.into_twos_complement_limbs_asc()[u]",
                &mut |(n, u)| {
                    let u = usize::exact_from(u);
                    let non_negative = n >= 0;
                    let limbs = n.into_twos_complement_limbs_asc();
                    if u >= limbs.len() {
                        if non_negative {
                            0
                        } else {
                            Limb::MAX
                        }
                    } else {
                        limbs[u]
                    };
                },
            ),
        ],
    );
}

fn benchmark_integer_twos_complement_limb_count(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.twos_complement_limb_count()",
        BenchmarkType::Single,
        integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("n"),
        &mut [(
            "Malachite",
            &mut |n| no_out!(n.twos_complement_limb_count()),
        )],
    );
}
