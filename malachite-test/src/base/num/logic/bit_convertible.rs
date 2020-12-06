use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_base_test_util::num::logic::bit_convertible::{
    from_bits_asc_alt, from_bits_asc_signed_naive, from_bits_asc_unsigned_naive, from_bits_desc_alt,
};
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    vecs_of_bool_var_2, vecs_of_bool_var_3, vecs_of_bool_var_4, vecs_of_bool_var_5,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_from_bits_asc);
    register_demo!(registry, demo_u16_from_bits_asc);
    register_demo!(registry, demo_u32_from_bits_asc);
    register_demo!(registry, demo_u64_from_bits_asc);
    register_demo!(registry, demo_usize_from_bits_asc);
    register_demo!(registry, demo_i8_from_bits_asc);
    register_demo!(registry, demo_i16_from_bits_asc);
    register_demo!(registry, demo_i32_from_bits_asc);
    register_demo!(registry, demo_i64_from_bits_asc);
    register_demo!(registry, demo_isize_from_bits_asc);

    register_demo!(registry, demo_u8_from_bits_desc);
    register_demo!(registry, demo_u16_from_bits_desc);
    register_demo!(registry, demo_u32_from_bits_desc);
    register_demo!(registry, demo_u64_from_bits_desc);
    register_demo!(registry, demo_usize_from_bits_desc);
    register_demo!(registry, demo_i8_from_bits_desc);
    register_demo!(registry, demo_i16_from_bits_desc);
    register_demo!(registry, demo_i32_from_bits_desc);
    register_demo!(registry, demo_i64_from_bits_desc);
    register_demo!(registry, demo_isize_from_bits_desc);

    register_bench!(registry, None, benchmark_u8_from_bits_asc_algorithms);
    register_bench!(registry, None, benchmark_u16_from_bits_asc_algorithms);
    register_bench!(registry, None, benchmark_u32_from_bits_asc_algorithms);
    register_bench!(registry, None, benchmark_u64_from_bits_asc_algorithms);
    register_bench!(registry, None, benchmark_usize_from_bits_asc_algorithms);
    register_bench!(registry, None, benchmark_i8_from_bits_asc_algorithms);
    register_bench!(registry, None, benchmark_i16_from_bits_asc_algorithms);
    register_bench!(registry, None, benchmark_i32_from_bits_asc_algorithms);
    register_bench!(registry, None, benchmark_i64_from_bits_asc_algorithms);
    register_bench!(registry, None, benchmark_isize_from_bits_asc_algorithms);

    register_bench!(registry, None, benchmark_u8_from_bits_desc_algorithms);
    register_bench!(registry, None, benchmark_u16_from_bits_desc_algorithms);
    register_bench!(registry, None, benchmark_u32_from_bits_desc_algorithms);
    register_bench!(registry, None, benchmark_u64_from_bits_desc_algorithms);
    register_bench!(registry, None, benchmark_usize_from_bits_desc_algorithms);
    register_bench!(registry, None, benchmark_i8_from_bits_desc_algorithms);
    register_bench!(registry, None, benchmark_i16_from_bits_desc_algorithms);
    register_bench!(registry, None, benchmark_i32_from_bits_desc_algorithms);
    register_bench!(registry, None, benchmark_i64_from_bits_desc_algorithms);
    register_bench!(registry, None, benchmark_isize_from_bits_desc_algorithms);
}

fn demo_unsigned_from_bits_asc<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for bits in vecs_of_bool_var_2::<T>(gm).take(limit) {
        println!(
            "{}::from_bits_asc({:?}) = {}",
            T::NAME,
            bits,
            T::from_bits_asc(bits.iter().cloned())
        );
    }
}

fn demo_signed_from_bits_asc<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for bits in vecs_of_bool_var_3::<T>(gm).take(limit) {
        println!(
            "{}::from_bits_asc({:?}) = {}",
            T::NAME,
            bits,
            T::from_bits_asc(bits.iter().cloned())
        );
    }
}

fn demo_unsigned_from_bits_desc<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for bits in vecs_of_bool_var_4::<T>(gm).take(limit) {
        println!(
            "{}::from_bits_desc({:?}) = {}",
            T::NAME,
            bits,
            T::from_bits_desc(bits.iter().cloned())
        );
    }
}

fn demo_signed_from_bits_desc<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for bits in vecs_of_bool_var_5::<T>(gm).take(limit) {
        println!(
            "{}::from_bits_desc({:?}) = {}",
            T::NAME,
            bits,
            T::from_bits_desc(bits.iter().cloned())
        );
    }
}

fn benchmark_unsigned_from_bits_asc_algorithms<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!("{}::from_bits_asc<I: Iterator<Item=bool>>(I)", T::NAME),
        BenchmarkType::Algorithms,
        vecs_of_bool_var_2::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|bits| bits.len()),
        "bits.len()",
        &mut [
            (
                "default",
                &mut (|ref bits| no_out!(T::from_bits_asc(bits.iter().cloned()))),
            ),
            (
                "alt",
                &mut (|ref bits| no_out!(from_bits_asc_alt::<T, _>(bits.iter().cloned()))),
            ),
            (
                "naive",
                &mut (|ref bits| {
                    no_out!(from_bits_asc_unsigned_naive::<T, _>(bits.iter().cloned()))
                }),
            ),
        ],
    );
}

fn benchmark_signed_from_bits_asc_algorithms<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark_old(
        &format!("{}::from_bits_asc<I: Iterator<Item=bool>>(I)", T::NAME),
        BenchmarkType::Algorithms,
        vecs_of_bool_var_3::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|bits| bits.len()),
        "bits.len()",
        &mut [
            (
                "default",
                &mut (|ref bits| no_out!(T::from_bits_asc(bits.iter().cloned()))),
            ),
            (
                "alt",
                &mut (|ref bits| no_out!(from_bits_asc_alt::<T, _>(bits.iter().cloned()))),
            ),
            (
                "naive",
                &mut (|ref bits| no_out!(from_bits_asc_signed_naive::<T, _>(bits.iter().cloned()))),
            ),
        ],
    );
}

fn benchmark_unsigned_from_bits_desc_algorithms<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!("{}::from_bits_desc<I: Iterator<Item=bool>>(I)", T::NAME),
        BenchmarkType::Algorithms,
        vecs_of_bool_var_4::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|bits| bits.len()),
        "bits.len()",
        &mut [
            (
                "Malachite",
                &mut (|ref bits| no_out!(T::from_bits_desc(bits.iter().cloned()))),
            ),
            (
                "alt",
                &mut (|ref bits| no_out!(from_bits_desc_alt::<T, _>(bits.iter().cloned()))),
            ),
        ],
    );
}

fn benchmark_signed_from_bits_desc_algorithms<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark_old(
        &format!("{}::from_bits_desc<I: Iterator<Item=bool>>(I)", T::NAME),
        BenchmarkType::Algorithms,
        vecs_of_bool_var_5::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|bits| bits.len()),
        "bits.len()",
        &mut [
            (
                "Malachite",
                &mut (|ref bits| no_out!(T::from_bits_desc(bits.iter().cloned()))),
            ),
            (
                "alt",
                &mut (|ref bits| no_out!(from_bits_desc_alt::<T, _>(bits.iter().cloned()))),
            ),
        ],
    );
}

macro_rules! unsigned {
    (
        $t:ident,
        $from_bits_asc_demo_name:ident,
        $from_bits_asc_bench_name:ident,
        $from_bits_desc_demo_name:ident,
        $from_bits_desc_bench_name:ident,
    ) => {
        fn $from_bits_asc_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_from_bits_asc::<$t>(gm, limit);
        }

        fn $from_bits_asc_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_from_bits_asc_algorithms::<$t>(gm, limit, file_name);
        }

        fn $from_bits_desc_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_from_bits_desc::<$t>(gm, limit);
        }

        fn $from_bits_desc_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_from_bits_desc_algorithms::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed {
    (
        $t:ident,
        $from_bits_asc_demo_name:ident,
        $from_bits_asc_bench_name:ident,
        $from_bits_desc_demo_name:ident,
        $from_bits_desc_bench_name:ident,
    ) => {
        fn $from_bits_asc_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_from_bits_asc::<$t>(gm, limit);
        }

        fn $from_bits_asc_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_from_bits_asc_algorithms::<$t>(gm, limit, file_name);
        }

        fn $from_bits_desc_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_from_bits_desc::<$t>(gm, limit);
        }

        fn $from_bits_desc_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_from_bits_desc_algorithms::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    demo_u8_from_bits_asc,
    benchmark_u8_from_bits_asc_algorithms,
    demo_u8_from_bits_desc,
    benchmark_u8_from_bits_desc_algorithms,
);
unsigned!(
    u16,
    demo_u16_from_bits_asc,
    benchmark_u16_from_bits_asc_algorithms,
    demo_u16_from_bits_desc,
    benchmark_u16_from_bits_desc_algorithms,
);
unsigned!(
    u32,
    demo_u32_from_bits_asc,
    benchmark_u32_from_bits_asc_algorithms,
    demo_u32_from_bits_desc,
    benchmark_u32_from_bits_desc_algorithms,
);
unsigned!(
    u64,
    demo_u64_from_bits_asc,
    benchmark_u64_from_bits_asc_algorithms,
    demo_u64_from_bits_desc,
    benchmark_u64_from_bits_desc_algorithms,
);
unsigned!(
    usize,
    demo_usize_from_bits_asc,
    benchmark_usize_from_bits_asc_algorithms,
    demo_usize_from_bits_desc,
    benchmark_usize_from_bits_desc_algorithms,
);
signed!(
    i8,
    demo_i8_from_bits_asc,
    benchmark_i8_from_bits_asc_algorithms,
    demo_i8_from_bits_desc,
    benchmark_i8_from_bits_desc_algorithms,
);
signed!(
    i16,
    demo_i16_from_bits_asc,
    benchmark_i16_from_bits_asc_algorithms,
    demo_i16_from_bits_desc,
    benchmark_i16_from_bits_desc_algorithms,
);
signed!(
    i32,
    demo_i32_from_bits_asc,
    benchmark_i32_from_bits_asc_algorithms,
    demo_i32_from_bits_desc,
    benchmark_i32_from_bits_desc_algorithms,
);
signed!(
    i64,
    demo_i64_from_bits_asc,
    benchmark_i64_from_bits_asc_algorithms,
    demo_i64_from_bits_desc,
    benchmark_i64_from_bits_desc_algorithms,
);
signed!(
    isize,
    demo_isize_from_bits_asc,
    benchmark_isize_from_bits_asc_algorithms,
    demo_isize_from_bits_desc,
    benchmark_isize_from_bits_desc_algorithms,
);
