use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::vecs_of_unsigned_var_1;
use inputs::natural::{naturals, nrm_naturals};
use malachite_base::num::traits::SignificantBits;
use malachite_nz::natural::logic::significant_bits::limbs_significant_bits;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_significant_bits);
    register_demo!(registry, demo_natural_significant_bits);
    register_bench!(registry, Small, benchmark_limbs_significant_bits);
    register_bench!(registry, Large, benchmark_natural_significant_bits);
}

fn demo_limbs_significant_bits(gm: GenerationMode, limit: usize) {
    for limbs in vecs_of_unsigned_var_1(gm).take(limit) {
        println!(
            "limbs_significant_bits({:?}) = {}",
            limbs,
            limbs_significant_bits(&limbs)
        );
    }
}

fn demo_natural_significant_bits(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("significant_bits({}) = {}", n, n.significant_bits());
    }
}

fn benchmark_limbs_significant_bits(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_significant_bits(&[u32])",
        BenchmarkType::Single,
        vecs_of_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|limbs| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|limbs| no_out!(limbs_significant_bits(&limbs))),
        )],
    );
}

fn benchmark_natural_significant_bits(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.significant_bits()",
        BenchmarkType::LibraryComparison,
        nrm_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|(_, _, n)| no_out!(n.significant_bits())),
            ),
            ("num", &mut (|(n, _, _)| no_out!(n.bits()))),
            ("rug", &mut (|(_, n, _)| no_out!(n.significant_bits()))),
        ],
    );
}
