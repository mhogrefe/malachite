// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::{ModTaylorShift, ModTaylorShiftAssign};
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_polynomial_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_2,
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_12,
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_13,
};
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_taylor_shift::*;
use malachite_base::unsigned_polynomial::arithmetic::mod_taylor_shift::{
    mod_taylor_shift_divide_and_conquer, mod_taylor_shift_horner,
};

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_taylor_shift);
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_taylor_shift_ref);
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_taylor_shift_assign);
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_taylor_shift_long);
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_taylor_shift_small_shift
    );

    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_taylor_shift_evaluation_strategy
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_taylor_shift_algorithms
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_taylor_shift_algorithms_long
    );
}

fn demo_unsigned_polynomial_mod_taylor_shift<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, c, m) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).mod_taylor_shift({c}, {m}) = {}",
            p.mod_taylor_shift(c, m)
        );
    }
}

fn demo_unsigned_polynomial_mod_taylor_shift_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, c, m) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_taylor_shift({c}, {m}) = {}",
            (&p).mod_taylor_shift(c, m)
        );
    }
}

fn demo_unsigned_polynomial_mod_taylor_shift_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, c, m) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.mod_taylor_shift_assign(c, m);
        println!("p := {p_old}; p.mod_taylor_shift_assign({c}, {m}); p = {p}");
    }
}

fn demo_unsigned_polynomial_mod_taylor_shift_long<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, c, m) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_12::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({p}).mod_taylor_shift({c}, {m}) = {}",
            (&p).mod_taylor_shift(c, m)
        );
    }
}

fn demo_unsigned_polynomial_mod_taylor_shift_small_shift<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, c, m) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_13::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({p}).mod_taylor_shift({c}, {m}) = {}",
            (&p).mod_taylor_shift(c, m)
        );
    }
}

fn benchmark_unsigned_polynomial_mod_taylor_shift_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_taylor_shift({}, {})",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            ("p.mod_taylor_shift(c, m)", &mut |(p, c, m)| {
                no_out!(p.mod_taylor_shift(c, m));
            }),
            ("(&p).mod_taylor_shift(c, m)", &mut |(p, c, m)| {
                no_out!((&p).mod_taylor_shift(c, m));
            }),
            ("p.mod_taylor_shift_assign(c, m)", &mut |(mut p, c, m)| {
                p.mod_taylor_shift_assign(c, m);
            }),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_taylor_shift_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_taylor_shift({}, {})",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            ("default", &mut |(p, c, m)| {
                no_out!((&p).mod_taylor_shift(c, m));
            }),
            ("Horner", &mut |(p, c, m)| {
                let mut xs = p.coefficients_asc().to_vec();
                mod_taylor_shift_horner(&mut xs, c, m);
            }),
            ("divide and conquer", &mut |(p, c, m)| {
                let mut xs = p.coefficients_asc().to_vec();
                mod_taylor_shift_divide_and_conquer(&mut xs, c, m);
            }),
            ("naive", &mut |(p, c, m)| {
                no_out!(mod_taylor_shift_naive(&p, c, m));
            }),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_taylor_shift_algorithms_long<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_taylor_shift({}, {})",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_unsigned_triple_gen_var_12::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            ("default", &mut |(p, c, m)| {
                no_out!((&p).mod_taylor_shift(c, m));
            }),
            ("naive", &mut |(p, c, m)| {
                no_out!(mod_taylor_shift_naive(&p, c, m));
            }),
        ],
    );
}
