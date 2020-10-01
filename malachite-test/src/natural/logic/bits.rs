use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitConvertible, BitIterable, SignificantBits};
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::natural::{naturals, pairs_of_natural_and_small_unsigned};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_bits);
    register_demo!(registry, demo_natural_bits_rev);
    register_demo!(registry, demo_natural_bits_size_hint);
    register_bench!(registry, Large, benchmark_natural_bits_size_hint);
    register_bench!(registry, Large, benchmark_natural_bits_get_algorithms);
}

fn demo_natural_bits(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("bits({}) = {:?}", n, n.bits().collect::<Vec<bool>>());
    }
}

fn demo_natural_bits_rev(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!(
            "bits({}).rev() = {:?}",
            n,
            n.bits().rev().collect::<Vec<bool>>()
        );
    }
}

fn demo_natural_bits_size_hint(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("bits({}).size_hint() = {:?}", n, n.bits().size_hint());
    }
}

fn benchmark_natural_bits_get_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural.bits()[u64]",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("Natural.bits()[u]", &mut (|(n, u)| no_out!(n.bits()[u]))),
            (
                "Natural.to_bits_asc()[u]",
                &mut (|(n, u)| {
                    let bits = n.to_bits_asc();
                    let u = usize::exact_from(u);
                    if u >= bits.len() {
                        n < 0
                    } else {
                        bits[u]
                    };
                }),
            ),
        ],
    );
}

fn benchmark_natural_bits_size_hint(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural.bits().size_hint()",
        BenchmarkType::Single,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [(
            "Natural.bits().size_hint()",
            &mut (|n| no_out!(n.bits().size_hint())),
        )],
    );
}
