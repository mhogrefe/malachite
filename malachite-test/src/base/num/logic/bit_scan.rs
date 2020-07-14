use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use rand::Rand;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType,
};
use malachite_test::inputs::base::{
    pairs_of_signed_and_small_unsigned, pairs_of_unsigned_and_small_unsigned,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_index_of_next_false_bit);
    register_demo!(registry, demo_u16_index_of_next_false_bit);
    register_demo!(registry, demo_u32_index_of_next_false_bit);
    register_demo!(registry, demo_u64_index_of_next_false_bit);
    register_demo!(registry, demo_usize_index_of_next_false_bit);
    register_demo!(registry, demo_i8_index_of_next_false_bit);
    register_demo!(registry, demo_i16_index_of_next_false_bit);
    register_demo!(registry, demo_i32_index_of_next_false_bit);
    register_demo!(registry, demo_i64_index_of_next_false_bit);
    register_demo!(registry, demo_isize_index_of_next_false_bit);

    register_demo!(registry, demo_u8_index_of_next_true_bit);
    register_demo!(registry, demo_u16_index_of_next_true_bit);
    register_demo!(registry, demo_u32_index_of_next_true_bit);
    register_demo!(registry, demo_u64_index_of_next_true_bit);
    register_demo!(registry, demo_usize_index_of_next_true_bit);
    register_demo!(registry, demo_i8_index_of_next_true_bit);
    register_demo!(registry, demo_i16_index_of_next_true_bit);
    register_demo!(registry, demo_i32_index_of_next_true_bit);
    register_demo!(registry, demo_i64_index_of_next_true_bit);
    register_demo!(registry, demo_isize_index_of_next_true_bit);

    register_bench!(registry, None, benchmark_u8_index_of_next_false_bit);
    register_bench!(registry, None, benchmark_u16_index_of_next_false_bit);
    register_bench!(registry, None, benchmark_u32_index_of_next_false_bit);
    register_bench!(registry, None, benchmark_u64_index_of_next_false_bit);
    register_bench!(registry, None, benchmark_usize_index_of_next_false_bit);
    register_bench!(registry, None, benchmark_i8_index_of_next_false_bit);
    register_bench!(registry, None, benchmark_i16_index_of_next_false_bit);
    register_bench!(registry, None, benchmark_i32_index_of_next_false_bit);
    register_bench!(registry, None, benchmark_i64_index_of_next_false_bit);
    register_bench!(registry, None, benchmark_isize_index_of_next_false_bit);

    register_bench!(registry, None, benchmark_u8_index_of_next_true_bit);
    register_bench!(registry, None, benchmark_u16_index_of_next_true_bit);
    register_bench!(registry, None, benchmark_u32_index_of_next_true_bit);
    register_bench!(registry, None, benchmark_u64_index_of_next_true_bit);
    register_bench!(registry, None, benchmark_usize_index_of_next_true_bit);
    register_bench!(registry, None, benchmark_i8_index_of_next_true_bit);
    register_bench!(registry, None, benchmark_i16_index_of_next_true_bit);
    register_bench!(registry, None, benchmark_i32_index_of_next_true_bit);
    register_bench!(registry, None, benchmark_i64_index_of_next_true_bit);
    register_bench!(registry, None, benchmark_isize_index_of_next_true_bit);
}

fn demo_unsigned_index_of_next_false_bit<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) {
    for (n, start) in pairs_of_unsigned_and_small_unsigned::<T, u64>(gm).take(limit) {
        println!(
            "{}.index_of_next_false_bit({}) = {:?}",
            n,
            start,
            n.index_of_next_false_bit(start)
        );
    }
}

fn demo_unsigned_index_of_next_true_bit<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) {
    for (n, start) in pairs_of_unsigned_and_small_unsigned::<T, u64>(gm).take(limit) {
        println!(
            "{}.index_of_next_true_bit({}) = {:?}",
            n,
            start,
            n.index_of_next_true_bit(start)
        );
    }
}

fn demo_signed_index_of_next_false_bit<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (n, start) in pairs_of_signed_and_small_unsigned::<T, u64>(gm).take(limit) {
        println!(
            "{}.index_of_next_false_bit({}) = {:?}",
            n,
            start,
            n.index_of_next_false_bit(start)
        );
    }
}

fn demo_signed_index_of_next_true_bit<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (n, start) in pairs_of_signed_and_small_unsigned::<T, u64>(gm).take(limit) {
        println!(
            "{}.index_of_next_true_bit({}) = {:?}",
            n,
            start,
            n.index_of_next_true_bit(start)
        );
    }
}

fn benchmark_unsigned_index_of_next_false_bit<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.index_of_next_false_bit(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_small_unsigned::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "index",
        &mut [(
            "malachite",
            &mut (|(n, index)| no_out!(n.index_of_next_false_bit(index))),
        )],
    );
}

fn benchmark_unsigned_index_of_next_true_bit<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.index_of_next_true_bit(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_small_unsigned::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "index",
        &mut [(
            "malachite",
            &mut (|(n, index)| no_out!(n.index_of_next_true_bit(index))),
        )],
    );
}

fn benchmark_signed_index_of_next_false_bit<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.index_of_next_false_bit(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_signed_and_small_unsigned::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "index",
        &mut [(
            "malachite",
            &mut (|(n, index)| no_out!(n.index_of_next_false_bit(index))),
        )],
    );
}

fn benchmark_signed_index_of_next_true_bit<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.index_of_next_true_bit(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_signed_and_small_unsigned::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "index",
        &mut [(
            "malachite",
            &mut (|(n, index)| no_out!(n.index_of_next_true_bit(index))),
        )],
    );
}

macro_rules! unsigned {
    (
        $t:ident,
        $index_of_next_false_bit_demo_name:ident,
        $index_of_next_true_bit_demo_name:ident,
        $index_of_next_false_bit_bench_name:ident,
        $index_of_next_true_bit_bench_name:ident
    ) => {
        fn $index_of_next_false_bit_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_index_of_next_false_bit::<$t>(gm, limit);
        }

        fn $index_of_next_true_bit_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_index_of_next_true_bit::<$t>(gm, limit);
        }

        fn $index_of_next_false_bit_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_index_of_next_false_bit::<$t>(gm, limit, file_name);
        }

        fn $index_of_next_true_bit_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_index_of_next_true_bit::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed {
    (
        $t:ident,
        $index_of_next_false_bit_demo_name:ident,
        $index_of_next_true_bit_demo_name:ident,
        $index_of_next_false_bit_bench_name:ident,
        $index_of_next_true_bit_bench_name:ident
    ) => {
        fn $index_of_next_false_bit_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_index_of_next_false_bit::<$t>(gm, limit);
        }

        fn $index_of_next_true_bit_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_index_of_next_true_bit::<$t>(gm, limit);
        }

        fn $index_of_next_false_bit_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_index_of_next_false_bit::<$t>(gm, limit, file_name);
        }

        fn $index_of_next_true_bit_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_index_of_next_true_bit::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    demo_u8_index_of_next_false_bit,
    demo_u8_index_of_next_true_bit,
    benchmark_u8_index_of_next_false_bit,
    benchmark_u8_index_of_next_true_bit
);
unsigned!(
    u16,
    demo_u16_index_of_next_false_bit,
    demo_u16_index_of_next_true_bit,
    benchmark_u16_index_of_next_false_bit,
    benchmark_u16_index_of_next_true_bit
);
unsigned!(
    u32,
    demo_u32_index_of_next_false_bit,
    demo_u32_index_of_next_true_bit,
    benchmark_u32_index_of_next_false_bit,
    benchmark_u32_index_of_next_true_bit
);
unsigned!(
    u64,
    demo_u64_index_of_next_false_bit,
    demo_u64_index_of_next_true_bit,
    benchmark_u64_index_of_next_false_bit,
    benchmark_u64_index_of_next_true_bit
);
unsigned!(
    usize,
    demo_usize_index_of_next_false_bit,
    demo_usize_index_of_next_true_bit,
    benchmark_usize_index_of_next_false_bit,
    benchmark_usize_index_of_next_true_bit
);
signed!(
    i8,
    demo_i8_index_of_next_false_bit,
    demo_i8_index_of_next_true_bit,
    benchmark_i8_index_of_next_false_bit,
    benchmark_i8_index_of_next_true_bit
);
signed!(
    i16,
    demo_i16_index_of_next_false_bit,
    demo_i16_index_of_next_true_bit,
    benchmark_i16_index_of_next_false_bit,
    benchmark_i16_index_of_next_true_bit
);
signed!(
    i32,
    demo_i32_index_of_next_false_bit,
    demo_i32_index_of_next_true_bit,
    benchmark_i32_index_of_next_false_bit,
    benchmark_i32_index_of_next_true_bit
);
signed!(
    i64,
    demo_i64_index_of_next_false_bit,
    demo_i64_index_of_next_true_bit,
    benchmark_i64_index_of_next_false_bit,
    benchmark_i64_index_of_next_true_bit
);
signed!(
    isize,
    demo_isize_index_of_next_false_bit,
    demo_isize_index_of_next_true_bit,
    benchmark_isize_index_of_next_false_bit,
    benchmark_isize_index_of_next_true_bit
);
