use std::fmt::Debug;

use malachite_base::named::Named;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::VecFromOtherTypeSlice;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::vecs_of_unsigned;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_vec_from_other_type_slice_u8);
    register_demo!(registry, demo_u16_vec_from_other_type_slice_u8);
    register_demo!(registry, demo_u32_vec_from_other_type_slice_u8);
    register_demo!(registry, demo_u64_vec_from_other_type_slice_u8);
    register_demo!(registry, demo_u128_vec_from_other_type_slice_u8);
    register_demo!(registry, demo_usize_vec_from_other_type_slice_u8);
    register_demo!(registry, demo_u8_vec_from_other_type_slice_u16);
    register_demo!(registry, demo_u16_vec_from_other_type_slice_u16);
    register_demo!(registry, demo_u32_vec_from_other_type_slice_u16);
    register_demo!(registry, demo_u64_vec_from_other_type_slice_u16);
    register_demo!(registry, demo_u128_vec_from_other_type_slice_u16);
    register_demo!(registry, demo_usize_vec_from_other_type_slice_u16);
    register_demo!(registry, demo_u8_vec_from_other_type_slice_u32);
    register_demo!(registry, demo_u16_vec_from_other_type_slice_u32);
    register_demo!(registry, demo_u32_vec_from_other_type_slice_u32);
    register_demo!(registry, demo_u64_vec_from_other_type_slice_u32);
    register_demo!(registry, demo_u128_vec_from_other_type_slice_u32);
    register_demo!(registry, demo_usize_vec_from_other_type_slice_u32);
    register_demo!(registry, demo_u8_vec_from_other_type_slice_u64);
    register_demo!(registry, demo_u16_vec_from_other_type_slice_u64);
    register_demo!(registry, demo_u32_vec_from_other_type_slice_u64);
    register_demo!(registry, demo_u64_vec_from_other_type_slice_u64);
    register_demo!(registry, demo_u128_vec_from_other_type_slice_u64);
    register_demo!(registry, demo_usize_vec_from_other_type_slice_u64);
    register_demo!(registry, demo_u8_vec_from_other_type_slice_usize);
    register_demo!(registry, demo_u16_vec_from_other_type_slice_usize);
    register_demo!(registry, demo_u32_vec_from_other_type_slice_usize);
    register_demo!(registry, demo_u64_vec_from_other_type_slice_usize);
    register_demo!(registry, demo_u128_vec_from_other_type_slice_usize);
    register_demo!(registry, demo_usize_vec_from_other_type_slice_usize);
    register_bench!(registry, Small, benchmark_u8_vec_from_other_type_slice_u8);
    register_bench!(registry, Small, benchmark_u16_vec_from_other_type_slice_u8);
    register_bench!(registry, Small, benchmark_u32_vec_from_other_type_slice_u8);
    register_bench!(registry, Small, benchmark_u64_vec_from_other_type_slice_u8);
    register_bench!(registry, Small, benchmark_u128_vec_from_other_type_slice_u8);
    register_bench!(
        registry,
        Small,
        benchmark_usize_vec_from_other_type_slice_u8
    );
    register_bench!(registry, Small, benchmark_u8_vec_from_other_type_slice_u16);
    register_bench!(registry, Small, benchmark_u16_vec_from_other_type_slice_u16);
    register_bench!(registry, Small, benchmark_u32_vec_from_other_type_slice_u16);
    register_bench!(registry, Small, benchmark_u64_vec_from_other_type_slice_u16);
    register_bench!(
        registry,
        Small,
        benchmark_u128_vec_from_other_type_slice_u16
    );
    register_bench!(
        registry,
        Small,
        benchmark_usize_vec_from_other_type_slice_u16
    );
    register_bench!(registry, Small, benchmark_u8_vec_from_other_type_slice_u32);
    register_bench!(registry, Small, benchmark_u16_vec_from_other_type_slice_u32);
    register_bench!(registry, Small, benchmark_u32_vec_from_other_type_slice_u32);
    register_bench!(registry, Small, benchmark_u64_vec_from_other_type_slice_u32);
    register_bench!(
        registry,
        Small,
        benchmark_u128_vec_from_other_type_slice_u32
    );
    register_bench!(
        registry,
        Small,
        benchmark_usize_vec_from_other_type_slice_u32
    );
    register_bench!(registry, Small, benchmark_u8_vec_from_other_type_slice_u64);
    register_bench!(registry, Small, benchmark_u16_vec_from_other_type_slice_u64);
    register_bench!(registry, Small, benchmark_u32_vec_from_other_type_slice_u64);
    register_bench!(registry, Small, benchmark_u64_vec_from_other_type_slice_u64);
    register_bench!(
        registry,
        Small,
        benchmark_u128_vec_from_other_type_slice_u64
    );
    register_bench!(
        registry,
        Small,
        benchmark_usize_vec_from_other_type_slice_u64
    );
    register_bench!(
        registry,
        Small,
        benchmark_u8_vec_from_other_type_slice_usize
    );
    register_bench!(
        registry,
        Small,
        benchmark_u16_vec_from_other_type_slice_usize
    );
    register_bench!(
        registry,
        Small,
        benchmark_u32_vec_from_other_type_slice_usize
    );
    register_bench!(
        registry,
        Small,
        benchmark_u64_vec_from_other_type_slice_usize
    );
    register_bench!(
        registry,
        Small,
        benchmark_u128_vec_from_other_type_slice_usize
    );
    register_bench!(
        registry,
        Small,
        benchmark_usize_vec_from_other_type_slice_usize
    );
}

fn demo_vec_from_other_type_slice<T: PrimitiveUnsigned + Rand, U: Debug + Named>(
    gm: GenerationMode,
    limit: usize,
) where
    U: VecFromOtherTypeSlice<T>,
{
    for xs in vecs_of_unsigned::<T>(gm).take(limit) {
        println!(
            "{}::vec_from_other_type_slice({:?}) = {:?}",
            U::NAME,
            xs,
            U::vec_from_other_type_slice(&xs)
        );
    }
}

fn bench_vec_from_other_type_slice<T: PrimitiveUnsigned + Rand, U: Named>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    U: VecFromOtherTypeSlice<T>,
{
    run_benchmark_old(
        &format!("{}.from_other_type_slice(&[{}])", U::NAME, T::NAME),
        BenchmarkType::Single,
        vecs_of_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|xs| xs.len()),
        "xs.len()",
        &mut [(
            "Malachite",
            &mut (|ref xs| no_out!(U::vec_from_other_type_slice(xs))),
        )],
    );
}

macro_rules! demo_and_bench {
    ($a:ident, $b: ident, $demo_name:ident, $bench_name:ident) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_vec_from_other_type_slice::<$a, $b>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            bench_vec_from_other_type_slice::<$a, $b>(gm, limit, file_name);
        }
    };
}

demo_and_bench!(
    u8,
    u8,
    demo_u8_vec_from_other_type_slice_u8,
    benchmark_u8_vec_from_other_type_slice_u8
);
demo_and_bench!(
    u8,
    u16,
    demo_u16_vec_from_other_type_slice_u8,
    benchmark_u16_vec_from_other_type_slice_u8
);
demo_and_bench!(
    u8,
    u32,
    demo_u32_vec_from_other_type_slice_u8,
    benchmark_u32_vec_from_other_type_slice_u8
);
demo_and_bench!(
    u8,
    u64,
    demo_u64_vec_from_other_type_slice_u8,
    benchmark_u64_vec_from_other_type_slice_u8
);
demo_and_bench!(
    u8,
    u128,
    demo_u128_vec_from_other_type_slice_u8,
    benchmark_u128_vec_from_other_type_slice_u8
);
demo_and_bench!(
    u8,
    usize,
    demo_usize_vec_from_other_type_slice_u8,
    benchmark_usize_vec_from_other_type_slice_u8
);
demo_and_bench!(
    u16,
    u8,
    demo_u8_vec_from_other_type_slice_u16,
    benchmark_u8_vec_from_other_type_slice_u16
);
demo_and_bench!(
    u16,
    u16,
    demo_u16_vec_from_other_type_slice_u16,
    benchmark_u16_vec_from_other_type_slice_u16
);
demo_and_bench!(
    u16,
    u32,
    demo_u32_vec_from_other_type_slice_u16,
    benchmark_u32_vec_from_other_type_slice_u16
);
demo_and_bench!(
    u16,
    u64,
    demo_u64_vec_from_other_type_slice_u16,
    benchmark_u64_vec_from_other_type_slice_u16
);
demo_and_bench!(
    u16,
    u128,
    demo_u128_vec_from_other_type_slice_u16,
    benchmark_u128_vec_from_other_type_slice_u16
);
demo_and_bench!(
    u16,
    usize,
    demo_usize_vec_from_other_type_slice_u16,
    benchmark_usize_vec_from_other_type_slice_u16
);
demo_and_bench!(
    u32,
    u8,
    demo_u8_vec_from_other_type_slice_u32,
    benchmark_u8_vec_from_other_type_slice_u32
);
demo_and_bench!(
    u32,
    u16,
    demo_u16_vec_from_other_type_slice_u32,
    benchmark_u16_vec_from_other_type_slice_u32
);
demo_and_bench!(
    u32,
    u32,
    demo_u32_vec_from_other_type_slice_u32,
    benchmark_u32_vec_from_other_type_slice_u32
);
demo_and_bench!(
    u32,
    u64,
    demo_u64_vec_from_other_type_slice_u32,
    benchmark_u64_vec_from_other_type_slice_u32
);
demo_and_bench!(
    u32,
    u128,
    demo_u128_vec_from_other_type_slice_u32,
    benchmark_u128_vec_from_other_type_slice_u32
);
demo_and_bench!(
    u32,
    usize,
    demo_usize_vec_from_other_type_slice_u32,
    benchmark_usize_vec_from_other_type_slice_u32
);
demo_and_bench!(
    u64,
    u8,
    demo_u8_vec_from_other_type_slice_u64,
    benchmark_u8_vec_from_other_type_slice_u64
);
demo_and_bench!(
    u64,
    u16,
    demo_u16_vec_from_other_type_slice_u64,
    benchmark_u16_vec_from_other_type_slice_u64
);
demo_and_bench!(
    u64,
    u32,
    demo_u32_vec_from_other_type_slice_u64,
    benchmark_u32_vec_from_other_type_slice_u64
);
demo_and_bench!(
    u64,
    u64,
    demo_u64_vec_from_other_type_slice_u64,
    benchmark_u64_vec_from_other_type_slice_u64
);
demo_and_bench!(
    u64,
    u128,
    demo_u128_vec_from_other_type_slice_u64,
    benchmark_u128_vec_from_other_type_slice_u64
);
demo_and_bench!(
    u64,
    usize,
    demo_usize_vec_from_other_type_slice_u64,
    benchmark_usize_vec_from_other_type_slice_u64
);
demo_and_bench!(
    usize,
    u8,
    demo_u8_vec_from_other_type_slice_usize,
    benchmark_u8_vec_from_other_type_slice_usize
);
demo_and_bench!(
    usize,
    u16,
    demo_u16_vec_from_other_type_slice_usize,
    benchmark_u16_vec_from_other_type_slice_usize
);
demo_and_bench!(
    usize,
    u32,
    demo_u32_vec_from_other_type_slice_usize,
    benchmark_u32_vec_from_other_type_slice_usize
);
demo_and_bench!(
    usize,
    u64,
    demo_u64_vec_from_other_type_slice_usize,
    benchmark_u64_vec_from_other_type_slice_usize
);
demo_and_bench!(
    usize,
    u128,
    demo_u128_vec_from_other_type_slice_usize,
    benchmark_u128_vec_from_other_type_slice_usize
);
demo_and_bench!(
    usize,
    usize,
    demo_usize_vec_from_other_type_slice_usize,
    benchmark_usize_vec_from_other_type_slice_usize
);
