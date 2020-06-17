use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use rand::Rand;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::unsigneds_var_2;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_next_power_of_two);
    register_demo!(registry, demo_u16_next_power_of_two);
    register_demo!(registry, demo_u32_next_power_of_two);
    register_demo!(registry, demo_u64_next_power_of_two);
    register_demo!(registry, demo_usize_next_power_of_two);
    register_demo!(registry, demo_u8_next_power_of_two_assign);
    register_demo!(registry, demo_u16_next_power_of_two_assign);
    register_demo!(registry, demo_u32_next_power_of_two_assign);
    register_demo!(registry, demo_u64_next_power_of_two_assign);
    register_demo!(registry, demo_usize_next_power_of_two_assign);

    register_bench!(registry, None, benchmark_u8_next_power_of_two);
    register_bench!(registry, None, benchmark_u16_next_power_of_two);
    register_bench!(registry, None, benchmark_u32_next_power_of_two);
    register_bench!(registry, None, benchmark_u64_next_power_of_two);
    register_bench!(registry, None, benchmark_usize_next_power_of_two);
    register_bench!(registry, None, benchmark_u8_next_power_of_two_assign);
    register_bench!(registry, None, benchmark_u16_next_power_of_two_assign);
    register_bench!(registry, None, benchmark_u32_next_power_of_two_assign);
    register_bench!(registry, None, benchmark_u64_next_power_of_two_assign);
    register_bench!(registry, None, benchmark_usize_next_power_of_two_assign);
}

fn demo_next_power_of_two<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for n in unsigneds_var_2::<T>(gm).take(limit) {
        println!("{}.next_power_of_two() = {}", n, n.next_power_of_two());
    }
}

fn demo_next_power_of_two_assign<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for mut n in unsigneds_var_2::<T>(gm).take(limit) {
        let old_n = n;
        n.next_power_of_two_assign();
        println!("n := {}; n.next_power_of_two_assign(); n = {}", old_n, n);
    }
}

fn benchmark_next_power_of_two<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.next_power_of_two()", T::NAME),
        BenchmarkType::Single,
        unsigneds_var_2::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [("malachite", &mut (|n| no_out!(n.next_power_of_two())))],
    );
}

fn benchmark_next_power_of_two_assign<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.next_power_of_two_assign()", T::NAME),
        BenchmarkType::Single,
        unsigneds_var_2::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [("malachite", &mut (|mut n| n.next_power_of_two_assign()))],
    );
}

macro_rules! demo_and_bench {
    (
        $t:ident,
        $demo_name:ident,
        $demo_assign_name:ident,
        $bench_name:ident,
        $bench_assign_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_next_power_of_two::<$t>(gm, limit);
        }

        fn $demo_assign_name(gm: GenerationMode, limit: usize) {
            demo_next_power_of_two_assign::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_next_power_of_two::<$t>(gm, limit, file_name);
        }

        fn $bench_assign_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_next_power_of_two_assign::<$t>(gm, limit, file_name);
        }
    };
}

demo_and_bench!(
    u8,
    demo_u8_next_power_of_two,
    demo_u8_next_power_of_two_assign,
    benchmark_u8_next_power_of_two,
    benchmark_u8_next_power_of_two_assign
);
demo_and_bench!(
    u16,
    demo_u16_next_power_of_two,
    demo_u16_next_power_of_two_assign,
    benchmark_u16_next_power_of_two,
    benchmark_u16_next_power_of_two_assign
);
demo_and_bench!(
    u32,
    demo_u32_next_power_of_two,
    demo_u32_next_power_of_two_assign,
    benchmark_u32_next_power_of_two,
    benchmark_u32_next_power_of_two_assign
);
demo_and_bench!(
    u64,
    demo_u64_next_power_of_two,
    demo_u64_next_power_of_two_assign,
    benchmark_u64_next_power_of_two,
    benchmark_u64_next_power_of_two_assign
);
demo_and_bench!(
    usize,
    demo_usize_next_power_of_two,
    demo_usize_next_power_of_two_assign,
    benchmark_usize_next_power_of_two,
    benchmark_usize_next_power_of_two_assign
);
