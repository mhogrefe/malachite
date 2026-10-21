// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_matrix::arithmetic::mul::classical::matrix_mul_classical;
use malachite_nz::integer_matrix::arithmetic::mul::double_word::matrix_mul_double_word;
use malachite_nz::integer_matrix::arithmetic::mul::small::matrix_mul_small;
use malachite_nz::test_util::bench::bucketers::{
    pair_natural_matrix_max_bit_bucketer, pair_natural_matrix_product_size_bucketer,
};
use malachite_nz::test_util::generators::{
    natural_matrix_pair_gen_var_4, natural_matrix_pair_gen_var_5, natural_matrix_pair_gen_var_6,
};
use malachite_nz::test_util::integer_matrix::arithmetic::mul::{
    matrix_mul_small_unblocked, matrix_mul_small_unblocked_column_reads, matrix_mul_strassen_any,
    matrix_mul_waksman_any,
};
use malachite_nz::test_util::integer_matrix::arithmetic::mul_fft::matrix_mul_fft;
use malachite_nz::test_util::natural_matrix::arithmetic::mul::{
    natural_matrix_mul_naive, natural_matrix_mul_with,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_mul);
    register_demo!(runner, demo_natural_matrix_mul_val_ref);
    register_demo!(runner, demo_natural_matrix_mul_ref_val);
    register_demo!(runner, demo_natural_matrix_mul_ref_ref);
    register_demo!(runner, demo_natural_matrix_mul_assign);
    register_demo!(runner, demo_natural_matrix_mul_assign_ref);
    register_demo!(
        runner,
        demo_natural_matrix_mul_large_dimensions_small_elements
    );
    register_demo!(
        runner,
        demo_natural_matrix_mul_large_dimensions_double_word_elements
    );
    register_demo!(runner, demo_natural_matrix_mul_large_elements);
    register_demo!(
        runner,
        demo_natural_matrix_mul_large_dimensions_and_elements
    );

    register_bench!(runner, benchmark_natural_matrix_mul_evaluation_strategy);
    register_bench!(runner, benchmark_natural_matrix_mul_algorithms);
    register_bench!(
        runner,
        benchmark_natural_matrix_mul_small_elements_algorithms
    );
    register_bench!(
        runner,
        benchmark_natural_matrix_mul_double_word_elements_algorithms
    );
    register_bench!(
        runner,
        benchmark_natural_matrix_mul_small_unblocked_algorithms
    );
    register_bench!(
        runner,
        benchmark_natural_matrix_mul_large_elements_algorithms
    );
}

fn demo_natural_matrix_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in natural_matrix_pair_gen_var_4().get(gm, config).take(limit) {
        let (a_old, b_old) = (a.clone(), b.clone());
        println!("{a_old} * {b_old} = {}", a * b);
    }
}

fn demo_natural_matrix_mul_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in natural_matrix_pair_gen_var_4().get(gm, config).take(limit) {
        let a_old = a.clone();
        println!("{a_old} * &{b} = {}", a * &b);
    }
}

fn demo_natural_matrix_mul_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in natural_matrix_pair_gen_var_4().get(gm, config).take(limit) {
        let b_old = b.clone();
        println!("&{a} * {b_old} = {}", &a * b);
    }
}

fn demo_natural_matrix_mul_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in natural_matrix_pair_gen_var_4().get(gm, config).take(limit) {
        println!("&{a} * &{b} = {}", &a * &b);
    }
}

fn demo_natural_matrix_mul_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, b) in natural_matrix_pair_gen_var_4().get(gm, config).take(limit) {
        let (a_old, b_old) = (a.clone(), b.clone());
        a *= b;
        println!("a := {a_old}; a *= {b_old}; a = {a}");
    }
}

fn demo_natural_matrix_mul_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, b) in natural_matrix_pair_gen_var_4().get(gm, config).take(limit) {
        let a_old = a.clone();
        a *= &b;
        println!("a := {a_old}; a *= &{b}; a = {a}");
    }
}

// Demos the product with the generator's means for the dimensions and the element sizes replaced,
// so that random inputs reach the large-input algorithms. Their lines are long.
fn demo_natural_matrix_mul_with_means(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    mean_dimension: u64,
    mean_bits: u64,
) {
    let mut config = config.clone();
    config.insert("mean_dimension_n", mean_dimension);
    config.insert("mean_bits_n", mean_bits);
    for (a, b) in natural_matrix_pair_gen_var_4().get(gm, &config).take(limit) {
        println!("&{a} * &{b} = {}", &a * &b);
    }
}

// Large dimensions with elements small enough that the largest usually fits in a small `fmpz`,
// which reach FLINT's blocked kernel for small elements. The element sizes are geometric, so the
// largest of many is several times the mean.
fn demo_natural_matrix_mul_large_dimensions_small_elements(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    demo_natural_matrix_mul_with_means(gm, config, limit, 40, 6);
}

// Large dimensions with elements whose largest usually fits in two words but not in a small `fmpz`,
// which reach FLINT's double-word kernel.
fn demo_natural_matrix_mul_large_dimensions_double_word_elements(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    demo_natural_matrix_mul_with_means(gm, config, limit, 40, 14);
}

// Large elements at moderate dimensions, which reach Waksman's and Strassen's algorithms.
fn demo_natural_matrix_mul_large_elements(gm: GenMode, config: &GenConfig, limit: usize) {
    demo_natural_matrix_mul_with_means(gm, config, limit, 10, 1500);
}

// Large dimensions with large elements, which reach the multimodular algorithm.
fn demo_natural_matrix_mul_large_dimensions_and_elements(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    demo_natural_matrix_mul_with_means(gm, config, limit, 40, 600);
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_natural_matrix_mul_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix * NaturalMatrix",
        BenchmarkType::EvaluationStrategy,
        natural_matrix_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_matrix_max_bit_bucketer("a", "b"),
        &mut [
            ("NaturalMatrix * NaturalMatrix", &mut |(a, b)| {
                no_out!(a * b);
            }),
            ("NaturalMatrix * &NaturalMatrix", &mut |(a, b)| {
                no_out!(a * &b);
            }),
            ("&NaturalMatrix * NaturalMatrix", &mut |(a, b)| {
                no_out!(&a * b);
            }),
            ("&NaturalMatrix * &NaturalMatrix", &mut |(a, b)| {
                no_out!(&a * &b);
            }),
            ("NaturalMatrix *= NaturalMatrix", &mut |(mut a, b)| a *= b),
            ("NaturalMatrix *= &NaturalMatrix", &mut |(mut a, b)| a *= &b),
        ],
    );
}

fn benchmark_natural_matrix_mul_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "&NaturalMatrix * &NaturalMatrix",
        BenchmarkType::Algorithms,
        natural_matrix_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_matrix_max_bit_bucketer("a", "b"),
        &mut [
            ("default", &mut |(a, b)| {
                no_out!(&a * &b);
            }),
            ("naive", &mut |(a, b)| {
                no_out!(natural_matrix_mul_naive(&a, &b));
            }),
        ],
    );
}

fn benchmark_natural_matrix_mul_small_elements_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "&NaturalMatrix * &NaturalMatrix",
        BenchmarkType::Algorithms,
        natural_matrix_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_matrix_max_bit_bucketer("a", "b"),
        &mut [
            ("default", &mut |(a, b)| {
                no_out!(&a * &b);
            }),
            ("classical", &mut |(a, b)| {
                no_out!(natural_matrix_mul_with(&a, &b, matrix_mul_classical));
            }),
            ("small", &mut |(a, b)| {
                no_out!(natural_matrix_mul_with(&a, &b, matrix_mul_small));
            }),
            ("small, unblocked", &mut |(a, b)| {
                no_out!(natural_matrix_mul_with(&a, &b, matrix_mul_small_unblocked));
            }),
            ("naive", &mut |(a, b)| {
                no_out!(natural_matrix_mul_naive(&a, &b));
            }),
        ],
    );
}

fn benchmark_natural_matrix_mul_double_word_elements_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "&NaturalMatrix * &NaturalMatrix",
        BenchmarkType::Algorithms,
        natural_matrix_pair_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_matrix_max_bit_bucketer("a", "b"),
        &mut [
            ("default", &mut |(a, b)| {
                no_out!(&a * &b);
            }),
            ("classical", &mut |(a, b)| {
                no_out!(natural_matrix_mul_with(&a, &b, matrix_mul_classical));
            }),
            ("double word", &mut |(a, b)| {
                no_out!(natural_matrix_mul_with(&a, &b, matrix_mul_double_word));
            }),
            ("naive", &mut |(a, b)| {
                no_out!(natural_matrix_mul_naive(&a, &b));
            }),
        ],
    );
}

// Compares the unblocked kernels for small elements, which read a transposed copy of the second
// matrix, with FLINT's, which read it down its columns. Both include the conversion of the second
// matrix to words.
fn benchmark_natural_matrix_mul_small_unblocked_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "&NaturalMatrix * &NaturalMatrix",
        BenchmarkType::Algorithms,
        natural_matrix_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_matrix_product_size_bucketer("a", "b"),
        &mut [
            ("transposed second matrix", &mut |(a, b)| {
                no_out!(natural_matrix_mul_with(&a, &b, matrix_mul_small_unblocked));
            }),
            ("column reads, as in FLINT", &mut |(a, b)| {
                no_out!(natural_matrix_mul_with(
                    &a,
                    &b,
                    matrix_mul_small_unblocked_column_reads
                ));
            }),
        ],
    );
}

// Compares the algorithms for large elements. Set `mean_bits_n` to a large value, such as 1000, to
// reach the sizes where FLINT chooses Waksman's or Strassen's algorithm. The FFT algorithm, which
// FLINT does not choose, was slower than the default at every size measured.
fn benchmark_natural_matrix_mul_large_elements_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "&NaturalMatrix * &NaturalMatrix",
        BenchmarkType::Algorithms,
        natural_matrix_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_matrix_product_size_bucketer("a", "b"),
        &mut [
            ("default", &mut |(a, b)| {
                no_out!(&a * &b);
            }),
            ("classical", &mut |(a, b)| {
                no_out!(natural_matrix_mul_with(&a, &b, matrix_mul_classical));
            }),
            ("Waksman", &mut |(a, b)| {
                no_out!(natural_matrix_mul_with(&a, &b, matrix_mul_waksman_any));
            }),
            ("Strassen", &mut |(a, b)| {
                no_out!(natural_matrix_mul_with(&a, &b, matrix_mul_strassen_any));
            }),
            ("FFT", &mut |(a, b)| {
                no_out!(natural_matrix_mul_with(&a, &b, matrix_mul_fft));
            }),
            ("naive", &mut |(a, b)| {
                no_out!(natural_matrix_mul_naive(&a, &b));
            }),
        ],
    );
}
