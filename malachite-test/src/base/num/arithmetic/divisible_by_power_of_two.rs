use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use rand::Rand;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{pairs_of_signed_and_small_unsigned, pairs_of_unsigned_and_small_unsigned};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_divisible_by_power_of_two);
    register_demo!(registry, demo_u16_divisible_by_power_of_two);
    register_demo!(registry, demo_u32_divisible_by_power_of_two);
    register_demo!(registry, demo_u64_divisible_by_power_of_two);
    register_demo!(registry, demo_usize_divisible_by_power_of_two);
    register_demo!(registry, demo_i8_divisible_by_power_of_two);
    register_demo!(registry, demo_i16_divisible_by_power_of_two);
    register_demo!(registry, demo_i32_divisible_by_power_of_two);
    register_demo!(registry, demo_i64_divisible_by_power_of_two);
    register_demo!(registry, demo_isize_divisible_by_power_of_two);

    register_bench!(registry, None, benchmark_u8_divisible_by_power_of_two);
    register_bench!(registry, None, benchmark_u16_divisible_by_power_of_two);
    register_bench!(registry, None, benchmark_u32_divisible_by_power_of_two);
    register_bench!(registry, None, benchmark_u64_divisible_by_power_of_two);
    register_bench!(registry, None, benchmark_usize_divisible_by_power_of_two);
    register_bench!(registry, None, benchmark_i8_divisible_by_power_of_two);
    register_bench!(registry, None, benchmark_i16_divisible_by_power_of_two);
    register_bench!(registry, None, benchmark_i32_divisible_by_power_of_two);
    register_bench!(registry, None, benchmark_i64_divisible_by_power_of_two);
    register_bench!(registry, None, benchmark_isize_divisible_by_power_of_two);
}

fn demo_unsigned_divisible_by_power_of_two<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) {
    for (u, pow) in pairs_of_unsigned_and_small_unsigned::<T, u64>(gm).take(limit) {
        if u.divisible_by_power_of_two(pow) {
            println!("{} is divisible by 2^{}", u, pow);
        } else {
            println!("{} is not divisible by 2^{}", u, pow);
        }
    }
}

fn demo_signed_divisible_by_power_of_two<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (i, pow) in pairs_of_signed_and_small_unsigned::<T, u64>(gm).take(limit) {
        if i.divisible_by_power_of_two(pow) {
            println!("{} is divisible by 2^{}", i, pow);
        } else {
            println!("{} is not divisible by 2^{}", i, pow);
        }
    }
}

fn benchmark_unsigned_divisible_by_power_of_two<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.divisible_by_power_of_two(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_small_unsigned::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(u, _)| usize::exact_from(u.significant_bits())),
        "u.significant_bits()",
        &mut [(
            "malachite",
            &mut (|(u, pow)| no_out!(u.divisible_by_power_of_two(pow))),
        )],
    );
}

fn benchmark_signed_divisible_by_power_of_two<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.divisible_by_power_of_two(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_signed_and_small_unsigned::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(i, _)| usize::exact_from(i.significant_bits())),
        "i.significant_bits()",
        &mut [(
            "malachite",
            &mut (|(i, pow)| no_out!(i.divisible_by_power_of_two(pow))),
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
            demo_unsigned_divisible_by_power_of_two::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_divisible_by_power_of_two::<$t>(gm, limit, file_name);
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
            demo_signed_divisible_by_power_of_two::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_divisible_by_power_of_two::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    demo_u8_divisible_by_power_of_two,
    benchmark_u8_divisible_by_power_of_two
);
unsigned!(
    u16,
    demo_u16_divisible_by_power_of_two,
    benchmark_u16_divisible_by_power_of_two
);
unsigned!(
    u32,
    demo_u32_divisible_by_power_of_two,
    benchmark_u32_divisible_by_power_of_two
);
unsigned!(
    u64,
    demo_u64_divisible_by_power_of_two,
    benchmark_u64_divisible_by_power_of_two
);
unsigned!(
    usize,
    demo_usize_divisible_by_power_of_two,
    benchmark_usize_divisible_by_power_of_two
);

signed!(
    i8,
    demo_i8_divisible_by_power_of_two,
    benchmark_i8_divisible_by_power_of_two
);
signed!(
    i16,
    demo_i16_divisible_by_power_of_two,
    benchmark_i16_divisible_by_power_of_two
);
signed!(
    i32,
    demo_i32_divisible_by_power_of_two,
    benchmark_i32_divisible_by_power_of_two
);
signed!(
    i64,
    demo_i64_divisible_by_power_of_two,
    benchmark_i64_divisible_by_power_of_two
);
signed!(
    isize,
    demo_isize_divisible_by_power_of_two,
    benchmark_isize_divisible_by_power_of_two
);
