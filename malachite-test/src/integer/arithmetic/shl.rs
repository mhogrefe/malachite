use malachite_base::named::Named;
use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::integer::{
    pairs_of_integer_and_small_signed, pairs_of_integer_and_small_unsigned,
    rm_pairs_of_integer_and_small_signed, rm_pairs_of_integer_and_small_unsigned,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_shl_assign_u8);
    register_demo!(registry, demo_integer_shl_assign_u16);
    register_demo!(registry, demo_integer_shl_assign_u32);
    register_demo!(registry, demo_integer_shl_assign_u64);
    register_demo!(registry, demo_integer_shl_assign_usize);

    register_demo!(registry, demo_integer_shl_u8);
    register_demo!(registry, demo_integer_shl_u16);
    register_demo!(registry, demo_integer_shl_u32);
    register_demo!(registry, demo_integer_shl_u64);
    register_demo!(registry, demo_integer_shl_usize);

    register_demo!(registry, demo_integer_shl_u8_ref);
    register_demo!(registry, demo_integer_shl_u16_ref);
    register_demo!(registry, demo_integer_shl_u32_ref);
    register_demo!(registry, demo_integer_shl_u64_ref);
    register_demo!(registry, demo_integer_shl_usize_ref);

    register_demo!(registry, demo_integer_shl_assign_i8);
    register_demo!(registry, demo_integer_shl_assign_i16);
    register_demo!(registry, demo_integer_shl_assign_i32);
    register_demo!(registry, demo_integer_shl_assign_i64);
    register_demo!(registry, demo_integer_shl_assign_isize);

    register_demo!(registry, demo_integer_shl_i8);
    register_demo!(registry, demo_integer_shl_i16);
    register_demo!(registry, demo_integer_shl_i32);
    register_demo!(registry, demo_integer_shl_i64);
    register_demo!(registry, demo_integer_shl_isize);

    register_demo!(registry, demo_integer_shl_i8_ref);
    register_demo!(registry, demo_integer_shl_i16_ref);
    register_demo!(registry, demo_integer_shl_i32_ref);
    register_demo!(registry, demo_integer_shl_i64_ref);
    register_demo!(registry, demo_integer_shl_isize_ref);

    register_bench!(
        registry,
        Large,
        benchmark_integer_shl_u8_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_shl_u16_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_shl_u32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_shl_u64_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_shl_usize_evaluation_strategy
    );

    register_bench!(
        registry,
        Large,
        benchmark_integer_shl_assign_u32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_shl_u32_library_comparison
    );

    register_bench!(
        registry,
        Large,
        benchmark_integer_shl_i8_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_shl_i16_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_shl_i32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_shl_i64_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_shl_isize_evaluation_strategy
    );

    register_bench!(
        registry,
        Large,
        benchmark_integer_shl_assign_i32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_shl_i32_library_comparison
    );
}

macro_rules! demos_and_benches_u {
    (
        $t:ident,
        $demo_integer_shl_assign_u:ident,
        $demo_integer_shl_u:ident,
        $demo_integer_shl_u_ref:ident,
        $benchmark_integer_shl_u_evaluation_strategy:ident
    ) => {
        fn $demo_integer_shl_assign_u(gm: GenerationMode, limit: usize) {
            for (mut n, u) in pairs_of_integer_and_small_unsigned::<$t>(gm).take(limit) {
                let n_old = n.clone();
                n <<= u;
                println!("x := {}; x <<= {}; x = {}", n_old, u, n);
            }
        }

        fn $demo_integer_shl_u(gm: GenerationMode, limit: usize) {
            for (n, u) in pairs_of_integer_and_small_unsigned::<$t>(gm).take(limit) {
                let n_old = n.clone();
                println!("{} << {} = {}", n_old, u, n << u);
            }
        }

        fn $demo_integer_shl_u_ref(gm: GenerationMode, limit: usize) {
            for (n, u) in pairs_of_integer_and_small_unsigned::<$t>(gm).take(limit) {
                println!("&{} << {} = {}", n, u, &n << u);
            }
        }

        fn $benchmark_integer_shl_u_evaluation_strategy(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            run_benchmark(
                &format!("Integer << {}", $t::NAME),
                BenchmarkType::EvaluationStrategy,
                pairs_of_integer_and_small_unsigned::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other)| usize::exact_from(other)),
                "other",
                &mut [
                    (
                        &format!("Integer << {}", $t::NAME),
                        &mut (|(x, y)| no_out!(x << y)),
                    ),
                    (
                        &format!("&Integer << {}", $t::NAME),
                        &mut (|(x, y)| no_out!(&x << y)),
                    ),
                ],
            );
        }
    };
}
demos_and_benches_u!(
    u8,
    demo_integer_shl_assign_u8,
    demo_integer_shl_u8,
    demo_integer_shl_u8_ref,
    benchmark_integer_shl_u8_evaluation_strategy
);
demos_and_benches_u!(
    u16,
    demo_integer_shl_assign_u16,
    demo_integer_shl_u16,
    demo_integer_shl_u16_ref,
    benchmark_integer_shl_u16_evaluation_strategy
);
demos_and_benches_u!(
    u32,
    demo_integer_shl_assign_u32,
    demo_integer_shl_u32,
    demo_integer_shl_u32_ref,
    benchmark_integer_shl_u32_evaluation_strategy
);
demos_and_benches_u!(
    u64,
    demo_integer_shl_assign_u64,
    demo_integer_shl_u64,
    demo_integer_shl_u64_ref,
    benchmark_integer_shl_u64_evaluation_strategy
);
demos_and_benches_u!(
    usize,
    demo_integer_shl_assign_usize,
    demo_integer_shl_usize,
    demo_integer_shl_usize_ref,
    benchmark_integer_shl_usize_evaluation_strategy
);

fn benchmark_integer_shl_assign_u32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer <<= u32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_small_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, other))| usize::exact_from(other)),
        "other",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x <<= y)),
            ("rug", &mut (|((mut x, y), _)| x <<= y)),
        ],
    );
}

fn benchmark_integer_shl_u32_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "Integer << u32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_small_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, other))| usize::exact_from(other)),
        "other",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x << y))),
            ("rug", &mut (|((x, y), _)| no_out!(x << y))),
        ],
    );
}

macro_rules! demos_and_benches_i {
    (
        $t:ident,
        $demo_integer_shl_assign_i:ident,
        $demo_integer_shl_i:ident,
        $demo_integer_shl_i_ref:ident,
        $benchmark_integer_shl_i_evaluation_strategy:ident
    ) => {
        fn $demo_integer_shl_assign_i(gm: GenerationMode, limit: usize) {
            for (mut n, i) in pairs_of_integer_and_small_signed::<$t>(gm).take(limit) {
                let n_old = n.clone();
                n <<= i;
                println!("x := {}; x <<= {}; x = {}", n_old, i, n);
            }
        }

        fn $demo_integer_shl_i(gm: GenerationMode, limit: usize) {
            for (n, i) in pairs_of_integer_and_small_signed::<$t>(gm).take(limit) {
                let n_old = n.clone();
                println!("{} << {} = {}", n_old, i, n << i);
            }
        }

        fn $demo_integer_shl_i_ref(gm: GenerationMode, limit: usize) {
            for (n, i) in pairs_of_integer_and_small_signed::<$t>(gm).take(limit) {
                println!("&{} << {} = {}", n, i, &n << i);
            }
        }

        fn $benchmark_integer_shl_i_evaluation_strategy(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            run_benchmark(
                &format!("Integer << {}", $t::NAME),
                BenchmarkType::EvaluationStrategy,
                pairs_of_integer_and_small_signed::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other)| usize::exact_from(other.unsigned_abs())),
                "|other|",
                &mut [
                    (
                        &format!("Integer << {}", $t::NAME),
                        &mut (|(x, y)| no_out!(x << y)),
                    ),
                    (
                        &format!("&Integer << {}", $t::NAME),
                        &mut (|(x, y)| no_out!(&x << y)),
                    ),
                ],
            );
        }
    };
}
demos_and_benches_i!(
    i8,
    demo_integer_shl_assign_i8,
    demo_integer_shl_i8,
    demo_integer_shl_i8_ref,
    benchmark_integer_shl_i8_evaluation_strategy
);
demos_and_benches_i!(
    i16,
    demo_integer_shl_assign_i16,
    demo_integer_shl_i16,
    demo_integer_shl_i16_ref,
    benchmark_integer_shl_i16_evaluation_strategy
);
demos_and_benches_i!(
    i32,
    demo_integer_shl_assign_i32,
    demo_integer_shl_i32,
    demo_integer_shl_i32_ref,
    benchmark_integer_shl_i32_evaluation_strategy
);
demos_and_benches_i!(
    i64,
    demo_integer_shl_assign_i64,
    demo_integer_shl_i64,
    demo_integer_shl_i64_ref,
    benchmark_integer_shl_i64_evaluation_strategy
);
demos_and_benches_i!(
    isize,
    demo_integer_shl_assign_isize,
    demo_integer_shl_isize,
    demo_integer_shl_isize_ref,
    benchmark_integer_shl_isize_evaluation_strategy
);

fn benchmark_integer_shl_assign_i32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer <<= i32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_small_signed::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, other))| usize::exact_from(other.unsigned_abs())),
        "|other|",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x <<= y)),
            ("rug", &mut (|((mut x, y), _)| x <<= y)),
        ],
    );
}

fn benchmark_integer_shl_i32_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "Integer << i32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_small_signed::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, other))| usize::exact_from(other.unsigned_abs())),
        "|other|",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x << y))),
            ("rug", &mut (|((x, y), _)| no_out!(x << y))),
        ],
    );
}
