use std::cmp::max;

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_nz::integer::logic::or::{
    limbs_neg_or_limb, limbs_neg_or_limb_in_place, limbs_neg_or_limb_to_out, limbs_neg_or_neg_limb,
    limbs_or_neg_neg, limbs_or_neg_neg_in_place_either, limbs_or_neg_neg_to_out, limbs_or_pos_neg,
    limbs_or_pos_neg_in_place_right, limbs_or_pos_neg_to_out, limbs_pos_or_neg_limb,
    limbs_slice_or_neg_neg_in_place_left, limbs_slice_or_pos_neg_in_place_left,
    limbs_vec_or_neg_neg_in_place_left, limbs_vec_or_pos_neg_in_place_left,
};
use malachite_nz_test_util::integer::logic::or::{integer_or_alt_1, integer_or_alt_2};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    pairs_of_unsigned_vec_and_positive_unsigned_var_2, pairs_of_unsigned_vec_and_unsigned_var_2,
    pairs_of_unsigned_vec_var_6, triples_of_limb_vec_var_6, triples_of_limb_vec_var_8,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_3,
};
use malachite_test::inputs::integer::{pairs_of_integers, rm_pairs_of_integers};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_neg_or_limb);
    register_demo!(registry, demo_limbs_neg_or_limb_to_out);
    register_demo!(registry, demo_limbs_neg_or_limb_in_place);
    register_demo!(registry, demo_limbs_pos_or_neg_limb);
    register_demo!(registry, demo_limbs_neg_or_neg_limb);
    register_demo!(registry, demo_limbs_or_pos_neg);
    register_demo!(registry, demo_limbs_or_pos_neg_to_out);
    register_demo!(registry, demo_limbs_slice_or_pos_neg_in_place_left);
    register_demo!(registry, demo_limbs_vec_or_pos_neg_in_place_left);
    register_demo!(registry, demo_limbs_or_pos_neg_in_place_right);
    register_demo!(registry, demo_limbs_or_neg_neg);
    register_demo!(registry, demo_limbs_or_neg_neg_to_out);
    register_demo!(registry, demo_limbs_slice_or_neg_neg_in_place_left);
    register_demo!(registry, demo_limbs_vec_or_neg_neg_in_place_left);
    register_demo!(registry, demo_limbs_or_neg_neg_in_place_either);
    register_demo!(registry, demo_integer_or_assign);
    register_demo!(registry, demo_integer_or_assign_ref);
    register_demo!(registry, demo_integer_or);
    register_demo!(registry, demo_integer_or_val_ref);
    register_demo!(registry, demo_integer_or_ref_val);
    register_demo!(registry, demo_integer_or_ref_ref);
    register_bench!(registry, Small, benchmark_limbs_neg_or_limb);
    register_bench!(registry, Small, benchmark_limbs_neg_or_limb_to_out);
    register_bench!(registry, Small, benchmark_limbs_neg_or_limb_in_place);
    register_bench!(registry, Small, benchmark_limbs_pos_or_neg_limb);
    register_bench!(registry, Small, benchmark_limbs_neg_or_neg_limb);
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
    register_bench!(registry, Small, benchmark_limbs_or_neg_neg);
    register_bench!(registry, Small, benchmark_limbs_or_neg_neg_to_out);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_slice_or_neg_neg_in_place_left
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_vec_or_neg_neg_in_place_left
    );
    register_bench!(registry, Small, benchmark_limbs_or_neg_neg_in_place_either);
    register_bench!(
        registry,
        Large,
        benchmark_integer_or_assign_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_or_assign_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_integer_or_library_comparison);
    register_bench!(registry, Large, benchmark_integer_or_algorithms);
    register_bench!(registry, Large, benchmark_integer_or_evaluation_strategy);
}

fn demo_limbs_neg_or_limb(gm: GenerationMode, limit: usize) {
    for (ref limbs, limb) in pairs_of_unsigned_vec_and_unsigned_var_2(gm).take(limit) {
        println!(
            "limbs_neg_or_limb({:?}, {}) = {:?}",
            limbs,
            limb,
            limbs_neg_or_limb(limbs, limb)
        );
    }
}

fn demo_limbs_neg_or_limb_to_out(gm: GenerationMode, limit: usize) {
    for (out, in_limbs, limb) in
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_3(gm).take(limit)
    {
        let mut out = out.to_vec();
        let out_old = out.clone();
        limbs_neg_or_limb_to_out(&mut out, &in_limbs, limb);
        println!(
            "out := {:?}; limbs_neg_or_limb_to_out(&mut out, {:?}, {}); \
             out = {:?}",
            out_old, in_limbs, limb, out
        );
    }
}

fn demo_limbs_neg_or_limb_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_unsigned_vec_and_unsigned_var_2(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let limbs_old = limbs.clone();
        limbs_neg_or_limb_in_place(&mut limbs, limb);
        println!(
            "limbs := {:?}; limbs_neg_or_limb_in_place(&mut limbs, {}); limbs = {:?}",
            limbs_old, limb, limbs
        );
    }
}

fn demo_limbs_pos_or_neg_limb(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_unsigned_vec_and_positive_unsigned_var_2(gm).take(limit) {
        println!(
            "limbs_pos_or_neg_limb({:?}, {}) = {}",
            limbs,
            limb,
            limbs_pos_or_neg_limb(&limbs, limb)
        );
    }
}

fn demo_limbs_neg_or_neg_limb(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_unsigned_vec_and_positive_unsigned_var_2(gm).take(limit) {
        println!(
            "limbs_neg_or_neg_limb({:?}, {}) = {}",
            limbs,
            limb,
            limbs_neg_or_neg_limb(&limbs, limb)
        );
    }
}

fn demo_limbs_or_pos_neg(gm: GenerationMode, limit: usize) {
    for (ref xs, ref ys) in pairs_of_unsigned_vec_var_6(gm).take(limit) {
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
        let out_old = out.clone();
        limbs_or_pos_neg_to_out(&mut out, xs, ys);
        println!(
            "out := {:?}; limbs_or_pos_neg_to_out(&mut out, {:?}, {:?}); \
             out = {:?}",
            out_old, xs, ys, out
        );
    }
}

fn demo_limbs_slice_or_pos_neg_in_place_left(gm: GenerationMode, limit: usize) {
    for (ref xs, ref ys) in pairs_of_unsigned_vec_var_6(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        let out = limbs_slice_or_pos_neg_in_place_left(&mut xs, ys);
        println!(
            "xs := {:?}; limbs_slice_or_pos_neg_in_place_left(&mut xs, {:?}) = {}; xs = {:?}",
            xs_old, ys, out, xs
        );
    }
}

fn demo_limbs_vec_or_pos_neg_in_place_left(gm: GenerationMode, limit: usize) {
    for (ref xs, ref ys) in pairs_of_unsigned_vec_var_6(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        limbs_vec_or_pos_neg_in_place_left(&mut xs, ys);
        println!(
            "xs := {:?}; limbs_vec_or_pos_neg_in_place_left(&mut xs, {:?}); xs = {:?}",
            xs_old, ys, xs
        );
    }
}

fn demo_limbs_or_pos_neg_in_place_right(gm: GenerationMode, limit: usize) {
    for (ref xs, ref ys) in pairs_of_unsigned_vec_var_6(gm).take(limit) {
        let mut ys = ys.to_vec();
        let ys_old = ys.clone();
        limbs_or_pos_neg_in_place_right(xs, &mut ys);
        println!(
            "ys := {:?}; limbs_or_pos_neg_in_place_right({:?}, &mut ys); ys = {:?}",
            xs, ys_old, ys
        );
    }
}

fn demo_limbs_or_neg_neg(gm: GenerationMode, limit: usize) {
    for (ref xs, ref ys) in pairs_of_unsigned_vec_var_6(gm).take(limit) {
        println!(
            "limbs_or_neg_neg({:?}, {:?}) = {:?}",
            xs,
            ys,
            limbs_or_neg_neg(xs, ys)
        );
    }
}

fn demo_limbs_or_neg_neg_to_out(gm: GenerationMode, limit: usize) {
    for (ref out, ref xs, ref ys) in triples_of_limb_vec_var_8(gm).take(limit) {
        let mut out = out.to_vec();
        let out_old = out.clone();
        limbs_or_neg_neg_to_out(&mut out, xs, ys);
        println!(
            "out := {:?}; limbs_or_neg_neg_to_out(&mut out, {:?}, {:?}); \
             out = {:?}",
            out_old, xs, ys, out
        );
    }
}

fn demo_limbs_slice_or_neg_neg_in_place_left(gm: GenerationMode, limit: usize) {
    for (ref xs, ref ys) in pairs_of_unsigned_vec_var_6(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        limbs_slice_or_neg_neg_in_place_left(&mut xs, ys);
        println!(
            "xs := {:?}; limbs_slice_or_neg_neg_in_place_left(&mut xs, {:?}); xs = {:?}",
            xs_old, ys, xs
        );
    }
}

fn demo_limbs_vec_or_neg_neg_in_place_left(gm: GenerationMode, limit: usize) {
    for (ref xs, ref ys) in pairs_of_unsigned_vec_var_6(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        limbs_vec_or_neg_neg_in_place_left(&mut xs, ys);
        println!(
            "xs := {:?}; limbs_vec_or_neg_neg_in_place_left(&mut xs, {:?}); xs = {:?}",
            xs_old, ys, xs
        );
    }
}

fn demo_limbs_or_neg_neg_in_place_either(gm: GenerationMode, limit: usize) {
    for (ref xs, ref ys) in pairs_of_unsigned_vec_var_6(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        let mut ys = ys.to_vec();
        let ys_old = ys.clone();
        let b = limbs_or_neg_neg_in_place_either(&mut xs, &mut ys);
        println!(
            "xs := {:?}; ys := {:?}; limbs_or_neg_neg_in_place_either(&mut xs, &mut ys) = {}; \
             xs = {:?}; ys = {:?}",
            xs_old, ys_old, b, xs, ys
        );
    }
}

fn demo_integer_or_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integers(gm).take(limit) {
        let x_old = x.clone();
        x |= y.clone();
        println!("x := {}; x |= {}; x = {}", x_old, y, x);
    }
}

fn demo_integer_or_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integers(gm).take(limit) {
        let x_old = x.clone();
        x |= &y;
        println!("x := {}; x |= &{}; x = {}", x_old, y, x);
    }
}

fn demo_integer_or(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} | {} = {}", x_old, y_old, x | y);
    }
}

fn demo_integer_or_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        let x_old = x.clone();
        println!("{} | &{} = {}", x_old, y, x | &y);
    }
}

fn demo_integer_or_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        let y_old = y.clone();
        println!("&{} | {} = {}", x, y_old, &x | y);
    }
}

fn demo_integer_or_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        println!("&{} | &{} = {}", x, y, &x | &y);
    }
}

fn benchmark_limbs_neg_or_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_neg_or_limb(&[Limb], Limb)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_unsigned_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "Malachite",
            &mut (|(limbs, limb)| no_out!(limbs_neg_or_limb(&limbs, limb))),
        )],
    );
}

fn benchmark_limbs_neg_or_limb_to_out(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_neg_or_limb_to_out(&mut [Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref in_limbs, _)| in_limbs.len()),
        "in_limbs.len()",
        &mut [(
            "Malachite",
            &mut (|(mut out, in_limbs, limb)| limbs_neg_or_limb_to_out(&mut out, &in_limbs, limb)),
        )],
    );
}

fn benchmark_limbs_neg_or_limb_in_place(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_neg_or_limb_in_place(&mut [Limb], Limb)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_unsigned_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "Malachite",
            &mut (|(mut limbs, limb)| limbs_neg_or_limb_in_place(&mut limbs, limb)),
        )],
    );
}

fn benchmark_limbs_pos_or_neg_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_pos_or_neg_limb(&[Limb], SignedLimb)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_positive_unsigned_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "Malachite",
            &mut (|(limbs, i)| no_out!(limbs_pos_or_neg_limb(&limbs, i))),
        )],
    );
}

fn benchmark_limbs_neg_or_neg_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_neg_or_neg_limb(&[Limb], SignedLimb)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_positive_unsigned_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "Malachite",
            &mut (|(limbs, i)| no_out!(limbs_neg_or_neg_limb(&limbs, i))),
        )],
    );
}

fn benchmark_limbs_or_pos_neg(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_or_pos_neg(&[u32], &[u32])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_6(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _)| xs.len()),
        "xs.len()",
        &mut [(
            "Malachite",
            &mut (|(ref xs, ref ys)| no_out!(limbs_or_pos_neg(xs, ys))),
        )],
    );
}

fn benchmark_limbs_or_pos_neg_to_out(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_or_pos_neg_to_out(&mut [u32], &[u32], &[u32])",
        BenchmarkType::Single,
        triples_of_limb_vec_var_6(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, _)| xs.len()),
        "xs.len()",
        &mut [(
            "Malachite",
            &mut (|(ref mut out, ref xs, ref ys)| limbs_or_pos_neg_to_out(out, xs, ys)),
        )],
    );
}

fn benchmark_limbs_slice_or_pos_neg_in_place_left(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "limbs_slice_or_pos_neg_in_place_left(&mut [u32], &[u32])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_6(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _)| xs.len()),
        "xs.len()",
        &mut [(
            "Malachite",
            &mut (|(ref mut xs, ref ys)| no_out!(limbs_slice_or_pos_neg_in_place_left(xs, ys))),
        )],
    );
}

fn benchmark_limbs_vec_or_pos_neg_in_place_left(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_vec_or_pos_neg_in_place_left(&Vec<u32>, &[u32])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_6(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _)| xs.len()),
        "xs.len()",
        &mut [(
            "Malachite",
            &mut (|(ref mut xs, ref ys)| limbs_vec_or_pos_neg_in_place_left(xs, ys)),
        )],
    );
}

fn benchmark_limbs_or_pos_neg_in_place_right(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_or_pos_neg_in_place_right(&[u32], &mut [u32])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_6(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _)| xs.len()),
        "xs.len()",
        &mut [(
            "Malachite",
            &mut (|(ref xs, ref mut ys)| limbs_or_pos_neg_in_place_right(xs, ys)),
        )],
    );
}

fn benchmark_limbs_or_neg_neg(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_or_neg_neg(&[Limb], &[Limb])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_6(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _)| xs.len()),
        "xs.len()",
        &mut [(
            "Malachite",
            &mut (|(ref xs, ref ys)| no_out!(limbs_or_neg_neg(xs, ys))),
        )],
    );
}

fn benchmark_limbs_or_neg_neg_to_out(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_or_neg_neg_to_out(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        triples_of_limb_vec_var_8(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, _)| xs.len()),
        "xs.len()",
        &mut [(
            "Malachite",
            &mut (|(ref mut out, ref xs, ref ys)| limbs_or_neg_neg_to_out(out, xs, ys)),
        )],
    );
}

fn benchmark_limbs_slice_or_neg_neg_in_place_left(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "limbs_slice_or_neg_neg_in_place_left(&mut [Limb], &[Limb])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_6(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _)| xs.len()),
        "xs.len()",
        &mut [(
            "Malachite",
            &mut (|(ref mut xs, ref ys)| limbs_slice_or_neg_neg_in_place_left(xs, ys)),
        )],
    );
}

fn benchmark_limbs_vec_or_neg_neg_in_place_left(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_vec_or_neg_neg_in_place_left(&mut Vec<Limb>, &[Limb])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_6(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _)| xs.len()),
        "xs.len()",
        &mut [(
            "Malachite",
            &mut (|(ref mut xs, ref ys)| limbs_vec_or_neg_neg_in_place_left(xs, ys)),
        )],
    );
}

fn benchmark_limbs_or_neg_neg_in_place_either(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_or_neg_neg_in_place_either(&mut [Limb], &mut [Limb])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_6(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _)| xs.len()),
        "xs.len()",
        &mut [(
            "Malachite",
            &mut (|(ref mut xs, ref mut ys)| no_out!(limbs_or_neg_neg_in_place_either(xs, ys))),
        )],
    );
}

fn benchmark_integer_or_assign_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Integer |= Integer",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref x, ref y))| {
            usize::exact_from(max(x.significant_bits(), y.significant_bits()))
        }),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("Malachite", &mut (|(_, (mut x, y))| x |= y)),
            ("rug", &mut (|((mut x, y), _)| x |= y)),
        ],
    );
}

fn benchmark_integer_or_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Integer |= Integer",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("Integer |= Integer", &mut (|(mut x, y)| no_out!(x |= y))),
            ("Integer |= &Integer", &mut (|(mut x, y)| no_out!(x |= &y))),
        ],
    );
}

fn benchmark_integer_or_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Integer | Integer",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref x, ref y))| {
            usize::exact_from(max(x.significant_bits(), y.significant_bits()))
        }),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("Malachite", &mut (|(_, (x, y))| no_out!(x | y))),
            ("rug", &mut (|((x, y), _)| no_out!(x | y))),
        ],
    );
}

fn benchmark_integer_or_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Integer | Integer",
        BenchmarkType::Algorithms,
        pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("default", &mut (|(ref x, ref y)| no_out!(x | y))),
            (
                "using bits explicitly",
                &mut (|(ref x, ref y)| no_out!(integer_or_alt_1(x, y))),
            ),
            (
                "using limbs explicitly",
                &mut (|(ref x, ref y)| no_out!(integer_or_alt_2(x, y))),
            ),
        ],
    );
}

fn benchmark_integer_or_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Integer | Integer",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("Integer | Integer", &mut (|(x, y)| no_out!(x | y))),
            ("Integer | &Integer", &mut (|(x, y)| no_out!(x | &y))),
            ("&Integer | Integer", &mut (|(x, y)| no_out!(&x | y))),
            ("&Integer | &Integer", &mut (|(x, y)| no_out!(&x | &y))),
        ],
    );
}
