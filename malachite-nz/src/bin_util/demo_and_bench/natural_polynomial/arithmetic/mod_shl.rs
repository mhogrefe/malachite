// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModShl, ModShlAssign};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::natural_polynomial_unsigned_natural_triple_gen_var_1;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_shl::*;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_natural_polynomial_mod_shl);
    register_unsigned_demos!(runner, demo_natural_polynomial_mod_shl_ref);
    register_unsigned_demos!(runner, demo_natural_polynomial_mod_shl_assign);

    register_unsigned_benches!(
        runner,
        benchmark_natural_polynomial_mod_shl_evaluation_strategy
    );
    register_unsigned_benches!(runner, benchmark_natural_polynomial_mod_shl_algorithms);
}

fn demo_natural_polynomial_mod_shl<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    NaturalPolynomial: ModShl<T, Natural, Output = NaturalPolynomial>,
{
    for (p, bits, m) in natural_polynomial_unsigned_natural_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let m_old = m.clone();
        println!(
            "({p_old}).mod_shl({bits}, {m_old}) = {}",
            p.mod_shl(bits, m)
        );
    }
}

fn demo_natural_polynomial_mod_shl_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a NaturalPolynomial: ModShl<T, Natural, Output = NaturalPolynomial>,
{
    for (p, bits, m) in natural_polynomial_unsigned_natural_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let m_old = m.clone();
        println!(
            "(&({p})).mod_shl({bits}, {m_old}) = {}",
            (&p).mod_shl(bits, m)
        );
    }
}

fn demo_natural_polynomial_mod_shl_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    NaturalPolynomial: ModShlAssign<T, Natural>,
{
    for (mut p, bits, m) in natural_polynomial_unsigned_natural_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let m_old = m.clone();
        p.mod_shl_assign(bits, m);
        println!("p := {p_old}; p.mod_shl_assign({bits}, {m_old}); p = {p}");
    }
}

fn benchmark_natural_polynomial_mod_shl_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    NaturalPolynomial: ModShl<T, Natural, Output = NaturalPolynomial> + ModShlAssign<T, Natural>,
    for<'a> &'a NaturalPolynomial: ModShl<T, Natural, Output = NaturalPolynomial>,
{
    run_benchmark(
        &format!("NaturalPolynomial.mod_shl({}, Natural)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_unsigned_natural_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_polynomial_bit_bucketer("p"),
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

fn benchmark_natural_polynomial_mod_shl_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    for<'a> &'a NaturalPolynomial: ModShl<T, Natural, Output = NaturalPolynomial>,
    u64: ExactFrom<T>,
{
    run_benchmark(
        &format!("NaturalPolynomial.mod_shl({}, Natural)", T::NAME),
        BenchmarkType::Algorithms,
        natural_polynomial_unsigned_natural_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, bits, m)| {
                no_out!((&p).mod_shl(bits, m));
            }),
            ("naive", &mut |(p, bits, m)| {
                no_out!(mod_shl_naive(&p, bits, &m));
            }),
        ],
    );
}
