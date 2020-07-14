use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use rand::Rand;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType,
};
use malachite_test::inputs::base::{
    positive_unsigneds, signeds_no_max, signeds_no_min, unsigneds_no_max,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_increment);
    register_demo!(registry, demo_u16_increment);
    register_demo!(registry, demo_u32_increment);
    register_demo!(registry, demo_u64_increment);
    register_demo!(registry, demo_usize_increment);
    register_demo!(registry, demo_i8_increment);
    register_demo!(registry, demo_i16_increment);
    register_demo!(registry, demo_i32_increment);
    register_demo!(registry, demo_i64_increment);
    register_demo!(registry, demo_isize_increment);
    register_bench!(registry, None, benchmark_u8_increment);
    register_bench!(registry, None, benchmark_u16_increment);
    register_bench!(registry, None, benchmark_u32_increment);
    register_bench!(registry, None, benchmark_u64_increment);
    register_bench!(registry, None, benchmark_usize_increment);
    register_bench!(registry, None, benchmark_i8_increment);
    register_bench!(registry, None, benchmark_i16_increment);
    register_bench!(registry, None, benchmark_i32_increment);
    register_bench!(registry, None, benchmark_i64_increment);
    register_bench!(registry, None, benchmark_isize_increment);

    register_demo!(registry, demo_u8_decrement);
    register_demo!(registry, demo_u16_decrement);
    register_demo!(registry, demo_u32_decrement);
    register_demo!(registry, demo_u64_decrement);
    register_demo!(registry, demo_usize_decrement);
    register_demo!(registry, demo_i8_decrement);
    register_demo!(registry, demo_i16_decrement);
    register_demo!(registry, demo_i32_decrement);
    register_demo!(registry, demo_i64_decrement);
    register_demo!(registry, demo_isize_decrement);
    register_bench!(registry, None, benchmark_u8_decrement);
    register_bench!(registry, None, benchmark_u16_decrement);
    register_bench!(registry, None, benchmark_u32_decrement);
    register_bench!(registry, None, benchmark_u64_decrement);
    register_bench!(registry, None, benchmark_usize_decrement);
    register_bench!(registry, None, benchmark_i8_decrement);
    register_bench!(registry, None, benchmark_i16_decrement);
    register_bench!(registry, None, benchmark_i32_decrement);
    register_bench!(registry, None, benchmark_i64_decrement);
    register_bench!(registry, None, benchmark_isize_decrement);
}

fn demo_unsigned_increment<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for mut n in unsigneds_no_max::<T>(gm).take(limit) {
        let n_old = n;
        n.increment();
        println!("n := {}; n.increment(); n = {}", n_old, n);
    }
}

fn demo_signed_increment<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for mut n in signeds_no_max::<T>(gm).take(limit) {
        let n_old = n;
        n.increment();
        println!("n := {}; n.increment(); n = {}", n_old, n);
    }
}

fn demo_unsigned_decrement<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for mut n in positive_unsigneds::<T>(gm).take(limit) {
        let n_old = n;
        n.decrement();
        println!("n := {}; n.decrement(); n = {}", n_old, n);
    }
}

fn demo_signed_decrement<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for mut n in signeds_no_min::<T>(gm).take(limit) {
        let n_old = n;
        n.decrement();
        println!("n := {}; n.decrement(); n = {}", n_old, n);
    }
}

fn benchmark_unsigned_increment<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.increment()", T::NAME),
        BenchmarkType::Single,
        unsigneds_no_max::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&n| usize::exact_from(n.significant_bits())),
        "index",
        &mut [("malachite", &mut (|mut n| n.increment()))],
    );
}

fn benchmark_signed_increment<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.increment()", T::NAME),
        BenchmarkType::Single,
        signeds_no_max::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&n| usize::exact_from(n.significant_bits())),
        "index",
        &mut [("malachite", &mut (|mut n| n.increment()))],
    );
}

fn benchmark_unsigned_decrement<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.decrement()", T::NAME),
        BenchmarkType::Single,
        positive_unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&n| usize::exact_from(n.significant_bits())),
        "index",
        &mut [("malachite", &mut (|mut n| n.decrement()))],
    );
}

fn benchmark_signed_decrement<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.decrement()", T::NAME),
        BenchmarkType::Single,
        signeds_no_min::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&n| usize::exact_from(n.significant_bits())),
        "index",
        &mut [("malachite", &mut (|mut n| n.decrement()))],
    );
}

macro_rules! unsigned {
    (
        $t:ident,
        $increment_demo_name:ident,
        $decrement_demo_name:ident,
        $increment_bench_name:ident,
        $decrement_bench_name:ident
    ) => {
        fn $increment_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_increment::<$t>(gm, limit);
        }

        fn $decrement_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_decrement::<$t>(gm, limit);
        }

        fn $increment_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_increment::<$t>(gm, limit, file_name);
        }

        fn $decrement_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_decrement::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed {
    (
        $t:ident,
        $increment_demo_name:ident,
        $decrement_demo_name:ident,
        $increment_bench_name:ident,
        $decrement_bench_name:ident
    ) => {
        fn $increment_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_increment::<$t>(gm, limit);
        }

        fn $decrement_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_decrement::<$t>(gm, limit);
        }

        fn $increment_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_increment::<$t>(gm, limit, file_name);
        }

        fn $decrement_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_decrement::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    demo_u8_increment,
    demo_u8_decrement,
    benchmark_u8_increment,
    benchmark_u8_decrement
);
unsigned!(
    u16,
    demo_u16_increment,
    demo_u16_decrement,
    benchmark_u16_increment,
    benchmark_u16_decrement
);
unsigned!(
    u32,
    demo_u32_increment,
    demo_u32_decrement,
    benchmark_u32_increment,
    benchmark_u32_decrement
);
unsigned!(
    u64,
    demo_u64_increment,
    demo_u64_decrement,
    benchmark_u64_increment,
    benchmark_u64_decrement
);
unsigned!(
    usize,
    demo_usize_increment,
    demo_usize_decrement,
    benchmark_usize_increment,
    benchmark_usize_decrement
);
signed!(
    i8,
    demo_i8_increment,
    demo_i8_decrement,
    benchmark_i8_increment,
    benchmark_i8_decrement
);
signed!(
    i16,
    demo_i16_increment,
    demo_i16_decrement,
    benchmark_i16_increment,
    benchmark_i16_decrement
);
signed!(
    i32,
    demo_i32_increment,
    demo_i32_decrement,
    benchmark_i32_increment,
    benchmark_i32_decrement
);
signed!(
    i64,
    demo_i64_increment,
    demo_i64_decrement,
    benchmark_i64_increment,
    benchmark_i64_decrement
);
signed!(
    isize,
    demo_isize_increment,
    demo_isize_decrement,
    benchmark_isize_increment,
    benchmark_isize_decrement
);
