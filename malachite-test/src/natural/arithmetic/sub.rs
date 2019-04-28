use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    pairs_of_unsigned_vec_var_1, pairs_of_unsigned_vec_var_3, triples_of_unsigned_vec_var_3,
    triples_of_unsigned_vec_var_9,
};
use inputs::natural::{
    nrm_pairs_of_naturals_var_1, pairs_of_naturals_var_1, rm_pairs_of_naturals_var_1,
};
use malachite_base::num::traits::SignificantBits;
use malachite_nz::natural::arithmetic::sub::{
    limbs_sub, limbs_sub_in_place_left, limbs_sub_in_place_right,
    limbs_sub_same_length_in_place_left, limbs_sub_same_length_in_place_right,
    limbs_sub_same_length_to_out, limbs_sub_to_out,
};
use std::cmp::{max, min};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_sub);
    register_demo!(registry, demo_limbs_sub_same_length_to_out);
    register_demo!(registry, demo_limbs_sub_to_out);
    register_demo!(registry, demo_limbs_sub_same_length_in_place_left);
    register_demo!(registry, demo_limbs_sub_in_place_left);
    register_demo!(registry, demo_limbs_sub_same_length_in_place_right);
    register_demo!(registry, demo_limbs_sub_in_place_right);
    register_demo!(registry, demo_natural_sub_assign);
    register_demo!(registry, demo_natural_sub_assign_ref);
    register_demo!(registry, demo_natural_sub);
    register_demo!(registry, demo_natural_sub_val_ref);
    register_demo!(registry, demo_natural_sub_ref_val);
    register_demo!(registry, demo_natural_sub_ref_ref);
    register_bench!(registry, Small, benchmark_limbs_sub);
    register_bench!(registry, Small, benchmark_limbs_sub_same_length_to_out);
    register_bench!(registry, Small, benchmark_limbs_sub_to_out);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_sub_same_length_in_place_left
    );
    register_bench!(registry, Small, benchmark_limbs_sub_in_place_left);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_sub_same_length_in_place_right
    );
    register_bench!(registry, Small, benchmark_limbs_sub_in_place_right);
    register_bench!(
        registry,
        Large,
        benchmark_natural_sub_assign_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_sub_assign_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_sub_library_comparison);
    register_bench!(registry, Large, benchmark_natural_sub_evaluation_strategy);
}

fn demo_limbs_sub(gm: GenerationMode, limit: usize) {
    for (xs, ys) in pairs_of_unsigned_vec_var_3(gm).take(limit) {
        println!("limbs_sub({:?}, {:?}) = {:?}", xs, ys, limbs_sub(&xs, &ys));
    }
}

fn demo_limbs_sub_same_length_to_out(gm: GenerationMode, limit: usize) {
    for (xs, ys, zs) in triples_of_unsigned_vec_var_3(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        let borrow = limbs_sub_same_length_to_out(&mut xs, &ys, &zs);
        println!(
            "limbs_out := {:?}; limbs_sub_same_length_to_out(&mut limbs_out, {:?}, {:?}) = \
             {}; limbs_out = {:?}",
            xs_old, ys, zs, borrow, xs
        );
    }
}

fn demo_limbs_sub_to_out(gm: GenerationMode, limit: usize) {
    for (xs, ys, zs) in triples_of_unsigned_vec_var_9(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        let borrow = limbs_sub_to_out(&mut xs, &ys, &zs);
        println!(
            "limbs_out := {:?}; limbs_sub_to_out(&mut limbs_out, {:?}, {:?}) = {}; \
             limbs_out = {:?}",
            xs_old, ys, zs, borrow, xs
        );
    }
}

fn demo_limbs_sub_same_length_in_place_left(gm: GenerationMode, limit: usize) {
    for (xs, ys) in pairs_of_unsigned_vec_var_1(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        let borrow = limbs_sub_same_length_in_place_left(&mut xs, &ys);
        println!(
            "xs := {:?}; limbs_sub_same_length_in_place_left(&mut xs, {:?}) = {}; xs = {:?}",
            xs_old, ys, borrow, xs
        );
    }
}

fn demo_limbs_sub_in_place_left(gm: GenerationMode, limit: usize) {
    for (xs, ys) in pairs_of_unsigned_vec_var_3(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        let borrow = limbs_sub_in_place_left(&mut xs, &ys);
        println!(
            "xs := {:?}; limbs_sub_in_place_left(&mut xs, {:?}) = {}; xs = {:?}",
            xs_old, ys, borrow, xs
        );
    }
}

fn demo_limbs_sub_same_length_in_place_right(gm: GenerationMode, limit: usize) {
    for (xs, ys) in pairs_of_unsigned_vec_var_1(gm).take(limit) {
        let mut ys = ys.to_vec();
        let ys_old = ys.clone();
        let borrow = limbs_sub_same_length_in_place_right(&xs, &mut ys);
        println!(
            "ys := {:?}; limbs_sub_same_length_in_place_right({:?}, &mut ys) = {}; ys = {:?}",
            ys_old, xs, borrow, xs
        );
    }
}

fn demo_limbs_sub_in_place_right(gm: GenerationMode, limit: usize) {
    for (xs, ys) in pairs_of_unsigned_vec_var_3(gm).take(limit) {
        let mut ys = ys.to_vec();
        let ys_old = ys.clone();
        let borrow = limbs_sub_in_place_right(&xs, &mut ys);
        println!(
            "ys := {:?}; limbs_sub_in_place_right({:?}, &mut ys) = {}; ys = {:?}",
            ys_old, xs, borrow, ys
        );
    }
}

fn demo_natural_sub_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_naturals_var_1(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x -= y;
        println!("x := {}; x -= {}; x = {}", x_old, y_old, x);
    }
}

fn demo_natural_sub_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_naturals_var_1(gm).take(limit) {
        let x_old = x.clone();
        x -= &y;
        println!("x := {}; x -= &{}; x = {}", x_old, y, x);
    }
}

fn demo_natural_sub(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals_var_1(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} - {} = {}", x_old, y_old, x - y);
    }
}

fn demo_natural_sub_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals_var_1(gm).take(limit) {
        let x_old = x.clone();
        println!("{} - &{} = {}", x_old, y, x - &y);
    }
}

fn demo_natural_sub_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals_var_1(gm).take(limit) {
        let y_old = y.clone();
        println!("&{} - {} = {}", x, y_old, &x - y);
    }
}

fn demo_natural_sub_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals_var_1(gm).take(limit) {
        println!("&{} - &{} = {}", x, y, &x - &y);
    }
}

fn benchmark_limbs_sub(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_sub(&[u32], &[u32])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _)| xs.len()),
        "xs.len() = ys.len()",
        &mut [("malachite", &mut (|(xs, ys)| no_out!(limbs_sub(&xs, &ys))))],
    );
}

fn benchmark_limbs_sub_same_length_to_out(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_sub_same_length_to_out(&mut [u32], &[u32], &[u32])",
        BenchmarkType::Single,
        triples_of_unsigned_vec_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ys, _)| ys.len()),
        "xs.len() = ys.len()",
        &mut [(
            "malachite",
            &mut (|(mut xs, ys, zs)| no_out!(limbs_sub_same_length_to_out(&mut xs, &ys, &zs))),
        )],
    );
}

fn benchmark_limbs_sub_to_out(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_sub_to_out(&mut [u32], &[u32], &[u32])",
        BenchmarkType::Single,
        triples_of_unsigned_vec_var_9(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ys, ref zs)| max(ys.len(), zs.len())),
        "xs.len() = ys.len()",
        &mut [(
            "malachite",
            &mut (|(mut xs, ys, zs)| no_out!(limbs_sub_to_out(&mut xs, &ys, &zs))),
        )],
    );
}

fn benchmark_limbs_sub_same_length_in_place_left(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_sub_same_length_in_place_left(&mut [u32], &[u32])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _)| xs.len()),
        "xs.len() = ys.len()",
        &mut [(
            "malachite",
            &mut (|(mut xs, ys)| no_out!(limbs_sub_same_length_in_place_left(&mut xs, &ys))),
        )],
    );
}

fn benchmark_limbs_sub_in_place_left(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_sub_in_place_left(&Vec<u32>, &[u32])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys)| min(xs.len(), ys.len())),
        "min(xs.len(), ys.len())",
        &mut [(
            "malachite",
            &mut (|(mut xs, ys)| no_out!(limbs_sub_in_place_left(&mut xs, &ys))),
        )],
    );
}

fn benchmark_limbs_sub_same_length_in_place_right(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_sub_same_length_in_place_right(&mut [u32], &[u32])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _)| xs.len()),
        "xs.len() = ys.len()",
        &mut [(
            "malachite",
            &mut (|(xs, mut ys)| no_out!(limbs_sub_same_length_in_place_right(&xs, &mut ys))),
        )],
    );
}

fn benchmark_limbs_sub_in_place_right(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_sub_in_place_right(&Vec<u32>, &[u32])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys)| min(xs.len(), ys.len())),
        "min(xs.len(), ys.len())",
        &mut [(
            "malachite",
            &mut (|(xs, mut ys)| no_out!(limbs_sub_in_place_right(&xs, &mut ys))),
        )],
    );
}

fn benchmark_natural_sub_assign_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural -= Natural",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_naturals_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref x, ref y))| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x -= y)),
            ("rug", &mut (|((mut x, y), _)| x -= y)),
        ],
    );
}

fn benchmark_natural_sub_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural -= Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_naturals_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("Natural -= Natural", &mut (|(mut x, y)| x -= y)),
            ("Natural -= &Natural", &mut (|(mut x, y)| x -= &y)),
        ],
    );
}

fn benchmark_natural_sub_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural - Natural",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_naturals_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref x, ref y))| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x - y))),
            ("num", &mut (|((x, y), _, _)| no_out!(x - y))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x - y))),
        ],
    );
}

fn benchmark_natural_sub_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural - Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_naturals_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("Natural - Natural", &mut (|(x, y)| no_out!(x - y))),
            ("Natural - &Natural", &mut (|(x, y)| no_out!(x - &y))),
            ("&Natural - Natural", &mut (|(x, y)| no_out!(&x - y))),
            ("&Natural - &Natural", &mut (|(x, y)| no_out!(&x - &y))),
        ],
    );
}
