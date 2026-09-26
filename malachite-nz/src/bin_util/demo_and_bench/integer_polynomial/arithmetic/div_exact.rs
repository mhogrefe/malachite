// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{DivExact, DivExactAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_1_integer_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::integer_polynomial_integer_pair_gen_var_3;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_div_exact);
    register_demo!(runner, demo_integer_polynomial_div_exact_ref);
    register_demo!(runner, demo_integer_polynomial_div_exact_assign);

    register_bench!(
        runner,
        benchmark_integer_polynomial_div_exact_evaluation_strategy
    );
}

fn demo_integer_polynomial_div_exact(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, c) in integer_polynomial_integer_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!("({p_old}).div_exact({c}) = {}", p.div_exact(&c));
    }
}

fn demo_integer_polynomial_div_exact_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, c) in integer_polynomial_integer_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).div_exact({c}) = {}", (&p).div_exact(&c));
    }
}

fn demo_integer_polynomial_div_exact_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, c) in integer_polynomial_integer_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.div_exact_assign(&c);
        println!("p := {p_old}; p.div_exact_assign({c}); p = {p}");
    }
}

fn benchmark_integer_polynomial_div_exact_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.div_exact(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_integer_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_polynomial_bit_bucketer("p"),
        &mut [
            ("IntegerPolynomial.div_exact(Integer)", &mut |(p, c)| {
                no_out!(p.div_exact(c));
            }),
            ("IntegerPolynomial.div_exact(&Integer)", &mut |(p, c)| {
                no_out!(p.div_exact(&c));
            }),
            ("(&IntegerPolynomial).div_exact(Integer)", &mut |(p, c)| {
                no_out!((&p).div_exact(c));
            }),
            ("(&IntegerPolynomial).div_exact(&Integer)", &mut |(p, c)| {
                no_out!((&p).div_exact(&c));
            }),
            (
                "IntegerPolynomial.div_exact_assign(Integer)",
                &mut |(mut p, c)| p.div_exact_assign(c),
            ),
            (
                "IntegerPolynomial.div_exact_assign(&Integer)",
                &mut |(mut p, c)| p.div_exact_assign(&c),
            ),
        ],
    );
}
