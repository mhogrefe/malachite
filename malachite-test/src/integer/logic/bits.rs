use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitConvertible, BitIterable, SignificantBits};

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{integers, pairs_of_integer_and_small_unsigned};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_bits);
    register_demo!(registry, demo_integer_bits_rev);
    register_demo!(registry, demo_integer_bits_index);
    register_bench!(registry, Large, benchmark_integer_bits_get_algorithms);
}

fn demo_integer_bits(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("bits({}) = {:?}", n, n.bits().collect::<Vec<bool>>());
    }
}

fn demo_integer_bits_rev(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "bits({}).rev() = {:?}",
            n,
            n.bits().rev().collect::<Vec<bool>>()
        );
    }
}

fn demo_integer_bits_index(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_small_unsigned(gm).take(limit) {
        println!("bits({})[{}] = {:?}", n, i, n.bits()[i]);
    }
}

fn benchmark_integer_bits_get_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.bits()[u64]",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("Integer.bits()[u]", &mut (|(n, u)| no_out!(n.bits()[u]))),
            (
                "Integer.to_bits_asc()[u]",
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
