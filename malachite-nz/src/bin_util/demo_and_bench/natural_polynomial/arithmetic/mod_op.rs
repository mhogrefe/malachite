// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Mod, ModAssign, PowerOf2};
use malachite_base::num::basic::traits::One;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::{
    natural_polynomial_gen, natural_polynomial_natural_pair_gen_var_1,
    natural_polynomial_unsigned_pair_gen,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_mod_op);
    register_demo!(runner, demo_natural_polynomial_mod_assign);
    register_demo!(runner, demo_natural_polynomial_rem);
    register_demo!(runner, demo_natural_polynomial_rem_ref);
    register_demo!(runner, demo_natural_polynomial_rem_assign);
    register_demo!(runner, demo_natural_polynomial_rem_special_moduli);

    register_unsigned_demos!(runner, demo_natural_polynomial_rem_unsigned);
    register_unsigned_demos!(runner, demo_natural_polynomial_rem_unsigned_ref);

    register_bench!(runner, benchmark_natural_polynomial_rem_evaluation_strategy);
    register_unsigned_benches!(runner, benchmark_natural_polynomial_rem_unsigned_algorithms);
}

fn demo_natural_polynomial_mod_op(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, m) in natural_polynomial_natural_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!("({p_old}).mod_op({m}) = {}", p.mod_op(&m));
    }
}

fn demo_natural_polynomial_mod_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, m) in natural_polynomial_natural_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.mod_assign(&m);
        println!("p := {p_old}; p.mod_assign({m}); p = {p}");
    }
}

fn demo_natural_polynomial_rem(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, m) in natural_polynomial_natural_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!("({p_old}) % {m} = {}", p % &m);
    }
}

fn demo_natural_polynomial_rem_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, m) in natural_polynomial_natural_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("&({p}) % {m} = {}", &p % &m);
    }
}

fn demo_natural_polynomial_rem_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, m) in natural_polynomial_natural_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p %= &m;
        println!("p := {p_old}; p %= {m}; p = {p}");
    }
}

// The moduli the property tests single out: 1, where everything vanishes; one more than the
// largest coefficient, which leaves the polynomial alone; and 2, 2^7, 2^64, and 2^100, which agree
// with `mod_power_of_2`.
fn demo_natural_polynomial_rem_special_moduli(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in natural_polynomial_gen().get(gm, config).take(limit) {
        let above = p
            .coefficients_asc()
            .iter()
            .max()
            .cloned()
            .unwrap_or_default()
            + Natural::ONE;
        for m in [Natural::ONE, above]
            .into_iter()
            .chain([1, 7, 64, 100].into_iter().map(Natural::power_of_2))
        {
            println!("&({p}) % {m} = {}", &p % &m);
        }
    }
}

// The remainders are what is being timed, so the benchmark discards them on purpose.
#[allow(unused_must_use)]
fn benchmark_natural_polynomial_rem_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial % Natural",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_natural_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            ("NaturalPolynomial % Natural", &mut |(p, m)| no_out!(p % m)),
            ("NaturalPolynomial % &Natural", &mut |(p, m)| {
                no_out!(p % &m);
            }),
            ("&NaturalPolynomial % Natural", &mut |(p, m)| {
                no_out!(&p % m);
            }),
            ("&NaturalPolynomial % &Natural", &mut |(p, m)| {
                no_out!(&p % &m);
            }),
            ("NaturalPolynomial %= Natural", &mut |(mut p, m)| p %= m),
            ("NaturalPolynomial %= &Natural", &mut |(mut p, m)| p %= &m),
            ("NaturalPolynomial.mod_op(&Natural)", &mut |(p, m)| {
                no_out!(p.mod_op(&m));
            }),
        ],
    );
}

// The generator's second value can be zero, which is not a divisor, so the demos and benchmarks
// step past it.
fn demo_natural_polynomial_rem_unsigned<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Natural: From<T>,
{
    for (p, m) in natural_polynomial_unsigned_pair_gen::<T>()
        .get(gm, config)
        .filter(|(_, m)| *m != T::ZERO)
        .take(limit)
    {
        let p_old = p.clone();
        println!("({p_old}) % {m} = {}", p % m);
    }
}

fn demo_natural_polynomial_rem_unsigned_ref<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Natural: From<T>,
{
    for (p, m) in natural_polynomial_unsigned_pair_gen::<T>()
        .get(gm, config)
        .filter(|(_, m)| *m != T::ZERO)
        .take(limit)
    {
        println!("&({p}) % {m} = {}", &p % m);
    }
}

// The remainders are what is being timed, so the benchmark discards them on purpose.
#[allow(unused_must_use)]
fn benchmark_natural_polynomial_rem_unsigned_algorithms<
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
        &format!("&NaturalPolynomial % {}", T::NAME),
        BenchmarkType::Algorithms,
        natural_polynomial_unsigned_pair_gen::<T>()
            .get(gm, config)
            .filter(|(_, m)| *m != T::ZERO),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, m)| no_out!(&p % m)),
            (
                "reducing modulo a Natural, then converting",
                &mut |(p, m)| {
                    let _ = UnsignedPolynomial::<T>::try_from(&p % Natural::from(m));
                },
            ),
        ],
    );
}
