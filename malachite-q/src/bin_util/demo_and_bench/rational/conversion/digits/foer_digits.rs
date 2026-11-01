// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_q::rational::conversion::digits::foer_digits::RationalFoerDigits;
use malachite_q::test_util::bench::bucketers::pair_1_rational_bit_bucketer;
use malachite_q::test_util::generators::{rational_gen, rational_natural_pair_gen_var_1};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_foer_digits);
    register_demo!(runner, demo_rational_foer_digits_decimal);
    register_bench!(runner, benchmark_rational_foer_digits);
}

// Renders the first 20 digits after the point, parenthesizing repeating digits and putting `[`
// before each digit that starts a repetition of the repeating part.
fn prefix_helper(after_point: RationalFoerDigits) -> String {
    let non_repeating_len = after_point.non_repeating_len();
    let mut s = String::new();
    for (count, d) in after_point.enumerate() {
        if count == 20 {
            s.push_str("...");
            break;
        }
        if d.period_start {
            s.push('[');
        }
        if d.repeating {
            s.push_str(&format!("({})", d.digit));
        } else {
            s.push_str(&d.digit.to_string());
        }
        s.push(' ');
    }
    format!("non_repeating_len = {non_repeating_len}, {}", s.trim_end())
}

fn demo_rational_foer_digits(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, base) in rational_natural_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (before_point, after_point) = n.foer_digits(&base);
        println!(
            "({}).foer_digits({}) = ({:?}, {})",
            n,
            base,
            before_point,
            prefix_helper(after_point)
        );
    }
}

fn demo_rational_foer_digits_decimal(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in rational_gen().get(gm, config).take(limit) {
        let (before_point, after_point) = n.foer_digits(&Natural::from(10u32));
        println!(
            "({}).foer_digits(10) = ({:?}, {})",
            n,
            before_point,
            prefix_helper(after_point)
        );
    }
}

fn benchmark_rational_foer_digits(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Rational.foer_digits(u64)",
        BenchmarkType::Single,
        rational_natural_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [(
            "Rational.foer_digits(&base).1.take(20).collect_vec()",
            &mut |(n, base)| {
                no_out!(n.foer_digits(&base).1.take(20).collect_vec());
            },
        )],
    );
}
