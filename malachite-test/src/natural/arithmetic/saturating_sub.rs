use std::cmp::max;

use malachite_base::num::arithmetic::traits::{SaturatingSub, SaturatingSubAssign};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType,
};
use malachite_test::inputs::natural::pairs_of_naturals;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_saturating_sub);
    register_demo!(registry, demo_natural_saturating_sub_val_ref);
    register_demo!(registry, demo_natural_saturating_sub_ref_val);
    register_demo!(registry, demo_natural_saturating_sub_ref_ref);
    register_demo!(registry, demo_natural_saturating_sub_assign);
    register_demo!(registry, demo_natural_saturating_sub_assign_ref);
    register_bench!(
        registry,
        Large,
        benchmark_natural_saturating_sub_assign_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_saturating_sub_evaluation_strategy
    );
}

fn demo_natural_saturating_sub(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "{}.saturating_sub({}) = {:?}",
            x_old,
            y_old,
            x.saturating_sub(y)
        );
    }
}

fn demo_natural_saturating_sub_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        println!(
            "{}.saturating_sub(&{}) = {:?}",
            x_old,
            y,
            x.saturating_sub(&y)
        );
    }
}

fn demo_natural_saturating_sub_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let y_old = y.clone();
        println!(
            "(&{}).saturating_sub({}) = {:?}",
            x,
            y_old,
            (&x).saturating_sub(y)
        );
    }
}

fn demo_natural_saturating_sub_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        println!(
            "(&{}).saturating_sub(&{}) = {:?}",
            x,
            y,
            (&x).saturating_sub(&y)
        );
    }
}

fn demo_natural_saturating_sub_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.saturating_sub_assign(y);
        println!(
            "x := {}; x.saturating_sub_assign({}); x = {}",
            x_old, y_old, x
        );
    }
}

fn demo_natural_saturating_sub_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        x.saturating_sub_assign(&y);
        println!("x := {}; x.saturating_sub_assign(&{}); x = {}", x_old, y, x);
    }
}

fn benchmark_natural_saturating_sub_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.saturating_sub_assign(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            (
                "Natural.saturating_sub_assign(Natural)",
                &mut (|(mut x, y)| x.saturating_sub_assign(y)),
            ),
            (
                "Natural.saturating_sub_assign(&Natural)",
                &mut (|(mut x, y)| x.saturating_sub_assign(&y)),
            ),
        ],
    );
}

fn benchmark_natural_saturating_sub_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.saturating_sub(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            (
                "Natural.saturating_sub(Natural)",
                &mut (|(x, y)| no_out!(x.saturating_sub(y))),
            ),
            (
                "Natural.saturating_sub(&Natural)",
                &mut (|(x, y)| no_out!(x.saturating_sub(&y))),
            ),
            (
                "&Natural.saturating_sub(Natural)",
                &mut (|(x, y)| no_out!((&x).saturating_sub(y))),
            ),
            (
                "&Natural.saturating_sub(&Natural)",
                &mut (|(x, y)| no_out!((&x).saturating_sub(&y))),
            ),
        ],
    );
}
