// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{MulTruncated, MulTruncatedAssign};
use malachite_base::test_util::bench::bucketers::triple_1_2_vec_max_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::arithmetic::mul_truncated::classical::*;
use malachite_nz::integer_polynomial::arithmetic::mul_truncated::karatsuba::*;
use malachite_nz::integer_polynomial::arithmetic::mul_truncated::kronecker::*;
use malachite_nz::integer_polynomial::arithmetic::mul_truncated::mul_truncated_to_out;
use malachite_nz::integer_polynomial::arithmetic::mul_truncated::schonhage_strassen::*;
use malachite_nz::integer_polynomial::arithmetic::mul_truncated::tiny::{
    mul_truncated_to_out_tiny_1, mul_truncated_to_out_tiny_2,
};
use malachite_nz::test_util::bench::bucketers::triple_1_2_integer_polynomial_max_bit_bucketer;
use malachite_nz::test_util::generators::{
    integer_polynomial_integer_polynomial_unsigned_triple_gen_var_1,
    integer_vec_integer_vec_unsigned_triple_gen_var_1,
    integer_vec_integer_vec_unsigned_triple_gen_var_2,
    integer_vec_integer_vec_unsigned_triple_gen_var_3,
    integer_vec_integer_vec_unsigned_triple_gen_var_5,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::mul::*;
use malachite_nz::test_util::integer_polynomial::arithmetic::mul_truncated::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_mul_truncated_to_out_classical);
    register_demo!(runner, demo_mul_truncated_to_out_tiny_1);
    register_demo!(runner, demo_mul_truncated_to_out_tiny_2);
    register_demo!(runner, demo_mul_truncated_to_out);
    register_demo!(runner, demo_integer_polynomial_mul_truncated);
    register_demo!(runner, demo_integer_polynomial_mul_truncated_val_ref);
    register_demo!(runner, demo_integer_polynomial_mul_truncated_ref_val);
    register_demo!(runner, demo_integer_polynomial_mul_truncated_ref_ref);
    register_demo!(runner, demo_integer_polynomial_mul_truncated_assign);
    register_demo!(runner, demo_integer_polynomial_mul_truncated_assign_ref);
    register_demo!(runner, demo_mul_truncated_to_out_karatsuba_n);
    register_demo!(runner, demo_mul_truncated_to_out_karatsuba);
    register_demo!(runner, demo_mul_truncated_to_out_kronecker);
    register_demo!(runner, demo_mul_truncated_to_out_schonhage_strassen);

    register_bench!(runner, benchmark_mul_truncated_to_out_algorithms);
    register_bench!(runner, benchmark_mul_truncated_to_out_tiny_1_algorithms);
    register_bench!(runner, benchmark_mul_truncated_to_out_tiny_2_algorithms);
    register_bench!(
        runner,
        benchmark_integer_polynomial_mul_truncated_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_integer_polynomial_mul_truncated_algorithms
    );
    register_bench!(
        runner,
        benchmark_integer_polynomial_mul_truncated_assign_evaluation_strategy
    );
}

fn demo_mul_truncated_to_out_classical(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, n) in integer_vec_integer_vec_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut out = vec![Integer::ZERO; usize::exact_from(n)];
        mul_truncated_to_out_classical(&mut out, &xs, &ys);
        println!("mul_truncated_to_out_classical(_, {xs:?}, {ys:?}) = {out:?}");
    }
}

fn demo_mul_truncated_to_out_tiny_1(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, n) in integer_vec_integer_vec_unsigned_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut out = vec![Integer::ZERO; usize::exact_from(n)];
        mul_truncated_to_out_tiny_1(&mut out, &xs, &ys);
        println!("mul_truncated_to_out_tiny_1(_, {xs:?}, {ys:?}) = {out:?}");
    }
}

fn demo_mul_truncated_to_out_tiny_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, n) in integer_vec_integer_vec_unsigned_triple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let mut out = vec![Integer::ZERO; usize::exact_from(n)];
        mul_truncated_to_out_tiny_2(&mut out, &xs, &ys);
        println!("mul_truncated_to_out_tiny_2(_, {xs:?}, {ys:?}) = {out:?}");
    }
}

fn demo_mul_truncated_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, n) in integer_vec_integer_vec_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut out = vec![Integer::ZERO; usize::exact_from(n)];
        mul_truncated_to_out(&mut out, &xs, &ys);
        println!("mul_truncated_to_out(_, {xs:?}, {ys:?}) = {out:?}");
    }
}

fn demo_integer_polynomial_mul_truncated(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, len) in integer_polynomial_integer_polynomial_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "({p_old}).mul_truncated({q_old}, {len}) = {}",
            p.mul_truncated(q, len)
        );
    }
}

fn demo_integer_polynomial_mul_truncated_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, len) in integer_polynomial_integer_polynomial_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).mul_truncated(&({q}), {len}) = {}",
            p.mul_truncated(&q, len)
        );
    }
}

fn demo_integer_polynomial_mul_truncated_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, len) in integer_polynomial_integer_polynomial_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let q_old = q.clone();
        println!(
            "(&({p})).mul_truncated({q_old}, {len}) = {}",
            (&p).mul_truncated(q, len)
        );
    }
}

fn demo_integer_polynomial_mul_truncated_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, len) in integer_polynomial_integer_polynomial_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mul_truncated(&({q}), {len}) = {}",
            (&p).mul_truncated(&q, len)
        );
    }
}

fn demo_integer_polynomial_mul_truncated_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q, len) in integer_polynomial_integer_polynomial_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        p.mul_truncated_assign(q, len);
        println!("p := {p_old}; p.mul_truncated_assign({q_old}, {len}); p = {p}");
    }
}

fn demo_integer_polynomial_mul_truncated_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q, len) in integer_polynomial_integer_polynomial_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.mul_truncated_assign(&q, len);
        println!("p := {p_old}; p.mul_truncated_assign(&({q}), {len}); p = {p}");
    }
}

fn benchmark_mul_truncated_to_out_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "mul_truncated_to_out(&mut [Integer], &[Integer], &[Integer])",
        BenchmarkType::Algorithms,
        integer_vec_integer_vec_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_vec_max_len_bucketer("xs", "ys"),
        &mut [
            ("classical", &mut |(xs, ys, n)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(n)];
                mul_truncated_to_out_classical(&mut out, &xs, &ys);
            }),
            ("default", &mut |(xs, ys, n)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(n)];
                mul_truncated_to_out(&mut out, &xs, &ys);
            }),
            ("naive", &mut |(xs, ys, n)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(n)];
                out.clone_from_slice(&integers_mul_naive(&xs, &ys)[..usize::exact_from(n)]);
            }),
            ("Karatsuba", &mut |(xs, ys, n)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(n)];
                mul_truncated_to_out_karatsuba(&mut out, &xs, &ys);
            }),
            ("Kronecker", &mut |(xs, ys, n)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(n)];
                mul_truncated_to_out_kronecker(&mut out, &xs, &ys);
            }),
            ("Schönhage-Strassen", &mut |(xs, ys, n)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(n)];
                mul_truncated_to_out_schonhage_strassen(&mut out, &xs, &ys);
            }),
        ],
    );
}

fn benchmark_mul_truncated_to_out_tiny_1_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "mul_truncated_to_out_tiny_1(&mut [Integer], &[Integer], &[Integer])",
        BenchmarkType::Algorithms,
        integer_vec_integer_vec_unsigned_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_vec_max_len_bucketer("xs", "ys"),
        &mut [
            ("tiny 1", &mut |(xs, ys, n)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(n)];
                mul_truncated_to_out_tiny_1(&mut out, &xs, &ys);
            }),
            ("tiny 2", &mut |(xs, ys, n)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(n)];
                mul_truncated_to_out_tiny_2(&mut out, &xs, &ys);
            }),
            ("classical", &mut |(xs, ys, n)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(n)];
                mul_truncated_to_out_classical(&mut out, &xs, &ys);
            }),
            ("naive", &mut |(xs, ys, n)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(n)];
                out.clone_from_slice(&integers_mul_naive(&xs, &ys)[..usize::exact_from(n)]);
            }),
        ],
    );
}

fn benchmark_mul_truncated_to_out_tiny_2_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "mul_truncated_to_out_tiny_2(&mut [Integer], &[Integer], &[Integer])",
        BenchmarkType::Algorithms,
        integer_vec_integer_vec_unsigned_triple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_vec_max_len_bucketer("xs", "ys"),
        &mut [
            ("tiny 2", &mut |(xs, ys, n)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(n)];
                mul_truncated_to_out_tiny_2(&mut out, &xs, &ys);
            }),
            ("classical", &mut |(xs, ys, n)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(n)];
                mul_truncated_to_out_classical(&mut out, &xs, &ys);
            }),
            ("naive", &mut |(xs, ys, n)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(n)];
                out.clone_from_slice(&integers_mul_naive(&xs, &ys)[..usize::exact_from(n)]);
            }),
        ],
    );
}

fn benchmark_integer_polynomial_mul_truncated_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.mul_truncated(IntegerPolynomial, u64)",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_integer_polynomial_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_integer_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            (
                "IntegerPolynomial.mul_truncated(IntegerPolynomial, u64)",
                &mut |(p, q, len)| {
                    no_out!(p.mul_truncated(q, len));
                },
            ),
            (
                "IntegerPolynomial.mul_truncated(&IntegerPolynomial, u64)",
                &mut |(p, q, len)| {
                    no_out!(p.mul_truncated(&q, len));
                },
            ),
            (
                "(&IntegerPolynomial).mul_truncated(IntegerPolynomial, u64)",
                &mut |(p, q, len)| {
                    no_out!((&p).mul_truncated(q, len));
                },
            ),
            (
                "(&IntegerPolynomial).mul_truncated(&IntegerPolynomial, u64)",
                &mut |(p, q, len)| {
                    no_out!((&p).mul_truncated(&q, len));
                },
            ),
        ],
    );
}

fn benchmark_integer_polynomial_mul_truncated_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.mul_truncated(IntegerPolynomial, u64)",
        BenchmarkType::Algorithms,
        integer_polynomial_integer_polynomial_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_integer_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q, len)| {
                no_out!(p.mul_truncated(q, len));
            }),
            ("naive", &mut |(p, q, len)| {
                no_out!(mul_truncated_naive(&p, &q, len));
            }),
        ],
    );
}

fn benchmark_integer_polynomial_mul_truncated_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.mul_truncated_assign(IntegerPolynomial, u64)",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_integer_polynomial_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_integer_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            (
                "IntegerPolynomial.mul_truncated_assign(IntegerPolynomial, u64)",
                &mut |(mut p, q, len)| p.mul_truncated_assign(q, len),
            ),
            (
                "IntegerPolynomial.mul_truncated_assign(&IntegerPolynomial, u64)",
                &mut |(mut p, q, len)| p.mul_truncated_assign(&q, len),
            ),
        ],
    );
}

fn demo_mul_truncated_to_out_karatsuba_n(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, n) in integer_vec_integer_vec_unsigned_triple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let mut out = vec![Integer::ZERO; usize::exact_from(n)];
        mul_truncated_to_out_karatsuba_n(&mut out, &xs, &ys);
        println!("mul_truncated_to_out_karatsuba_n(_, {xs:?}, {ys:?}) = {out:?}");
    }
}

fn demo_mul_truncated_to_out_karatsuba(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, n) in integer_vec_integer_vec_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut out = vec![Integer::ZERO; usize::exact_from(n)];
        mul_truncated_to_out_karatsuba(&mut out, &xs, &ys);
        println!("mul_truncated_to_out_karatsuba(_, {xs:?}, {ys:?}) = {out:?}");
    }
}

fn demo_mul_truncated_to_out_kronecker(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, n) in integer_vec_integer_vec_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut out = vec![Integer::ZERO; usize::exact_from(n)];
        mul_truncated_to_out_kronecker(&mut out, &xs, &ys);
        println!("mul_truncated_to_out_kronecker(_, {xs:?}, {ys:?}) = {out:?}");
    }
}

fn demo_mul_truncated_to_out_schonhage_strassen(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, n) in integer_vec_integer_vec_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut out = vec![Integer::ZERO; usize::exact_from(n)];
        mul_truncated_to_out_schonhage_strassen(&mut out, &xs, &ys);
        println!("mul_truncated_to_out_schonhage_strassen(_, {xs:?}, {ys:?}) = {out:?}");
    }
}
