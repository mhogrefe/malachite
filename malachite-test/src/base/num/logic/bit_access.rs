use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    pairs_of_signed_and_u64_width_range, pairs_of_signed_and_u64_width_range_var_1,
    pairs_of_signed_and_u64_width_range_var_2, pairs_of_unsigned_and_small_unsigned,
    pairs_of_unsigned_and_u64_width_range, triples_of_signed_unsigned_width_range_and_bool_var_1,
    triples_of_unsigned_unsigned_width_range_and_bool_var_1,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_set_bit);
    register_demo!(registry, demo_u16_set_bit);
    register_demo!(registry, demo_u32_set_bit);
    register_demo!(registry, demo_u64_set_bit);
    register_demo!(registry, demo_usize_set_bit);
    register_demo!(registry, demo_i8_set_bit);
    register_demo!(registry, demo_i16_set_bit);
    register_demo!(registry, demo_i32_set_bit);
    register_demo!(registry, demo_i64_set_bit);
    register_demo!(registry, demo_isize_set_bit);

    register_demo!(registry, demo_u8_clear_bit);
    register_demo!(registry, demo_u16_clear_bit);
    register_demo!(registry, demo_u32_clear_bit);
    register_demo!(registry, demo_u64_clear_bit);
    register_demo!(registry, demo_usize_clear_bit);
    register_demo!(registry, demo_i8_clear_bit);
    register_demo!(registry, demo_i16_clear_bit);
    register_demo!(registry, demo_i32_clear_bit);
    register_demo!(registry, demo_i64_clear_bit);
    register_demo!(registry, demo_isize_clear_bit);

    register_demo!(registry, demo_u8_assign_bit);
    register_demo!(registry, demo_u16_assign_bit);
    register_demo!(registry, demo_u32_assign_bit);
    register_demo!(registry, demo_u64_assign_bit);
    register_demo!(registry, demo_usize_assign_bit);
    register_demo!(registry, demo_i8_assign_bit);
    register_demo!(registry, demo_i16_assign_bit);
    register_demo!(registry, demo_i32_assign_bit);
    register_demo!(registry, demo_i64_assign_bit);
    register_demo!(registry, demo_isize_assign_bit);

    register_demo!(registry, demo_u8_flip_bit);
    register_demo!(registry, demo_u16_flip_bit);
    register_demo!(registry, demo_u32_flip_bit);
    register_demo!(registry, demo_u64_flip_bit);
    register_demo!(registry, demo_usize_flip_bit);
    register_demo!(registry, demo_i8_flip_bit);
    register_demo!(registry, demo_i16_flip_bit);
    register_demo!(registry, demo_i32_flip_bit);
    register_demo!(registry, demo_i64_flip_bit);
    register_demo!(registry, demo_isize_flip_bit);

    register_bench!(registry, None, benchmark_u8_set_bit);
    register_bench!(registry, None, benchmark_u16_set_bit);
    register_bench!(registry, None, benchmark_u32_set_bit);
    register_bench!(registry, None, benchmark_u64_set_bit);
    register_bench!(registry, None, benchmark_usize_set_bit);
    register_bench!(registry, None, benchmark_i8_set_bit);
    register_bench!(registry, None, benchmark_i16_set_bit);
    register_bench!(registry, None, benchmark_i32_set_bit);
    register_bench!(registry, None, benchmark_i64_set_bit);
    register_bench!(registry, None, benchmark_isize_set_bit);

    register_bench!(registry, None, benchmark_u8_clear_bit);
    register_bench!(registry, None, benchmark_u16_clear_bit);
    register_bench!(registry, None, benchmark_u32_clear_bit);
    register_bench!(registry, None, benchmark_u64_clear_bit);
    register_bench!(registry, None, benchmark_usize_clear_bit);
    register_bench!(registry, None, benchmark_i8_clear_bit);
    register_bench!(registry, None, benchmark_i16_clear_bit);
    register_bench!(registry, None, benchmark_i32_clear_bit);
    register_bench!(registry, None, benchmark_i64_clear_bit);
    register_bench!(registry, None, benchmark_isize_clear_bit);

    register_bench!(registry, None, benchmark_u8_assign_bit);
    register_bench!(registry, None, benchmark_u16_assign_bit);
    register_bench!(registry, None, benchmark_u32_assign_bit);
    register_bench!(registry, None, benchmark_u64_assign_bit);
    register_bench!(registry, None, benchmark_usize_assign_bit);
    register_bench!(registry, None, benchmark_i8_assign_bit);
    register_bench!(registry, None, benchmark_i16_assign_bit);
    register_bench!(registry, None, benchmark_i32_assign_bit);
    register_bench!(registry, None, benchmark_i64_assign_bit);
    register_bench!(registry, None, benchmark_isize_assign_bit);

    register_bench!(registry, None, benchmark_u8_flip_bit);
    register_bench!(registry, None, benchmark_u16_flip_bit);
    register_bench!(registry, None, benchmark_u32_flip_bit);
    register_bench!(registry, None, benchmark_u64_flip_bit);
    register_bench!(registry, None, benchmark_usize_flip_bit);
    register_bench!(registry, None, benchmark_i8_flip_bit);
    register_bench!(registry, None, benchmark_i16_flip_bit);
    register_bench!(registry, None, benchmark_i32_flip_bit);
    register_bench!(registry, None, benchmark_i64_flip_bit);
    register_bench!(registry, None, benchmark_isize_flip_bit);
}

fn demo_unsigned_set_bit<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (mut n, index) in pairs_of_unsigned_and_u64_width_range::<T>(gm).take(limit) {
        let n_old = n;
        n.set_bit(index);
        println!("x := {}; x.set_bit({}); x = {}", n_old, index, n);
    }
}

fn demo_signed_set_bit<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (mut n, index) in pairs_of_signed_and_u64_width_range_var_1::<T>(gm).take(limit) {
        let n_old = n;
        n.set_bit(index);
        println!("x := {}; x.set_bit({}); x = {}", n_old, index, n);
    }
}

fn demo_unsigned_clear_bit<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (mut n, index) in pairs_of_unsigned_and_small_unsigned::<T, u64>(gm).take(limit) {
        let n_old = n;
        n.clear_bit(index);
        println!("x := {}; x.clear_bit({}); x = {}", n_old, index, n);
    }
}

fn demo_signed_clear_bit<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (mut n, index) in pairs_of_signed_and_u64_width_range_var_2::<T>(gm).take(limit) {
        let n_old = n;
        n.clear_bit(index);
        println!("x := {}; x.clear_bit({}); x = {}", n_old, index, n);
    }
}

fn demo_unsigned_assign_bit<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (mut n, index, bit) in
        triples_of_unsigned_unsigned_width_range_and_bool_var_1::<T, u64>(gm).take(limit)
    {
        let n_old = n;
        n.assign_bit(index, bit);
        println!(
            "x := {}; x.assign_bit({}, {}); x = {}",
            n_old, index, bit, n
        );
    }
}

fn demo_signed_assign_bit<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (mut n, index, bit) in
        triples_of_signed_unsigned_width_range_and_bool_var_1::<T, u64>(gm).take(limit)
    {
        let n_old = n;
        n.assign_bit(index, bit);
        println!(
            "x := {}; x.assign_bit({}, {}); x = {}",
            n_old, index, bit, n
        );
    }
}

fn demo_unsigned_flip_bit<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (mut n, index) in pairs_of_unsigned_and_u64_width_range::<T>(gm).take(limit) {
        let n_old = n;
        n.flip_bit(index);
        println!("x := {}; x.flip_bit({}); x = {}", n_old, index, n);
    }
}

fn demo_signed_flip_bit<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (mut n, index) in pairs_of_signed_and_u64_width_range::<T>(gm).take(limit) {
        let n_old = n;
        n.flip_bit(index);
        println!("x := {}; x.flip_bit({}); x = {}", n_old, index, n);
    }
}

fn benchmark_unsigned_set_bit<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!("{}.set_bit(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_u64_width_range::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "index",
        &mut [("Malachite", &mut (|(mut n, index)| n.set_bit(index)))],
    );
}

fn benchmark_signed_set_bit<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark_old(
        &format!("{}.set_bit(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_signed_and_u64_width_range_var_1::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "index",
        &mut [("Malachite", &mut (|(mut n, index)| n.set_bit(index)))],
    );
}

fn benchmark_unsigned_clear_bit<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!("{}.clear_bit(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_small_unsigned::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "index",
        &mut [("Malachite", &mut (|(mut n, index)| n.clear_bit(index)))],
    );
}

fn benchmark_signed_clear_bit<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark_old(
        &format!("{}.clear_bit(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_signed_and_u64_width_range_var_2::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "index",
        &mut [("Malachite", &mut (|(mut n, index)| n.clear_bit(index)))],
    );
}

fn benchmark_unsigned_assign_bit<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!("{}.assign_bit(u64)", T::NAME),
        BenchmarkType::Single,
        triples_of_unsigned_unsigned_width_range_and_bool_var_1::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index, _)| usize::exact_from(index)),
        "index",
        &mut [(
            "Malachite",
            &mut (|(mut n, index, bit)| n.assign_bit(index, bit)),
        )],
    );
}

fn benchmark_signed_assign_bit<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark_old(
        &format!("{}.assign_bit(u64)", T::NAME),
        BenchmarkType::Single,
        triples_of_signed_unsigned_width_range_and_bool_var_1::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index, _)| usize::exact_from(index)),
        "index",
        &mut [(
            "Malachite",
            &mut (|(mut n, index, bit)| n.assign_bit(index, bit)),
        )],
    );
}

fn benchmark_unsigned_flip_bit<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!("{}.flip_bit(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_u64_width_range::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "index",
        &mut [("Malachite", &mut (|(mut n, index)| n.flip_bit(index)))],
    );
}

fn benchmark_signed_flip_bit<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark_old(
        &format!("{}.flip_bit(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_signed_and_u64_width_range::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "index",
        &mut [("Malachite", &mut (|(mut n, index)| n.flip_bit(index)))],
    );
}

macro_rules! unsigned {
    (
        $t:ident,
        $set_bit_demo_name:ident,
        $clear_bit_demo_name:ident,
        $assign_bit_demo_name:ident,
        $flip_bit_demo_name:ident,
        $set_bit_bench_name:ident,
        $clear_bit_bench_name:ident,
        $assign_bit_bench_name:ident,
        $flip_bit_bench_name:ident
    ) => {
        fn $set_bit_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_set_bit::<$t>(gm, limit);
        }

        fn $clear_bit_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_clear_bit::<$t>(gm, limit);
        }

        fn $assign_bit_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_assign_bit::<$t>(gm, limit);
        }

        fn $flip_bit_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_flip_bit::<$t>(gm, limit);
        }

        fn $set_bit_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_set_bit::<$t>(gm, limit, file_name);
        }

        fn $clear_bit_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_clear_bit::<$t>(gm, limit, file_name);
        }

        fn $assign_bit_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_assign_bit::<$t>(gm, limit, file_name);
        }

        fn $flip_bit_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_flip_bit::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed {
    (
        $t:ident,
        $set_bit_demo_name:ident,
        $clear_bit_demo_name:ident,
        $assign_bit_demo_name:ident,
        $flip_bit_demo_name:ident,
        $set_bit_bench_name:ident,
        $clear_bit_bench_name:ident,
        $assign_bit_bench_name:ident,
        $flip_bit_bench_name:ident
    ) => {
        fn $set_bit_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_set_bit::<$t>(gm, limit);
        }

        fn $clear_bit_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_clear_bit::<$t>(gm, limit);
        }

        fn $assign_bit_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_assign_bit::<$t>(gm, limit);
        }

        fn $flip_bit_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_flip_bit::<$t>(gm, limit);
        }

        fn $set_bit_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_set_bit::<$t>(gm, limit, file_name);
        }

        fn $clear_bit_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_clear_bit::<$t>(gm, limit, file_name);
        }

        fn $assign_bit_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_assign_bit::<$t>(gm, limit, file_name);
        }

        fn $flip_bit_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_flip_bit::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    demo_u8_set_bit,
    demo_u8_clear_bit,
    demo_u8_assign_bit,
    demo_u8_flip_bit,
    benchmark_u8_set_bit,
    benchmark_u8_clear_bit,
    benchmark_u8_assign_bit,
    benchmark_u8_flip_bit
);
unsigned!(
    u16,
    demo_u16_set_bit,
    demo_u16_clear_bit,
    demo_u16_assign_bit,
    demo_u16_flip_bit,
    benchmark_u16_set_bit,
    benchmark_u16_clear_bit,
    benchmark_u16_assign_bit,
    benchmark_u16_flip_bit
);
unsigned!(
    u32,
    demo_u32_set_bit,
    demo_u32_clear_bit,
    demo_u32_assign_bit,
    demo_u32_flip_bit,
    benchmark_u32_set_bit,
    benchmark_u32_clear_bit,
    benchmark_u32_assign_bit,
    benchmark_u32_flip_bit
);
unsigned!(
    u64,
    demo_u64_set_bit,
    demo_u64_clear_bit,
    demo_u64_assign_bit,
    demo_u64_flip_bit,
    benchmark_u64_set_bit,
    benchmark_u64_clear_bit,
    benchmark_u64_assign_bit,
    benchmark_u64_flip_bit
);
unsigned!(
    usize,
    demo_usize_set_bit,
    demo_usize_clear_bit,
    demo_usize_assign_bit,
    demo_usize_flip_bit,
    benchmark_usize_set_bit,
    benchmark_usize_clear_bit,
    benchmark_usize_assign_bit,
    benchmark_usize_flip_bit
);
signed!(
    i8,
    demo_i8_set_bit,
    demo_i8_clear_bit,
    demo_i8_assign_bit,
    demo_i8_flip_bit,
    benchmark_i8_set_bit,
    benchmark_i8_clear_bit,
    benchmark_i8_assign_bit,
    benchmark_i8_flip_bit
);
signed!(
    i16,
    demo_i16_set_bit,
    demo_i16_clear_bit,
    demo_i16_assign_bit,
    demo_i16_flip_bit,
    benchmark_i16_set_bit,
    benchmark_i16_clear_bit,
    benchmark_i16_assign_bit,
    benchmark_i16_flip_bit
);
signed!(
    i32,
    demo_i32_set_bit,
    demo_i32_clear_bit,
    demo_i32_assign_bit,
    demo_i32_flip_bit,
    benchmark_i32_set_bit,
    benchmark_i32_clear_bit,
    benchmark_i32_assign_bit,
    benchmark_i32_flip_bit
);
signed!(
    i64,
    demo_i64_set_bit,
    demo_i64_clear_bit,
    demo_i64_assign_bit,
    demo_i64_flip_bit,
    benchmark_i64_set_bit,
    benchmark_i64_clear_bit,
    benchmark_i64_assign_bit,
    benchmark_i64_flip_bit
);
signed!(
    isize,
    demo_isize_set_bit,
    demo_isize_clear_bit,
    demo_isize_assign_bit,
    demo_isize_flip_bit,
    benchmark_isize_set_bit,
    benchmark_isize_clear_bit,
    benchmark_isize_assign_bit,
    benchmark_isize_flip_bit
);
