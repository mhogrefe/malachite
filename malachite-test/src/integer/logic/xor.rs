use std::cmp::max;

use malachite_base::num::traits::SignificantBits;
use malachite_nz::integer::logic::xor::{
    limbs_xor_neg_neg, limbs_xor_neg_neg_in_place_either, limbs_xor_neg_neg_in_place_left,
    limbs_xor_neg_neg_to_out,
};
use malachite_nz::integer::Integer;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{pairs_of_limb_vec_var_1, triples_of_limb_vec_var_7};
use inputs::integer::{pairs_of_integers, rm_pairs_of_integers};
use integer::logic::{integer_op_bits, integer_op_limbs};

pub fn integer_xor_alt_1(x: &Integer, y: &Integer) -> Integer {
    integer_op_bits(&|a, b| a ^ b, x, y)
}

pub fn integer_xor_alt_2(x: &Integer, y: &Integer) -> Integer {
    integer_op_limbs(&|a, b| a ^ b, x, y)
}

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_xor_neg_neg);
    register_demo!(registry, demo_limbs_xor_neg_neg_to_out);
    register_demo!(registry, demo_limbs_xor_neg_neg_in_place_left);
    register_demo!(registry, demo_limbs_xor_neg_neg_in_place_either);
    register_demo!(registry, demo_integer_xor_assign);
    register_demo!(registry, demo_integer_xor_assign_ref);
    register_demo!(registry, demo_integer_xor);
    register_demo!(registry, demo_integer_xor_val_ref);
    register_demo!(registry, demo_integer_xor_ref_val);
    register_demo!(registry, demo_integer_xor_ref_ref);
    register_bench!(registry, Small, benchmark_limbs_xor_neg_neg);
    register_bench!(registry, Small, benchmark_limbs_xor_neg_neg_to_out);
    register_bench!(registry, Small, benchmark_limbs_xor_neg_neg_in_place_left);
    register_bench!(registry, Small, benchmark_limbs_xor_neg_neg_in_place_either);
    register_bench!(
        registry,
        Large,
        benchmark_integer_xor_assign_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_xor_assign_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_integer_xor_library_comparison);
    register_bench!(registry, Large, benchmark_integer_xor_algorithms);
    register_bench!(registry, Large, benchmark_integer_xor_evaluation_strategy);
}

fn demo_limbs_xor_neg_neg(gm: GenerationMode, limit: usize) {
    for (ref xs, ref ys) in pairs_of_limb_vec_var_1(gm).take(limit) {
        println!(
            "limbs_xor_neg_neg({:?}, {:?}) = {:?}",
            xs,
            ys,
            limbs_xor_neg_neg(xs, ys)
        );
    }
}

fn demo_limbs_xor_neg_neg_to_out(gm: GenerationMode, limit: usize) {
    for (ref out, ref xs, ref ys) in triples_of_limb_vec_var_7(gm).take(limit) {
        let mut out = out.to_vec();
        let mut out_old = out.clone();
        limbs_xor_neg_neg_to_out(&mut out, xs, ys);
        println!(
            "out := {:?}; limbs_xor_neg_neg_to_out(&mut out, {:?}, {:?}); \
             out = {:?}",
            out_old, xs, ys, out
        );
    }
}

fn demo_limbs_xor_neg_neg_in_place_left(gm: GenerationMode, limit: usize) {
    for (ref xs, ref ys) in pairs_of_limb_vec_var_1(gm).take(limit) {
        let mut xs = xs.to_vec();
        let mut xs_old = xs.clone();
        limbs_xor_neg_neg_in_place_left(&mut xs, ys);
        println!(
            "xs := {:?}; limbs_xor_neg_neg_in_place_left(&mut xs, {:?}); xs = {:?}",
            xs_old, ys, xs
        );
    }
}

fn demo_limbs_xor_neg_neg_in_place_either(gm: GenerationMode, limit: usize) {
    for (ref xs, ref ys) in pairs_of_limb_vec_var_1(gm).take(limit) {
        let mut xs = xs.to_vec();
        let mut xs_old = xs.clone();
        let mut ys = ys.to_vec();
        let mut ys_old = ys.clone();
        let b = limbs_xor_neg_neg_in_place_either(&mut xs, &mut ys);
        println!(
            "xs := {:?}; ys := {:?}; limbs_xor_neg_neg_in_place_either(&mut xs, &mut ys) = {}; \
             xs = {:?}; ys = {:?}",
            xs_old, ys_old, b, xs, ys
        );
    }
}

fn demo_integer_xor_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integers(gm).take(limit) {
        let x_old = x.clone();
        x ^= y.clone();
        println!("x := {}; x ^= {}; x = {}", x_old, y, x);
    }
}

fn demo_integer_xor_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integers(gm).take(limit) {
        let x_old = x.clone();
        x ^= &y;
        println!("x := {}; x ^= &{}; x = {}", x_old, y, x);
    }
}

fn demo_integer_xor(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} ^ {} = {}", x_old, y_old, x ^ y);
    }
}

fn demo_integer_xor_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        let x_old = x.clone();
        println!("{} ^ &{} = {}", x_old, y, x ^ &y);
    }
}

fn demo_integer_xor_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        let y_old = y.clone();
        println!("&{} ^ {} = {}", x, y_old, &x ^ y);
    }
}

fn demo_integer_xor_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        println!("&{} ^ &{} = {}", x, y, &x ^ &y);
    }
}

fn benchmark_limbs_xor_neg_neg(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_xor_neg_neg(&[Limb], &[Limb])",
        BenchmarkType::Single,
        pairs_of_limb_vec_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys)| max(xs.len(), ys.len())),
        "max(xs.len(), ys.len())",
        &mut [(
            "malachite",
            &mut (|(ref xs, ref ys)| no_out!(limbs_xor_neg_neg(xs, ys))),
        )],
    );
}

fn benchmark_limbs_xor_neg_neg_to_out(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_xor_neg_neg_to_out(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        triples_of_limb_vec_var_7(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| max(xs.len(), ys.len())),
        "max(xs.len(), ys.len())",
        &mut [(
            "malachite",
            &mut (|(ref mut out, ref xs, ref ys)| limbs_xor_neg_neg_to_out(out, xs, ys)),
        )],
    );
}

fn benchmark_limbs_xor_neg_neg_in_place_left(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_xor_neg_neg_in_place_left(&mut Vec<Limb>, &[Limb])",
        BenchmarkType::Single,
        pairs_of_limb_vec_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys)| max(xs.len(), ys.len())),
        "max(xs.len(), ys.len())",
        &mut [(
            "malachite",
            &mut (|(ref mut xs, ref ys)| no_out!(limbs_xor_neg_neg_in_place_left(xs, ys))),
        )],
    );
}

fn benchmark_limbs_xor_neg_neg_in_place_either(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_xor_neg_neg_in_place_either(&mut [Limb], &mut [Limb])",
        BenchmarkType::Single,
        pairs_of_limb_vec_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys)| max(xs.len(), ys.len())),
        "max(xs.len(), ys.len())",
        &mut [(
            "malachite",
            &mut (|(ref mut xs, ref mut ys)| no_out!(limbs_xor_neg_neg_in_place_either(xs, ys))),
        )],
    );
}

fn benchmark_integer_xor_assign_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer ^= Integer",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref x, ref y))| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x ^= y)),
            ("rug", &mut (|((mut x, y), _)| x ^= y)),
        ],
    );
}

fn benchmark_integer_xor_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer ^= Integer",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("Integer ^= Integer", &mut (|(mut x, y)| no_out!(x ^= y))),
            ("Integer ^= &Integer", &mut (|(mut x, y)| no_out!(x ^= &y))),
        ],
    );
}

fn benchmark_integer_xor_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer ^ Integer",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref x, ref y))| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x ^ y))),
            ("rug", &mut (|((x, y), _)| no_out!(x ^ y))),
        ],
    );
}

fn benchmark_integer_xor_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer ^ Integer",
        BenchmarkType::Algorithms,
        pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("default", &mut (|(ref x, ref y)| no_out!(x ^ y))),
            (
                "using bits explicitly",
                &mut (|(ref x, ref y)| no_out!(integer_xor_alt_1(x, y))),
            ),
            (
                "using limbs explicitly",
                &mut (|(ref x, ref y)| no_out!(integer_xor_alt_2(x, y))),
            ),
        ],
    );
}

fn benchmark_integer_xor_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer ^ Integer",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("Integer ^ Integer", &mut (|(x, y)| no_out!(x ^ y))),
            ("Integer ^ &Integer", &mut (|(x, y)| no_out!(x ^ &y))),
            ("&Integer ^ Integer", &mut (|(x, y)| no_out!(&x ^ y))),
            ("&Integer ^ &Integer", &mut (|(x, y)| no_out!(&x ^ &y))),
        ],
    );
}
