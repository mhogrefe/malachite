use std::cmp::max;

use malachite_base::named::Named;
use malachite_base::num::arithmetic::traits::{ShrRound, ShrRoundAssign, UnsignedAbs};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_nz::natural::arithmetic::shr_round::{
    limbs_shr_exact, limbs_shr_round, limbs_shr_round_nearest, limbs_shr_round_up,
    limbs_vec_shr_exact_in_place, limbs_vec_shr_round_in_place,
    limbs_vec_shr_round_nearest_in_place, limbs_vec_shr_round_up_in_place,
};
use malachite_nz::platform::Limb;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    pairs_of_unsigned_vec_and_small_unsigned, pairs_of_unsigned_vec_and_small_unsigned_var_1,
    triples_of_unsigned_vec_small_unsigned_and_rounding_mode_var_1,
};
use malachite_test::inputs::natural::{
    triples_of_natural_small_signed_and_rounding_mode_var_2,
    triples_of_natural_small_unsigned_and_rounding_mode_var_1,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_shr_round_up);
    register_demo!(registry, demo_limbs_shr_round_nearest);
    register_demo!(registry, demo_limbs_shr_exact);
    register_demo!(registry, demo_limbs_shr_round);
    register_demo!(registry, demo_limbs_vec_shr_round_up_in_place);
    register_demo!(registry, demo_limbs_vec_shr_round_nearest_in_place);
    register_demo!(registry, demo_limbs_vec_shr_exact_in_place);
    register_demo!(registry, demo_limbs_vec_shr_round_in_place);

    register_demo!(registry, demo_natural_shr_round_assign_i8);
    register_demo!(registry, demo_natural_shr_round_assign_i16);
    register_demo!(registry, demo_natural_shr_round_assign_i32);
    register_demo!(registry, demo_natural_shr_round_assign_i64);
    register_demo!(registry, demo_natural_shr_round_assign_isize);
    register_demo!(registry, demo_natural_shr_round_assign_u8);
    register_demo!(registry, demo_natural_shr_round_assign_u16);
    register_demo!(registry, demo_natural_shr_round_assign_u32);
    register_demo!(registry, demo_natural_shr_round_assign_u64);
    register_demo!(registry, demo_natural_shr_round_assign_usize);

    register_demo!(registry, demo_natural_shr_round_i8);
    register_demo!(registry, demo_natural_shr_round_i16);
    register_demo!(registry, demo_natural_shr_round_i32);
    register_demo!(registry, demo_natural_shr_round_i64);
    register_demo!(registry, demo_natural_shr_round_isize);
    register_demo!(registry, demo_natural_shr_round_u8);
    register_demo!(registry, demo_natural_shr_round_u16);
    register_demo!(registry, demo_natural_shr_round_u32);
    register_demo!(registry, demo_natural_shr_round_u64);
    register_demo!(registry, demo_natural_shr_round_usize);

    register_demo!(registry, demo_natural_shr_round_i8_ref);
    register_demo!(registry, demo_natural_shr_round_i16_ref);
    register_demo!(registry, demo_natural_shr_round_i32_ref);
    register_demo!(registry, demo_natural_shr_round_i64_ref);
    register_demo!(registry, demo_natural_shr_round_isize_ref);
    register_demo!(registry, demo_natural_shr_round_u8_ref);
    register_demo!(registry, demo_natural_shr_round_u16_ref);
    register_demo!(registry, demo_natural_shr_round_u32_ref);
    register_demo!(registry, demo_natural_shr_round_u64_ref);
    register_demo!(registry, demo_natural_shr_round_usize_ref);

    register_bench!(registry, Small, benchmark_limbs_shr_round_up);
    register_bench!(registry, Small, benchmark_limbs_shr_round_nearest);
    register_bench!(registry, Small, benchmark_limbs_shr_exact);
    register_bench!(registry, Small, benchmark_limbs_shr_round);
    register_bench!(registry, Small, benchmark_limbs_vec_shr_round_up_in_place);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_vec_shr_round_nearest_in_place
    );
    register_bench!(registry, Small, benchmark_limbs_vec_shr_exact_in_place);
    register_bench!(registry, Small, benchmark_limbs_vec_shr_round_in_place);

    register_bench!(registry, Large, benchmark_natural_shr_round_assign_u8);
    register_bench!(registry, Large, benchmark_natural_shr_round_assign_u16);
    register_bench!(registry, Large, benchmark_natural_shr_round_assign_u32);
    register_bench!(registry, Large, benchmark_natural_shr_round_assign_u64);
    register_bench!(registry, Large, benchmark_natural_shr_round_assign_usize);
    register_bench!(registry, Large, benchmark_natural_shr_round_assign_i8);
    register_bench!(registry, Large, benchmark_natural_shr_round_assign_i16);
    register_bench!(registry, Large, benchmark_natural_shr_round_assign_i32);
    register_bench!(registry, Large, benchmark_natural_shr_round_assign_i64);
    register_bench!(registry, Large, benchmark_natural_shr_round_assign_isize);

    register_bench!(
        registry,
        Large,
        benchmark_natural_shr_round_u8_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shr_round_u16_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shr_round_u32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shr_round_u64_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shr_round_usize_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shr_round_i8_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shr_round_i16_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shr_round_i32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shr_round_i64_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shr_round_isize_evaluation_strategy
    );
}

fn demo_limbs_shr_round_up(gm: GenerationMode, limit: usize) {
    for (limbs, bits) in pairs_of_unsigned_vec_and_small_unsigned_var_1(gm).take(limit) {
        println!(
            "limbs_shr_round_up({:?}, {}) = {:?}",
            limbs,
            bits,
            limbs_shr_round_up(&limbs, bits)
        );
    }
}

fn demo_limbs_shr_round_nearest(gm: GenerationMode, limit: usize) {
    for (limbs, bits) in pairs_of_unsigned_vec_and_small_unsigned(gm).take(limit) {
        println!(
            "limbs_shr_round_nearest({:?}, {}) = {:?}",
            limbs,
            bits,
            limbs_shr_round_nearest(&limbs, bits)
        );
    }
}

fn demo_limbs_shr_exact(gm: GenerationMode, limit: usize) {
    for (limbs, bits) in pairs_of_unsigned_vec_and_small_unsigned_var_1(gm).take(limit) {
        println!(
            "limbs_shr_exact({:?}, {}) = {:?}",
            limbs,
            bits,
            limbs_shr_exact(&limbs, bits)
        );
    }
}

fn demo_limbs_shr_round(gm: GenerationMode, limit: usize) {
    for (limbs, bits, rm) in
        triples_of_unsigned_vec_small_unsigned_and_rounding_mode_var_1(gm).take(limit)
    {
        println!(
            "limbs_shr_round({:?}, {}, {}) = {:?}",
            limbs,
            bits,
            rm,
            limbs_shr_round(&limbs, bits, rm)
        );
    }
}

fn demo_limbs_vec_shr_round_up_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, bits) in pairs_of_unsigned_vec_and_small_unsigned_var_1(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let limbs_old = limbs.clone();
        limbs_vec_shr_round_up_in_place(&mut limbs, bits);
        println!(
            "limbs := {:?}; limbs_vec_shr_round_up_in_place(&mut limbs, {}); limbs = {:?}",
            limbs_old, bits, limbs
        );
    }
}

fn demo_limbs_vec_shr_round_nearest_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, bits) in pairs_of_unsigned_vec_and_small_unsigned(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let limbs_old = limbs.clone();
        limbs_vec_shr_round_nearest_in_place(&mut limbs, bits);
        println!(
            "limbs := {:?}; limbs_vec_shr_round_nearest_in_place(&mut limbs, {}); limbs = {:?}",
            limbs_old, bits, limbs
        );
    }
}

fn demo_limbs_vec_shr_exact_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, bits) in pairs_of_unsigned_vec_and_small_unsigned_var_1(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let limbs_old = limbs.clone();
        let result = limbs_vec_shr_exact_in_place(&mut limbs, bits);
        println!(
            "limbs := {:?}; limbs_vec_shr_exact_in_place(&mut limbs, {}) = {}; limbs = {:?}",
            limbs_old, bits, result, limbs
        );
    }
}

fn demo_limbs_vec_shr_round_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, bits, rm) in
        triples_of_unsigned_vec_small_unsigned_and_rounding_mode_var_1(gm).take(limit)
    {
        let mut limbs = limbs.to_vec();
        let limbs_old = limbs.clone();
        let result = limbs_vec_shr_round_in_place(&mut limbs, bits, rm);
        println!(
            "limbs := {:?}; limbs_vec_shr_round_in_place(&mut limbs, {}, {}) = {}; limbs = {:?}",
            limbs_old, bits, rm, result, limbs
        );
    }
}

fn benchmark_limbs_shr_round_up(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_shr_round_up(&[Limb], u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, bits)| {
            usize::exact_from(max(
                1,
                isize::exact_from(limbs.len()) - isize::exact_from(bits >> Limb::LOG_WIDTH),
            ))
        }),
        "max(1, limbs.len() - bits / Limb::WIDTH)",
        &mut [(
            "Malachite",
            &mut (|(limbs, bits)| no_out!(limbs_shr_round_up(&limbs, bits))),
        )],
    );
}

fn benchmark_limbs_shr_round_nearest(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_shr_round_nearest(&[Limb], u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, bits)| {
            usize::exact_from(max(
                1,
                isize::exact_from(limbs.len()) - isize::exact_from(bits >> Limb::LOG_WIDTH),
            ))
        }),
        "max(1, limbs.len() - bits / Limb::WIDTH)",
        &mut [(
            "Malachite",
            &mut (|(limbs, bits)| no_out!(limbs_shr_round_nearest(&limbs, bits))),
        )],
    );
}

fn benchmark_limbs_shr_exact(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_shr_exact(&[Limb], u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, bits)| {
            usize::exact_from(max(
                1,
                isize::exact_from(limbs.len()) - isize::exact_from(bits >> Limb::LOG_WIDTH),
            ))
        }),
        "max(1, limbs.len() - bits / Limb::WIDTH)",
        &mut [(
            "Malachite",
            &mut (|(limbs, bits)| no_out!(limbs_shr_exact(&limbs, bits))),
        )],
    );
}

fn benchmark_limbs_shr_round(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_shr_round(&[Limb], u64, RoundingMode)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_small_unsigned_and_rounding_mode_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, bits, _)| {
            usize::exact_from(max(
                1,
                isize::exact_from(limbs.len()) - isize::exact_from(bits >> Limb::LOG_WIDTH),
            ))
        }),
        "max(1, limbs.len() - bits / Limb::WIDTH)",
        &mut [(
            "Malachite",
            &mut (|(limbs, bits, rm)| no_out!(limbs_shr_round(&limbs, bits, rm))),
        )],
    );
}

fn benchmark_limbs_vec_shr_round_up_in_place(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_vec_shr_round_up_in_place(&mut Vec<Limb>, u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, bits)| limbs.len() + usize::exact_from(bits >> Limb::LOG_WIDTH)),
        "limbs.len()",
        &mut [(
            "Malachite",
            &mut (|(mut limbs, bits)| limbs_vec_shr_round_up_in_place(&mut limbs, bits)),
        )],
    );
}

fn benchmark_limbs_vec_shr_round_nearest_in_place(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "limbs_vec_shr_round_nearest_in_place(&mut Vec<Limb>, u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, bits)| limbs.len() + usize::exact_from(bits >> Limb::LOG_WIDTH)),
        "limbs.len()",
        &mut [(
            "Malachite",
            &mut (|(mut limbs, bits)| limbs_vec_shr_round_nearest_in_place(&mut limbs, bits)),
        )],
    );
}

fn benchmark_limbs_vec_shr_exact_in_place(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_vec_shr_exact_in_place(&mut Vec<Limb>, u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, bits)| limbs.len() + usize::exact_from(bits >> Limb::LOG_WIDTH)),
        "limbs.len()",
        &mut [(
            "Malachite",
            &mut (|(mut limbs, bits)| no_out!(limbs_vec_shr_exact_in_place(&mut limbs, bits))),
        )],
    );
}

fn benchmark_limbs_vec_shr_round_in_place(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_vec_shr_round_in_place(&mut Vec<Limb>, u64, RoundingMode)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_small_unsigned_and_rounding_mode_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, bits, _)| limbs.len() + usize::exact_from(bits >> Limb::LOG_WIDTH)),
        "limbs.len()",
        &mut [(
            "Malachite",
            &mut (|(mut limbs, bits, rm)| {
                no_out!(limbs_vec_shr_round_in_place(&mut limbs, bits, rm))
            }),
        )],
    );
}

macro_rules! demos_and_benches_unsigned {
    (
        $t:ident,
        $demo_natural_shr_round_assign_u:ident,
        $demo_natural_shr_round_u:ident,
        $demo_natural_shr_round_u_ref:ident,
        $benchmark_natural_shr_round_assign_u:ident,
        $benchmark_natural_shr_round_u_evaluation_strategy:ident
    ) => {
        fn $demo_natural_shr_round_assign_u(gm: GenerationMode, limit: usize) {
            for (mut n, u, rm) in
                triples_of_natural_small_unsigned_and_rounding_mode_var_1::<$t>(gm).take(limit)
            {
                let n_old = n.clone();
                n.shr_round_assign(u, rm);
                println!(
                    "x := {}; x.shr_round_assign({}, {}); x = {}",
                    n_old, u, rm, n
                );
            }
        }

        fn $demo_natural_shr_round_u(gm: GenerationMode, limit: usize) {
            for (n, u, rm) in
                triples_of_natural_small_unsigned_and_rounding_mode_var_1::<$t>(gm).take(limit)
            {
                let n_old = n.clone();
                println!(
                    "{}.shr_round({}, {}) = {}",
                    n_old,
                    u,
                    rm,
                    n.shr_round(u, rm)
                );
            }
        }

        fn $demo_natural_shr_round_u_ref(gm: GenerationMode, limit: usize) {
            for (n, u, rm) in
                triples_of_natural_small_unsigned_and_rounding_mode_var_1::<$t>(gm).take(limit)
            {
                println!(
                    "(&{}).shr_round({}, {}) = {}",
                    n,
                    u,
                    rm,
                    (&n).shr_round(u, rm)
                );
            }
        }

        fn $benchmark_natural_shr_round_assign_u(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            run_benchmark_old(
                &format!("Natural.shr_round_assign({}, RoundingMode)", $t::NAME),
                BenchmarkType::Single,
                triples_of_natural_small_unsigned_and_rounding_mode_var_1::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other, _)| usize::exact_from(other)),
                "other",
                &mut [(
                    "Malachite",
                    &mut (|(mut x, y, rm)| x.shr_round_assign(y, rm)),
                )],
            );
        }

        fn $benchmark_natural_shr_round_u_evaluation_strategy(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            run_benchmark_old(
                &format!("Natural.shr_round({}, RoundingMode)", $t::NAME),
                BenchmarkType::EvaluationStrategy,
                triples_of_natural_small_unsigned_and_rounding_mode_var_1::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other, _)| usize::exact_from(other)),
                "other",
                &mut [
                    (
                        &format!("Natural.shr_round({}, RoundingMode)", $t::NAME),
                        &mut (|(x, y, rm)| no_out!(x.shr_round(y, rm))),
                    ),
                    (
                        &format!("(&Natural).shr_round({}, RoundingMode)", $t::NAME),
                        &mut (|(x, y, rm)| no_out!((&x).shr_round(y, rm))),
                    ),
                ],
            );
        }
    };
}
demos_and_benches_unsigned!(
    u8,
    demo_natural_shr_round_assign_u8,
    demo_natural_shr_round_u8,
    demo_natural_shr_round_u8_ref,
    benchmark_natural_shr_round_assign_u8,
    benchmark_natural_shr_round_u8_evaluation_strategy
);
demos_and_benches_unsigned!(
    u16,
    demo_natural_shr_round_assign_u16,
    demo_natural_shr_round_u16,
    demo_natural_shr_round_u16_ref,
    benchmark_natural_shr_round_assign_u16,
    benchmark_natural_shr_round_u16_evaluation_strategy
);
demos_and_benches_unsigned!(
    u32,
    demo_natural_shr_round_assign_u32,
    demo_natural_shr_round_u32,
    demo_natural_shr_round_u32_ref,
    benchmark_natural_shr_round_assign_u32,
    benchmark_natural_shr_round_u32_evaluation_strategy
);
demos_and_benches_unsigned!(
    u64,
    demo_natural_shr_round_assign_u64,
    demo_natural_shr_round_u64,
    demo_natural_shr_round_u64_ref,
    benchmark_natural_shr_round_assign_u64,
    benchmark_natural_shr_round_u64_evaluation_strategy
);
demos_and_benches_unsigned!(
    usize,
    demo_natural_shr_round_assign_usize,
    demo_natural_shr_round_usize,
    demo_natural_shr_round_usize_ref,
    benchmark_natural_shr_round_assign_usize,
    benchmark_natural_shr_round_usize_evaluation_strategy
);

macro_rules! demos_and_benches_signed {
    (
        $t:ident,
        $demo_natural_shr_round_assign_i:ident,
        $demo_natural_shr_round_i:ident,
        $demo_natural_shr_round_i_ref:ident,
        $benchmark_natural_shr_round_assign_i:ident,
        $benchmark_natural_shr_round_i_evaluation_strategy:ident
    ) => {
        fn $demo_natural_shr_round_assign_i(gm: GenerationMode, limit: usize) {
            for (mut n, i, rm) in
                triples_of_natural_small_signed_and_rounding_mode_var_2::<$t>(gm).take(limit)
            {
                let n_old = n.clone();
                n.shr_round_assign(i, rm);
                println!(
                    "x := {}; x.shr_round_assign({}, {}); x = {}",
                    n_old, i, rm, n
                );
            }
        }

        fn $demo_natural_shr_round_i(gm: GenerationMode, limit: usize) {
            for (n, i, rm) in
                triples_of_natural_small_signed_and_rounding_mode_var_2::<$t>(gm).take(limit)
            {
                let n_old = n.clone();
                println!(
                    "{}.shr_round({}, {}) = {}",
                    n_old,
                    i,
                    rm,
                    n.shr_round(i, rm)
                );
            }
        }

        fn $demo_natural_shr_round_i_ref(gm: GenerationMode, limit: usize) {
            for (n, i, rm) in
                triples_of_natural_small_signed_and_rounding_mode_var_2::<$t>(gm).take(limit)
            {
                println!(
                    "(&{}).shr_round({}, {}) = {}",
                    n,
                    i,
                    rm,
                    (&n).shr_round(i, rm)
                );
            }
        }

        fn $benchmark_natural_shr_round_assign_i(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            run_benchmark_old(
                &format!("Natural.shr_round_assign({}, RoundingMode)", $t::NAME),
                BenchmarkType::Single,
                triples_of_natural_small_signed_and_rounding_mode_var_2::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other, _)| usize::exact_from(other.unsigned_abs())),
                "|other|",
                &mut [(
                    "Malachite",
                    &mut (|(mut x, y, rm)| x.shr_round_assign(y, rm)),
                )],
            );
        }

        fn $benchmark_natural_shr_round_i_evaluation_strategy(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            run_benchmark_old(
                &format!("Natural.shr_round({}, RoundingMode)", $t::NAME),
                BenchmarkType::EvaluationStrategy,
                triples_of_natural_small_signed_and_rounding_mode_var_2::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other, _)| usize::exact_from(other.unsigned_abs())),
                "|other|",
                &mut [
                    (
                        &format!("Natural.shr_round({}, RoundingMode)", $t::NAME),
                        &mut (|(x, y, rm)| no_out!(x.shr_round(y, rm))),
                    ),
                    (
                        &format!("(&Natural).shr_round({}, RoundingMode)", $t::NAME),
                        &mut (|(x, y, rm)| no_out!((&x).shr_round(y, rm))),
                    ),
                ],
            );
        }
    };
}
demos_and_benches_signed!(
    i8,
    demo_natural_shr_round_assign_i8,
    demo_natural_shr_round_i8,
    demo_natural_shr_round_i8_ref,
    benchmark_natural_shr_round_assign_i8,
    benchmark_natural_shr_round_i8_evaluation_strategy
);
demos_and_benches_signed!(
    i16,
    demo_natural_shr_round_assign_i16,
    demo_natural_shr_round_i16,
    demo_natural_shr_round_i16_ref,
    benchmark_natural_shr_round_assign_i16,
    benchmark_natural_shr_round_i16_evaluation_strategy
);
demos_and_benches_signed!(
    i32,
    demo_natural_shr_round_assign_i32,
    demo_natural_shr_round_i32,
    demo_natural_shr_round_i32_ref,
    benchmark_natural_shr_round_assign_i32,
    benchmark_natural_shr_round_i32_evaluation_strategy
);
demos_and_benches_signed!(
    i64,
    demo_natural_shr_round_assign_i64,
    demo_natural_shr_round_i64,
    demo_natural_shr_round_i64_ref,
    benchmark_natural_shr_round_assign_i64,
    benchmark_natural_shr_round_i64_evaluation_strategy
);
demos_and_benches_signed!(
    isize,
    demo_natural_shr_round_assign_isize,
    demo_natural_shr_round_isize,
    demo_natural_shr_round_isize_ref,
    benchmark_natural_shr_round_assign_isize,
    benchmark_natural_shr_round_isize_evaluation_strategy
);
