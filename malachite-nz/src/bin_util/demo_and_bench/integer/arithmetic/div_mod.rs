// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CeilingDivAssignMod, CeilingDivMod, CeilingMod, DivAssignMod, DivAssignRem, DivMod, DivRem,
    DivRound, Mod,
};
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::{
    pair_1_integer_bit_bucketer, pair_2_pair_1_integer_bit_bucketer,
    triple_3_pair_1_integer_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    integer_pair_gen_var_1, integer_pair_gen_var_1_nrm, integer_pair_gen_var_1_rm,
};
use num::Integer as NumInteger;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_div_mod);
    register_demo!(runner, demo_integer_div_mod_val_ref);
    register_demo!(runner, demo_integer_div_mod_ref_val);
    register_demo!(runner, demo_integer_div_mod_ref_ref);
    register_demo!(runner, demo_integer_div_assign_mod);
    register_demo!(runner, demo_integer_div_assign_mod_ref);
    register_demo!(runner, demo_integer_div_rem);
    register_demo!(runner, demo_integer_div_rem_val_ref);
    register_demo!(runner, demo_integer_div_rem_ref_val);
    register_demo!(runner, demo_integer_div_rem_ref_ref);
    register_demo!(runner, demo_integer_div_assign_rem);
    register_demo!(runner, demo_integer_div_assign_rem_ref);
    register_demo!(runner, demo_integer_ceiling_div_mod);
    register_demo!(runner, demo_integer_ceiling_div_mod_val_ref);
    register_demo!(runner, demo_integer_ceiling_div_mod_ref_val);
    register_demo!(runner, demo_integer_ceiling_div_mod_ref_ref);
    register_demo!(runner, demo_integer_ceiling_div_assign_mod);
    register_demo!(runner, demo_integer_ceiling_div_assign_mod_ref);

    register_bench!(runner, benchmark_integer_div_mod_library_comparison);
    register_bench!(runner, benchmark_integer_div_mod_algorithms);
    register_bench!(runner, benchmark_integer_div_mod_evaluation_strategy);
    register_bench!(runner, benchmark_integer_div_assign_mod_evaluation_strategy);
    register_bench!(runner, benchmark_integer_div_rem_library_comparison);
    register_bench!(runner, benchmark_integer_div_rem_algorithms);
    register_bench!(runner, benchmark_integer_div_rem_evaluation_strategy);
    register_bench!(runner, benchmark_integer_div_assign_rem_evaluation_strategy);
    register_bench!(runner, benchmark_integer_ceiling_div_mod_library_comparison);
    register_bench!(runner, benchmark_integer_ceiling_div_mod_algorithms);
    register_bench!(
        runner,
        benchmark_integer_ceiling_div_mod_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_integer_ceiling_div_assign_mod_evaluation_strategy
    );
}

fn demo_integer_div_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{}.div_mod({}) = {:?}", x_old, y_old, x.div_mod(y));
    }
}

fn demo_integer_div_mod_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("{}.div_mod(&{}) = {:?}", x_old, y, x.div_mod(&y));
    }
}

fn demo_integer_div_mod_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!("(&{}).div_mod({}) = {:?}", x, y_old, (&x).div_mod(y));
    }
}

fn demo_integer_div_mod_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        println!("(&{}).div_mod(&{}) = {:?}", x, y, (&x).div_mod(&y));
    }
}

fn demo_integer_div_assign_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let remainder = x.div_assign_mod(y);
        println!("x := {x_old}; x.div_assign_mod({y_old}) = {remainder}; x = {x}");
    }
}

fn demo_integer_div_assign_mod_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let remainder = x.div_assign_mod(&y);
        println!("x := {x_old}; x.div_assign_mod(&{y}) = {remainder}; x = {x}");
    }
}

fn demo_integer_div_rem(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{}.div_rem({}) = {:?}", x_old, y_old, x.div_rem(y));
    }
}

fn demo_integer_div_rem_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("{}.div_rem(&{}) = {:?}", x_old, y, x.div_rem(&y));
    }
}

fn demo_integer_div_rem_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!("(&{}).div_rem({}) = {:?}", x, y_old, (&x).div_rem(y));
    }
}

fn demo_integer_div_rem_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        println!("(&{}).div_rem(&{}) = {:?}", x, y, (&x).div_rem(&y));
    }
}

fn demo_integer_div_assign_rem(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let remainder = x.div_assign_rem(y);
        println!("x := {x_old}; x.div_assign_rem({y_old}) = {remainder}; x = {x}");
    }
}

fn demo_integer_div_assign_rem_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let remainder = x.div_assign_rem(&y);
        println!("x := {x_old}; x.div_assign_rem(&{y}) = {remainder}; x = {x}");
    }
}

fn demo_integer_ceiling_div_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "{}.ceiling_div_mod({}) = {:?}",
            x_old,
            y_old,
            x.ceiling_div_mod(y)
        );
    }
}

fn demo_integer_ceiling_div_mod_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "{}.ceiling_div_mod(&{}) = {:?}",
            x_old,
            y,
            x.ceiling_div_mod(&y)
        );
    }
}

fn demo_integer_ceiling_div_mod_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!(
            "(&{}).ceiling_div_mod({}) = {:?}",
            x,
            y_old,
            (&x).ceiling_div_mod(y)
        );
    }
}

fn demo_integer_ceiling_div_mod_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        println!(
            "(&{}).ceiling_div_mod(&{}) = {:?}",
            x,
            y,
            (&x).ceiling_div_mod(&y)
        );
    }
}

fn demo_integer_ceiling_div_assign_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let remainder = x.ceiling_div_assign_mod(y);
        println!("x := {x_old}; x.ceiling_div_assign_mod({y_old}) = {remainder}; x = {x}");
    }
}

fn demo_integer_ceiling_div_assign_mod_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let remainder = x.ceiling_div_assign_mod(&y);
        println!("x := {x_old}; x.ceiling_div_assign_mod(&{y}) = {remainder}; x = {x}");
    }
}

fn benchmark_integer_div_mod_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.div_mod(Integer)",
        BenchmarkType::LibraryComparison,
        integer_pair_gen_var_1_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, _, (x, y))| no_out!(x.div_mod(y))),
            ("num", &mut |((x, y), _, _)| no_out!(x.div_mod_floor(&y))),
            ("rug", &mut |(_, (x, y), _)| no_out!(x.div_rem_floor(y))),
        ],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_integer_div_mod_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.div_mod(Integer)",
        BenchmarkType::Algorithms,
        integer_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            ("standard", &mut |(x, y)| no_out!(x.div_mod(y))),
            ("using div_round and mod_op", &mut |(x, y)| {
                no_out!(((&x).div_round(&y, Floor), x.mod_op(y)))
            }),
        ],
    );
}

fn benchmark_integer_div_mod_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.div_mod(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Integer.div_mod(Integer)", &mut |(x, y)| {
                no_out!(x.div_mod(y))
            }),
            ("Integer.div_mod(&Integer)", &mut |(x, y)| {
                no_out!(x.div_mod(&y))
            }),
            ("(&Integer).div_mod(Integer)", &mut |(x, y)| {
                no_out!((&x).div_mod(y))
            }),
            ("(&Integer).div_mod(&Integer)", &mut |(x, y)| {
                no_out!((&x).div_mod(&y))
            }),
        ],
    );
}

fn benchmark_integer_div_assign_mod_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.div_assign_mod(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Integer.div_assign_mod(Integer)", &mut |(mut x, y)| {
                no_out!(x.div_assign_mod(y))
            }),
            ("Integer.div_assign_mod(&Integer)", &mut |(mut x, y)| {
                no_out!(x.div_assign_mod(&y))
            }),
        ],
    );
}

fn benchmark_integer_div_rem_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.div_rem(Integer)",
        BenchmarkType::LibraryComparison,
        integer_pair_gen_var_1_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, _, (x, y))| no_out!(x.div_rem(y))),
            ("num", &mut |((x, y), _, _)| no_out!(x.div_rem(&y))),
            ("rug", &mut |(_, (x, y), _)| no_out!(x.div_rem(y))),
        ],
    );
}
#[allow(clippy::no_effect)]
fn benchmark_integer_div_rem_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.div_rem(Integer)",
        BenchmarkType::Algorithms,
        integer_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            ("standard", &mut |(x, y)| no_out!(x.div_rem(y))),
            ("using / and %", &mut |(x, y)| no_out!((&x / &y, x % y))),
        ],
    );
}

fn benchmark_integer_div_rem_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.div_rem(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Integer.div_rem(Integer)", &mut |(x, y)| {
                no_out!(x.div_rem(y))
            }),
            ("Integer.div_rem(&Integer)", &mut |(x, y)| {
                no_out!(x.div_rem(&y))
            }),
            ("(&Integer).div_rem(Integer)", &mut |(x, y)| {
                no_out!((&x).div_rem(y))
            }),
            ("(&Integer).div_rem(&Integer)", &mut |(x, y)| {
                no_out!((&x).div_rem(&y))
            }),
        ],
    );
}

fn benchmark_integer_div_assign_rem_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.div_assign_rem(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Integer.div_assign_rem(Integer)", &mut |(mut x, y)| {
                no_out!(x.div_assign_rem(y))
            }),
            ("Integer.div_assign_rem(&Integer)", &mut |(mut x, y)| {
                no_out!(x.div_assign_rem(&y))
            }),
        ],
    );
}

fn benchmark_integer_ceiling_div_mod_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.ceiling_div_mod(Integer)",
        BenchmarkType::LibraryComparison,
        integer_pair_gen_var_1_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_integer_bit_bucketer("x"),
        &mut [
            (
                "Malachite",
                &mut |(_, (x, y))| no_out!(x.ceiling_div_mod(y)),
            ),
            ("rug", &mut |((x, y), _)| no_out!(x.div_rem_ceil(y))),
        ],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_integer_ceiling_div_mod_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.ceiling_div_mod(Integer)",
        BenchmarkType::Algorithms,
        integer_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            ("standard", &mut |(x, y)| no_out!(x.ceiling_div_mod(y))),
            ("using div_round and ceiling_mod", &mut |(x, y)| {
                ((&x).div_round(&y, Ceiling), x.ceiling_mod(y));
            }),
        ],
    );
}

fn benchmark_integer_ceiling_div_mod_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.ceiling_div_mod(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Integer.ceiling_div_mod(Integer)", &mut |(x, y)| {
                no_out!(x.ceiling_div_mod(y))
            }),
            ("Integer.ceiling_div_mod(&Integer)", &mut |(x, y)| {
                no_out!(x.ceiling_div_mod(&y))
            }),
            ("(&Integer).ceiling_div_mod(Integer)", &mut |(x, y)| {
                no_out!((&x).ceiling_div_mod(y))
            }),
            ("(&Integer).ceiling_div_mod(&Integer)", &mut |(x, y)| {
                no_out!((&x).ceiling_div_mod(&y))
            }),
        ],
    );
}

fn benchmark_integer_ceiling_div_assign_mod_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.ceiling_div_assign_mod(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            (
                "Integer.ceiling_div_assign_mod(Integer)",
                &mut |(mut x, y)| no_out!(x.ceiling_div_assign_mod(y)),
            ),
            (
                "Integer.ceiling_div_assign_mod(&Integer)",
                &mut |(mut x, y)| no_out!(x.ceiling_div_assign_mod(&y)),
            ),
        ],
    );
}
