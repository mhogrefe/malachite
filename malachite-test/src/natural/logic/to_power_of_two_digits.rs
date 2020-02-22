use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{PowerOfTwoDigits, SignificantBits};
use malachite_nz::natural::Natural;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::{
    pairs_of_natural_and_small_u64_var_3, pairs_of_natural_and_small_unsigned_var_3,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_to_power_of_two_digits_asc_u8);
    register_demo!(registry, demo_natural_to_power_of_two_digits_asc_u16);
    register_demo!(registry, demo_natural_to_power_of_two_digits_asc_u32);
    register_demo!(registry, demo_natural_to_power_of_two_digits_asc_u64);
    register_demo!(registry, demo_natural_to_power_of_two_digits_asc_u128);
    register_demo!(registry, demo_natural_to_power_of_two_digits_asc_usize);

    register_demo!(registry, demo_natural_to_power_of_two_digits_desc_u8);
    register_demo!(registry, demo_natural_to_power_of_two_digits_desc_u16);
    register_demo!(registry, demo_natural_to_power_of_two_digits_desc_u32);
    register_demo!(registry, demo_natural_to_power_of_two_digits_desc_u64);
    register_demo!(registry, demo_natural_to_power_of_two_digits_desc_u128);
    register_demo!(registry, demo_natural_to_power_of_two_digits_desc_usize);

    register_demo!(registry, demo_natural_to_power_of_two_digits_asc_natural);
    register_demo!(registry, demo_natural_to_power_of_two_digits_desc_natural);

    register_bench!(
        registry,
        Small,
        benchmark_natural_to_power_of_two_digits_asc_u8_algorithms
    );
    register_bench!(
        registry,
        Small,
        benchmark_natural_to_power_of_two_digits_asc_u16_algorithms
    );
    register_bench!(
        registry,
        Small,
        benchmark_natural_to_power_of_two_digits_asc_u32_algorithms
    );
    register_bench!(
        registry,
        Small,
        benchmark_natural_to_power_of_two_digits_asc_u64_algorithms
    );
    register_bench!(
        registry,
        Small,
        benchmark_natural_to_power_of_two_digits_asc_u128_algorithms
    );
    register_bench!(
        registry,
        Small,
        benchmark_natural_to_power_of_two_digits_asc_usize_algorithms
    );

    register_bench!(
        registry,
        Small,
        benchmark_natural_to_power_of_two_digits_desc_u8
    );
    register_bench!(
        registry,
        Small,
        benchmark_natural_to_power_of_two_digits_desc_u16
    );
    register_bench!(
        registry,
        Small,
        benchmark_natural_to_power_of_two_digits_desc_u32
    );
    register_bench!(
        registry,
        Small,
        benchmark_natural_to_power_of_two_digits_desc_u64
    );
    register_bench!(
        registry,
        Small,
        benchmark_natural_to_power_of_two_digits_desc_u128
    );
    register_bench!(
        registry,
        Small,
        benchmark_natural_to_power_of_two_digits_desc_usize
    );

    register_bench!(
        registry,
        Small,
        benchmark_natural_to_power_of_two_digits_asc_natural_algorithms
    );
    register_bench!(
        registry,
        Small,
        benchmark_natural_to_power_of_two_digits_desc_natural
    );
}

fn demo_to_power_of_two_digits_asc<T: PrimitiveUnsigned>(gm: GenerationMode, limit: usize)
where
    Natural: PowerOfTwoDigits<T>,
{
    for (n, log_base) in pairs_of_natural_and_small_u64_var_3::<T>(gm).take(limit) {
        println!(
            "{}.to_power_of_two_digits_asc({}) = {:?}",
            n,
            log_base,
            PowerOfTwoDigits::<T>::to_power_of_two_digits_asc(&n, log_base)
        );
    }
}

fn demo_to_power_of_two_digits_desc<T: PrimitiveUnsigned>(gm: GenerationMode, limit: usize)
where
    Natural: PowerOfTwoDigits<T>,
{
    for (n, log_base) in pairs_of_natural_and_small_u64_var_3::<T>(gm).take(limit) {
        println!(
            "{}.to_power_of_two_digits_desc({}) = {:?}",
            n,
            log_base,
            PowerOfTwoDigits::<T>::to_power_of_two_digits_desc(&n, log_base)
        );
    }
}

fn demo_natural_to_power_of_two_digits_asc_natural(gm: GenerationMode, limit: usize) {
    for (n, log_base) in pairs_of_natural_and_small_unsigned_var_3(gm).take(limit) {
        println!(
            "{}.to_power_of_two_digits_asc({}) = {:?}",
            n,
            log_base,
            PowerOfTwoDigits::<Natural>::to_power_of_two_digits_asc(&n, log_base)
        );
    }
}

fn demo_natural_to_power_of_two_digits_desc_natural(gm: GenerationMode, limit: usize) {
    for (n, log_base) in pairs_of_natural_and_small_unsigned_var_3(gm).take(limit) {
        println!(
            "{}.to_power_of_two_digits_desc({}) = {:?}",
            n,
            log_base,
            PowerOfTwoDigits::<Natural>::to_power_of_two_digits_desc(&n, log_base)
        );
    }
}

fn benchmark_to_power_of_two_digits_asc_algorithms<
    T: PrimitiveUnsigned,
    F: Fn(&Natural, u64) -> Vec<T>,
>(
    to_power_of_two_digits_asc_naive: F,
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    Natural: PowerOfTwoDigits<T>,
{
    m_run_benchmark(
        &format!(
            "PowerOfTwoDigits::<{}>::to_power_of_two_digits_asc(&Natural, u64)",
            T::NAME
        ),
        BenchmarkType::Algorithms,
        pairs_of_natural_and_small_u64_var_3::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "default",
                &mut (|(n, log_base)| {
                    no_out!(PowerOfTwoDigits::<T>::to_power_of_two_digits_asc(
                        &n, log_base
                    ))
                }),
            ),
            (
                "naive",
                &mut (|(n, log_base)| no_out!(to_power_of_two_digits_asc_naive(&n, log_base))),
            ),
        ],
    );
}

fn benchmark_to_power_of_two_digits_desc<T: PrimitiveUnsigned>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    Natural: PowerOfTwoDigits<T>,
{
    m_run_benchmark(
        &format!(
            "PowerOfTwoDigits::<{}>::to_power_of_two_digits_desc(&Natural, u64)",
            T::NAME
        ),
        BenchmarkType::Single,
        pairs_of_natural_and_small_u64_var_3::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [(
            "malachite",
            &mut (|(n, log_base)| {
                no_out!(PowerOfTwoDigits::<T>::to_power_of_two_digits_desc(
                    &n, log_base
                ))
            }),
        )],
    );
}

fn benchmark_natural_to_power_of_two_digits_asc_natural_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "PowerOfTwoDigits::<Natural>::to_power_of_two_digits_asc(&Natural, u64)",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_small_unsigned_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|(n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "default",
                &mut (|(n, log_base)| {
                    no_out!(PowerOfTwoDigits::<Natural>::to_power_of_two_digits_asc(
                        &n, log_base
                    ))
                }),
            ),
            (
                "naive",
                &mut (|(n, log_base)| {
                    no_out!(n._to_power_of_two_digits_asc_natural_naive(log_base))
                }),
            ),
        ],
    );
}

fn benchmark_natural_to_power_of_two_digits_desc_natural(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "PowerOfTwoDigits::<Natural>::to_power_of_two_digits_desc(&Natural, u64)",
        BenchmarkType::Single,
        pairs_of_natural_and_small_unsigned_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|(n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [(
            "malachite",
            &mut (|(n, log_base)| {
                no_out!(PowerOfTwoDigits::<Natural>::to_power_of_two_digits_desc(
                    &n, log_base
                ))
            }),
        )],
    );
}

macro_rules! demo_and_bench {
    (
        $t:ident,
        $to_power_of_two_digits_asc_naive:ident,
        $to_power_of_two_digits_asc_demo_name:ident,
        $to_power_of_two_digits_desc_demo_name:ident,
        $to_power_of_two_digits_asc_bench_name:ident,
        $to_power_of_two_digits_desc_bench_name:ident
    ) => {
        fn $to_power_of_two_digits_asc_demo_name(gm: GenerationMode, limit: usize) {
            demo_to_power_of_two_digits_asc::<$t>(gm, limit);
        }

        fn $to_power_of_two_digits_desc_demo_name(gm: GenerationMode, limit: usize) {
            demo_to_power_of_two_digits_desc::<$t>(gm, limit);
        }

        fn $to_power_of_two_digits_asc_bench_name(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            benchmark_to_power_of_two_digits_asc_algorithms::<$t, _>(
                Natural::$to_power_of_two_digits_asc_naive,
                gm,
                limit,
                file_name,
            );
        }

        fn $to_power_of_two_digits_desc_bench_name(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            benchmark_to_power_of_two_digits_desc::<$t>(gm, limit, file_name);
        }
    };
}

demo_and_bench!(
    u8,
    _to_power_of_two_digits_asc_u8_naive,
    demo_natural_to_power_of_two_digits_asc_u8,
    demo_natural_to_power_of_two_digits_desc_u8,
    benchmark_natural_to_power_of_two_digits_asc_u8_algorithms,
    benchmark_natural_to_power_of_two_digits_desc_u8
);
demo_and_bench!(
    u16,
    _to_power_of_two_digits_asc_u16_naive,
    demo_natural_to_power_of_two_digits_asc_u16,
    demo_natural_to_power_of_two_digits_desc_u16,
    benchmark_natural_to_power_of_two_digits_asc_u16_algorithms,
    benchmark_natural_to_power_of_two_digits_desc_u16
);
demo_and_bench!(
    u32,
    _to_power_of_two_digits_asc_u32_naive,
    demo_natural_to_power_of_two_digits_asc_u32,
    demo_natural_to_power_of_two_digits_desc_u32,
    benchmark_natural_to_power_of_two_digits_asc_u32_algorithms,
    benchmark_natural_to_power_of_two_digits_desc_u32
);
demo_and_bench!(
    u64,
    _to_power_of_two_digits_asc_u64_naive,
    demo_natural_to_power_of_two_digits_asc_u64,
    demo_natural_to_power_of_two_digits_desc_u64,
    benchmark_natural_to_power_of_two_digits_asc_u64_algorithms,
    benchmark_natural_to_power_of_two_digits_desc_u64
);
demo_and_bench!(
    u128,
    _to_power_of_two_digits_asc_u128_naive,
    demo_natural_to_power_of_two_digits_asc_u128,
    demo_natural_to_power_of_two_digits_desc_u128,
    benchmark_natural_to_power_of_two_digits_asc_u128_algorithms,
    benchmark_natural_to_power_of_two_digits_desc_u128
);
demo_and_bench!(
    usize,
    _to_power_of_two_digits_asc_usize_naive,
    demo_natural_to_power_of_two_digits_asc_usize,
    demo_natural_to_power_of_two_digits_desc_usize,
    benchmark_natural_to_power_of_two_digits_asc_usize_algorithms,
    benchmark_natural_to_power_of_two_digits_desc_usize
);
