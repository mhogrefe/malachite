// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::DivisibleBy;
use malachite_base::slices::slice_test_zero;
use malachite_base::test_util::bench::bucketers::pair_1_vec_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_vec_pair_gen_var_15, unsigned_vec_pair_gen_var_16,
    unsigned_vec_unsigned_pair_gen_var_22,
};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::divisible_by::{
    limbs_divisible_by, limbs_divisible_by_limb, limbs_divisible_by_ref_ref,
    limbs_divisible_by_ref_val, limbs_divisible_by_val_ref,
};
use malachite_nz::natural::arithmetic::mod_op::{limbs_mod, limbs_mod_limb};
use malachite_nz::test_util::bench::bucketers::{
    pair_1_natural_bit_bucketer, triple_3_pair_1_natural_bit_bucketer,
};
use malachite_nz::test_util::generators::{natural_pair_gen, natural_pair_gen_nrm};
use malachite_nz::test_util::natural::arithmetic::divisible_by::{
    combined_limbs_divisible_by_limb, num_divisible_by,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_divisible_by_limb);
    register_demo!(runner, demo_limbs_divisible_by);
    register_demo!(runner, demo_limbs_divisible_by_val_ref);
    register_demo!(runner, demo_limbs_divisible_by_ref_val);
    register_demo!(runner, demo_limbs_divisible_by_ref_ref);
    register_demo!(runner, demo_natural_divisible_by);
    register_demo!(runner, demo_natural_divisible_by_val_ref);
    register_demo!(runner, demo_natural_divisible_by_ref_val);
    register_demo!(runner, demo_natural_divisible_by_ref_ref);

    register_bench!(runner, benchmark_limbs_divisible_by_limb_algorithms);
    register_bench!(runner, benchmark_limbs_divisible_by_algorithms);
    register_bench!(runner, benchmark_limbs_divisible_by_evaluation_strategy);
    register_bench!(runner, benchmark_natural_divisible_by_algorithms);
    register_bench!(runner, benchmark_natural_divisible_by_evaluation_strategy);
    register_bench!(runner, benchmark_natural_divisible_by_library_comparison);
}

fn demo_limbs_divisible_by_limb(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, y) in unsigned_vec_unsigned_pair_gen_var_22()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_divisible_by_limb({:?}, {}) = {}",
            xs,
            y,
            limbs_divisible_by_limb(&xs, y)
        );
    }
}

fn demo_limbs_divisible_by(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut ns, mut ds) in unsigned_vec_pair_gen_var_15().get(gm, config).take(limit) {
        let ns_old = ns.clone();
        let ds_old = ds.clone();
        println!(
            "limbs_divisible_by({:?}, {:?}) = {}",
            ns_old,
            ds_old,
            limbs_divisible_by(&mut ns, &mut ds)
        );
    }
}

fn demo_limbs_divisible_by_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut ns, ds) in unsigned_vec_pair_gen_var_15().get(gm, config).take(limit) {
        let ns_old = ns.clone();
        println!(
            "limbs_divisible_by_val_ref({:?}, {:?}) = {}",
            ns_old,
            ds,
            limbs_divisible_by_val_ref(&mut ns, &ds)
        );
    }
}

fn demo_limbs_divisible_by_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (ns, mut ds) in unsigned_vec_pair_gen_var_15().get(gm, config).take(limit) {
        let ds_old = ds.clone();
        println!(
            "limbs_divisible_by_ref_val({:?}, {:?}) = {}",
            ns,
            ds_old,
            limbs_divisible_by_ref_val(&ns, &mut ds)
        );
    }
}

fn demo_limbs_divisible_by_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (ns, ds) in unsigned_vec_pair_gen_var_15().get(gm, config).take(limit) {
        println!(
            "limbs_divisible_by_ref_ref({:?}, {:?}) = {}",
            ns,
            ds,
            limbs_divisible_by_ref_ref(&ns, &ds)
        );
    }
}

fn demo_natural_divisible_by(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        if x.divisible_by(y) {
            println!("{x_old} is divisible by {y_old}");
        } else {
            println!("{x_old} is not divisible by {y_old}");
        }
    }
}

fn demo_natural_divisible_by_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        if x.divisible_by(&y) {
            println!("{x_old} is divisible by {y}");
        } else {
            println!("{x_old} is not divisible by {y}");
        }
    }
}

fn demo_natural_divisible_by_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let y_old = y.clone();
        if (&x).divisible_by(y) {
            println!("{x} is divisible by {y_old}");
        } else {
            println!("{x} is not divisible by {y_old}");
        }
    }
}

fn demo_natural_divisible_by_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let y_old = y.clone();
        if (&x).divisible_by(y) {
            println!("{x} is divisible by {y_old}");
        } else {
            println!("{x} is not divisible by {y_old}");
        }
    }
}

// use large params
#[allow(clippy::unnecessary_operation, unused_must_use)]
fn benchmark_limbs_divisible_by_limb_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_divisible_by_limb(&[Limb], Limb)",
        BenchmarkType::Algorithms,
        unsigned_vec_unsigned_pair_gen_var_22().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [
            ("limbs_divisible_by_limb", &mut |(xs, y)| {
                no_out!(limbs_divisible_by_limb(&xs, y))
            }),
            ("divisibility using limbs_mod_limb", &mut |(xs, y)| {
                no_out!(limbs_mod_limb(&xs, y) == 0)
            }),
            ("combined_limbs_divisible_by_limb", &mut |(xs, y)| {
                no_out!(combined_limbs_divisible_by_limb(&xs, y))
            }),
        ],
    );
}

// use large params
fn benchmark_limbs_divisible_by_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_divisible_by(&[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_pair_gen_var_16().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [
            ("limbs_divisible_by", &mut |(mut ns, mut ds)| {
                no_out!(limbs_divisible_by(&mut ns, &mut ds))
            }),
            ("divisibility using limbs_mod", &mut |(ns, ds)| {
                no_out!(slice_test_zero(&limbs_mod(&ns, &ds)))
            }),
        ],
    );
}

// use large params
fn benchmark_limbs_divisible_by_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_divisible_by(&[Limb], &[Limb])",
        BenchmarkType::EvaluationStrategy,
        unsigned_vec_pair_gen_var_15().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [
            (
                "limbs_divisible_by(&mut [Limb], &mut [Limb])",
                &mut |(mut ns, mut ds)| no_out!(limbs_divisible_by(&mut ns, &mut ds)),
            ),
            (
                "limbs_divisible_by_val_ref(&mut [Limb], &mut [Limb])",
                &mut |(mut ns, ds)| no_out!(limbs_divisible_by_val_ref(&mut ns, &ds)),
            ),
            (
                "limbs_divisible_by_ref_val(&mut [Limb], &mut [Limb])",
                &mut |(ns, mut ds)| no_out!(limbs_divisible_by_ref_val(&ns, &mut ds)),
            ),
            (
                "limbs_divisible_by_ref_ref(&mut [Limb], &mut [Limb])",
                &mut |(ns, ds)| no_out!(limbs_divisible_by_ref_ref(&ns, &ds)),
            ),
        ],
    );
}

#[allow(clippy::no_effect, clippy::short_circuit_statement, unused_must_use)]
fn benchmark_natural_divisible_by_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.divisible_by(Natural)",
        BenchmarkType::Algorithms,
        natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [
            ("standard", &mut |(x, y)| no_out!(x.divisible_by(y))),
            ("using %", &mut |(x, y)| {
                no_out!(x == 0 || y != 0 && x % y == 0)
            }),
        ],
    );
}

fn benchmark_natural_divisible_by_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.divisible_by(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [
            ("Natural.divisible_by(Natural)", &mut |(x, y)| {
                no_out!(x.divisible_by(y))
            }),
            ("Natural.divisible_by(&Natural)", &mut |(x, y)| {
                no_out!(x.divisible_by(&y))
            }),
            ("(&Natural).divisible_by(Natural)", &mut |(x, y)| {
                no_out!((&x).divisible_by(y))
            }),
            ("(&Natural).divisible_by(&Natural)", &mut |(x, y)| {
                no_out!((&x).divisible_by(&y))
            }),
        ],
    );
}

fn benchmark_natural_divisible_by_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.divisible_by(Natural)",
        BenchmarkType::LibraryComparison,
        natural_pair_gen_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_1_natural_bit_bucketer("n"),
        &mut [
            (
                "Malachite",
                &mut |(_, _, (x, y))| no_out!(x.divisible_by(y)),
            ),
            ("num", &mut |((x, y), _, _)| {
                no_out!(num_divisible_by(&x, &y))
            }),
            ("rug", &mut |(_, (x, y), _)| no_out!(x.is_divisible(&y))),
        ],
    );
}
