use std::cmp::max;

use malachite_base::num::traits::SignificantBits;
use malachite_nz::integer::logic::or_natural::{
    limbs_or_pos_neg, limbs_or_pos_neg_in_place_right, limbs_or_pos_neg_to_out,
    limbs_slice_or_pos_neg_in_place_left, limbs_vec_or_pos_neg_in_place_left,
};

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{pairs_of_limb_vec_var_1, triples_of_limb_vec_var_6};
use inputs::integer::{
    pairs_of_integer_and_natural, pairs_of_natural_and_integer, rm_pairs_of_integer_and_natural,
    rm_pairs_of_natural_and_integer,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_or_pos_neg);
    register_demo!(registry, demo_limbs_or_pos_neg_to_out);
    register_demo!(registry, demo_limbs_slice_or_pos_neg_in_place_left);
    register_demo!(registry, demo_limbs_vec_or_pos_neg_in_place_left);
    register_demo!(registry, demo_limbs_or_pos_neg_in_place_right);
    register_demo!(registry, demo_integer_or_natural_assign);
    register_demo!(registry, demo_integer_or_natural_assign_ref);
    register_demo!(registry, demo_integer_or_natural);
    register_demo!(registry, demo_integer_or_natural_val_ref);
    register_demo!(registry, demo_integer_or_natural_ref_val);
    register_demo!(registry, demo_integer_or_natural_ref_ref);
    register_demo!(registry, demo_natural_or_integer);
    register_demo!(registry, demo_natural_or_integer_val_ref);
    register_demo!(registry, demo_natural_or_integer_ref_val);
    register_demo!(registry, demo_natural_or_integer_ref_ref);
    register_bench!(registry, Small, benchmark_limbs_or_pos_neg);
    register_bench!(registry, Small, benchmark_limbs_or_pos_neg_to_out);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_slice_or_pos_neg_in_place_left
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_vec_or_pos_neg_in_place_left
    );
    register_bench!(registry, Small, benchmark_limbs_or_pos_neg_in_place_right);
    register_bench!(
        registry,
        Large,
        benchmark_integer_or_natural_assign_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_or_natural_assign_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_or_natural_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_or_natural_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_or_integer_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_or_integer_evaluation_strategy
    );
}

fn demo_limbs_or_pos_neg(gm: GenerationMode, limit: usize) {
    for (ref xs, ref ys) in pairs_of_limb_vec_var_1(gm).take(limit) {
        println!(
            "limbs_or_pos_neg({:?}, {:?}) = {:?}",
            xs,
            ys,
            limbs_or_pos_neg(xs, ys)
        );
    }
}

fn demo_limbs_or_pos_neg_to_out(gm: GenerationMode, limit: usize) {
    for (ref out, ref xs, ref ys) in triples_of_limb_vec_var_6(gm).take(limit) {
        let mut out = out.to_vec();
        let mut out_old = out.clone();
        limbs_or_pos_neg_to_out(&mut out, xs, ys);
        println!(
            "out := {:?}; limbs_or_pos_neg_to_out(&mut out, {:?}, {:?}); \
             out = {:?}",
            out_old, xs, ys, out
        );
    }
}

fn demo_limbs_slice_or_pos_neg_in_place_left(gm: GenerationMode, limit: usize) {
    for (ref xs, ref ys) in pairs_of_limb_vec_var_1(gm).take(limit) {
        let mut xs = xs.to_vec();
        let mut xs_old = xs.clone();
        let out = limbs_slice_or_pos_neg_in_place_left(&mut xs, ys);
        println!(
            "xs := {:?}; limbs_slice_or_pos_neg_in_place_left(&mut xs, {:?}) = {}; xs = {:?}",
            xs_old, ys, out, xs
        );
    }
}

fn demo_limbs_vec_or_pos_neg_in_place_left(gm: GenerationMode, limit: usize) {
    for (ref xs, ref ys) in pairs_of_limb_vec_var_1(gm).take(limit) {
        let mut xs = xs.to_vec();
        let mut xs_old = xs.clone();
        limbs_vec_or_pos_neg_in_place_left(&mut xs, ys);
        println!(
            "xs := {:?}; limbs_vec_or_pos_neg_in_place_left(&mut xs, {:?}); xs = {:?}",
            xs_old, ys, xs
        );
    }
}

fn demo_limbs_or_pos_neg_in_place_right(gm: GenerationMode, limit: usize) {
    for (ref xs, ref ys) in pairs_of_limb_vec_var_1(gm).take(limit) {
        let mut ys = ys.to_vec();
        let mut ys_old = ys.clone();
        limbs_or_pos_neg_in_place_right(xs, &mut ys);
        println!(
            "ys := {:?}; limbs_or_pos_neg_in_place_right({:?}, &mut ys); ys = {:?}",
            xs, ys_old, ys
        );
    }
}

fn demo_integer_or_natural_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integer_and_natural(gm).take(limit) {
        let x_old = x.clone();
        x |= y.clone();
        println!("x := {}; x |= {}; x = {}", x_old, y, x);
    }
}

fn demo_integer_or_natural_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integer_and_natural(gm).take(limit) {
        let x_old = x.clone();
        x |= &y;
        println!("x := {}; x |= &{}; x = {}", x_old, y, x);
    }
}

fn demo_integer_or_natural(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_natural(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} | {} = {}", x_old, y_old, x | y);
    }
}

fn demo_integer_or_natural_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_natural(gm).take(limit) {
        let x_old = x.clone();
        println!("{} | &{} = {}", x_old, y, x | &y);
    }
}

fn demo_integer_or_natural_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_natural(gm).take(limit) {
        let y_old = y.clone();
        println!("&{} | {} = {}", x, y_old, &x | y);
    }
}

fn demo_integer_or_natural_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_natural(gm).take(limit) {
        println!("&{} | &{} = {}", x, y, &x | &y);
    }
}

fn demo_natural_or_integer(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_integer(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} | {} = {}", x_old, y_old, x | y);
    }
}

fn demo_natural_or_integer_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_integer(gm).take(limit) {
        let x_old = x.clone();
        println!("{} | &{} = {}", x_old, y, x | &y);
    }
}

fn demo_natural_or_integer_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_integer(gm).take(limit) {
        let y_old = y.clone();
        println!("&{} | {} = {}", x, y_old, &x | y);
    }
}

fn demo_natural_or_integer_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_integer(gm).take(limit) {
        println!("&{} | &{} = {}", x, y, &x | &y);
    }
}

fn benchmark_limbs_or_pos_neg(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_or_pos_neg(&[u32], &[u32])",
        BenchmarkType::Single,
        pairs_of_limb_vec_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _)| xs.len()),
        "xs.len()",
        &mut [(
            "malachite",
            &mut (|(ref xs, ref ys)| no_out!(limbs_or_pos_neg(xs, ys))),
        )],
    );
}

fn benchmark_limbs_or_pos_neg_to_out(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_or_pos_neg_to_out(&mut [u32], &[u32], &[u32])",
        BenchmarkType::Single,
        triples_of_limb_vec_var_6(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, _)| xs.len()),
        "xs.len()",
        &mut [(
            "malachite",
            &mut (|(ref mut out, ref xs, ref ys)| limbs_or_pos_neg_to_out(out, xs, ys)),
        )],
    );
}

fn benchmark_limbs_slice_or_pos_neg_in_place_left(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_slice_or_pos_neg_in_place_left(&mut [u32], &[u32])",
        BenchmarkType::Single,
        pairs_of_limb_vec_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _)| xs.len()),
        "xs.len()",
        &mut [(
            "malachite",
            &mut (|(ref mut xs, ref ys)| no_out!(limbs_slice_or_pos_neg_in_place_left(xs, ys))),
        )],
    );
}

fn benchmark_limbs_vec_or_pos_neg_in_place_left(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_vec_or_pos_neg_in_place_left(&Vec<u32>, &[u32])",
        BenchmarkType::Single,
        pairs_of_limb_vec_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _)| xs.len()),
        "xs.len()",
        &mut [(
            "malachite",
            &mut (|(ref mut xs, ref ys)| limbs_vec_or_pos_neg_in_place_left(xs, ys)),
        )],
    );
}

fn benchmark_limbs_or_pos_neg_in_place_right(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_or_pos_neg_in_place_right(&[u32], &mut [u32])",
        BenchmarkType::Single,
        pairs_of_limb_vec_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _)| xs.len()),
        "xs.len()",
        &mut [(
            "malachite",
            &mut (|(ref xs, ref mut ys)| limbs_or_pos_neg_in_place_right(xs, ys)),
        )],
    );
}

fn benchmark_integer_or_natural_assign_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer |= Natural",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref x, ref y))| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x |= y)),
            ("rug", &mut (|((mut x, y), _)| x |= y)),
        ],
    );
}

fn benchmark_integer_or_natural_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer |= Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("Integer |= Natural", &mut (|(mut x, y)| no_out!(x |= y))),
            ("Integer |= &Natural", &mut (|(mut x, y)| no_out!(x |= &y))),
        ],
    );
}

fn benchmark_integer_or_natural_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer | Natural",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref x, ref y))| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x | y))),
            ("rug", &mut (|((x, y), _)| no_out!(x | y))),
        ],
    );
}

fn benchmark_integer_or_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer | Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("Integer | Natural", &mut (|(x, y)| no_out!(x | y))),
            ("Integer | &Natural", &mut (|(x, y)| no_out!(x | &y))),
            ("&Integer | Natural", &mut (|(x, y)| no_out!(&x | y))),
            ("&Integer | &Natural", &mut (|(x, y)| no_out!(&x | &y))),
        ],
    );
}

fn benchmark_natural_or_integer_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural | Integer",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_integer(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref x, ref y))| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x | y))),
            ("rug", &mut (|((x, y), _)| no_out!(x | y))),
        ],
    );
}

fn benchmark_natural_or_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural | Integer",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_integer(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("Natural | Integer", &mut (|(x, y)| no_out!(x | y))),
            ("Natural | &Integer", &mut (|(x, y)| no_out!(x | &y))),
            ("&Natural | Integer", &mut (|(x, y)| no_out!(&x | y))),
            ("&Natural | &Integer", &mut (|(x, y)| no_out!(&x | &y))),
        ],
    );
}
