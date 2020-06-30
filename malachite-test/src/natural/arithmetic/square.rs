use malachite_base::num::arithmetic::traits::{Square, SquareAssign};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::arithmetic::mul::_limbs_mul_greater_to_out_basecase;
use malachite_nz::natural::arithmetic::square::_limbs_square_to_out_basecase;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::pairs_of_unsigned_vec_var_17;
use inputs::natural::naturals;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_square_to_out_basecase);
    register_demo!(registry, demo_natural_square_assign);
    register_demo!(registry, demo_natural_square);
    register_demo!(registry, demo_natural_square_ref);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_square_to_out_basecase_algorithms
    );
    register_bench!(registry, Large, benchmark_natural_square_assign);
    register_bench!(registry, Large, benchmark_natural_square_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_natural_square_evaluation_strategy
    );
}

fn demo_limbs_square_to_out_basecase(gm: GenerationMode, limit: usize) {
    for (mut xs, ys) in pairs_of_unsigned_vec_var_17(gm).take(limit) {
        let xs_old = xs.clone();
        _limbs_square_to_out_basecase(&mut xs, &ys);
        println!(
            "out := {:?}; _limbs_square_to_out_basecase(&mut out, {:?}); out = {:?}",
            xs_old, ys, xs
        );
    }
}

fn demo_natural_square_assign(gm: GenerationMode, limit: usize) {
    for mut n in naturals(gm).take(limit) {
        let old_n = n.clone();
        n.square_assign();
        println!("n := {}; n.square_assign(); n = {}", n, old_n);
    }
}

fn demo_natural_square(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("{} ^ 2 = {}", n.clone(), n.square());
    }
}

fn demo_natural_square_ref(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("&{} ^ 2 = {}", n, (&n).square());
    }
}

fn benchmark_limbs_square_to_out_basecase_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "_limbs_square_to_out_basecase(&mut [Limb], &[Limb])",
        BenchmarkType::Algorithms,
        pairs_of_unsigned_vec_var_17(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _)| xs.len()),
        "xs.len()",
        &mut [
            (
                "default",
                &mut (|(mut xs, ys)| _limbs_square_to_out_basecase(&mut xs, &ys)),
            ),
            (
                "using _limbs_mul_greater_to_out_basecase",
                &mut (|(mut xs, ys)| _limbs_mul_greater_to_out_basecase(&mut xs, &ys, &ys)),
            ),
        ],
    );
}

fn benchmark_natural_square_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.square_assign()",
        BenchmarkType::Single,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [("malachite", &mut (|mut n| n.square_assign()))],
    );
}

fn benchmark_natural_square_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.square()",
        BenchmarkType::Algorithms,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|ref n| no_out!(n.square()))),
            ("using *", &mut (|ref n| no_out!(n * n))),
        ],
    );
}

fn benchmark_natural_square_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.square()",
        BenchmarkType::EvaluationStrategy,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("Natural.square()", &mut (|n| no_out!(n.square()))),
            ("(&Natural).square()", &mut (|n| no_out!((&n).square()))),
        ],
    );
}
