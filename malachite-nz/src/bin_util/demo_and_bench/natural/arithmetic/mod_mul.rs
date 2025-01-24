// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModMul, ModMulAssign, ModMulPrecomputed};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_pair_gen_var_36;
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::mod_mul::{
    limbs_mod_mul_two_limbs, limbs_precompute_mod_mul_two_limbs,
};
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::{
    limb_pair_significant_bits_bucketer, limbs_mod_mul_two_limbs_bucketer,
    triple_3_natural_bit_bucketer,
};
use malachite_nz::test_util::generators::{large_type_gen_var_21, natural_triple_gen_var_3};
use malachite_nz::test_util::natural::arithmetic::mod_mul::{
    limbs_mod_mul_two_limbs_naive, limbs_precompute_mod_mul_two_limbs_alt,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_precompute_mod_mul_two_limbs);
    register_demo!(runner, demo_limbs_mod_mul_two_limbs);
    register_demo!(runner, demo_natural_mod_mul_assign);
    register_demo!(runner, demo_natural_mod_mul_assign_val_ref);
    register_demo!(runner, demo_natural_mod_mul_assign_ref_val);
    register_demo!(runner, demo_natural_mod_mul_assign_ref_ref);
    register_demo!(runner, demo_natural_mod_mul);
    register_demo!(runner, demo_natural_mod_mul_val_val_ref);
    register_demo!(runner, demo_natural_mod_mul_val_ref_val);
    register_demo!(runner, demo_natural_mod_mul_val_ref_ref);
    register_demo!(runner, demo_natural_mod_mul_ref_val_val);
    register_demo!(runner, demo_natural_mod_mul_ref_val_ref);
    register_demo!(runner, demo_natural_mod_mul_ref_ref_val);
    register_demo!(runner, demo_natural_mod_mul_ref_ref_ref);

    register_bench!(
        runner,
        benchmark_limbs_precompute_mod_mul_two_limbs_algorithms
    );
    register_bench!(runner, benchmark_limbs_mod_mul_two_limbs);
    register_bench!(runner, benchmark_natural_mod_mul_assign_evaluation_strategy);
    register_bench!(runner, benchmark_natural_mod_mul_algorithms);
    register_bench!(runner, benchmark_natural_mod_mul_evaluation_strategy);
    register_bench!(runner, benchmark_natural_mod_mul_precomputed_algorithms);
}

fn demo_limbs_precompute_mod_mul_two_limbs(gm: GenMode, config: &GenConfig, limit: usize) {
    for (m_1, m_0) in unsigned_pair_gen_var_36().get(gm, config).take(limit) {
        println!(
            "limbs_precompute_mod_mul_two_limbs({}, {}) = {:?}",
            m_1,
            m_0,
            limbs_precompute_mod_mul_two_limbs(m_1, m_0)
        );
    }
}

fn demo_limbs_mod_mul_two_limbs(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x_1, x_0, y_1, y_0, m_1, m_0, inv_2, inv_1, inv_0) in
        large_type_gen_var_21().get(gm, config).take(limit)
    {
        println!(
            "limbs_mod_mul_two_limbs({}, {}, {}, {}, {}, {}, {}, {}, {}) = {:?}",
            x_1,
            x_0,
            y_1,
            y_0,
            m_1,
            m_0,
            inv_2,
            inv_1,
            inv_0,
            limbs_mod_mul_two_limbs(x_1, x_0, y_1, y_0, m_1, m_0, inv_2, inv_1, inv_0)
        );
    }
}

fn demo_natural_mod_mul_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, m) in natural_triple_gen_var_3().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let m_old = m.clone();
        x.mod_mul_assign(y, m);
        println!("x := {x_old}; x.mod_mul_assign({y_old}, {m_old}); x = {x}");
    }
}

fn demo_natural_mod_mul_assign_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, m) in natural_triple_gen_var_3().get(gm, config).take(limit) {
        let m_old = m.clone();
        let y_old = y.clone();
        x.mod_mul_assign(y, &m);
        println!("x := {x}; x.mod_mul_assign({y_old}, &{m_old}); x = {x}");
    }
}

fn demo_natural_mod_mul_assign_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, m) in natural_triple_gen_var_3().get(gm, config).take(limit) {
        let x_old = x.clone();
        let m_old = m.clone();
        x.mod_mul_assign(&y, m);
        println!("x := {x_old}; x.mod_mul_assign(&{y}, {m_old}); x = {x}");
    }
}

fn demo_natural_mod_mul_assign_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, m) in natural_triple_gen_var_3().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.mod_mul_assign(&y, &m);
        println!("x := {x_old}; x.mod_mul_assign(&{y}, &{m}); x = {x}");
    }
}

fn demo_natural_mod_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in natural_triple_gen_var_3().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let m_old = m.clone();
        println!("{} * {} ≡ {} mod {}", x_old, y_old, x.mod_mul(y, m), m_old);
    }
}

fn demo_natural_mod_mul_val_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in natural_triple_gen_var_3().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} * {} ≡ {} mod {}", x_old, y_old, x.mod_mul(y, &m), m);
    }
}

fn demo_natural_mod_mul_val_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in natural_triple_gen_var_3().get(gm, config).take(limit) {
        let x_old = x.clone();
        let m_old = m.clone();
        println!("{} * {} ≡ {} mod {}", x_old, y, x.mod_mul(&y, m), m_old);
    }
}

fn demo_natural_mod_mul_val_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in natural_triple_gen_var_3().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("{} * {} ≡ {} mod {}", x_old, y, x.mod_mul(&y, &m), m);
    }
}

fn demo_natural_mod_mul_ref_val_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in natural_triple_gen_var_3().get(gm, config).take(limit) {
        let y_old = y.clone();
        let m_old = m.clone();
        println!("{} * {} ≡ {} mod {}", x, y_old, (&x).mod_mul(y, m), m_old);
    }
}

fn demo_natural_mod_mul_ref_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in natural_triple_gen_var_3().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!("{} * {} ≡ {} mod {}", x, y_old, (&x).mod_mul(y, &m), m);
    }
}

fn demo_natural_mod_mul_ref_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in natural_triple_gen_var_3().get(gm, config).take(limit) {
        let m_old = m.clone();
        println!("{} * {} ≡ {} mod {}", x, y, (&x).mod_mul(&y, m), m_old);
    }
}

fn demo_natural_mod_mul_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in natural_triple_gen_var_3().get(gm, config).take(limit) {
        println!("{} * {} ≡ {} mod {}", x, y, (&x).mod_mul(&y, &m), m);
    }
}

fn benchmark_limbs_precompute_mod_mul_two_limbs_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_precompute_mod_mul_two_limbs(Limb, Limb)",
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_36().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &limb_pair_significant_bits_bucketer("m"),
        &mut [
            ("default", &mut |(m_1, m_0)| {
                no_out!(limbs_precompute_mod_mul_two_limbs(m_1, m_0))
            }),
            ("alt", &mut |(m_1, m_0)| {
                no_out!(limbs_precompute_mod_mul_two_limbs_alt(m_1, m_0))
            }),
        ],
    );
}

fn benchmark_limbs_mod_mul_two_limbs(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mod_mul_two_limbs(Limb, Limb, Limb, Limb, Limb, Limb, Limb, Limb, Limb)",
        BenchmarkType::Single,
        large_type_gen_var_21().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &limbs_mod_mul_two_limbs_bucketer(),
        &mut [
            ("default", &mut |(
                x_1,
                x_0,
                y_1,
                y_0,
                m_1,
                m_0,
                inv_2,
                inv_1,
                inv_0,
            )| {
                no_out!(limbs_mod_mul_two_limbs(
                    x_1, x_0, y_1, y_0, m_1, m_0, inv_2, inv_1, inv_0
                ))
            }),
            ("naive", &mut |(x_1, x_0, y_1, y_0, m_1, m_0, _, _, _)| {
                no_out!(limbs_mod_mul_two_limbs_naive(x_1, x_0, y_1, y_0, m_1, m_0))
            }),
        ],
    );
}

fn benchmark_natural_mod_mul_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_mul_assign(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_triple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_natural_bit_bucketer("m"),
        &mut [
            (
                "Natural.mod_mul_assign(Natural, Natural)",
                &mut |(mut x, y, m)| no_out!(x.mod_mul_assign(y, m)),
            ),
            (
                "Natural.mod_mul_assign(Natural, &Natural)",
                &mut |(mut x, y, m)| no_out!(x.mod_mul_assign(y, &m)),
            ),
            (
                "Natural.mod_mul_assign(&Natural, Natural)",
                &mut |(mut x, y, m)| no_out!(x.mod_mul_assign(&y, m)),
            ),
            (
                "Natural.mod_mul_assign(&Natural, &Natural)",
                &mut |(mut x, y, m)| no_out!(x.mod_mul_assign(&y, &m)),
            ),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_natural_mod_mul_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_mul(Natural, Natural)",
        BenchmarkType::Algorithms,
        natural_triple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_natural_bit_bucketer("m"),
        &mut [
            ("default", &mut |(x, y, m)| no_out!(x.mod_mul(y, m))),
            ("naive", &mut |(x, y, m)| no_out!((x * y) % m)),
        ],
    );
}

fn benchmark_natural_mod_mul_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_mul(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_triple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_natural_bit_bucketer("m"),
        &mut [
            ("Natural.mod_mul(Natural, Natural)", &mut |(x, y, m)| {
                no_out!(x.mod_mul(y, m))
            }),
            ("Natural.mod_mul(Natural, &Natural)", &mut |(x, y, m)| {
                no_out!(x.mod_mul(y, &m))
            }),
            ("Natural.mod_mul(&Natural, Natural)", &mut |(x, y, m)| {
                no_out!(x.mod_mul(&y, m))
            }),
            ("Natural.mod_mul(&Natural, &Natural)", &mut |(x, y, m)| {
                no_out!(x.mod_mul(&y, &m))
            }),
            ("(&Natural).mod_mul(Natural, Natural)", &mut |(x, y, m)| {
                no_out!((&x).mod_mul(y, m))
            }),
            ("(&Natural).mod_mul(Natural, &Natural)", &mut |(x, y, m)| {
                no_out!((&x).mod_mul(y, &m))
            }),
            ("(&Natural).mod_mul(&Natural, Natural)", &mut |(x, y, m)| {
                no_out!((&x).mod_mul(&y, m))
            }),
            (
                "(&Natural).mod_mul(&Natural, &Natural)",
                &mut |(x, y, m)| no_out!((&x).mod_mul(&y, &m)),
            ),
        ],
    );
}

fn benchmark_natural_mod_mul_precomputed_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_mul_precomputed(Natural, Natural, &ModMulData)",
        BenchmarkType::Algorithms,
        natural_triple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_natural_bit_bucketer("m"),
        &mut [
            ("default", &mut |(x, y, m)| {
                for _ in 0..10 {
                    (&x).mod_mul(&y, &m);
                }
            }),
            ("precomputed", &mut |(x, y, m)| {
                let data = ModMulPrecomputed::<Natural>::precompute_mod_mul_data(&m);
                for _ in 0..10 {
                    (&x).mod_mul_precomputed(&y, &m, &data);
                }
            }),
        ],
    );
}
