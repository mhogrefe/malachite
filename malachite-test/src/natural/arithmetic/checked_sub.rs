use std::cmp::max;

use malachite_base::num::arithmetic::traits::CheckedSub;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz_test_util::natural::arithmetic::checked_sub::checked_sub;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::{nrm_pairs_of_naturals, pairs_of_naturals};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_checked_sub);
    register_demo!(registry, demo_natural_checked_sub_val_ref);
    register_demo!(registry, demo_natural_checked_sub_ref_val);
    register_demo!(registry, demo_natural_checked_sub_ref_ref);
    register_bench!(
        registry,
        Large,
        benchmark_natural_checked_sub_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_checked_sub_evaluation_strategy
    );
}

fn demo_natural_checked_sub(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{}.checked_sub({}) = {:?}", x_old, y_old, x.checked_sub(y));
    }
}

fn demo_natural_checked_sub_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        println!("{}.checked_sub(&{}) = {:?}", x_old, y, x.checked_sub(&y));
    }
}

fn demo_natural_checked_sub_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let y_old = y.clone();
        println!(
            "(&{}).checked_sub({}) = {:?}",
            x,
            y_old,
            (&x).checked_sub(y)
        );
    }
}

fn demo_natural_checked_sub_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        println!("(&{}).checked_sub(&{}) = {:?}", x, y, (&x).checked_sub(&y));
    }
}

fn benchmark_natural_checked_sub_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.checked_sub(Natural)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref x, ref y))| {
            usize::exact_from(max(x.significant_bits(), y.significant_bits()))
        }),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            (
                "malachite",
                &mut (|(_, _, (x, y))| no_out!(x.checked_sub(y))),
            ),
            ("num", &mut (|((x, y), _, _)| no_out!(checked_sub(x, y)))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(checked_sub(x, y)))),
        ],
    );
}

fn benchmark_natural_checked_sub_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.checked_sub(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            (
                "Natural.checked_sub(Natural)",
                &mut (|(x, y)| no_out!(x.checked_sub(y))),
            ),
            (
                "Natural.checked_sub(&Natural)",
                &mut (|(x, y)| no_out!(x.checked_sub(&y))),
            ),
            (
                "&Natural.checked_sub(Natural)",
                &mut (|(x, y)| no_out!((&x).checked_sub(y))),
            ),
            (
                "&Natural.checked_sub(&Natural)",
                &mut (|(x, y)| no_out!((&x).checked_sub(&y))),
            ),
        ],
    );
}
