// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::strings::gmp_format::{gmp_format, parse_gmp_conversion_spec};
use malachite_base::test_util::bench::bucketers::pair_max_bit_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    char_gen, signed_string_pair_gen_var_1, string_gen, string_gen_var_16,
    unsigned_pair_gen_var_27, unsigned_string_pair_gen_var_4,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_parse_gmp_conversion_spec);
    register_demo!(runner, demo_gmp_format_unsigned);
    register_demo!(runner, demo_gmp_format_signed);
    register_demo!(runner, demo_gmp_format_char);
    register_demo!(runner, demo_gmp_format_str);
    register_demo!(runner, demo_gmp_format_star);
    register_demo!(runner, demo_gmp_format_multiple);
    register_bench!(runner, benchmark_gmp_format);
}

// The generator sweeps every flag subset, type character (including the `R`-with-rounding forms),
// conversion character, and a range of widths and precisions, so every field of the parsed spec
// shows up populated.
fn demo_parse_gmp_conversion_spec(gm: GenMode, config: &GenConfig, limit: usize) {
    for s in string_gen_var_16().get(gm, config).take(limit) {
        println!(
            "parse_gmp_conversion_spec({:?}) = {:?}",
            s,
            parse_gmp_conversion_spec(&s.as_bytes()[1..], &mut || None)
        );
    }
}

// The pair generators sweep every flag subset, C length modifier, and integer conversion, with
// widths and precisions, over the values.
fn demo_gmp_format_unsigned(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, fmt) in unsigned_string_pair_gen_var_4::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!("gmp_format({fmt:?}, [{x}]) = {:?}", gmp_format(&fmt, &[&x]));
    }
}

fn demo_gmp_format_signed(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, fmt) in signed_string_pair_gen_var_1::<i64>()
        .get(gm, config)
        .take(limit)
    {
        println!("gmp_format({fmt:?}, [{x}]) = {:?}", gmp_format(&fmt, &[&x]));
    }
}

// The fixed template lists cycle through every option meaningful for the conversion, so together
// with the generated values these demos exercise the whole `%c` and `%s` spaces.
fn demo_gmp_format_char(gm: GenMode, config: &GenConfig, limit: usize) {
    const TEMPLATES: [&str; 6] = ["%c", "%5c", "%-5c|", "%2c", "%-c", "%12c"];
    for (i, c) in char_gen().get(gm, config).take(limit).enumerate() {
        let fmt = TEMPLATES[i % TEMPLATES.len()];
        println!(
            "gmp_format({fmt:?}, [{c:?}]) = {:?}",
            gmp_format(fmt, &[&c])
        );
    }
}

fn demo_gmp_format_str(gm: GenMode, config: &GenConfig, limit: usize) {
    const TEMPLATES: [&str; 9] =
        ["%s", "%12s", "%-12s|", "%.4s", "%12.4s", "%-12.4s|", "%.s", "%.0s", "%3s"];
    for (i, s) in string_gen().get(gm, config).take(limit).enumerate() {
        let fmt = TEMPLATES[i % TEMPLATES.len()];
        println!(
            "gmp_format({fmt:?}, [{s:?}]) = {:?}",
            gmp_format(fmt, &[&s])
        );
    }
}

// `*` widths and precisions drawn from the argument list, including the negative widths that mean
// left justification.
fn demo_gmp_format_star(gm: GenMode, config: &GenConfig, limit: usize) {
    for (i, (x, y)) in unsigned_pair_gen_var_27::<u64>()
        .get(gm, config)
        .take(limit)
        .enumerate()
    {
        let w = i64::wrapping_from(y) % 15 - 7;
        let p = i64::wrapping_from(y) % 9;
        let (fmt, out) = match i % 4 {
            0 => ("%*d", gmp_format("%*d", &[&w, &x])),
            1 => ("%.*d", gmp_format("%.*d", &[&p, &x])),
            2 => ("%*.*x", gmp_format("%*.*x", &[&w, &p, &x])),
            _ => ("%0*o", gmp_format("%0*o", &[&w, &x])),
        };
        println!("gmp_format({fmt:?}, [{w}, {p}, {x}]) = {out:?}");
    }
}

// Several conversions in one template, consuming the values in order.
fn demo_gmp_format_multiple(gm: GenMode, config: &GenConfig, limit: usize) {
    const TEMPLATES: [&str; 4] = ["%d and %#x", "[%08u|%-8o]", "%X %% %d", "%'d, % i"];
    for (i, (x, y)) in unsigned_pair_gen_var_27::<u64>()
        .get(gm, config)
        .take(limit)
        .enumerate()
    {
        let fmt = TEMPLATES[i % TEMPLATES.len()];
        println!(
            "gmp_format({fmt:?}, [{x}, {y}]) = {:?}",
            gmp_format(fmt, &[&x, &y])
        );
    }
}

fn benchmark_gmp_format(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "gmp_format(&str, &[&dyn GmpFormatArg])",
        BenchmarkType::Single,
        unsigned_pair_gen_var_27::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| {
            no_out!(gmp_format("x = %#x, y = %5d", &[&x, &y]));
        })],
    );
}
