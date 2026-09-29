// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::BitUnpack;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::bench::bucketers::pair_1_integer_bit_bucketer;
use malachite_nz::test_util::generators::integer_unsigned_pair_gen_var_6;
use malachite_nz::test_util::integer_polynomial::arithmetic::bit_unpack::bit_unpack_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_bit_unpack);
    register_demo!(runner, demo_integer_polynomial_bit_unpack_ref);
    register_bench!(
        runner,
        benchmark_integer_polynomial_bit_unpack_evaluation_strategy
    );
    register_bench!(runner, benchmark_integer_polynomial_bit_unpack_algorithms);
}

fn demo_integer_polynomial_bit_unpack(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, bits) in integer_unsigned_pair_gen_var_6::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "IntegerPolynomial::bit_unpack({n_old}, {bits}) = {}",
            IntegerPolynomial::bit_unpack(n, bits)
        );
    }
}

fn demo_integer_polynomial_bit_unpack_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, bits) in integer_unsigned_pair_gen_var_6::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "IntegerPolynomial::bit_unpack(&{n}, {bits}) = {}",
            IntegerPolynomial::bit_unpack(&n, bits)
        );
    }
}

fn benchmark_integer_polynomial_bit_unpack_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial::bit_unpack(Integer, u64)",
        BenchmarkType::EvaluationStrategy,
        integer_unsigned_pair_gen_var_6::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("n"),
        &mut [
            (
                "IntegerPolynomial::bit_unpack(Integer, u64)",
                &mut |(n, bits)| {
                    no_out!(IntegerPolynomial::bit_unpack(n, bits));
                },
            ),
            (
                "IntegerPolynomial::bit_unpack(&Integer, u64)",
                &mut |(n, bits)| {
                    no_out!(IntegerPolynomial::bit_unpack(&n, bits));
                },
            ),
        ],
    );
}

fn benchmark_integer_polynomial_bit_unpack_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial::bit_unpack(&Integer, u64)",
        BenchmarkType::Algorithms,
        integer_unsigned_pair_gen_var_6::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("n"),
        &mut [
            ("default", &mut |(n, bits)| {
                no_out!(IntegerPolynomial::bit_unpack(&n, bits));
            }),
            ("naive", &mut |(n, bits)| {
                no_out!(bit_unpack_naive(&n, bits));
            }),
        ],
    );
}
