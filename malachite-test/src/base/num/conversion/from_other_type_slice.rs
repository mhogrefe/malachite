use std::fmt::Display;

use malachite_base::named::Named;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::FromOtherTypeSlice;
use rand::Rand;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType,
};
use malachite_test::inputs::base::vecs_of_unsigned;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_from_other_type_slice_u8);
    register_demo!(registry, demo_u16_from_other_type_slice_u8);
    register_demo!(registry, demo_u32_from_other_type_slice_u8);
    register_demo!(registry, demo_u64_from_other_type_slice_u8);
    register_demo!(registry, demo_u128_from_other_type_slice_u8);
    register_demo!(registry, demo_usize_from_other_type_slice_u8);
    register_demo!(registry, demo_u8_from_other_type_slice_u16);
    register_demo!(registry, demo_u16_from_other_type_slice_u16);
    register_demo!(registry, demo_u32_from_other_type_slice_u16);
    register_demo!(registry, demo_u64_from_other_type_slice_u16);
    register_demo!(registry, demo_u128_from_other_type_slice_u16);
    register_demo!(registry, demo_usize_from_other_type_slice_u16);
    register_demo!(registry, demo_u8_from_other_type_slice_u32);
    register_demo!(registry, demo_u16_from_other_type_slice_u32);
    register_demo!(registry, demo_u32_from_other_type_slice_u32);
    register_demo!(registry, demo_u64_from_other_type_slice_u32);
    register_demo!(registry, demo_u128_from_other_type_slice_u32);
    register_demo!(registry, demo_usize_from_other_type_slice_u32);
    register_demo!(registry, demo_u8_from_other_type_slice_u64);
    register_demo!(registry, demo_u16_from_other_type_slice_u64);
    register_demo!(registry, demo_u32_from_other_type_slice_u64);
    register_demo!(registry, demo_u64_from_other_type_slice_u64);
    register_demo!(registry, demo_u128_from_other_type_slice_u64);
    register_demo!(registry, demo_usize_from_other_type_slice_u64);
    register_demo!(registry, demo_u8_from_other_type_slice_usize);
    register_demo!(registry, demo_u16_from_other_type_slice_usize);
    register_demo!(registry, demo_u32_from_other_type_slice_usize);
    register_demo!(registry, demo_u64_from_other_type_slice_usize);
    register_demo!(registry, demo_u128_from_other_type_slice_usize);
    register_demo!(registry, demo_usize_from_other_type_slice_usize);
    register_bench!(registry, Small, benchmark_u8_from_other_type_slice_u8);
    register_bench!(registry, Small, benchmark_u16_from_other_type_slice_u8);
    register_bench!(registry, Small, benchmark_u32_from_other_type_slice_u8);
    register_bench!(registry, Small, benchmark_u64_from_other_type_slice_u8);
    register_bench!(registry, Small, benchmark_u128_from_other_type_slice_u8);
    register_bench!(registry, Small, benchmark_usize_from_other_type_slice_u8);
    register_bench!(registry, Small, benchmark_u8_from_other_type_slice_u16);
    register_bench!(registry, Small, benchmark_u16_from_other_type_slice_u16);
    register_bench!(registry, Small, benchmark_u32_from_other_type_slice_u16);
    register_bench!(registry, Small, benchmark_u64_from_other_type_slice_u16);
    register_bench!(registry, Small, benchmark_u128_from_other_type_slice_u16);
    register_bench!(registry, Small, benchmark_usize_from_other_type_slice_u16);
    register_bench!(registry, Small, benchmark_u8_from_other_type_slice_u32);
    register_bench!(registry, Small, benchmark_u16_from_other_type_slice_u32);
    register_bench!(registry, Small, benchmark_u32_from_other_type_slice_u32);
    register_bench!(registry, Small, benchmark_u64_from_other_type_slice_u32);
    register_bench!(registry, Small, benchmark_u128_from_other_type_slice_u32);
    register_bench!(registry, Small, benchmark_usize_from_other_type_slice_u32);
    register_bench!(registry, Small, benchmark_u8_from_other_type_slice_u64);
    register_bench!(registry, Small, benchmark_u16_from_other_type_slice_u64);
    register_bench!(registry, Small, benchmark_u32_from_other_type_slice_u64);
    register_bench!(registry, Small, benchmark_u64_from_other_type_slice_u64);
    register_bench!(registry, Small, benchmark_u128_from_other_type_slice_u64);
    register_bench!(registry, Small, benchmark_usize_from_other_type_slice_u64);
    register_bench!(registry, Small, benchmark_u8_from_other_type_slice_usize);
    register_bench!(registry, Small, benchmark_u16_from_other_type_slice_usize);
    register_bench!(registry, Small, benchmark_u32_from_other_type_slice_usize);
    register_bench!(registry, Small, benchmark_u64_from_other_type_slice_usize);
    register_bench!(registry, Small, benchmark_u128_from_other_type_slice_usize);
    register_bench!(registry, Small, benchmark_usize_from_other_type_slice_usize);
}

fn demo_from_other_type_slice<T: PrimitiveUnsigned + Rand, U: Display + Named>(
    gm: GenerationMode,
    limit: usize,
) where
    U: FromOtherTypeSlice<T>,
{
    for xs in vecs_of_unsigned::<T>(gm).take(limit) {
        println!(
            "{}::from_other_type_slice({:?}) = {}",
            U::NAME,
            xs,
            U::from_other_type_slice(&xs)
        );
    }
}

fn bench_from_other_type_slice<T: PrimitiveUnsigned + Rand, U: Named>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    U: FromOtherTypeSlice<T>,
{
    m_run_benchmark(
        &format!("{}.from_other_type_slice(&[{}])", U::NAME, T::NAME),
        BenchmarkType::Single,
        vecs_of_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|xs| xs.len()),
        "xs.len()",
        &mut [(
            "malachite",
            &mut (|ref xs| no_out!(U::from_other_type_slice(xs))),
        )],
    );
}

macro_rules! demo_and_bench {
    ($a:ident, $b: ident, $demo_name:ident, $bench_name:ident) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_from_other_type_slice::<$a, $b>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            bench_from_other_type_slice::<$a, $b>(gm, limit, file_name);
        }
    };
}

demo_and_bench!(
    u8,
    u8,
    demo_u8_from_other_type_slice_u8,
    benchmark_u8_from_other_type_slice_u8
);
demo_and_bench!(
    u8,
    u16,
    demo_u16_from_other_type_slice_u8,
    benchmark_u16_from_other_type_slice_u8
);
demo_and_bench!(
    u8,
    u32,
    demo_u32_from_other_type_slice_u8,
    benchmark_u32_from_other_type_slice_u8
);
demo_and_bench!(
    u8,
    u64,
    demo_u64_from_other_type_slice_u8,
    benchmark_u64_from_other_type_slice_u8
);
demo_and_bench!(
    u8,
    u128,
    demo_u128_from_other_type_slice_u8,
    benchmark_u128_from_other_type_slice_u8
);
demo_and_bench!(
    u8,
    usize,
    demo_usize_from_other_type_slice_u8,
    benchmark_usize_from_other_type_slice_u8
);
demo_and_bench!(
    u16,
    u8,
    demo_u8_from_other_type_slice_u16,
    benchmark_u8_from_other_type_slice_u16
);
demo_and_bench!(
    u16,
    u16,
    demo_u16_from_other_type_slice_u16,
    benchmark_u16_from_other_type_slice_u16
);
demo_and_bench!(
    u16,
    u32,
    demo_u32_from_other_type_slice_u16,
    benchmark_u32_from_other_type_slice_u16
);
demo_and_bench!(
    u16,
    u64,
    demo_u64_from_other_type_slice_u16,
    benchmark_u64_from_other_type_slice_u16
);
demo_and_bench!(
    u16,
    u128,
    demo_u128_from_other_type_slice_u16,
    benchmark_u128_from_other_type_slice_u16
);
demo_and_bench!(
    u16,
    usize,
    demo_usize_from_other_type_slice_u16,
    benchmark_usize_from_other_type_slice_u16
);
demo_and_bench!(
    u32,
    u8,
    demo_u8_from_other_type_slice_u32,
    benchmark_u8_from_other_type_slice_u32
);
demo_and_bench!(
    u32,
    u16,
    demo_u16_from_other_type_slice_u32,
    benchmark_u16_from_other_type_slice_u32
);
demo_and_bench!(
    u32,
    u32,
    demo_u32_from_other_type_slice_u32,
    benchmark_u32_from_other_type_slice_u32
);
demo_and_bench!(
    u32,
    u64,
    demo_u64_from_other_type_slice_u32,
    benchmark_u64_from_other_type_slice_u32
);
demo_and_bench!(
    u32,
    u128,
    demo_u128_from_other_type_slice_u32,
    benchmark_u128_from_other_type_slice_u32
);
demo_and_bench!(
    u32,
    usize,
    demo_usize_from_other_type_slice_u32,
    benchmark_usize_from_other_type_slice_u32
);
demo_and_bench!(
    u64,
    u8,
    demo_u8_from_other_type_slice_u64,
    benchmark_u8_from_other_type_slice_u64
);
demo_and_bench!(
    u64,
    u16,
    demo_u16_from_other_type_slice_u64,
    benchmark_u16_from_other_type_slice_u64
);
demo_and_bench!(
    u64,
    u32,
    demo_u32_from_other_type_slice_u64,
    benchmark_u32_from_other_type_slice_u64
);
demo_and_bench!(
    u64,
    u64,
    demo_u64_from_other_type_slice_u64,
    benchmark_u64_from_other_type_slice_u64
);
demo_and_bench!(
    u64,
    u128,
    demo_u128_from_other_type_slice_u64,
    benchmark_u128_from_other_type_slice_u64
);
demo_and_bench!(
    u64,
    usize,
    demo_usize_from_other_type_slice_u64,
    benchmark_usize_from_other_type_slice_u64
);
demo_and_bench!(
    usize,
    u8,
    demo_u8_from_other_type_slice_usize,
    benchmark_u8_from_other_type_slice_usize
);
demo_and_bench!(
    usize,
    u16,
    demo_u16_from_other_type_slice_usize,
    benchmark_u16_from_other_type_slice_usize
);
demo_and_bench!(
    usize,
    u32,
    demo_u32_from_other_type_slice_usize,
    benchmark_u32_from_other_type_slice_usize
);
demo_and_bench!(
    usize,
    u64,
    demo_u64_from_other_type_slice_usize,
    benchmark_u64_from_other_type_slice_usize
);
demo_and_bench!(
    usize,
    u128,
    demo_u128_from_other_type_slice_usize,
    benchmark_u128_from_other_type_slice_usize
);
demo_and_bench!(
    usize,
    usize,
    demo_usize_from_other_type_slice_usize,
    benchmark_usize_from_other_type_slice_usize
);
