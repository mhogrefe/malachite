use malachite_base::num::arithmetic::mod_pow::_simple_binary_mod_pow;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::num::arithmetic::mod_pow::_naive_mod_pow;
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    pairs_of_unsigneds_var_5, triples_of_unsigned_small_unsigned_and_unsigned_var_1,
    triples_of_unsigned_unsigned_and_unsigned_var_1,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_mod_pow);
    register_demo!(registry, demo_u16_mod_pow);
    register_demo!(registry, demo_u32_mod_pow);
    register_demo!(registry, demo_u64_mod_pow);
    register_demo!(registry, demo_usize_mod_pow);
    register_demo!(registry, demo_u8_mod_pow_assign);
    register_demo!(registry, demo_u16_mod_pow_assign);
    register_demo!(registry, demo_u32_mod_pow_assign);
    register_demo!(registry, demo_u64_mod_pow_assign);
    register_demo!(registry, demo_usize_mod_pow_assign);

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

    register_bench!(registry, None, benchmark_u8_mod_pow_algorithms);
    register_bench!(registry, None, benchmark_u16_mod_pow_algorithms);
    register_bench!(registry, None, benchmark_u32_mod_pow_algorithms);
    register_bench!(registry, None, benchmark_u64_mod_pow_algorithms);
    register_bench!(registry, None, benchmark_usize_mod_pow_algorithms);
    register_bench!(registry, None, benchmark_u8_mod_pow_naive_algorithms);
    register_bench!(registry, None, benchmark_u16_mod_pow_naive_algorithms);
    register_bench!(registry, None, benchmark_u32_mod_pow_naive_algorithms);
    register_bench!(registry, None, benchmark_u64_mod_pow_naive_algorithms);
    register_bench!(registry, None, benchmark_usize_mod_pow_naive_algorithms);

    register_bench!(registry, None, benchmark_u8_mod_pow_assign);
    register_bench!(registry, None, benchmark_u16_mod_pow_assign);
    register_bench!(registry, None, benchmark_u32_mod_pow_assign);
    register_bench!(registry, None, benchmark_u64_mod_pow_assign);
    register_bench!(registry, None, benchmark_usize_mod_pow_assign);

    register_bench!(registry, None, benchmark_u8_mod_pow_precomputed_algorithms);
    register_bench!(registry, None, benchmark_u16_mod_pow_precomputed_algorithms);
    register_bench!(registry, None, benchmark_u32_mod_pow_precomputed_algorithms);
    register_bench!(registry, None, benchmark_u64_mod_pow_precomputed_algorithms);
    register_bench!(
        registry,
        None,
        benchmark_usize_mod_pow_precomputed_algorithms
    );

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

fn demo_mod_pow<T: PrimitiveUnsigned + Rand + SampleRange>(gm: GenerationMode, limit: usize) {
    for (x, exp, m) in triples_of_unsigned_unsigned_and_unsigned_var_1::<T, u64>(gm).take(limit) {
        println!("{}.pow({}) === {} mod {}", x, exp, x.mod_pow(exp, m), m);
    }
}

fn demo_mod_pow_assign<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
) {
    for (mut x, exp, m) in triples_of_unsigned_unsigned_and_unsigned_var_1::<T, u64>(gm).take(limit)
    {
        let old_x = x;
        x.mod_pow_assign(exp, m);
        println!(
            "x := {}; x.mod_pow_assign({}, {}); x = {}",
            old_x, exp, m, x
        );
    }
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

fn benchmark_mod_pow_algorithms<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_pow(u64, {})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        triples_of_unsigned_unsigned_and_unsigned_var_1::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, exp, _)| usize::exact_from(x.significant_bits() * exp.significant_bits())),
        "x.significant_bits() * exp.significant_bits()",
        &mut [
            ("default", &mut (|(x, exp, m)| no_out!(x.mod_pow(exp, m)))),
            (
                "simple binary",
                &mut (|(x, exp, m)| no_out!(_simple_binary_mod_pow(x, exp, m))),
            ),
        ],
    );
}

fn benchmark_mod_pow_naive_algorithms<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_pow(u64, {})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        triples_of_unsigned_small_unsigned_and_unsigned_var_1::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, exp, _)| usize::exact_from(x.significant_bits() * exp.significant_bits())),
        "x.significant_bits() * exp.significant_bits()",
        &mut [
            ("default", &mut (|(x, exp, m)| no_out!(x.mod_pow(exp, m)))),
            (
                "naive",
                &mut (|(x, exp, m)| no_out!(_naive_mod_pow(x, exp, m))),
            ),
            (
                "simple binary",
                &mut (|(x, exp, m)| no_out!(_simple_binary_mod_pow(x, exp, m))),
            ),
        ],
    );
}

fn benchmark_mod_pow_assign<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_pow_assign(u64, {})", T::NAME, T::NAME),
        BenchmarkType::Single,
        triples_of_unsigned_unsigned_and_unsigned_var_1::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, exp, _)| usize::exact_from(x.significant_bits() * exp.significant_bits())),
        "x.significant_bits() * exp.significant_bits()",
        &mut [(
            "malachite",
            &mut (|(mut x, exp, m)| x.mod_pow_assign(exp, m)),
        )],
    );
}

fn benchmark_mod_pow_precomputed_algorithms<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_pow(u64, {})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        triples_of_unsigned_unsigned_and_unsigned_var_1::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, exp, _)| usize::exact_from(x.significant_bits() * exp.significant_bits())),
        "x.significant_bits() * exp.significant_bits()",
        &mut [
            (
                "default",
                &mut (|(x, exp, m)| {
                    for _ in 0..10 {
                        x.mod_pow(exp, m);
                    }
                }),
            ),
            (
                "precomputed",
                &mut (|(x, exp, m)| {
                    let data = T::precompute_mod_pow_data(&m);
                    for _ in 0..10 {
                        x.mod_pow_precomputed(exp, m, &data);
                    }
                }),
            ),
        ],
    );
}

fn benchmark_mod_square<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_square({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigneds_var_5::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, _)| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [("malachite", &mut (|(x, m)| no_out!(x.mod_square(m))))],
    );
}

fn benchmark_mod_square_assign<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_square_assign({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigneds_var_5::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, _)| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [("malachite", &mut (|(mut x, m)| x.mod_square_assign(m)))],
    );
}

fn benchmark_mod_square_precomputed_algorithms<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
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

macro_rules! mod_pow_unsigned {
    (
        $t:ident,
        $demo_name:ident,
        $demo_assign_name:ident,
        $bench_name:ident,
        $bench_naive_name:ident,
        $bench_assign_name:ident,
        $bench_precomputed_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_mod_pow::<$t>(gm, limit);
        }

        fn $demo_assign_name(gm: GenerationMode, limit: usize) {
            demo_mod_pow_assign::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_pow_algorithms::<$t>(gm, limit, file_name);
        }

        fn $bench_naive_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_pow_naive_algorithms::<$t>(gm, limit, file_name);
        }

        fn $bench_assign_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_pow_assign::<$t>(gm, limit, file_name);
        }

        fn $bench_precomputed_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_pow_precomputed_algorithms::<$t>(gm, limit, file_name);
        }
    };
}

mod_pow_unsigned!(
    u8,
    demo_u8_mod_pow,
    demo_u8_mod_pow_assign,
    benchmark_u8_mod_pow_algorithms,
    benchmark_u8_mod_pow_naive_algorithms,
    benchmark_u8_mod_pow_assign,
    benchmark_u8_mod_pow_precomputed_algorithms
);
mod_pow_unsigned!(
    u16,
    demo_u16_mod_pow,
    demo_u16_mod_pow_assign,
    benchmark_u16_mod_pow_algorithms,
    benchmark_u16_mod_pow_naive_algorithms,
    benchmark_u16_mod_pow_assign,
    benchmark_u16_mod_pow_precomputed_algorithms
);
mod_pow_unsigned!(
    u32,
    demo_u32_mod_pow,
    demo_u32_mod_pow_assign,
    benchmark_u32_mod_pow_algorithms,
    benchmark_u32_mod_pow_naive_algorithms,
    benchmark_u32_mod_pow_assign,
    benchmark_u32_mod_pow_precomputed_algorithms
);
mod_pow_unsigned!(
    u64,
    demo_u64_mod_pow,
    demo_u64_mod_pow_assign,
    benchmark_u64_mod_pow_algorithms,
    benchmark_u64_mod_pow_naive_algorithms,
    benchmark_u64_mod_pow_assign,
    benchmark_u64_mod_pow_precomputed_algorithms
);
mod_pow_unsigned!(
    usize,
    demo_usize_mod_pow,
    demo_usize_mod_pow_assign,
    benchmark_usize_mod_pow_algorithms,
    benchmark_usize_mod_pow_naive_algorithms,
    benchmark_usize_mod_pow_assign,
    benchmark_usize_mod_pow_precomputed_algorithms
);

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
