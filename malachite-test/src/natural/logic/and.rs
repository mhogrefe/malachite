use std::cmp::{max, min};

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_nz::natural::logic::and::{
    limbs_and, limbs_and_in_place_either, limbs_and_limb, limbs_and_same_length_to_out,
    limbs_and_to_out, limbs_slice_and_in_place_left, limbs_slice_and_same_length_in_place_left,
    limbs_vec_and_in_place_left,
};
use malachite_nz_test_util::natural::logic::and::{natural_and_alt_1, natural_and_alt_2};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    pairs_of_nonempty_unsigned_vec_and_unsigned, pairs_of_unsigned_vec,
    pairs_of_unsigned_vec_var_1, triples_of_unsigned_vec_var_3, triples_of_unsigned_vec_var_4,
};
use malachite_test::inputs::natural::{
    nrm_pairs_of_naturals, pairs_of_naturals, rm_pairs_of_naturals,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_and_limb);
    register_demo!(registry, demo_limbs_and);
    register_demo!(registry, demo_limbs_and_same_length_to_out);
    register_demo!(registry, demo_limbs_and_to_out);
    register_demo!(registry, demo_limbs_slice_and_same_length_in_place_left);
    register_demo!(registry, demo_limbs_slice_and_in_place_left);
    register_demo!(registry, demo_limbs_vec_and_in_place_left);
    register_demo!(registry, demo_limbs_and_in_place_either);
    register_demo!(registry, demo_natural_and_assign);
    register_demo!(registry, demo_natural_and_assign_ref);
    register_demo!(registry, demo_natural_and);
    register_demo!(registry, demo_natural_and_val_ref);
    register_demo!(registry, demo_natural_and_ref_val);
    register_demo!(registry, demo_natural_and_ref_ref);
    register_bench!(registry, Small, benchmark_limbs_and_limb);
    register_bench!(registry, Small, benchmark_limbs_and);
    register_bench!(registry, Small, benchmark_limbs_and_same_length_to_out);
    register_bench!(registry, Small, benchmark_limbs_and_to_out);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_slice_and_same_length_in_place_left
    );
    register_bench!(registry, Small, benchmark_limbs_slice_and_in_place_left);
    register_bench!(registry, Small, benchmark_limbs_vec_and_in_place_left);
    register_bench!(registry, Small, benchmark_limbs_and_in_place_either);
    register_bench!(
        registry,
        Large,
        benchmark_natural_and_assign_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_and_assign_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_and_library_comparison);
    register_bench!(registry, Large, benchmark_natural_and_algorithms);
    register_bench!(registry, Large, benchmark_natural_and_evaluation_strategy);
}

fn demo_limbs_and_limb(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_nonempty_unsigned_vec_and_unsigned(gm).take(limit) {
        println!(
            "limbs_and_limb({:?}, {}) = {:?}",
            limbs,
            limb,
            limbs_and_limb(&limbs, limb)
        );
    }
}

fn demo_limbs_and(gm: GenerationMode, limit: usize) {
    for (xs, ys) in pairs_of_unsigned_vec(gm).take(limit) {
        println!("limbs_and({:?}, {:?}) = {:?}", xs, ys, limbs_and(&xs, &ys));
    }
}

fn demo_limbs_and_same_length_to_out(gm: GenerationMode, limit: usize) {
    for (xs, ys, zs) in triples_of_unsigned_vec_var_3(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        limbs_and_same_length_to_out(&mut xs, &ys, &zs);
        println!(
            "out := {:?}; limbs_and_same_length_to_out(&mut out, {:?}, {:?}); \
             out = {:?}",
            xs_old, ys, zs, xs
        );
    }
}

fn demo_limbs_and_to_out(gm: GenerationMode, limit: usize) {
    for (xs, ys, zs) in triples_of_unsigned_vec_var_4(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        limbs_and_to_out(&mut xs, &ys, &zs);
        println!(
            "out := {:?}; limbs_and_to_out(&mut out, {:?}, {:?}); out = {:?}",
            xs_old, ys, zs, xs
        );
    }
}

fn demo_limbs_slice_and_same_length_in_place_left(gm: GenerationMode, limit: usize) {
    for (xs, ys) in pairs_of_unsigned_vec_var_1(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        limbs_slice_and_same_length_in_place_left(&mut xs, &ys);
        println!(
            "xs := {:?}; limbs_slice_and_same_length_in_place_left(&mut xs, {:?}); xs = {:?}",
            xs_old, ys, xs
        );
    }
}

fn demo_limbs_slice_and_in_place_left(gm: GenerationMode, limit: usize) {
    for (xs, ys) in pairs_of_unsigned_vec(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        let truncate_size = limbs_slice_and_in_place_left(&mut xs, &ys);
        println!(
            "xs := {:?}; limbs_slice_and_in_place_left(&mut xs, {:?}) = {:?}; xs = {:?}",
            xs_old, ys, truncate_size, xs
        );
    }
}

fn demo_limbs_vec_and_in_place_left(gm: GenerationMode, limit: usize) {
    for (xs, ys) in pairs_of_unsigned_vec(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        limbs_vec_and_in_place_left(&mut xs, &ys);
        println!(
            "xs := {:?}; limbs_vec_and_in_place_left(&mut xs, {:?}); xs = {:?}",
            xs_old, ys, xs
        );
    }
}

fn demo_limbs_and_in_place_either(gm: GenerationMode, limit: usize) {
    for (xs, ys) in pairs_of_unsigned_vec(gm).take(limit) {
        let mut xs = xs.to_vec();
        let mut ys = ys.to_vec();
        let xs_old = xs.clone();
        let ys_old = ys.clone();
        let right = limbs_and_in_place_either(&mut xs, &mut ys);
        println!(
            "xs := {:?}; ys := {:?}; limbs_and_in_place_either(&mut xs, &mut ys) = {}; \
             xs = {:?}; ys = {:?}",
            xs_old, ys_old, right, xs, ys
        );
    }
}

fn demo_natural_and_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        x &= y.clone();
        println!("x := {}; x &= {}; x = {}", x_old, y, x);
    }
}

fn demo_natural_and_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        x &= &y;
        println!("x := {}; x &= &{}; x = {}", x_old, y, x);
    }
}

fn demo_natural_and(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} & {} = {}", x_old, y_old, x & y);
    }
}

fn demo_natural_and_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        println!("{} & &{} = {}", x_old, y, x & &y);
    }
}

fn demo_natural_and_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let y_old = y.clone();
        println!("&{} & {} = {}", x, y_old, &x & y);
    }
}

fn demo_natural_and_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        println!("&{} & &{} = {}", x, y, &x & &y);
    }
}

fn benchmark_limbs_and_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_and_limb(&[Limb], Limb)",
        BenchmarkType::Single,
        pairs_of_nonempty_unsigned_vec_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(limbs, limb)| no_out!(limbs_and_limb(&limbs, limb))),
        )],
    );
}

fn benchmark_limbs_and(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_and(&[u32], &[u32])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _)| xs.len()),
        "xs.len() = ys.len()",
        &mut [("malachite", &mut (|(xs, ys)| no_out!(limbs_and(&xs, &ys))))],
    );
}

fn benchmark_limbs_and_same_length_to_out(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_and_same_length_to_out(&mut [u32], &[u32], &[u32])",
        BenchmarkType::Single,
        triples_of_unsigned_vec_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ys, _)| ys.len()),
        "xs.len() = ys.len()",
        &mut [(
            "malachite",
            &mut (|(mut xs, ys, zs)| limbs_and_same_length_to_out(&mut xs, &ys, &zs)),
        )],
    );
}

fn benchmark_limbs_and_to_out(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_and_to_out(&mut [u32], &[u32], &[u32])",
        BenchmarkType::Single,
        triples_of_unsigned_vec_var_4(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ys, ref zs)| max(ys.len(), zs.len())),
        "xs.len() = ys.len()",
        &mut [(
            "malachite",
            &mut (|(mut xs, ys, zs)| limbs_and_to_out(&mut xs, &ys, &zs)),
        )],
    );
}

fn benchmark_limbs_slice_and_same_length_in_place_left(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_slice_and_same_length_in_place_left(&mut [u32], &[u32])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _)| xs.len()),
        "xs.len() = ys.len()",
        &mut [(
            "malachite",
            &mut (|(mut xs, ys)| limbs_slice_and_same_length_in_place_left(&mut xs, &ys)),
        )],
    );
}

fn benchmark_limbs_slice_and_in_place_left(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_slice_and_in_place_left(&mut [u32], &[u32])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys)| min(xs.len(), ys.len())),
        "min(xs.len(), ys.len())",
        &mut [(
            "malachite",
            &mut (|(mut xs, ys)| no_out!(limbs_slice_and_in_place_left(&mut xs, &ys))),
        )],
    );
}

fn benchmark_limbs_vec_and_in_place_left(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_vec_and_in_place_left(&Vec<u32>, &[u32])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys)| min(xs.len(), ys.len())),
        "min(xs.len(), ys.len())",
        &mut [(
            "malachite",
            &mut (|(mut xs, ys)| limbs_vec_and_in_place_left(&mut xs, &ys)),
        )],
    );
}

fn benchmark_limbs_and_in_place_either(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_and_in_place_either(&mut [u32], &mut [u32])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys)| min(xs.len(), ys.len())),
        "min(xs.len(), ys.len())",
        &mut [(
            "malachite",
            &mut (|(mut xs, mut ys)| no_out!(limbs_and_in_place_either(&mut xs, &mut ys))),
        )],
    );
}

fn benchmark_natural_and_assign_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural &= Natural",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref x, ref y))| {
            usize::exact_from(max(x.significant_bits(), y.significant_bits()))
        }),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x &= y)),
            ("rug", &mut (|((mut x, y), _)| x &= y)),
        ],
    );
}

fn benchmark_natural_and_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural &= Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("Natural &= Natural", &mut (|(mut x, y)| no_out!(x &= y))),
            ("Natural &= &Natural", &mut (|(mut x, y)| no_out!(x &= &y))),
        ],
    );
}

fn benchmark_natural_and_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "Natural & Natural",
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
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x & y))),
            ("num", &mut (|((x, y), _, _)| no_out!(x & y))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x & y))),
        ],
    );
}

fn benchmark_natural_and_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "Natural & Natural",
        BenchmarkType::Algorithms,
        pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("default", &mut (|(ref x, ref y)| no_out!(x & y))),
            (
                "using bits explicitly",
                &mut (|(ref x, ref y)| no_out!(natural_and_alt_1(x, y))),
            ),
            (
                "using limbs explicitly",
                &mut (|(ref x, ref y)| no_out!(natural_and_alt_2(x, y))),
            ),
        ],
    );
}

fn benchmark_natural_and_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "Natural & Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("Natural & Natural", &mut (|(x, y)| no_out!(x & y))),
            ("Natural & &Natural", &mut (|(x, y)| no_out!(x & &y))),
            ("&Natural & Natural", &mut (|(x, y)| no_out!(&x & y))),
            ("&Natural & &Natural", &mut (|(x, y)| no_out!(&x & &y))),
        ],
    );
}
