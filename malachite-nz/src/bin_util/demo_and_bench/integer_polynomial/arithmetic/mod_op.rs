// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Mod, PowerOf2};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::pair_1_integer_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::{
    integer_polynomial_gen, integer_polynomial_integer_pair_gen_var_1,
    integer_polynomial_natural_pair_gen_var_1, integer_polynomial_unsigned_pair_gen,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_mod_op);
    register_demo!(runner, demo_integer_polynomial_mod_op_ref);
    register_demo!(runner, demo_integer_polynomial_mod_op_power_of_2_moduli);

    register_unsigned_demos!(runner, demo_integer_polynomial_mod_op_unsigned);
    register_demo!(runner, demo_integer_polynomial_rem);
    register_demo!(runner, demo_integer_polynomial_rem_ref);
    register_demo!(runner, demo_integer_polynomial_rem_assign);

    register_bench!(
        runner,
        benchmark_integer_polynomial_mod_op_evaluation_strategy
    );
    register_unsigned_benches!(
        runner,
        benchmark_integer_polynomial_mod_op_unsigned_algorithms
    );
    register_bench!(runner, benchmark_integer_polynomial_rem_evaluation_strategy);
}

fn demo_integer_polynomial_mod_op(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, m) in integer_polynomial_natural_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!("({p_old}).mod_op({m}) = {}", p.mod_op(&m));
    }
}

fn demo_integer_polynomial_mod_op_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, m) in integer_polynomial_natural_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).mod_op({m}) = {}", (&p).mod_op(&m));
    }
}

// The moduli 1, 2, 2^7, 2^64, and 2^100 are the ones the property tests single out: everything
// vanishes for the first, and the others agree with `mod_power_of_2`.
fn demo_integer_polynomial_mod_op_power_of_2_moduli(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in integer_polynomial_gen().get(gm, config).take(limit) {
        for pow in [0, 1, 7, 64, 100] {
            let m = Natural::power_of_2(pow);
            println!("(&({p})).mod_op({m}) = {}", (&p).mod_op(&m));
        }
    }
}

fn benchmark_integer_polynomial_mod_op_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.mod_op(Natural)",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_natural_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_polynomial_bit_bucketer("p"),
        &mut [
            ("IntegerPolynomial.mod_op(Natural)", &mut |(p, m)| {
                no_out!(p.mod_op(m));
            }),
            ("IntegerPolynomial.mod_op(&Natural)", &mut |(p, m)| {
                no_out!(p.mod_op(&m));
            }),
            ("(&IntegerPolynomial).mod_op(Natural)", &mut |(p, m)| {
                no_out!((&p).mod_op(m));
            }),
            ("(&IntegerPolynomial).mod_op(&Natural)", &mut |(p, m)| {
                no_out!((&p).mod_op(&m));
            }),
        ],
    );
}

// The generator's second value can be zero, which is not a divisor, so the demo and benchmark step
// past it.
fn demo_integer_polynomial_mod_op_unsigned<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Natural: From<T>,
{
    for (p, m) in integer_polynomial_unsigned_pair_gen::<T>()
        .get(gm, config)
        .filter(|(_, m)| *m != T::ZERO)
        .take(limit)
    {
        println!("(&({p})).mod_op({m}) = {}", (&p).mod_op(m));
    }
}

// The remainders are what is being timed, so the benchmark discards them on purpose.
#[allow(unused_must_use)]
fn benchmark_integer_polynomial_mod_op_unsigned_algorithms<
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
        &format!("(&IntegerPolynomial).mod_op({})", T::NAME),
        BenchmarkType::Algorithms,
        integer_polynomial_unsigned_pair_gen::<T>()
            .get(gm, config)
            .filter(|(_, m)| *m != T::ZERO),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, m)| no_out!((&p).mod_op(m))),
            (
                "reducing modulo a Natural, then converting",
                &mut |(p, m)| {
                    let _ = malachite_base::unsigned_polynomial::UnsignedPolynomial::<T>::try_from(
                        (&p).mod_op(Natural::from(m)),
                    );
                },
            ),
        ],
    );
}

fn demo_integer_polynomial_rem(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, m) in integer_polynomial_integer_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!("({p_old}) % {m} = {}", p % &m);
    }
}

fn demo_integer_polynomial_rem_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, m) in integer_polynomial_integer_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("&({p}) % {m} = {}", &p % &m);
    }
}

fn demo_integer_polynomial_rem_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, m) in integer_polynomial_integer_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p %= &m;
        println!("p := {p_old}; p %= {m}; p = {p}");
    }
}

// The remainders are what is being timed, so the benchmark discards them on purpose.
#[allow(unused_must_use)]
fn benchmark_integer_polynomial_rem_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial % Integer",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_integer_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_polynomial_bit_bucketer("p"),
        &mut [
            ("IntegerPolynomial % Integer", &mut |(p, m)| no_out!(p % m)),
            ("IntegerPolynomial % &Integer", &mut |(p, m)| {
                no_out!(p % &m);
            }),
            ("&IntegerPolynomial % Integer", &mut |(p, m)| {
                no_out!(&p % m);
            }),
            ("&IntegerPolynomial % &Integer", &mut |(p, m)| {
                no_out!(&p % &m);
            }),
            ("IntegerPolynomial %= Integer", &mut |(mut p, m)| p %= m),
            ("IntegerPolynomial %= &Integer", &mut |(mut p, m)| p %= &m),
        ],
    );
}
