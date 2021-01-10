use itertools::Itertools;
use malachite_base::named::Named;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{
    ExactFrom, PowerOfTwoDigitIterable, PowerOfTwoDigitIterator, PowerOfTwoDigits,
};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_nz::natural::Natural;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::natural::{
    pairs_of_natural_and_small_u64_var_3, pairs_of_natural_and_small_unsigned_var_3,
    triples_of_natural_small_u64_and_small_u64_var_2,
    triples_of_natural_small_u64_and_small_u64_var_3,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_power_of_two_digits_u8);
    register_demo!(registry, demo_natural_power_of_two_digits_u16);
    register_demo!(registry, demo_natural_power_of_two_digits_u32);
    register_demo!(registry, demo_natural_power_of_two_digits_u64);
    register_demo!(registry, demo_natural_power_of_two_digits_u128);
    register_demo!(registry, demo_natural_power_of_two_digits_usize);

    register_demo!(registry, demo_natural_power_of_two_digits_rev_u8);
    register_demo!(registry, demo_natural_power_of_two_digits_rev_u16);
    register_demo!(registry, demo_natural_power_of_two_digits_rev_u32);
    register_demo!(registry, demo_natural_power_of_two_digits_rev_u64);
    register_demo!(registry, demo_natural_power_of_two_digits_rev_u128);
    register_demo!(registry, demo_natural_power_of_two_digits_rev_usize);

    register_demo!(registry, demo_natural_power_of_two_digits_size_hint_u8);
    register_demo!(registry, demo_natural_power_of_two_digits_size_hint_u16);
    register_demo!(registry, demo_natural_power_of_two_digits_size_hint_u32);
    register_demo!(registry, demo_natural_power_of_two_digits_size_hint_u64);
    register_demo!(registry, demo_natural_power_of_two_digits_size_hint_u128);
    register_demo!(registry, demo_natural_power_of_two_digits_size_hint_usize);

    register_demo!(registry, demo_natural_power_of_two_digits_get_u8);
    register_demo!(registry, demo_natural_power_of_two_digits_get_u16);
    register_demo!(registry, demo_natural_power_of_two_digits_get_u32);
    register_demo!(registry, demo_natural_power_of_two_digits_get_u64);
    register_demo!(registry, demo_natural_power_of_two_digits_get_u128);
    register_demo!(registry, demo_natural_power_of_two_digits_get_usize);

    register_demo!(registry, demo_natural_power_of_two_digits_natural);
    register_demo!(registry, demo_natural_power_of_two_digits_rev_natural);
    register_demo!(registry, demo_natural_power_of_two_digits_size_hint_natural);
    register_demo!(registry, demo_natural_power_of_two_digits_get_natural);

    register_bench!(
        registry,
        Large,
        benchmark_natural_power_of_two_digits_size_hint_u8
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_power_of_two_digits_size_hint_u16
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_power_of_two_digits_size_hint_u32
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_power_of_two_digits_size_hint_u64
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_power_of_two_digits_size_hint_u128
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_power_of_two_digits_size_hint_usize
    );

    register_bench!(
        registry,
        Large,
        benchmark_natural_power_of_two_digits_get_algorithms_u8
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_power_of_two_digits_get_algorithms_u16
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_power_of_two_digits_get_algorithms_u32
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_power_of_two_digits_get_algorithms_u64
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_power_of_two_digits_get_algorithms_u128
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_power_of_two_digits_get_algorithms_usize
    );

    register_bench!(
        registry,
        Large,
        benchmark_natural_power_of_two_digits_size_hint_natural
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_power_of_two_digits_get_natural
    );
}

macro_rules! demo_and_bench {
    (
        $t:ident,
        $power_of_two_digits_demo_name:ident,
        $power_of_two_digits_rev_demo_name:ident,
        $power_of_two_digits_size_hint_demo_name:ident,
        $power_of_two_digits_get_demo_name:ident,
        $power_of_two_digits_size_hint_bench_name:ident,
        $power_of_two_digits_get_bench_name:ident
    ) => {
        fn $power_of_two_digits_demo_name(gm: GenerationMode, limit: usize) {
            for (n, log_base) in pairs_of_natural_and_small_u64_var_3::<$t>(gm).take(limit) {
                println!(
                    "power_of_two_digits({}, {}) = {:?}",
                    n,
                    log_base,
                    PowerOfTwoDigitIterable::<$t>::power_of_two_digits(&n, log_base).collect_vec()
                );
            }
        }

        fn $power_of_two_digits_rev_demo_name(gm: GenerationMode, limit: usize) {
            for (n, log_base) in pairs_of_natural_and_small_u64_var_3::<$t>(gm).take(limit) {
                println!(
                    "power_of_two_digits({}, {}).rev() = {:?}",
                    n,
                    log_base,
                    PowerOfTwoDigitIterable::<$t>::power_of_two_digits(&n, log_base)
                        .rev()
                        .collect_vec()
                );
            }
        }

        fn $power_of_two_digits_size_hint_demo_name(gm: GenerationMode, limit: usize) {
            for (n, log_base) in pairs_of_natural_and_small_u64_var_3::<$t>(gm).take(limit) {
                println!(
                    "power_of_two_digits({}, {}).size_hint() = {:?}",
                    n,
                    log_base,
                    PowerOfTwoDigitIterable::<$t>::power_of_two_digits(&n, log_base).size_hint()
                );
            }
        }

        fn $power_of_two_digits_get_demo_name(gm: GenerationMode, limit: usize) {
            for (n, log_base, i) in
                triples_of_natural_small_u64_and_small_u64_var_2::<$t>(gm).take(limit)
            {
                println!(
                    "power_of_two_digits({}, {}).get({}) = {:?}",
                    n,
                    log_base,
                    i,
                    PowerOfTwoDigitIterable::<$t>::power_of_two_digits(&n, log_base).get(i)
                );
            }
        }

        fn $power_of_two_digits_size_hint_bench_name(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            run_benchmark_old(
                &format!(
                    "PowerOfTwoDigitIterable::<{}>::power_of_two_digits(&Natural, u64).size_hint()",
                    $t::NAME
                ),
                BenchmarkType::Single,
                pairs_of_natural_and_small_u64_var_3::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|(n, _)| usize::exact_from(n.significant_bits())),
                "n.significant_bits()",
                &mut [(
                    &format!(
                        "PowerOfTwoDigitIterable::<{}>::power_of_two_digits(&Natural, u64)\
                        .size_hint()",
                        $t::NAME
                    ),
                    &mut (|(n, log_base)| {
                        no_out!(
                            PowerOfTwoDigitIterable::<$t>::power_of_two_digits(&n, log_base)
                                .size_hint()
                        )
                    }),
                )],
            );
        }

        fn $power_of_two_digits_get_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            run_benchmark_old(
                &format!(
                    "PowerOfTwoDigitIterable::<{}>::power_of_two_digits(&Natural, u64).get(u64)",
                    $t::NAME
                ),
                BenchmarkType::Algorithms,
                triples_of_natural_small_u64_and_small_u64_var_2::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(ref n, _, _)| usize::exact_from(n.significant_bits())),
                "n.significant_bits()",
                &mut [
                    (
                        "power_of_two_digits(&Natural, u64).get(u64)",
                        &mut (|(n, log_base, i)| {
                            no_out!(PowerOfTwoDigitIterable::<$t>::power_of_two_digits(
                                &n, log_base
                            )
                            .get(i))
                        }),
                    ),
                    (
                        "Natural.to_power_of_two_digits_asc(u64)[u64]",
                        &mut (|(n, log_base, i)| {
                            let digits =
                                PowerOfTwoDigits::<$t>::to_power_of_two_digits_asc(&n, log_base);
                            let i = usize::exact_from(i);
                            if i >= digits.len() {
                                0
                            } else {
                                digits[i]
                            };
                        }),
                    ),
                ],
            );
        }
    };
}

demo_and_bench!(
    u8,
    demo_natural_power_of_two_digits_u8,
    demo_natural_power_of_two_digits_rev_u8,
    demo_natural_power_of_two_digits_size_hint_u8,
    demo_natural_power_of_two_digits_get_u8,
    benchmark_natural_power_of_two_digits_size_hint_u8,
    benchmark_natural_power_of_two_digits_get_algorithms_u8
);
demo_and_bench!(
    u16,
    demo_natural_power_of_two_digits_u16,
    demo_natural_power_of_two_digits_rev_u16,
    demo_natural_power_of_two_digits_size_hint_u16,
    demo_natural_power_of_two_digits_get_u16,
    benchmark_natural_power_of_two_digits_size_hint_u16,
    benchmark_natural_power_of_two_digits_get_algorithms_u16
);
demo_and_bench!(
    u32,
    demo_natural_power_of_two_digits_u32,
    demo_natural_power_of_two_digits_rev_u32,
    demo_natural_power_of_two_digits_size_hint_u32,
    demo_natural_power_of_two_digits_get_u32,
    benchmark_natural_power_of_two_digits_size_hint_u32,
    benchmark_natural_power_of_two_digits_get_algorithms_u32
);
demo_and_bench!(
    u64,
    demo_natural_power_of_two_digits_u64,
    demo_natural_power_of_two_digits_rev_u64,
    demo_natural_power_of_two_digits_size_hint_u64,
    demo_natural_power_of_two_digits_get_u64,
    benchmark_natural_power_of_two_digits_size_hint_u64,
    benchmark_natural_power_of_two_digits_get_algorithms_u64
);
demo_and_bench!(
    u128,
    demo_natural_power_of_two_digits_u128,
    demo_natural_power_of_two_digits_rev_u128,
    demo_natural_power_of_two_digits_size_hint_u128,
    demo_natural_power_of_two_digits_get_u128,
    benchmark_natural_power_of_two_digits_size_hint_u128,
    benchmark_natural_power_of_two_digits_get_algorithms_u128
);
demo_and_bench!(
    usize,
    demo_natural_power_of_two_digits_usize,
    demo_natural_power_of_two_digits_rev_usize,
    demo_natural_power_of_two_digits_size_hint_usize,
    demo_natural_power_of_two_digits_get_usize,
    benchmark_natural_power_of_two_digits_size_hint_usize,
    benchmark_natural_power_of_two_digits_get_algorithms_usize
);

fn demo_natural_power_of_two_digits_natural(gm: GenerationMode, limit: usize) {
    for (n, log_base) in pairs_of_natural_and_small_unsigned_var_3(gm).take(limit) {
        println!(
            "power_of_two_digits({}, {}) = {:?}",
            n,
            log_base,
            PowerOfTwoDigitIterable::<Natural>::power_of_two_digits(&n, log_base).collect_vec()
        );
    }
}

fn demo_natural_power_of_two_digits_rev_natural(gm: GenerationMode, limit: usize) {
    for (n, log_base) in pairs_of_natural_and_small_unsigned_var_3(gm).take(limit) {
        println!(
            "power_of_two_digits({}, {}).rev() = {:?}",
            n,
            log_base,
            PowerOfTwoDigitIterable::<Natural>::power_of_two_digits(&n, log_base)
                .rev()
                .collect_vec()
        );
    }
}

fn demo_natural_power_of_two_digits_size_hint_natural(gm: GenerationMode, limit: usize) {
    for (n, log_base) in pairs_of_natural_and_small_unsigned_var_3(gm).take(limit) {
        println!(
            "power_of_two_digits({}, {}).size_hint() = {:?}",
            n,
            log_base,
            PowerOfTwoDigitIterable::<Natural>::power_of_two_digits(&n, log_base).size_hint()
        );
    }
}

fn demo_natural_power_of_two_digits_get_natural(gm: GenerationMode, limit: usize) {
    for (n, log_base, i) in triples_of_natural_small_u64_and_small_u64_var_3(gm).take(limit) {
        println!(
            "power_of_two_digits({}, {}).get({}) = {:?}",
            n,
            log_base,
            i,
            PowerOfTwoDigitIterable::<Natural>::power_of_two_digits(&n, log_base).get(i)
        );
    }
}

fn benchmark_natural_power_of_two_digits_size_hint_natural(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "PowerOfTwoDigitIterable::<Natural>::power_of_two_digits(&Natural, u64).size_hint()",
        BenchmarkType::Single,
        pairs_of_natural_and_small_unsigned_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|(n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [(
            "PowerOfTwoDigitIterable::<Natural>::power_of_two_digits(&Natural, u64).size_hint()",
            &mut (|(n, log_base)| {
                no_out!(
                    PowerOfTwoDigitIterable::<Natural>::power_of_two_digits(&n, log_base)
                        .size_hint()
                )
            }),
        )],
    );
}

#[allow(path_statements)]
fn benchmark_natural_power_of_two_digits_get_natural(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "PowerOfTwoDigitIterable::<Natural>::power_of_two_digits(&Natural, u64).get(u64)",
        BenchmarkType::Algorithms,
        triples_of_natural_small_u64_and_small_u64_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "power_of_two_digits(&Natural, u64).get(u64)",
                &mut (|(n, log_base, i)| {
                    no_out!(
                        PowerOfTwoDigitIterable::<Natural>::power_of_two_digits(&n, log_base)
                            .get(i)
                    )
                }),
            ),
            (
                "Natural.to_power_of_two_digits_asc(u64)[u64]",
                &mut (|(n, log_base, i)| {
                    let digits =
                        PowerOfTwoDigits::<Natural>::to_power_of_two_digits_asc(&n, log_base);
                    let i = usize::exact_from(i);
                    if i >= digits.len() {
                        Natural::ZERO;
                    } else {
                        let _ = digits[i];
                    }
                }),
            ),
        ],
    );
}
