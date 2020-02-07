use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::integers::_to_bits_desc_alt;
use malachite_base::num::logic::signeds::{_to_bits_asc_signed_naive, _to_bits_desc_signed_naive};
use malachite_base::num::logic::unsigneds::{
    _to_bits_asc_unsigned_naive, _to_bits_desc_unsigned_naive,
};
use rand::Rand;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{signeds, unsigneds};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_to_bits_asc);
    register_demo!(registry, demo_u16_to_bits_asc);
    register_demo!(registry, demo_u32_to_bits_asc);
    register_demo!(registry, demo_u64_to_bits_asc);
    register_demo!(registry, demo_usize_to_bits_asc);
    register_demo!(registry, demo_i8_to_bits_asc);
    register_demo!(registry, demo_i16_to_bits_asc);
    register_demo!(registry, demo_i32_to_bits_asc);
    register_demo!(registry, demo_i64_to_bits_asc);
    register_demo!(registry, demo_isize_to_bits_asc);

    register_demo!(registry, demo_u8_to_bits_desc);
    register_demo!(registry, demo_u16_to_bits_desc);
    register_demo!(registry, demo_u32_to_bits_desc);
    register_demo!(registry, demo_u64_to_bits_desc);
    register_demo!(registry, demo_usize_to_bits_desc);
    register_demo!(registry, demo_i8_to_bits_desc);
    register_demo!(registry, demo_i16_to_bits_desc);
    register_demo!(registry, demo_i32_to_bits_desc);
    register_demo!(registry, demo_i64_to_bits_desc);
    register_demo!(registry, demo_isize_to_bits_desc);

    register_bench!(registry, None, benchmark_u8_to_bits_asc_algorithms);
    register_bench!(registry, None, benchmark_u16_to_bits_asc_algorithms);
    register_bench!(registry, None, benchmark_u32_to_bits_asc_algorithms);
    register_bench!(registry, None, benchmark_u64_to_bits_asc_algorithms);
    register_bench!(registry, None, benchmark_usize_to_bits_asc_algorithms);
    register_bench!(registry, None, benchmark_i8_to_bits_asc_algorithms);
    register_bench!(registry, None, benchmark_i16_to_bits_asc_algorithms);
    register_bench!(registry, None, benchmark_i32_to_bits_asc_algorithms);
    register_bench!(registry, None, benchmark_i64_to_bits_asc_algorithms);
    register_bench!(registry, None, benchmark_isize_to_bits_asc_algorithms);

    register_bench!(registry, None, benchmark_u8_to_bits_desc_algorithms);
    register_bench!(registry, None, benchmark_u16_to_bits_desc_algorithms);
    register_bench!(registry, None, benchmark_u32_to_bits_desc_algorithms);
    register_bench!(registry, None, benchmark_u64_to_bits_desc_algorithms);
    register_bench!(registry, None, benchmark_usize_to_bits_desc_algorithms);
    register_bench!(registry, None, benchmark_i8_to_bits_desc_algorithms);
    register_bench!(registry, None, benchmark_i16_to_bits_desc_algorithms);
    register_bench!(registry, None, benchmark_i32_to_bits_desc_algorithms);
    register_bench!(registry, None, benchmark_i64_to_bits_desc_algorithms);
    register_bench!(registry, None, benchmark_isize_to_bits_desc_algorithms);
}

fn demo_unsigned_to_bits_asc<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for u in unsigneds::<T>(gm).take(limit) {
        println!("{}.to_bits_asc() = {:?}", u, u.to_bits_asc());
    }
}

fn demo_signed_to_bits_asc<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for i in signeds::<T>(gm).take(limit) {
        println!("{}.to_bits_asc() = {:?}", i, i.to_bits_asc());
    }
}

fn demo_unsigned_to_bits_desc<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for u in unsigneds::<T>(gm).take(limit) {
        println!("{}.to_bits_desc() = {:?}", u, u.to_bits_desc());
    }
}

fn demo_signed_to_bits_desc<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for i in signeds::<T>(gm).take(limit) {
        println!("{}.to_bits_desc() = {:?}", i, i.to_bits_desc());
    }
}

fn benchmark_unsigned_to_bits_asc_algorithms<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.to_bits_asc()", T::NAME),
        BenchmarkType::Algorithms,
        unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|u| usize::wrapping_from(u.significant_bits())),
        "u.significant_bits()",
        &mut [
            ("malachite", &mut (|u| no_out!(u.to_bits_asc()))),
            ("naive", &mut (|u| no_out!(_to_bits_asc_unsigned_naive(u)))),
        ],
    );
}

fn benchmark_signed_to_bits_asc_algorithms<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.to_bits_asc()", T::NAME),
        BenchmarkType::Algorithms,
        signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|i| usize::wrapping_from(i.significant_bits())),
        "i.significant_bits()",
        &mut [
            ("malachite", &mut (|i| no_out!(i.to_bits_asc()))),
            ("naive", &mut (|i| no_out!(_to_bits_asc_signed_naive(i)))),
        ],
    );
}

fn benchmark_unsigned_to_bits_desc_algorithms<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.to_bits_asc()", T::NAME),
        BenchmarkType::Algorithms,
        unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|u| usize::wrapping_from(u.significant_bits())),
        "u.significant_bits()",
        &mut [
            ("default", &mut (|u| no_out!(u.to_bits_asc()))),
            ("alt", &mut (|u| no_out!(_to_bits_desc_alt(&u)))),
            ("naive", &mut (|u| no_out!(_to_bits_desc_unsigned_naive(u)))),
        ],
    );
}

fn benchmark_signed_to_bits_desc_algorithms<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.to_bits_asc()", T::NAME),
        BenchmarkType::Algorithms,
        signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|i| usize::wrapping_from(i.significant_bits())),
        "i.significant_bits()",
        &mut [
            ("default", &mut (|i| no_out!(i.to_bits_asc()))),
            ("alt", &mut (|i| no_out!(_to_bits_desc_alt(&i)))),
            ("naive", &mut (|u| no_out!(_to_bits_desc_signed_naive(u)))),
        ],
    );
}

macro_rules! unsigned {
    (
        $t:ident,
        $asc_demo_name:ident,
        $asc_bench_name:ident,
        $desc_demo_name:ident,
        $desc_bench_name:ident
    ) => {
        fn $asc_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_to_bits_asc::<$t>(gm, limit);
        }

        fn $asc_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_to_bits_asc_algorithms::<$t>(gm, limit, file_name);
        }

        fn $desc_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_to_bits_desc::<$t>(gm, limit);
        }

        fn $desc_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_to_bits_desc_algorithms::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed {
    (
        $t:ident,
        $asc_demo_name:ident,
        $asc_bench_name:ident,
        $desc_demo_name:ident,
        $desc_bench_name:ident
    ) => {
        fn $asc_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_to_bits_asc::<$t>(gm, limit);
        }

        fn $asc_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_to_bits_asc_algorithms::<$t>(gm, limit, file_name);
        }

        fn $desc_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_to_bits_desc::<$t>(gm, limit);
        }

        fn $desc_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_to_bits_desc_algorithms::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    demo_u8_to_bits_asc,
    benchmark_u8_to_bits_asc_algorithms,
    demo_u8_to_bits_desc,
    benchmark_u8_to_bits_desc_algorithms
);
unsigned!(
    u16,
    demo_u16_to_bits_asc,
    benchmark_u16_to_bits_asc_algorithms,
    demo_u16_to_bits_desc,
    benchmark_u16_to_bits_desc_algorithms
);
unsigned!(
    u32,
    demo_u32_to_bits_asc,
    benchmark_u32_to_bits_asc_algorithms,
    demo_u32_to_bits_desc,
    benchmark_u32_to_bits_desc_algorithms
);
unsigned!(
    u64,
    demo_u64_to_bits_asc,
    benchmark_u64_to_bits_asc_algorithms,
    demo_u64_to_bits_desc,
    benchmark_u64_to_bits_desc_algorithms
);
unsigned!(
    usize,
    demo_usize_to_bits_asc,
    benchmark_usize_to_bits_asc_algorithms,
    demo_usize_to_bits_desc,
    benchmark_usize_to_bits_desc_algorithms
);

signed!(
    i8,
    demo_i8_to_bits_asc,
    benchmark_i8_to_bits_asc_algorithms,
    demo_i8_to_bits_desc,
    benchmark_i8_to_bits_desc_algorithms
);
signed!(
    i16,
    demo_i16_to_bits_asc,
    benchmark_i16_to_bits_asc_algorithms,
    demo_i16_to_bits_desc,
    benchmark_i16_to_bits_desc_algorithms
);
signed!(
    i32,
    demo_i32_to_bits_asc,
    benchmark_i32_to_bits_asc_algorithms,
    demo_i32_to_bits_desc,
    benchmark_i32_to_bits_desc_algorithms
);
signed!(
    i64,
    demo_i64_to_bits_asc,
    benchmark_i64_to_bits_asc_algorithms,
    demo_i64_to_bits_desc,
    benchmark_i64_to_bits_desc_algorithms
);
signed!(
    isize,
    demo_isize_to_bits_asc,
    benchmark_isize_to_bits_asc_algorithms,
    demo_isize_to_bits_desc,
    benchmark_isize_to_bits_desc_algorithms
);
