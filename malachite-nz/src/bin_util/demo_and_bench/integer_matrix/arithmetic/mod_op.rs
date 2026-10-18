// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Mod;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::pair_1_integer_matrix_bit_bucketer;
use malachite_nz::test_util::generators::{
    integer_matrix_integer_pair_gen_var_1, integer_matrix_natural_pair_gen_var_1,
    integer_matrix_unsigned_pair_gen_var_1,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_mod_op);
    register_demo!(runner, demo_integer_matrix_mod_op_ref);
    register_unsigned_demos!(runner, demo_integer_matrix_mod_op_unsigned);
    register_demo!(runner, demo_integer_matrix_rem);
    register_demo!(runner, demo_integer_matrix_rem_ref);
    register_demo!(runner, demo_integer_matrix_rem_assign);

    register_bench!(runner, benchmark_integer_matrix_mod_op_evaluation_strategy);
    register_bench!(runner, benchmark_integer_matrix_rem_evaluation_strategy);
    register_unsigned_benches!(runner, benchmark_integer_matrix_mod_op_unsigned_algorithms);
}

fn demo_integer_matrix_mod_op(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in integer_matrix_natural_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!("{a_old}.mod_op({m}) = {}", a.mod_op(&m));
    }
}

fn demo_integer_matrix_mod_op_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in integer_matrix_natural_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{a}).mod_op({m}) = {}", (&a).mod_op(&m));
    }
}

fn demo_integer_matrix_mod_op_unsigned<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Natural: From<T>,
{
    for (a, m) in integer_matrix_unsigned_pair_gen_var_1::<T>()
        .get(gm, config)
        .filter(|(_, m)| *m != T::ZERO)
        .take(limit)
    {
        println!("(&{a}).mod_op({m}) = {}", (&a).mod_op(m));
    }
}

fn benchmark_integer_matrix_mod_op_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.mod_op(Natural)",
        BenchmarkType::EvaluationStrategy,
        integer_matrix_natural_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_matrix_bit_bucketer("a"),
        &mut [
            ("IntegerMatrix.mod_op(Natural)", &mut |(a, m)| {
                no_out!(a.mod_op(m));
            }),
            ("IntegerMatrix.mod_op(&Natural)", &mut |(a, m)| {
                no_out!(a.mod_op(&m));
            }),
            ("(&IntegerMatrix).mod_op(Natural)", &mut |(a, m)| {
                no_out!((&a).mod_op(m));
            }),
            ("(&IntegerMatrix).mod_op(&Natural)", &mut |(a, m)| {
                no_out!((&a).mod_op(&m));
            }),
        ],
    );
}

fn benchmark_integer_matrix_mod_op_unsigned_algorithms<
    T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural> + for<'a> TryFrom<&'a Natural>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: From<T>,
{
    run_benchmark(
        &format!("(&IntegerMatrix).mod_op({})", T::NAME),
        BenchmarkType::Algorithms,
        integer_matrix_unsigned_pair_gen_var_1::<T>()
            .get(gm, config)
            .filter(|(_, m)| *m != T::ZERO),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |(a, m)| {
                no_out!((&a).mod_op(m));
            }),
            (
                "reducing modulo a Natural, then converting",
                &mut |(a, m)| {
                    let _ = UnsignedMatrix::<T>::try_from((&a).mod_op(Natural::from(m)));
                },
            ),
        ],
    );
}

fn demo_integer_matrix_rem(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in integer_matrix_integer_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!("{a_old} % {m} = {}", a % &m);
    }
}

fn demo_integer_matrix_rem_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in integer_matrix_integer_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("&{a} % {m} = {}", &a % &m);
    }
}

fn demo_integer_matrix_rem_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, m) in integer_matrix_integer_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a %= &m;
        println!("a := {a_old}; a %= {m}; a = {a}");
    }
}

fn benchmark_integer_matrix_rem_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix % Integer",
        BenchmarkType::EvaluationStrategy,
        integer_matrix_integer_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_matrix_bit_bucketer("a"),
        &mut [
            ("IntegerMatrix % Integer", &mut |(a, m)| {
                no_out!(a % m);
            }),
            ("IntegerMatrix % &Integer", &mut |(a, m)| {
                no_out!(a % &m);
            }),
            ("&IntegerMatrix % Integer", &mut |(a, m)| {
                no_out!(&a % m);
            }),
            ("&IntegerMatrix % &Integer", &mut |(a, m)| {
                no_out!(&a % &m);
            }),
            ("IntegerMatrix %= Integer", &mut |(mut a, m)| {
                a %= m;
            }),
            ("IntegerMatrix %= &Integer", &mut |(mut a, m)| {
                a %= &m;
            }),
        ],
    );
}
