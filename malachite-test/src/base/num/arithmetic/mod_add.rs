use std::cmp::max;

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use rand::distributions::range::SampleRange;
use rand::Rand;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::triples_of_unsigneds_var_1;

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
    for (x, y, modulus) in triples_of_unsigneds_var_1::<T>(gm).take(limit) {
        println!(
            "{} + {} === {} mod {}",
            x,
            y,
            x.mod_add(y, modulus),
            modulus
        );
    }
}

fn demo_mod_add_assign<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
) {
    for (mut x, y, modulus) in triples_of_unsigneds_var_1::<T>(gm).take(limit) {
        let old_x = x;
        x.mod_add_assign(y, modulus);
        println!(
            "x := {}; x.mod_add_assign({}, {}); x = {}",
            old_x, y, modulus, x
        );
    }
}

fn benchmark_mod_add<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.mod_add({}, u64)", T::NAME, T::NAME),
        BenchmarkType::Single,
        triples_of_unsigneds_var_1::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y, _)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [(
            "malachite",
            &mut (|(x, y, modulus)| no_out!(x.mod_add(y, modulus))),
        )],
    );
}

fn benchmark_mod_add_assign<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.mod_add_assign({}, u64)", T::NAME, T::NAME),
        BenchmarkType::Single,
        triples_of_unsigneds_var_1::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y, _)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [(
            "malachite",
            &mut (|(mut x, y, modulus)| x.mod_add_assign(y, modulus)),
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
