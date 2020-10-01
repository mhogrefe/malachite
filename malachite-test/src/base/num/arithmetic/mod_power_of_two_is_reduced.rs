use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::pairs_of_unsigned_and_small_unsigned;

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
    for (n, pow) in pairs_of_unsigned_and_small_unsigned::<T, u64>(gm).take(limit) {
        if n.mod_power_of_two_is_reduced(pow) {
            println!("{} is reduced mod 2^{}", n, pow);
        } else {
            println!("{} is not reduced mod 2^{}", n, pow);
        }
    }
}

fn benchmark_mod_power_of_two_is_reduced<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!("{}.mod_power_of_two_is_reduced(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_small_unsigned::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [(
            "Malachite",
            &mut (|(n, pow)| no_out!(n.mod_power_of_two_is_reduced(pow))),
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
