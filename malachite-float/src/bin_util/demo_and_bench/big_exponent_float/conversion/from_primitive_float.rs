// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::{
    primitive_float_bucketer, triple_1_2_primitive_float_bit_u64_max_bucketer,
};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::primitive_float_gen;
use malachite_base::test_util::runner::Runner;
use malachite_float::BigExponentFloat;
use malachite_float::test_util::generators::primitive_float_unsigned_rounding_mode_triple_gen_var_3;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_from_f64);
    register_demo!(
        runner,
        demo_big_exponent_float_from_primitive_float_prec_round
    );

    register_bench!(runner, benchmark_big_exponent_float_from_f64);
    register_bench!(
        runner,
        benchmark_big_exponent_float_from_primitive_float_prec_round
    );
}

fn demo_big_exponent_float_from_f64(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in primitive_float_gen::<f64>().get(gm, config).take(limit) {
        println!(
            "BigExponentFloat::from({}) = {:#x}",
            NiceFloat(x),
            BigExponentFloat::from(x)
        );
    }
}

fn demo_big_exponent_float_from_primitive_float_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, prec, rm) in primitive_float_unsigned_rounding_mode_triple_gen_var_3::<f64>()
        .get(gm, config)
        .take(limit)
    {
        let (y, o) = BigExponentFloat::from_primitive_float_prec_round(x, prec, rm);
        println!(
            "BigExponentFloat::from_primitive_float_prec_round({}, {prec}, {rm}) = ({y:#x}, {o:?})",
            NiceFloat(x)
        );
    }
}

fn benchmark_big_exponent_float_from_f64(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::from(f64)",
        BenchmarkType::Single,
        primitive_float_gen::<f64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("x"),
        &mut [("Malachite", &mut |x| {
            no_out!(BigExponentFloat::from(x));
        })],
    );
}

fn benchmark_big_exponent_float_from_primitive_float_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::from_primitive_float_prec_round(f64, u64, RoundingMode)",
        BenchmarkType::Single,
        primitive_float_unsigned_rounding_mode_triple_gen_var_3::<f64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_primitive_float_bit_u64_max_bucketer("x", "prec"),
        &mut [("Malachite", &mut |(x, prec, rm)| {
            no_out!(BigExponentFloat::from_primitive_float_prec_round(
                x, prec, rm
            ));
        })],
    );
}
