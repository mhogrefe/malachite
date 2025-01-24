// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::pair_2_pair_1_rational_bit_bucketer;
use malachite_q::test_util::generators::{
    rational_primitive_float_pair_gen, rational_primitive_float_pair_gen_rm,
};
use malachite_q::Rational;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_rational_partial_eq_float);
    register_primitive_float_demos!(runner, demo_float_partial_eq_rational);

    register_primitive_float_benches!(
        runner,
        benchmark_rational_partial_eq_float_library_comparison
    );
    register_primitive_float_benches!(
        runner,
        benchmark_float_partial_eq_rational_library_comparison
    );
}

fn demo_rational_partial_eq_float<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Rational: PartialEq<T>,
{
    for (n, f) in rational_primitive_float_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        if n == f {
            println!("{} = {}", n, NiceFloat(f));
        } else {
            println!("{} ≠ {}", n, NiceFloat(f));
        }
    }
}

fn demo_float_partial_eq_rational<T: PartialEq<Rational> + PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, f) in rational_primitive_float_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        if f == n {
            println!("{} = {}", NiceFloat(f), n);
        } else {
            println!("{} ≠ {}", NiceFloat(f), n);
        }
    }
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_rational_partial_eq_float_library_comparison<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Rational: PartialEq<T>,
    rug::Rational: PartialEq<T>,
{
    run_benchmark(
        &format!("Rational == {}", T::NAME),
        BenchmarkType::LibraryComparison,
        rational_primitive_float_pair_gen_rm::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_rational_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(x == y)),
            ("rug", &mut |((x, y), _)| no_out!(x == y)),
        ],
    );
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_float_partial_eq_rational_library_comparison<
    T: PartialEq<Rational> + PartialEq<rug::Rational> + PrimitiveFloat,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{} == Rational", T::NAME),
        BenchmarkType::LibraryComparison,
        rational_primitive_float_pair_gen_rm::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_rational_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(y == x)),
            ("rug", &mut |((x, y), _)| no_out!(y == x)),
        ],
    );
}
