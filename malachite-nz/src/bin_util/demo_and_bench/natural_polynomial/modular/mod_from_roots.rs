// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::modular::traits::ModFromRoots;
use malachite_base::test_util::bench::bucketers::pair_1_vec_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::natural_vec_natural_pair_gen_var_2;
use malachite_nz::test_util::natural_polynomial::modular::mod_from_roots::mod_from_roots_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_mod_from_roots);
    register_demo!(runner, demo_natural_polynomial_mod_from_roots_ref);
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_from_roots_algorithms
    );
}

fn demo_natural_polynomial_mod_from_roots(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, m) in natural_vec_natural_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let m_old = m.clone();
        println!(
            "NaturalPolynomial::mod_from_roots(&{xs:?}, {m_old}) = {}",
            NaturalPolynomial::mod_from_roots(&xs, m)
        );
    }
}

fn demo_natural_polynomial_mod_from_roots_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, m) in natural_vec_natural_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "NaturalPolynomial::mod_from_roots(&{xs:?}, &{m}) = {}",
            NaturalPolynomial::mod_from_roots(&xs, &m)
        );
    }
}

fn benchmark_natural_polynomial_mod_from_roots_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial::mod_from_roots(&[Natural], &Natural)",
        BenchmarkType::Algorithms,
        natural_vec_natural_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [
            ("default", &mut |(xs, m)| {
                no_out!(NaturalPolynomial::mod_from_roots(&xs, &m));
            }),
            ("naive", &mut |(xs, m)| {
                no_out!(mod_from_roots_naive(&xs, &m));
            }),
        ],
    );
}
