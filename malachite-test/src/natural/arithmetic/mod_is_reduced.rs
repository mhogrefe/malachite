use malachite_base::num::arithmetic::traits::ModIsReduced;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::natural::pairs_of_natural_and_positive_natural;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_mod_is_reduced);
    register_bench!(registry, None, benchmark_natural_mod_is_reduced);
}

fn demo_natural_mod_is_reduced(gm: GenerationMode, limit: usize) {
    for (n, m) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        if n.mod_is_reduced(&m) {
            println!("{} is reduced mod {}", n, m);
        } else {
            println!("{} is not reduced mod {}", n, m);
        }
    }
}

fn benchmark_natural_mod_is_reduced(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural.mod_is_reduced(&Natural)",
        BenchmarkType::Single,
        pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [("Malachite", &mut (|(n, m)| no_out!(n.mod_is_reduced(&m))))],
    );
}
