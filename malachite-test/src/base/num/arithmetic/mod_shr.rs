use malachite_base::num::arithmetic::traits::{ModShr, ModShrAssign};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::triples_of_unsigned_signed_and_unsigned_var_1;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_mod_shr_assign_i8);
    register_demo!(registry, demo_u8_mod_shr_assign_i16);
    register_demo!(registry, demo_u8_mod_shr_assign_i32);
    register_demo!(registry, demo_u8_mod_shr_assign_i64);
    register_demo!(registry, demo_u8_mod_shr_assign_isize);
    register_demo!(registry, demo_u16_mod_shr_assign_i8);
    register_demo!(registry, demo_u16_mod_shr_assign_i16);
    register_demo!(registry, demo_u16_mod_shr_assign_i32);
    register_demo!(registry, demo_u16_mod_shr_assign_i64);
    register_demo!(registry, demo_u16_mod_shr_assign_isize);
    register_demo!(registry, demo_u32_mod_shr_assign_i8);
    register_demo!(registry, demo_u32_mod_shr_assign_i16);
    register_demo!(registry, demo_u32_mod_shr_assign_i32);
    register_demo!(registry, demo_u32_mod_shr_assign_i64);
    register_demo!(registry, demo_u32_mod_shr_assign_isize);
    register_demo!(registry, demo_u64_mod_shr_assign_i8);
    register_demo!(registry, demo_u64_mod_shr_assign_i16);
    register_demo!(registry, demo_u64_mod_shr_assign_i32);
    register_demo!(registry, demo_u64_mod_shr_assign_i64);
    register_demo!(registry, demo_u64_mod_shr_assign_isize);
    register_demo!(registry, demo_usize_mod_shr_assign_i8);
    register_demo!(registry, demo_usize_mod_shr_assign_i16);
    register_demo!(registry, demo_usize_mod_shr_assign_i32);
    register_demo!(registry, demo_usize_mod_shr_assign_i64);
    register_demo!(registry, demo_usize_mod_shr_assign_isize);

    register_demo!(registry, demo_u8_mod_shr_i8);
    register_demo!(registry, demo_u8_mod_shr_i16);
    register_demo!(registry, demo_u8_mod_shr_i32);
    register_demo!(registry, demo_u8_mod_shr_i64);
    register_demo!(registry, demo_u8_mod_shr_isize);
    register_demo!(registry, demo_u16_mod_shr_i8);
    register_demo!(registry, demo_u16_mod_shr_i16);
    register_demo!(registry, demo_u16_mod_shr_i32);
    register_demo!(registry, demo_u16_mod_shr_i64);
    register_demo!(registry, demo_u16_mod_shr_isize);
    register_demo!(registry, demo_u32_mod_shr_i8);
    register_demo!(registry, demo_u32_mod_shr_i16);
    register_demo!(registry, demo_u32_mod_shr_i32);
    register_demo!(registry, demo_u32_mod_shr_i64);
    register_demo!(registry, demo_u32_mod_shr_isize);
    register_demo!(registry, demo_u64_mod_shr_i8);
    register_demo!(registry, demo_u64_mod_shr_i16);
    register_demo!(registry, demo_u64_mod_shr_i32);
    register_demo!(registry, demo_u64_mod_shr_i64);
    register_demo!(registry, demo_u64_mod_shr_isize);
    register_demo!(registry, demo_usize_mod_shr_i8);
    register_demo!(registry, demo_usize_mod_shr_i16);
    register_demo!(registry, demo_usize_mod_shr_i32);
    register_demo!(registry, demo_usize_mod_shr_i64);
    register_demo!(registry, demo_usize_mod_shr_isize);

    register_bench!(registry, Large, benchmark_u8_mod_shr_assign_i8);
    register_bench!(registry, Large, benchmark_u8_mod_shr_assign_i16);
    register_bench!(registry, Large, benchmark_u8_mod_shr_assign_i32);
    register_bench!(registry, Large, benchmark_u8_mod_shr_assign_i64);
    register_bench!(registry, Large, benchmark_u8_mod_shr_assign_isize);
    register_bench!(registry, Large, benchmark_u16_mod_shr_assign_i8);
    register_bench!(registry, Large, benchmark_u16_mod_shr_assign_i16);
    register_bench!(registry, Large, benchmark_u16_mod_shr_assign_i32);
    register_bench!(registry, Large, benchmark_u16_mod_shr_assign_i64);
    register_bench!(registry, Large, benchmark_u16_mod_shr_assign_isize);
    register_bench!(registry, Large, benchmark_u32_mod_shr_assign_i8);
    register_bench!(registry, Large, benchmark_u32_mod_shr_assign_i16);
    register_bench!(registry, Large, benchmark_u32_mod_shr_assign_i32);
    register_bench!(registry, Large, benchmark_u32_mod_shr_assign_i64);
    register_bench!(registry, Large, benchmark_u32_mod_shr_assign_isize);
    register_bench!(registry, Large, benchmark_u64_mod_shr_assign_i8);
    register_bench!(registry, Large, benchmark_u64_mod_shr_assign_i16);
    register_bench!(registry, Large, benchmark_u64_mod_shr_assign_i32);
    register_bench!(registry, Large, benchmark_u64_mod_shr_assign_i64);
    register_bench!(registry, Large, benchmark_u64_mod_shr_assign_isize);
    register_bench!(registry, Large, benchmark_usize_mod_shr_assign_i8);
    register_bench!(registry, Large, benchmark_usize_mod_shr_assign_i16);
    register_bench!(registry, Large, benchmark_usize_mod_shr_assign_i32);
    register_bench!(registry, Large, benchmark_usize_mod_shr_assign_i64);
    register_bench!(registry, Large, benchmark_usize_mod_shr_assign_isize);

    register_bench!(registry, Large, benchmark_u8_mod_shr_i8);
    register_bench!(registry, Large, benchmark_u8_mod_shr_i16);
    register_bench!(registry, Large, benchmark_u8_mod_shr_i32);
    register_bench!(registry, Large, benchmark_u8_mod_shr_i64);
    register_bench!(registry, Large, benchmark_u8_mod_shr_isize);
    register_bench!(registry, Large, benchmark_u16_mod_shr_i8);
    register_bench!(registry, Large, benchmark_u16_mod_shr_i16);
    register_bench!(registry, Large, benchmark_u16_mod_shr_i32);
    register_bench!(registry, Large, benchmark_u16_mod_shr_i64);
    register_bench!(registry, Large, benchmark_u16_mod_shr_isize);
    register_bench!(registry, Large, benchmark_u32_mod_shr_i8);
    register_bench!(registry, Large, benchmark_u32_mod_shr_i16);
    register_bench!(registry, Large, benchmark_u32_mod_shr_i32);
    register_bench!(registry, Large, benchmark_u32_mod_shr_i64);
    register_bench!(registry, Large, benchmark_u32_mod_shr_isize);
    register_bench!(registry, Large, benchmark_u64_mod_shr_i8);
    register_bench!(registry, Large, benchmark_u64_mod_shr_i16);
    register_bench!(registry, Large, benchmark_u64_mod_shr_i32);
    register_bench!(registry, Large, benchmark_u64_mod_shr_i64);
    register_bench!(registry, Large, benchmark_u64_mod_shr_isize);
    register_bench!(registry, Large, benchmark_usize_mod_shr_i8);
    register_bench!(registry, Large, benchmark_usize_mod_shr_i16);
    register_bench!(registry, Large, benchmark_usize_mod_shr_i32);
    register_bench!(registry, Large, benchmark_usize_mod_shr_i64);
    register_bench!(registry, Large, benchmark_usize_mod_shr_isize);
}

fn demo_mod_shr<T: PrimitiveUnsigned + Rand + SampleRange, U: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) where
    T: ModShr<U, T, Output = T>,
    U::UnsignedOfEqualWidth: Rand,
    U: WrappingFrom<<U as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, pow, m) in triples_of_unsigned_signed_and_unsigned_var_1::<T, U>(gm).take(limit) {
        println!("{}.pow({}) === {} mod {}", x, pow, x.mod_shr(pow, m), m);
    }
}

fn demo_mod_shr_assign<T: PrimitiveUnsigned + Rand + SampleRange, U: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) where
    T: ModShrAssign<U, T>,
    U::UnsignedOfEqualWidth: Rand,
    U: WrappingFrom<<U as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (mut x, pow, m) in triples_of_unsigned_signed_and_unsigned_var_1::<T, U>(gm).take(limit) {
        let old_x = x;
        x.mod_shr_assign(pow, m);
        println!(
            "x := {}; x.mod_shr_assign({}, {}); x = {}",
            old_x, pow, m, x
        );
    }
}

fn benchmark_mod_shr<T: PrimitiveUnsigned + Rand + SampleRange, U: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T: ModShr<U, T, Output = T>,
    U::UnsignedOfEqualWidth: Rand,
    U: WrappingFrom<<U as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark_old(
        &format!("{}.mod_shr({}, {})", T::NAME, U::NAME, T::NAME),
        BenchmarkType::Single,
        triples_of_unsigned_signed_and_unsigned_var_1::<T, U>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, pow, _)| usize::exact_from(pow.significant_bits())),
        "pow.significant_bits()",
        &mut [("Malachite", &mut (|(x, pow, m)| no_out!(x.mod_shr(pow, m))))],
    );
}

fn benchmark_mod_shr_assign<T: PrimitiveUnsigned + Rand + SampleRange, U: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T: ModShrAssign<U, T>,
    U::UnsignedOfEqualWidth: Rand,
    U: WrappingFrom<<U as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark_old(
        &format!("{}.mod_shr_assign({}, {})", T::NAME, U::NAME, T::NAME),
        BenchmarkType::Single,
        triples_of_unsigned_signed_and_unsigned_var_1::<T, U>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, pow, _)| usize::exact_from(pow.significant_bits())),
        "pow.significant_bits()",
        &mut [(
            "Malachite",
            &mut (|(mut x, pow, m)| x.mod_shr_assign(pow, m)),
        )],
    );
}

macro_rules! mod_shr_u_i {
    (
        $t:ident,
        $u:ident,
        $demo_mod_shr_assign:ident,
        $demo_mod_shr:ident,
        $benchmark_mod_shr_assign:ident,
        $benchmark_mod_shr:ident
    ) => {
        fn $demo_mod_shr_assign(gm: GenerationMode, limit: usize) {
            demo_mod_shr_assign::<$t, $u>(gm, limit);
        }

        fn $demo_mod_shr(gm: GenerationMode, limit: usize) {
            demo_mod_shr::<$t, $u>(gm, limit);
        }

        fn $benchmark_mod_shr_assign(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_shr::<$t, $u>(gm, limit, file_name);
        }

        fn $benchmark_mod_shr(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_shr_assign::<$t, $u>(gm, limit, file_name);
        }
    };
}
mod_shr_u_i!(
    u8,
    i8,
    demo_u8_mod_shr_assign_i8,
    demo_u8_mod_shr_i8,
    benchmark_u8_mod_shr_assign_i8,
    benchmark_u8_mod_shr_i8
);
mod_shr_u_i!(
    u8,
    i16,
    demo_u8_mod_shr_assign_i16,
    demo_u8_mod_shr_i16,
    benchmark_u8_mod_shr_assign_i16,
    benchmark_u8_mod_shr_i16
);
mod_shr_u_i!(
    u8,
    i32,
    demo_u8_mod_shr_assign_i32,
    demo_u8_mod_shr_i32,
    benchmark_u8_mod_shr_assign_i32,
    benchmark_u8_mod_shr_i32
);
mod_shr_u_i!(
    u8,
    i64,
    demo_u8_mod_shr_assign_i64,
    demo_u8_mod_shr_i64,
    benchmark_u8_mod_shr_assign_i64,
    benchmark_u8_mod_shr_i64
);
mod_shr_u_i!(
    u8,
    isize,
    demo_u8_mod_shr_assign_isize,
    demo_u8_mod_shr_isize,
    benchmark_u8_mod_shr_assign_isize,
    benchmark_u8_mod_shr_isize
);

mod_shr_u_i!(
    u16,
    i8,
    demo_u16_mod_shr_assign_i8,
    demo_u16_mod_shr_i8,
    benchmark_u16_mod_shr_assign_i8,
    benchmark_u16_mod_shr_i8
);
mod_shr_u_i!(
    u16,
    i16,
    demo_u16_mod_shr_assign_i16,
    demo_u16_mod_shr_i16,
    benchmark_u16_mod_shr_assign_i16,
    benchmark_u16_mod_shr_i16
);
mod_shr_u_i!(
    u16,
    i32,
    demo_u16_mod_shr_assign_i32,
    demo_u16_mod_shr_i32,
    benchmark_u16_mod_shr_assign_i32,
    benchmark_u16_mod_shr_i32
);
mod_shr_u_i!(
    u16,
    i64,
    demo_u16_mod_shr_assign_i64,
    demo_u16_mod_shr_i64,
    benchmark_u16_mod_shr_assign_i64,
    benchmark_u16_mod_shr_i64
);
mod_shr_u_i!(
    u16,
    isize,
    demo_u16_mod_shr_assign_isize,
    demo_u16_mod_shr_isize,
    benchmark_u16_mod_shr_assign_isize,
    benchmark_u16_mod_shr_isize
);

mod_shr_u_i!(
    u32,
    i8,
    demo_u32_mod_shr_assign_i8,
    demo_u32_mod_shr_i8,
    benchmark_u32_mod_shr_assign_i8,
    benchmark_u32_mod_shr_i8
);
mod_shr_u_i!(
    u32,
    i16,
    demo_u32_mod_shr_assign_i16,
    demo_u32_mod_shr_i16,
    benchmark_u32_mod_shr_assign_i16,
    benchmark_u32_mod_shr_i16
);
mod_shr_u_i!(
    u32,
    i32,
    demo_u32_mod_shr_assign_i32,
    demo_u32_mod_shr_i32,
    benchmark_u32_mod_shr_assign_i32,
    benchmark_u32_mod_shr_i32
);
mod_shr_u_i!(
    u32,
    i64,
    demo_u32_mod_shr_assign_i64,
    demo_u32_mod_shr_i64,
    benchmark_u32_mod_shr_assign_i64,
    benchmark_u32_mod_shr_i64
);
mod_shr_u_i!(
    u32,
    isize,
    demo_u32_mod_shr_assign_isize,
    demo_u32_mod_shr_isize,
    benchmark_u32_mod_shr_assign_isize,
    benchmark_u32_mod_shr_isize
);

mod_shr_u_i!(
    u64,
    i8,
    demo_u64_mod_shr_assign_i8,
    demo_u64_mod_shr_i8,
    benchmark_u64_mod_shr_assign_i8,
    benchmark_u64_mod_shr_i8
);
mod_shr_u_i!(
    u64,
    i16,
    demo_u64_mod_shr_assign_i16,
    demo_u64_mod_shr_i16,
    benchmark_u64_mod_shr_assign_i16,
    benchmark_u64_mod_shr_i16
);
mod_shr_u_i!(
    u64,
    i32,
    demo_u64_mod_shr_assign_i32,
    demo_u64_mod_shr_i32,
    benchmark_u64_mod_shr_assign_i32,
    benchmark_u64_mod_shr_i32
);
mod_shr_u_i!(
    u64,
    i64,
    demo_u64_mod_shr_assign_i64,
    demo_u64_mod_shr_i64,
    benchmark_u64_mod_shr_assign_i64,
    benchmark_u64_mod_shr_i64
);
mod_shr_u_i!(
    u64,
    isize,
    demo_u64_mod_shr_assign_isize,
    demo_u64_mod_shr_isize,
    benchmark_u64_mod_shr_assign_isize,
    benchmark_u64_mod_shr_isize
);

mod_shr_u_i!(
    usize,
    i8,
    demo_usize_mod_shr_assign_i8,
    demo_usize_mod_shr_i8,
    benchmark_usize_mod_shr_assign_i8,
    benchmark_usize_mod_shr_i8
);
mod_shr_u_i!(
    usize,
    i16,
    demo_usize_mod_shr_assign_i16,
    demo_usize_mod_shr_i16,
    benchmark_usize_mod_shr_assign_i16,
    benchmark_usize_mod_shr_i16
);
mod_shr_u_i!(
    usize,
    i32,
    demo_usize_mod_shr_assign_i32,
    demo_usize_mod_shr_i32,
    benchmark_usize_mod_shr_assign_i32,
    benchmark_usize_mod_shr_i32
);
mod_shr_u_i!(
    usize,
    i64,
    demo_usize_mod_shr_assign_i64,
    demo_usize_mod_shr_i64,
    benchmark_usize_mod_shr_assign_i64,
    benchmark_usize_mod_shr_i64
);
mod_shr_u_i!(
    usize,
    isize,
    demo_usize_mod_shr_assign_isize,
    demo_usize_mod_shr_isize,
    benchmark_usize_mod_shr_assign_isize,
    benchmark_usize_mod_shr_isize
);
