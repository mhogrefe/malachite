use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::positive_unsigneds;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_floor_log_base_2);
    register_demo!(registry, demo_u16_floor_log_base_2);
    register_demo!(registry, demo_u32_floor_log_base_2);
    register_demo!(registry, demo_u64_floor_log_base_2);
    register_demo!(registry, demo_usize_floor_log_base_2);
    register_demo!(registry, demo_u8_ceiling_log_base_2);
    register_demo!(registry, demo_u16_ceiling_log_base_2);
    register_demo!(registry, demo_u32_ceiling_log_base_2);
    register_demo!(registry, demo_u64_ceiling_log_base_2);
    register_demo!(registry, demo_usize_ceiling_log_base_2);
    register_bench!(registry, None, benchmark_u8_floor_log_base_2);
    register_bench!(registry, None, benchmark_u16_floor_log_base_2);
    register_bench!(registry, None, benchmark_u32_floor_log_base_2);
    register_bench!(registry, None, benchmark_u64_floor_log_base_2);
    register_bench!(registry, None, benchmark_usize_floor_log_base_2);
    register_bench!(registry, None, benchmark_u8_ceiling_log_base_2);
    register_bench!(registry, None, benchmark_u16_ceiling_log_base_2);
    register_bench!(registry, None, benchmark_u32_ceiling_log_base_2);
    register_bench!(registry, None, benchmark_u64_ceiling_log_base_2);
    register_bench!(registry, None, benchmark_usize_ceiling_log_base_2);
}

fn demo_floor_log_base_2<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for n in positive_unsigneds::<T>(gm).take(limit) {
        println!("{}.floor_log_base_2() = {}", n, n.floor_log_base_2());
    }
}

fn demo_ceiling_log_base_2<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for n in positive_unsigneds::<T>(gm).take(limit) {
        println!("{}.ceiling_log_base_2() = {}", n, n.ceiling_log_base_2());
    }
}

fn benchmark_floor_log_base_2<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!("{}.floor_log_base_2()", T::NAME),
        BenchmarkType::Single,
        positive_unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [("Malachite", &mut (|n| no_out!(n.floor_log_base_2())))],
    );
}

fn benchmark_ceiling_log_base_2<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!("{}.ceiling_log_base_2()", T::NAME),
        BenchmarkType::Single,
        positive_unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [("Malachite", &mut (|n| no_out!(n.ceiling_log_base_2())))],
    );
}

macro_rules! unsigned {
    (
        $t:ident,
        $demo_name_floor:ident,
        $demo_name_ceiling:ident,
        $bench_name_floor:ident,
        $bench_name_ceiling:ident
    ) => {
        fn $demo_name_floor(gm: GenerationMode, limit: usize) {
            demo_floor_log_base_2::<$t>(gm, limit);
        }

        fn $demo_name_ceiling(gm: GenerationMode, limit: usize) {
            demo_ceiling_log_base_2::<$t>(gm, limit);
        }

        fn $bench_name_floor(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_floor_log_base_2::<$t>(gm, limit, file_name);
        }

        fn $bench_name_ceiling(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_ceiling_log_base_2::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    demo_u8_floor_log_base_2,
    demo_u8_ceiling_log_base_2,
    benchmark_u8_floor_log_base_2,
    benchmark_u8_ceiling_log_base_2
);
unsigned!(
    u16,
    demo_u16_floor_log_base_2,
    demo_u16_ceiling_log_base_2,
    benchmark_u16_floor_log_base_2,
    benchmark_u16_ceiling_log_base_2
);
unsigned!(
    u32,
    demo_u32_floor_log_base_2,
    demo_u32_ceiling_log_base_2,
    benchmark_u32_floor_log_base_2,
    benchmark_u32_ceiling_log_base_2
);
unsigned!(
    u64,
    demo_u64_floor_log_base_2,
    demo_u64_ceiling_log_base_2,
    benchmark_u64_floor_log_base_2,
    benchmark_u64_ceiling_log_base_2
);
unsigned!(
    usize,
    demo_usize_floor_log_base_2,
    demo_usize_ceiling_log_base_2,
    benchmark_usize_floor_log_base_2,
    benchmark_usize_ceiling_log_base_2
);
