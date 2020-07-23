use std::cmp::{max, min};

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_nz::natural::logic::xor::{
    limbs_xor, limbs_xor_in_place_either, limbs_xor_in_place_left, limbs_xor_limb,
    limbs_xor_limb_in_place, limbs_xor_limb_to_out, limbs_xor_same_length,
    limbs_xor_same_length_in_place_left, limbs_xor_same_length_to_out, limbs_xor_to_out,
};
use malachite_nz_test_util::natural::logic::xor::{natural_xor_alt_1, natural_xor_alt_2};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    pairs_of_nonempty_unsigned_vec_and_unsigned, pairs_of_unsigned_vec,
    pairs_of_unsigned_vec_var_1, triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_2,
    triples_of_unsigned_vec_var_3, triples_of_unsigned_vec_var_4,
};
use malachite_test::inputs::natural::{
    nrm_pairs_of_naturals, pairs_of_naturals, rm_pairs_of_naturals,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_xor_limb);
    register_demo!(registry, demo_limbs_xor_limb_to_out);
    register_demo!(registry, demo_limbs_xor_limb_in_place);
    register_demo!(registry, demo_limbs_xor_same_length);
    register_demo!(registry, demo_limbs_xor);
    register_demo!(registry, demo_limbs_xor_same_length_to_out);
    register_demo!(registry, demo_limbs_xor_to_out);
    register_demo!(registry, demo_limbs_xor_same_length_in_place_left);
    register_demo!(registry, demo_limbs_xor_in_place_left);
    register_demo!(registry, demo_limbs_xor_in_place_either);
    register_demo!(registry, demo_natural_xor_assign);
    register_demo!(registry, demo_natural_xor_assign_ref);
    register_demo!(registry, demo_natural_xor);
    register_demo!(registry, demo_natural_xor_val_ref);
    register_demo!(registry, demo_natural_xor_ref_val);
    register_demo!(registry, demo_natural_xor_ref_ref);
    register_bench!(registry, Small, benchmark_limbs_xor_limb);
    register_bench!(registry, Small, benchmark_limbs_xor_limb_to_out);
    register_bench!(registry, Small, benchmark_limbs_xor_limb_in_place);
    register_bench!(registry, Small, benchmark_limbs_xor_same_length);
    register_bench!(registry, Small, benchmark_limbs_xor);
    register_bench!(registry, Small, benchmark_limbs_xor_same_length_to_out);
    register_bench!(registry, Small, benchmark_limbs_xor_to_out);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_xor_same_length_in_place_left
    );
    register_bench!(registry, Small, benchmark_limbs_xor_in_place_left);
    register_bench!(registry, Small, benchmark_limbs_xor_in_place_either);
    register_bench!(
        registry,
        Large,
        benchmark_natural_xor_assign_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_xor_assign_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_xor_library_comparison);
    register_bench!(registry, Large, benchmark_natural_xor_algorithms);
    register_bench!(registry, Large, benchmark_natural_xor_evaluation_strategy);
}

fn demo_limbs_xor_limb(gm: GenerationMode, limit: usize) {
    for (ref limbs, limb) in pairs_of_nonempty_unsigned_vec_and_unsigned(gm).take(limit) {
        println!(
            "limbs_xor_limb({:?}, {}) = {:?}",
            limbs,
            limb,
            limbs_xor_limb(limbs, limb)
        );
    }
}

fn demo_limbs_xor_limb_to_out(gm: GenerationMode, limit: usize) {
    for (out, in_limbs, limb) in
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_2(gm).take(limit)
    {
        let mut out = out.to_vec();
        let out_old = out.clone();
        limbs_xor_limb_to_out(&mut out, &in_limbs, limb);
        println!(
            "out := {:?}; limbs_xor_limb_to_out(&mut out, {:?}, {}); out = {:?}",
            out_old, in_limbs, limb, out
        );
    }
}

fn demo_limbs_xor_limb_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_nonempty_unsigned_vec_and_unsigned(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let limbs_old = limbs.clone();
        limbs_xor_limb_in_place(&mut limbs, limb);
        println!(
            "limbs := {:?}; limbs_xor_limb_in_place(&mut limbs, {}); limbs = {:?}",
            limbs_old, limb, limbs
        );
    }
}

fn demo_limbs_xor_same_length(gm: GenerationMode, limit: usize) {
    for (xs, ys) in pairs_of_unsigned_vec_var_1(gm).take(limit) {
        println!(
            "limbs_xor_same_length({:?}, {:?}) = {:?}",
            xs,
            ys,
            limbs_xor_same_length(&xs, &ys)
        );
    }
}

fn demo_limbs_xor(gm: GenerationMode, limit: usize) {
    for (xs, ys) in pairs_of_unsigned_vec(gm).take(limit) {
        println!("limbs_xor({:?}, {:?}) = {:?}", xs, ys, limbs_xor(&xs, &ys));
    }
}

fn demo_limbs_xor_same_length_to_out(gm: GenerationMode, limit: usize) {
    for (xs, ys, zs) in triples_of_unsigned_vec_var_3(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        limbs_xor_same_length_to_out(&mut xs, &ys, &zs);
        println!(
            "out := {:?}; limbs_xor_same_length_to_out(&mut out, {:?}, {:?}); \
             out = {:?}",
            xs_old, ys, zs, xs
        );
    }
}

fn demo_limbs_xor_to_out(gm: GenerationMode, limit: usize) {
    for (xs, ys, zs) in triples_of_unsigned_vec_var_4(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        limbs_xor_to_out(&mut xs, &ys, &zs);
        println!(
            "out := {:?}; limbs_xor_to_out(&mut out, {:?}, {:?}); out = {:?}",
            xs_old, ys, zs, xs
        );
    }
}

fn demo_limbs_xor_same_length_in_place_left(gm: GenerationMode, limit: usize) {
    for (xs, ys) in pairs_of_unsigned_vec_var_1(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        limbs_xor_same_length_in_place_left(&mut xs, &ys);
        println!(
            "xs := {:?}; limbs_xor_same_length_in_place_left(&mut xs, {:?}); xs = {:?}",
            xs_old, ys, xs
        );
    }
}

fn demo_limbs_xor_in_place_left(gm: GenerationMode, limit: usize) {
    for (xs, ys) in pairs_of_unsigned_vec(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        limbs_xor_in_place_left(&mut xs, &ys);
        println!(
            "xs := {:?}; limbs_xor_in_place_left(&mut xs, {:?}); xs = {:?}",
            xs_old, ys, xs
        );
    }
}

fn demo_natural_xor_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        x ^= y.clone();
        println!("x := {}; x ^= {}; x = {}", x_old, y, x);
    }
}

fn demo_natural_xor_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        x ^= &y;
        println!("x := {}; x ^= &{}; x = {}", x_old, y, x);
    }
}

fn demo_natural_xor(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} ^ {} = {}", x_old, y_old, x ^ y);
    }
}

fn demo_natural_xor_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        println!("{} ^ &{} = {}", x_old, y, x ^ &y);
    }
}

fn demo_natural_xor_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let y_old = y.clone();
        println!("&{} ^ {} = {}", x, y_old, &x ^ y);
    }
}

fn demo_natural_xor_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        println!("&{} ^ &{} = {}", x, y, &x ^ &y);
    }
}

fn demo_limbs_xor_in_place_either(gm: GenerationMode, limit: usize) {
    for (xs, ys) in pairs_of_unsigned_vec(gm).take(limit) {
        let mut xs = xs.to_vec();
        let mut ys = ys.to_vec();
        let xs_old = xs.clone();
        let ys_old = ys.clone();
        let right = limbs_xor_in_place_either(&mut xs, &mut ys);
        println!(
            "xs := {:?}; ys := {:?}; limbs_xor_in_place_either(&mut xs, &mut ys) = {}; \
             xs = {:?}; ys = {:?}",
            xs_old, ys_old, right, xs, ys
        );
    }
}

fn benchmark_limbs_xor_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_xor_limb(&[Limb], Limb)",
        BenchmarkType::Single,
        pairs_of_nonempty_unsigned_vec_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(limbs, limb)| no_out!(limbs_xor_limb(&limbs, limb))),
        )],
    );
}

fn benchmark_limbs_xor_limb_to_out(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_xor_limb_to_out(&mut [Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref in_limbs, _)| in_limbs.len()),
        "in_limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut out, in_limbs, limb)| {
                no_out!(limbs_xor_limb_to_out(&mut out, &in_limbs, limb))
            }),
        )],
    );
}

fn benchmark_limbs_xor_limb_in_place(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_xor_limb_in_place(&mut [Limb], Limb)",
        BenchmarkType::Single,
        pairs_of_nonempty_unsigned_vec_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut limbs, limb)| no_out!(limbs_xor_limb_in_place(&mut limbs, limb))),
        )],
    );
}

fn benchmark_limbs_xor_same_length(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_xor_same_length(&[u32], &[u32])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _)| xs.len()),
        "xs.len() = ys.len()",
        &mut [(
            "malachite",
            &mut (|(xs, ys)| no_out!(limbs_xor_same_length(&xs, &ys))),
        )],
    );
}

fn benchmark_limbs_xor(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_xor(&[u32], &[u32])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _)| xs.len()),
        "xs.len() = ys.len()",
        &mut [("malachite", &mut (|(xs, ys)| no_out!(limbs_xor(&xs, &ys))))],
    );
}

fn benchmark_limbs_xor_same_length_to_out(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_xor_same_length_to_out(&mut [u32], &[u32], &[u32])",
        BenchmarkType::Single,
        triples_of_unsigned_vec_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ys, _)| ys.len()),
        "xs.len() = ys.len()",
        &mut [(
            "malachite",
            &mut (|(mut xs, ys, zs)| limbs_xor_same_length_to_out(&mut xs, &ys, &zs)),
        )],
    );
}

fn benchmark_limbs_xor_to_out(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_xor_to_out(&mut [u32], &[u32], &[u32])",
        BenchmarkType::Single,
        triples_of_unsigned_vec_var_4(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ys, ref zs)| max(ys.len(), zs.len())),
        "xs.len() = ys.len()",
        &mut [(
            "malachite",
            &mut (|(mut xs, ys, zs)| limbs_xor_to_out(&mut xs, &ys, &zs)),
        )],
    );
}

fn benchmark_limbs_xor_same_length_in_place_left(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_xor_same_length_in_place_left(&mut [u32], &[u32])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _)| xs.len()),
        "xs.len() = ys.len()",
        &mut [(
            "malachite",
            &mut (|(mut xs, ys)| limbs_xor_same_length_in_place_left(&mut xs, &ys)),
        )],
    );
}

fn benchmark_limbs_xor_in_place_left(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_xor_in_place_left(&mut Vec<u32>, &[u32])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ys)| ys.len()),
        "ys.len()",
        &mut [(
            "malachite",
            &mut (|(mut xs, ys)| limbs_xor_in_place_left(&mut xs, &ys)),
        )],
    );
}

fn benchmark_limbs_xor_in_place_either(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_xor_in_place_either(&mut Vec<u32>, &mut Vec<u32>)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys)| min(xs.len(), ys.len())),
        "min(xs.len(), ys.len())",
        &mut [(
            "malachite",
            &mut (|(mut xs, mut ys)| no_out!(limbs_xor_in_place_either(&mut xs, &mut ys))),
        )],
    );
}

fn benchmark_natural_xor_assign_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural ^= Natural",
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
            ("malachite", &mut (|(_, (mut x, y))| x ^= y)),
            ("rug", &mut (|((mut x, y), _)| x ^= y)),
        ],
    );
}

fn benchmark_natural_xor_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural ^= Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("Natural ^= Natural", &mut (|(mut x, y)| no_out!(x ^= y))),
            ("Natural ^= &Natural", &mut (|(mut x, y)| no_out!(x ^= &y))),
        ],
    );
}

fn benchmark_natural_xor_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "Natural ^ Natural",
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
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x ^ y))),
            ("num", &mut (|((x, y), _, _)| no_out!(x ^ y))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x ^ y))),
        ],
    );
}

fn benchmark_natural_xor_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "Natural ^ Natural",
        BenchmarkType::Algorithms,
        pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("default", &mut (|(ref x, ref y)| no_out!(x ^ y))),
            (
                "using bits explicitly",
                &mut (|(ref x, ref y)| no_out!(natural_xor_alt_1(x, y))),
            ),
            (
                "using limbs explicitly",
                &mut (|(ref x, ref y)| no_out!(natural_xor_alt_2(x, y))),
            ),
        ],
    );
}

fn benchmark_natural_xor_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "Natural ^ Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("Natural ^ Natural", &mut (|(x, y)| no_out!(x ^ y))),
            ("Natural ^ &Natural", &mut (|(x, y)| no_out!(x ^ &y))),
            ("&Natural ^ Natural", &mut (|(x, y)| no_out!(&x ^ y))),
            ("&Natural ^ &Natural", &mut (|(x, y)| no_out!(&x ^ &y))),
        ],
    );
}
