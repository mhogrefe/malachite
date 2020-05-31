use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base_test_util::num::logic::bit_convertible::{
    from_bits_asc_alt, from_bits_asc_signed_naive, from_bits_asc_unsigned_naive,
    from_bits_desc_alt, from_bits_desc_signed_naive, from_bits_desc_unsigned_naive,
    to_bits_asc_alt, to_bits_asc_signed_naive, to_bits_asc_unsigned_naive, to_bits_desc_alt,
    to_bits_desc_signed_naive, to_bits_desc_unsigned_naive,
};
use rand::Rand;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    signeds, unsigneds, vecs_of_bool_var_2, vecs_of_bool_var_3, vecs_of_bool_var_4,
    vecs_of_bool_var_5,
};

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

    register_bench!(registry, None, benchmark_u8_to_bits_asc_evaluation_strategy);
    register_bench!(
        registry,
        None,
        benchmark_u16_to_bits_asc_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u32_to_bits_asc_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u64_to_bits_asc_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_usize_to_bits_asc_evaluation_strategy
    );
    register_bench!(registry, None, benchmark_i8_to_bits_asc_evaluation_strategy);
    register_bench!(
        registry,
        None,
        benchmark_i16_to_bits_asc_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_i32_to_bits_asc_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_i64_to_bits_asc_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_isize_to_bits_asc_evaluation_strategy
    );

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

    register_bench!(
        registry,
        None,
        benchmark_u8_to_bits_desc_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u16_to_bits_desc_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u32_to_bits_desc_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u64_to_bits_desc_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_usize_to_bits_desc_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_i8_to_bits_desc_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_i16_to_bits_desc_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_i32_to_bits_desc_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_i64_to_bits_desc_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_isize_to_bits_desc_evaluation_strategy
    );

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

fn demo_unsigned_from_bits_asc<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for bits in vecs_of_bool_var_2::<T>(gm).take(limit) {
        println!(
            "{}::from_bits_asc({:?}) = {}",
            T::NAME,
            bits,
            T::from_bits_asc(&bits)
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
            T::from_bits_asc(&bits)
        );
    }
}

fn demo_unsigned_from_bits_desc<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for bits in vecs_of_bool_var_4::<T>(gm).take(limit) {
        println!(
            "{}::from_bits_desc({:?}) = {}",
            T::NAME,
            bits,
            T::from_bits_desc(&bits)
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
            T::from_bits_desc(&bits)
        );
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
        &(|u| usize::exact_from(u.significant_bits())),
        "u.significant_bits()",
        &mut [
            ("malachite", &mut (|u| no_out!(u.to_bits_asc()))),
            ("alt", &mut (|u| no_out!(to_bits_asc_alt(&u)))),
            ("naive", &mut (|u| no_out!(to_bits_asc_unsigned_naive(u)))),
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
        &(|i| usize::exact_from(i.significant_bits())),
        "i.significant_bits()",
        &mut [
            ("malachite", &mut (|i| no_out!(i.to_bits_asc()))),
            ("alt", &mut (|i| no_out!(to_bits_asc_alt(&i)))),
            ("naive", &mut (|i| no_out!(to_bits_asc_signed_naive(i)))),
        ],
    );
}

fn benchmark_unsigned_to_bits_asc_evaluation_strategy<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.to_bits_asc()", T::NAME),
        BenchmarkType::EvaluationStrategy,
        unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|u| usize::exact_from(u.significant_bits())),
        "u.significant_bits()",
        &mut [
            (
                &format!("{}.to_bits_asc()", T::NAME),
                &mut (|n| no_out!(n.to_bits_asc())),
            ),
            (
                &format!("{}.bits().collect::<Vec<bool>>()", T::NAME),
                &mut (|n| no_out!(n.bits().collect::<Vec<bool>>())),
            ),
        ],
    );
}

fn benchmark_signed_to_bits_asc_evaluation_strategy<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.to_bits_asc()", T::NAME),
        BenchmarkType::EvaluationStrategy,
        signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|u| usize::exact_from(u.significant_bits())),
        "u.significant_bits()",
        &mut [
            (
                &format!("{}.to_bits_asc()", T::NAME),
                &mut (|n| no_out!(n.to_bits_asc())),
            ),
            (
                &format!("{}.bits().collect::<Vec<bool>>()", T::NAME),
                &mut (|n| no_out!(n.bits().collect::<Vec<bool>>())),
            ),
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
        &(|u| usize::exact_from(u.significant_bits())),
        "u.significant_bits()",
        &mut [
            ("default", &mut (|u| no_out!(u.to_bits_asc()))),
            ("alt", &mut (|u| no_out!(to_bits_desc_alt(&u)))),
            ("naive", &mut (|u| no_out!(to_bits_desc_unsigned_naive(u)))),
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
        &(|i| usize::exact_from(i.significant_bits())),
        "i.significant_bits()",
        &mut [
            ("default", &mut (|i| no_out!(i.to_bits_asc()))),
            ("alt", &mut (|i| no_out!(to_bits_desc_alt(&i)))),
            ("naive", &mut (|u| no_out!(to_bits_desc_signed_naive(u)))),
        ],
    );
}

fn benchmark_unsigned_to_bits_desc_evaluation_strategy<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.to_bits_desc()", T::NAME),
        BenchmarkType::EvaluationStrategy,
        unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|u| usize::exact_from(u.significant_bits())),
        "u.significant_bits()",
        &mut [
            (
                &format!("{}.to_bits_desc()", T::NAME),
                &mut (|n| no_out!(n.to_bits_desc())),
            ),
            (
                &format!("{}.bits().rev().collect::<Vec<bool>>()", T::NAME),
                &mut (|n| no_out!(n.bits().rev().collect::<Vec<bool>>())),
            ),
        ],
    );
}

fn benchmark_signed_to_bits_desc_evaluation_strategy<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.to_bits_desc()", T::NAME),
        BenchmarkType::EvaluationStrategy,
        signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|u| usize::exact_from(u.significant_bits())),
        "u.significant_bits()",
        &mut [
            (
                &format!("{}.to_bits_desc()", T::NAME),
                &mut (|n| no_out!(n.to_bits_desc())),
            ),
            (
                &format!("{}.bits().rev().collect::<Vec<bool>>()", T::NAME),
                &mut (|n| no_out!(n.bits().rev().collect::<Vec<bool>>())),
            ),
        ],
    );
}

fn benchmark_unsigned_from_bits_asc_algorithms<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}::from_bits_asc(&[bool])", T::NAME),
        BenchmarkType::Algorithms,
        vecs_of_bool_var_2::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|bits| bits.len()),
        "bits.len()",
        &mut [
            ("default", &mut (|ref bits| no_out!(T::from_bits_asc(bits)))),
            (
                "alt",
                &mut (|ref bits| no_out!(from_bits_asc_alt::<T>(bits))),
            ),
            (
                "naive",
                &mut (|ref bits| no_out!(from_bits_asc_unsigned_naive::<T>(bits))),
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
    m_run_benchmark(
        &format!("{}::from_bits_asc(&[bool])", T::NAME),
        BenchmarkType::Algorithms,
        vecs_of_bool_var_3::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|bits| bits.len()),
        "bits.len()",
        &mut [
            ("default", &mut (|ref bits| no_out!(T::from_bits_asc(bits)))),
            (
                "alt",
                &mut (|ref bits| no_out!(from_bits_asc_alt::<T>(bits))),
            ),
            (
                "naive",
                &mut (|ref bits| no_out!(from_bits_asc_signed_naive::<T>(bits))),
            ),
        ],
    );
}

fn benchmark_unsigned_from_bits_desc_algorithms<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}::from_bits_desc(&[bool])", T::NAME),
        BenchmarkType::Algorithms,
        vecs_of_bool_var_4::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|bits| bits.len()),
        "bits.len()",
        &mut [
            (
                "malachite",
                &mut (|ref bits| no_out!(T::from_bits_desc(bits))),
            ),
            (
                "alt",
                &mut (|ref bits| no_out!(from_bits_desc_alt::<T>(bits))),
            ),
            (
                "naive",
                &mut (|ref bits| no_out!(from_bits_desc_unsigned_naive::<T>(bits))),
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
    m_run_benchmark(
        &format!("{}::from_bits_desc(&[bool])", T::NAME),
        BenchmarkType::Algorithms,
        vecs_of_bool_var_5::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|bits| bits.len()),
        "bits.len()",
        &mut [
            (
                "malachite",
                &mut (|ref bits| no_out!(T::from_bits_desc(bits))),
            ),
            (
                "alt",
                &mut (|ref bits| no_out!(from_bits_desc_alt::<T>(bits))),
            ),
            (
                "naive",
                &mut (|ref bits| no_out!(from_bits_desc_signed_naive::<T>(bits))),
            ),
        ],
    );
}

macro_rules! unsigned {
    (
        $t:ident,
        $to_bits_asc_demo_name:ident,
        $to_bits_asc_bench_name_a:ident,
        $to_bits_asc_bench_name_es:ident,
        $to_bits_desc_demo_name:ident,
        $to_bits_desc_bench_name_a:ident,
        $to_bits_desc_bench_name_es:ident,
        $from_bits_asc_demo_name:ident,
        $from_bits_asc_bench_name:ident,
        $from_bits_desc_demo_name:ident,
        $from_bits_desc_bench_name:ident,
    ) => {
        fn $to_bits_asc_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_to_bits_asc::<$t>(gm, limit);
        }

        fn $to_bits_asc_bench_name_a(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_to_bits_asc_algorithms::<$t>(gm, limit, file_name);
        }

        fn $to_bits_asc_bench_name_es(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_to_bits_asc_evaluation_strategy::<$t>(gm, limit, file_name);
        }

        fn $to_bits_desc_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_to_bits_desc::<$t>(gm, limit);
        }

        fn $to_bits_desc_bench_name_a(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_to_bits_desc_algorithms::<$t>(gm, limit, file_name);
        }

        fn $to_bits_desc_bench_name_es(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_to_bits_desc_evaluation_strategy::<$t>(gm, limit, file_name);
        }

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
        $to_bits_asc_demo_name:ident,
        $to_bits_asc_bench_name_a:ident,
        $to_bits_asc_bench_name_es:ident,
        $to_bits_desc_demo_name:ident,
        $to_bits_desc_bench_name_a:ident,
        $to_bits_desc_bench_name_es:ident,
        $from_bits_asc_demo_name:ident,
        $from_bits_asc_bench_name:ident,
        $from_bits_desc_demo_name:ident,
        $from_bits_desc_bench_name:ident,
    ) => {
        fn $to_bits_asc_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_to_bits_asc::<$t>(gm, limit);
        }

        fn $to_bits_asc_bench_name_a(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_to_bits_asc_algorithms::<$t>(gm, limit, file_name);
        }

        fn $to_bits_asc_bench_name_es(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_to_bits_asc_evaluation_strategy::<$t>(gm, limit, file_name);
        }

        fn $to_bits_desc_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_to_bits_desc::<$t>(gm, limit);
        }

        fn $to_bits_desc_bench_name_a(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_to_bits_desc_algorithms::<$t>(gm, limit, file_name);
        }

        fn $to_bits_desc_bench_name_es(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_to_bits_desc_evaluation_strategy::<$t>(gm, limit, file_name);
        }

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
    demo_u8_to_bits_asc,
    benchmark_u8_to_bits_asc_algorithms,
    benchmark_u8_to_bits_asc_evaluation_strategy,
    demo_u8_to_bits_desc,
    benchmark_u8_to_bits_desc_algorithms,
    benchmark_u8_to_bits_desc_evaluation_strategy,
    demo_u8_from_bits_asc,
    benchmark_u8_from_bits_asc_algorithms,
    demo_u8_from_bits_desc,
    benchmark_u8_from_bits_desc_algorithms,
);
unsigned!(
    u16,
    demo_u16_to_bits_asc,
    benchmark_u16_to_bits_asc_algorithms,
    benchmark_u16_to_bits_asc_evaluation_strategy,
    demo_u16_to_bits_desc,
    benchmark_u16_to_bits_desc_algorithms,
    benchmark_u16_to_bits_desc_evaluation_strategy,
    demo_u16_from_bits_asc,
    benchmark_u16_from_bits_asc_algorithms,
    demo_u16_from_bits_desc,
    benchmark_u16_from_bits_desc_algorithms,
);
unsigned!(
    u32,
    demo_u32_to_bits_asc,
    benchmark_u32_to_bits_asc_algorithms,
    benchmark_u32_to_bits_asc_evaluation_strategy,
    demo_u32_to_bits_desc,
    benchmark_u32_to_bits_desc_algorithms,
    benchmark_u32_to_bits_desc_evaluation_strategy,
    demo_u32_from_bits_asc,
    benchmark_u32_from_bits_asc_algorithms,
    demo_u32_from_bits_desc,
    benchmark_u32_from_bits_desc_algorithms,
);
unsigned!(
    u64,
    demo_u64_to_bits_asc,
    benchmark_u64_to_bits_asc_algorithms,
    benchmark_u64_to_bits_asc_evaluation_strategy,
    demo_u64_to_bits_desc,
    benchmark_u64_to_bits_desc_algorithms,
    benchmark_u64_to_bits_desc_evaluation_strategy,
    demo_u64_from_bits_asc,
    benchmark_u64_from_bits_asc_algorithms,
    demo_u64_from_bits_desc,
    benchmark_u64_from_bits_desc_algorithms,
);
unsigned!(
    usize,
    demo_usize_to_bits_asc,
    benchmark_usize_to_bits_asc_algorithms,
    benchmark_usize_to_bits_asc_evaluation_strategy,
    demo_usize_to_bits_desc,
    benchmark_usize_to_bits_desc_algorithms,
    benchmark_usize_to_bits_desc_evaluation_strategy,
    demo_usize_from_bits_asc,
    benchmark_usize_from_bits_asc_algorithms,
    demo_usize_from_bits_desc,
    benchmark_usize_from_bits_desc_algorithms,
);
signed!(
    i8,
    demo_i8_to_bits_asc,
    benchmark_i8_to_bits_asc_algorithms,
    benchmark_i8_to_bits_asc_evaluation_strategy,
    demo_i8_to_bits_desc,
    benchmark_i8_to_bits_desc_algorithms,
    benchmark_i8_to_bits_desc_evaluation_strategy,
    demo_i8_from_bits_asc,
    benchmark_i8_from_bits_asc_algorithms,
    demo_i8_from_bits_desc,
    benchmark_i8_from_bits_desc_algorithms,
);
signed!(
    i16,
    demo_i16_to_bits_asc,
    benchmark_i16_to_bits_asc_algorithms,
    benchmark_i16_to_bits_asc_evaluation_strategy,
    demo_i16_to_bits_desc,
    benchmark_i16_to_bits_desc_algorithms,
    benchmark_i16_to_bits_desc_evaluation_strategy,
    demo_i16_from_bits_asc,
    benchmark_i16_from_bits_asc_algorithms,
    demo_i16_from_bits_desc,
    benchmark_i16_from_bits_desc_algorithms,
);
signed!(
    i32,
    demo_i32_to_bits_asc,
    benchmark_i32_to_bits_asc_algorithms,
    benchmark_i32_to_bits_asc_evaluation_strategy,
    demo_i32_to_bits_desc,
    benchmark_i32_to_bits_desc_algorithms,
    benchmark_i32_to_bits_desc_evaluation_strategy,
    demo_i32_from_bits_asc,
    benchmark_i32_from_bits_asc_algorithms,
    demo_i32_from_bits_desc,
    benchmark_i32_from_bits_desc_algorithms,
);
signed!(
    i64,
    demo_i64_to_bits_asc,
    benchmark_i64_to_bits_asc_algorithms,
    benchmark_i64_to_bits_asc_evaluation_strategy,
    demo_i64_to_bits_desc,
    benchmark_i64_to_bits_desc_algorithms,
    benchmark_i64_to_bits_desc_evaluation_strategy,
    demo_i64_from_bits_asc,
    benchmark_i64_from_bits_asc_algorithms,
    demo_i64_from_bits_desc,
    benchmark_i64_from_bits_desc_algorithms,
);
signed!(
    isize,
    demo_isize_to_bits_asc,
    benchmark_isize_to_bits_asc_algorithms,
    benchmark_isize_to_bits_asc_evaluation_strategy,
    demo_isize_to_bits_desc,
    benchmark_isize_to_bits_desc_algorithms,
    benchmark_isize_to_bits_desc_evaluation_strategy,
    demo_isize_from_bits_asc,
    benchmark_isize_from_bits_asc_algorithms,
    demo_isize_from_bits_desc,
    benchmark_isize_from_bits_desc_algorithms,
);
