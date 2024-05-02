// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CeilingRoot, CeilingRootAssign, CheckedRoot, FloorRoot, FloorRootAssign, Pow, RootAssignRem,
    RootRem,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::bucketers::pair_1_vec_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_unsigned_pair_gen_var_14;
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::root::{limbs_floor_root, limbs_root_rem};
use malachite_nz::test_util::bench::bucketers::{
    natural_bit_bucketer, pair_1_natural_bit_bucketer, pair_2_natural_bit_bucketer,
    pair_2_pair_1_natural_bit_bucketer, triple_3_natural_bit_bucketer,
    triple_3_pair_1_natural_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    natural_gen, natural_gen_nrm, natural_gen_rm, natural_unsigned_pair_gen_var_7,
    natural_unsigned_pair_gen_var_7_nrm, natural_unsigned_pair_gen_var_7_rm,
};
use malachite_nz::test_util::natural::arithmetic::root::{
    ceiling_root_binary, checked_root_binary, floor_root_binary, root_rem_binary,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_floor_root);
    register_demo!(runner, demo_limbs_root_rem);

    register_demo!(runner, demo_natural_floor_cbrt);
    register_demo!(runner, demo_natural_floor_cbrt_ref);
    register_demo!(runner, demo_natural_floor_cbrt_assign);
    register_demo!(runner, demo_natural_ceiling_cbrt);
    register_demo!(runner, demo_natural_ceiling_cbrt_ref);
    register_demo!(runner, demo_natural_ceiling_cbrt_assign);
    register_demo!(runner, demo_natural_checked_cbrt);
    register_demo!(runner, demo_natural_checked_cbrt_ref);
    register_demo!(runner, demo_natural_cbrt_rem);
    register_demo!(runner, demo_natural_cbrt_rem_ref);
    register_demo!(runner, demo_natural_cbrt_assign_rem);

    register_demo!(runner, demo_natural_floor_root);
    register_demo!(runner, demo_natural_floor_root_ref);
    register_demo!(runner, demo_natural_floor_root_assign);
    register_demo!(runner, demo_natural_ceiling_root);
    register_demo!(runner, demo_natural_ceiling_root_ref);
    register_demo!(runner, demo_natural_ceiling_root_assign);
    register_demo!(runner, demo_natural_checked_root);
    register_demo!(runner, demo_natural_checked_root_ref);
    register_demo!(runner, demo_natural_root_rem);
    register_demo!(runner, demo_natural_root_rem_ref);
    register_demo!(runner, demo_natural_root_assign_rem);

    register_bench!(runner, benchmark_limbs_floor_root);
    register_bench!(runner, benchmark_limbs_root_rem);

    register_bench!(runner, benchmark_natural_floor_cbrt_evaluation_strategy);
    register_bench!(runner, benchmark_natural_floor_cbrt_algorithms);
    register_bench!(runner, benchmark_natural_floor_cbrt_library_comparison);
    register_bench!(runner, benchmark_natural_floor_cbrt_assign);
    register_bench!(runner, benchmark_natural_ceiling_cbrt_evaluation_strategy);
    register_bench!(runner, benchmark_natural_ceiling_cbrt_algorithms);
    register_bench!(runner, benchmark_natural_ceiling_cbrt_assign);
    register_bench!(runner, benchmark_natural_checked_cbrt_evaluation_strategy);
    register_bench!(runner, benchmark_natural_checked_cbrt_algorithms);
    register_bench!(runner, benchmark_natural_cbrt_rem_evaluation_strategy);
    register_bench!(runner, benchmark_natural_cbrt_rem_algorithms);
    register_bench!(runner, benchmark_natural_cbrt_rem_library_comparison);
    register_bench!(runner, benchmark_natural_cbrt_assign_rem);

    register_bench!(runner, benchmark_natural_floor_root_evaluation_strategy);
    register_bench!(runner, benchmark_natural_floor_root_algorithms);
    register_bench!(runner, benchmark_natural_floor_root_library_comparison);
    register_bench!(runner, benchmark_natural_floor_root_assign);
    register_bench!(runner, benchmark_natural_ceiling_root_evaluation_strategy);
    register_bench!(runner, benchmark_natural_ceiling_root_algorithms);
    register_bench!(runner, benchmark_natural_ceiling_root_assign);
    register_bench!(runner, benchmark_natural_checked_root_evaluation_strategy);
    register_bench!(runner, benchmark_natural_checked_root_algorithms);
    register_bench!(runner, benchmark_natural_root_rem_evaluation_strategy);
    register_bench!(runner, benchmark_natural_root_rem_algorithms);
    register_bench!(runner, benchmark_natural_root_rem_library_comparison);
    register_bench!(runner, benchmark_natural_root_assign_rem);
}

fn demo_limbs_floor_root(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, exp) in unsigned_vec_unsigned_pair_gen_var_14()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_root({:?}, {}) = {:?}",
            xs,
            exp,
            limbs_floor_root(&xs, exp)
        );
    }
}

fn demo_limbs_root_rem(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, exp) in unsigned_vec_unsigned_pair_gen_var_14()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_root_rem({:?}, {}) = {:?}",
            xs,
            exp,
            limbs_root_rem(&xs, exp)
        );
    }
}

fn demo_natural_floor_cbrt(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in natural_gen().get(gm, config).take(limit) {
        println!("{}.floor_root(3) = {}", x, x.clone().floor_root(3));
    }
}

fn demo_natural_floor_cbrt_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in natural_gen().get(gm, config).take(limit) {
        println!("(&{}).floor_root(3) = {}", x, (&x).floor_root(3));
    }
}

fn demo_natural_floor_cbrt_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in natural_gen().get(gm, config).take(limit) {
        let old_x = x.clone();
        x.floor_root_assign(3);
        println!("x := {old_x}; x.floor_root_assign(3); x = {x}");
    }
}

fn demo_natural_ceiling_cbrt(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in natural_gen().get(gm, config).take(limit) {
        println!("{}.ceiling_root(3) = {}", x, x.clone().ceiling_root(3));
    }
}

fn demo_natural_ceiling_cbrt_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in natural_gen().get(gm, config).take(limit) {
        println!("(&{}).ceiling_root(3) = {}", x, (&x).ceiling_root(3));
    }
}

fn demo_natural_ceiling_cbrt_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in natural_gen().get(gm, config).take(limit) {
        let old_x = x.clone();
        x.ceiling_root_assign(3);
        println!("x := {old_x}; x.ceiling_root_assign(3); x = {x}");
    }
}

fn demo_natural_checked_cbrt(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in natural_gen().get(gm, config).take(limit) {
        println!("{}.checked_root(3) = {:?}", x, x.clone().checked_root(3));
    }
}

fn demo_natural_checked_cbrt_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in natural_gen().get(gm, config).take(limit) {
        println!("(&{}).checked_root(3) = {:?}", x, (&x).checked_root(3));
    }
}

fn demo_natural_cbrt_rem(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in natural_gen().get(gm, config).take(limit) {
        println!("{}.root_rem(3) = {:?}", x, x.clone().root_rem(3));
    }
}

fn demo_natural_cbrt_rem_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in natural_gen().get(gm, config).take(limit) {
        println!("(&{}).root_rem(3) = {:?}", x, (&x).root_rem(3));
    }
}

fn demo_natural_cbrt_assign_rem(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in natural_gen().get(gm, config).take(limit) {
        let old_x = x.clone();
        let rem = x.root_assign_rem(3);
        println!("x := {old_x}; x.root_assign_rem(3) = {rem}; x = {x}");
    }
}

fn demo_natural_floor_root(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, exp) in natural_unsigned_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        println!("{}.floor_root({}) = {}", x, exp, x.clone().floor_root(exp));
    }
}

fn demo_natural_floor_root_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, exp) in natural_unsigned_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{}).floor_root({}) = {}", x, exp, (&x).floor_root(exp));
    }
}

fn demo_natural_floor_root_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, exp) in natural_unsigned_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let old_x = x.clone();
        x.floor_root_assign(exp);
        println!("x := {old_x}; x.floor_root_assign({exp}); x = {x}");
    }
}

fn demo_natural_ceiling_root(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, exp) in natural_unsigned_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.ceiling_root({}) = {}",
            x,
            exp,
            x.clone().ceiling_root(exp)
        );
    }
}

fn demo_natural_ceiling_root_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, exp) in natural_unsigned_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).ceiling_root({}) = {}",
            x,
            exp,
            (&x).ceiling_root(exp)
        );
    }
}

fn demo_natural_ceiling_root_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, exp) in natural_unsigned_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let old_x = x.clone();
        x.ceiling_root_assign(exp);
        println!("x := {old_x}; x.ceiling_root_assign({exp}); x = {x}");
    }
}

fn demo_natural_checked_root(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, exp) in natural_unsigned_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.checked_root({}) = {:?}",
            x,
            exp,
            x.clone().checked_root(exp)
        );
    }
}

fn demo_natural_checked_root_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, exp) in natural_unsigned_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).checked_root({}) = {:?}",
            x,
            exp,
            (&x).checked_root(exp)
        );
    }
}

fn demo_natural_root_rem(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, exp) in natural_unsigned_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        println!("{}.root_rem({}) = {:?}", x, exp, x.clone().root_rem(exp));
    }
}

fn demo_natural_root_rem_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, exp) in natural_unsigned_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{}).root_rem({}) = {:?}", x, exp, (&x).root_rem(exp));
    }
}

fn demo_natural_root_assign_rem(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, exp) in natural_unsigned_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let old_x = x.clone();
        let rem = x.root_assign_rem(exp);
        println!("x := {old_x}; x.root_assign_rem({exp}) = {rem}; x = {x}");
    }
}

fn benchmark_limbs_floor_root(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_floor_root(&[Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_14().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, exp)| {
            no_out!(limbs_floor_root(&xs, exp))
        })],
    );
}

fn benchmark_limbs_root_rem(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_root_rem(&[Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_14().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, exp)| {
            no_out!(limbs_root_rem(&xs, exp))
        })],
    );
}

fn benchmark_natural_floor_cbrt_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.floor_root(3)",
        BenchmarkType::EvaluationStrategy,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [
            ("Natural.floor_root(3)", &mut |x| no_out!(x.floor_root(3))),
            ("(&Natural).floor_root(3)", &mut |x| {
                no_out!((&x).floor_root(3))
            }),
        ],
    );
}

fn benchmark_natural_floor_cbrt_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.floor_root(3)",
        BenchmarkType::Algorithms,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [
            ("default", &mut |x| no_out!(x.floor_root(3))),
            ("binary", &mut |x| no_out!(floor_root_binary(&x, 3))),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_natural_floor_cbrt_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.floor_root(3)",
        BenchmarkType::LibraryComparison,
        natural_gen_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_natural_bit_bucketer("x"),
        &mut [
            ("num", &mut |(x, _, _)| no_out!(x.nth_root(3))),
            ("rug", &mut |(_, x, _)| no_out!(x.root(3))),
            ("Malachite", &mut |(_, _, x)| no_out!(x.floor_root(3))),
        ],
    );
}

fn benchmark_natural_floor_cbrt_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.floor_root_assign(3)",
        BenchmarkType::Single,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [("Malachite", &mut |mut x| x.floor_root_assign(3))],
    );
}

fn benchmark_natural_ceiling_cbrt_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.ceiling_root(3)",
        BenchmarkType::EvaluationStrategy,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [
            ("Natural.ceiling_root(3)", &mut |x| {
                no_out!(x.ceiling_root(3))
            }),
            ("(&Natural).ceiling_root(3)", &mut |x| {
                no_out!((&x).ceiling_root(3))
            }),
        ],
    );
}

fn benchmark_natural_ceiling_cbrt_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.ceiling_root(3)",
        BenchmarkType::Algorithms,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [
            ("default", &mut |x| no_out!(x.ceiling_root(3))),
            ("binary", &mut |x| no_out!(ceiling_root_binary(&x, 3))),
        ],
    );
}

fn benchmark_natural_ceiling_cbrt_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.ceiling_root_assign(3)",
        BenchmarkType::Single,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [("Malachite", &mut |mut x| x.ceiling_root_assign(3))],
    );
}

fn benchmark_natural_checked_cbrt_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.checked_root(3)",
        BenchmarkType::EvaluationStrategy,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [
            ("Natural.checked_root(3)", &mut |x| {
                no_out!(x.checked_root(3))
            }),
            ("(&Natural).checked_root(3)", &mut |x| {
                no_out!((&x).checked_root(3))
            }),
        ],
    );
}

fn benchmark_natural_checked_cbrt_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.checked_root(3)",
        BenchmarkType::Algorithms,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [
            ("default", &mut |x| no_out!(x.checked_root(3))),
            ("binary", &mut |x| no_out!(checked_root_binary(&x, 3))),
        ],
    );
}

fn benchmark_natural_cbrt_rem_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.root_rem(3)",
        BenchmarkType::EvaluationStrategy,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [
            ("Natural.root_rem(3)", &mut |x| no_out!(x.root_rem(3))),
            ("(&Natural).root_rem(3)", &mut |x| no_out!((&x).root_rem(3))),
        ],
    );
}

#[allow(clippy::no_effect)]
fn benchmark_natural_cbrt_rem_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.root_rem(3)",
        BenchmarkType::Algorithms,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [
            ("default", &mut |x| no_out!(x.root_rem(3))),
            ("floor and subtraction", &mut |x| {
                let root = (&x).floor_root(3);
                let pow = (&root).pow(3);
                (root, x - pow);
            }),
            ("binary", &mut |x| no_out!(root_rem_binary(&x, 3))),
        ],
    );
}

fn benchmark_natural_cbrt_rem_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.root_rem(3)",
        BenchmarkType::LibraryComparison,
        natural_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("x"),
        &mut [
            ("rug", &mut |(x, _)| {
                no_out!(x.root_rem(rug::Integer::new(), 3))
            }),
            ("Malachite", &mut |(_, x)| no_out!(x.root_rem(3))),
        ],
    );
}

fn benchmark_natural_cbrt_assign_rem(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.root_assign_rem(3)",
        BenchmarkType::Single,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [("Malachite", &mut |mut x| no_out!(x.root_assign_rem(3)))],
    );
}

fn benchmark_natural_floor_root_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.floor_root(u64)",
        BenchmarkType::EvaluationStrategy,
        natural_unsigned_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [
            ("Natural.floor_root()", &mut |(x, exp)| {
                no_out!(x.floor_root(exp))
            }),
            ("(&Natural).floor_root()", &mut |(x, exp)| {
                no_out!((&x).floor_root(exp))
            }),
        ],
    );
}

fn benchmark_natural_floor_root_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.floor_root(u64)",
        BenchmarkType::Algorithms,
        natural_unsigned_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [
            ("default", &mut |(x, exp)| no_out!(x.floor_root(exp))),
            ("binary", &mut |(x, exp)| {
                no_out!(floor_root_binary(&x, exp))
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_natural_floor_root_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.floor_root(u64)",
        BenchmarkType::LibraryComparison,
        natural_unsigned_pair_gen_var_7_nrm::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_1_natural_bit_bucketer("x"),
        &mut [
            ("num", &mut |((x, exp), _, _)| {
                no_out!(x.nth_root(u32::exact_from(exp)))
            }),
            ("rug", &mut |(_, (x, exp), _)| {
                no_out!(x.root(u32::exact_from(exp)))
            }),
            ("Malachite", &mut |(_, _, (x, exp))| {
                no_out!(x.floor_root(exp))
            }),
        ],
    );
}

fn benchmark_natural_floor_root_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.floor_root_assign(u64)",
        BenchmarkType::Single,
        natural_unsigned_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [("Malachite", &mut |(mut x, exp)| x.floor_root_assign(exp))],
    );
}

fn benchmark_natural_ceiling_root_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.ceiling_root(u64)",
        BenchmarkType::EvaluationStrategy,
        natural_unsigned_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [
            ("Natural.ceiling_root()", &mut |(x, exp)| {
                no_out!(x.ceiling_root(exp))
            }),
            ("(&Natural).ceiling_root()", &mut |(x, exp)| {
                no_out!((&x).ceiling_root(exp))
            }),
        ],
    );
}

fn benchmark_natural_ceiling_root_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.ceiling_root(u64)",
        BenchmarkType::Algorithms,
        natural_unsigned_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [
            ("default", &mut |(x, exp)| no_out!(x.ceiling_root(exp))),
            ("binary", &mut |(x, exp)| {
                no_out!(ceiling_root_binary(&x, exp))
            }),
        ],
    );
}

fn benchmark_natural_ceiling_root_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.ceiling_root_assign(u64)",
        BenchmarkType::Single,
        natural_unsigned_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [("Malachite", &mut |(mut x, exp)| x.ceiling_root_assign(exp))],
    );
}

fn benchmark_natural_checked_root_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.checked_root(u64)",
        BenchmarkType::EvaluationStrategy,
        natural_unsigned_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [
            ("Natural.checked_root()", &mut |(x, exp)| {
                no_out!(x.checked_root(exp))
            }),
            ("(&Natural).checked_root()", &mut |(x, exp)| {
                no_out!((&x).checked_root(exp))
            }),
        ],
    );
}

fn benchmark_natural_checked_root_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.checked_root(u64)",
        BenchmarkType::Algorithms,
        natural_unsigned_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [
            ("default", &mut |(x, exp)| no_out!(x.checked_root(exp))),
            ("binary", &mut |(x, exp)| {
                no_out!(checked_root_binary(&x, exp))
            }),
        ],
    );
}

fn benchmark_natural_root_rem_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.root_rem(u64)",
        BenchmarkType::EvaluationStrategy,
        natural_unsigned_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [
            ("Natural.root_rem()", &mut |(x, exp)| {
                no_out!(x.root_rem(exp))
            }),
            ("(&Natural).root_rem()", &mut |(x, exp)| {
                no_out!((&x).root_rem(exp))
            }),
        ],
    );
}

#[allow(clippy::no_effect)]
fn benchmark_natural_root_rem_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.root_rem(u64)",
        BenchmarkType::Algorithms,
        natural_unsigned_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [
            ("default", &mut |(x, exp)| no_out!(x.root_rem(exp))),
            ("floor and subtraction", &mut |(x, exp)| {
                let root = (&x).floor_root(exp);
                let pow = (&root).pow(exp);
                (root, x - pow);
            }),
            ("binary", &mut |(x, exp)| no_out!(root_rem_binary(&x, exp))),
        ],
    );
}

fn benchmark_natural_root_rem_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.root_rem(u64)",
        BenchmarkType::LibraryComparison,
        natural_unsigned_pair_gen_var_7_rm::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_natural_bit_bucketer("x"),
        &mut [
            ("rug", &mut |((x, exp), _)| {
                no_out!(x.root_rem(rug::Integer::new(), u32::exact_from(exp)))
            }),
            ("Malachite", &mut |(_, (x, exp))| no_out!(x.root_rem(exp))),
        ],
    );
}

fn benchmark_natural_root_assign_rem(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.root_assign_rem(u64)",
        BenchmarkType::Single,
        natural_unsigned_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [("Malachite", &mut |(mut x, exp)| {
            no_out!(x.root_assign_rem(exp))
        })],
    );
}
