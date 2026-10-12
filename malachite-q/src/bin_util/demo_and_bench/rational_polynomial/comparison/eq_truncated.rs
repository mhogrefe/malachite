// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::*;
use malachite_base::num::arithmetic::traits::{DivExact, Gcd};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::OrdProducts;
use malachite_base::polynomial::{EqTruncated, MulPowerOfX, Polynomial, slices_eq_truncated};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::bench::bucketers::{
    triple_1_2_rational_polynomial_max_bit_bucketer, triple_1_rational_polynomial_bit_bucketer,
};
use malachite_q::test_util::generators::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_polynomial_eq_truncated);
    register_unsigned_demos!(
        runner,
        demo_rational_polynomial_eq_truncated_unsigned_polynomial
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_eq_truncated_rational_polynomial
    );
    register_demo!(
        runner,
        demo_rational_polynomial_eq_truncated_natural_polynomial
    );
    register_demo!(
        runner,
        demo_natural_polynomial_eq_truncated_rational_polynomial
    );
    register_demo!(
        runner,
        demo_rational_polynomial_eq_truncated_integer_polynomial
    );
    register_demo!(
        runner,
        demo_integer_polynomial_eq_truncated_rational_polynomial
    );
    register_bench!(runner, benchmark_rational_polynomial_eq_truncated);
    register_bench!(
        runner,
        benchmark_rational_polynomial_eq_truncated_equal_prefix_algorithms
    );
    register_bench!(
        runner,
        benchmark_rational_polynomial_eq_truncated_constant_differs_algorithms
    );
    register_bench!(
        runner,
        benchmark_rational_polynomial_eq_truncated_random_algorithms
    );
    register_unsigned_benches!(
        runner,
        benchmark_rational_polynomial_eq_truncated_unsigned_polynomial
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_eq_truncated_rational_polynomial
    );
    register_bench!(
        runner,
        benchmark_rational_polynomial_eq_truncated_natural_polynomial
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_eq_truncated_rational_polynomial
    );
    register_bench!(
        runner,
        benchmark_rational_polynomial_eq_truncated_integer_polynomial
    );
    register_bench!(
        runner,
        benchmark_integer_polynomial_eq_truncated_rational_polynomial
    );
}

fn demo_rational_polynomial_eq_truncated(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, len) in rational_polynomial_rational_polynomial_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({p}).eq_truncated(&({q}), {len}) = {}",
            p.eq_truncated(&q, len)
        );
    }
}

fn demo_rational_polynomial_eq_truncated_unsigned_polynomial<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Integer: From<T> + PartialEq<T>,
{
    for (p, q, len) in rational_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({p}).eq_truncated(&({q}), {len}) = {}",
            p.eq_truncated(&q, len)
        );
    }
}

fn demo_unsigned_polynomial_eq_truncated_rational_polynomial<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Integer: From<T> + PartialEq<T>,
{
    for (p, q, len) in rational_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({q}).eq_truncated(&({p}), {len}) = {}",
            q.eq_truncated(&p, len)
        );
    }
}

fn demo_rational_polynomial_eq_truncated_natural_polynomial(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, len) in rational_polynomial_natural_polynomial_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({p}).eq_truncated(&({q}), {len}) = {}",
            p.eq_truncated(&q, len)
        );
    }
}

fn demo_natural_polynomial_eq_truncated_rational_polynomial(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, len) in rational_polynomial_natural_polynomial_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({q}).eq_truncated(&({p}), {len}) = {}",
            q.eq_truncated(&p, len)
        );
    }
}

fn demo_rational_polynomial_eq_truncated_integer_polynomial(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, len) in rational_polynomial_integer_polynomial_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({p}).eq_truncated(&({q}), {len}) = {}",
            p.eq_truncated(&q, len)
        );
    }
}

fn demo_integer_polynomial_eq_truncated_rational_polynomial(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, len) in rational_polynomial_integer_polynomial_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({q}).eq_truncated(&({p}), {len}) = {}",
            q.eq_truncated(&p, len)
        );
    }
}

fn benchmark_rational_polynomial_eq_truncated(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.eq_truncated(&RationalPolynomial, u64)",
        BenchmarkType::Single,
        rational_polynomial_rational_polynomial_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_rational_polynomial_max_bit_bucketer("p", "q"),
        &mut [("Malachite", &mut |(p, q, len)| {
            no_out!(p.eq_truncated(&q, len));
        })],
    );
}

fn benchmark_rational_polynomial_eq_truncated_unsigned_polynomial<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: From<T> + PartialEq<T>,
{
    run_benchmark(
        &format!(
            "RationalPolynomial.eq_truncated(&UnsignedPolynomial<{}>, u64)",
            T::NAME
        ),
        BenchmarkType::Single,
        rational_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_rational_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |(p, q, len)| {
            no_out!(p.eq_truncated(&q, len));
        })],
    );
}

fn benchmark_unsigned_polynomial_eq_truncated_rational_polynomial<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: From<T> + PartialEq<T>,
{
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.eq_truncated(&RationalPolynomial, u64)",
            T::NAME
        ),
        BenchmarkType::Single,
        rational_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_rational_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |(p, q, len)| {
            no_out!(q.eq_truncated(&p, len));
        })],
    );
}

fn benchmark_rational_polynomial_eq_truncated_natural_polynomial(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.eq_truncated(&NaturalPolynomial, u64)",
        BenchmarkType::Single,
        rational_polynomial_natural_polynomial_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_rational_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |(p, q, len)| {
            no_out!(p.eq_truncated(&q, len));
        })],
    );
}

fn benchmark_natural_polynomial_eq_truncated_rational_polynomial(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.eq_truncated(&RationalPolynomial, u64)",
        BenchmarkType::Single,
        rational_polynomial_natural_polynomial_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_rational_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |(p, q, len)| {
            no_out!(q.eq_truncated(&p, len));
        })],
    );
}

fn benchmark_rational_polynomial_eq_truncated_integer_polynomial(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.eq_truncated(&IntegerPolynomial, u64)",
        BenchmarkType::Single,
        rational_polynomial_integer_polynomial_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_rational_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |(p, q, len)| {
            no_out!(p.eq_truncated(&q, len));
        })],
    );
}

fn benchmark_integer_polynomial_eq_truncated_rational_polynomial(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.eq_truncated(&RationalPolynomial, u64)",
        BenchmarkType::Single,
        rational_polynomial_integer_polynomial_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_rational_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |(p, q, len)| {
            no_out!(q.eq_truncated(&p, len));
        })],
    );
}

// `eq_truncated` for two `RationalPolynomial`s, with each pair of coefficients compared through
// `Integer::cmp_products` instead of by forming both products.
fn eq_truncated_cmp_products(p: &RationalPolynomial, q: &RationalPolynomial, len: u64) -> bool {
    let xs = p.numerator_ref().coefficients_asc();
    let ys = q.numerator_ref().coefficients_asc();
    let is_zero = |x: &Integer| *x == 0u32;
    if p.denominator_ref() == q.denominator_ref() {
        return slices_eq_truncated(xs, ys, len, is_zero, is_zero, |x, y| x == y);
    }
    let gcd = p.denominator_ref().gcd(q.denominator_ref());
    let x_factor = Integer::from(q.denominator_ref().div_exact(&gcd));
    let y_factor = Integer::from(p.denominator_ref().div_exact(&gcd));
    slices_eq_truncated(xs, ys, len, is_zero, is_zero, |x, y| {
        Integer::cmp_products(x, &x_factor, y, &y_factor) == Equal
    })
}

fn eq_truncated_algorithms(
    gm: GenMode,
    limit: usize,
    file_name: &str,
    inputs: impl Iterator<Item = (RationalPolynomial, RationalPolynomial, u64)>,
) {
    run_benchmark(
        "RationalPolynomial.eq_truncated(&RationalPolynomial, u64)",
        BenchmarkType::Algorithms,
        inputs,
        gm.name(),
        limit,
        file_name,
        &triple_1_2_rational_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q, len)| {
                no_out!(p.eq_truncated(&q, len));
            }),
            ("cmp_products", &mut |(p, q, len)| {
                no_out!(eq_truncated_cmp_products(&p, &q, len));
            }),
        ],
    );
}

// Triples `(p, p + r * x^n, n)`, with `n` the length of `p`: the two agree below `x^n`, usually
// over different denominators, so every coefficient is compared and every comparison is equal.
fn benchmark_rational_polynomial_eq_truncated_equal_prefix_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    let inputs = rational_polynomial_pair_gen()
        .get(gm, config)
        .map(|(p, r)| {
            let n = p.numerator_ref().coefficients_asc().len() as u64;
            let q = &p + r.mul_power_of_x(n);
            (p, q, n)
        });
    eq_truncated_algorithms(gm, limit, file_name, inputs);
}

// Triples `(p, p + c, u64::MAX)`, with `c` the constant term of a generated polynomial: the scan
// stops at the first coefficient whenever `c` is nonzero.
fn benchmark_rational_polynomial_eq_truncated_constant_differs_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    let inputs = rational_polynomial_pair_gen()
        .get(gm, config)
        .map(|(p, r)| {
            let c = RationalPolynomial::from_coefficients_asc(vec![r.coefficient(0)]);
            let q = &p + c;
            (p, q, u64::MAX)
        });
    eq_truncated_algorithms(gm, limit, file_name, inputs);
}

// Triples `(p, q, u64::MAX)` of independent random polynomials.
fn benchmark_rational_polynomial_eq_truncated_random_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    let inputs = rational_polynomial_pair_gen()
        .get(gm, config)
        .map(|(p, q)| (p, q, u64::MAX));
    eq_truncated_algorithms(gm, limit, file_name, inputs);
}
