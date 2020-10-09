use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::pairs_of_unsigneds_var_5;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_mod_square);
    register_demo!(registry, demo_u16_mod_square);
    register_demo!(registry, demo_u32_mod_square);
    register_demo!(registry, demo_u64_mod_square);
    register_demo!(registry, demo_usize_mod_square);
    register_demo!(registry, demo_u8_mod_square_assign);
    register_demo!(registry, demo_u16_mod_square_assign);
    register_demo!(registry, demo_u32_mod_square_assign);
    register_demo!(registry, demo_u64_mod_square_assign);
    register_demo!(registry, demo_usize_mod_square_assign);

    register_bench!(registry, None, benchmark_u8_mod_square);
    register_bench!(registry, None, benchmark_u16_mod_square);
    register_bench!(registry, None, benchmark_u32_mod_square);
    register_bench!(registry, None, benchmark_u64_mod_square);
    register_bench!(registry, None, benchmark_usize_mod_square);

    register_bench!(registry, None, benchmark_u8_mod_square_assign);
    register_bench!(registry, None, benchmark_u16_mod_square_assign);
    register_bench!(registry, None, benchmark_u32_mod_square_assign);
    register_bench!(registry, None, benchmark_u64_mod_square_assign);
    register_bench!(registry, None, benchmark_usize_mod_square_assign);

    register_bench!(
        registry,
        None,
        benchmark_u8_mod_square_precomputed_algorithms
    );
    register_bench!(
        registry,
        None,
        benchmark_u16_mod_square_precomputed_algorithms
    );
    register_bench!(
        registry,
        None,
        benchmark_u32_mod_square_precomputed_algorithms
    );
    register_bench!(
        registry,
        None,
        benchmark_u64_mod_square_precomputed_algorithms
    );
    register_bench!(
        registry,
        None,
        benchmark_usize_mod_square_precomputed_algorithms
    );
}

fn demo_mod_square<T: PrimitiveUnsigned + Rand + SampleRange>(gm: GenerationMode, limit: usize) {
    for (x, m) in pairs_of_unsigneds_var_5::<T>(gm).take(limit) {
        println!("{}.square() === {} mod {}", x, x.mod_square(m), m);
    }
}

fn demo_mod_square_assign<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
) {
    for (mut x, m) in pairs_of_unsigneds_var_5::<T>(gm).take(limit) {
        let old_x = x;
        x.mod_square_assign(m);
        println!("x := {}; x.mod_square_assign({}); x = {}", old_x, m, x);
    }
}

fn benchmark_mod_square<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!("{}.mod_square({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigneds_var_5::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, _)| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [("Malachite", &mut (|(x, m)| no_out!(x.mod_square(m))))],
    );
}

fn benchmark_mod_square_assign<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!("{}.mod_square_assign({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigneds_var_5::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, _)| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [("Malachite", &mut (|(mut x, m)| x.mod_square_assign(m)))],
    );
}

fn benchmark_mod_square_precomputed_algorithms<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!("{}.mod_square({})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        pairs_of_unsigneds_var_5::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, _)| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [
            (
                "default",
                &mut (|(x, m)| {
                    for _ in 0..10 {
                        x.mod_square(m);
                    }
                }),
            ),
            (
                "precomputed",
                &mut (|(x, m)| {
                    let data = T::precompute_mod_pow_data(&m);
                    for _ in 0..10 {
                        x.mod_square_precomputed(m, &data);
                    }
                }),
            ),
        ],
    );
}

macro_rules! mod_square_unsigned {
    (
        $t:ident,
        $demo_name:ident,
        $demo_assign_name:ident,
        $bench_name:ident,
        $bench_assign_name:ident,
        $bench_precomputed_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_mod_square::<$t>(gm, limit);
        }

        fn $demo_assign_name(gm: GenerationMode, limit: usize) {
            demo_mod_square_assign::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_square::<$t>(gm, limit, file_name);
        }

        fn $bench_assign_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_square_assign::<$t>(gm, limit, file_name);
        }

        fn $bench_precomputed_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_square_precomputed_algorithms::<$t>(gm, limit, file_name);
        }
    };
}

mod_square_unsigned!(
    u8,
    demo_u8_mod_square,
    demo_u8_mod_square_assign,
    benchmark_u8_mod_square,
    benchmark_u8_mod_square_assign,
    benchmark_u8_mod_square_precomputed_algorithms
);
mod_square_unsigned!(
    u16,
    demo_u16_mod_square,
    demo_u16_mod_square_assign,
    benchmark_u16_mod_square,
    benchmark_u16_mod_square_assign,
    benchmark_u16_mod_square_precomputed_algorithms
);
mod_square_unsigned!(
    u32,
    demo_u32_mod_square,
    demo_u32_mod_square_assign,
    benchmark_u32_mod_square,
    benchmark_u32_mod_square_assign,
    benchmark_u32_mod_square_precomputed_algorithms
);
mod_square_unsigned!(
    u64,
    demo_u64_mod_square,
    demo_u64_mod_square_assign,
    benchmark_u64_mod_square,
    benchmark_u64_mod_square_assign,
    benchmark_u64_mod_square_precomputed_algorithms
);
mod_square_unsigned!(
    usize,
    demo_usize_mod_square,
    demo_usize_mod_square_assign,
    benchmark_usize_mod_square,
    benchmark_usize_mod_square_assign,
    benchmark_usize_mod_square_precomputed_algorithms
);
