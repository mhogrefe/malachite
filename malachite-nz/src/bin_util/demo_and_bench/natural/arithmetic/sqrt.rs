// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CeilingSqrt, CeilingSqrtAssign, CheckedSqrt, FloorSqrt, FloorSqrtAssign, SqrtAssignRem,
    SqrtRem, Square,
};
use malachite_base::test_util::bench::bucketers::{
    pair_2_vec_len_bucketer, pair_max_bit_bucketer, quadruple_2_vec_len_bucketer,
    triple_3_vec_len_bucketer, vec_len_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    large_type_gen_var_2, unsigned_pair_gen_var_31, unsigned_vec_gen_var_1,
    unsigned_vec_pair_gen_var_4, unsigned_vec_pair_gen_var_5, unsigned_vec_triple_gen_var_28,
};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::sqrt::{
    limbs_ceiling_sqrt, limbs_checked_sqrt, limbs_floor_sqrt, limbs_sqrt_helper, limbs_sqrt_rem,
    limbs_sqrt_rem_helper, limbs_sqrt_rem_helper_scratch_len, limbs_sqrt_rem_to_out,
    limbs_sqrt_to_out, sqrt_rem_2_newton,
};
use malachite_nz::test_util::bench::bucketers::{
    natural_bit_bucketer, pair_2_natural_bit_bucketer, triple_3_natural_bit_bucketer,
};
use malachite_nz::test_util::generators::{natural_gen, natural_gen_nrm, natural_gen_rm};
use malachite_nz::test_util::natural::arithmetic::sqrt::{
    ceiling_sqrt_binary, checked_sqrt_binary, floor_sqrt_binary, sqrt_rem_binary,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_sqrt_rem_2_newton);
    register_demo!(runner, demo_limbs_sqrt_rem_helper);
    register_demo!(runner, demo_limbs_sqrt_helper);
    register_demo!(runner, demo_limbs_sqrt_to_out);
    register_demo!(runner, demo_limbs_sqrt_rem_to_out);
    register_demo!(runner, demo_limbs_floor_sqrt);
    register_demo!(runner, demo_limbs_ceiling_sqrt);
    register_demo!(runner, demo_limbs_checked_sqrt);
    register_demo!(runner, demo_limbs_sqrt_rem);
    register_demo!(runner, demo_natural_floor_sqrt);
    register_demo!(runner, demo_natural_floor_sqrt_ref);
    register_demo!(runner, demo_natural_floor_sqrt_assign);
    register_demo!(runner, demo_natural_ceiling_sqrt);
    register_demo!(runner, demo_natural_ceiling_sqrt_ref);
    register_demo!(runner, demo_natural_ceiling_sqrt_assign);
    register_demo!(runner, demo_natural_checked_sqrt);
    register_demo!(runner, demo_natural_checked_sqrt_ref);
    register_demo!(runner, demo_natural_sqrt_rem);
    register_demo!(runner, demo_natural_sqrt_rem_ref);
    register_demo!(runner, demo_natural_sqrt_assign_rem);
    register_bench!(runner, benchmark_sqrt_rem_2_newton);
    register_bench!(runner, benchmark_limbs_sqrt_rem_helper);
    register_bench!(runner, benchmark_limbs_sqrt_helper);
    register_bench!(runner, benchmark_limbs_sqrt_to_out);
    register_bench!(runner, benchmark_limbs_sqrt_rem_to_out);
    register_bench!(runner, benchmark_limbs_floor_sqrt);
    register_bench!(runner, benchmark_limbs_ceiling_sqrt);
    register_bench!(runner, benchmark_limbs_checked_sqrt);
    register_bench!(runner, benchmark_limbs_sqrt_rem);
    register_bench!(runner, benchmark_natural_floor_sqrt_evaluation_strategy);
    register_bench!(runner, benchmark_natural_floor_sqrt_algorithms);
    register_bench!(runner, benchmark_natural_floor_sqrt_library_comparison);
    register_bench!(runner, benchmark_natural_floor_sqrt_assign);
    register_bench!(runner, benchmark_natural_ceiling_sqrt_evaluation_strategy);
    register_bench!(runner, benchmark_natural_ceiling_sqrt_algorithms);
    register_bench!(runner, benchmark_natural_ceiling_sqrt_assign);
    register_bench!(runner, benchmark_natural_checked_sqrt_evaluation_strategy);
    register_bench!(runner, benchmark_natural_checked_sqrt_algorithms);
    register_bench!(runner, benchmark_natural_sqrt_rem_evaluation_strategy);
    register_bench!(runner, benchmark_natural_sqrt_rem_algorithms);
    register_bench!(runner, benchmark_natural_sqrt_rem_library_comparison);
    register_bench!(runner, benchmark_natural_sqrt_assign_rem);
}

fn demo_sqrt_rem_2_newton(gm: GenMode, config: &GenConfig, limit: usize) {
    for (h_hi, h_lo) in unsigned_pair_gen_var_31().get(gm, config).take(limit) {
        println!(
            "sqrt_rem_2_newton({}, {}) = {:?}",
            h_hi,
            h_lo,
            sqrt_rem_2_newton(h_hi, h_lo)
        );
    }
}

fn demo_limbs_sqrt_rem_helper(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, mut xs) in unsigned_vec_pair_gen_var_4().get(gm, config).take(limit) {
        let mut scratch = vec![0; limbs_sqrt_rem_helper_scratch_len(out.len())];
        let old_out = out.clone();
        let old_xs = xs.clone();
        let r_hi = limbs_sqrt_rem_helper(&mut out, &mut xs, 0, &mut scratch);
        println!(
            "out := {old_out:?}, xs := {old_xs:?}; \
            limbs_sqrt_rem_helper(&mut out, &mut xs, 0, &mut scratch) = {r_hi}; \
            out = {out:?}, xs = {xs:?}",
        );
    }
}

fn demo_limbs_sqrt_helper(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, xs, shift, odd) in large_type_gen_var_2().get(gm, config).take(limit) {
        let old_out = out.clone();
        let r = limbs_sqrt_helper(&mut out, &xs, shift, odd);
        println!(
            "out := {old_out:?}, \
            limbs_sqrt_helper(&mut out, {xs:?}, {shift}, {odd}) = {r}; out = {out:?}",
        );
    }
}

fn demo_limbs_sqrt_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, xs) in unsigned_vec_pair_gen_var_5().get(gm, config).take(limit) {
        let old_out = out.clone();
        limbs_sqrt_to_out(&mut out, &xs);
        println!("out := {old_out:?}, limbs_sqrt_to_out(&mut out, {xs:?}); out = {out:?}");
    }
}

fn demo_limbs_sqrt_rem_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut sqrt_out, mut rem_out, xs) in
        unsigned_vec_triple_gen_var_28().get(gm, config).take(limit)
    {
        let old_sqrt_out = sqrt_out.clone();
        let old_rem_out = rem_out.clone();
        let r = limbs_sqrt_rem_to_out(&mut sqrt_out, &mut rem_out, &xs);
        println!(
            "out := {old_sqrt_out:?}, rem_out := {old_rem_out:?}; \
            limbs_sqrt_rem_to_out(&mut sqrt_out, &mut rem_out, {xs:?}) = {r}; \
            sqrt_out = {sqrt_out:?}, rem_out = {rem_out:?}",
        );
    }
}

fn demo_limbs_floor_sqrt(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_vec_gen_var_1().get(gm, config).take(limit) {
        println!("limbs_floor_sqrt({:?}) = {:?}", xs, limbs_floor_sqrt(&xs));
    }
}

fn demo_limbs_ceiling_sqrt(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_vec_gen_var_1().get(gm, config).take(limit) {
        println!(
            "limbs_ceiling_sqrt({:?}) = {:?}",
            xs,
            limbs_ceiling_sqrt(&xs)
        );
    }
}

fn demo_limbs_checked_sqrt(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_vec_gen_var_1().get(gm, config).take(limit) {
        println!(
            "limbs_checked_sqrt({:?}) = {:?}",
            xs,
            limbs_checked_sqrt(&xs)
        );
    }
}

fn demo_limbs_sqrt_rem(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_vec_gen_var_1().get(gm, config).take(limit) {
        println!("limbs_sqrt_rem({:?}) = {:?}", xs, limbs_sqrt_rem(&xs));
    }
}

fn demo_natural_floor_sqrt(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in natural_gen().get(gm, config).take(limit) {
        println!("{}.floor_sqrt() = {}", x, x.clone().floor_sqrt());
    }
}

fn demo_natural_floor_sqrt_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in natural_gen().get(gm, config).take(limit) {
        println!("(&{}).floor_sqrt() = {}", x, (&x).floor_sqrt());
    }
}

fn demo_natural_floor_sqrt_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in natural_gen().get(gm, config).take(limit) {
        let old_x = x.clone();
        x.floor_sqrt_assign();
        println!("x := {old_x}; x.floor_sqrt_assign(); x = {x}");
    }
}

fn demo_natural_ceiling_sqrt(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in natural_gen().get(gm, config).take(limit) {
        println!("{}.ceiling_sqrt() = {}", x, x.clone().ceiling_sqrt());
    }
}

fn demo_natural_ceiling_sqrt_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in natural_gen().get(gm, config).take(limit) {
        println!("(&{}).ceiling_sqrt() = {}", x, (&x).ceiling_sqrt());
    }
}

fn demo_natural_ceiling_sqrt_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in natural_gen().get(gm, config).take(limit) {
        let old_x = x.clone();
        x.ceiling_sqrt_assign();
        println!("x := {old_x}; x.ceiling_sqrt_assign(); x = {x}");
    }
}

fn demo_natural_checked_sqrt(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in natural_gen().get(gm, config).take(limit) {
        println!("{}.checked_sqrt() = {:?}", x, x.clone().checked_sqrt());
    }
}

fn demo_natural_checked_sqrt_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in natural_gen().get(gm, config).take(limit) {
        println!("(&{}).checked_sqrt() = {:?}", x, (&x).checked_sqrt());
    }
}

fn demo_natural_sqrt_rem(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in natural_gen().get(gm, config).take(limit) {
        println!("{}.sqrt_rem() = {:?}", x, x.clone().sqrt_rem());
    }
}

fn demo_natural_sqrt_rem_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in natural_gen().get(gm, config).take(limit) {
        println!("(&{}).sqrt_rem() = {:?}", x, (&x).sqrt_rem());
    }
}

fn demo_natural_sqrt_assign_rem(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in natural_gen().get(gm, config).take(limit) {
        let old_x = x.clone();
        let rem = x.sqrt_assign_rem();
        println!("x := {old_x}; x.sqrt_assign_rem() = {rem}; x = {x}");
    }
}

fn benchmark_sqrt_rem_2_newton(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "sqrt_rem_2_newton(Limb, Limb)",
        BenchmarkType::Single,
        unsigned_pair_gen_var_31().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("n_hi", "n_lo"),
        &mut [("sqrt_rem_2_newton(Limb, Limb)", &mut |(n_hi, n_lo)| {
            no_out!(sqrt_rem_2_newton(n_hi, n_lo))
        })],
    );
}

fn benchmark_limbs_sqrt_rem_helper(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_sqrt_rem_helper(&mut [Limb], &mut [Limb], Limb, &mut [Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_vec_len_bucketer("xs"),
        &mut [(
            "limbs_sqrt_rem_helper(&mut [Limb], &mut [Limb], Limb, &mut [Limb])",
            &mut |(mut out, mut xs)| {
                let mut scratch = vec![0; limbs_sqrt_rem_helper_scratch_len(out.len())];
                limbs_sqrt_rem_helper(&mut out, &mut xs, 0, &mut scratch);
            },
        )],
    );
}

fn benchmark_limbs_sqrt_helper(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_sqrt_helper(&mut [Limb], &[Limb], u64, bool)",
        BenchmarkType::Single,
        large_type_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_2_vec_len_bucketer("xs"),
        &mut [(
            "limbs_sqrt_helper(&mut [Limb], &[Limb], u64, bool)",
            &mut |(mut out, xs, shift, odd)| no_out!(limbs_sqrt_helper(&mut out, &xs, shift, odd)),
        )],
    );
}

fn benchmark_limbs_sqrt_to_out(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_sqrt_to_out(&mut [Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_vec_len_bucketer("xs"),
        &mut [("limbs_sqrt_to_out(&mut [Limb], &[Limb])", &mut |(
            mut out,
            xs,
        )| {
            limbs_sqrt_to_out(&mut out, &xs)
        })],
    );
}

fn benchmark_limbs_sqrt_rem_to_out(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_sqrt_rem_to_out(&mut [Limb], &mut [Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_triple_gen_var_28().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_vec_len_bucketer("xs"),
        &mut [(
            "limbs_sqrt_rem_to_out(&mut [Limb], &mut [Limb], &[Limb])",
            &mut |(mut sqrt_out, mut rem_out, xs)| {
                no_out!(limbs_sqrt_rem_to_out(&mut sqrt_out, &mut rem_out, &xs))
            },
        )],
    );
}

fn benchmark_limbs_floor_sqrt(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_floor_sqrt(&[Limb])",
        BenchmarkType::Single,
        unsigned_vec_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [("limbs_floor_sqrt(&[Limb])", &mut |xs| {
            no_out!(limbs_floor_sqrt(&xs))
        })],
    );
}

fn benchmark_limbs_ceiling_sqrt(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_floor_sqrt(&[Limb])",
        BenchmarkType::Single,
        unsigned_vec_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [("limbs_ceiling_sqrt(&[Limb])", &mut |xs| {
            no_out!(limbs_ceiling_sqrt(&xs))
        })],
    );
}

fn benchmark_limbs_checked_sqrt(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_checked_sqrt(&[Limb])",
        BenchmarkType::Single,
        unsigned_vec_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [("limbs_checked_sqrt(&[Limb])", &mut |xs| {
            no_out!(limbs_checked_sqrt(&xs))
        })],
    );
}

fn benchmark_limbs_sqrt_rem(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_sqrt_rem(&[Limb])",
        BenchmarkType::Single,
        unsigned_vec_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [("limbs_sqrt_rem(&[Limb])", &mut |xs| {
            no_out!(limbs_sqrt_rem(&xs))
        })],
    );
}

fn benchmark_natural_floor_sqrt_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.floor_sqrt()",
        BenchmarkType::EvaluationStrategy,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [
            ("Natural.floor_sqrt()", &mut |x| no_out!(x.floor_sqrt())),
            ("(&Natural).floor_sqrt()", &mut |x| {
                no_out!((&x).floor_sqrt())
            }),
        ],
    );
}

fn benchmark_natural_floor_sqrt_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.floor_sqrt()",
        BenchmarkType::Algorithms,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [
            ("default", &mut |x| no_out!(x.floor_sqrt())),
            ("binary", &mut |x| no_out!(floor_sqrt_binary(&x))),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_natural_floor_sqrt_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.floor_sqrt()",
        BenchmarkType::LibraryComparison,
        natural_gen_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_natural_bit_bucketer("x"),
        &mut [
            ("num", &mut |(x, _, _)| no_out!(x.sqrt())),
            ("rug", &mut |(_, x, _)| no_out!(x.sqrt())),
            ("Malachite", &mut |(_, _, x)| no_out!(x.floor_sqrt())),
        ],
    );
}

fn benchmark_natural_floor_sqrt_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.floor_sqrt_assign()",
        BenchmarkType::Single,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [("Malachite", &mut |mut x| x.floor_sqrt_assign())],
    );
}

fn benchmark_natural_ceiling_sqrt_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.ceiling_sqrt()",
        BenchmarkType::EvaluationStrategy,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [
            ("Natural.ceiling_sqrt()", &mut |x| no_out!(x.ceiling_sqrt())),
            ("(&Natural).ceiling_sqrt()", &mut |x| {
                no_out!((&x).ceiling_sqrt())
            }),
        ],
    );
}

fn benchmark_natural_ceiling_sqrt_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.ceiling_sqrt()",
        BenchmarkType::Algorithms,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [
            ("default", &mut |x| no_out!(x.ceiling_sqrt())),
            ("binary", &mut |x| no_out!(ceiling_sqrt_binary(&x))),
        ],
    );
}

fn benchmark_natural_ceiling_sqrt_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.ceiling_sqrt_assign()",
        BenchmarkType::Single,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [("Malachite", &mut |mut x| x.ceiling_sqrt_assign())],
    );
}

fn benchmark_natural_checked_sqrt_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.checked_sqrt()",
        BenchmarkType::EvaluationStrategy,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [
            ("Natural.checked_sqrt()", &mut |x| no_out!(x.checked_sqrt())),
            ("(&Natural).checked_sqrt()", &mut |x| {
                no_out!((&x).checked_sqrt())
            }),
        ],
    );
}

fn benchmark_natural_checked_sqrt_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.checked_sqrt()",
        BenchmarkType::Algorithms,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [
            ("default", &mut |x| no_out!(x.checked_sqrt())),
            ("binary", &mut |x| no_out!(checked_sqrt_binary(&x))),
        ],
    );
}

fn benchmark_natural_sqrt_rem_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.sqrt_rem()",
        BenchmarkType::EvaluationStrategy,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [
            ("Natural.sqrt_rem()", &mut |x| no_out!(x.sqrt_rem())),
            ("(&Natural).sqrt_rem()", &mut |x| no_out!((&x).sqrt_rem())),
        ],
    );
}

#[allow(clippy::no_effect)]
fn benchmark_natural_sqrt_rem_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.sqrt_rem()",
        BenchmarkType::Algorithms,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [
            ("default", &mut |x| no_out!(x.sqrt_rem())),
            ("floor and subtraction", &mut |x| {
                let sqrt = (&x).floor_sqrt();
                let square = (&sqrt).square();
                (sqrt, x - square);
            }),
            ("binary", &mut |x| no_out!(sqrt_rem_binary(&x))),
        ],
    );
}

fn benchmark_natural_sqrt_rem_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.sqrt_rem()",
        BenchmarkType::LibraryComparison,
        natural_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("x"),
        &mut [
            (
                "rug",
                &mut |(x, _)| no_out!(x.sqrt_rem(rug::Integer::new())),
            ),
            ("Malachite", &mut |(_, x)| no_out!(x.sqrt_rem())),
        ],
    );
}

fn benchmark_natural_sqrt_assign_rem(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.sqrt_assign_rem()",
        BenchmarkType::Single,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [("Malachite", &mut |mut x| no_out!(x.sqrt_assign_rem()))],
    );
}
