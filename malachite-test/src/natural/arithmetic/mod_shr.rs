use malachite_base::named::Named;
use malachite_base::num::arithmetic::traits::{ModShr, ModShrAssign};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::natural::triples_of_natural_small_signed_and_natural_var_1;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_mod_shr_assign_i8);
    register_demo!(registry, demo_natural_mod_shr_assign_i16);
    register_demo!(registry, demo_natural_mod_shr_assign_i32);
    register_demo!(registry, demo_natural_mod_shr_assign_i64);
    register_demo!(registry, demo_natural_mod_shr_assign_isize);

    register_demo!(registry, demo_natural_mod_shr_assign_i8_ref);
    register_demo!(registry, demo_natural_mod_shr_assign_i16_ref);
    register_demo!(registry, demo_natural_mod_shr_assign_i32_ref);
    register_demo!(registry, demo_natural_mod_shr_assign_i64_ref);
    register_demo!(registry, demo_natural_mod_shr_assign_isize_ref);

    register_demo!(registry, demo_natural_mod_shr_i8);
    register_demo!(registry, demo_natural_mod_shr_i16);
    register_demo!(registry, demo_natural_mod_shr_i32);
    register_demo!(registry, demo_natural_mod_shr_i64);
    register_demo!(registry, demo_natural_mod_shr_isize);

    register_demo!(registry, demo_natural_mod_shr_i8_val_ref);
    register_demo!(registry, demo_natural_mod_shr_i16_val_ref);
    register_demo!(registry, demo_natural_mod_shr_i32_val_ref);
    register_demo!(registry, demo_natural_mod_shr_i64_val_ref);
    register_demo!(registry, demo_natural_mod_shr_isize_val_ref);

    register_demo!(registry, demo_natural_mod_shr_i8_ref_val);
    register_demo!(registry, demo_natural_mod_shr_i16_ref_val);
    register_demo!(registry, demo_natural_mod_shr_i32_ref_val);
    register_demo!(registry, demo_natural_mod_shr_i64_ref_val);
    register_demo!(registry, demo_natural_mod_shr_isize_ref_val);

    register_demo!(registry, demo_natural_mod_shr_i8_ref_ref);
    register_demo!(registry, demo_natural_mod_shr_i16_ref_ref);
    register_demo!(registry, demo_natural_mod_shr_i32_ref_ref);
    register_demo!(registry, demo_natural_mod_shr_i64_ref_ref);
    register_demo!(registry, demo_natural_mod_shr_isize_ref_ref);

    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_shr_assign_i8_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_shr_assign_i16_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_shr_assign_i32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_shr_assign_i64_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_shr_assign_isize_evaluation_strategy
    );

    register_bench!(registry, Large, benchmark_natural_mod_shr_i8_algorithms);
    register_bench!(registry, Large, benchmark_natural_mod_shr_i16_algorithms);
    register_bench!(registry, Large, benchmark_natural_mod_shr_i32_algorithms);
    register_bench!(registry, Large, benchmark_natural_mod_shr_i64_algorithms);
    register_bench!(registry, Large, benchmark_natural_mod_shr_isize_algorithms);

    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_shr_i8_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_shr_i16_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_shr_i32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_shr_i64_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_shr_isize_evaluation_strategy
    );
}

macro_rules! demos_and_benches_signed {
    (
        $t:ident,
        $demo_natural_mod_shr_assign_i:ident,
        $demo_natural_mod_shr_assign_i_ref:ident,
        $demo_natural_mod_shr_i:ident,
        $demo_natural_mod_shr_i_val_ref:ident,
        $demo_natural_mod_shr_i_ref_val:ident,
        $demo_natural_mod_shr_i_ref_ref:ident,
        $benchmark_natural_mod_shr_assign_i_evaluation_strategy:ident,
        $benchmark_natural_mod_shr_i_algorithms:ident,
        $benchmark_natural_mod_shr_i_evaluation_strategy:ident
    ) => {
        fn $demo_natural_mod_shr_assign_i(gm: GenerationMode, limit: usize) {
            for (mut n, i, m) in
                triples_of_natural_small_signed_and_natural_var_1::<$t>(gm).take(limit)
            {
                let n_old = n.clone();
                n.mod_shr_assign(i, m.clone());
                println!("x := {}; x.mod_shr_assign({}, {}); x = {}", n_old, i, m, n);
            }
        }

        fn $demo_natural_mod_shr_assign_i_ref(gm: GenerationMode, limit: usize) {
            for (mut n, i, m) in
                triples_of_natural_small_signed_and_natural_var_1::<$t>(gm).take(limit)
            {
                let n_old = n.clone();
                n.mod_shr_assign(i, &m);
                println!("x := {}; x.mod_shr_assign({}, &{}); x = {}", n_old, i, m, n);
            }
        }

        fn $demo_natural_mod_shr_i(gm: GenerationMode, limit: usize) {
            for (n, i, m) in triples_of_natural_small_signed_and_natural_var_1::<$t>(gm).take(limit)
            {
                let n_old = n.clone();
                println!(
                    "{}.mod_shr({}, {}) = {}",
                    n_old,
                    i,
                    m.clone(),
                    n.mod_shr(i, m)
                );
            }
        }

        fn $demo_natural_mod_shr_i_val_ref(gm: GenerationMode, limit: usize) {
            for (n, i, m) in triples_of_natural_small_signed_and_natural_var_1::<$t>(gm).take(limit)
            {
                let n_old = n.clone();
                println!("{}.mod_shr({}, &{}) = {}", n_old, i, m, n.mod_shr(i, &m));
            }
        }

        fn $demo_natural_mod_shr_i_ref_val(gm: GenerationMode, limit: usize) {
            for (n, i, m) in triples_of_natural_small_signed_and_natural_var_1::<$t>(gm).take(limit)
            {
                println!(
                    "(&{}).mod_shr({}, {}) = {}",
                    n,
                    i,
                    m.clone(),
                    (&n).mod_shr(i, m)
                );
            }
        }

        fn $demo_natural_mod_shr_i_ref_ref(gm: GenerationMode, limit: usize) {
            for (n, i, m) in triples_of_natural_small_signed_and_natural_var_1::<$t>(gm).take(limit)
            {
                println!("(&{}).mod_shr({}, &{}) = {}", n, i, m, (&n).mod_shr(i, &m));
            }
        }

        fn $benchmark_natural_mod_shr_assign_i_evaluation_strategy(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            run_benchmark_old(
                &format!("Natural.mod_shr_assign({}, Natural)", $t::NAME),
                BenchmarkType::EvaluationStrategy,
                triples_of_natural_small_signed_and_natural_var_1::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, _, ref m)| usize::exact_from(m.significant_bits())),
                "other",
                &mut [
                    (
                        &format!("Natural.mod_shr_assign({}, Natural)", $t::NAME),
                        &mut |(mut x, y, m)| no_out!(x.mod_shr_assign(y, m)),
                    ),
                    (
                        &format!("Natural.mod_shr_assign({}, &Natural)", $t::NAME),
                        &mut |(mut x, y, m)| no_out!(x.mod_shr_assign(y, &m)),
                    ),
                ],
            );
        }

        fn $benchmark_natural_mod_shr_i_algorithms(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            run_benchmark_old(
                &format!("Natural.mod_shr({}, Natural)", $t::NAME),
                BenchmarkType::Algorithms,
                triples_of_natural_small_signed_and_natural_var_1::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, _, ref m)| usize::exact_from(m.significant_bits())),
                "other",
                &mut [
                    ("default", &mut |(x, y, m)| no_out!(x.mod_shr(y, m))),
                    ("using >> and %", &mut |(x, y, m)| no_out!((x >> y) % m)),
                ],
            );
        }

        fn $benchmark_natural_mod_shr_i_evaluation_strategy(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            run_benchmark_old(
                &format!("Natural.mod_shr({}, Natural)", $t::NAME),
                BenchmarkType::EvaluationStrategy,
                triples_of_natural_small_signed_and_natural_var_1::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, _, ref m)| usize::exact_from(m.significant_bits())),
                "other",
                &mut [
                    (
                        &format!("Natural.mod_shr({}, Natural)", $t::NAME),
                        &mut |(x, y, m)| no_out!(x.mod_shr(y, m)),
                    ),
                    (
                        &format!("Natural.mod_shr({}, &Natural)", $t::NAME),
                        &mut |(x, y, m)| no_out!(x.mod_shr(y, &m)),
                    ),
                    (
                        &format!("(&Natural).mod_shr({}, Natural)", $t::NAME),
                        &mut |(x, y, m)| no_out!((&x).mod_shr(y, m)),
                    ),
                    (
                        &format!("(&Natural).mod_shr({}, &Natural)", $t::NAME),
                        &mut |(x, y, m)| no_out!((&x).mod_shr(y, &m)),
                    ),
                ],
            );
        }
    };
}
demos_and_benches_signed!(
    i8,
    demo_natural_mod_shr_assign_i8,
    demo_natural_mod_shr_assign_i8_ref,
    demo_natural_mod_shr_i8,
    demo_natural_mod_shr_i8_val_ref,
    demo_natural_mod_shr_i8_ref_val,
    demo_natural_mod_shr_i8_ref_ref,
    benchmark_natural_mod_shr_assign_i8_evaluation_strategy,
    benchmark_natural_mod_shr_i8_algorithms,
    benchmark_natural_mod_shr_i8_evaluation_strategy
);
demos_and_benches_signed!(
    i16,
    demo_natural_mod_shr_assign_i16,
    demo_natural_mod_shr_assign_i16_ref,
    demo_natural_mod_shr_i16,
    demo_natural_mod_shr_i16_val_ref,
    demo_natural_mod_shr_i16_ref_val,
    demo_natural_mod_shr_i16_ref_ref,
    benchmark_natural_mod_shr_assign_i16_evaluation_strategy,
    benchmark_natural_mod_shr_i16_algorithms,
    benchmark_natural_mod_shr_i16_evaluation_strategy
);
demos_and_benches_signed!(
    i32,
    demo_natural_mod_shr_assign_i32,
    demo_natural_mod_shr_assign_i32_ref,
    demo_natural_mod_shr_i32,
    demo_natural_mod_shr_i32_val_ref,
    demo_natural_mod_shr_i32_ref_val,
    demo_natural_mod_shr_i32_ref_ref,
    benchmark_natural_mod_shr_assign_i32_evaluation_strategy,
    benchmark_natural_mod_shr_i32_algorithms,
    benchmark_natural_mod_shr_i32_evaluation_strategy
);
demos_and_benches_signed!(
    i64,
    demo_natural_mod_shr_assign_i64,
    demo_natural_mod_shr_assign_i64_ref,
    demo_natural_mod_shr_i64,
    demo_natural_mod_shr_i64_val_ref,
    demo_natural_mod_shr_i64_ref_val,
    demo_natural_mod_shr_i64_ref_ref,
    benchmark_natural_mod_shr_assign_i64_evaluation_strategy,
    benchmark_natural_mod_shr_i64_algorithms,
    benchmark_natural_mod_shr_i64_evaluation_strategy
);
demos_and_benches_signed!(
    isize,
    demo_natural_mod_shr_assign_isize,
    demo_natural_mod_shr_assign_isize_ref,
    demo_natural_mod_shr_isize,
    demo_natural_mod_shr_isize_val_ref,
    demo_natural_mod_shr_isize_ref_val,
    demo_natural_mod_shr_isize_ref_ref,
    benchmark_natural_mod_shr_assign_isize_evaluation_strategy,
    benchmark_natural_mod_shr_isize_algorithms,
    benchmark_natural_mod_shr_isize_evaluation_strategy
);
