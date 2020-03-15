use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use rand::Rand;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::pairs_of_unsigned_and_small_unsigned;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_mod_power_of_two_is_reduced);
    register_demo!(registry, demo_u16_mod_power_of_two_is_reduced);
    register_demo!(registry, demo_u32_mod_power_of_two_is_reduced);
    register_demo!(registry, demo_u64_mod_power_of_two_is_reduced);
    register_demo!(registry, demo_usize_mod_power_of_two_is_reduced);
    register_bench!(registry, None, benchmark_u8_mod_power_of_two_is_reduced);
    register_bench!(registry, None, benchmark_u16_mod_power_of_two_is_reduced);
    register_bench!(registry, None, benchmark_u32_mod_power_of_two_is_reduced);
    register_bench!(registry, None, benchmark_u64_mod_power_of_two_is_reduced);
    register_bench!(registry, None, benchmark_usize_mod_power_of_two_is_reduced);
}

fn demo_mod_power_of_two_is_reduced<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (n, log_base) in pairs_of_unsigned_and_small_unsigned::<T, u64>(gm).take(limit) {
        if n.mod_power_of_two_is_reduced(log_base) {
            println!("{} is reduced mod 2^{}", n, log_base);
        } else {
            println!("{} is not reduced mod 2^{}", n, log_base);
        }
    }
}

fn benchmark_mod_power_of_two_is_reduced<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.mod_power_of_two_is_reduced(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_small_unsigned::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [(
            "malachite",
            &mut (|(n, log_base)| no_out!(n.mod_power_of_two_is_reduced(log_base))),
        )],
    );
}

macro_rules! unsigned {
    (
        $t:ident,
        $demo_name:ident,
        $bench_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_mod_power_of_two_is_reduced::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_power_of_two_is_reduced::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    demo_u8_mod_power_of_two_is_reduced,
    benchmark_u8_mod_power_of_two_is_reduced
);
unsigned!(
    u16,
    demo_u16_mod_power_of_two_is_reduced,
    benchmark_u16_mod_power_of_two_is_reduced
);
unsigned!(
    u32,
    demo_u32_mod_power_of_two_is_reduced,
    benchmark_u32_mod_power_of_two_is_reduced
);
unsigned!(
    u64,
    demo_u64_mod_power_of_two_is_reduced,
    benchmark_u64_mod_power_of_two_is_reduced
);
unsigned!(
    usize,
    demo_usize_mod_power_of_two_is_reduced,
    benchmark_usize_mod_power_of_two_is_reduced
);
