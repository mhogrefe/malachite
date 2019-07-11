use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::vecs_of_unsigned_var_1;
use inputs::natural::{naturals, rm_naturals};
use malachite_base::num::arithmetic::traits::IsPowerOfTwo;
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::arithmetic::is_power_of_two::limbs_is_power_of_two;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_is_power_of_two);
    register_demo!(registry, demo_natural_is_power_of_two);
    register_bench!(registry, Small, benchmark_limbs_is_power_of_two);
    register_bench!(
        registry,
        Large,
        benchmark_natural_is_power_of_two_library_comparison
    );
}

fn demo_limbs_is_power_of_two(gm: GenerationMode, limit: usize) {
    for limbs in vecs_of_unsigned_var_1(gm).take(limit) {
        println!(
            "limbs_is_power_of_two({:?}) = {:?}",
            limbs,
            limbs_is_power_of_two(&limbs)
        );
    }
}

fn demo_natural_is_power_of_two(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        if n.is_power_of_two() {
            println!("{} is a power of two", n);
        } else {
            println!("{} is not a power of two", n);
        }
    }
}

fn benchmark_limbs_is_power_of_two(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_is_power_of_two(&[u32])",
        BenchmarkType::Single,
        vecs_of_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref limbs| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|ref limbs| no_out!(limbs_is_power_of_two(limbs))),
        )],
    );
}

fn benchmark_natural_is_power_of_two_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.is_power_of_two()",
        BenchmarkType::LibraryComparison,
        rm_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, n)| no_out!(n.is_power_of_two()))),
            ("rug", &mut (|(n, _)| no_out!(n.is_power_of_two()))),
        ],
    );
}
