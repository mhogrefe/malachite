// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::BitUnpack;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::arithmetic::bit_unpack::{
    limbs_unpack_coefficients, limbs_unpack_coefficients_unsigned,
};
use malachite_nz::platform::Limb;
use malachite_nz::test_util::bench::bucketers::pair_1_integer_bit_bucketer;
use malachite_nz::test_util::generators::{
    integer_unsigned_pair_gen_var_6, natural_unsigned_pair_gen_var_7,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::bit_unpack::bit_unpack_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_bit_unpack);
    register_demo!(runner, demo_integer_polynomial_bit_unpack_ref);
    register_demo!(runner, demo_limbs_unpack_coefficients);
    register_demo!(runner, demo_limbs_unpack_coefficients_unsigned);
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

// The limbs of a positive Natural, padded with zero limbs to cover every field, unpacked into every
// field that holds any of its bits, with and without negation. The result is followed by the
// returned borrow.
fn demo_limbs_unpack_coefficients(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, bits) in natural_unsigned_pair_gen_var_7::<u64>()
        .get(gm, config)
        .filter(|(n, _)| *n != 0u32)
        .take(limit)
    {
        let nhi = usize::exact_from(n.significant_bits().div_ceil(bits));
        let mut xs = n.to_limbs_asc();
        // Every field lies within the first nhi * bits bits; one more limb covers FLINT's reads.
        xs.resize(
            usize::exact_from((u64::exact_from(nhi) * bits).div_ceil(Limb::WIDTH)) + 1,
            0,
        );
        for negate in [false, true] {
            let mut out = vec![Integer::ZERO; nhi];
            let borrow = limbs_unpack_coefficients(&mut out, 0, nhi, &xs, bits, negate);
            println!("limbs_unpack_coefficients({xs:?}, {bits}, {negate}) = {out:?}, {borrow}");
        }
    }
}

// The limbs of a positive Natural, padded with zero limbs to cover every field, unpacked as
// unsigned numbers into every field that holds any of its bits.
fn demo_limbs_unpack_coefficients_unsigned(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, bits) in natural_unsigned_pair_gen_var_7::<u64>()
        .get(gm, config)
        .filter(|(n, _)| *n != 0u32)
        .take(limit)
    {
        let nhi = usize::exact_from(n.significant_bits().div_ceil(bits));
        let mut xs = n.to_limbs_asc();
        // Every field lies within the first nhi * bits bits; one more limb covers FLINT's reads.
        xs.resize(
            usize::exact_from((u64::exact_from(nhi) * bits).div_ceil(Limb::WIDTH)) + 1,
            0,
        );
        let mut out = vec![Integer::ZERO; nhi];
        limbs_unpack_coefficients_unsigned(&mut out, 0, nhi, &xs, bits);
        println!("limbs_unpack_coefficients_unsigned({xs:?}, {bits}) = {out:?}");
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
