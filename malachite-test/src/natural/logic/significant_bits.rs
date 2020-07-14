use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::logic::significant_bits::limbs_significant_bits;
use malachite_nz::platform::Limb;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType,
};
use malachite_test::inputs::base::vecs_of_unsigned_var_1;
use malachite_test::inputs::natural::{naturals, nrm_naturals};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_significant_bits);
    register_demo!(registry, demo_natural_significant_bits);
    register_bench!(registry, Small, benchmark_limbs_significant_bits);
    register_bench!(registry, Large, benchmark_natural_significant_bits);
}

fn demo_limbs_significant_bits(gm: GenerationMode, limit: usize) {
    for limbs in vecs_of_unsigned_var_1::<Limb>(gm).take(limit) {
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
        vecs_of_unsigned_var_1::<Limb>(gm),
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
        &(|&(_, _, ref n)| usize::exact_from(n.significant_bits())),
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
