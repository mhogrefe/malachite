// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CeilingDivMod, CeilingMod, CeilingModAssign, DivMod, DivRem, Mod, ModAssign,
};
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
use rug::ops::RemRounding;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_mod);
    register_demo!(runner, demo_integer_mod_val_ref);
    register_demo!(runner, demo_integer_mod_ref_val);
    register_demo!(runner, demo_integer_mod_ref_ref);
    register_demo!(runner, demo_integer_mod_assign);
    register_demo!(runner, demo_integer_mod_assign_ref);
    register_demo!(runner, demo_integer_rem);
    register_demo!(runner, demo_integer_rem_val_ref);
    register_demo!(runner, demo_integer_rem_ref_val);
    register_demo!(runner, demo_integer_rem_ref_ref);
    register_demo!(runner, demo_integer_rem_assign);
    register_demo!(runner, demo_integer_rem_assign_ref);
    register_demo!(runner, demo_integer_ceiling_mod);
    register_demo!(runner, demo_integer_ceiling_mod_val_ref);
    register_demo!(runner, demo_integer_ceiling_mod_ref_val);
    register_demo!(runner, demo_integer_ceiling_mod_ref_ref);
    register_demo!(runner, demo_integer_ceiling_mod_assign);
    register_demo!(runner, demo_integer_ceiling_mod_assign_ref);

    register_bench!(runner, benchmark_integer_mod_library_comparison);
    register_bench!(runner, benchmark_integer_mod_algorithms);
    register_bench!(runner, benchmark_integer_mod_evaluation_strategy);
    register_bench!(runner, benchmark_integer_mod_assign_evaluation_strategy);
    register_bench!(runner, benchmark_integer_rem_library_comparison);
    register_bench!(runner, benchmark_integer_rem_algorithms);
    register_bench!(runner, benchmark_integer_rem_evaluation_strategy);
    register_bench!(runner, benchmark_integer_rem_assign_evaluation_strategy);
    register_bench!(runner, benchmark_integer_ceiling_mod_library_comparison);
    register_bench!(runner, benchmark_integer_ceiling_mod_algorithms);
    register_bench!(runner, benchmark_integer_ceiling_mod_evaluation_strategy);
    register_bench!(
        runner,
        benchmark_integer_ceiling_mod_assign_evaluation_strategy
    );
}

fn demo_integer_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{}.mod_op({}) = {}", x_old, y_old, x.mod_op(y));
    }
}

fn demo_integer_mod_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("{}.mod_op(&{}) = {}", x_old, y, x.mod_op(&y));
    }
}

fn demo_integer_mod_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!("(&{}).mod_op({}) = {:?}", x, y_old, (&x).mod_op(y));
    }
}

fn demo_integer_mod_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        println!("(&{}).mod_op(&{}) = {:?}", x, y, (&x).mod_op(&y));
    }
}

fn demo_integer_mod_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.mod_assign(y);
        println!("x := {x_old}; x.mod_assign({y_old}); x = {x}");
    }
}

fn demo_integer_mod_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.mod_assign(&y);
        println!("x := {x_old}; x.mod_assign(&{y}); x = {x}");
    }
}

fn demo_integer_rem(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} % {} = {:?}", x_old, y_old, x % y);
    }
}

fn demo_integer_rem_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("{} % &{} = {:?}", x_old, y, x % &y);
    }
}

fn demo_integer_rem_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!("&{} % {} = {:?}", x, y_old, &x % y);
    }
}

fn demo_integer_rem_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        println!("&{} % &{} = {:?}", x, y, &x % &y);
    }
}

fn demo_integer_rem_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x %= y;
        println!("x := {x_old}; x %= {y_old}; x = {x}");
    }
}

fn demo_integer_rem_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        x %= &y;
        println!("x := {x_old}; x %= &{y}; x = {x}");
    }
}

fn demo_integer_ceiling_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{}.ceiling_mod({}) = {}", x_old, y_old, x.ceiling_mod(y));
    }
}

fn demo_integer_ceiling_mod_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("{}.ceiling_mod(&{}) = {}", x_old, y, x.ceiling_mod(&y));
    }
}

fn demo_integer_ceiling_mod_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!("(&{}).ceiling_mod({}) = {}", x, y_old, (&x).ceiling_mod(y));
    }
}

fn demo_integer_ceiling_mod_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        println!("(&{}).ceiling_mod(&{}) = {}", x, y, (&x).ceiling_mod(&y));
    }
}

fn demo_integer_ceiling_mod_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.ceiling_mod_assign(y);
        println!("x := {x_old}; x.ceiling_mod_assign({y_old}); x = {x}");
    }
}

fn demo_integer_ceiling_mod_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.ceiling_mod_assign(&y);
        println!("x := {x_old}; x.ceiling_mod_assign(&{y}); x = {x}");
    }
}

fn benchmark_integer_mod_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.mod_op(Integer)",
        BenchmarkType::LibraryComparison,
        integer_pair_gen_var_1_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, _, (x, y))| no_out!(x.mod_op(y))),
            ("num", &mut |((x, y), _, _)| no_out!(x.mod_floor(&y))),
            ("rug", &mut |(_, (x, y), _)| no_out!(x.rem_floor(y))),
        ],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_integer_mod_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.mod_op(Integer)",
        BenchmarkType::Algorithms,
        integer_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            ("standard", &mut |(x, y)| no_out!(x.mod_op(y))),
            ("using div_mod", &mut |(x, y)| no_out!(x.div_mod(y).1)),
        ],
    );
}

fn benchmark_integer_mod_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.mod_op(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            (
                "Integer.mod_op(Integer)",
                &mut |(x, y)| no_out!(x.mod_op(y)),
            ),
            ("Integer.mod_op(&Integer)", &mut |(x, y)| {
                no_out!(x.mod_op(&y))
            }),
            ("(&Integer).mod_op(Integer)", &mut |(x, y)| {
                no_out!((&x).mod_op(y))
            }),
            ("(&Integer).mod_op(&Integer)", &mut |(x, y)| {
                no_out!((&x).mod_op(&y))
            }),
        ],
    );
}

fn benchmark_integer_mod_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.mod_assign(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Integer.mod_assign(Integer)", &mut |(mut x, y)| {
                no_out!(x.mod_assign(y))
            }),
            ("Integer.mod_assign(&Integer)", &mut |(mut x, y)| {
                no_out!(x.mod_assign(&y))
            }),
        ],
    );
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_integer_rem_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.rem(Integer)",
        BenchmarkType::LibraryComparison,
        integer_pair_gen_var_1_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, _, (x, y))| no_out!(x % y)),
            ("num", &mut |((x, y), _, _)| no_out!(x % y)),
            ("rug", &mut |(_, (x, y), _)| no_out!(x % y)),
        ],
    );
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_integer_rem_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.rem(Integer)",
        BenchmarkType::Algorithms,
        integer_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            ("standard", &mut |(x, y)| no_out!(x % y)),
            ("using div_rem", &mut |(x, y)| no_out!(x.div_rem(y).1)),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_integer_rem_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.rem(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Integer % Integer", &mut |(x, y)| no_out!(x % y)),
            ("Integer % &Integer", &mut |(x, y)| no_out!(x % &y)),
            ("&Integer % Integer", &mut |(x, y)| no_out!(&x % y)),
            ("&Integer % &Integer", &mut |(x, y)| no_out!(&x % &y)),
        ],
    );
}

fn benchmark_integer_rem_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.rem_assign(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Integer %= Integer", &mut |(mut x, y)| x %= y),
            ("Integer %= &Integer", &mut |(mut x, y)| x %= &y),
        ],
    );
}

fn benchmark_integer_ceiling_mod_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.ceiling_mod(Integer)",
        BenchmarkType::LibraryComparison,
        integer_pair_gen_var_1_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(x.ceiling_mod(y))),
            ("rug", &mut |((x, y), _)| no_out!(x.rem_ceil(y))),
        ],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_integer_ceiling_mod_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.ceiling_mod(Integer)",
        BenchmarkType::Algorithms,
        integer_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            ("standard", &mut |(x, y)| no_out!(x.ceiling_mod(y))),
            ("using ceiling_div_mod", &mut |(x, y)| {
                no_out!(x.ceiling_div_mod(y).1)
            }),
        ],
    );
}

fn benchmark_integer_ceiling_mod_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.ceiling_mod(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Integer.ceiling_mod(Integer)", &mut |(x, y)| {
                no_out!(x.ceiling_mod(y))
            }),
            ("Integer.ceiling_mod(&Integer)", &mut |(x, y)| {
                no_out!(x.ceiling_mod(&y))
            }),
            ("(&Integer).ceiling_mod(Integer)", &mut |(x, y)| {
                no_out!((&x).ceiling_mod(y))
            }),
            ("(&Integer).ceiling_mod(&Integer)", &mut |(x, y)| {
                no_out!((&x).ceiling_mod(&y))
            }),
        ],
    );
}

fn benchmark_integer_ceiling_mod_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.ceiling_mod_assign(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Integer.ceiling_mod_assign(Integer)", &mut |(mut x, y)| {
                no_out!(x.ceiling_mod_assign(y))
            }),
            ("Integer.ceiling_mod_assign(&Integer)", &mut |(mut x, y)| {
                no_out!(x.ceiling_mod_assign(&y))
            }),
        ],
    );
}
