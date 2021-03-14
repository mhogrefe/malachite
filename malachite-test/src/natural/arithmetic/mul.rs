use malachite_base::num::arithmetic::traits::PowerOfTwo;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_nz::natural::arithmetic::mul::mul_low::{
    _limbs_mul_low_same_length_basecase, _limbs_mul_low_same_length_basecase_alt,
    _limbs_mul_low_same_length_divide_and_conquer,
    _limbs_mul_low_same_length_divide_and_conquer_scratch_len,
    _limbs_mul_low_same_length_divide_and_conquer_shared_scratch, _limbs_mul_low_same_length_large,
    limbs_mul_low_same_length,
};
use malachite_nz::natural::arithmetic::mul::{
    _limbs_mul_greater_to_out_basecase, limbs_mul_same_length_to_out,
};
use malachite_nz::platform::Limb;
use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    triples_of_unsigned_vec_var_25, triples_of_unsigned_vec_var_46, triples_of_unsigned_vec_var_47,
    triples_of_unsigned_vec_var_48, triples_of_unsigned_vec_var_49, triples_of_unsigned_vec_var_52,
};
use malachite_test::inputs::natural::{
    nrm_pairs_of_naturals, pairs_of_naturals, rm_pairs_of_naturals,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_mul_low_same_length_basecase);
    register_demo!(
        registry,
        demo_limbs_mul_low_same_length_divide_and_conquer_shared_scratch
    );
    register_demo!(registry, demo_limbs_mul_low_same_length_divide_and_conquer);
    register_demo!(registry, demo_limbs_mul_low_same_length);
    register_demo!(registry, demo_natural_mul_assign);
    register_demo!(registry, demo_natural_mul_assign_ref);
    register_demo!(registry, demo_natural_mul);
    register_demo!(registry, demo_natural_mul_val_ref);
    register_demo!(registry, demo_natural_mul_ref_val);
    register_demo!(registry, demo_natural_mul_ref_ref);
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_low_same_length_basecase_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_low_same_length_basecase_algorithms_2
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_low_same_length_divide_and_conquer_shared_scratch_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_low_same_length_divide_and_conquer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_low_same_length_large_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_low_same_length_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mul_assign_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mul_assign_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_mul_library_comparison);
    register_bench!(registry, Large, benchmark_natural_mul_evaluation_strategy);
}

fn demo_limbs_mul_low_same_length_basecase(gm: GenerationMode, limit: usize) {
    for (mut out, xs, ys) in triples_of_unsigned_vec_var_46(gm).take(limit) {
        let out_old = out.clone();
        _limbs_mul_low_same_length_basecase(&mut out, &xs, &ys);
        println!(
            "out := {:?}; _limbs_mul_low_same_length_basecase(&mut out, {:?}, {:?}); out = {:?}",
            out_old, xs, ys, out
        );
    }
}

fn demo_limbs_mul_low_same_length_divide_and_conquer_shared_scratch(
    gm: GenerationMode,
    limit: usize,
) {
    for (xs, ys, zs) in triples_of_unsigned_vec_var_48(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        _limbs_mul_low_same_length_divide_and_conquer_shared_scratch(&mut xs, &ys, &zs);
        println!(
            "out := {:?}; \
             _limbs_mul_low_same_length_divide_and_conquer_shared_scratch(&mut out, {:?}, {:?}); \
             out = {:?}",
            xs_old, ys, zs, xs
        );
    }
}

fn demo_limbs_mul_low_same_length_divide_and_conquer(gm: GenerationMode, limit: usize) {
    for (xs, ys, zs) in triples_of_unsigned_vec_var_49(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        let mut scratch = vec![0; ys.len() << 1];
        _limbs_mul_low_same_length_divide_and_conquer(&mut xs, &ys, &zs, &mut scratch);
        println!(
            "out := {:?}; _limbs_mul_low_same_length_divide_and_conquer(&mut out, {:?}, {:?}); \
             out = {:?}",
            xs_old, ys, zs, xs
        );
    }
}

fn demo_limbs_mul_low_same_length(gm: GenerationMode, limit: usize) {
    for (xs, ys, zs) in triples_of_unsigned_vec_var_25(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        limbs_mul_low_same_length(&mut xs, &ys, &zs);
        println!(
            "out := {:?}; limbs_mul_low_same_length(&mut out, {:?}, {:?}); out = {:?}",
            xs_old, ys, zs, xs
        );
    }
}

fn demo_natural_mul_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        x *= y.clone();
        println!("x := {}; x *= {}; x = {}", x_old, y, x);
    }
}

fn demo_natural_mul_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        x *= &y;
        println!("x := {}; x *= &{}; x = {}", x_old, y, x);
    }
}

fn demo_natural_mul(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} * {} = {}", x_old, y_old, x * y);
    }
}

fn demo_natural_mul_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        println!("{} * &{} = {}", x_old, y, x * &y);
    }
}

fn demo_natural_mul_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let y_old = y.clone();
        println!("&{} * {} = {}", x, y_old, &x * y);
    }
}

fn demo_natural_mul_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        println!("&{} * &{} = {}", x, y, &x * &y);
    }
}

fn benchmark_limbs_mul_low_same_length_basecase_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "limbs_mul_low_same_length_basecase(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_46(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, _)| xs.len()),
        "xs.len()",
        &mut [
            (
                "standard",
                &mut (|(mut out, xs, ys)| _limbs_mul_low_same_length_basecase(&mut out, &xs, &ys)),
            ),
            (
                "alt",
                &mut (|(mut out, xs, ys)| {
                    _limbs_mul_low_same_length_basecase_alt(&mut out, &xs, &ys)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_low_same_length_basecase_algorithms_2(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "limbs_mul_low_same_length_basecase(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_47(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, _)| xs.len()),
        "xs.len()",
        &mut [
            (
                "basecase mul",
                &mut (|(mut out, xs, ys)| _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)),
            ),
            (
                "basecase mul low",
                &mut (|(mut out, xs, ys)| _limbs_mul_low_same_length_basecase(&mut out, &xs, &ys)),
            ),
        ],
    );
}

fn benchmark_limbs_mul_low_same_length_divide_and_conquer_shared_scratch_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "limbs_mul_low_same_length_divide_and_conquer_shared_scratch\
         (&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_48(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, _)| xs.len()),
        "xs.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out, xs, ys)| _limbs_mul_low_same_length_basecase(&mut out, &xs, &ys)),
            ),
            (
                "divide-and-conquer",
                &mut (|(mut out, xs, ys)| {
                    _limbs_mul_low_same_length_divide_and_conquer_shared_scratch(&mut out, &xs, &ys)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_low_same_length_divide_and_conquer_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "limbs_mul_low_same_length_divide_and_conquer(&mut [Limb], &[Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_49(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, _)| xs.len()),
        "xs.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out, xs, ys)| _limbs_mul_low_same_length_basecase(&mut out, &xs, &ys)),
            ),
            (
                "divide-and-conquer",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch = vec![0; ys.len() << 1];
                    _limbs_mul_low_same_length_divide_and_conquer(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_low_same_length_large_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "limbs_mul_low_same_length_large(&mut [Limb], &[Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_52(gm.with_scale(u32::power_of_two(15))),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, _)| xs.len()),
        "xs.len()",
        &mut [
            (
                "mul low divide-and-conquer",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![
                            0;
                            _limbs_mul_low_same_length_divide_and_conquer_scratch_len(xs.len())
                        ];
                    _limbs_mul_low_same_length_divide_and_conquer(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
            (
                "mul low large",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![
                            0;
                            _limbs_mul_low_same_length_divide_and_conquer_scratch_len(xs.len())
                        ];
                    _limbs_mul_low_same_length_large(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_low_same_length_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "limbs_mul_low_same_length(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_25(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, _)| xs.len()),
        "xs.len()",
        &mut [
            (
                "mul low",
                &mut (|(mut out, xs, ys)| limbs_mul_low_same_length(&mut out, &xs, &ys)),
            ),
            (
                "mul",
                &mut (|(mut out, xs, ys)| limbs_mul_same_length_to_out(&mut out, &xs, &ys)),
            ),
        ],
    );
}

fn benchmark_natural_mul_assign_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural *= Natural",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_naturals(gm.with_scale(u32::exact_from(16 * Limb::WIDTH))),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref x, ref y))| usize::exact_from(x.significant_bits() + y.significant_bits())),
        "x.significant_bits() + y.significant_bits()",
        &mut [
            ("Malachite", &mut (|(_, (mut x, y))| x *= y)),
            ("rug", &mut (|((mut x, y), _)| x *= y)),
        ],
    );
}

fn benchmark_natural_mul_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural *= Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| usize::exact_from(x.significant_bits() + y.significant_bits())),
        "x.significant_bits() + y.significant_bits()",
        &mut [
            ("Natural *= Natural", &mut (|(mut x, y)| no_out!(x *= y))),
            ("Natural *= &Natural", &mut (|(mut x, y)| no_out!(x *= &y))),
        ],
    );
}

fn benchmark_natural_mul_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural * Natural",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_naturals(gm.with_scale(1 << 15)),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref x, ref y))| usize::exact_from(x.significant_bits() + y.significant_bits())),
        "x.significant_bits() + y.significant_bits()",
        &mut [
            ("Malachite", &mut (|(_, _, (x, y))| no_out!(x * y))),
            ("num", &mut (|((x, y), _, _)| no_out!(x * y))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x * y))),
        ],
    );
}

fn benchmark_natural_mul_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural * Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| usize::exact_from(x.significant_bits() + y.significant_bits())),
        "x.significant_bits() + y.significant_bits()",
        &mut [
            ("Natural * Natural", &mut (|(x, y)| no_out!(x * y))),
            ("Natural * &Natural", &mut (|(x, y)| no_out!(x * &y))),
            ("&Natural * Natural", &mut (|(x, y)| no_out!(&x * y))),
            ("&Natural * &Natural", &mut (|(x, y)| no_out!(&x * &y))),
        ],
    );
}
