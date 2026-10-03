// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::EqTruncated;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::bench::bucketers::{
    triple_1_2_integer_polynomial_max_bit_bucketer, triple_1_integer_polynomial_bit_bucketer,
};
use malachite_nz::test_util::generators::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_eq_truncated);
    register_unsigned_demos!(
        runner,
        demo_integer_polynomial_eq_truncated_unsigned_polynomial
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_eq_truncated_integer_polynomial
    );
    register_demo!(
        runner,
        demo_integer_polynomial_eq_truncated_natural_polynomial
    );
    register_demo!(
        runner,
        demo_natural_polynomial_eq_truncated_integer_polynomial
    );
    register_bench!(runner, benchmark_integer_polynomial_eq_truncated);
    register_unsigned_benches!(
        runner,
        benchmark_integer_polynomial_eq_truncated_unsigned_polynomial
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_eq_truncated_integer_polynomial
    );
    register_bench!(
        runner,
        benchmark_integer_polynomial_eq_truncated_natural_polynomial
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_eq_truncated_integer_polynomial
    );
}

fn demo_integer_polynomial_eq_truncated(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, len) in integer_polynomial_integer_polynomial_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({p}).eq_truncated(&({q}), {len}) = {}",
            p.eq_truncated(&q, len)
        );
    }
}

fn demo_integer_polynomial_eq_truncated_unsigned_polynomial<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Integer: PartialEq<T>,
{
    for (p, q, len) in integer_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({p}).eq_truncated(&({q}), {len}) = {}",
            p.eq_truncated(&q, len)
        );
    }
}

fn demo_unsigned_polynomial_eq_truncated_integer_polynomial<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Integer: PartialEq<T>,
{
    for (p, q, len) in integer_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({q}).eq_truncated(&({p}), {len}) = {}",
            q.eq_truncated(&p, len)
        );
    }
}

fn demo_integer_polynomial_eq_truncated_natural_polynomial(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, len) in integer_polynomial_natural_polynomial_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({p}).eq_truncated(&({q}), {len}) = {}",
            p.eq_truncated(&q, len)
        );
    }
}

fn demo_natural_polynomial_eq_truncated_integer_polynomial(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, len) in integer_polynomial_natural_polynomial_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({q}).eq_truncated(&({p}), {len}) = {}",
            q.eq_truncated(&p, len)
        );
    }
}

fn benchmark_integer_polynomial_eq_truncated(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.eq_truncated(&IntegerPolynomial, u64)",
        BenchmarkType::Single,
        integer_polynomial_integer_polynomial_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_integer_polynomial_max_bit_bucketer("p", "q"),
        &mut [("Malachite", &mut |(p, q, len)| {
            no_out!(p.eq_truncated(&q, len));
        })],
    );
}

fn benchmark_integer_polynomial_eq_truncated_unsigned_polynomial<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: PartialEq<T>,
{
    run_benchmark(
        &format!(
            "IntegerPolynomial.eq_truncated(&UnsignedPolynomial<{}>, u64)",
            T::NAME
        ),
        BenchmarkType::Single,
        integer_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_integer_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |(p, q, len)| {
            no_out!(p.eq_truncated(&q, len));
        })],
    );
}

fn benchmark_unsigned_polynomial_eq_truncated_integer_polynomial<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: PartialEq<T>,
{
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.eq_truncated(&IntegerPolynomial, u64)",
            T::NAME
        ),
        BenchmarkType::Single,
        integer_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_integer_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |(p, q, len)| {
            no_out!(q.eq_truncated(&p, len));
        })],
    );
}

fn benchmark_integer_polynomial_eq_truncated_natural_polynomial(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.eq_truncated(&NaturalPolynomial, u64)",
        BenchmarkType::Single,
        integer_polynomial_natural_polynomial_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_integer_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |(p, q, len)| {
            no_out!(p.eq_truncated(&q, len));
        })],
    );
}

fn benchmark_natural_polynomial_eq_truncated_integer_polynomial(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.eq_truncated(&IntegerPolynomial, u64)",
        BenchmarkType::Single,
        integer_polynomial_natural_polynomial_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_integer_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |(p, q, len)| {
            no_out!(q.eq_truncated(&p, len));
        })],
    );
}
