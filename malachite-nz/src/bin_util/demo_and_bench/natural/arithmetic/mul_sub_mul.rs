// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{MulSubMul, MulSubMulAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::quadruple_natural_max_bit_bucketer;
use malachite_nz::test_util::generators::natural_quadruple_gen_var_4;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_mul_sub_mul);
    register_demo!(runner, demo_natural_mul_sub_mul_val_val_val_ref);
    register_demo!(runner, demo_natural_mul_sub_mul_val_val_ref_val);
    register_demo!(runner, demo_natural_mul_sub_mul_val_val_ref_ref);
    register_demo!(runner, demo_natural_mul_sub_mul_val_ref_val_val);
    register_demo!(runner, demo_natural_mul_sub_mul_val_ref_val_ref);
    register_demo!(runner, demo_natural_mul_sub_mul_val_ref_ref_val);
    register_demo!(runner, demo_natural_mul_sub_mul_val_ref_ref_ref);
    register_demo!(runner, demo_natural_mul_sub_mul_ref_ref_ref_ref);
    register_demo!(runner, demo_natural_mul_sub_mul_assign);
    register_demo!(runner, demo_natural_mul_sub_mul_assign_val_val_ref);
    register_demo!(runner, demo_natural_mul_sub_mul_assign_val_ref_val);
    register_demo!(runner, demo_natural_mul_sub_mul_assign_val_ref_ref);
    register_demo!(runner, demo_natural_mul_sub_mul_assign_ref_val_val);
    register_demo!(runner, demo_natural_mul_sub_mul_assign_ref_val_ref);
    register_demo!(runner, demo_natural_mul_sub_mul_assign_ref_ref_val);
    register_demo!(runner, demo_natural_mul_sub_mul_assign_ref_ref_ref);
    register_bench!(runner, benchmark_natural_mul_sub_mul_evaluation_strategy);
    register_bench!(runner, benchmark_natural_mul_sub_mul_algorithms);
}

fn demo_natural_mul_sub_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, w) in natural_quadruple_gen_var_4().get(gm, config).take(limit) {
        println!(
            "({}).mul_sub_mul({}, {}, {}) = {}",
            x.clone(),
            y.clone(),
            z.clone(),
            w.clone(),
            x.clone().mul_sub_mul(y.clone(), z.clone(), w.clone())
        );
    }
}

fn demo_natural_mul_sub_mul_val_val_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, w) in natural_quadruple_gen_var_4().get(gm, config).take(limit) {
        println!(
            "({}).mul_sub_mul({}, {}, {}) = {}",
            x.clone(),
            y.clone(),
            z.clone(),
            w.clone(),
            x.clone().mul_sub_mul(y.clone(), z.clone(), &w.clone())
        );
    }
}

fn demo_natural_mul_sub_mul_val_val_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, w) in natural_quadruple_gen_var_4().get(gm, config).take(limit) {
        println!(
            "({}).mul_sub_mul({}, {}, {}) = {}",
            x.clone(),
            y.clone(),
            z.clone(),
            w.clone(),
            x.clone().mul_sub_mul(y.clone(), &z.clone(), w.clone())
        );
    }
}

fn demo_natural_mul_sub_mul_val_val_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, w) in natural_quadruple_gen_var_4().get(gm, config).take(limit) {
        println!(
            "({}).mul_sub_mul({}, {}, {}) = {}",
            x.clone(),
            y.clone(),
            z.clone(),
            w.clone(),
            x.clone().mul_sub_mul(y.clone(), &z.clone(), &w.clone())
        );
    }
}

fn demo_natural_mul_sub_mul_val_ref_val_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, w) in natural_quadruple_gen_var_4().get(gm, config).take(limit) {
        println!(
            "({}).mul_sub_mul({}, {}, {}) = {}",
            x.clone(),
            y.clone(),
            z.clone(),
            w.clone(),
            x.clone().mul_sub_mul(&y.clone(), z.clone(), w.clone())
        );
    }
}

fn demo_natural_mul_sub_mul_val_ref_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, w) in natural_quadruple_gen_var_4().get(gm, config).take(limit) {
        println!(
            "({}).mul_sub_mul({}, {}, {}) = {}",
            x.clone(),
            y.clone(),
            z.clone(),
            w.clone(),
            x.clone().mul_sub_mul(&y.clone(), z.clone(), &w.clone())
        );
    }
}

fn demo_natural_mul_sub_mul_val_ref_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, w) in natural_quadruple_gen_var_4().get(gm, config).take(limit) {
        println!(
            "({}).mul_sub_mul({}, {}, {}) = {}",
            x.clone(),
            y.clone(),
            z.clone(),
            w.clone(),
            x.clone().mul_sub_mul(&y.clone(), &z.clone(), w.clone())
        );
    }
}

fn demo_natural_mul_sub_mul_val_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, w) in natural_quadruple_gen_var_4().get(gm, config).take(limit) {
        println!(
            "({}).mul_sub_mul({}, {}, {}) = {}",
            x.clone(),
            y.clone(),
            z.clone(),
            w.clone(),
            x.clone().mul_sub_mul(&y.clone(), &z.clone(), &w.clone())
        );
    }
}

fn demo_natural_mul_sub_mul_ref_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, w) in natural_quadruple_gen_var_4().get(gm, config).take(limit) {
        println!(
            "({}).mul_sub_mul({}, {}, {}) = {}",
            x.clone(),
            y.clone(),
            z.clone(),
            w.clone(),
            x.mul_sub_mul(&y.clone(), &z.clone(), &w.clone())
        );
    }
}

fn demo_natural_mul_sub_mul_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, z, w) in natural_quadruple_gen_var_4().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.mul_sub_mul_assign(y.clone(), z.clone(), w.clone());
        println!("x := {x_old}; x.mul_sub_mul_assign({y}, {z}, {w}); x = {x}");
    }
}

fn demo_natural_mul_sub_mul_assign_val_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, z, w) in natural_quadruple_gen_var_4().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.mul_sub_mul_assign(y.clone(), z.clone(), &w.clone());
        println!("x := {x_old}; x.mul_sub_mul_assign({y}, {z}, {w}); x = {x}");
    }
}

fn demo_natural_mul_sub_mul_assign_val_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, z, w) in natural_quadruple_gen_var_4().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.mul_sub_mul_assign(y.clone(), &z.clone(), w.clone());
        println!("x := {x_old}; x.mul_sub_mul_assign({y}, {z}, {w}); x = {x}");
    }
}

fn demo_natural_mul_sub_mul_assign_val_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, z, w) in natural_quadruple_gen_var_4().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.mul_sub_mul_assign(y.clone(), &z.clone(), &w.clone());
        println!("x := {x_old}; x.mul_sub_mul_assign({y}, {z}, {w}); x = {x}");
    }
}

fn demo_natural_mul_sub_mul_assign_ref_val_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, z, w) in natural_quadruple_gen_var_4().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.mul_sub_mul_assign(&y.clone(), z.clone(), w.clone());
        println!("x := {x_old}; x.mul_sub_mul_assign({y}, {z}, {w}); x = {x}");
    }
}

fn demo_natural_mul_sub_mul_assign_ref_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, z, w) in natural_quadruple_gen_var_4().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.mul_sub_mul_assign(&y.clone(), z.clone(), &w.clone());
        println!("x := {x_old}; x.mul_sub_mul_assign({y}, {z}, {w}); x = {x}");
    }
}

fn demo_natural_mul_sub_mul_assign_ref_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, z, w) in natural_quadruple_gen_var_4().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.mul_sub_mul_assign(&y.clone(), &z.clone(), w.clone());
        println!("x := {x_old}; x.mul_sub_mul_assign({y}, {z}, {w}); x = {x}");
    }
}

fn demo_natural_mul_sub_mul_assign_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, z, w) in natural_quadruple_gen_var_4().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.mul_sub_mul_assign(&y.clone(), &z.clone(), &w.clone());
        println!("x := {x_old}; x.mul_sub_mul_assign({y}, {z}, {w}); x = {x}");
    }
}

fn benchmark_natural_mul_sub_mul_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mul_sub_mul(Natural, Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_quadruple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_natural_max_bit_bucketer("x", "y", "z", "w"),
        &mut [
            (
                "Natural.mul_sub_mul(Natural, Natural, Natural)",
                &mut |(x, y, z, w)| no_out!(x.mul_sub_mul(y, z, w)),
            ),
            (
                "Natural.mul_sub_mul(Natural, Natural, &Natural)",
                &mut |(x, y, z, w)| no_out!(x.mul_sub_mul(y, z, &w)),
            ),
            (
                "Natural.mul_sub_mul(Natural, &Natural, Natural)",
                &mut |(x, y, z, w)| no_out!(x.mul_sub_mul(y, &z, w)),
            ),
            (
                "Natural.mul_sub_mul(Natural, &Natural, &Natural)",
                &mut |(x, y, z, w)| no_out!(x.mul_sub_mul(y, &z, &w)),
            ),
            (
                "Natural.mul_sub_mul(&Natural, Natural, Natural)",
                &mut |(x, y, z, w)| no_out!(x.mul_sub_mul(&y, z, w)),
            ),
            (
                "Natural.mul_sub_mul(&Natural, Natural, &Natural)",
                &mut |(x, y, z, w)| no_out!(x.mul_sub_mul(&y, z, &w)),
            ),
            (
                "Natural.mul_sub_mul(&Natural, &Natural, Natural)",
                &mut |(x, y, z, w)| no_out!(x.mul_sub_mul(&y, &z, w)),
            ),
            (
                "Natural.mul_sub_mul(&Natural, &Natural, &Natural)",
                &mut |(x, y, z, w)| no_out!(x.mul_sub_mul(&y, &z, &w)),
            ),
            (
                "(&Natural).mul_sub_mul(&Natural, &Natural, &Natural)",
                &mut |(x, y, z, w)| no_out!((&x).mul_sub_mul(&y, &z, &w)),
            ),
        ],
    );
}

fn benchmark_natural_mul_sub_mul_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mul_sub_mul(Natural, Natural, Natural)",
        BenchmarkType::Algorithms,
        natural_quadruple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_natural_max_bit_bucketer("x", "y", "z", "w"),
        &mut [
            ("default", &mut |(x, y, z, w)| {
                no_out!(x.mul_sub_mul(y, z, w));
            }),
            ("naive", &mut |(x, y, z, w)| {
                let _ = x * y - z * w;
            }),
        ],
    );
}
