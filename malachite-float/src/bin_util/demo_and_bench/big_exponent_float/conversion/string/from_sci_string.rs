// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::string::options::FromSciStringOptions;
use malachite_base::num::conversion::traits::ToSci;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::BigExponentFloat;
use malachite_float::test_util::bench::bucketers::pair_1_big_exponent_float_complexity_bucketer;
use malachite_float::test_util::big_exponent_float::big_exponent_float_exact_string;
use malachite_float::test_util::generators::big_exponent_float_to_sci_options_pair_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(
        runner,
        demo_big_exponent_float_from_sci_string_with_options_prec
    );

    register_bench!(
        runner,
        benchmark_big_exponent_float_from_sci_string_with_options_prec
    );
}

// Strings written by `to_sci`, read back in the same base with a precision derived from the value
// and the rounding mode of the options (`Nearest` in place of `Exact`, which the string may not
// allow).
fn demo_big_exponent_float_from_sci_string_with_options_prec(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, options) in big_exponent_float_to_sci_options_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let s = x.to_sci_with_options(options).to_string();
        let prec = 1 + (x.get_prec().unwrap_or(0) * 7) % 100;
        let rm = match options.get_rounding_mode() {
            Exact => Nearest,
            rm => rm,
        };
        let mut from_options = FromSciStringOptions::default();
        from_options.set_base(options.get_base());
        from_options.set_rounding_mode(rm);
        let (y, o) =
            BigExponentFloat::from_sci_string_with_options_prec(&s, from_options, prec).unwrap();
        println!(
            "from_sci_string_with_options_prec(\"{s}\", {}, {prec}, {rm}) = ({}, {o:?})",
            options.get_base(),
            big_exponent_float_exact_string(&y)
        );
    }
}

fn benchmark_big_exponent_float_from_sci_string_with_options_prec(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::from_sci_string_with_options_prec(&str, FromSciStringOptions, u64)",
        BenchmarkType::Single,
        big_exponent_float_to_sci_options_pair_gen_var_1()
            .get(gm, config)
            .map(|(x, options)| {
                let mut from_options = FromSciStringOptions::default();
                from_options.set_base(options.get_base());
                (
                    x.clone(),
                    (x.to_sci_with_options(options).to_string(), from_options),
                )
            }),
        gm.name(),
        limit,
        file_name,
        &pair_1_big_exponent_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |(_, (s, options))| {
            no_out!(BigExponentFloat::from_sci_string_with_options_prec(
                &s, options, 64
            ));
        })],
    );
}
