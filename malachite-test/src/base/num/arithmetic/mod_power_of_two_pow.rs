use std::cmp::max;

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, NoSpecialGenerationMode, ScaleType};
use malachite_test::inputs::base::triples_of_unsigned_unsigned_and_small_u64_var_2;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(registry, demo_u8_mod_power_of_two_pow);
    register_ns_demo!(registry, demo_u16_mod_power_of_two_pow);
    register_ns_demo!(registry, demo_u32_mod_power_of_two_pow);
    register_ns_demo!(registry, demo_u64_mod_power_of_two_pow);
    register_ns_demo!(registry, demo_usize_mod_power_of_two_pow);
    register_ns_demo!(registry, demo_u8_mod_power_of_two_pow_assign);
    register_ns_demo!(registry, demo_u16_mod_power_of_two_pow_assign);
    register_ns_demo!(registry, demo_u32_mod_power_of_two_pow_assign);
    register_ns_demo!(registry, demo_u64_mod_power_of_two_pow_assign);
    register_ns_demo!(registry, demo_usize_mod_power_of_two_pow_assign);

    register_ns_bench!(registry, None, benchmark_u8_mod_power_of_two_pow);
    register_ns_bench!(registry, None, benchmark_u16_mod_power_of_two_pow);
    register_ns_bench!(registry, None, benchmark_u32_mod_power_of_two_pow);
    register_ns_bench!(registry, None, benchmark_u64_mod_power_of_two_pow);
    register_ns_bench!(registry, None, benchmark_usize_mod_power_of_two_pow);
    register_ns_bench!(registry, None, benchmark_u8_mod_power_of_two_pow_assign);
    register_ns_bench!(registry, None, benchmark_u16_mod_power_of_two_pow_assign);
    register_ns_bench!(registry, None, benchmark_u32_mod_power_of_two_pow_assign);
    register_ns_bench!(registry, None, benchmark_u64_mod_power_of_two_pow_assign);
    register_ns_bench!(registry, None, benchmark_usize_mod_power_of_two_pow_assign);
}

fn demo_mod_power_of_two_pow<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: NoSpecialGenerationMode,
    limit: usize,
) {
    for (x, exp, pow) in triples_of_unsigned_unsigned_and_small_u64_var_2::<T>(gm).take(limit) {
        println!(
            "{}.pow({}) === {} mod 2^{}",
            x,
            exp,
            x.mod_power_of_two_pow(exp, pow),
            pow
        );
    }
}

fn demo_mod_power_of_two_pow_assign<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: NoSpecialGenerationMode,
    limit: usize,
) {
    for (mut x, exp, pow) in triples_of_unsigned_unsigned_and_small_u64_var_2::<T>(gm).take(limit) {
        let old_x = x;
        x.mod_power_of_two_pow_assign(exp, pow);
        println!(
            "x := {}; x.mod_power_of_two_pow_assign({}, {}); x = {}",
            old_x, exp, pow, x
        );
    }
}

fn benchmark_mod_power_of_two_pow<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: NoSpecialGenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!("{}.mod_power_of_two_pow(u64, u64)", T::NAME),
        BenchmarkType::Single,
        triples_of_unsigned_unsigned_and_small_u64_var_2::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, exp, _)| usize::exact_from(max(x.significant_bits(), exp.significant_bits()))),
        "max(x.significant_bits(), exp.significant_bits())",
        &mut [(
            "Malachite",
            &mut (|(x, exp, pow)| no_out!(x.mod_power_of_two_pow(exp, pow))),
        )],
    );
}

fn benchmark_mod_power_of_two_pow_assign<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: NoSpecialGenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!("{}.mod_power_of_two_pow_assign({}, u64)", T::NAME, T::NAME),
        BenchmarkType::Single,
        triples_of_unsigned_unsigned_and_small_u64_var_2::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, exp, _)| usize::exact_from(max(x.significant_bits(), exp.significant_bits()))),
        "max(x.significant_bits(), exp.significant_bits())",
        &mut [(
            "Malachite",
            &mut (|(mut x, exp, pow)| x.mod_power_of_two_pow_assign(exp, pow)),
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
            demo_mod_power_of_two_pow::<$t>(gm, limit);
        }

        fn $demo_assign_name(gm: NoSpecialGenerationMode, limit: usize) {
            demo_mod_power_of_two_pow_assign::<$t>(gm, limit);
        }

        fn $bench_name(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_power_of_two_pow::<$t>(gm, limit, file_name);
        }

        fn $bench_assign_name(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_power_of_two_pow_assign::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    demo_u8_mod_power_of_two_pow,
    demo_u8_mod_power_of_two_pow_assign,
    benchmark_u8_mod_power_of_two_pow,
    benchmark_u8_mod_power_of_two_pow_assign
);
unsigned!(
    u16,
    demo_u16_mod_power_of_two_pow,
    demo_u16_mod_power_of_two_pow_assign,
    benchmark_u16_mod_power_of_two_pow,
    benchmark_u16_mod_power_of_two_pow_assign
);
unsigned!(
    u32,
    demo_u32_mod_power_of_two_pow,
    demo_u32_mod_power_of_two_pow_assign,
    benchmark_u32_mod_power_of_two_pow,
    benchmark_u32_mod_power_of_two_pow_assign
);
unsigned!(
    u64,
    demo_u64_mod_power_of_two_pow,
    demo_u64_mod_power_of_two_pow_assign,
    benchmark_u64_mod_power_of_two_pow,
    benchmark_u64_mod_power_of_two_pow_assign
);
unsigned!(
    usize,
    demo_usize_mod_power_of_two_pow,
    demo_usize_mod_power_of_two_pow_assign,
    benchmark_usize_mod_power_of_two_pow,
    benchmark_usize_mod_power_of_two_pow_assign
);
