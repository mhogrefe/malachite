use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::pairs_of_unsigneds_var_4;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_mod_is_reduced);
    register_demo!(registry, demo_u16_mod_is_reduced);
    register_demo!(registry, demo_u32_mod_is_reduced);
    register_demo!(registry, demo_u64_mod_is_reduced);
    register_demo!(registry, demo_usize_mod_is_reduced);
    register_bench!(registry, None, benchmark_u8_mod_is_reduced);
    register_bench!(registry, None, benchmark_u16_mod_is_reduced);
    register_bench!(registry, None, benchmark_u32_mod_is_reduced);
    register_bench!(registry, None, benchmark_u64_mod_is_reduced);
    register_bench!(registry, None, benchmark_usize_mod_is_reduced);
}

fn demo_mod_is_reduced<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (n, m) in pairs_of_unsigneds_var_4::<T>(gm).take(limit) {
        if n.mod_is_reduced(&m) {
            println!("{} is reduced mod {}", n, m);
        } else {
            println!("{} is not reduced mod {}", n, m);
        }
    }
}

fn benchmark_mod_is_reduced<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_is_reduced(&{})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigneds_var_4::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(n, m)| no_out!(n.mod_is_reduced(&m))))],
    );
}

macro_rules! unsigned {
    (
        $t:ident,
        $demo_name:ident,
        $bench_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_mod_is_reduced::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_is_reduced::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(u8, demo_u8_mod_is_reduced, benchmark_u8_mod_is_reduced);
unsigned!(u16, demo_u16_mod_is_reduced, benchmark_u16_mod_is_reduced);
unsigned!(u32, demo_u32_mod_is_reduced, benchmark_u32_mod_is_reduced);
unsigned!(u64, demo_u64_mod_is_reduced, benchmark_u64_mod_is_reduced);
unsigned!(
    usize,
    demo_usize_mod_is_reduced,
    benchmark_usize_mod_is_reduced
);
