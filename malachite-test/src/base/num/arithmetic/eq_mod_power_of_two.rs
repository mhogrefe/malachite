use std::cmp::max;

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    triples_of_signed_signed_and_small_unsigned, triples_of_unsigned_unsigned_and_small_unsigned,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_eq_mod_power_of_two);
    register_demo!(registry, demo_u16_eq_mod_power_of_two);
    register_demo!(registry, demo_u32_eq_mod_power_of_two);
    register_demo!(registry, demo_u64_eq_mod_power_of_two);
    register_demo!(registry, demo_usize_eq_mod_power_of_two);
    register_demo!(registry, demo_i8_eq_mod_power_of_two);
    register_demo!(registry, demo_i16_eq_mod_power_of_two);
    register_demo!(registry, demo_i32_eq_mod_power_of_two);
    register_demo!(registry, demo_i64_eq_mod_power_of_two);
    register_demo!(registry, demo_isize_eq_mod_power_of_two);

    register_bench!(registry, None, benchmark_u8_eq_mod_power_of_two);
    register_bench!(registry, None, benchmark_u16_eq_mod_power_of_two);
    register_bench!(registry, None, benchmark_u32_eq_mod_power_of_two);
    register_bench!(registry, None, benchmark_u64_eq_mod_power_of_two);
    register_bench!(registry, None, benchmark_usize_eq_mod_power_of_two);
    register_bench!(registry, None, benchmark_i8_eq_mod_power_of_two);
    register_bench!(registry, None, benchmark_i16_eq_mod_power_of_two);
    register_bench!(registry, None, benchmark_i32_eq_mod_power_of_two);
    register_bench!(registry, None, benchmark_i64_eq_mod_power_of_two);
    register_bench!(registry, None, benchmark_isize_eq_mod_power_of_two);
}

fn demo_unsigned_eq_mod_power_of_two<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) {
    for (u, v, pow) in triples_of_unsigned_unsigned_and_small_unsigned::<T, u64>(gm).take(limit) {
        if u.eq_mod_power_of_two(v, pow) {
            println!("{} is equal to {} mod 2^{}", u, v, pow);
        } else {
            println!("{} is not equal to {} mod 2^{}", u, v, pow);
        }
    }
}

fn demo_signed_eq_mod_power_of_two<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (i, j, pow) in triples_of_signed_signed_and_small_unsigned::<T, u64>(gm).take(limit) {
        if i.eq_mod_power_of_two(j, pow) {
            println!("{} is equal to {} mod 2^{}", i, j, pow);
        } else {
            println!("{} is not equal to {} mod 2^{}", i, j, pow);
        }
    }
}

fn benchmark_unsigned_eq_mod_power_of_two<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.eq_mod_power_of_two({}, u64)", T::NAME, T::NAME),
        BenchmarkType::Single,
        triples_of_unsigned_unsigned_and_small_unsigned::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(u, v, _)| usize::exact_from(max(u.significant_bits(), v.significant_bits()))),
        "max(u.significant_bits(), v.significant_bits())",
        &mut [(
            "malachite",
            &mut (|(u, v, pow)| no_out!(u.eq_mod_power_of_two(v, pow))),
        )],
    );
}

fn benchmark_signed_eq_mod_power_of_two<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark(
        &format!("{}.eq_mod_power_of_two({}, u64)", T::NAME, T::NAME),
        BenchmarkType::Single,
        triples_of_signed_signed_and_small_unsigned::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(i, j, _)| usize::exact_from(max(i.significant_bits(), j.significant_bits()))),
        "max(i.significant_bits(), j.significant_bits())",
        &mut [(
            "malachite",
            &mut (|(i, j, pow)| no_out!(i.eq_mod_power_of_two(j, pow))),
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
            demo_unsigned_eq_mod_power_of_two::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_eq_mod_power_of_two::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed {
    (
        $t:ident,
        $demo_name:ident,
        $bench_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_eq_mod_power_of_two::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_eq_mod_power_of_two::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    demo_u8_eq_mod_power_of_two,
    benchmark_u8_eq_mod_power_of_two
);
unsigned!(
    u16,
    demo_u16_eq_mod_power_of_two,
    benchmark_u16_eq_mod_power_of_two
);
unsigned!(
    u32,
    demo_u32_eq_mod_power_of_two,
    benchmark_u32_eq_mod_power_of_two
);
unsigned!(
    u64,
    demo_u64_eq_mod_power_of_two,
    benchmark_u64_eq_mod_power_of_two
);
unsigned!(
    usize,
    demo_usize_eq_mod_power_of_two,
    benchmark_usize_eq_mod_power_of_two
);

signed!(
    i8,
    demo_i8_eq_mod_power_of_two,
    benchmark_i8_eq_mod_power_of_two
);
signed!(
    i16,
    demo_i16_eq_mod_power_of_two,
    benchmark_i16_eq_mod_power_of_two
);
signed!(
    i32,
    demo_i32_eq_mod_power_of_two,
    benchmark_i32_eq_mod_power_of_two
);
signed!(
    i64,
    demo_i64_eq_mod_power_of_two,
    benchmark_i64_eq_mod_power_of_two
);
signed!(
    isize,
    demo_isize_eq_mod_power_of_two,
    benchmark_isize_eq_mod_power_of_two
);
