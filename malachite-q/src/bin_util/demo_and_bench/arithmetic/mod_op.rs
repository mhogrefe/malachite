// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{CeilingMod, CeilingModAssign, Mod, ModAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::arithmetic::mod_op::{ceiling_mod_naive, mod_op_naive, rem_naive};
use malachite_q::test_util::bench::bucketers::{
    pair_1_rational_bit_bucketer, pair_2_pair_1_rational_bit_bucketer,
};
use malachite_q::test_util::generators::{rational_pair_gen_var_1, rational_pair_gen_var_1_nm};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_mod);
    register_demo!(runner, demo_rational_mod_val_ref);
    register_demo!(runner, demo_rational_mod_ref_val);
    register_demo!(runner, demo_rational_mod_ref_ref);
    register_demo!(runner, demo_rational_mod_assign);
    register_demo!(runner, demo_rational_mod_assign_ref);
    register_demo!(runner, demo_rational_rem);
    register_demo!(runner, demo_rational_rem_val_ref);
    register_demo!(runner, demo_rational_rem_ref_val);
    register_demo!(runner, demo_rational_rem_ref_ref);
    register_demo!(runner, demo_rational_rem_assign);
    register_demo!(runner, demo_rational_rem_assign_ref);
    register_demo!(runner, demo_rational_ceiling_mod);
    register_demo!(runner, demo_rational_ceiling_mod_val_ref);
    register_demo!(runner, demo_rational_ceiling_mod_ref_val);
    register_demo!(runner, demo_rational_ceiling_mod_ref_ref);
    register_demo!(runner, demo_rational_ceiling_mod_assign);
    register_demo!(runner, demo_rational_ceiling_mod_assign_ref);

    register_bench!(runner, benchmark_rational_mod_evaluation_strategy);
    register_bench!(runner, benchmark_rational_mod_algorithms);
    register_bench!(runner, benchmark_rational_mod_assign_evaluation_strategy);
    register_bench!(runner, benchmark_rational_rem_library_comparison);
    register_bench!(runner, benchmark_rational_rem_evaluation_strategy);
    register_bench!(runner, benchmark_rational_rem_assign_evaluation_strategy);
    register_bench!(runner, benchmark_rational_ceiling_mod_evaluation_strategy);
    register_bench!(runner, benchmark_rational_ceiling_mod_algorithms);
    register_bench!(
        runner,
        benchmark_rational_ceiling_mod_assign_evaluation_strategy
    );
}

fn demo_rational_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{}.mod_op({}) = {}", x_old, y_old, x.mod_op(y));
    }
}

fn demo_rational_mod_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("{}.mod_op(&{}) = {}", x_old, y, x.mod_op(&y));
    }
}

fn demo_rational_mod_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_pair_gen_var_1().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!("(&{}).mod_op({}) = {:?}", x, y_old, (&x).mod_op(y));
    }
}

fn demo_rational_mod_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_pair_gen_var_1().get(gm, config).take(limit) {
        println!("(&{}).mod_op(&{}) = {:?}", x, y, (&x).mod_op(&y));
    }
}

fn demo_rational_mod_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in rational_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.mod_assign(y);
        println!("x := {x_old}; x.mod_assign({y_old}); x = {x}");
    }
}

fn demo_rational_mod_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in rational_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.mod_assign(&y);
        println!("x := {x_old}; x.mod_assign(&{y}); x = {x}");
    }
}

fn demo_rational_rem(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} % {} = {:?}", x_old, y_old, x % y);
    }
}

fn demo_rational_rem_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("{} % &{} = {:?}", x_old, y, x % &y);
    }
}

fn demo_rational_rem_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_pair_gen_var_1().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!("&{} % {} = {:?}", x, y_old, &x % y);
    }
}

fn demo_rational_rem_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_pair_gen_var_1().get(gm, config).take(limit) {
        println!("&{} % &{} = {:?}", x, y, &x % &y);
    }
}

fn demo_rational_rem_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in rational_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x %= y;
        println!("x := {x_old}; x %= {y_old}; x = {x}");
    }
}

fn demo_rational_rem_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in rational_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        x %= &y;
        println!("x := {x_old}; x %= &{y}; x = {x}");
    }
}

fn demo_rational_ceiling_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{}.ceiling_mod({}) = {}", x_old, y_old, x.ceiling_mod(y));
    }
}

fn demo_rational_ceiling_mod_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("{}.ceiling_mod(&{}) = {}", x_old, y, x.ceiling_mod(&y));
    }
}

fn demo_rational_ceiling_mod_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_pair_gen_var_1().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!("(&{}).ceiling_mod({}) = {}", x, y_old, (&x).ceiling_mod(y));
    }
}

fn demo_rational_ceiling_mod_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_pair_gen_var_1().get(gm, config).take(limit) {
        println!("(&{}).ceiling_mod(&{}) = {}", x, y, (&x).ceiling_mod(&y));
    }
}

fn demo_rational_ceiling_mod_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in rational_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.ceiling_mod_assign(y);
        println!("x := {x_old}; x.ceiling_mod_assign({y_old}); x = {x}");
    }
}

fn demo_rational_ceiling_mod_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in rational_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.ceiling_mod_assign(&y);
        println!("x := {x_old}; x.ceiling_mod_assign(&{y}); x = {x}");
    }
}

fn benchmark_rational_mod_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.mod_op(Rational)",
        BenchmarkType::EvaluationStrategy,
        rational_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [
            ("Rational.mod_op(Rational)", &mut |(x, y)| {
                no_out!(x.mod_op(y));
            }),
            ("Rational.mod_op(&Rational)", &mut |(x, y)| {
                no_out!(x.mod_op(&y));
            }),
            ("(&Rational).mod_op(Rational)", &mut |(x, y)| {
                no_out!((&x).mod_op(y));
            }),
            ("(&Rational).mod_op(&Rational)", &mut |(x, y)| {
                no_out!((&x).mod_op(&y));
            }),
        ],
    );
}

fn benchmark_rational_mod_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.mod_op(Rational)",
        BenchmarkType::Algorithms,
        rational_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [
            ("default", &mut |(x, y)| {
                no_out!(x.mod_op(y));
            }),
            ("naive", &mut |(x, y)| {
                no_out!(mod_op_naive(x, y));
            }),
        ],
    );
}

fn benchmark_rational_mod_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.mod_assign(Rational)",
        BenchmarkType::EvaluationStrategy,
        rational_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [
            ("Rational.mod_assign(Rational)", &mut |(mut x, y)| {
                no_out!(x.mod_assign(y));
            }),
            ("Rational.mod_assign(&Rational)", &mut |(mut x, y)| {
                no_out!(x.mod_assign(&y));
            }),
        ],
    );
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_rational_rem_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.rem(Rational)",
        BenchmarkType::LibraryComparison,
        rational_pair_gen_var_1_nm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_rational_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(x % y)),
            ("num", &mut |((x, y), _)| no_out!(x % y)),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_rational_rem_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.rem(Rational)",
        BenchmarkType::EvaluationStrategy,
        rational_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [
            ("Rational % Rational", &mut |(x, y)| no_out!(x % y)),
            ("Rational % &Rational", &mut |(x, y)| no_out!(x % &y)),
            ("&Rational % Rational", &mut |(x, y)| no_out!(&x % y)),
            ("&Rational % &Rational", &mut |(x, y)| no_out!(&x % &y)),
        ],
    );
}

#[allow(unused)]
fn benchmark_rational_rem_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational % Rational",
        BenchmarkType::Algorithms,
        rational_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [
            ("default", &mut |(x, y)| {
                no_out!(x % y);
            }),
            ("naive", &mut |(x, y)| {
                no_out!(rem_naive(x, y));
            }),
        ],
    );
}

fn benchmark_rational_rem_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.rem_assign(Rational)",
        BenchmarkType::EvaluationStrategy,
        rational_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [
            ("Rational %= Rational", &mut |(mut x, y)| x %= y),
            ("Rational %= &Rational", &mut |(mut x, y)| x %= &y),
        ],
    );
}

fn benchmark_rational_ceiling_mod_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.ceiling_mod(Rational)",
        BenchmarkType::EvaluationStrategy,
        rational_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [
            ("Rational.ceiling_mod(Rational)", &mut |(x, y)| {
                no_out!(x.ceiling_mod(y));
            }),
            ("Rational.ceiling_mod(&Rational)", &mut |(x, y)| {
                no_out!(x.ceiling_mod(&y));
            }),
            ("(&Rational).ceiling_mod(Rational)", &mut |(x, y)| {
                no_out!((&x).ceiling_mod(y));
            }),
            ("(&Rational).ceiling_mod(&Rational)", &mut |(x, y)| {
                no_out!((&x).ceiling_mod(&y));
            }),
        ],
    );
}

fn benchmark_rational_ceiling_mod_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.ceiling_mod(Rational)",
        BenchmarkType::Algorithms,
        rational_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [
            ("default", &mut |(x, y)| {
                no_out!(x.ceiling_mod(y));
            }),
            ("naive", &mut |(x, y)| {
                no_out!(ceiling_mod_naive(x, y));
            }),
        ],
    );
}

fn benchmark_rational_ceiling_mod_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.ceiling_mod_assign(Rational)",
        BenchmarkType::EvaluationStrategy,
        rational_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [
            (
                "Rational.ceiling_mod_assign(Rational)",
                &mut |(mut x, y)| {
                    no_out!(x.ceiling_mod_assign(y));
                },
            ),
            (
                "Rational.ceiling_mod_assign(&Rational)",
                &mut |(mut x, y)| {
                    no_out!(x.ceiling_mod_assign(&y));
                },
            ),
        ],
    );
}
