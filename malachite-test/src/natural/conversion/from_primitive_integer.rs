use malachite_base::named::Named;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, SaturatingFrom,
};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::Natural;
use num::BigUint;
use rand::Rand;
use rug;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{natural_signeds, signeds, unsigneds};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_from_u8);
    register_demo!(registry, demo_natural_from_u16);
    register_demo!(registry, demo_natural_from_u32);
    register_demo!(registry, demo_natural_from_u64);
    register_demo!(registry, demo_natural_from_usize);
    register_demo!(registry, demo_natural_checked_from_i8);
    register_demo!(registry, demo_natural_checked_from_i16);
    register_demo!(registry, demo_natural_checked_from_i32);
    register_demo!(registry, demo_natural_checked_from_i64);
    register_demo!(registry, demo_natural_checked_from_isize);
    register_demo!(registry, demo_natural_exact_from_i8);
    register_demo!(registry, demo_natural_exact_from_i16);
    register_demo!(registry, demo_natural_exact_from_i32);
    register_demo!(registry, demo_natural_exact_from_i64);
    register_demo!(registry, demo_natural_exact_from_isize);
    register_demo!(registry, demo_natural_saturating_from_i8);
    register_demo!(registry, demo_natural_saturating_from_i16);
    register_demo!(registry, demo_natural_saturating_from_i32);
    register_demo!(registry, demo_natural_saturating_from_i64);
    register_demo!(registry, demo_natural_saturating_from_isize);
    register_demo!(registry, demo_natural_convertible_from_i8);
    register_demo!(registry, demo_natural_convertible_from_i16);
    register_demo!(registry, demo_natural_convertible_from_i32);
    register_demo!(registry, demo_natural_convertible_from_i64);
    register_demo!(registry, demo_natural_convertible_from_isize);
    register_bench!(registry, None, benchmark_natural_from_u8);
    register_bench!(registry, None, benchmark_natural_from_u16);
    register_bench!(registry, None, benchmark_natural_from_u32);
    register_bench!(registry, None, benchmark_natural_from_u64);
    register_bench!(registry, None, benchmark_natural_from_usize);
    register_bench!(registry, None, benchmark_natural_checked_from_i8);
    register_bench!(registry, None, benchmark_natural_checked_from_i16);
    register_bench!(registry, None, benchmark_natural_checked_from_i32);
    register_bench!(registry, None, benchmark_natural_checked_from_i64);
    register_bench!(registry, None, benchmark_natural_checked_from_isize);
    register_bench!(registry, None, benchmark_natural_exact_from_i8);
    register_bench!(registry, None, benchmark_natural_exact_from_i16);
    register_bench!(registry, None, benchmark_natural_exact_from_i32);
    register_bench!(registry, None, benchmark_natural_exact_from_i64);
    register_bench!(registry, None, benchmark_natural_exact_from_isize);
    register_bench!(registry, None, benchmark_natural_saturating_from_i8);
    register_bench!(registry, None, benchmark_natural_saturating_from_i16);
    register_bench!(registry, None, benchmark_natural_saturating_from_i32);
    register_bench!(registry, None, benchmark_natural_saturating_from_i64);
    register_bench!(registry, None, benchmark_natural_saturating_from_isize);
    register_bench!(registry, None, benchmark_natural_convertible_from_i8);
    register_bench!(registry, None, benchmark_natural_convertible_from_i16);
    register_bench!(registry, None, benchmark_natural_convertible_from_i32);
    register_bench!(registry, None, benchmark_natural_convertible_from_i64);
    register_bench!(registry, None, benchmark_natural_convertible_from_isize);
    register_bench!(
        registry,
        None,
        benchmark_natural_from_u32_library_comparison
    );
    register_bench!(
        registry,
        None,
        benchmark_natural_from_u64_library_comparison
    );
}

fn demo_natural_from_unsigned<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize)
where
    Natural: From<T>,
{
    for u in unsigneds::<T>(gm).take(limit) {
        println!("Natural::from({}) = {}", u, Natural::from(u));
    }
}

fn benchmark_natural_from_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    Natural: From<T>,
{
    m_run_benchmark(
        &format!("Natural::from({})", T::NAME),
        BenchmarkType::Single,
        unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&u| usize::exact_from(u.significant_bits())),
        "u.significant_bits()",
        &mut [("malachite", &mut (|u| no_out!(Natural::from(u))))],
    );
}

macro_rules! demo_and_bench_unsigned {
    ($t:ident, $demo_name:ident, $bench_name:ident) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_natural_from_unsigned::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_natural_from_unsigned::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! demo_and_bench_signed {
    (
        $t:ident,
        $checked_from_demo_name:ident,
        $exact_from_demo_name:ident,
        $saturating_from_demo_name:ident,
        $convertible_from_demo_name:ident,
        $checked_from_bench_name:ident,
        $exact_from_bench_name:ident,
        $saturating_from_bench_name:ident,
        $convertible_from_bench_name:ident
    ) => {
        fn $checked_from_demo_name(gm: GenerationMode, limit: usize) {
            for i in signeds::<$t>(gm).take(limit) {
                println!(
                    "Natural::checked_from({}) = {:?}",
                    i,
                    Natural::checked_from(i)
                );
            }
        }

        fn $exact_from_demo_name(gm: GenerationMode, limit: usize) {
            for i in natural_signeds::<$t>(gm).take(limit) {
                println!("Natural::exact_from({}) = {}", i, Natural::exact_from(i));
            }
        }

        fn $saturating_from_demo_name(gm: GenerationMode, limit: usize) {
            for i in signeds::<$t>(gm).take(limit) {
                println!(
                    "Natural::saturating_from({}) = {}",
                    i,
                    Natural::saturating_from(i)
                );
            }
        }

        fn $convertible_from_demo_name(gm: GenerationMode, limit: usize) {
            for i in signeds::<$t>(gm).take(limit) {
                println!(
                    "{} is {}convertible to a Limb",
                    i,
                    if Natural::convertible_from(i) {
                        ""
                    } else {
                        "not "
                    },
                );
            }
        }

        fn $checked_from_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            m_run_benchmark(
                &format!("Natural::checked_from({})", $t::NAME),
                BenchmarkType::Single,
                signeds::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&i| usize::exact_from(i.significant_bits())),
                "i.significant_bits()",
                &mut [("malachite", &mut (|i| no_out!(Natural::checked_from(i))))],
            );
        }

        fn $exact_from_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            m_run_benchmark(
                &format!("Natural::exact_from({})", $t::NAME),
                BenchmarkType::Single,
                signeds::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&i| usize::exact_from(i.significant_bits())),
                "i.significant_bits()",
                &mut [("malachite", &mut (|i| no_out!(Natural::exact_from(i))))],
            );
        }

        fn $saturating_from_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            m_run_benchmark(
                &format!("Natural::saturating_from({})", $t::NAME),
                BenchmarkType::Single,
                signeds::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&i| usize::exact_from(i.significant_bits())),
                "i.significant_bits()",
                &mut [("malachite", &mut (|i| no_out!(Natural::saturating_from(i))))],
            );
        }

        fn $convertible_from_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            m_run_benchmark(
                &format!("Natural::convertible_from({})", $t::NAME),
                BenchmarkType::Single,
                signeds::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|i| usize::exact_from(i.significant_bits())),
                "i.significant_bits()",
                &mut [(
                    "malachite",
                    &mut (|i| no_out!(Natural::convertible_from(i))),
                )],
            );
        }
    };
}

demo_and_bench_unsigned!(u8, demo_natural_from_u8, benchmark_natural_from_u8);
demo_and_bench_unsigned!(u16, demo_natural_from_u16, benchmark_natural_from_u16);
demo_and_bench_unsigned!(u32, demo_natural_from_u32, benchmark_natural_from_u32);
demo_and_bench_unsigned!(u64, demo_natural_from_u64, benchmark_natural_from_u64);
demo_and_bench_unsigned!(usize, demo_natural_from_usize, benchmark_natural_from_usize);

demo_and_bench_signed!(
    i8,
    demo_natural_checked_from_i8,
    demo_natural_exact_from_i8,
    demo_natural_saturating_from_i8,
    demo_natural_convertible_from_i8,
    benchmark_natural_checked_from_i8,
    benchmark_natural_exact_from_i8,
    benchmark_natural_saturating_from_i8,
    benchmark_natural_convertible_from_i8
);
demo_and_bench_signed!(
    i16,
    demo_natural_checked_from_i16,
    demo_natural_exact_from_i16,
    demo_natural_saturating_from_i16,
    demo_natural_convertible_from_i16,
    benchmark_natural_checked_from_i16,
    benchmark_natural_exact_from_i16,
    benchmark_natural_saturating_from_i16,
    benchmark_natural_convertible_from_i16
);
demo_and_bench_signed!(
    i32,
    demo_natural_checked_from_i32,
    demo_natural_exact_from_i32,
    demo_natural_saturating_from_i32,
    demo_natural_convertible_from_i32,
    benchmark_natural_checked_from_i32,
    benchmark_natural_exact_from_i32,
    benchmark_natural_saturating_from_i32,
    benchmark_natural_convertible_from_i32
);
demo_and_bench_signed!(
    i64,
    demo_natural_checked_from_i64,
    demo_natural_exact_from_i64,
    demo_natural_saturating_from_i64,
    demo_natural_convertible_from_i64,
    benchmark_natural_checked_from_i64,
    benchmark_natural_exact_from_i64,
    benchmark_natural_saturating_from_i64,
    benchmark_natural_convertible_from_i64
);
demo_and_bench_signed!(
    isize,
    demo_natural_checked_from_isize,
    demo_natural_exact_from_isize,
    demo_natural_saturating_from_isize,
    demo_natural_convertible_from_isize,
    benchmark_natural_checked_from_isize,
    benchmark_natural_exact_from_isize,
    benchmark_natural_saturating_from_isize,
    benchmark_natural_convertible_from_isize
);

fn benchmark_natural_from_u32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural::from(u32)",
        BenchmarkType::LibraryComparison,
        unsigneds::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&u| usize::exact_from(u.significant_bits())),
        "u.significant_bits()",
        &mut [
            ("malachite", &mut (|u| no_out!(Natural::from(u)))),
            ("num", &mut (|u| no_out!(BigUint::from(u)))),
            ("rug", &mut (|u| no_out!(rug::Integer::from(u)))),
        ],
    );
}

fn benchmark_natural_from_u64_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural::from(u64)",
        BenchmarkType::LibraryComparison,
        unsigneds::<u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&u| usize::exact_from(u.significant_bits())),
        "u.significant_bits()",
        &mut [
            ("malachite", &mut (|u| no_out!(Natural::from(u)))),
            ("num", &mut (|u| no_out!(BigUint::from(u)))),
            ("rug", &mut (|u| no_out!(rug::Integer::from(u)))),
        ],
    );
}
