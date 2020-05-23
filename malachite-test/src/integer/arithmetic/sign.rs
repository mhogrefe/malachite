use std::cmp::Ordering;

use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz_test_util::integer::arithmetic::sign::num_sign;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{integers, nrm_integers};

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
    m_run_benchmark(
        "Integer.sign()",
        BenchmarkType::LibraryComparison,
        nrm_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref n)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, n)| no_out!(n.sign()))),
            ("num", &mut (|(n, _, _)| no_out!(num_sign(&n)))),
            ("rug", &mut (|(_, n, _)| no_out!(n.cmp0()))),
        ],
    );
}
