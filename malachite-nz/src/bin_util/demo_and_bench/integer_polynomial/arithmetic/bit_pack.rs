// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::BitPack;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_1_integer_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::{
    integer_polynomial_unsigned_pair_gen_var_1, integer_polynomial_unsigned_pair_gen_var_4,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::bit_pack::bit_pack_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_bit_pack);
    register_demo!(runner, demo_integer_polynomial_bit_pack_ref);
    register_demo!(runner, demo_integer_polynomial_bit_pack_small_bits);
    register_bench!(
        runner,
        benchmark_integer_polynomial_bit_pack_evaluation_strategy
    );
    register_bench!(runner, benchmark_integer_polynomial_bit_pack_algorithms);
}

fn demo_integer_polynomial_bit_pack(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, bits) in integer_polynomial_unsigned_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!("({p_old}).bit_pack({bits}) = {}", p.bit_pack(bits));
    }
}

fn demo_integer_polynomial_bit_pack_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, bits) in integer_polynomial_unsigned_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).bit_pack({bits}) = {}", (&p).bit_pack(bits));
    }
}

// Field widths up to 19 bits, mostly narrower than the coefficients, so that the fields overlap,
// and including 0.
fn demo_integer_polynomial_bit_pack_small_bits(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, bits) in integer_polynomial_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).bit_pack({bits}) = {}", (&p).bit_pack(bits));
    }
}

fn benchmark_integer_polynomial_bit_pack_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.bit_pack(u64)",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_unsigned_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_polynomial_bit_bucketer("p"),
        &mut [
            ("IntegerPolynomial.bit_pack(u64)", &mut |(p, bits)| {
                no_out!(p.bit_pack(bits));
            }),
            ("(&IntegerPolynomial).bit_pack(u64)", &mut |(p, bits)| {
                no_out!((&p).bit_pack(bits));
            }),
        ],
    );
}

fn benchmark_integer_polynomial_bit_pack_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.bit_pack(u64)",
        BenchmarkType::Algorithms,
        integer_polynomial_unsigned_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, bits)| {
                no_out!((&p).bit_pack(bits));
            }),
            ("naive", &mut |(p, bits)| {
                no_out!(bit_pack_naive(&p, bits));
            }),
        ],
    );
}
