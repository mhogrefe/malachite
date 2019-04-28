use malachite_base::num::traits::SignificantBits;
use malachite_nz::natural::logic::count_ones::limbs_count_ones;
use malachite_nz::natural::Natural;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::vecs_of_unsigned;
use inputs::natural::naturals;

pub fn natural_count_ones_alt_1(n: &Natural) -> u64 {
    n.bits().filter(|&b| b).count() as u64
}

pub fn natural_count_ones_alt_2(n: &Natural) -> u64 {
    n.limbs().map(|limb| u64::from(limb.count_ones())).sum()
}

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_count_ones);
    register_demo!(registry, demo_natural_count_ones);
    register_bench!(registry, Small, benchmark_limbs_count_ones);
    register_bench!(registry, Large, benchmark_natural_count_ones_algorithms);
}

fn demo_limbs_count_ones(gm: GenerationMode, limit: usize) {
    for limbs in vecs_of_unsigned(gm).take(limit) {
        println!(
            "limbs_count_ones({:?}) = {}",
            limbs,
            limbs_count_ones(&limbs)
        );
    }
}

fn demo_natural_count_ones(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("count_ones({}) = {}", n, n.count_ones());
    }
}

fn benchmark_limbs_count_ones(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_count_ones(&[u32])",
        BenchmarkType::Single,
        vecs_of_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|limbs| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|limbs| no_out!(limbs_count_ones(&limbs))),
        )],
    );
}

fn benchmark_natural_count_ones_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.count_ones()",
        BenchmarkType::Algorithms,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("default", &mut (|n| no_out!(n.count_ones()))),
            (
                "using bits explicitly",
                &mut (|n| no_out!(natural_count_ones_alt_1(&n))),
            ),
            (
                "using limbs explicitly",
                &mut (|n| no_out!(natural_count_ones_alt_2(&n))),
            ),
        ],
    );
}
