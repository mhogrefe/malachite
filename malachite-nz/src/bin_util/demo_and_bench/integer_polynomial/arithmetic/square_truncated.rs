// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{MulTruncated, SquareTruncated, SquareTruncatedAssign};
use malachite_base::test_util::bench::bucketers::pair_1_vec_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::arithmetic::square_truncated::classical::*;
use malachite_nz::integer_polynomial::arithmetic::square_truncated::karatsuba::*;
use malachite_nz::integer_polynomial::arithmetic::square_truncated::square_truncated_to_out;
use malachite_nz::integer_polynomial::arithmetic::square_truncated::tiny::{
    square_truncated_to_out_tiny_1, square_truncated_to_out_tiny_2,
};
use malachite_nz::test_util::bench::bucketers::pair_1_integer_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::{
    integer_polynomial_unsigned_pair_gen_var_3, integer_vec_unsigned_pair_gen_var_1,
    integer_vec_unsigned_pair_gen_var_2, integer_vec_unsigned_pair_gen_var_3,
    integer_vec_unsigned_pair_gen_var_4,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::mul::*;
use malachite_nz::test_util::integer_polynomial::arithmetic::square_truncated::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_square_truncated_to_out_classical);
    register_demo!(runner, demo_square_truncated_to_out_tiny_1);
    register_demo!(runner, demo_square_truncated_to_out_tiny_2);
    register_demo!(runner, demo_square_truncated_to_out);
    register_demo!(runner, demo_integer_polynomial_square_truncated);
    register_demo!(runner, demo_integer_polynomial_square_truncated_ref);
    register_demo!(runner, demo_integer_polynomial_square_truncated_assign);
    register_demo!(runner, demo_square_truncated_to_out_karatsuba_n);
    register_demo!(runner, demo_square_truncated_to_out_karatsuba);

    register_bench!(runner, benchmark_square_truncated_to_out_algorithms);
    register_bench!(runner, benchmark_square_truncated_to_out_tiny_1_algorithms);
    register_bench!(runner, benchmark_square_truncated_to_out_tiny_2_algorithms);
    register_bench!(
        runner,
        benchmark_integer_polynomial_square_truncated_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_integer_polynomial_square_truncated_algorithms
    );
}

fn demo_square_truncated_to_out_classical(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, n) in integer_vec_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut out = vec![Integer::ZERO; usize::exact_from(n)];
        square_truncated_to_out_classical(&mut out, &xs);
        println!("square_truncated_to_out_classical(_, {xs:?}) = {out:?}");
    }
}

fn demo_square_truncated_to_out_tiny_1(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, n) in integer_vec_unsigned_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut out = vec![Integer::ZERO; usize::exact_from(n)];
        square_truncated_to_out_tiny_1(&mut out, &xs);
        println!("square_truncated_to_out_tiny_1(_, {xs:?}) = {out:?}");
    }
}

fn demo_square_truncated_to_out_tiny_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, n) in integer_vec_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let mut out = vec![Integer::ZERO; usize::exact_from(n)];
        square_truncated_to_out_tiny_2(&mut out, &xs);
        println!("square_truncated_to_out_tiny_2(_, {xs:?}) = {out:?}");
    }
}

fn demo_square_truncated_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, n) in integer_vec_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut out = vec![Integer::ZERO; usize::exact_from(n)];
        square_truncated_to_out(&mut out, &xs);
        println!("square_truncated_to_out(_, {xs:?}) = {out:?}");
    }
}

fn demo_integer_polynomial_square_truncated(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, len) in integer_polynomial_unsigned_pair_gen_var_3::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).square_truncated({len}) = {}",
            p.square_truncated(len)
        );
    }
}

fn demo_integer_polynomial_square_truncated_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, len) in integer_polynomial_unsigned_pair_gen_var_3::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).square_truncated({len}) = {}",
            (&p).square_truncated(len)
        );
    }
}

fn demo_integer_polynomial_square_truncated_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, len) in integer_polynomial_unsigned_pair_gen_var_3::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.square_truncated_assign(len);
        println!("p := {p_old}; p.square_truncated_assign({len}); p = {p}");
    }
}

fn benchmark_square_truncated_to_out_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "square_truncated_to_out(&mut [Integer], &[Integer])",
        BenchmarkType::Algorithms,
        integer_vec_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [
            ("classical", &mut |(xs, n)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(n)];
                square_truncated_to_out_classical(&mut out, &xs);
            }),
            ("default", &mut |(xs, n)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(n)];
                square_truncated_to_out(&mut out, &xs);
            }),
            ("naive", &mut |(xs, n)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(n)];
                out.clone_from_slice(&integers_mul_naive(&xs, &xs)[..usize::exact_from(n)]);
            }),
            ("Karatsuba", &mut |(xs, n)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(n)];
                square_truncated_to_out_karatsuba(&mut out, &xs);
            }),
        ],
    );
}

fn benchmark_square_truncated_to_out_tiny_1_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "square_truncated_to_out_tiny_1(&mut [Integer], &[Integer])",
        BenchmarkType::Algorithms,
        integer_vec_unsigned_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [
            ("tiny 1", &mut |(xs, n)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(n)];
                square_truncated_to_out_tiny_1(&mut out, &xs);
            }),
            ("tiny 2", &mut |(xs, n)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(n)];
                square_truncated_to_out_tiny_2(&mut out, &xs);
            }),
            ("classical", &mut |(xs, n)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(n)];
                square_truncated_to_out_classical(&mut out, &xs);
            }),
            ("naive", &mut |(xs, n)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(n)];
                out.clone_from_slice(&integers_mul_naive(&xs, &xs)[..usize::exact_from(n)]);
            }),
        ],
    );
}

fn benchmark_square_truncated_to_out_tiny_2_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "square_truncated_to_out_tiny_2(&mut [Integer], &[Integer])",
        BenchmarkType::Algorithms,
        integer_vec_unsigned_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [
            ("tiny 2", &mut |(xs, n)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(n)];
                square_truncated_to_out_tiny_2(&mut out, &xs);
            }),
            ("classical", &mut |(xs, n)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(n)];
                square_truncated_to_out_classical(&mut out, &xs);
            }),
            ("naive", &mut |(xs, n)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(n)];
                out.clone_from_slice(&integers_mul_naive(&xs, &xs)[..usize::exact_from(n)]);
            }),
        ],
    );
}

fn benchmark_integer_polynomial_square_truncated_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.square_truncated(u64)",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_unsigned_pair_gen_var_3::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_polynomial_bit_bucketer("p"),
        &mut [
            (
                "IntegerPolynomial.square_truncated(u64)",
                &mut |(p, len)| {
                    no_out!(p.square_truncated(len));
                },
            ),
            (
                "(&IntegerPolynomial).square_truncated(u64)",
                &mut |(p, len)| {
                    no_out!((&p).square_truncated(len));
                },
            ),
        ],
    );
}

fn benchmark_integer_polynomial_square_truncated_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.square_truncated(u64)",
        BenchmarkType::Algorithms,
        integer_polynomial_unsigned_pair_gen_var_3::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, len)| {
                no_out!(p.square_truncated(len));
            }),
            ("using mul_truncated", &mut |(p, len)| {
                no_out!((&p).mul_truncated(&p, len));
            }),
            ("naive", &mut |(p, len)| {
                no_out!(square_truncated_naive(&p, len));
            }),
        ],
    );
}

fn demo_square_truncated_to_out_karatsuba_n(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, n) in integer_vec_unsigned_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let mut out = vec![Integer::ZERO; usize::exact_from(n)];
        square_truncated_to_out_karatsuba_n(&mut out, &xs);
        println!("square_truncated_to_out_karatsuba_n(_, {xs:?}) = {out:?}");
    }
}

fn demo_square_truncated_to_out_karatsuba(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, n) in integer_vec_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut out = vec![Integer::ZERO; usize::exact_from(n)];
        square_truncated_to_out_karatsuba(&mut out, &xs);
        println!("square_truncated_to_out_karatsuba(_, {xs:?}) = {out:?}");
    }
}
