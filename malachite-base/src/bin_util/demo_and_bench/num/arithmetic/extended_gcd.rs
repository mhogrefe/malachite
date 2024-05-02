// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::extended_gcd::extended_gcd_unsigned_binary;
use malachite_base::num::arithmetic::traits::ExtendedGcd;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::test_util::bench::bucketers::pair_max_bit_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{signed_pair_gen, unsigned_pair_gen_var_27};
use malachite_base::test_util::num::arithmetic::extended_gcd::extended_gcd_unsigned_euclidean;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_signed_match_demos!(runner, demo_extended_gcd_unsigned);
    register_unsigned_signed_match_demos!(runner, demo_extended_gcd_signed);

    register_unsigned_signed_match_benches!(runner, benchmark_extended_gcd_algorithms_unsigned);
    register_signed_benches!(runner, benchmark_extended_gcd_signed);
}

fn demo_extended_gcd_unsigned<
    U: ExtendedGcd<Cofactor = S> + PrimitiveUnsigned,
    S: PrimitiveSigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y) in unsigned_pair_gen_var_27::<U>().get(gm, config).take(limit) {
        println!("{}.extended_gcd({}) = {:?}", x, y, x.extended_gcd(y));
    }
}

fn demo_extended_gcd_signed<U: PrimitiveUnsigned, S: ExtendedGcd<Gcd = U> + PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y) in signed_pair_gen::<S>().get(gm, config).take(limit) {
        println!("{}.extended_gcd({}) = {:?}", x, y, x.extended_gcd(y));
    }
}

fn benchmark_extended_gcd_algorithms_unsigned<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.extended_gcd({})", U::NAME, U::NAME),
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_27::<U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y)| no_out!(x.extended_gcd(y))),
            ("Euclidean", &mut |(x, y)| {
                no_out!(extended_gcd_unsigned_euclidean::<U, S>(x, y))
            }),
            ("binary", &mut |(x, y)| {
                no_out!(extended_gcd_unsigned_binary::<U, S>(x, y))
            }),
        ],
    );
}

fn benchmark_extended_gcd_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.extended_gcd({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("default", &mut |(x, y)| no_out!(x.extended_gcd(y)))],
    );
}
