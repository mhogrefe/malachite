use std::cmp::Ordering;

use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_nz_test_util::integer::arithmetic::sign::num_sign;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::integer::{integers, nrm_integers};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_sign);
    register_bench!(registry, Large, benchmark_integer_sign_library_comparison);
}

fn demo_integer_sign(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        match n.sign() {
            Ordering::Less => println!("{} is negative", n),
            Ordering::Equal => println!("{} is zero", n),
            Ordering::Greater => println!("{} is positive", n),
        }
    }
}

fn benchmark_integer_sign_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Integer.sign()",
        BenchmarkType::LibraryComparison,
        nrm_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref n)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("Malachite", &mut (|(_, _, n)| no_out!(n.sign()))),
            ("num", &mut (|(n, _, _)| no_out!(num_sign(&n)))),
            ("rug", &mut (|(_, n, _)| no_out!(n.cmp0()))),
        ],
    );
}
