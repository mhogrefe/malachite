// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPowerOf2Shl, ModPowerOf2ShlAssign};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::natural_polynomial_unsigned_unsigned_triple_gen_var_1;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_power_of_2_shl::*;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_natural_polynomial_mod_power_of_2_shl);
    register_unsigned_demos!(runner, demo_natural_polynomial_mod_power_of_2_shl_ref);
    register_unsigned_demos!(runner, demo_natural_polynomial_mod_power_of_2_shl_assign);

    register_unsigned_benches!(
        runner,
        benchmark_natural_polynomial_mod_power_of_2_shl_evaluation_strategy
    );
    register_unsigned_benches!(
        runner,
        benchmark_natural_polynomial_mod_power_of_2_shl_algorithms
    );
}

fn demo_natural_polynomial_mod_power_of_2_shl<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    NaturalPolynomial: ModPowerOf2Shl<T, Output = NaturalPolynomial>,
{
    for (p, bits, pow) in natural_polynomial_unsigned_unsigned_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).mod_power_of_2_shl({bits}, {pow}) = {}",
            p.mod_power_of_2_shl(bits, pow)
        );
    }
}

fn demo_natural_polynomial_mod_power_of_2_shl_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a NaturalPolynomial: ModPowerOf2Shl<T, Output = NaturalPolynomial>,
{
    for (p, bits, pow) in natural_polynomial_unsigned_unsigned_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_power_of_2_shl({bits}, {pow}) = {}",
            (&p).mod_power_of_2_shl(bits, pow)
        );
    }
}

fn demo_natural_polynomial_mod_power_of_2_shl_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    NaturalPolynomial: ModPowerOf2ShlAssign<T>,
{
    for (mut p, bits, pow) in natural_polynomial_unsigned_unsigned_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.mod_power_of_2_shl_assign(bits, pow);
        println!("p := {p_old}; p.mod_power_of_2_shl_assign({bits}, {pow}); p = {p}");
    }
}

fn benchmark_natural_polynomial_mod_power_of_2_shl_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    NaturalPolynomial: ModPowerOf2Shl<T, Output = NaturalPolynomial> + ModPowerOf2ShlAssign<T>,
    for<'a> &'a NaturalPolynomial: ModPowerOf2Shl<T, Output = NaturalPolynomial>,
{
    run_benchmark(
        &format!("NaturalPolynomial.mod_power_of_2_shl({}, u64)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_unsigned_unsigned_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            ("p.mod_power_of_2_shl(bits, pow)", &mut |(p, bits, pow)| {
                no_out!(p.mod_power_of_2_shl(bits, pow));
            }),
            ("(&p).mod_power_of_2_shl(bits, pow)", &mut |(
                p,
                bits,
                pow,
            )| {
                no_out!((&p).mod_power_of_2_shl(bits, pow));
            }),
            (
                "p.mod_power_of_2_shl_assign(bits, pow)",
                &mut |(mut p, bits, pow)| p.mod_power_of_2_shl_assign(bits, pow),
            ),
        ],
    );
}

fn benchmark_natural_polynomial_mod_power_of_2_shl_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    for<'a> &'a NaturalPolynomial: ModPowerOf2Shl<T, Output = NaturalPolynomial>,
    u64: ExactFrom<T>,
{
    run_benchmark(
        &format!("NaturalPolynomial.mod_power_of_2_shl({}, u64)", T::NAME),
        BenchmarkType::Algorithms,
        natural_polynomial_unsigned_unsigned_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, bits, pow)| {
                no_out!((&p).mod_power_of_2_shl(bits, pow));
            }),
            ("naive", &mut |(p, bits, pow)| {
                no_out!(mod_power_of_2_shl_naive(&p, bits, pow));
            }),
        ],
    );
}
