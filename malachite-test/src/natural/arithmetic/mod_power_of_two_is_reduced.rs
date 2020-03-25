use malachite_base::num::arithmetic::traits::{ModIsReduced, ModPowerOfTwoIsReduced, PowerOfTwo};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::Natural;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::pairs_of_natural_and_small_unsigned;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_mod_power_of_two_is_reduced);
    register_bench!(
        registry,
        None,
        benchmark_natural_mod_power_of_two_is_reduced
    );
}

fn demo_natural_mod_power_of_two_is_reduced(gm: GenerationMode, limit: usize) {
    for (n, log_base) in pairs_of_natural_and_small_unsigned(gm).take(limit) {
        if n.mod_power_of_two_is_reduced(log_base) {
            println!("{} is reduced mod 2^{}", n, log_base);
        } else {
            println!("{} is not reduced mod 2^{}", n, log_base);
        }
    }
}

fn benchmark_natural_mod_power_of_two_is_reduced(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.mod_power_of_two_is_reduced(u64)",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "default",
                &mut (|(n, log_base)| no_out!(n.mod_power_of_two_is_reduced(log_base))),
            ),
            (
                "using mod_is_reduced",
                &mut (|(n, log_base)| no_out!(n.mod_is_reduced(&Natural::power_of_two(log_base)))),
            ),
        ],
    );
}
