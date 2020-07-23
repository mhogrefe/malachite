use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{pairs_of_signed_and_unsigned, pairs_of_unsigned_and_unsigned};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_rotate_left);
    register_demo!(registry, demo_u16_rotate_left);
    register_demo!(registry, demo_u32_rotate_left);
    register_demo!(registry, demo_u64_rotate_left);
    register_demo!(registry, demo_usize_rotate_left);
    register_demo!(registry, demo_i8_rotate_left);
    register_demo!(registry, demo_i16_rotate_left);
    register_demo!(registry, demo_i32_rotate_left);
    register_demo!(registry, demo_i64_rotate_left);
    register_demo!(registry, demo_isize_rotate_left);

    register_demo!(registry, demo_u8_rotate_right);
    register_demo!(registry, demo_u16_rotate_right);
    register_demo!(registry, demo_u32_rotate_right);
    register_demo!(registry, demo_u64_rotate_right);
    register_demo!(registry, demo_usize_rotate_right);
    register_demo!(registry, demo_i8_rotate_right);
    register_demo!(registry, demo_i16_rotate_right);
    register_demo!(registry, demo_i32_rotate_right);
    register_demo!(registry, demo_i64_rotate_right);
    register_demo!(registry, demo_isize_rotate_right);

    register_bench!(registry, None, benchmark_u8_rotate_left);
    register_bench!(registry, None, benchmark_u16_rotate_left);
    register_bench!(registry, None, benchmark_u32_rotate_left);
    register_bench!(registry, None, benchmark_u64_rotate_left);
    register_bench!(registry, None, benchmark_usize_rotate_left);
    register_bench!(registry, None, benchmark_i8_rotate_left);
    register_bench!(registry, None, benchmark_i16_rotate_left);
    register_bench!(registry, None, benchmark_i32_rotate_left);
    register_bench!(registry, None, benchmark_i64_rotate_left);
    register_bench!(registry, None, benchmark_isize_rotate_left);

    register_bench!(registry, None, benchmark_u8_rotate_right);
    register_bench!(registry, None, benchmark_u16_rotate_right);
    register_bench!(registry, None, benchmark_u32_rotate_right);
    register_bench!(registry, None, benchmark_u64_rotate_right);
    register_bench!(registry, None, benchmark_usize_rotate_right);
    register_bench!(registry, None, benchmark_i8_rotate_right);
    register_bench!(registry, None, benchmark_i16_rotate_right);
    register_bench!(registry, None, benchmark_i32_rotate_right);
    register_bench!(registry, None, benchmark_i64_rotate_right);
    register_bench!(registry, None, benchmark_isize_rotate_right);
}

fn demo_unsigned_rotate_left<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (n, index) in pairs_of_unsigned_and_unsigned::<T, u64>(gm).take(limit) {
        println!("{}.rotate_left({}) = {}", n, index, n.rotate_left(index));
    }
}

fn demo_signed_rotate_left<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (n, index) in pairs_of_signed_and_unsigned::<T, u64>(gm).take(limit) {
        println!("({}).rotate_left({}) = {}", n, index, n.rotate_left(index));
    }
}

fn demo_unsigned_rotate_right<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (n, index) in pairs_of_unsigned_and_unsigned::<T, u64>(gm).take(limit) {
        println!("{}.rotate_right({}) = {}", n, index, n.rotate_right(index));
    }
}

fn demo_signed_rotate_right<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (n, index) in pairs_of_signed_and_unsigned::<T, u64>(gm).take(limit) {
        println!(
            "({}).rotate_right({}) = {}",
            n,
            index,
            n.rotate_right(index)
        );
    }
}

fn benchmark_unsigned_rotate_left<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.rotate_left(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_unsigned::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "index",
        &mut [(
            "malachite",
            &mut (|(n, index)| no_out!(n.rotate_left(index))),
        )],
    );
}

fn benchmark_signed_rotate_left<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark(
        &format!("{}.rotate_left(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_signed_and_unsigned::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "index",
        &mut [(
            "malachite",
            &mut (|(n, index)| no_out!(n.rotate_left(index))),
        )],
    );
}

fn benchmark_unsigned_rotate_right<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.rotate_right(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_unsigned::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "index",
        &mut [(
            "malachite",
            &mut (|(n, index)| no_out!(n.rotate_right(index))),
        )],
    );
}

fn benchmark_signed_rotate_right<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark(
        &format!("{}.rotate_right(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_signed_and_unsigned::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "index",
        &mut [(
            "malachite",
            &mut (|(n, index)| no_out!(n.rotate_right(index))),
        )],
    );
}

macro_rules! unsigned {
    (
        $t:ident,
        $rotate_left_demo_name:ident,
        $rotate_right_demo_name:ident,
        $rotate_left_bench_name:ident,
        $rotate_right_bench_name:ident
    ) => {
        fn $rotate_left_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_rotate_left::<$t>(gm, limit);
        }

        fn $rotate_right_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_rotate_right::<$t>(gm, limit);
        }

        fn $rotate_left_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_rotate_left::<$t>(gm, limit, file_name);
        }

        fn $rotate_right_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_rotate_right::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed {
    (
        $t:ident,
        $rotate_left_demo_name:ident,
        $rotate_right_demo_name:ident,
        $rotate_left_bench_name:ident,
        $rotate_right_bench_name:ident
    ) => {
        fn $rotate_left_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_rotate_left::<$t>(gm, limit);
        }

        fn $rotate_right_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_rotate_right::<$t>(gm, limit);
        }

        fn $rotate_left_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_rotate_left::<$t>(gm, limit, file_name);
        }

        fn $rotate_right_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_rotate_right::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    demo_u8_rotate_left,
    demo_u8_rotate_right,
    benchmark_u8_rotate_left,
    benchmark_u8_rotate_right
);
unsigned!(
    u16,
    demo_u16_rotate_left,
    demo_u16_rotate_right,
    benchmark_u16_rotate_left,
    benchmark_u16_rotate_right
);
unsigned!(
    u32,
    demo_u32_rotate_left,
    demo_u32_rotate_right,
    benchmark_u32_rotate_left,
    benchmark_u32_rotate_right
);
unsigned!(
    u64,
    demo_u64_rotate_left,
    demo_u64_rotate_right,
    benchmark_u64_rotate_left,
    benchmark_u64_rotate_right
);
unsigned!(
    usize,
    demo_usize_rotate_left,
    demo_usize_rotate_right,
    benchmark_usize_rotate_left,
    benchmark_usize_rotate_right
);
signed!(
    i8,
    demo_i8_rotate_left,
    demo_i8_rotate_right,
    benchmark_i8_rotate_left,
    benchmark_i8_rotate_right
);
signed!(
    i16,
    demo_i16_rotate_left,
    demo_i16_rotate_right,
    benchmark_i16_rotate_left,
    benchmark_i16_rotate_right
);
signed!(
    i32,
    demo_i32_rotate_left,
    demo_i32_rotate_right,
    benchmark_i32_rotate_left,
    benchmark_i32_rotate_right
);
signed!(
    i64,
    demo_i64_rotate_left,
    demo_i64_rotate_right,
    benchmark_i64_rotate_left,
    benchmark_i64_rotate_right
);
signed!(
    isize,
    demo_isize_rotate_left,
    demo_isize_rotate_right,
    benchmark_isize_rotate_left,
    benchmark_isize_rotate_right
);
