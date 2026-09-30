// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::ops::MulAssign;
use malachite_base::num::basic::traits::Zero;
use malachite_base::test_util::bench::bucketers::pair_sum_vec_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::arithmetic::mul::classical::mul_to_out_classical;
use malachite_nz::integer_polynomial::arithmetic::mul::karatsuba::mul_to_out_karatsuba;
use malachite_nz::integer_polynomial::arithmetic::mul::kronecker::mul_to_out_kronecker;
use malachite_nz::integer_polynomial::arithmetic::mul::mul_greater_to_out;
use malachite_nz::integer_polynomial::arithmetic::mul::tiny::{
    mul_to_out_tiny_1, mul_to_out_tiny_2,
};
use malachite_nz::test_util::bench::bucketers::pair_integer_polynomial_max_bit_bucketer;
use malachite_nz::test_util::generators::{
    integer_polynomial_pair_gen, integer_vec_pair_gen_var_1, integer_vec_pair_gen_var_2,
    integer_vec_pair_gen_var_3,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::mul::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_mul_to_out_classical);
    register_demo!(runner, demo_mul_to_out_tiny_1);
    register_demo!(runner, demo_mul_to_out_tiny_2);
    register_demo!(runner, demo_mul_greater_to_out);
    register_demo!(runner, demo_integer_polynomial_mul);
    register_demo!(runner, demo_integer_polynomial_mul_val_ref);
    register_demo!(runner, demo_integer_polynomial_mul_ref_val);
    register_demo!(runner, demo_integer_polynomial_mul_ref_ref);
    register_demo!(runner, demo_integer_polynomial_mul_assign);
    register_demo!(runner, demo_integer_polynomial_mul_assign_ref);
    register_demo!(runner, demo_mul_to_out_karatsuba);
    register_demo!(runner, demo_mul_to_out_kronecker);

    register_bench!(runner, benchmark_mul_to_out_algorithms);
    register_bench!(runner, benchmark_mul_to_out_tiny_1_algorithms);
    register_bench!(runner, benchmark_mul_to_out_tiny_2_algorithms);
    register_bench!(runner, benchmark_integer_polynomial_mul_evaluation_strategy);
    register_bench!(runner, benchmark_integer_polynomial_mul_algorithms);
    register_bench!(
        runner,
        benchmark_integer_polynomial_mul_assign_evaluation_strategy
    );
}

fn demo_mul_to_out_classical(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys) in integer_vec_pair_gen_var_1().get(gm, config).take(limit) {
        let mut out = vec![Integer::ZERO; xs.len() + ys.len() - 1];
        mul_to_out_classical(&mut out, &xs, &ys);
        println!("mul_to_out_classical(_, {xs:?}, {ys:?}) = {out:?}");
    }
}

fn demo_mul_to_out_tiny_1(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys) in integer_vec_pair_gen_var_2().get(gm, config).take(limit) {
        let mut out = vec![Integer::ZERO; xs.len() + ys.len() - 1];
        mul_to_out_tiny_1(&mut out, &xs, &ys);
        println!("mul_to_out_tiny_1(_, {xs:?}, {ys:?}) = {out:?}");
    }
}

fn demo_mul_to_out_tiny_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys) in integer_vec_pair_gen_var_3().get(gm, config).take(limit) {
        let mut out = vec![Integer::ZERO; xs.len() + ys.len() - 1];
        mul_to_out_tiny_2(&mut out, &xs, &ys);
        println!("mul_to_out_tiny_2(_, {xs:?}, {ys:?}) = {out:?}");
    }
}

fn demo_mul_greater_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys) in integer_vec_pair_gen_var_1().get(gm, config).take(limit) {
        let (xs, ys) = if xs.len() >= ys.len() {
            (xs, ys)
        } else {
            (ys, xs)
        };
        let mut out = vec![Integer::ZERO; xs.len() + ys.len() - 1];
        mul_greater_to_out(&mut out, &xs, &ys);
        println!("mul_greater_to_out(_, {xs:?}, {ys:?}) = {out:?}");
    }
}

fn demo_integer_polynomial_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        let q_old = q.clone();
        println!("({p_old}) * ({q_old}) = {}", p * q);
    }
}

fn demo_integer_polynomial_mul_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        println!("({p_old}) * &({q}) = {}", p * &q);
    }
}

fn demo_integer_polynomial_mul_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen().get(gm, config).take(limit) {
        let q_old = q.clone();
        println!("&({p}) * ({q_old}) = {}", &p * q);
    }
}

fn demo_integer_polynomial_mul_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen().get(gm, config).take(limit) {
        println!("&({p}) * &({q}) = {}", &p * &q);
    }
}

fn demo_integer_polynomial_mul_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q) in integer_polynomial_pair_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        let q_old = q.clone();
        p *= q;
        println!("p := {p_old}; p *= {q_old}; p = {p}");
    }
}

fn demo_integer_polynomial_mul_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q) in integer_polynomial_pair_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        p *= &q;
        println!("p := {p_old}; p *= &{q}; p = {p}");
    }
}

fn benchmark_mul_to_out_algorithms(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "mul_to_out(&mut [Integer], &[Integer], &[Integer])",
        BenchmarkType::Algorithms,
        integer_vec_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_sum_vec_len_bucketer("xs", "ys"),
        &mut [
            ("classical", &mut |(xs, ys)| {
                let mut out = vec![Integer::ZERO; xs.len() + ys.len() - 1];
                mul_to_out_classical(&mut out, &xs, &ys);
            }),
            ("default", &mut |(xs, ys)| {
                let mut out = vec![Integer::ZERO; xs.len() + ys.len() - 1];
                if xs.len() >= ys.len() {
                    mul_greater_to_out(&mut out, &xs, &ys);
                } else {
                    mul_greater_to_out(&mut out, &ys, &xs);
                }
            }),
            ("naive", &mut |(xs, ys)| {
                let mut out = vec![Integer::ZERO; xs.len() + ys.len() - 1];
                out.clone_from_slice(&integers_mul_naive(&xs, &ys));
            }),
            ("Karatsuba", &mut |(xs, ys)| {
                let mut out = vec![Integer::ZERO; xs.len() + ys.len() - 1];
                if xs.len() >= ys.len() {
                    mul_to_out_karatsuba(&mut out, &xs, &ys);
                } else {
                    mul_to_out_karatsuba(&mut out, &ys, &xs);
                }
            }),
            ("Kronecker", &mut |(xs, ys)| {
                let mut out = vec![Integer::ZERO; xs.len() + ys.len() - 1];
                mul_to_out_kronecker(&mut out, &xs, &ys);
            }),
        ],
    );
}

fn benchmark_mul_to_out_tiny_1_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "mul_to_out_tiny_1(&mut [Integer], &[Integer], &[Integer])",
        BenchmarkType::Algorithms,
        integer_vec_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_sum_vec_len_bucketer("xs", "ys"),
        &mut [
            ("tiny 1", &mut |(xs, ys)| {
                let mut out = vec![Integer::ZERO; xs.len() + ys.len() - 1];
                mul_to_out_tiny_1(&mut out, &xs, &ys);
            }),
            ("tiny 2", &mut |(xs, ys)| {
                let mut out = vec![Integer::ZERO; xs.len() + ys.len() - 1];
                mul_to_out_tiny_2(&mut out, &xs, &ys);
            }),
            ("classical", &mut |(xs, ys)| {
                let mut out = vec![Integer::ZERO; xs.len() + ys.len() - 1];
                mul_to_out_classical(&mut out, &xs, &ys);
            }),
            ("naive", &mut |(xs, ys)| {
                let mut out = vec![Integer::ZERO; xs.len() + ys.len() - 1];
                out.clone_from_slice(&integers_mul_naive(&xs, &ys));
            }),
        ],
    );
}

fn benchmark_mul_to_out_tiny_2_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "mul_to_out_tiny_2(&mut [Integer], &[Integer], &[Integer])",
        BenchmarkType::Algorithms,
        integer_vec_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_sum_vec_len_bucketer("xs", "ys"),
        &mut [
            ("tiny 2", &mut |(xs, ys)| {
                let mut out = vec![Integer::ZERO; xs.len() + ys.len() - 1];
                mul_to_out_tiny_2(&mut out, &xs, &ys);
            }),
            ("classical", &mut |(xs, ys)| {
                let mut out = vec![Integer::ZERO; xs.len() + ys.len() - 1];
                mul_to_out_classical(&mut out, &xs, &ys);
            }),
            ("naive", &mut |(xs, ys)| {
                let mut out = vec![Integer::ZERO; xs.len() + ys.len() - 1];
                out.clone_from_slice(&integers_mul_naive(&xs, &ys));
            }),
        ],
    );
}

fn benchmark_integer_polynomial_mul_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial * IntegerPolynomial",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("IntegerPolynomial * IntegerPolynomial", &mut |(p, q)| {
                no_out!(p * q);
            }),
            ("IntegerPolynomial * &IntegerPolynomial", &mut |(p, q)| {
                no_out!(p * &q);
            }),
            ("&IntegerPolynomial * IntegerPolynomial", &mut |(p, q)| {
                no_out!(&p * q);
            }),
            ("&IntegerPolynomial * &IntegerPolynomial", &mut |(p, q)| {
                no_out!(&p * &q);
            }),
        ],
    );
}

fn benchmark_integer_polynomial_mul_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial * IntegerPolynomial",
        BenchmarkType::Algorithms,
        integer_polynomial_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q)| {
                no_out!(p * q);
            }),
            ("naive", &mut |(p, q)| {
                no_out!(mul_naive(&p, &q));
            }),
        ],
    );
}

fn benchmark_integer_polynomial_mul_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial *= IntegerPolynomial",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            (
                "IntegerPolynomial *= IntegerPolynomial",
                &mut |(mut p, q)| p.mul_assign(q),
            ),
            (
                "IntegerPolynomial *= &IntegerPolynomial",
                &mut |(mut p, q)| p.mul_assign(&q),
            ),
        ],
    );
}

fn demo_mul_to_out_karatsuba(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys) in integer_vec_pair_gen_var_1().get(gm, config).take(limit) {
        let (xs, ys) = if xs.len() >= ys.len() {
            (xs, ys)
        } else {
            (ys, xs)
        };
        let mut out = vec![Integer::ZERO; xs.len() + ys.len() - 1];
        mul_to_out_karatsuba(&mut out, &xs, &ys);
        println!("mul_to_out_karatsuba(_, {xs:?}, {ys:?}) = {out:?}");
    }
}

fn demo_mul_to_out_kronecker(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys) in integer_vec_pair_gen_var_1().get(gm, config).take(limit) {
        let mut out = vec![Integer::ZERO; xs.len() + ys.len() - 1];
        mul_to_out_kronecker(&mut out, &xs, &ys);
        println!("mul_to_out_kronecker(_, {xs:?}, {ys:?}) = {out:?}");
    }
}
