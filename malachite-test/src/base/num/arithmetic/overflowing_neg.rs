use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use rand::Rand;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{signeds, unsigneds};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_overflowing_neg_assign);
    register_demo!(registry, demo_u16_overflowing_neg_assign);
    register_demo!(registry, demo_u32_overflowing_neg_assign);
    register_demo!(registry, demo_u64_overflowing_neg_assign);
    register_demo!(registry, demo_usize_overflowing_neg_assign);
    register_demo!(registry, demo_i8_overflowing_neg_assign);
    register_demo!(registry, demo_i16_overflowing_neg_assign);
    register_demo!(registry, demo_i32_overflowing_neg_assign);
    register_demo!(registry, demo_i64_overflowing_neg_assign);
    register_demo!(registry, demo_isize_overflowing_neg_assign);

    register_bench!(registry, None, benchmark_u8_overflowing_neg_assign);
    register_bench!(registry, None, benchmark_u16_overflowing_neg_assign);
    register_bench!(registry, None, benchmark_u32_overflowing_neg_assign);
    register_bench!(registry, None, benchmark_u64_overflowing_neg_assign);
    register_bench!(registry, None, benchmark_usize_overflowing_neg_assign);
    register_bench!(registry, None, benchmark_i8_overflowing_neg_assign);
    register_bench!(registry, None, benchmark_i16_overflowing_neg_assign);
    register_bench!(registry, None, benchmark_i32_overflowing_neg_assign);
    register_bench!(registry, None, benchmark_i64_overflowing_neg_assign);
    register_bench!(registry, None, benchmark_isize_overflowing_neg_assign);
}

fn demo_unsigned_overflowing_neg_assign<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) {
    for mut u in unsigneds::<T>(gm).take(limit) {
        let old_u = u;
        let overflow = u.overflowing_neg_assign();
        println!(
            "u := {}; u.overflowing_neg_assign() = {}; u = {}",
            old_u, overflow, u
        );
    }
}

fn demo_signed_overflowing_neg_assign<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for mut i in signeds::<T>(gm).take(limit) {
        let old_i = i;
        let overflow = i.overflowing_neg_assign();
        println!(
            "i := {}; i.overflowing_neg_assign() = {}; i = {}",
            old_i, overflow, i
        );
    }
}

fn benchmark_unsigned_overflowing_neg_assign<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.overflowing_neg_assign()", T::NAME),
        BenchmarkType::Single,
        unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|u| usize::exact_from(u.significant_bits())),
        "u.significant_bits()",
        &mut [(
            "malachite",
            &mut (|mut u| no_out!(u.overflowing_neg_assign())),
        )],
    );
}

fn benchmark_signed_overflowing_neg_assign<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.overflowing_neg_assign()", T::NAME),
        BenchmarkType::Single,
        signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|i| usize::exact_from(i.significant_bits())),
        "i.significant_bits()",
        &mut [(
            "malachite",
            &mut (|mut i| no_out!(i.overflowing_neg_assign())),
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
            demo_unsigned_overflowing_neg_assign::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_overflowing_neg_assign::<$t>(gm, limit, file_name);
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
            demo_signed_overflowing_neg_assign::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_overflowing_neg_assign::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    demo_u8_overflowing_neg_assign,
    benchmark_u8_overflowing_neg_assign
);
unsigned!(
    u16,
    demo_u16_overflowing_neg_assign,
    benchmark_u16_overflowing_neg_assign
);
unsigned!(
    u32,
    demo_u32_overflowing_neg_assign,
    benchmark_u32_overflowing_neg_assign
);
unsigned!(
    u64,
    demo_u64_overflowing_neg_assign,
    benchmark_u64_overflowing_neg_assign
);
unsigned!(
    usize,
    demo_usize_overflowing_neg_assign,
    benchmark_usize_overflowing_neg_assign
);

signed!(
    i8,
    demo_i8_overflowing_neg_assign,
    benchmark_i8_overflowing_neg_assign
);
signed!(
    i16,
    demo_i16_overflowing_neg_assign,
    benchmark_i16_overflowing_neg_assign
);
signed!(
    i32,
    demo_i32_overflowing_neg_assign,
    benchmark_i32_overflowing_neg_assign
);
signed!(
    i64,
    demo_i64_overflowing_neg_assign,
    benchmark_i64_overflowing_neg_assign
);
signed!(
    isize,
    demo_isize_overflowing_neg_assign,
    benchmark_isize_overflowing_neg_assign
);
