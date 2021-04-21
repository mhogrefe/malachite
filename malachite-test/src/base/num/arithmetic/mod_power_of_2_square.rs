use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, NoSpecialGenerationMode, ScaleType};
use malachite_test::inputs::base::pairs_of_unsigned_and_small_u64_var_2;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(registry, demo_u8_mod_power_of_2_square);
    register_ns_demo!(registry, demo_u16_mod_power_of_2_square);
    register_ns_demo!(registry, demo_u32_mod_power_of_2_square);
    register_ns_demo!(registry, demo_u64_mod_power_of_2_square);
    register_ns_demo!(registry, demo_usize_mod_power_of_2_square);
    register_ns_demo!(registry, demo_u8_mod_power_of_2_square_assign);
    register_ns_demo!(registry, demo_u16_mod_power_of_2_square_assign);
    register_ns_demo!(registry, demo_u32_mod_power_of_2_square_assign);
    register_ns_demo!(registry, demo_u64_mod_power_of_2_square_assign);
    register_ns_demo!(registry, demo_usize_mod_power_of_2_square_assign);

    register_ns_bench!(registry, None, benchmark_u8_mod_power_of_2_square);
    register_ns_bench!(registry, None, benchmark_u16_mod_power_of_2_square);
    register_ns_bench!(registry, None, benchmark_u32_mod_power_of_2_square);
    register_ns_bench!(registry, None, benchmark_u64_mod_power_of_2_square);
    register_ns_bench!(registry, None, benchmark_usize_mod_power_of_2_square);
    register_ns_bench!(registry, None, benchmark_u8_mod_power_of_2_square_assign);
    register_ns_bench!(registry, None, benchmark_u16_mod_power_of_2_square_assign);
    register_ns_bench!(registry, None, benchmark_u32_mod_power_of_2_square_assign);
    register_ns_bench!(registry, None, benchmark_u64_mod_power_of_2_square_assign);
    register_ns_bench!(registry, None, benchmark_usize_mod_power_of_2_square_assign);
}

fn demo_mod_power_of_2_square<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: NoSpecialGenerationMode,
    limit: usize,
) {
    for (n, pow) in pairs_of_unsigned_and_small_u64_var_2::<T>(gm).take(limit) {
        println!(
            "{}.square() === {} mod 2^{}",
            n,
            n.mod_power_of_2_square(pow),
            pow
        );
    }
}

fn demo_mod_power_of_2_square_assign<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: NoSpecialGenerationMode,
    limit: usize,
) {
    for (mut n, pow) in pairs_of_unsigned_and_small_u64_var_2::<T>(gm).take(limit) {
        let old_n = n;
        n.mod_power_of_2_square_assign(pow);
        println!(
            "n := {}; n.mod_power_of_2_square_assign({}); n = {}",
            old_n, pow, n
        );
    }
}

fn benchmark_mod_power_of_2_square<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: NoSpecialGenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!("{}.mod_power_of_2_square(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_small_u64_var_2::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [(
            "Malachite",
            &mut (|(n, pow)| no_out!(n.mod_power_of_2_square(pow))),
        )],
    );
}

fn benchmark_mod_power_of_2_square_assign<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: NoSpecialGenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!("{}.mod_power_of_2_square_assign(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_small_u64_var_2::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [(
            "Malachite",
            &mut (|(mut n, pow)| n.mod_power_of_2_square_assign(pow)),
        )],
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
        fn $demo_name(gm: NoSpecialGenerationMode, limit: usize) {
            demo_mod_power_of_2_square::<$t>(gm, limit);
        }

        fn $demo_assign_name(gm: NoSpecialGenerationMode, limit: usize) {
            demo_mod_power_of_2_square_assign::<$t>(gm, limit);
        }

        fn $bench_name(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_power_of_2_square::<$t>(gm, limit, file_name);
        }

        fn $bench_assign_name(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_power_of_2_square_assign::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    demo_u8_mod_power_of_2_square,
    demo_u8_mod_power_of_2_square_assign,
    benchmark_u8_mod_power_of_2_square,
    benchmark_u8_mod_power_of_2_square_assign
);
unsigned!(
    u16,
    demo_u16_mod_power_of_2_square,
    demo_u16_mod_power_of_2_square_assign,
    benchmark_u16_mod_power_of_2_square,
    benchmark_u16_mod_power_of_2_square_assign
);
unsigned!(
    u32,
    demo_u32_mod_power_of_2_square,
    demo_u32_mod_power_of_2_square_assign,
    benchmark_u32_mod_power_of_2_square,
    benchmark_u32_mod_power_of_2_square_assign
);
unsigned!(
    u64,
    demo_u64_mod_power_of_2_square,
    demo_u64_mod_power_of_2_square_assign,
    benchmark_u64_mod_power_of_2_square,
    benchmark_u64_mod_power_of_2_square_assign
);
unsigned!(
    usize,
    demo_usize_mod_power_of_2_square,
    demo_usize_mod_power_of_2_square_assign,
    benchmark_usize_mod_power_of_2_square,
    benchmark_usize_mod_power_of_2_square_assign
);
