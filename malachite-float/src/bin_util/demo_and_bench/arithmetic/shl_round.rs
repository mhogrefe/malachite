// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ShlRound, ShlRoundAssign};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::arithmetic::shl_round::{
    rug_shl_prec_round_signed, rug_shl_prec_round_unsigned, rug_shl_prec_signed,
    rug_shl_prec_unsigned, rug_shl_round_signed, rug_shl_round_unsigned, shl_prec_naive,
    shl_prec_round_naive, shl_round_naive,
};
use malachite_float::test_util::bench::bucketers::{
    pair_2_quadruple_1_float_complexity_bucketer, pair_2_triple_1_float_complexity_bucketer,
    quadruple_1_float_complexity_bucketer, triple_1_float_complexity_bucketer,
};
use malachite_float::test_util::generators::{
    float_signed_rounding_mode_triple_gen_var_1, float_signed_rounding_mode_triple_gen_var_2,
    float_signed_rounding_mode_triple_gen_var_3_rm,
    float_signed_unsigned_rounding_mode_quadruple_gen_var_1,
    float_signed_unsigned_rounding_mode_quadruple_gen_var_1_rm,
    float_signed_unsigned_rounding_mode_quadruple_gen_var_2,
    float_signed_unsigned_triple_gen_var_1, float_signed_unsigned_triple_gen_var_1_rm,
    float_signed_unsigned_triple_gen_var_2, float_unsigned_rounding_mode_triple_gen_var_5,
    float_unsigned_rounding_mode_triple_gen_var_6,
    float_unsigned_rounding_mode_triple_gen_var_7_rm,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_1,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_1_rm,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_2,
    float_unsigned_unsigned_triple_gen_var_1, float_unsigned_unsigned_triple_gen_var_1_rm,
    float_unsigned_unsigned_triple_gen_var_2,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use std::ops::Shl;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_float_shl_prec_assign_unsigned);
    register_unsigned_demos!(runner, demo_float_shl_prec_assign_unsigned_debug);
    register_signed_demos!(runner, demo_float_shl_prec_assign_signed);
    register_signed_demos!(runner, demo_float_shl_prec_assign_signed_debug);
    register_unsigned_demos!(runner, demo_float_shl_prec_unsigned);
    register_unsigned_demos!(runner, demo_float_shl_prec_unsigned_debug);
    register_unsigned_demos!(runner, demo_float_shl_prec_unsigned_extreme);
    register_unsigned_demos!(runner, demo_float_shl_prec_unsigned_extreme_debug);
    register_signed_demos!(runner, demo_float_shl_prec_signed);
    register_signed_demos!(runner, demo_float_shl_prec_signed_debug);
    register_signed_demos!(runner, demo_float_shl_prec_signed_extreme);
    register_signed_demos!(runner, demo_float_shl_prec_signed_extreme_debug);
    register_unsigned_demos!(runner, demo_float_shl_prec_unsigned_ref);
    register_unsigned_demos!(runner, demo_float_shl_prec_unsigned_ref_debug);
    register_signed_demos!(runner, demo_float_shl_prec_signed_ref);
    register_signed_demos!(runner, demo_float_shl_prec_signed_ref_debug);
    register_unsigned_demos!(runner, demo_float_shl_prec_round_assign_unsigned);
    register_unsigned_demos!(runner, demo_float_shl_prec_round_assign_unsigned_debug);
    register_signed_demos!(runner, demo_float_shl_prec_round_assign_signed);
    register_signed_demos!(runner, demo_float_shl_prec_round_assign_signed_debug);
    register_unsigned_demos!(runner, demo_float_shl_prec_round_unsigned);
    register_unsigned_demos!(runner, demo_float_shl_prec_round_unsigned_debug);
    register_unsigned_demos!(runner, demo_float_shl_prec_round_unsigned_extreme);
    register_unsigned_demos!(runner, demo_float_shl_prec_round_unsigned_extreme_debug);
    register_signed_demos!(runner, demo_float_shl_prec_round_signed);
    register_signed_demos!(runner, demo_float_shl_prec_round_signed_debug);
    register_signed_demos!(runner, demo_float_shl_prec_round_signed_extreme);
    register_signed_demos!(runner, demo_float_shl_prec_round_signed_extreme_debug);
    register_unsigned_demos!(runner, demo_float_shl_prec_round_unsigned_ref);
    register_unsigned_demos!(runner, demo_float_shl_prec_round_unsigned_ref_debug);
    register_signed_demos!(runner, demo_float_shl_prec_round_signed_ref);
    register_signed_demos!(runner, demo_float_shl_prec_round_signed_ref_debug);
    register_unsigned_demos!(runner, demo_float_shl_round_assign_unsigned);
    register_unsigned_demos!(runner, demo_float_shl_round_assign_unsigned_debug);
    register_signed_demos!(runner, demo_float_shl_round_assign_signed);
    register_signed_demos!(runner, demo_float_shl_round_assign_signed_debug);
    register_unsigned_demos!(runner, demo_float_shl_round_unsigned);
    register_unsigned_demos!(runner, demo_float_shl_round_unsigned_debug);
    register_unsigned_demos!(runner, demo_float_shl_round_unsigned_extreme);
    register_unsigned_demos!(runner, demo_float_shl_round_unsigned_extreme_debug);
    register_signed_demos!(runner, demo_float_shl_round_signed);
    register_signed_demos!(runner, demo_float_shl_round_signed_debug);
    register_signed_demos!(runner, demo_float_shl_round_signed_extreme);
    register_signed_demos!(runner, demo_float_shl_round_signed_extreme_debug);
    register_unsigned_demos!(runner, demo_float_shl_round_unsigned_ref);
    register_unsigned_demos!(runner, demo_float_shl_round_unsigned_ref_debug);
    register_signed_demos!(runner, demo_float_shl_round_signed_ref);
    register_signed_demos!(runner, demo_float_shl_round_signed_ref_debug);

    register_unsigned_benches!(runner, benchmark_float_shl_prec_assign_unsigned);
    register_signed_benches!(runner, benchmark_float_shl_prec_assign_signed);
    register_unsigned_benches!(
        runner,
        benchmark_float_shl_prec_unsigned_evaluation_strategy
    );
    register_signed_benches!(runner, benchmark_float_shl_prec_signed_evaluation_strategy);
    register_unsigned_benches!(runner, benchmark_float_shl_prec_unsigned_algorithms);
    register_signed_benches!(runner, benchmark_float_shl_prec_signed_algorithms);
    register_bench!(
        runner,
        benchmark_float_shl_prec_assign_u32_library_comparison
    );
    register_bench!(
        runner,
        benchmark_float_shl_prec_assign_i32_library_comparison
    );
    register_unsigned_benches!(runner, benchmark_float_shl_prec_round_assign_unsigned);
    register_signed_benches!(runner, benchmark_float_shl_prec_round_assign_signed);
    register_unsigned_benches!(
        runner,
        benchmark_float_shl_prec_round_unsigned_evaluation_strategy
    );
    register_signed_benches!(
        runner,
        benchmark_float_shl_prec_round_signed_evaluation_strategy
    );
    register_unsigned_benches!(runner, benchmark_float_shl_prec_round_unsigned_algorithms);
    register_signed_benches!(runner, benchmark_float_shl_prec_round_signed_algorithms);
    register_bench!(
        runner,
        benchmark_float_shl_prec_round_assign_u32_library_comparison
    );
    register_bench!(
        runner,
        benchmark_float_shl_prec_round_assign_i32_library_comparison
    );
    register_unsigned_benches!(runner, benchmark_float_shl_round_assign_unsigned);
    register_signed_benches!(runner, benchmark_float_shl_round_assign_signed);
    register_unsigned_benches!(
        runner,
        benchmark_float_shl_round_unsigned_evaluation_strategy
    );
    register_signed_benches!(runner, benchmark_float_shl_round_signed_evaluation_strategy);
    register_unsigned_benches!(runner, benchmark_float_shl_round_unsigned_algorithms);
    register_signed_benches!(runner, benchmark_float_shl_round_signed_algorithms);
    register_bench!(
        runner,
        benchmark_float_shl_round_assign_u32_library_comparison
    );
    register_bench!(
        runner,
        benchmark_float_shl_round_assign_i32_library_comparison
    );
}

fn demo_float_shl_prec_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut n, u, prec) in float_unsigned_unsigned_triple_gen_var_1::<T, _>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        let o = n.shl_prec_assign(u, prec);
        println!("x := {n_old}; x.shl_prec_assign({u}, {prec}) = {o:?}; x = {n}");
    }
}

fn demo_float_shl_prec_assign_unsigned_debug<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut n, u, prec) in float_unsigned_unsigned_triple_gen_var_1::<T, _>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        let o = n.shl_prec_assign(u, prec);
        println!(
            "x := {:#x}; x.shl_prec_assign({}, {}) = {:?}; x = {:#x}",
            ComparableFloat(n_old),
            u,
            prec,
            o,
            ComparableFloat(n)
        );
    }
}

fn demo_float_shl_prec_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut n, u, prec) in float_signed_unsigned_triple_gen_var_1::<T, _>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        let o = n.shl_prec_assign(u, prec);
        println!("x := {n_old}; x.shl_prec_assign({u}, {prec}) = {o:?}; x = {n}");
    }
}

fn demo_float_shl_prec_assign_signed_debug<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut n, u, prec) in float_signed_unsigned_triple_gen_var_1::<T, _>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        let o = n.shl_prec_assign(u, prec);
        println!(
            "x := {:#x}; x.shl_prec_assign({}, {}) = {:?}; x = {:#x}",
            ComparableFloat(n_old),
            u,
            prec,
            o,
            ComparableFloat(n)
        );
    }
}

fn demo_float_shl_prec_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, u, prec) in float_unsigned_unsigned_triple_gen_var_1::<T, _>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "{}.shl_prec({}, {}) = {:?}",
            n_old,
            u,
            prec,
            n.shl_prec(u, prec)
        );
    }
}

fn demo_float_shl_prec_unsigned_debug<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, u, prec) in float_unsigned_unsigned_triple_gen_var_1::<T, _>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        let (shifted, o) = n.shl_prec(u, prec);
        println!(
            "{:#x}.shl_prec({}, {}) = ({:#x}, {:?})",
            ComparableFloat(n_old),
            u,
            prec,
            ComparableFloat(shifted),
            o
        );
    }
}

fn demo_float_shl_prec_unsigned_extreme<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, u, prec) in float_unsigned_unsigned_triple_gen_var_2::<T, _>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "{}.shl_prec({}, {}) = {:?}",
            n_old,
            u,
            prec,
            n.shl_prec(u, prec)
        );
    }
}

fn demo_float_shl_prec_unsigned_extreme_debug<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, u, prec) in float_unsigned_unsigned_triple_gen_var_2::<T, _>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        let (shifted, o) = n.shl_prec(u, prec);
        println!(
            "{:#x}.shl_prec({}, {}) = ({:#x}, {:?})",
            ComparableFloat(n_old),
            u,
            prec,
            ComparableFloat(shifted),
            o
        );
    }
}

fn demo_float_shl_prec_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, u, prec) in float_signed_unsigned_triple_gen_var_1::<T, _>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "{}.shl_prec({}, {}) = {:?}",
            n_old,
            u,
            prec,
            n.shl_prec(u, prec)
        );
    }
}

fn demo_float_shl_prec_signed_debug<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, u, prec) in float_signed_unsigned_triple_gen_var_1::<T, _>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        let (shifted, o) = n.shl_prec(u, prec);
        println!(
            "{:#x}.shl_prec({}, {}) = ({:#x}, {:?})",
            ComparableFloat(n_old),
            u,
            prec,
            ComparableFloat(shifted),
            o
        );
    }
}

fn demo_float_shl_prec_signed_extreme<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, u, prec) in float_signed_unsigned_triple_gen_var_2::<T, _>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "{}.shl_prec({}, {}) = {:?}",
            n_old,
            u,
            prec,
            n.shl_prec(u, prec)
        );
    }
}

fn demo_float_shl_prec_signed_extreme_debug<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, u, prec) in float_signed_unsigned_triple_gen_var_2::<T, _>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        let (shifted, o) = n.shl_prec(u, prec);
        println!(
            "{:#x}.shl_prec({}, {}) = ({:#x}, {:?})",
            ComparableFloat(n_old),
            u,
            prec,
            ComparableFloat(shifted),
            o
        );
    }
}

fn demo_float_shl_prec_unsigned_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, u, prec) in float_unsigned_unsigned_triple_gen_var_1::<T, _>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.shl_prec_ref({}, {}) = {:?}",
            n,
            u,
            prec,
            n.shl_prec_ref(u, prec)
        );
    }
}

fn demo_float_shl_prec_unsigned_ref_debug<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, u, prec) in float_unsigned_unsigned_triple_gen_var_1::<T, _>()
        .get(gm, config)
        .take(limit)
    {
        let (shifted, o) = n.shl_prec_ref(u, prec);
        println!(
            "{:#x}.shl_prec_ref({}, {}) = ({:#x}, {:?})",
            ComparableFloat(n),
            u,
            prec,
            ComparableFloat(shifted),
            o
        );
    }
}

fn demo_float_shl_prec_signed_ref<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, u, prec) in float_signed_unsigned_triple_gen_var_1::<T, _>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.shl_prec_ref({}, {}) = {:?}",
            n,
            u,
            prec,
            n.shl_prec_ref(u, prec)
        );
    }
}

fn demo_float_shl_prec_signed_ref_debug<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, u, prec) in float_signed_unsigned_triple_gen_var_1::<T, _>()
        .get(gm, config)
        .take(limit)
    {
        let (shifted, o) = n.shl_prec_ref(u, prec);
        println!(
            "{:#x}.shl_prec_ref({}, {}) = ({:#x}, {:?})",
            ComparableFloat(n),
            u,
            prec,
            ComparableFloat(shifted),
            o
        );
    }
}

fn demo_float_shl_prec_round_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut n, u, prec, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        let o = n.shl_prec_round_assign(u, prec, rm);
        println!("x := {n_old}; x.shl_prec_round_assign({u}, {prec}, {rm}) = {o:?}; x = {n}");
    }
}

fn demo_float_shl_prec_round_assign_unsigned_debug<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut n, u, prec, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        let o = n.shl_prec_round_assign(u, prec, rm);
        println!(
            "x := {:#x}; x.shl_prec_round_assign({}, {}, {}) = {:?}; x = {:#x}",
            ComparableFloat(n_old),
            u,
            prec,
            rm,
            o,
            ComparableFloat(n)
        );
    }
}

fn demo_float_shl_prec_round_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut n, u, prec, rm) in float_signed_unsigned_rounding_mode_quadruple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        let o = n.shl_prec_round_assign(u, prec, rm);
        println!("x := {n_old}; x.shl_prec_round_assign({u}, {prec}, {rm}) = {o:?}; x = {n}");
    }
}

fn demo_float_shl_prec_round_assign_signed_debug<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut n, u, prec, rm) in float_signed_unsigned_rounding_mode_quadruple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        let o = n.shl_prec_round_assign(u, prec, rm);
        println!(
            "x := {:#x}; x.shl_prec_round_assign({}, {}, {}) = {:?}; x = {:#x}",
            ComparableFloat(n_old),
            u,
            prec,
            rm,
            o,
            ComparableFloat(n)
        );
    }
}

fn demo_float_shl_prec_round_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, u, prec, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "{}.shl_prec_round({}, {}, {}) = {:?}",
            n_old,
            u,
            prec,
            rm,
            n.shl_prec_round(u, prec, rm)
        );
    }
}

fn demo_float_shl_prec_round_unsigned_debug<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, u, prec, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        let (shifted, o) = n.shl_prec_round(u, prec, rm);
        println!(
            "{:#x}.shl_prec_round({}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(n_old),
            u,
            prec,
            rm,
            ComparableFloat(shifted),
            o
        );
    }
}

fn demo_float_shl_prec_round_unsigned_extreme<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, u, prec, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "{}.shl_prec_round({}, {}, {}) = {:?}",
            n_old,
            u,
            prec,
            rm,
            n.shl_prec_round(u, prec, rm)
        );
    }
}

fn demo_float_shl_prec_round_unsigned_extreme_debug<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, u, prec, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        let (shifted, o) = n.shl_prec_round(u, prec, rm);
        println!(
            "{:#x}.shl_prec_round({}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(n_old),
            u,
            prec,
            rm,
            ComparableFloat(shifted),
            o
        );
    }
}

fn demo_float_shl_prec_round_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, u, prec, rm) in float_signed_unsigned_rounding_mode_quadruple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "{}.shl_prec_round({}, {}, {}) = {:?}",
            n_old,
            u,
            prec,
            rm,
            n.shl_prec_round(u, prec, rm)
        );
    }
}

fn demo_float_shl_prec_round_signed_debug<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, u, prec, rm) in float_signed_unsigned_rounding_mode_quadruple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        let (shifted, o) = n.shl_prec_round(u, prec, rm);
        println!(
            "{:#x}.shl_prec_round({}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(n_old),
            u,
            prec,
            rm,
            ComparableFloat(shifted),
            o
        );
    }
}

fn demo_float_shl_prec_round_signed_extreme<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, u, prec, rm) in float_signed_unsigned_rounding_mode_quadruple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "{}.shl_prec_round({}, {}, {}) = {:?}",
            n_old,
            u,
            prec,
            rm,
            n.shl_prec_round(u, prec, rm)
        );
    }
}

fn demo_float_shl_prec_round_signed_extreme_debug<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, u, prec, rm) in float_signed_unsigned_rounding_mode_quadruple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        let (shifted, o) = n.shl_prec_round(u, prec, rm);
        println!(
            "{:#x}.shl_prec_round({}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(n_old),
            u,
            prec,
            rm,
            ComparableFloat(shifted),
            o
        );
    }
}

fn demo_float_shl_prec_round_unsigned_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, u, prec, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.shl_prec_round_ref({}, {}, {}) = {:?}",
            n,
            u,
            prec,
            rm,
            n.shl_prec_round_ref(u, prec, rm)
        );
    }
}

fn demo_float_shl_prec_round_unsigned_ref_debug<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, u, prec, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let (shifted, o) = n.shl_prec_round_ref(u, prec, rm);
        println!(
            "{:#x}.shl_prec_round_ref({}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(n),
            u,
            prec,
            rm,
            ComparableFloat(shifted),
            o
        );
    }
}

fn demo_float_shl_prec_round_signed_ref<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, u, prec, rm) in float_signed_unsigned_rounding_mode_quadruple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.shl_prec_round_ref({}, {}, {}) = {:?}",
            n,
            u,
            prec,
            rm,
            n.shl_prec_round_ref(u, prec, rm)
        );
    }
}

fn demo_float_shl_prec_round_signed_ref_debug<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, u, prec, rm) in float_signed_unsigned_rounding_mode_quadruple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let (shifted, o) = n.shl_prec_round_ref(u, prec, rm);
        println!(
            "{:#x}.shl_prec_round_ref({}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(n),
            u,
            prec,
            rm,
            ComparableFloat(shifted),
            o
        );
    }
}

fn demo_float_shl_round_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: ShlRoundAssign<T>,
{
    for (mut n, u, rm) in float_unsigned_rounding_mode_triple_gen_var_5::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        let o = n.shl_round_assign(u, rm);
        println!("x := {n_old}; x.shl_round_assign({u}, {rm}) = {o:?}; x = {n}");
    }
}

fn demo_float_shl_round_assign_unsigned_debug<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: ShlRoundAssign<T>,
{
    for (mut n, u, rm) in float_unsigned_rounding_mode_triple_gen_var_5::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        let o = n.shl_round_assign(u, rm);
        println!(
            "x := {:#x}; x.shl_round_assign({}, {}) = {:?}; x = {:#x}",
            ComparableFloat(n_old),
            u,
            rm,
            o,
            ComparableFloat(n)
        );
    }
}

fn demo_float_shl_round_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: ShlRoundAssign<T>,
{
    for (mut n, i, rm) in float_signed_rounding_mode_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        let o = n.shl_round_assign(i, rm);
        println!("x := {n_old}; x.shl_round_assign({i}, {rm}) = {o:?}; x = {n}");
    }
}

fn demo_float_shl_round_assign_signed_debug<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: ShlRoundAssign<T>,
{
    for (mut n, i, rm) in float_signed_rounding_mode_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        let o = n.shl_round_assign(i, rm);
        println!(
            "x := {:#x}; x.shl_round_assign({}, {}) = {:?}; x = {:#x}",
            ComparableFloat(n_old),
            i,
            rm,
            o,
            ComparableFloat(n)
        );
    }
}

fn demo_float_shl_round_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: ShlRound<T, Output = Float>,
{
    for (n, u, rm) in float_unsigned_rounding_mode_triple_gen_var_5::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "{}.shl_round({}, {}) = {:?}",
            n_old,
            u,
            rm,
            n.shl_round(u, rm)
        );
    }
}

fn demo_float_shl_round_unsigned_debug<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: ShlRound<T, Output = Float>,
{
    for (n, u, rm) in float_unsigned_rounding_mode_triple_gen_var_5::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        let (shifted, o) = n.shl_round(u, rm);
        println!(
            "{:#x}.shl_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(n_old),
            u,
            rm,
            ComparableFloat(shifted),
            o
        );
    }
}

fn demo_float_shl_round_unsigned_extreme<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: ShlRound<T, Output = Float>,
{
    for (n, u, rm) in float_unsigned_rounding_mode_triple_gen_var_6::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "{}.shl_round({}, {}) = {:?}",
            n_old,
            u,
            rm,
            n.shl_round(u, rm)
        );
    }
}

fn demo_float_shl_round_unsigned_extreme_debug<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: ShlRound<T, Output = Float>,
{
    for (n, u, rm) in float_unsigned_rounding_mode_triple_gen_var_6::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        let (shifted, o) = n.shl_round(u, rm);
        println!(
            "{:#x}.shl_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(n_old),
            u,
            rm,
            ComparableFloat(shifted),
            o
        );
    }
}

fn demo_float_shl_round_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: ShlRound<T, Output = Float>,
{
    for (n, i, rm) in float_signed_rounding_mode_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "{}.shl_round({}, {}) = {:?}",
            n_old,
            i,
            rm,
            n.shl_round(i, rm)
        );
    }
}

fn demo_float_shl_round_signed_debug<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: ShlRound<T, Output = Float>,
{
    for (n, i, rm) in float_signed_rounding_mode_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        let (shifted, o) = n.shl_round(i, rm);
        println!(
            "{:#x}.shl_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(n_old),
            i,
            rm,
            ComparableFloat(shifted),
            o
        );
    }
}

fn demo_float_shl_round_signed_extreme<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: ShlRound<T, Output = Float>,
{
    for (n, i, rm) in float_signed_rounding_mode_triple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "{}.shl_round({}, {}) = {:?}",
            n_old,
            i,
            rm,
            n.shl_round(i, rm)
        );
    }
}

fn demo_float_shl_round_signed_extreme_debug<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: ShlRound<T, Output = Float>,
{
    for (n, i, rm) in float_signed_rounding_mode_triple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        let (shifted, o) = n.shl_round(i, rm);
        println!(
            "{:#x}.shl_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(n_old),
            i,
            rm,
            ComparableFloat(shifted),
            o
        );
    }
}

fn demo_float_shl_round_unsigned_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a Float: ShlRound<T, Output = Float>,
{
    for (n, u, rm) in float_unsigned_rounding_mode_triple_gen_var_5::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).shl_round({}, {}) = {:?}",
            n,
            u,
            rm,
            (&n).shl_round(u, rm)
        );
    }
}

fn demo_float_shl_round_unsigned_ref_debug<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a Float: ShlRound<T, Output = Float>,
{
    for (n, u, rm) in float_unsigned_rounding_mode_triple_gen_var_5::<T>()
        .get(gm, config)
        .take(limit)
    {
        let (shifted, o) = (&n).shl_round(u, rm);
        println!(
            "(&{:#x}.shl_round({}, {}) = ({:#x}, {:?})",
            ComparableFloatRef(&n),
            u,
            rm,
            ComparableFloatRef(&shifted),
            o
        );
    }
}

fn demo_float_shl_round_signed_ref<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a Float: ShlRound<T, Output = Float>,
{
    for (n, i, rm) in float_signed_rounding_mode_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).shl_round({}, {}) = {:?}",
            n,
            i,
            rm,
            (&n).shl_round(i, rm)
        );
    }
}

fn demo_float_shl_round_signed_ref_debug<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a Float: ShlRound<T, Output = Float>,
{
    for (n, i, rm) in float_signed_rounding_mode_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let (shifted, o) = (&n).shl_round(i, rm);
        println!(
            "(&{:#x}).shl_round({}, {}) = ({:#x}, {:?})",
            ComparableFloatRef(&n),
            i,
            rm,
            ComparableFloatRef(&shifted),
            o
        );
    }
}

fn benchmark_float_shl_prec_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("Float.shl_prec_assign({}, u64)", T::NAME),
        BenchmarkType::Single,
        float_unsigned_unsigned_triple_gen_var_1::<T, _>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_float_complexity_bucketer("n"),
        &mut [("Malachite", &mut |(mut n, u, prec)| {
            no_out!(n.shl_prec_assign(u, prec));
        })],
    );
}

fn benchmark_float_shl_prec_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("Float.shl_prec_assign({}, u64)", T::NAME),
        BenchmarkType::Single,
        float_signed_unsigned_triple_gen_var_1::<T, _>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_float_complexity_bucketer("n"),
        &mut [("Malachite", &mut |(mut n, u, prec)| {
            no_out!(n.shl_prec_assign(u, prec));
        })],
    );
}

fn benchmark_float_shl_prec_unsigned_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("Float.shl_prec({}, u64)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        float_unsigned_unsigned_triple_gen_var_1::<T, _>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_float_complexity_bucketer("n"),
        &mut [
            (
                &format!("Float.shl_prec({}, u64)", T::NAME),
                &mut |(n, u, prec)| no_out!(n.shl_prec(u, prec)),
            ),
            (
                &format!("Float.shl_prec_ref({}, u64)", T::NAME),
                &mut |(n, u, prec)| no_out!(n.shl_prec_ref(u, prec)),
            ),
        ],
    );
}

fn benchmark_float_shl_prec_signed_evaluation_strategy<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("Float.shl_prec({}, u64)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        float_signed_unsigned_triple_gen_var_1::<T, _>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_float_complexity_bucketer("n"),
        &mut [
            (
                &format!("Float.shl_prec({}, u64)", T::NAME),
                &mut |(n, i, prec)| no_out!(n.shl_prec(i, prec)),
            ),
            (
                &format!("Float.shl_prec_ref({}, u64)", T::NAME),
                &mut |(n, i, prec)| no_out!(n.shl_prec_ref(i, prec)),
            ),
        ],
    );
}

fn benchmark_float_shl_prec_unsigned_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Rational: Shl<T, Output = Rational>,
{
    run_benchmark(
        &format!("Float.shl_prec({}, u64)", T::NAME),
        BenchmarkType::Algorithms,
        float_unsigned_unsigned_triple_gen_var_1::<T, _>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_float_complexity_bucketer("n"),
        &mut [
            ("default", &mut |(n, u, prec)| no_out!(n.shl_prec(u, prec))),
            ("naive", &mut |(n, u, prec)| {
                no_out!(shl_prec_naive(n, u, prec));
            }),
        ],
    );
}

fn benchmark_float_shl_prec_signed_algorithms<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Rational: Shl<T, Output = Rational>,
{
    run_benchmark(
        &format!("Float.shl_prec({}, u64)", T::NAME),
        BenchmarkType::Algorithms,
        float_signed_unsigned_triple_gen_var_1::<T, _>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_float_complexity_bucketer("n"),
        &mut [
            ("default", &mut |(n, u, prec)| no_out!(n.shl_prec(u, prec))),
            ("naive", &mut |(n, u, prec)| {
                no_out!(shl_prec_naive(n, u, prec));
            }),
        ],
    );
}

fn benchmark_float_shl_prec_assign_u32_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.shl_prec(i32, u64)",
        BenchmarkType::LibraryComparison,
        float_unsigned_unsigned_triple_gen_var_1_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_1_float_complexity_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, (n, u, prec))| {
                no_out!(n.shl_prec(u, prec));
            }),
            ("rug", &mut |((n, u, prec), _)| {
                no_out!(rug_shl_prec_unsigned(&n, u, prec));
            }),
        ],
    );
}

fn benchmark_float_shl_prec_assign_i32_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.shl_prec(i32, u64)",
        BenchmarkType::LibraryComparison,
        float_signed_unsigned_triple_gen_var_1_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_1_float_complexity_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, (n, i, prec))| {
                no_out!(n.shl_prec(i, prec));
            }),
            ("rug", &mut |((n, i, prec), _)| {
                no_out!(rug_shl_prec_signed(&n, i, prec));
            }),
        ],
    );
}

fn benchmark_float_shl_prec_round_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "Float.shl_prec_round_assign({}, u64, RoundingMode)",
            T::NAME
        ),
        BenchmarkType::Single,
        float_unsigned_unsigned_rounding_mode_quadruple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_float_complexity_bucketer("n"),
        &mut [("Malachite", &mut |(mut n, u, prec, rm)| {
            no_out!(n.shl_prec_round_assign(u, prec, rm));
        })],
    );
}

fn benchmark_float_shl_prec_round_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "Float.shl_prec_round_assign({}, u64, RoundingMode)",
            T::NAME
        ),
        BenchmarkType::Single,
        float_signed_unsigned_rounding_mode_quadruple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_float_complexity_bucketer("n"),
        &mut [("Malachite", &mut |(mut n, u, prec, rm)| {
            no_out!(n.shl_prec_round_assign(u, prec, rm));
        })],
    );
}

fn benchmark_float_shl_prec_round_unsigned_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("Float.shl_prec_round({}, u64, RoundingMode)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        float_unsigned_unsigned_rounding_mode_quadruple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_float_complexity_bucketer("n"),
        &mut [
            (
                &format!("Float.shl_prec_round({}, u64, RoundingMode)", T::NAME),
                &mut |(n, u, prec, rm)| no_out!(n.shl_prec_round(u, prec, rm)),
            ),
            (
                &format!("Float.shl_prec_round_ref({}, u64, RoundingMode)", T::NAME),
                &mut |(n, u, prec, rm)| no_out!(n.shl_prec_round_ref(u, prec, rm)),
            ),
        ],
    );
}

fn benchmark_float_shl_prec_round_signed_evaluation_strategy<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("Float.shl_prec_round({}, u64, RoundingMode)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        float_signed_unsigned_rounding_mode_quadruple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_float_complexity_bucketer("n"),
        &mut [
            (
                &format!("Float.shl_prec_round({}, u64, RoundingMode)", T::NAME),
                &mut |(n, i, prec, rm)| no_out!(n.shl_prec_round(i, prec, rm)),
            ),
            (
                &format!("Float.shl_prec_round_ref({}, u64, RoundingMode)", T::NAME),
                &mut |(n, i, prec, rm)| no_out!(n.shl_prec_round_ref(i, prec, rm)),
            ),
        ],
    );
}

fn benchmark_float_shl_prec_round_unsigned_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Rational: Shl<T, Output = Rational>,
{
    run_benchmark(
        &format!("Float.shl_prec_round({}, u64, RoundingMode)", T::NAME),
        BenchmarkType::Algorithms,
        float_unsigned_unsigned_rounding_mode_quadruple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_float_complexity_bucketer("n"),
        &mut [
            ("default", &mut |(n, u, prec, rm)| {
                no_out!(n.shl_prec_round(u, prec, rm));
            }),
            ("naive", &mut |(n, u, prec, rm)| {
                no_out!(shl_prec_round_naive(n, u, prec, rm));
            }),
        ],
    );
}

fn benchmark_float_shl_prec_round_signed_algorithms<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Rational: Shl<T, Output = Rational>,
{
    run_benchmark(
        &format!("Float.shl_prec_round({}, u64, RoundingMode)", T::NAME),
        BenchmarkType::Algorithms,
        float_signed_unsigned_rounding_mode_quadruple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_float_complexity_bucketer("n"),
        &mut [
            ("default", &mut |(n, u, prec, rm)| {
                no_out!(n.shl_prec_round(u, prec, rm));
            }),
            ("naive", &mut |(n, u, prec, rm)| {
                no_out!(shl_prec_round_naive(n, u, prec, rm));
            }),
        ],
    );
}

fn benchmark_float_shl_prec_round_assign_u32_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.shl_prec_round(i32, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_unsigned_unsigned_rounding_mode_quadruple_gen_var_1_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_float_complexity_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, (n, u, prec, rm))| {
                no_out!(n.shl_prec_round(u, prec, rm));
            }),
            ("rug", &mut |((n, u, prec, rm), _)| {
                no_out!(rug_shl_prec_round_unsigned(&n, u, prec, rm));
            }),
        ],
    );
}

fn benchmark_float_shl_prec_round_assign_i32_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.shl_prec_round(i32, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_signed_unsigned_rounding_mode_quadruple_gen_var_1_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_float_complexity_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, (n, i, prec, rm))| {
                no_out!(n.shl_prec_round(i, prec, rm));
            }),
            ("rug", &mut |((n, i, prec, rm), _)| {
                no_out!(rug_shl_prec_round_signed(&n, i, prec, rm));
            }),
        ],
    );
}

fn benchmark_float_shl_round_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: ShlRoundAssign<T>,
{
    run_benchmark(
        &format!("Float.shl_round_assign({}, RoundingMode)", T::NAME),
        BenchmarkType::Single,
        float_unsigned_rounding_mode_triple_gen_var_5::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_float_complexity_bucketer("n"),
        &mut [("Malachite", &mut |(mut n, u, rm)| {
            no_out!(n.shl_round_assign(u, rm));
        })],
    );
}

fn benchmark_float_shl_round_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: ShlRoundAssign<T>,
{
    run_benchmark(
        &format!("Float.shl_round_assign({}, RoundingMode)", T::NAME),
        BenchmarkType::Single,
        float_signed_rounding_mode_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_float_complexity_bucketer("n"),
        &mut [("Malachite", &mut |(mut n, u, rm)| {
            no_out!(n.shl_round_assign(u, rm));
        })],
    );
}

fn benchmark_float_shl_round_unsigned_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: ShlRound<T>,
    for<'a> &'a Float: ShlRound<T>,
{
    run_benchmark(
        &format!("Float.shl_round({}, RoundingMode)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        float_unsigned_rounding_mode_triple_gen_var_5::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_float_complexity_bucketer("n"),
        &mut [
            (
                &format!("Float.shl_round({}, RoundingMode)", T::NAME),
                &mut |(n, u, rm)| no_out!(n.shl_round(u, rm)),
            ),
            (
                &format!("(&Float).shl_round({}, RoundingMode)", T::NAME),
                &mut |(n, u, rm)| no_out!((&n).shl_round(u, rm)),
            ),
        ],
    );
}

fn benchmark_float_shl_round_signed_evaluation_strategy<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: ShlRound<T>,
    for<'a> &'a Float: ShlRound<T>,
{
    run_benchmark(
        &format!("Float.shl_round({}, RoundingMode)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        float_signed_rounding_mode_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_float_complexity_bucketer("n"),
        &mut [
            (
                &format!("Float.shl_round({}, RoundingMode)", T::NAME),
                &mut |(n, i, rm)| no_out!(n.shl_round(i, rm)),
            ),
            (
                &format!("(&Float).shl_round({}, RoundingMode)", T::NAME),
                &mut |(n, i, rm)| no_out!((&n).shl_round(i, rm)),
            ),
        ],
    );
}

fn benchmark_float_shl_round_unsigned_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: ShlRound<T>,
    Rational: Shl<T, Output = Rational>,
    for<'a> &'a Float: ShlRound<T>,
{
    run_benchmark(
        &format!("Float.shl_round({}, RoundingMode)", T::NAME),
        BenchmarkType::Algorithms,
        float_unsigned_rounding_mode_triple_gen_var_5::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_float_complexity_bucketer("n"),
        &mut [
            ("default", &mut |(n, u, rm)| no_out!(n.shl_round(u, rm))),
            ("naive", &mut |(n, u, rm)| {
                no_out!(shl_round_naive(n, u, rm));
            }),
        ],
    );
}

fn benchmark_float_shl_round_signed_algorithms<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: ShlRound<T>,
    Rational: Shl<T, Output = Rational>,
    for<'a> &'a Float: ShlRound<T>,
{
    run_benchmark(
        &format!("Float.shl_round({}, RoundingMode)", T::NAME),
        BenchmarkType::Algorithms,
        float_signed_rounding_mode_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_float_complexity_bucketer("n"),
        &mut [
            ("default", &mut |(n, u, rm)| no_out!(n.shl_round(u, rm))),
            ("naive", &mut |(n, u, rm)| {
                no_out!(shl_round_naive(n, u, rm));
            }),
        ],
    );
}

fn benchmark_float_shl_round_assign_u32_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.shl_round(i32, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_unsigned_rounding_mode_triple_gen_var_7_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_1_float_complexity_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, (n, u, rm))| {
                no_out!((&n).shl_round(u, rm));
            }),
            ("rug", &mut |((n, u, rm), _)| {
                no_out!(rug_shl_round_unsigned(&n, u, rm));
            }),
        ],
    );
}

fn benchmark_float_shl_round_assign_i32_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.shl_round(i32, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_signed_rounding_mode_triple_gen_var_3_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_1_float_complexity_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, (n, i, rm))| {
                no_out!((&n).shl_round(i, rm));
            }),
            ("rug", &mut |((n, i, rm), _)| {
                no_out!(rug_shl_round_signed(&n, i, rm));
            }),
        ],
    );
}
