use common::GenerationMode;
use malachite_nz::integer::Integer;
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::orderings::{exhaustive_orderings, random_orderings};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{lex_pairs, random_pairs};
use rust_wheels::iterators::vecs::{exhaustive_vecs, random_vecs};
use std::cmp::Ordering;

type It = Iterator<Item = (Ordering, Vec<u32>)>;

pub fn exhaustive_inputs() -> Box<It> {
    Box::new(
        lex_pairs(exhaustive_vecs(exhaustive_u()), exhaustive_orderings())
            .map(|(limbs, sign)| (sign, limbs))
            .filter(|&(sign, ref limbs)| {
                limbs.iter().all(|&limb| limb == 0) == (sign == Ordering::Equal)
            }),
    )
}

pub fn random_inputs(scale: u32) -> Box<It> {
    Box::new(
        random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_orderings(seed)),
            &(|seed| random_vecs(seed, scale, &(|seed_2| random_x(seed_2)))),
        ).filter(|&(sign, ref limbs)| {
            limbs.iter().all(|&limb| limb == 0) == (sign == Ordering::Equal)
        }),
    )
}

pub fn select_inputs(gm: GenerationMode) -> Box<It> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs(),
        GenerationMode::Random(scale) => random_inputs(scale),
    }
}

pub fn demo_integer_from_sign_and_limbs_le(gm: GenerationMode, limit: usize) {
    for (sign, limbs) in select_inputs(gm).take(limit) {
        println!(
            "from_sign_and_limbs_le({:?}, {:?}) = {:?}",
            sign,
            limbs,
            Integer::from_sign_and_limbs_le(sign, limbs.as_slice())
        );
    }
}

pub fn demo_integer_from_sign_and_limbs_be(gm: GenerationMode, limit: usize) {
    for (sign, limbs) in select_inputs(gm).take(limit) {
        println!(
            "from_sign_and_limbs_be({:?}, {:?}) = {:?}",
            sign,
            limbs,
            Integer::from_sign_and_limbs_be(sign, limbs.as_slice())
        );
    }
}

pub fn benchmark_integer_from_sign_and_limbs_le(gm: GenerationMode, limit: usize, file_name: &str) {
    println!(
        "benchmarking {} Integer::from_sign_and_limbs_le(&[u32])",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: select_inputs(gm),
        function_f: &(|(sign, limbs): (Ordering, Vec<u32>)| {
            Integer::from_sign_and_limbs_le(sign, limbs.as_slice())
        }),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, ref limbs)| limbs.len()),
        limit,
        f_name: "malachite",
        title: "Integer::from\\\\_sign\\\\_and\\\\_limbs\\\\_le(Ordering, \\\\&[u32])",
        x_axis_label: "xs.len()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_from_sign_and_limbs_be(gm: GenerationMode, limit: usize, file_name: &str) {
    println!(
        "benchmarking {} Integer::from_sign_and_limbs_be(&[u32])",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: select_inputs(gm),
        function_f: &(|(sign, limbs): (Ordering, Vec<u32>)| {
            Integer::from_sign_and_limbs_be(sign, limbs.as_slice())
        }),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, ref limbs)| limbs.len()),
        limit,
        f_name: "malachite",
        title: "Integer::from\\\\_sign\\\\_and\\\\_limbs\\\\_be(Ordering, \\\\&[u32])",
        x_axis_label: "xs.len()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
