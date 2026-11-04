// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::modular::traits::CheckedModPowerOf2Lcm;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::bench::bucketers::triple_1_2_natural_polynomial_max_bit_bucketer;
use malachite_nz::test_util::generators::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_checked_mod_power_of_2_lcm);
    register_demo!(
        runner,
        demo_natural_polynomial_checked_mod_power_of_2_lcm_ref_ref
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_checked_mod_power_of_2_lcm_evaluation_strategy
    );
}

fn show_checked(x: Option<NaturalPolynomial>) -> String {
    x.map_or_else(|| "None".to_string(), |p| p.to_string())
}

fn demo_natural_polynomial_checked_mod_power_of_2_lcm(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, pow) in natural_polynomial_natural_polynomial_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "({p_old}).checked_mod_power_of_2_lcm({q_old}, {pow}) = {}",
            show_checked(p.checked_mod_power_of_2_lcm(q, pow))
        );
    }
}

fn demo_natural_polynomial_checked_mod_power_of_2_lcm_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, pow) in natural_polynomial_natural_polynomial_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).checked_mod_power_of_2_lcm(&({q}), {pow}) = {}",
            show_checked((&p).checked_mod_power_of_2_lcm(&q, pow))
        );
    }
}

fn benchmark_natural_polynomial_checked_mod_power_of_2_lcm_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.checked_mod_power_of_2_lcm(NaturalPolynomial, u64)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_natural_polynomial_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("p.checked_mod_power_of_2_lcm(q, pow)", &mut |(
                p,
                q,
                pow,
            )| {
                no_out!(p.checked_mod_power_of_2_lcm(q, pow));
            }),
            (
                "p.checked_mod_power_of_2_lcm(&q, pow)",
                &mut |(p, q, pow)| {
                    no_out!(p.checked_mod_power_of_2_lcm(&q, pow));
                },
            ),
            (
                "(&p).checked_mod_power_of_2_lcm(q, pow)",
                &mut |(p, q, pow)| {
                    no_out!((&p).checked_mod_power_of_2_lcm(q, pow));
                },
            ),
            (
                "(&p).checked_mod_power_of_2_lcm(&q, pow)",
                &mut |(p, q, pow)| {
                    no_out!((&p).checked_mod_power_of_2_lcm(&q, pow));
                },
            ),
        ],
    );
}
