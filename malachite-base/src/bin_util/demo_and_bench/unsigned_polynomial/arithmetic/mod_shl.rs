// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModShl, ModShlAssign};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_polynomial_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_polynomial_unsigned_unsigned_triple_gen_var_4;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_shl::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_unsigned_demos!(runner, demo_unsigned_polynomial_mod_shl);
    register_unsigned_unsigned_demos!(runner, demo_unsigned_polynomial_mod_shl_ref);
    register_unsigned_unsigned_demos!(runner, demo_unsigned_polynomial_mod_shl_assign);

    register_unsigned_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_shl_evaluation_strategy
    );
    register_unsigned_unsigned_benches!(runner, benchmark_unsigned_polynomial_mod_shl_algorithms);
}

fn demo_unsigned_polynomial_mod_shl<
    T: PrimitiveUnsigned + ModShl<U, T, Output = T>,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    UnsignedPolynomial<T>: ModShl<U, T, Output = UnsignedPolynomial<T>>,
{
    for (p, bits, m) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_4::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!("({p_old}).mod_shl({bits}, {m}) = {}", p.mod_shl(bits, m));
    }
}

fn demo_unsigned_polynomial_mod_shl_ref<
    T: PrimitiveUnsigned + ModShl<U, T, Output = T>,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a UnsignedPolynomial<T>: ModShl<U, T, Output = UnsignedPolynomial<T>>,
{
    for (p, bits, m) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_4::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).mod_shl({bits}, {m}) = {}", (&p).mod_shl(bits, m));
    }
}

fn demo_unsigned_polynomial_mod_shl_assign<
    T: PrimitiveUnsigned + ModShl<U, T, Output = T>,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    UnsignedPolynomial<T>: ModShlAssign<U, T>,
{
    for (mut p, bits, m) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_4::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.mod_shl_assign(bits, m);
        println!("p := {p_old}; p.mod_shl_assign({bits}, {m}); p = {p}");
    }
}

fn benchmark_unsigned_polynomial_mod_shl_evaluation_strategy<
    T: PrimitiveUnsigned + ModShl<U, T, Output = T>,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    UnsignedPolynomial<T>: ModShl<U, T, Output = UnsignedPolynomial<T>> + ModShlAssign<U, T>,
    for<'a> &'a UnsignedPolynomial<T>: ModShl<U, T, Output = UnsignedPolynomial<T>>,
{
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_shl({}, {})",
            T::NAME,
            U::NAME,
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_unsigned_triple_gen_var_4::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            ("p.mod_shl(bits, m)", &mut |(p, bits, m)| {
                no_out!(p.mod_shl(bits, m));
            }),
            ("(&p).mod_shl(bits, m)", &mut |(p, bits, m)| {
                no_out!((&p).mod_shl(bits, m));
            }),
            ("p.mod_shl_assign(bits, m)", &mut |(mut p, bits, m)| {
                p.mod_shl_assign(bits, m);
            }),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_shl_algorithms<
    T: PrimitiveUnsigned + ModShl<U, T, Output = T>,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    for<'a> &'a UnsignedPolynomial<T>: ModShl<U, T, Output = UnsignedPolynomial<T>>,
{
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_shl({}, {})",
            T::NAME,
            U::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_unsigned_triple_gen_var_4::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            ("default", &mut |(p, bits, m)| {
                no_out!((&p).mod_shl(bits, m));
            }),
            ("naive", &mut |(p, bits, m)| {
                no_out!(mod_shl_naive(&p, bits, m));
            }),
        ],
    );
}
