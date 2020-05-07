use malachite_base::named::Named;
use malachite_base::num::arithmetic::traits::{ShrRound, ShrRoundAssign};
use malachite_base::num::conversion::traits::ExactFrom;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::triples_of_integer_small_unsigned_and_rounding_mode_var_1;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_shr_round_assign_u8);
    register_demo!(registry, demo_integer_shr_round_assign_u16);
    register_demo!(registry, demo_integer_shr_round_assign_u32);
    register_demo!(registry, demo_integer_shr_round_assign_u64);
    register_demo!(registry, demo_integer_shr_round_assign_usize);

    register_demo!(registry, demo_integer_shr_round_u8);
    register_demo!(registry, demo_integer_shr_round_u16);
    register_demo!(registry, demo_integer_shr_round_u32);
    register_demo!(registry, demo_integer_shr_round_u64);
    register_demo!(registry, demo_integer_shr_round_usize);

    register_demo!(registry, demo_integer_shr_round_u8_ref);
    register_demo!(registry, demo_integer_shr_round_u16_ref);
    register_demo!(registry, demo_integer_shr_round_u32_ref);
    register_demo!(registry, demo_integer_shr_round_u64_ref);
    register_demo!(registry, demo_integer_shr_round_usize_ref);

    register_bench!(registry, Large, benchmark_integer_shr_round_assign_u8);
    register_bench!(registry, Large, benchmark_integer_shr_round_assign_u16);
    register_bench!(registry, Large, benchmark_integer_shr_round_assign_u32);
    register_bench!(registry, Large, benchmark_integer_shr_round_assign_u64);
    register_bench!(registry, Large, benchmark_integer_shr_round_assign_usize);

    register_bench!(
        registry,
        Large,
        benchmark_integer_shr_round_u8_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_shr_round_u16_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_shr_round_u32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_shr_round_u64_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_shr_round_usize_evaluation_strategy
    );
}

macro_rules! demos_and_benches {
    (
        $t:ident,
        $demo_integer_shr_round_assign_u:ident,
        $demo_integer_shr_round_u:ident,
        $demo_integer_shr_round_u_ref:ident,
        $benchmark_integer_shr_round_assign_u:ident,
        $benchmark_integer_shr_round_u_evaluation_strategy:ident
    ) => {
        fn $demo_integer_shr_round_assign_u(gm: GenerationMode, limit: usize) {
            for (mut n, u, rm) in
                triples_of_integer_small_unsigned_and_rounding_mode_var_1::<$t>(gm).take(limit)
            {
                let n_old = n.clone();
                n.shr_round_assign(u, rm);
                println!(
                    "x := {}; x.shr_round_assign({}, {}); x = {}",
                    n_old, u, rm, n
                );
            }
        }

        fn $demo_integer_shr_round_u(gm: GenerationMode, limit: usize) {
            for (n, u, rm) in
                triples_of_integer_small_unsigned_and_rounding_mode_var_1::<$t>(gm).take(limit)
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

        fn $demo_integer_shr_round_u_ref(gm: GenerationMode, limit: usize) {
            for (n, u, rm) in
                triples_of_integer_small_unsigned_and_rounding_mode_var_1::<$t>(gm).take(limit)
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

        fn $benchmark_integer_shr_round_assign_u(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            m_run_benchmark(
                &format!("Integer.shr_round_assign({}, RoundingMode)", $t::NAME),
                BenchmarkType::Single,
                triples_of_integer_small_unsigned_and_rounding_mode_var_1::<$t>(gm),
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

        fn $benchmark_integer_shr_round_u_evaluation_strategy(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            m_run_benchmark(
                &format!("Integer.shr_round({}, RoundingMode)", $t::NAME),
                BenchmarkType::EvaluationStrategy,
                triples_of_integer_small_unsigned_and_rounding_mode_var_1::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other, _)| usize::exact_from(other)),
                "other",
                &mut [
                    (
                        &format!("Integer.shr_round({}, RoundingMode)", $t::NAME),
                        &mut (|(x, y, rm)| no_out!(x.shr_round(y, rm))),
                    ),
                    (
                        &format!("(&Integer).shr_round({}, RoundingMode)", $t::NAME),
                        &mut (|(x, y, rm)| no_out!((&x).shr_round(y, rm))),
                    ),
                ],
            );
        }
    };
}
demos_and_benches!(
    u8,
    demo_integer_shr_round_assign_u8,
    demo_integer_shr_round_u8,
    demo_integer_shr_round_u8_ref,
    benchmark_integer_shr_round_assign_u8,
    benchmark_integer_shr_round_u8_evaluation_strategy
);
demos_and_benches!(
    u16,
    demo_integer_shr_round_assign_u16,
    demo_integer_shr_round_u16,
    demo_integer_shr_round_u16_ref,
    benchmark_integer_shr_round_assign_u16,
    benchmark_integer_shr_round_u16_evaluation_strategy
);
demos_and_benches!(
    u32,
    demo_integer_shr_round_assign_u32,
    demo_integer_shr_round_u32,
    demo_integer_shr_round_u32_ref,
    benchmark_integer_shr_round_assign_u32,
    benchmark_integer_shr_round_u32_evaluation_strategy
);
demos_and_benches!(
    u64,
    demo_integer_shr_round_assign_u64,
    demo_integer_shr_round_u64,
    demo_integer_shr_round_u64_ref,
    benchmark_integer_shr_round_assign_u64,
    benchmark_integer_shr_round_u64_evaluation_strategy
);
demos_and_benches!(
    usize,
    demo_integer_shr_round_assign_usize,
    demo_integer_shr_round_usize,
    demo_integer_shr_round_usize_ref,
    benchmark_integer_shr_round_assign_usize,
    benchmark_integer_shr_round_usize_evaluation_strategy
);
