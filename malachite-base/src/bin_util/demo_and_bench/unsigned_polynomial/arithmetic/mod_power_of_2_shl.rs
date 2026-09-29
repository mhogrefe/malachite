// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPowerOf2Shl, ModPowerOf2ShlAssign};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_polynomial_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_polynomial_unsigned_unsigned_triple_gen_var_3;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_shl::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_unsigned_demos!(runner, demo_unsigned_polynomial_mod_power_of_2_shl);
    register_unsigned_unsigned_demos!(runner, demo_unsigned_polynomial_mod_power_of_2_shl_ref);
    register_unsigned_unsigned_demos!(runner, demo_unsigned_polynomial_mod_power_of_2_shl_assign);

    register_unsigned_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_shl_evaluation_strategy
    );
    register_unsigned_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_shl_algorithms
    );
}

fn demo_unsigned_polynomial_mod_power_of_2_shl<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    UnsignedPolynomial<T>: ModPowerOf2Shl<U, Output = UnsignedPolynomial<T>>,
{
    for (p, bits, pow) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_3::<T, U>()
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

fn demo_unsigned_polynomial_mod_power_of_2_shl_ref<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a UnsignedPolynomial<T>: ModPowerOf2Shl<U, Output = UnsignedPolynomial<T>>,
{
    for (p, bits, pow) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_3::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_power_of_2_shl({bits}, {pow}) = {}",
            (&p).mod_power_of_2_shl(bits, pow)
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_shl_assign<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    UnsignedPolynomial<T>: ModPowerOf2ShlAssign<U>,
{
    for (mut p, bits, pow) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_3::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.mod_power_of_2_shl_assign(bits, pow);
        println!("p := {p_old}; p.mod_power_of_2_shl_assign({bits}, {pow}); p = {p}");
    }
}

fn benchmark_unsigned_polynomial_mod_power_of_2_shl_evaluation_strategy<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    UnsignedPolynomial<T>:
        ModPowerOf2Shl<U, Output = UnsignedPolynomial<T>> + ModPowerOf2ShlAssign<U>,
    for<'a> &'a UnsignedPolynomial<T>: ModPowerOf2Shl<U, Output = UnsignedPolynomial<T>>,
{
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_power_of_2_shl({}, u64)",
            T::NAME,
            U::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_unsigned_triple_gen_var_3::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
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

fn benchmark_unsigned_polynomial_mod_power_of_2_shl_algorithms<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    for<'a> &'a UnsignedPolynomial<T>: ModPowerOf2Shl<U, Output = UnsignedPolynomial<T>>,
{
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_power_of_2_shl({}, u64)",
            T::NAME,
            U::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_unsigned_triple_gen_var_3::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
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
