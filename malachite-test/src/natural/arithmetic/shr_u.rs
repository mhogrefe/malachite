use std::cmp::max;

use malachite_base::named::Named;
use malachite_base::num::arithmetic::traits::{ShrRound, ShrRoundAssign};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::natural::arithmetic::shr_u::{
    limbs_shr, limbs_shr_exact, limbs_shr_round, limbs_shr_round_to_nearest, limbs_shr_round_up,
    limbs_shr_to_out, limbs_slice_shr_in_place, limbs_vec_shr_exact_in_place,
    limbs_vec_shr_in_place, limbs_vec_shr_round_in_place, limbs_vec_shr_round_to_nearest_in_place,
    limbs_vec_shr_round_up_in_place,
};
use malachite_nz::platform::Limb;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    pairs_of_unsigned_vec_and_small_unsigned, pairs_of_unsigned_vec_and_small_unsigned_var_1,
    pairs_of_unsigned_vec_and_u64_var_2,
    triples_of_unsigned_vec_small_unsigned_and_rounding_mode_var_1,
    triples_of_unsigned_vec_unsigned_vec_and_u64_var_6,
};
use inputs::natural::{
    pairs_of_natural_and_small_unsigned, rm_pairs_of_natural_and_small_unsigned,
    triples_of_natural_small_unsigned_and_rounding_mode_var_1,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_shr);
    register_demo!(registry, demo_limbs_shr_round_up);
    register_demo!(registry, demo_limbs_shr_round_to_nearest);
    register_demo!(registry, demo_limbs_shr_exact);
    register_demo!(registry, demo_limbs_shr_round);
    register_demo!(registry, demo_limbs_shr_to_out);
    register_demo!(registry, demo_limbs_slice_shr_in_place);
    register_demo!(registry, demo_limbs_vec_shr_in_place);
    register_demo!(registry, demo_limbs_vec_shr_round_up_in_place);
    register_demo!(registry, demo_limbs_vec_shr_round_to_nearest_in_place);
    register_demo!(registry, demo_limbs_vec_shr_exact_in_place);
    register_demo!(registry, demo_limbs_vec_shr_round_in_place);

    register_demo!(registry, demo_natural_shr_assign_u8);
    register_demo!(registry, demo_natural_shr_assign_u16);
    register_demo!(registry, demo_natural_shr_assign_u32);
    register_demo!(registry, demo_natural_shr_assign_u64);
    register_demo!(registry, demo_natural_shr_assign_usize);

    register_demo!(registry, demo_natural_shr_u8);
    register_demo!(registry, demo_natural_shr_u16);
    register_demo!(registry, demo_natural_shr_u32);
    register_demo!(registry, demo_natural_shr_u64);
    register_demo!(registry, demo_natural_shr_usize);

    register_demo!(registry, demo_natural_shr_u8_ref);
    register_demo!(registry, demo_natural_shr_u16_ref);
    register_demo!(registry, demo_natural_shr_u32_ref);
    register_demo!(registry, demo_natural_shr_u64_ref);
    register_demo!(registry, demo_natural_shr_usize_ref);

    register_demo!(registry, demo_natural_shr_round_assign_u8);
    register_demo!(registry, demo_natural_shr_round_assign_u16);
    register_demo!(registry, demo_natural_shr_round_assign_u32);
    register_demo!(registry, demo_natural_shr_round_assign_u64);
    register_demo!(registry, demo_natural_shr_round_assign_usize);

    register_demo!(registry, demo_natural_shr_round_u8);
    register_demo!(registry, demo_natural_shr_round_u16);
    register_demo!(registry, demo_natural_shr_round_u32);
    register_demo!(registry, demo_natural_shr_round_u64);
    register_demo!(registry, demo_natural_shr_round_usize);

    register_demo!(registry, demo_natural_shr_round_u8_ref);
    register_demo!(registry, demo_natural_shr_round_u16_ref);
    register_demo!(registry, demo_natural_shr_round_u32_ref);
    register_demo!(registry, demo_natural_shr_round_u64_ref);
    register_demo!(registry, demo_natural_shr_round_usize_ref);

    register_bench!(registry, Small, benchmark_limbs_shr);
    register_bench!(registry, Small, benchmark_limbs_shr_round_up);
    register_bench!(registry, Small, benchmark_limbs_shr_round_to_nearest);
    register_bench!(registry, Small, benchmark_limbs_shr_exact);
    register_bench!(registry, Small, benchmark_limbs_shr_round);
    register_bench!(registry, Small, benchmark_limbs_shr_to_out);
    register_bench!(registry, Small, benchmark_limbs_slice_shr_in_place);
    register_bench!(registry, Small, benchmark_limbs_vec_shr_in_place);
    register_bench!(registry, Small, benchmark_limbs_vec_shr_round_up_in_place);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_vec_shr_round_to_nearest_in_place
    );
    register_bench!(registry, Small, benchmark_limbs_vec_shr_exact_in_place);
    register_bench!(registry, Small, benchmark_limbs_vec_shr_round_in_place);

    register_bench!(
        registry,
        Large,
        benchmark_natural_shr_u8_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shr_u16_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shr_u32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shr_u64_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shr_usize_evaluation_strategy
    );

    register_bench!(registry, Large, benchmark_natural_shr_round_assign_u8);
    register_bench!(registry, Large, benchmark_natural_shr_round_assign_u16);
    register_bench!(registry, Large, benchmark_natural_shr_round_assign_u32);
    register_bench!(registry, Large, benchmark_natural_shr_round_assign_u64);
    register_bench!(registry, Large, benchmark_natural_shr_round_assign_usize);

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
        benchmark_natural_shr_assign_u32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shr_u32_library_comparison
    );
}

fn demo_limbs_shr(gm: GenerationMode, limit: usize) {
    for (limbs, bits) in pairs_of_unsigned_vec_and_small_unsigned(gm).take(limit) {
        println!(
            "limbs_shr({:?}, {}) = {:?}",
            limbs,
            bits,
            limbs_shr(&limbs, bits)
        );
    }
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

fn demo_limbs_shr_round_to_nearest(gm: GenerationMode, limit: usize) {
    for (limbs, bits) in pairs_of_unsigned_vec_and_small_unsigned(gm).take(limit) {
        println!(
            "limbs_shr_round_to_nearest({:?}, {}) = {:?}",
            limbs,
            bits,
            limbs_shr_round_to_nearest(&limbs, bits)
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

fn demo_limbs_shr_to_out(gm: GenerationMode, limit: usize) {
    for (out, in_limbs, bits) in triples_of_unsigned_vec_unsigned_vec_and_u64_var_6(gm).take(limit)
    {
        let mut out = out.to_vec();
        let out_old = out.clone();
        let carry = limbs_shr_to_out(&mut out, &in_limbs, bits);
        println!(
            "out := {:?}; limbs_shr_to_out(&mut out, {:?}, {}) = {}; out = {:?}",
            out_old, in_limbs, bits, carry, out
        );
    }
}

fn demo_limbs_slice_shr_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, bits) in pairs_of_unsigned_vec_and_u64_var_2(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let limbs_old = limbs.clone();
        let carry = limbs_slice_shr_in_place(&mut limbs, bits);
        println!(
            "limbs := {:?}; limbs_slice_shr_in_place(&mut limbs, {}) = {}; limbs = {:?}",
            limbs_old, bits, carry, limbs
        );
    }
}

fn demo_limbs_vec_shr_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, bits) in pairs_of_unsigned_vec_and_small_unsigned(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let limbs_old = limbs.clone();
        limbs_vec_shr_in_place(&mut limbs, bits);
        println!(
            "limbs := {:?}; limbs_vec_shr_in_place(&mut limbs, {}); limbs = {:?}",
            limbs_old, bits, limbs
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

fn demo_limbs_vec_shr_round_to_nearest_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, bits) in pairs_of_unsigned_vec_and_small_unsigned(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let limbs_old = limbs.clone();
        limbs_vec_shr_round_to_nearest_in_place(&mut limbs, bits);
        println!(
            "limbs := {:?}; limbs_vec_shr_round_to_nearest_in_place(&mut limbs, {}); limbs = {:?}",
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

macro_rules! demos_and_benches {
    (
        $t:ident,
        $demo_natural_shr_assign_u:ident,
        $demo_natural_shr_u:ident,
        $demo_natural_shr_u_ref:ident,
        $demo_natural_shr_round_assign_u:ident,
        $demo_natural_shr_round_u:ident,
        $demo_natural_shr_round_u_ref:ident,
        $benchmark_natural_shr_u_evaluation_strategy:ident,
        $benchmark_natural_shr_round_assign_u:ident,
        $benchmark_natural_shr_round_u_evaluation_strategy:ident
    ) => {
        fn $demo_natural_shr_assign_u(gm: GenerationMode, limit: usize) {
            for (mut n, u) in pairs_of_natural_and_small_unsigned::<$t>(gm).take(limit) {
                let n_old = n.clone();
                n >>= u;
                println!("x := {}; x >>= {}; x = {}", n_old, u, n);
            }
        }

        fn $demo_natural_shr_u(gm: GenerationMode, limit: usize) {
            for (n, u) in pairs_of_natural_and_small_unsigned::<$t>(gm).take(limit) {
                let n_old = n.clone();
                println!("{} >> {} = {}", n_old, u, n >> u);
            }
        }

        fn $demo_natural_shr_u_ref(gm: GenerationMode, limit: usize) {
            for (n, u) in pairs_of_natural_and_small_unsigned::<$t>(gm).take(limit) {
                println!("&{} >> {} = {}", n, u, &n >> u);
            }
        }

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

        fn $benchmark_natural_shr_u_evaluation_strategy(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            m_run_benchmark(
                &format!("Natural >> {}", $t::NAME),
                BenchmarkType::EvaluationStrategy,
                pairs_of_natural_and_small_unsigned::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other)| usize::exact_from(other)),
                "other",
                &mut [
                    (
                        &format!("Natural >> {}", $t::NAME),
                        &mut (|(x, y)| no_out!(x >> y)),
                    ),
                    (
                        &format!("&Natural >> {}", $t::NAME),
                        &mut (|(x, y)| no_out!(&x >> y)),
                    ),
                ],
            );
        }

        fn $benchmark_natural_shr_round_assign_u(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            m_run_benchmark(
                &format!("Natural.shr_round_assign({}, RoundingMode)", $t::NAME),
                BenchmarkType::Single,
                triples_of_natural_small_unsigned_and_rounding_mode_var_1::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other, _)| usize::exact_from(other)),
                "other",
                &mut [(
                    "malachite",
                    &mut (|(mut x, y, rm)| x.shr_round_assign(y, rm)),
                )],
            );
        }

        fn $benchmark_natural_shr_round_u_evaluation_strategy(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            m_run_benchmark(
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
demos_and_benches!(
    u8,
    demo_natural_shr_assign_u8,
    demo_natural_shr_u8,
    demo_natural_shr_u8_ref,
    demo_natural_shr_round_assign_u8,
    demo_natural_shr_round_u8,
    demo_natural_shr_round_u8_ref,
    benchmark_natural_shr_u8_evaluation_strategy,
    benchmark_natural_shr_round_assign_u8,
    benchmark_natural_shr_round_u8_evaluation_strategy
);
demos_and_benches!(
    u16,
    demo_natural_shr_assign_u16,
    demo_natural_shr_u16,
    demo_natural_shr_u16_ref,
    demo_natural_shr_round_assign_u16,
    demo_natural_shr_round_u16,
    demo_natural_shr_round_u16_ref,
    benchmark_natural_shr_u16_evaluation_strategy,
    benchmark_natural_shr_round_assign_u16,
    benchmark_natural_shr_round_u16_evaluation_strategy
);
demos_and_benches!(
    u32,
    demo_natural_shr_assign_u32,
    demo_natural_shr_u32,
    demo_natural_shr_u32_ref,
    demo_natural_shr_round_assign_u32,
    demo_natural_shr_round_u32,
    demo_natural_shr_round_u32_ref,
    benchmark_natural_shr_u32_evaluation_strategy,
    benchmark_natural_shr_round_assign_u32,
    benchmark_natural_shr_round_u32_evaluation_strategy
);
demos_and_benches!(
    u64,
    demo_natural_shr_assign_u64,
    demo_natural_shr_u64,
    demo_natural_shr_u64_ref,
    demo_natural_shr_round_assign_u64,
    demo_natural_shr_round_u64,
    demo_natural_shr_round_u64_ref,
    benchmark_natural_shr_u64_evaluation_strategy,
    benchmark_natural_shr_round_assign_u64,
    benchmark_natural_shr_round_u64_evaluation_strategy
);
demos_and_benches!(
    usize,
    demo_natural_shr_assign_usize,
    demo_natural_shr_usize,
    demo_natural_shr_usize_ref,
    demo_natural_shr_round_assign_usize,
    demo_natural_shr_round_usize,
    demo_natural_shr_round_usize_ref,
    benchmark_natural_shr_usize_evaluation_strategy,
    benchmark_natural_shr_round_assign_usize,
    benchmark_natural_shr_round_usize_evaluation_strategy
);

fn benchmark_limbs_shr(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_shr(&[Limb], u64)",
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
            "malachite",
            &mut (|(limbs, bits)| no_out!(limbs_shr(&limbs, bits))),
        )],
    );
}

fn benchmark_limbs_shr_round_up(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
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
            "malachite",
            &mut (|(limbs, bits)| no_out!(limbs_shr_round_up(&limbs, bits))),
        )],
    );
}

fn benchmark_limbs_shr_round_to_nearest(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_shr_round_to_nearest(&[Limb], u64)",
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
            "malachite",
            &mut (|(limbs, bits)| no_out!(limbs_shr_round_to_nearest(&limbs, bits))),
        )],
    );
}

fn benchmark_limbs_shr_exact(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
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
            "malachite",
            &mut (|(limbs, bits)| no_out!(limbs_shr_exact(&limbs, bits))),
        )],
    );
}

fn benchmark_limbs_shr_round(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
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
            "malachite",
            &mut (|(limbs, bits, rm)| no_out!(limbs_shr_round(&limbs, bits, rm))),
        )],
    );
}

fn benchmark_limbs_shr_to_out(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_shr_to_out(&mut [Limb], &[Limb], u64)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_u64_var_6(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref in_limbs, _)| in_limbs.len()),
        "in_limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut out, in_limbs, bits)| no_out!(limbs_shr_to_out(&mut out, &in_limbs, bits))),
        )],
    );
}

fn benchmark_limbs_slice_shr_in_place(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_slice_shr_in_place(&mut [Limb], u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_u64_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut limbs, bits)| no_out!(limbs_slice_shr_in_place(&mut limbs, bits))),
        )],
    );
}

fn benchmark_limbs_vec_shr_in_place(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_vec_shr_in_place(&mut Vec<Limb>, u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, bits)| limbs.len() + usize::exact_from(bits >> Limb::LOG_WIDTH)),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut limbs, bits)| limbs_vec_shr_in_place(&mut limbs, bits)),
        )],
    );
}

fn benchmark_limbs_vec_shr_round_up_in_place(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_vec_shr_round_up_in_place(&mut Vec<Limb>, u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, bits)| limbs.len() + usize::exact_from(bits >> Limb::LOG_WIDTH)),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut limbs, bits)| limbs_vec_shr_round_up_in_place(&mut limbs, bits)),
        )],
    );
}

fn benchmark_limbs_vec_shr_round_to_nearest_in_place(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_vec_shr_round_to_nearest_in_place(&mut Vec<Limb>, u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, bits)| limbs.len() + usize::exact_from(bits >> Limb::LOG_WIDTH)),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut limbs, bits)| limbs_vec_shr_round_to_nearest_in_place(&mut limbs, bits)),
        )],
    );
}

fn benchmark_limbs_vec_shr_exact_in_place(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_vec_shr_exact_in_place(&mut Vec<Limb>, u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, bits)| limbs.len() + usize::exact_from(bits >> Limb::LOG_WIDTH)),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut limbs, bits)| no_out!(limbs_vec_shr_exact_in_place(&mut limbs, bits))),
        )],
    );
}

fn benchmark_limbs_vec_shr_round_in_place(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_vec_shr_round_in_place(&mut Vec<Limb>, u64, RoundingMode)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_small_unsigned_and_rounding_mode_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, bits, _)| limbs.len() + usize::exact_from(bits >> Limb::LOG_WIDTH)),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut limbs, bits, rm)| {
                no_out!(limbs_vec_shr_round_in_place(&mut limbs, bits, rm))
            }),
        )],
    );
}

fn benchmark_natural_shr_assign_u32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural >>= u32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_small_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, other))| usize::exact_from(other)),
        "other",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x >>= y)),
            ("rug", &mut (|((mut x, y), _)| x >>= y)),
        ],
    );
}

fn benchmark_natural_shr_u32_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural >> u32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_small_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, other))| usize::exact_from(other)),
        "other",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x >> y))),
            ("rug", &mut (|((x, y), _)| no_out!(x >> y))),
        ],
    );
}
