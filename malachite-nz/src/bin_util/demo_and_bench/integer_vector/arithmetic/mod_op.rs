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
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::pair_1_integer_vector_bit_bucketer;
use malachite_nz::test_util::generators::{
    integer_vector_integer_pair_gen_var_1, integer_vector_natural_pair_gen_var_1,
    integer_vector_unsigned_pair_gen,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_vector_mod_op);
    register_demo!(runner, demo_integer_vector_mod_op_ref);
    register_unsigned_demos!(runner, demo_integer_vector_mod_op_unsigned);
    register_demo!(runner, demo_integer_vector_rem);
    register_demo!(runner, demo_integer_vector_rem_ref);
    register_demo!(runner, demo_integer_vector_rem_assign);

    register_bench!(runner, benchmark_integer_vector_mod_op_evaluation_strategy);
    register_bench!(runner, benchmark_integer_vector_rem_evaluation_strategy);
    register_unsigned_benches!(runner, benchmark_integer_vector_mod_op_unsigned_algorithms);
}

fn demo_integer_vector_mod_op(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, m) in integer_vector_natural_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!("{v_old}.mod_op({m}) = {}", v.mod_op(&m));
    }
}

fn demo_integer_vector_mod_op_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, m) in integer_vector_natural_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{v}).mod_op({m}) = {}", (&v).mod_op(&m));
    }
}

fn demo_integer_vector_mod_op_unsigned<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Natural: From<T>,
{
    for (v, m) in integer_vector_unsigned_pair_gen::<T>()
        .get(gm, config)
        .filter(|(_, m)| *m != T::ZERO)
        .take(limit)
    {
        println!("(&{v}).mod_op({m}) = {}", (&v).mod_op(m));
    }
}

fn benchmark_integer_vector_mod_op_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerVector.mod_op(Natural)",
        BenchmarkType::EvaluationStrategy,
        integer_vector_natural_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_vector_bit_bucketer("v"),
        &mut [
            ("IntegerVector.mod_op(Natural)", &mut |(v, m)| {
                no_out!(v.mod_op(m));
            }),
            ("IntegerVector.mod_op(&Natural)", &mut |(v, m)| {
                no_out!(v.mod_op(&m));
            }),
            ("(&IntegerVector).mod_op(Natural)", &mut |(v, m)| {
                no_out!((&v).mod_op(m));
            }),
            ("(&IntegerVector).mod_op(&Natural)", &mut |(v, m)| {
                no_out!((&v).mod_op(&m));
            }),
        ],
    );
}

fn benchmark_integer_vector_mod_op_unsigned_algorithms<
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
        &format!("(&IntegerVector).mod_op({})", T::NAME),
        BenchmarkType::Algorithms,
        integer_vector_unsigned_pair_gen::<T>()
            .get(gm, config)
            .filter(|(_, m)| *m != T::ZERO),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_vector_bit_bucketer("v"),
        &mut [
            ("default", &mut |(v, m)| {
                no_out!((&v).mod_op(m));
            }),
            (
                "reducing modulo a Natural, then converting",
                &mut |(v, m)| {
                    let _ = UnsignedVector::<T>::try_from((&v).mod_op(Natural::from(m)));
                },
            ),
        ],
    );
}

fn demo_integer_vector_rem(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, m) in integer_vector_integer_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!("{v_old} % {m} = {}", v % &m);
    }
}

fn demo_integer_vector_rem_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, m) in integer_vector_integer_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("&{v} % {m} = {}", &v % &m);
    }
}

fn demo_integer_vector_rem_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut v, m) in integer_vector_integer_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v %= &m;
        println!("v := {v_old}; v %= {m}; v = {v}");
    }
}

fn benchmark_integer_vector_rem_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerVector % Integer",
        BenchmarkType::EvaluationStrategy,
        integer_vector_integer_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_vector_bit_bucketer("v"),
        &mut [
            ("IntegerVector % Integer", &mut |(v, m)| {
                no_out!(v % m);
            }),
            ("IntegerVector % &Integer", &mut |(v, m)| {
                no_out!(v % &m);
            }),
            ("&IntegerVector % Integer", &mut |(v, m)| {
                no_out!(&v % m);
            }),
            ("&IntegerVector % &Integer", &mut |(v, m)| {
                no_out!(&v % &m);
            }),
            ("IntegerVector %= Integer", &mut |(mut v, m)| {
                v %= m;
            }),
            ("IntegerVector %= &Integer", &mut |(mut v, m)| {
                v %= &m;
            }),
        ],
    );
}
