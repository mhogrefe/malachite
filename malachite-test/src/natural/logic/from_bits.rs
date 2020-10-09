use malachite_base::num::logic::traits::BitConvertible;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_base_test_util::num::logic::bit_convertible::{
    from_bits_asc_alt, from_bits_desc_alt,
};
use malachite_nz::natural::Natural;
use malachite_nz_test_util::natural::logic::from_bits::{
    from_bits_asc_naive, from_bits_desc_naive,
};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::vecs_of_bool;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_from_bits_asc);
    register_demo!(registry, demo_natural_from_bits_desc);
    register_bench!(registry, Large, benchmark_natural_from_bits_asc_algorithms);
    register_bench!(registry, Large, benchmark_natural_from_bits_desc_algorithms);
}

fn demo_natural_from_bits_asc(gm: GenerationMode, limit: usize) {
    for bits in vecs_of_bool(gm).take(limit) {
        println!(
            "from_bits_asc({:?}) = {:?}",
            bits,
            Natural::from_bits_asc(bits.iter().cloned())
        );
    }
}

fn demo_natural_from_bits_desc(gm: GenerationMode, limit: usize) {
    for bits in vecs_of_bool(gm).take(limit) {
        println!(
            "from_bits_desc({:?}) = {:?}",
            bits,
            Natural::from_bits_desc(bits.iter().cloned())
        );
    }
}

fn benchmark_natural_from_bits_asc_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural::from_bits_asc<I: Iterator<Item=bool>>(I)",
        BenchmarkType::Algorithms,
        vecs_of_bool(gm),
        gm.name(),
        limit,
        file_name,
        &(|bits| bits.len()),
        "bits.len()",
        &mut [
            (
                "default",
                &mut (|ref bits| no_out!(Natural::from_bits_asc(bits.iter().cloned()))),
            ),
            (
                "alt",
                &mut (|ref bits| no_out!(from_bits_asc_alt::<Natural, _>(bits.iter().cloned()))),
            ),
            (
                "naive",
                &mut (|ref bits| no_out!(from_bits_asc_naive(bits.iter().cloned()))),
            ),
        ],
    );
}

fn benchmark_natural_from_bits_desc_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural::from_bits_desc<I: Iterator<Item=bool>>(I)",
        BenchmarkType::Algorithms,
        vecs_of_bool(gm),
        gm.name(),
        limit,
        file_name,
        &(|bits| bits.len()),
        "bits.len()",
        &mut [
            (
                "default",
                &mut (|ref bits| no_out!(Natural::from_bits_desc(bits.iter().cloned()))),
            ),
            (
                "alt",
                &mut (|ref bits| no_out!(from_bits_desc_alt::<Natural, _>(bits.iter().cloned()))),
            ),
            (
                "naive",
                &mut (|ref bits| no_out!(from_bits_desc_naive(bits.iter().cloned()))),
            ),
        ],
    );
}
