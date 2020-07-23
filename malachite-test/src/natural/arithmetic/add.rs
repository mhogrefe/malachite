use std::cmp::{max, min};

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_nz::natural::arithmetic::add::{
    _limbs_add_to_out_aliased, limbs_add, limbs_add_greater, limbs_add_greater_to_out,
    limbs_add_limb, limbs_add_limb_to_out, limbs_add_same_length_to_out, limbs_add_to_out,
    limbs_slice_add_greater_in_place_left, limbs_slice_add_in_place_either,
    limbs_slice_add_limb_in_place, limbs_slice_add_same_length_in_place_left,
    limbs_vec_add_in_place_either, limbs_vec_add_in_place_left, limbs_vec_add_limb_in_place,
};
use malachite_nz::platform::Limb;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    pairs_of_nonempty_unsigned_vec_and_unsigned, pairs_of_unsigned_vec,
    pairs_of_unsigned_vec_and_unsigned, pairs_of_unsigned_vec_var_1, pairs_of_unsigned_vec_var_3,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1,
    triples_of_unsigned_vec_usize_and_unsigned_vec_var_1, triples_of_unsigned_vec_var_3,
    triples_of_unsigned_vec_var_4, triples_of_unsigned_vec_var_9,
};
use malachite_test::inputs::natural::{
    nrm_pairs_of_naturals, pairs_of_naturals, rm_pairs_of_naturals,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_add_limb);
    register_demo!(registry, demo_limbs_add_limb_to_out);
    register_demo!(registry, demo_limbs_slice_add_limb_in_place);
    register_demo!(registry, demo_limbs_vec_add_limb_in_place);
    register_demo!(registry, demo_limbs_add_greater);
    register_demo!(registry, demo_limbs_add);
    register_demo!(registry, demo_limbs_add_same_length_to_out);
    register_demo!(registry, demo_limbs_add_greater_to_out);
    register_demo!(registry, demo_limbs_add_to_out);
    register_demo!(registry, demo_limbs_add_to_out_aliased);
    register_demo!(registry, demo_limbs_slice_add_same_length_in_place_left);
    register_demo!(registry, demo_limbs_slice_add_greater_in_place_left);
    register_demo!(registry, demo_limbs_vec_add_in_place_left);
    register_demo!(registry, demo_limbs_slice_add_in_place_either);
    register_demo!(registry, demo_limbs_vec_add_in_place_either);
    register_demo!(registry, demo_natural_add_assign);
    register_demo!(registry, demo_natural_add_assign_ref);
    register_demo!(registry, demo_natural_add);
    register_demo!(registry, demo_natural_add_val_ref);
    register_demo!(registry, demo_natural_add_ref_val);
    register_demo!(registry, demo_natural_add_ref_ref);
    register_bench!(registry, Small, benchmark_limbs_add_limb);
    register_bench!(registry, Small, benchmark_limbs_add_limb_to_out);
    register_bench!(registry, Small, benchmark_limbs_slice_add_limb_in_place);
    register_bench!(registry, Small, benchmark_limbs_vec_add_limb_in_place);
    register_bench!(registry, Small, benchmark_limbs_add_greater);
    register_bench!(registry, Small, benchmark_limbs_add);
    register_bench!(registry, Small, benchmark_limbs_add_same_length_to_out);
    register_bench!(registry, Small, benchmark_limbs_add_greater_to_out);
    register_bench!(registry, Small, benchmark_limbs_add_to_out);
    register_bench!(registry, Small, benchmark_limbs_add_to_out_aliased);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_slice_add_same_length_in_place_left
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_slice_add_greater_in_place_left
    );
    register_bench!(registry, Small, benchmark_limbs_vec_add_in_place_left);
    register_bench!(registry, Small, benchmark_limbs_slice_add_in_place_either);
    register_bench!(registry, Small, benchmark_limbs_vec_add_in_place_either);
    register_bench!(
        registry,
        Large,
        benchmark_natural_add_assign_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_add_assign_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_add_library_comparison);
    register_bench!(registry, Large, benchmark_natural_add_evaluation_strategy);
}

fn demo_limbs_add_limb(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_unsigned_vec_and_unsigned(gm).take(limit) {
        println!(
            "limbs_add_limb({:?}, {}) = {:?}",
            limbs,
            limb,
            limbs_add_limb(&limbs, limb)
        );
    }
}

fn demo_limbs_add_limb_to_out(gm: GenerationMode, limit: usize) {
    for (out, in_limbs, limb) in
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1(gm).take(limit)
    {
        let mut out = out.to_vec();
        let out_old = out.clone();
        let carry = limbs_add_limb_to_out(&mut out, &in_limbs, limb);
        println!(
            "out := {:?}; limbs_add_limb_to_out(&mut out, {:?}, {}) = {}; \
             out = {:?}",
            out_old, in_limbs, limb, carry, out
        );
    }
}

fn demo_limbs_slice_add_limb_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_unsigned_vec_and_unsigned::<Limb>(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let limbs_old = limbs.clone();
        let carry = limbs_slice_add_limb_in_place(&mut limbs, limb);
        println!(
            "limbs := {:?}; limbs_slice_add_limb_in_place(&mut limbs, {}) = {}; limbs = {:?}",
            limbs_old, limb, carry, limbs
        );
    }
}

fn demo_limbs_vec_add_limb_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_nonempty_unsigned_vec_and_unsigned(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let limbs_old = limbs.clone();
        limbs_vec_add_limb_in_place(&mut limbs, limb);
        println!(
            "limbs := {:?}; limbs_vec_add_limb_in_place(&mut limbs, {}); limbs = {:?}",
            limbs_old, limb, limbs
        );
    }
}

fn demo_limbs_add_greater(gm: GenerationMode, limit: usize) {
    for (xs, ys) in pairs_of_unsigned_vec_var_3(gm).take(limit) {
        println!(
            "limbs_add_greater({:?}, {:?}) = {:?}",
            xs,
            ys,
            limbs_add_greater(&xs, &ys)
        );
    }
}

fn demo_limbs_add(gm: GenerationMode, limit: usize) {
    for (xs, ys) in pairs_of_unsigned_vec(gm).take(limit) {
        println!("limbs_add({:?}, {:?}) = {:?}", xs, ys, limbs_add(&xs, &ys));
    }
}

fn demo_limbs_add_same_length_to_out(gm: GenerationMode, limit: usize) {
    for (xs, ys, zs) in triples_of_unsigned_vec_var_3(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        let carry = limbs_add_same_length_to_out(&mut xs, &ys, &zs);
        println!(
            "out := {:?}; limbs_add_same_length_to_out(&mut out, {:?}, {:?}) = \
             {}; out = {:?}",
            xs_old, ys, zs, carry, xs
        );
    }
}

fn demo_limbs_add_greater_to_out(gm: GenerationMode, limit: usize) {
    for (xs, ys, zs) in triples_of_unsigned_vec_var_9(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        let carry = limbs_add_greater_to_out(&mut xs, &ys, &zs);
        println!(
            "out := {:?}; limbs_add_greater_to_out(&mut out, {:?}, {:?}) = \
             {}; out = {:?}",
            xs_old, ys, zs, carry, xs
        );
    }
}

fn demo_limbs_add_to_out(gm: GenerationMode, limit: usize) {
    for (xs, ys, zs) in triples_of_unsigned_vec_var_4(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        let carry = limbs_add_to_out(&mut xs, &ys, &zs);
        println!(
            "out := {:?}; limbs_add_to_out(&mut out, {:?}, {:?}) = {}; \
             out = {:?}",
            xs_old, ys, zs, carry, xs
        );
    }
}

fn demo_limbs_add_to_out_aliased(gm: GenerationMode, limit: usize) {
    for (xs, in_size, ys) in triples_of_unsigned_vec_usize_and_unsigned_vec_var_1(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        let carry = _limbs_add_to_out_aliased(&mut xs, in_size, &ys);
        println!(
            "xs := {:?}; _limbs_add_to_out_aliased(&mut xs, {}, {:?}) = {}; xs = {:?}",
            xs_old, in_size, ys, carry, xs
        );
    }
}

fn demo_limbs_slice_add_same_length_in_place_left(gm: GenerationMode, limit: usize) {
    for (xs, ys) in pairs_of_unsigned_vec_var_1(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        let carry = limbs_slice_add_same_length_in_place_left(&mut xs, &ys);
        println!(
            "xs := {:?}; limbs_slice_add_same_length_in_place_left(&mut xs, {:?}) = {}; xs = {:?}",
            xs_old, ys, carry, xs
        );
    }
}

fn demo_limbs_slice_add_greater_in_place_left(gm: GenerationMode, limit: usize) {
    for (xs, ys) in pairs_of_unsigned_vec_var_3(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        let carry = limbs_slice_add_greater_in_place_left(&mut xs, &ys);
        println!(
            "xs := {:?}; limbs_slice_add_greater_in_place_left(&mut xs, {:?}) = {}; xs = {:?}",
            xs_old, ys, carry, xs
        );
    }
}

fn demo_limbs_vec_add_in_place_left(gm: GenerationMode, limit: usize) {
    for (xs, ys) in pairs_of_unsigned_vec(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        limbs_vec_add_in_place_left(&mut xs, &ys);
        println!(
            "xs := {:?}; limbs_vec_add_in_place_left(&mut xs, {:?}); xs = {:?}",
            xs_old, ys, xs
        );
    }
}

fn demo_limbs_slice_add_in_place_either(gm: GenerationMode, limit: usize) {
    for (xs, ys) in pairs_of_unsigned_vec(gm).take(limit) {
        let mut xs = xs.to_vec();
        let mut ys = ys.to_vec();
        let xs_old = xs.clone();
        let ys_old = ys.clone();
        let result = limbs_slice_add_in_place_either(&mut xs, &mut ys);
        println!(
            "xs := {:?}; ys := {:?}; limbs_slice_add_in_place_either(&mut xs, &mut ys) = \
             {:?}; xs = {:?}; ys = {:?}",
            xs_old, ys_old, result, xs, ys
        );
    }
}

fn demo_limbs_vec_add_in_place_either(gm: GenerationMode, limit: usize) {
    for (xs, ys) in pairs_of_unsigned_vec(gm).take(limit) {
        let mut xs = xs.to_vec();
        let mut ys = ys.to_vec();
        let xs_old = xs.clone();
        let ys_old = ys.clone();
        let right = limbs_vec_add_in_place_either(&mut xs, &mut ys);
        println!(
            "xs := {:?}; ys := {:?}; limbs_vec_add_in_place_either(&mut xs, &mut ys) = {}; \
             xs = {:?}; ys = {:?}",
            xs_old, ys_old, right, xs, ys
        );
    }
}

fn demo_natural_add_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        x += y.clone();
        println!("x := {}; x += {}; x = {}", x_old, y, x);
    }
}

fn demo_natural_add_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        x += &y;
        println!("x := {}; x += &{}; x = {}", x_old, y, x);
    }
}

fn demo_natural_add(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} + {} = {}", x_old, y_old, x + y);
    }
}

fn demo_natural_add_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        println!("{} + &{} = {}", x_old, y, x + &y);
    }
}

fn demo_natural_add_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let y_old = y.clone();
        println!("&{} + {} = {}", x, y_old, &x + y);
    }
}

fn demo_natural_add_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        println!("&{} + &{} = {}", x, y, &x + &y);
    }
}

fn benchmark_limbs_add_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_add_limb(&[Limb], Limb)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(limbs, limb)| no_out!(limbs_add_limb(&limbs, limb))),
        )],
    );
}

fn benchmark_limbs_add_limb_to_out(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_add_limb_to_out(&mut [Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref in_limbs, _)| in_limbs.len()),
        "in_limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut out, in_limbs, limb)| {
                no_out!(limbs_add_limb_to_out(&mut out, &in_limbs, limb))
            }),
        )],
    );
}

fn benchmark_limbs_slice_add_limb_in_place(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_slice_add_limb_in_place(&mut [Limb], Limb)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut limbs, limb)| no_out!(limbs_slice_add_limb_in_place(&mut limbs, limb))),
        )],
    );
}

fn benchmark_limbs_vec_add_limb_in_place(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_vec_add_limb_in_place(&mut Vec<Limb>, Limb)",
        BenchmarkType::Single,
        pairs_of_nonempty_unsigned_vec_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut limbs, limb)| limbs_vec_add_limb_in_place(&mut limbs, limb)),
        )],
    );
}

fn benchmark_limbs_add_greater(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_add_greater(&[Limb], &[Limb])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _)| xs.len()),
        "xs.len()",
        &mut [(
            "malachite",
            &mut (|(xs, ys)| no_out!(limbs_add_greater(&xs, &ys))),
        )],
    );
}

fn benchmark_limbs_add(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_add(&[Limb], &[Limb])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys)| max(xs.len(), ys.len())),
        "max(xs.len(), ys.len())",
        &mut [("malachite", &mut (|(xs, ys)| no_out!(limbs_add(&xs, &ys))))],
    );
}

fn benchmark_limbs_add_same_length_to_out(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_add_same_length_to_out(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        triples_of_unsigned_vec_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, _)| xs.len()),
        "xs.len() = ys.len()",
        &mut [(
            "malachite",
            &mut (|(mut xs, ys, zs)| no_out!(limbs_add_same_length_to_out(&mut xs, &ys, &zs))),
        )],
    );
}

fn benchmark_limbs_add_greater_to_out(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_add_greater_to_out(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        triples_of_unsigned_vec_var_9(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, _)| xs.len()),
        "xs.len()",
        &mut [(
            "malachite",
            &mut (|(mut out, xs, ys)| no_out!(limbs_add_greater_to_out(&mut out, &xs, &ys))),
        )],
    );
}

fn benchmark_limbs_add_to_out(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_add_to_out(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        triples_of_unsigned_vec_var_4(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| max(xs.len(), ys.len())),
        "max(xs.len(), ys.len())",
        &mut [(
            "malachite",
            &mut (|(mut out, xs, ys)| no_out!(limbs_add_to_out(&mut out, &xs, &ys))),
        )],
    );
}

fn benchmark_limbs_add_to_out_aliased(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "_limbs_add_to_out_aliased(&[Limb], &[Limb])",
        BenchmarkType::Single,
        triples_of_unsigned_vec_usize_and_unsigned_vec_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _, _)| xs.len()),
        "xs.len()",
        &mut [(
            "malachite",
            &mut (|(mut xs, in_size, ys)| {
                no_out!(_limbs_add_to_out_aliased(&mut xs, in_size, &ys))
            }),
        )],
    );
}

fn benchmark_limbs_slice_add_same_length_in_place_left(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_slice_add_same_length_in_place_left(&mut [Limb], &[Limb])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _)| xs.len()),
        "xs.len() = ys.len()",
        &mut [(
            "malachite",
            &mut (|(mut xs, ys)| no_out!(limbs_slice_add_same_length_in_place_left(&mut xs, &ys))),
        )],
    );
}

fn benchmark_limbs_slice_add_greater_in_place_left(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_slice_add_greater_in_place_left(&mut [Limb], &[Limb])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _)| xs.len()),
        "xs.len()",
        &mut [(
            "malachite",
            &mut (|(mut xs, ys)| no_out!(limbs_slice_add_greater_in_place_left(&mut xs, &ys))),
        )],
    );
}

fn benchmark_limbs_vec_add_in_place_left(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_vec_add_in_place_left(&Vec<Limb>, &[Limb])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys)| min(xs.len(), ys.len())),
        "min(xs.len(), ys.len())",
        &mut [(
            "malachite",
            &mut (|(mut xs, ys)| no_out!(limbs_vec_add_in_place_left(&mut xs, &ys))),
        )],
    );
}

fn benchmark_limbs_slice_add_in_place_either(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_slice_add_in_place_either(&mut [Limb], &mut [Limb])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys)| min(xs.len(), ys.len())),
        "min(xs.len(), ys.len())",
        &mut [(
            "malachite",
            &mut (|(mut xs, mut ys)| no_out!(limbs_slice_add_in_place_either(&mut xs, &mut ys))),
        )],
    );
}

fn benchmark_limbs_vec_add_in_place_either(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_vec_add_in_place_either(&mut [Limb], &mut [Limb])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys)| min(xs.len(), ys.len())),
        "min(xs.len(), ys.len())",
        &mut [(
            "malachite",
            &mut (|(mut xs, mut ys)| no_out!(limbs_vec_add_in_place_either(&mut xs, &mut ys))),
        )],
    );
}

fn benchmark_natural_add_assign_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural += Natural",
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
            ("malachite", &mut (|(_, (mut x, y))| x += y)),
            ("rug", &mut (|((mut x, y), _)| x += y)),
        ],
    );
}

fn benchmark_natural_add_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural += Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("Natural += Natural", &mut (|(mut x, y)| no_out!(x += y))),
            ("Natural += &Natural", &mut (|(mut x, y)| no_out!(x += &y))),
        ],
    );
}

fn benchmark_natural_add_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "Natural + Natural",
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
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x + y))),
            ("num", &mut (|((x, y), _, _)| no_out!(x + y))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x + y))),
        ],
    );
}

fn benchmark_natural_add_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "Natural + Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("Natural + Natural", &mut (|(x, y)| no_out!(x + y))),
            ("Natural + &Natural", &mut (|(x, y)| no_out!(x + &y))),
            ("&Natural + Natural", &mut (|(x, y)| no_out!(&x + y))),
            ("&Natural + &Natural", &mut (|(x, y)| no_out!(&x + &y))),
        ],
    );
}
