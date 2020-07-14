use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use rand::Rand;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType,
};
use malachite_test::inputs::base::pairs_of_unsigneds_var_5;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_mod_neg);
    register_demo!(registry, demo_u16_mod_neg);
    register_demo!(registry, demo_u32_mod_neg);
    register_demo!(registry, demo_u64_mod_neg);
    register_demo!(registry, demo_usize_mod_neg);
    register_demo!(registry, demo_u8_mod_neg_assign);
    register_demo!(registry, demo_u16_mod_neg_assign);
    register_demo!(registry, demo_u32_mod_neg_assign);
    register_demo!(registry, demo_u64_mod_neg_assign);
    register_demo!(registry, demo_usize_mod_neg_assign);

    register_bench!(registry, None, benchmark_u8_mod_neg);
    register_bench!(registry, None, benchmark_u16_mod_neg);
    register_bench!(registry, None, benchmark_u32_mod_neg);
    register_bench!(registry, None, benchmark_u64_mod_neg);
    register_bench!(registry, None, benchmark_usize_mod_neg);
    register_bench!(registry, None, benchmark_u8_mod_neg_assign);
    register_bench!(registry, None, benchmark_u16_mod_neg_assign);
    register_bench!(registry, None, benchmark_u32_mod_neg_assign);
    register_bench!(registry, None, benchmark_u64_mod_neg_assign);
    register_bench!(registry, None, benchmark_usize_mod_neg_assign);
}

fn demo_mod_neg<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (n, m) in pairs_of_unsigneds_var_5::<T>(gm).take(limit) {
        println!("-{} === {} mod {}", n, n.mod_neg(m), m);
    }
}

fn demo_mod_neg_assign<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (mut n, m) in pairs_of_unsigneds_var_5::<T>(gm).take(limit) {
        let old_n = n;
        n.mod_neg_assign(m);
        println!("n := {}; n.mod_neg_assign({}); n = {}", old_n, m, n);
    }
}

fn benchmark_mod_neg<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.mod_neg(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigneds_var_5::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(n, m)| no_out!(n.mod_neg(m))))],
    );
}

fn benchmark_mod_neg_assign<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.mod_neg_assign(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigneds_var_5::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(mut n, m)| n.mod_neg_assign(m)))],
    );
}

macro_rules! unsigned {
    (
        $t:ident,
        $demo_name:ident,
        $demo_assign_name:ident,
        $bench_name:ident,
        $bench_assign_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_mod_neg::<$t>(gm, limit);
        }

        fn $demo_assign_name(gm: GenerationMode, limit: usize) {
            demo_mod_neg_assign::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_neg::<$t>(gm, limit, file_name);
        }

        fn $bench_assign_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_neg_assign::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    demo_u8_mod_neg,
    demo_u8_mod_neg_assign,
    benchmark_u8_mod_neg,
    benchmark_u8_mod_neg_assign
);
unsigned!(
    u16,
    demo_u16_mod_neg,
    demo_u16_mod_neg_assign,
    benchmark_u16_mod_neg,
    benchmark_u16_mod_neg_assign
);
unsigned!(
    u32,
    demo_u32_mod_neg,
    demo_u32_mod_neg_assign,
    benchmark_u32_mod_neg,
    benchmark_u32_mod_neg_assign
);
unsigned!(
    u64,
    demo_u64_mod_neg,
    demo_u64_mod_neg_assign,
    benchmark_u64_mod_neg,
    benchmark_u64_mod_neg_assign
);
unsigned!(
    usize,
    demo_usize_mod_neg,
    demo_usize_mod_neg_assign,
    benchmark_usize_mod_neg,
    benchmark_usize_mod_neg_assign
);
