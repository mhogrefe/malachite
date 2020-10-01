use std::cmp::max;

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::triples_of_unsigneds_var_1;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_mod_add);
    register_demo!(registry, demo_u16_mod_add);
    register_demo!(registry, demo_u32_mod_add);
    register_demo!(registry, demo_u64_mod_add);
    register_demo!(registry, demo_usize_mod_add);
    register_demo!(registry, demo_u8_mod_add_assign);
    register_demo!(registry, demo_u16_mod_add_assign);
    register_demo!(registry, demo_u32_mod_add_assign);
    register_demo!(registry, demo_u64_mod_add_assign);
    register_demo!(registry, demo_usize_mod_add_assign);

    register_bench!(registry, None, benchmark_u8_mod_add);
    register_bench!(registry, None, benchmark_u16_mod_add);
    register_bench!(registry, None, benchmark_u32_mod_add);
    register_bench!(registry, None, benchmark_u64_mod_add);
    register_bench!(registry, None, benchmark_usize_mod_add);
    register_bench!(registry, None, benchmark_u8_mod_add_assign);
    register_bench!(registry, None, benchmark_u16_mod_add_assign);
    register_bench!(registry, None, benchmark_u32_mod_add_assign);
    register_bench!(registry, None, benchmark_u64_mod_add_assign);
    register_bench!(registry, None, benchmark_usize_mod_add_assign);
}

fn demo_mod_add<T: PrimitiveUnsigned + Rand + SampleRange>(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_unsigneds_var_1::<T>(gm).take(limit) {
        println!("{} + {} === {} mod {}", x, y, x.mod_add(y, m), m);
    }
}

fn demo_mod_add_assign<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
) {
    for (mut x, y, m) in triples_of_unsigneds_var_1::<T>(gm).take(limit) {
        let old_x = x;
        x.mod_add_assign(y, m);
        println!("x := {}; x.mod_add_assign({}, {}); x = {}", old_x, y, m, x);
    }
}

fn benchmark_mod_add<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!("{}.mod_add({}, u64)", T::NAME, T::NAME),
        BenchmarkType::Single,
        triples_of_unsigneds_var_1::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y, _)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("Malachite", &mut (|(x, y, m)| no_out!(x.mod_add(y, m))))],
    );
}

fn benchmark_mod_add_assign<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!("{}.mod_add_assign({}, u64)", T::NAME, T::NAME),
        BenchmarkType::Single,
        triples_of_unsigneds_var_1::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y, _)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("Malachite", &mut (|(mut x, y, m)| x.mod_add_assign(y, m)))],
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
            demo_mod_add::<$t>(gm, limit);
        }

        fn $demo_assign_name(gm: GenerationMode, limit: usize) {
            demo_mod_add_assign::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_add::<$t>(gm, limit, file_name);
        }

        fn $bench_assign_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_add_assign::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    demo_u8_mod_add,
    demo_u8_mod_add_assign,
    benchmark_u8_mod_add,
    benchmark_u8_mod_add_assign
);
unsigned!(
    u16,
    demo_u16_mod_add,
    demo_u16_mod_add_assign,
    benchmark_u16_mod_add,
    benchmark_u16_mod_add_assign
);
unsigned!(
    u32,
    demo_u32_mod_add,
    demo_u32_mod_add_assign,
    benchmark_u32_mod_add,
    benchmark_u32_mod_add_assign
);
unsigned!(
    u64,
    demo_u64_mod_add,
    demo_u64_mod_add_assign,
    benchmark_u64_mod_add,
    benchmark_u64_mod_add_assign
);
unsigned!(
    usize,
    demo_usize_mod_add,
    demo_usize_mod_add_assign,
    benchmark_usize_mod_add,
    benchmark_usize_mod_add_assign
);
