// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModInverse;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_2_natural_bit_bucketer;
use malachite_nz::test_util::generators::natural_pair_gen_var_11;
use malachite_nz::test_util::natural::arithmetic::mod_inverse::mod_inverse_simple;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_mod_inverse);
    register_demo!(runner, demo_natural_mod_inverse_val_ref);
    register_demo!(runner, demo_natural_mod_inverse_ref_val);
    register_demo!(runner, demo_natural_mod_inverse_ref_ref);

    register_bench!(runner, benchmark_natural_mod_inverse_evaluation_strategy);
    register_bench!(runner, benchmark_natural_mod_inverse_algorithms);
}

fn demo_natural_mod_inverse(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, m) in natural_pair_gen_var_11().get(gm, config).take(limit) {
        let n_old = n.clone();
        let m_old = m.clone();
        if let Some(inverse) = n.mod_inverse(m) {
            println!("{n_old}⁻¹ ≡ {inverse} mod {m_old}");
        } else {
            println!("{n_old} is not invertible mod {m_old}");
        }
    }
}

fn demo_natural_mod_inverse_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, m) in natural_pair_gen_var_11().get(gm, config).take(limit) {
        let n_old = n.clone();
        if let Some(inverse) = n.mod_inverse(&m) {
            println!("{n_old}⁻¹ ≡ {inverse} mod {m}");
        } else {
            println!("{n_old} is not invertible mod {m}");
        }
    }
}

fn demo_natural_mod_inverse_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, m) in natural_pair_gen_var_11().get(gm, config).take(limit) {
        let m_old = m.clone();
        if let Some(inverse) = (&n).mod_inverse(m) {
            println!("{n}⁻¹ ≡ {inverse} mod {m_old}");
        } else {
            println!("{n} is not invertible mod {m_old}");
        }
    }
}

fn demo_natural_mod_inverse_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, m) in natural_pair_gen_var_11().get(gm, config).take(limit) {
        if let Some(inverse) = (&n).mod_inverse(&m) {
            println!("{n}⁻¹ ≡ {inverse} mod {m}");
        } else {
            println!("{n} is not invertible mod {m}");
        }
    }
}

fn benchmark_natural_mod_inverse_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_inverse(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("m"),
        &mut [
            ("Natural.mod_inverse(Natural)", &mut |(n, m)| {
                no_out!(n.mod_inverse(m))
            }),
            ("Natural.mod_inverse(&Natural)", &mut |(n, m)| {
                no_out!(n.mod_inverse(&m))
            }),
            ("(&Natural).mod_inverse(Natural)", &mut |(n, m)| {
                no_out!((&n).mod_inverse(m))
            }),
            ("(&Natural).mod_inverse(&Natural)", &mut |(n, m)| {
                no_out!((&n).mod_inverse(&m))
            }),
        ],
    );
}

fn benchmark_natural_mod_inverse_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_inverse(Natural)",
        BenchmarkType::Algorithms,
        natural_pair_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("m"),
        &mut [
            ("default", &mut |(n, m)| no_out!(n.mod_inverse(m))),
            ("simple", &mut |(n, m)| no_out!(mod_inverse_simple(n, m))),
        ],
    );
}
