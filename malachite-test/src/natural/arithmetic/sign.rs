use std::cmp::Ordering;

use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::natural::{naturals, nrm_naturals};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_sign);
    register_bench!(registry, Large, benchmark_natural_sign_library_comparison);
}

fn demo_natural_sign(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        match n.sign() {
            Ordering::Less => println!("{} is negative", n),
            Ordering::Equal => println!("{} is zero", n),
            Ordering::Greater => println!("{} is positive", n),
        }
    }
}

fn benchmark_natural_sign_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "Natural.sign()",
        BenchmarkType::LibraryComparison,
        nrm_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref n)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, n)| no_out!(n.sign()))),
            ("rug", &mut (|(_, n, _)| no_out!(n.cmp0()))),
        ],
    );
}
