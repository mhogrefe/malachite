use std::fmt::Debug;

use malachite_base::named::Named;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, VecFromOtherType};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::unsigneds;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_vec_from_other_type_u8);
    register_demo!(registry, demo_u16_vec_from_other_type_u8);
    register_demo!(registry, demo_u32_vec_from_other_type_u8);
    register_demo!(registry, demo_u64_vec_from_other_type_u8);
    register_demo!(registry, demo_u128_vec_from_other_type_u8);
    register_demo!(registry, demo_usize_vec_from_other_type_u8);
    register_demo!(registry, demo_u8_vec_from_other_type_u16);
    register_demo!(registry, demo_u16_vec_from_other_type_u16);
    register_demo!(registry, demo_u32_vec_from_other_type_u16);
    register_demo!(registry, demo_u64_vec_from_other_type_u16);
    register_demo!(registry, demo_u128_vec_from_other_type_u16);
    register_demo!(registry, demo_usize_vec_from_other_type_u16);
    register_demo!(registry, demo_u8_vec_from_other_type_u32);
    register_demo!(registry, demo_u16_vec_from_other_type_u32);
    register_demo!(registry, demo_u32_vec_from_other_type_u32);
    register_demo!(registry, demo_u64_vec_from_other_type_u32);
    register_demo!(registry, demo_u128_vec_from_other_type_u32);
    register_demo!(registry, demo_usize_vec_from_other_type_u32);
    register_demo!(registry, demo_u8_vec_from_other_type_u64);
    register_demo!(registry, demo_u16_vec_from_other_type_u64);
    register_demo!(registry, demo_u32_vec_from_other_type_u64);
    register_demo!(registry, demo_u64_vec_from_other_type_u64);
    register_demo!(registry, demo_u128_vec_from_other_type_u64);
    register_demo!(registry, demo_usize_vec_from_other_type_u64);
    register_demo!(registry, demo_u8_vec_from_other_type_usize);
    register_demo!(registry, demo_u16_vec_from_other_type_usize);
    register_demo!(registry, demo_u32_vec_from_other_type_usize);
    register_demo!(registry, demo_u64_vec_from_other_type_usize);
    register_demo!(registry, demo_u128_vec_from_other_type_usize);
    register_demo!(registry, demo_usize_vec_from_other_type_usize);
    register_bench!(registry, None, benchmark_u8_vec_from_other_type_u8);
    register_bench!(registry, None, benchmark_u16_vec_from_other_type_u8);
    register_bench!(registry, None, benchmark_u32_vec_from_other_type_u8);
    register_bench!(registry, None, benchmark_u64_vec_from_other_type_u8);
    register_bench!(registry, None, benchmark_u128_vec_from_other_type_u8);
    register_bench!(registry, None, benchmark_usize_vec_from_other_type_u8);
    register_bench!(registry, None, benchmark_u8_vec_from_other_type_u16);
    register_bench!(registry, None, benchmark_u16_vec_from_other_type_u16);
    register_bench!(registry, None, benchmark_u32_vec_from_other_type_u16);
    register_bench!(registry, None, benchmark_u64_vec_from_other_type_u16);
    register_bench!(registry, None, benchmark_u128_vec_from_other_type_u16);
    register_bench!(registry, None, benchmark_usize_vec_from_other_type_u16);
    register_bench!(registry, None, benchmark_u8_vec_from_other_type_u32);
    register_bench!(registry, None, benchmark_u16_vec_from_other_type_u32);
    register_bench!(registry, None, benchmark_u32_vec_from_other_type_u32);
    register_bench!(registry, None, benchmark_u64_vec_from_other_type_u32);
    register_bench!(registry, None, benchmark_u128_vec_from_other_type_u32);
    register_bench!(registry, None, benchmark_usize_vec_from_other_type_u32);
    register_bench!(registry, None, benchmark_u8_vec_from_other_type_u64);
    register_bench!(registry, None, benchmark_u16_vec_from_other_type_u64);
    register_bench!(registry, None, benchmark_u32_vec_from_other_type_u64);
    register_bench!(registry, None, benchmark_u64_vec_from_other_type_u64);
    register_bench!(registry, None, benchmark_u128_vec_from_other_type_u64);
    register_bench!(registry, None, benchmark_usize_vec_from_other_type_u64);
    register_bench!(registry, None, benchmark_u8_vec_from_other_type_usize);
    register_bench!(registry, None, benchmark_u16_vec_from_other_type_usize);
    register_bench!(registry, None, benchmark_u32_vec_from_other_type_usize);
    register_bench!(registry, None, benchmark_u64_vec_from_other_type_usize);
    register_bench!(registry, None, benchmark_u128_vec_from_other_type_usize);
    register_bench!(registry, None, benchmark_usize_vec_from_other_type_usize);
}

fn demo_vec_from_other_type<T: PrimitiveUnsigned + Rand, U: Debug + Named>(
    gm: GenerationMode,
    limit: usize,
) where
    U: VecFromOtherType<T>,
{
    for n in unsigneds::<T>(gm).take(limit) {
        println!(
            "{}::vec_from_other_type({}) = {:?}",
            U::NAME,
            n,
            U::vec_from_other_type(n)
        );
    }
}

fn bench_vec_from_other_type<T: PrimitiveUnsigned + Rand, U: Named>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    U: VecFromOtherType<T>,
{
    run_benchmark(
        &format!("{}.from_other_type({})", U::NAME, T::NAME),
        BenchmarkType::Single,
        unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [("malachite", &mut (|n| no_out!(U::vec_from_other_type(n))))],
    );
}

macro_rules! demo_and_bench {
    ($a:ident, $b: ident, $demo_name:ident, $bench_name:ident) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_vec_from_other_type::<$a, $b>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            bench_vec_from_other_type::<$a, $b>(gm, limit, file_name);
        }
    };
}

demo_and_bench!(
    u8,
    u8,
    demo_u8_vec_from_other_type_u8,
    benchmark_u8_vec_from_other_type_u8
);
demo_and_bench!(
    u8,
    u16,
    demo_u16_vec_from_other_type_u8,
    benchmark_u16_vec_from_other_type_u8
);
demo_and_bench!(
    u8,
    u32,
    demo_u32_vec_from_other_type_u8,
    benchmark_u32_vec_from_other_type_u8
);
demo_and_bench!(
    u8,
    u64,
    demo_u64_vec_from_other_type_u8,
    benchmark_u64_vec_from_other_type_u8
);
demo_and_bench!(
    u8,
    u128,
    demo_u128_vec_from_other_type_u8,
    benchmark_u128_vec_from_other_type_u8
);
demo_and_bench!(
    u8,
    usize,
    demo_usize_vec_from_other_type_u8,
    benchmark_usize_vec_from_other_type_u8
);
demo_and_bench!(
    u16,
    u8,
    demo_u8_vec_from_other_type_u16,
    benchmark_u8_vec_from_other_type_u16
);
demo_and_bench!(
    u16,
    u16,
    demo_u16_vec_from_other_type_u16,
    benchmark_u16_vec_from_other_type_u16
);
demo_and_bench!(
    u16,
    u32,
    demo_u32_vec_from_other_type_u16,
    benchmark_u32_vec_from_other_type_u16
);
demo_and_bench!(
    u16,
    u64,
    demo_u64_vec_from_other_type_u16,
    benchmark_u64_vec_from_other_type_u16
);
demo_and_bench!(
    u16,
    u128,
    demo_u128_vec_from_other_type_u16,
    benchmark_u128_vec_from_other_type_u16
);
demo_and_bench!(
    u16,
    usize,
    demo_usize_vec_from_other_type_u16,
    benchmark_usize_vec_from_other_type_u16
);
demo_and_bench!(
    u32,
    u8,
    demo_u8_vec_from_other_type_u32,
    benchmark_u8_vec_from_other_type_u32
);
demo_and_bench!(
    u32,
    u16,
    demo_u16_vec_from_other_type_u32,
    benchmark_u16_vec_from_other_type_u32
);
demo_and_bench!(
    u32,
    u32,
    demo_u32_vec_from_other_type_u32,
    benchmark_u32_vec_from_other_type_u32
);
demo_and_bench!(
    u32,
    u64,
    demo_u64_vec_from_other_type_u32,
    benchmark_u64_vec_from_other_type_u32
);
demo_and_bench!(
    u32,
    u128,
    demo_u128_vec_from_other_type_u32,
    benchmark_u128_vec_from_other_type_u32
);
demo_and_bench!(
    u32,
    usize,
    demo_usize_vec_from_other_type_u32,
    benchmark_usize_vec_from_other_type_u32
);
demo_and_bench!(
    u64,
    u8,
    demo_u8_vec_from_other_type_u64,
    benchmark_u8_vec_from_other_type_u64
);
demo_and_bench!(
    u64,
    u16,
    demo_u16_vec_from_other_type_u64,
    benchmark_u16_vec_from_other_type_u64
);
demo_and_bench!(
    u64,
    u32,
    demo_u32_vec_from_other_type_u64,
    benchmark_u32_vec_from_other_type_u64
);
demo_and_bench!(
    u64,
    u64,
    demo_u64_vec_from_other_type_u64,
    benchmark_u64_vec_from_other_type_u64
);
demo_and_bench!(
    u64,
    u128,
    demo_u128_vec_from_other_type_u64,
    benchmark_u128_vec_from_other_type_u64
);
demo_and_bench!(
    u64,
    usize,
    demo_usize_vec_from_other_type_u64,
    benchmark_usize_vec_from_other_type_u64
);
demo_and_bench!(
    usize,
    u8,
    demo_u8_vec_from_other_type_usize,
    benchmark_u8_vec_from_other_type_usize
);
demo_and_bench!(
    usize,
    u16,
    demo_u16_vec_from_other_type_usize,
    benchmark_u16_vec_from_other_type_usize
);
demo_and_bench!(
    usize,
    u32,
    demo_u32_vec_from_other_type_usize,
    benchmark_u32_vec_from_other_type_usize
);
demo_and_bench!(
    usize,
    u64,
    demo_u64_vec_from_other_type_usize,
    benchmark_u64_vec_from_other_type_usize
);
demo_and_bench!(
    usize,
    u128,
    demo_u128_vec_from_other_type_usize,
    benchmark_u128_vec_from_other_type_usize
);
demo_and_bench!(
    usize,
    usize,
    demo_usize_vec_from_other_type_usize,
    benchmark_usize_vec_from_other_type_usize
);
