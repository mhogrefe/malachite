use malachite_base::num::arithmetic::traits::{Square, SquareAssign};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_nz::natural::arithmetic::mul::fft::limbs_mul_greater_to_out_fft;
use malachite_nz::natural::arithmetic::mul::limbs_mul_greater_to_out_basecase;
use malachite_nz::natural::arithmetic::square::{
    limbs_square_to_out_basecase, limbs_square_to_out_toom_2,
    limbs_square_to_out_toom_2_scratch_len, limbs_square_to_out_toom_3,
    limbs_square_to_out_toom_3_scratch_len, limbs_square_to_out_toom_4,
    limbs_square_to_out_toom_4_scratch_len, limbs_square_to_out_toom_6,
    limbs_square_to_out_toom_6_scratch_len, limbs_square_to_out_toom_8,
    limbs_square_to_out_toom_8_scratch_len,
};
use malachite_nz_test_util::natural::arithmetic::square::limbs_square_to_out_basecase_unrestricted;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    pairs_of_unsigned_vec_var_17, pairs_of_unsigned_vec_var_18, pairs_of_unsigned_vec_var_19,
    pairs_of_unsigned_vec_var_21, pairs_of_unsigned_vec_var_22, pairs_of_unsigned_vec_var_23,
};
use malachite_test::inputs::natural::{naturals, pairs_of_naturals_var_3};

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
    register_bench!(
        registry,
        Large,
        benchmark_limbs_square_to_out_toom_2_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_square_to_out_toom_3_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_square_to_out_toom_4_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_square_to_out_toom_6_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_square_to_out_toom_8_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_square_to_out_fft_algorithms
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
        limbs_square_to_out_basecase(&mut xs, &ys);
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
    run_benchmark_old(
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
                &mut (|(mut xs, ys)| limbs_square_to_out_basecase(&mut xs, &ys)),
            ),
            (
                "using _limbs_mul_greater_to_out_basecase",
                &mut (|(mut xs, ys)| limbs_mul_greater_to_out_basecase(&mut xs, &ys, &ys)),
            ),
        ],
    );
}

fn benchmark_limbs_square_to_out_toom_2_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "_limbs_square_to_out_toom_2(&mut [Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Algorithms,
        pairs_of_unsigned_vec_var_18(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs)| xs.len()),
        "xs.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out, xs)| limbs_square_to_out_basecase_unrestricted(&mut out, &xs)),
            ),
            (
                "Toom2",
                &mut (|(mut out, xs)| {
                    let mut scratch = vec![0; limbs_square_to_out_toom_2_scratch_len(xs.len())];
                    limbs_square_to_out_toom_2(&mut out, &xs, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_square_to_out_toom_3_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "_limbs_square_to_out_toom_3(&mut [Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Algorithms,
        pairs_of_unsigned_vec_var_19(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs)| xs.len()),
        "xs.len()",
        &mut [
            (
                "Toom2",
                &mut (|(mut out, xs)| {
                    let mut scratch = vec![0; limbs_square_to_out_toom_2_scratch_len(xs.len())];
                    limbs_square_to_out_toom_2(&mut out, &xs, &mut scratch)
                }),
            ),
            (
                "Toom3",
                &mut (|(mut out, xs)| {
                    let mut scratch = vec![0; limbs_square_to_out_toom_3_scratch_len(xs.len())];
                    limbs_square_to_out_toom_3(&mut out, &xs, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_square_to_out_toom_4_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "_limbs_square_to_out_toom_4(&mut [Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Algorithms,
        pairs_of_unsigned_vec_var_21(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs)| xs.len()),
        "xs.len()",
        &mut [
            (
                "Toom3",
                &mut (|(mut out, xs)| {
                    let mut scratch = vec![0; limbs_square_to_out_toom_3_scratch_len(xs.len())];
                    limbs_square_to_out_toom_3(&mut out, &xs, &mut scratch)
                }),
            ),
            (
                "Toom4",
                &mut (|(mut out, xs)| {
                    let mut scratch = vec![0; limbs_square_to_out_toom_4_scratch_len(xs.len())];
                    limbs_square_to_out_toom_4(&mut out, &xs, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_square_to_out_toom_6_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "_limbs_square_to_out_toom_6(&mut [Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Algorithms,
        pairs_of_unsigned_vec_var_22(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs)| xs.len()),
        "xs.len()",
        &mut [
            (
                "Toom4",
                &mut (|(mut out, xs)| {
                    let mut scratch = vec![0; limbs_square_to_out_toom_4_scratch_len(xs.len())];
                    limbs_square_to_out_toom_4(&mut out, &xs, &mut scratch)
                }),
            ),
            (
                "Toom6",
                &mut (|(mut out, xs)| {
                    let mut scratch = vec![0; limbs_square_to_out_toom_6_scratch_len(xs.len())];
                    limbs_square_to_out_toom_6(&mut out, &xs, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_square_to_out_toom_8_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "_limbs_square_to_out_toom_8(&mut [Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Algorithms,
        pairs_of_unsigned_vec_var_23(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs)| xs.len()),
        "xs.len()",
        &mut [
            (
                "Toom6",
                &mut (|(mut out, xs)| {
                    let mut scratch = vec![0; limbs_square_to_out_toom_6_scratch_len(xs.len())];
                    limbs_square_to_out_toom_6(&mut out, &xs, &mut scratch)
                }),
            ),
            (
                "Toom8",
                &mut (|(mut out, xs)| {
                    let mut scratch = vec![0; limbs_square_to_out_toom_8_scratch_len(xs.len())];
                    limbs_square_to_out_toom_8(&mut out, &xs, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_square_to_out_fft_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "_limbs_mul_greater_to_out_fft(&mut [Limb], &[Limb], &[Limb]) for squaring",
        BenchmarkType::Algorithms,
        pairs_of_unsigned_vec_var_23(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs)| xs.len()),
        "xs.len()",
        &mut [
            (
                "Toom8",
                &mut (|(mut out, xs)| {
                    let mut scratch = vec![0; limbs_square_to_out_toom_8_scratch_len(xs.len())];
                    limbs_square_to_out_toom_8(&mut out, &xs, &mut scratch)
                }),
            ),
            (
                "FFT",
                &mut (|(mut out, xs)| limbs_mul_greater_to_out_fft(&mut out, &xs, &xs)),
            ),
        ],
    );
}

fn benchmark_natural_square_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural.square_assign()",
        BenchmarkType::Single,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [("Malachite", &mut (|mut n| n.square_assign()))],
    );
}

fn benchmark_natural_square_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural.square()",
        BenchmarkType::Algorithms,
        pairs_of_naturals_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|(n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|(ref n, _)| no_out!(n.square()))),
            ("using *", &mut (|(n_1, n_2)| no_out!(n_1 * n_2))),
        ],
    );
}

fn benchmark_natural_square_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
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
