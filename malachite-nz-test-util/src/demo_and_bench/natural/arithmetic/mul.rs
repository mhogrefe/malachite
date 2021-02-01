use malachite_base_test_util::bench::bucketers::pair_1_vec_len_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::unsigned_vec_unsigned_pair_gen;
use malachite_base_test_util::runner::Runner;
use malachite_nz::natural::arithmetic::mul::limb::{
    limbs_mul_limb, limbs_slice_mul_limb_in_place, limbs_vec_mul_limb_in_place,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_mul_limb);
    register_demo!(runner, demo_limbs_slice_mul_limb_in_place);
    register_demo!(runner, demo_limbs_vec_mul_limb_in_place);
    register_bench!(runner, benchmark_limbs_mul_limb);
    register_bench!(runner, benchmark_limbs_slice_mul_limb_in_place);
    register_bench!(runner, benchmark_limbs_vec_mul_limb_in_place);
}

fn demo_limbs_mul_limb(gm: GenMode, config: GenConfig, limit: usize) {
    for (xs, y) in unsigned_vec_unsigned_pair_gen()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "limbs_mul_limb({:?}, {}) = {:?}",
            xs,
            y,
            limbs_mul_limb(&xs, y)
        );
    }
}

//TODO limbs_mul_limb_with_carry_to_out

//TODO demo_limbs_mul_limb_to_out

//TODO demo_limbs_slice_mul_limb_with_carry_in_place

fn demo_limbs_slice_mul_limb_in_place(gm: GenMode, config: GenConfig, limit: usize) {
    for (xs, y) in unsigned_vec_unsigned_pair_gen()
        .get(gm, &config)
        .take(limit)
    {
        let xs_old = xs;
        let mut xs = xs_old.clone();
        let carry = limbs_slice_mul_limb_in_place(&mut xs, y);
        println!(
            "xs := {:?}; limbs_slice_mul_limb_in_place(&mut xs, {}) = {}; xs = {:?}",
            xs_old, y, carry, xs
        );
    }
}

fn demo_limbs_vec_mul_limb_in_place(gm: GenMode, config: GenConfig, limit: usize) {
    for (xs, y) in unsigned_vec_unsigned_pair_gen()
        .get(gm, &config)
        .take(limit)
    {
        let xs_old = xs;
        let mut xs = xs_old.clone();
        limbs_vec_mul_limb_in_place(&mut xs, y);
        println!(
            "xs := {:?}; limbs_vec_mul_limb_in_place(&mut xs, {}); xs = {:?}",
            xs_old, y, xs
        );
    }
}

fn benchmark_limbs_mul_limb(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_mul_limb(&[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, y)| no_out!(limbs_mul_limb(&xs, y)))],
    );
}

//TODO limbs_mul_limb_with_carry_to_out

//TODO demo_limbs_mul_limb_to_out

//TODO demo_limbs_slice_mul_limb_with_carry_in_place

fn benchmark_limbs_slice_mul_limb_in_place(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_slice_mul_limb_in_place(&mut [Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, y)| {
            no_out!(limbs_slice_mul_limb_in_place(&mut xs, y))
        })],
    );
}

fn benchmark_limbs_vec_mul_limb_in_place(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_vec_mul_limb_in_place(&mut Vec<Limb>, Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, y)| {
            limbs_vec_mul_limb_in_place(&mut xs, y)
        })],
    );
}
