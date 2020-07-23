use malachite_base::named::Named;
use malachite_base::num::arithmetic::traits::{ShlRound, ShlRoundAssign, UnsignedAbs};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::natural::triples_of_natural_small_signed_and_rounding_mode_var_1;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_shl_round_assign_i8);
    register_demo!(registry, demo_natural_shl_round_assign_i16);
    register_demo!(registry, demo_natural_shl_round_assign_i32);
    register_demo!(registry, demo_natural_shl_round_assign_i64);
    register_demo!(registry, demo_natural_shl_round_assign_isize);

    register_demo!(registry, demo_natural_shl_round_i8);
    register_demo!(registry, demo_natural_shl_round_i16);
    register_demo!(registry, demo_natural_shl_round_i32);
    register_demo!(registry, demo_natural_shl_round_i64);
    register_demo!(registry, demo_natural_shl_round_isize);

    register_demo!(registry, demo_natural_shl_round_i8_ref);
    register_demo!(registry, demo_natural_shl_round_i16_ref);
    register_demo!(registry, demo_natural_shl_round_i32_ref);
    register_demo!(registry, demo_natural_shl_round_i64_ref);
    register_demo!(registry, demo_natural_shl_round_isize_ref);

    register_bench!(registry, Large, benchmark_natural_shl_round_assign_i8);
    register_bench!(registry, Large, benchmark_natural_shl_round_assign_i16);
    register_bench!(registry, Large, benchmark_natural_shl_round_assign_i32);
    register_bench!(registry, Large, benchmark_natural_shl_round_assign_i64);
    register_bench!(registry, Large, benchmark_natural_shl_round_assign_isize);

    register_bench!(
        registry,
        Large,
        benchmark_natural_shl_round_i8_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shl_round_i16_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shl_round_i32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shl_round_i64_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shl_round_isize_evaluation_strategy
    );
}

macro_rules! demos_and_benches_signed {
    (
        $t:ident,
        $demo_natural_shl_round_assign_i:ident,
        $demo_natural_shl_round_i:ident,
        $demo_natural_shl_round_i_ref:ident,
        $benchmark_natural_shl_round_assign_i:ident,
        $benchmark_natural_shl_round_i_evaluation_strategy:ident
    ) => {
        fn $demo_natural_shl_round_assign_i(gm: GenerationMode, limit: usize) {
            for (mut n, i, rm) in
                triples_of_natural_small_signed_and_rounding_mode_var_1::<$t>(gm).take(limit)
            {
                let n_old = n.clone();
                n.shl_round_assign(i, rm);
                println!(
                    "x := {}; x.shl_round_assign({}, {}); x = {}",
                    n_old, i, rm, n
                );
            }
        }

        fn $demo_natural_shl_round_i(gm: GenerationMode, limit: usize) {
            for (n, i, rm) in
                triples_of_natural_small_signed_and_rounding_mode_var_1::<$t>(gm).take(limit)
            {
                let n_old = n.clone();
                println!(
                    "{}.shl_round({}, {}) = {}",
                    n_old,
                    i,
                    rm,
                    n.shl_round(i, rm)
                );
            }
        }

        fn $demo_natural_shl_round_i_ref(gm: GenerationMode, limit: usize) {
            for (n, i, rm) in
                triples_of_natural_small_signed_and_rounding_mode_var_1::<$t>(gm).take(limit)
            {
                println!(
                    "(&{}).shl_round({}, {}) = {}",
                    n,
                    i,
                    rm,
                    (&n).shl_round(i, rm)
                );
            }
        }

        fn $benchmark_natural_shl_round_assign_i(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            run_benchmark(
                &format!("Natural.shl_round_assign({}, RoundingMode)", $t::NAME),
                BenchmarkType::Single,
                triples_of_natural_small_signed_and_rounding_mode_var_1::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other, _)| usize::exact_from(other.unsigned_abs())),
                "|other|",
                &mut [(
                    "malachite",
                    &mut (|(mut x, y, rm)| x.shl_round_assign(y, rm)),
                )],
            );
        }

        fn $benchmark_natural_shl_round_i_evaluation_strategy(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            run_benchmark(
                &format!("Natural.shl_round({}, RoundingMode)", $t::NAME),
                BenchmarkType::EvaluationStrategy,
                triples_of_natural_small_signed_and_rounding_mode_var_1::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other, _)| usize::exact_from(other.unsigned_abs())),
                "|other|",
                &mut [
                    (
                        &format!("Natural.shl_round({}, RoundingMode)", $t::NAME),
                        &mut (|(x, y, rm)| no_out!(x.shl_round(y, rm))),
                    ),
                    (
                        &format!("(&Natural).shl_round({}, RoundingMode)", $t::NAME),
                        &mut (|(x, y, rm)| no_out!((&x).shl_round(y, rm))),
                    ),
                ],
            );
        }
    };
}
demos_and_benches_signed!(
    i8,
    demo_natural_shl_round_assign_i8,
    demo_natural_shl_round_i8,
    demo_natural_shl_round_i8_ref,
    benchmark_natural_shl_round_assign_i8,
    benchmark_natural_shl_round_i8_evaluation_strategy
);
demos_and_benches_signed!(
    i16,
    demo_natural_shl_round_assign_i16,
    demo_natural_shl_round_i16,
    demo_natural_shl_round_i16_ref,
    benchmark_natural_shl_round_assign_i16,
    benchmark_natural_shl_round_i16_evaluation_strategy
);
demos_and_benches_signed!(
    i32,
    demo_natural_shl_round_assign_i32,
    demo_natural_shl_round_i32,
    demo_natural_shl_round_i32_ref,
    benchmark_natural_shl_round_assign_i32,
    benchmark_natural_shl_round_i32_evaluation_strategy
);
demos_and_benches_signed!(
    i64,
    demo_natural_shl_round_assign_i64,
    demo_natural_shl_round_i64,
    demo_natural_shl_round_i64_ref,
    benchmark_natural_shl_round_assign_i64,
    benchmark_natural_shl_round_i64_evaluation_strategy
);
demos_and_benches_signed!(
    isize,
    demo_natural_shl_round_assign_isize,
    demo_natural_shl_round_isize,
    demo_natural_shl_round_isize_ref,
    benchmark_natural_shl_round_assign_isize,
    benchmark_natural_shl_round_isize_evaluation_strategy
);
