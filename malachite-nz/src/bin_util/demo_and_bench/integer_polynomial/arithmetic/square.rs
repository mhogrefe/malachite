// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Square, SquareAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::test_util::bench::bucketers::vec_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::arithmetic::mul::classical::mul_to_out_classical;
use malachite_nz::integer_polynomial::arithmetic::square::classical::square_to_out_classical;
use malachite_nz::integer_polynomial::arithmetic::square::karatsuba::square_to_out_karatsuba;
use malachite_nz::integer_polynomial::arithmetic::square::kronecker::square_to_out_kronecker;
use malachite_nz::integer_polynomial::arithmetic::square::schonhage_strassen::*;
use malachite_nz::integer_polynomial::arithmetic::square::square_to_out;
use malachite_nz::integer_polynomial::arithmetic::square::tiny::{
    square_to_out_tiny_1, square_to_out_tiny_2,
};
use malachite_nz::test_util::bench::bucketers::integer_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::{
    integer_polynomial_gen, integer_vec_gen_var_1, integer_vec_gen_var_2, integer_vec_gen_var_3,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::mul::*;
use malachite_nz::test_util::integer_polynomial::arithmetic::square::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_square_to_out_classical);
    register_demo!(runner, demo_square_to_out_tiny_1);
    register_demo!(runner, demo_square_to_out_tiny_2);
    register_demo!(runner, demo_square_to_out);
    register_demo!(runner, demo_integer_polynomial_square);
    register_demo!(runner, demo_integer_polynomial_square_ref);
    register_demo!(runner, demo_integer_polynomial_square_assign);
    register_demo!(runner, demo_square_to_out_karatsuba);
    register_demo!(runner, demo_square_to_out_kronecker);
    register_demo!(runner, demo_square_to_out_schonhage_strassen);

    register_bench!(runner, benchmark_square_to_out_algorithms);
    register_bench!(runner, benchmark_square_to_out_tiny_1_algorithms);
    register_bench!(runner, benchmark_square_to_out_tiny_2_algorithms);
    register_bench!(
        runner,
        benchmark_integer_polynomial_square_evaluation_strategy
    );
    register_bench!(runner, benchmark_integer_polynomial_square_algorithms);
}

fn demo_square_to_out_classical(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in integer_vec_gen_var_1().get(gm, config).take(limit) {
        let mut out = vec![Integer::ZERO; (xs.len() << 1) - 1];
        square_to_out_classical(&mut out, &xs);
        println!("square_to_out_classical(_, {xs:?}) = {out:?}");
    }
}

fn demo_square_to_out_tiny_1(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in integer_vec_gen_var_2().get(gm, config).take(limit) {
        let mut out = vec![Integer::ZERO; (xs.len() << 1) - 1];
        square_to_out_tiny_1(&mut out, &xs);
        println!("square_to_out_tiny_1(_, {xs:?}) = {out:?}");
    }
}

fn demo_square_to_out_tiny_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in integer_vec_gen_var_3().get(gm, config).take(limit) {
        let mut out = vec![Integer::ZERO; (xs.len() << 1) - 1];
        square_to_out_tiny_2(&mut out, &xs);
        println!("square_to_out_tiny_2(_, {xs:?}) = {out:?}");
    }
}

fn demo_square_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in integer_vec_gen_var_1().get(gm, config).take(limit) {
        let mut out = vec![Integer::ZERO; (xs.len() << 1) - 1];
        square_to_out(&mut out, &xs);
        println!("square_to_out(_, {xs:?}) = {out:?}");
    }
}

fn demo_integer_polynomial_square(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in integer_polynomial_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        println!("({p_old}).square() = {}", p.square());
    }
}

fn demo_integer_polynomial_square_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in integer_polynomial_gen().get(gm, config).take(limit) {
        println!("(&({p})).square() = {}", (&p).square());
    }
}

fn demo_integer_polynomial_square_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut p in integer_polynomial_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        p.square_assign();
        println!("p := {p_old}; p.square_assign(); p = {p}");
    }
}

fn benchmark_square_to_out_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "square_to_out(&mut [Integer], &[Integer])",
        BenchmarkType::Algorithms,
        integer_vec_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [
            ("classical", &mut |xs| {
                let mut out = vec![Integer::ZERO; (xs.len() << 1) - 1];
                square_to_out_classical(&mut out, &xs);
            }),
            ("default", &mut |xs| {
                let mut out = vec![Integer::ZERO; (xs.len() << 1) - 1];
                square_to_out(&mut out, &xs);
            }),
            ("mul classical", &mut |xs| {
                let mut out = vec![Integer::ZERO; (xs.len() << 1) - 1];
                mul_to_out_classical(&mut out, &xs, &xs.clone());
            }),
            ("naive", &mut |xs| {
                let mut out = vec![Integer::ZERO; (xs.len() << 1) - 1];
                out.clone_from_slice(&integers_mul_naive(&xs, &xs));
            }),
            ("Karatsuba", &mut |xs| {
                let mut out = vec![Integer::ZERO; (xs.len() << 1) - 1];
                square_to_out_karatsuba(&mut out, &xs);
            }),
            ("Kronecker", &mut |xs| {
                let mut out = vec![Integer::ZERO; (xs.len() << 1) - 1];
                square_to_out_kronecker(&mut out, &xs);
            }),
            ("Schönhage-Strassen", &mut |xs| {
                let mut out = vec![Integer::ZERO; (xs.len() << 1) - 1];
                square_to_out_schonhage_strassen(&mut out, &xs);
            }),
        ],
    );
}

fn benchmark_square_to_out_tiny_1_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "square_to_out_tiny_1(&mut [Integer], &[Integer])",
        BenchmarkType::Algorithms,
        integer_vec_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [
            ("tiny 1", &mut |xs| {
                let mut out = vec![Integer::ZERO; (xs.len() << 1) - 1];
                square_to_out_tiny_1(&mut out, &xs);
            }),
            ("tiny 2", &mut |xs| {
                let mut out = vec![Integer::ZERO; (xs.len() << 1) - 1];
                square_to_out_tiny_2(&mut out, &xs);
            }),
            ("classical", &mut |xs| {
                let mut out = vec![Integer::ZERO; (xs.len() << 1) - 1];
                square_to_out_classical(&mut out, &xs);
            }),
            ("naive", &mut |xs| {
                let mut out = vec![Integer::ZERO; (xs.len() << 1) - 1];
                out.clone_from_slice(&integers_mul_naive(&xs, &xs));
            }),
        ],
    );
}

fn benchmark_square_to_out_tiny_2_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "square_to_out_tiny_2(&mut [Integer], &[Integer])",
        BenchmarkType::Algorithms,
        integer_vec_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [
            ("tiny 2", &mut |xs| {
                let mut out = vec![Integer::ZERO; (xs.len() << 1) - 1];
                square_to_out_tiny_2(&mut out, &xs);
            }),
            ("classical", &mut |xs| {
                let mut out = vec![Integer::ZERO; (xs.len() << 1) - 1];
                square_to_out_classical(&mut out, &xs);
            }),
            ("naive", &mut |xs| {
                let mut out = vec![Integer::ZERO; (xs.len() << 1) - 1];
                out.clone_from_slice(&integers_mul_naive(&xs, &xs));
            }),
        ],
    );
}

fn benchmark_integer_polynomial_square_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.square()",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_polynomial_bit_bucketer("p"),
        &mut [
            ("IntegerPolynomial.square()", &mut |p| {
                no_out!(p.square());
            }),
            ("(&IntegerPolynomial).square()", &mut |p| {
                no_out!((&p).square());
            }),
        ],
    );
}

fn benchmark_integer_polynomial_square_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.square()",
        BenchmarkType::Algorithms,
        integer_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |p| {
                no_out!(p.square());
            }),
            ("using *", &mut |p| {
                no_out!(&p * &p);
            }),
            ("naive", &mut |p| {
                no_out!(square_naive(&p));
            }),
        ],
    );
}

fn demo_square_to_out_karatsuba(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in integer_vec_gen_var_1().get(gm, config).take(limit) {
        let mut out = vec![Integer::ZERO; (xs.len() << 1) - 1];
        square_to_out_karatsuba(&mut out, &xs);
        println!("square_to_out_karatsuba(_, {xs:?}) = {out:?}");
    }
}

fn demo_square_to_out_kronecker(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in integer_vec_gen_var_1().get(gm, config).take(limit) {
        let mut out = vec![Integer::ZERO; (xs.len() << 1) - 1];
        square_to_out_kronecker(&mut out, &xs);
        println!("square_to_out_kronecker(_, {xs:?}) = {out:?}");
    }
}

fn demo_square_to_out_schonhage_strassen(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in integer_vec_gen_var_1().get(gm, config).take(limit) {
        let mut out = vec![Integer::ZERO; (xs.len() << 1) - 1];
        square_to_out_schonhage_strassen(&mut out, &xs);
        println!("square_to_out_schonhage_strassen(_, {xs:?}) = {out:?}");
    }
}
