use malachite_base::named::Named;
use malachite_base::num::arithmetic::traits::{ShrRound, ShrRoundAssign, UnsignedAbs};
use malachite_base::num::conversion::traits::ExactFrom;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType,
};
use malachite_test::inputs::integer::{
    triples_of_integer_small_signed_and_rounding_mode_var_2,
    triples_of_integer_small_unsigned_and_rounding_mode_var_1,
};

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

    register_demo!(registry, demo_integer_shr_round_assign_i8);
    register_demo!(registry, demo_integer_shr_round_assign_i16);
    register_demo!(registry, demo_integer_shr_round_assign_i32);
    register_demo!(registry, demo_integer_shr_round_assign_i64);
    register_demo!(registry, demo_integer_shr_round_assign_isize);

    register_demo!(registry, demo_integer_shr_round_i8);
    register_demo!(registry, demo_integer_shr_round_i16);
    register_demo!(registry, demo_integer_shr_round_i32);
    register_demo!(registry, demo_integer_shr_round_i64);
    register_demo!(registry, demo_integer_shr_round_isize);

    register_demo!(registry, demo_integer_shr_round_i8_ref);
    register_demo!(registry, demo_integer_shr_round_i16_ref);
    register_demo!(registry, demo_integer_shr_round_i32_ref);
    register_demo!(registry, demo_integer_shr_round_i64_ref);
    register_demo!(registry, demo_integer_shr_round_isize_ref);

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

    register_bench!(registry, Large, benchmark_integer_shr_round_assign_i8);
    register_bench!(registry, Large, benchmark_integer_shr_round_assign_i16);
    register_bench!(registry, Large, benchmark_integer_shr_round_assign_i32);
    register_bench!(registry, Large, benchmark_integer_shr_round_assign_i64);
    register_bench!(registry, Large, benchmark_integer_shr_round_assign_isize);

    register_bench!(
        registry,
        Large,
        benchmark_integer_shr_round_i8_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_shr_round_i16_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_shr_round_i32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_shr_round_i64_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_shr_round_isize_evaluation_strategy
    );
}

macro_rules! demos_and_benches_u {
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
demos_and_benches_u!(
    u8,
    demo_integer_shr_round_assign_u8,
    demo_integer_shr_round_u8,
    demo_integer_shr_round_u8_ref,
    benchmark_integer_shr_round_assign_u8,
    benchmark_integer_shr_round_u8_evaluation_strategy
);
demos_and_benches_u!(
    u16,
    demo_integer_shr_round_assign_u16,
    demo_integer_shr_round_u16,
    demo_integer_shr_round_u16_ref,
    benchmark_integer_shr_round_assign_u16,
    benchmark_integer_shr_round_u16_evaluation_strategy
);
demos_and_benches_u!(
    u32,
    demo_integer_shr_round_assign_u32,
    demo_integer_shr_round_u32,
    demo_integer_shr_round_u32_ref,
    benchmark_integer_shr_round_assign_u32,
    benchmark_integer_shr_round_u32_evaluation_strategy
);
demos_and_benches_u!(
    u64,
    demo_integer_shr_round_assign_u64,
    demo_integer_shr_round_u64,
    demo_integer_shr_round_u64_ref,
    benchmark_integer_shr_round_assign_u64,
    benchmark_integer_shr_round_u64_evaluation_strategy
);
demos_and_benches_u!(
    usize,
    demo_integer_shr_round_assign_usize,
    demo_integer_shr_round_usize,
    demo_integer_shr_round_usize_ref,
    benchmark_integer_shr_round_assign_usize,
    benchmark_integer_shr_round_usize_evaluation_strategy
);

macro_rules! demos_and_benches_i {
    (
        $t:ident,
        $demo_integer_shr_round_assign_i:ident,
        $demo_integer_shr_round_i:ident,
        $demo_integer_shr_round_i_ref:ident,
        $benchmark_integer_shr_round_assign_i:ident,
        $benchmark_integer_shr_round_i_evaluation_strategy:ident
    ) => {
        fn $demo_integer_shr_round_assign_i(gm: GenerationMode, limit: usize) {
            for (mut n, i, rm) in
                triples_of_integer_small_signed_and_rounding_mode_var_2::<$t>(gm).take(limit)
            {
                let n_old = n.clone();
                n.shr_round_assign(i, rm);
                println!(
                    "x := {}; x.shr_round_assign({}, {}); x = {}",
                    n_old, i, rm, n
                );
            }
        }

        fn $demo_integer_shr_round_i(gm: GenerationMode, limit: usize) {
            for (n, i, rm) in
                triples_of_integer_small_signed_and_rounding_mode_var_2::<$t>(gm).take(limit)
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

        fn $demo_integer_shr_round_i_ref(gm: GenerationMode, limit: usize) {
            for (n, i, rm) in
                triples_of_integer_small_signed_and_rounding_mode_var_2::<$t>(gm).take(limit)
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

        fn $benchmark_integer_shr_round_assign_i(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            m_run_benchmark(
                &format!("Integer.shr_round_assign({}, RoundingMode)", $t::NAME),
                BenchmarkType::Single,
                triples_of_integer_small_signed_and_rounding_mode_var_2::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other, _)| usize::exact_from(other.unsigned_abs())),
                "|other|",
                &mut [(
                    "malachite",
                    &mut (|(mut x, y, rm)| x.shr_round_assign(y, rm)),
                )],
            );
        }

        fn $benchmark_integer_shr_round_i_evaluation_strategy(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            m_run_benchmark(
                &format!("Integer.shr_round({}, RoundingMode)", $t::NAME),
                BenchmarkType::EvaluationStrategy,
                triples_of_integer_small_signed_and_rounding_mode_var_2::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other, _)| usize::exact_from(other.unsigned_abs())),
                "|other|",
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
demos_and_benches_i!(
    i8,
    demo_integer_shr_round_assign_i8,
    demo_integer_shr_round_i8,
    demo_integer_shr_round_i8_ref,
    benchmark_integer_shr_round_assign_i8,
    benchmark_integer_shr_round_i8_evaluation_strategy
);
demos_and_benches_i!(
    i16,
    demo_integer_shr_round_assign_i16,
    demo_integer_shr_round_i16,
    demo_integer_shr_round_i16_ref,
    benchmark_integer_shr_round_assign_i16,
    benchmark_integer_shr_round_i16_evaluation_strategy
);
demos_and_benches_i!(
    i32,
    demo_integer_shr_round_assign_i32,
    demo_integer_shr_round_i32,
    demo_integer_shr_round_i32_ref,
    benchmark_integer_shr_round_assign_i32,
    benchmark_integer_shr_round_i32_evaluation_strategy
);
demos_and_benches_i!(
    i64,
    demo_integer_shr_round_assign_i64,
    demo_integer_shr_round_i64,
    demo_integer_shr_round_i64_ref,
    benchmark_integer_shr_round_assign_i64,
    benchmark_integer_shr_round_i64_evaluation_strategy
);
demos_and_benches_i!(
    isize,
    demo_integer_shr_round_assign_isize,
    demo_integer_shr_round_isize,
    demo_integer_shr_round_isize_ref,
    benchmark_integer_shr_round_assign_isize,
    benchmark_integer_shr_round_isize_evaluation_strategy
);
