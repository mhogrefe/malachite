use malachite_base_test_util::bench::bucketers::{
    pair_1_vec_len_bucketer, pair_sum_vec_len_bucketer, quadruple_2_vec_len_bucketer,
    triple_1_vec_len_bucketer, triple_2_vec_len_bucketer,
};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{
    large_type_gen_var_1, unsigned_vec_pair_gen_var_1, unsigned_vec_pair_gen_var_2,
    unsigned_vec_triple_gen_var_1, unsigned_vec_unsigned_pair_gen,
    unsigned_vec_unsigned_unsigned_triple_gen, unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1,
};
use malachite_base_test_util::runner::Runner;
use malachite_nz::natural::arithmetic::mul::limb::{
    limbs_mul_limb, limbs_mul_limb_to_out, limbs_mul_limb_with_carry_to_out,
    limbs_slice_mul_limb_in_place, limbs_slice_mul_limb_with_carry_in_place,
    limbs_vec_mul_limb_in_place,
};
use malachite_nz::natural::arithmetic::mul::{
    limbs_mul, limbs_mul_greater, limbs_mul_same_length_to_out,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_mul_limb);
    register_demo!(runner, demo_limbs_mul_limb_with_carry_to_out);
    register_demo!(runner, demo_limbs_mul_limb_to_out);
    register_demo!(runner, demo_limbs_slice_mul_limb_with_carry_in_place);
    register_demo!(runner, demo_limbs_slice_mul_limb_in_place);
    register_demo!(runner, demo_limbs_vec_mul_limb_in_place);
    register_demo!(runner, demo_limbs_mul_greater);
    register_demo!(runner, demo_limbs_mul);
    register_demo!(runner, demo_limbs_mul_same_length_to_out);
    register_bench!(runner, benchmark_limbs_mul_limb);
    register_bench!(runner, benchmark_limbs_mul_limb_with_carry_to_out);
    register_bench!(runner, benchmark_limbs_mul_limb_to_out);
    register_bench!(runner, benchmark_limbs_slice_mul_limb_with_carry_in_place);
    register_bench!(runner, benchmark_limbs_slice_mul_limb_in_place);
    register_bench!(runner, benchmark_limbs_vec_mul_limb_in_place);
    register_bench!(runner, benchmark_limbs_mul_greater);
    register_bench!(runner, benchmark_limbs_mul);
    register_bench!(runner, benchmark_limbs_mul_same_length_to_out);
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

fn demo_limbs_mul_limb_with_carry_to_out(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut out, xs, y, carry) in large_type_gen_var_1().get(gm, &config).take(limit) {
        let out_old = out.clone();
        let carry_out = limbs_mul_limb_with_carry_to_out(&mut out, &xs, y, carry);
        println!(
            "out := {:?}; limbs_mul_limb_with_carry_to_out(&mut out, {:?}, {}, {}) = {}; \
             out = {:?}",
            out_old, xs, y, carry, carry_out, out
        );
    }
}

fn demo_limbs_mul_limb_to_out(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut out, xs, y) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1()
        .get(gm, &config)
        .take(limit)
    {
        let out_old = out.clone();
        let carry = limbs_mul_limb_to_out(&mut out, &xs, y);
        println!(
            "out := {:?}; limbs_mul_limb_to_out(&mut out, {:?}, {}) = {}; out = {:?}",
            out_old, xs, y, carry, out
        );
    }
}

fn demo_limbs_slice_mul_limb_with_carry_in_place(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut xs, y, carry) in unsigned_vec_unsigned_unsigned_triple_gen()
        .get(gm, &config)
        .take(limit)
    {
        let xs_old = xs.clone();
        let carry_out = limbs_slice_mul_limb_with_carry_in_place(&mut xs, y, carry);
        println!(
            "xs := {:?}; limbs_slice_mul_limb_with_carry_in_place(&mut xs, {}, {}) = {}; xs = {:?}",
            xs_old, y, carry, carry_out, xs
        );
    }
}

fn demo_limbs_slice_mul_limb_in_place(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut xs, y) in unsigned_vec_unsigned_pair_gen()
        .get(gm, &config)
        .take(limit)
    {
        let xs_old = xs.clone();
        let carry = limbs_slice_mul_limb_in_place(&mut xs, y);
        println!(
            "xs := {:?}; limbs_slice_mul_limb_in_place(&mut xs, {}) = {}; xs = {:?}",
            xs_old, y, carry, xs
        );
    }
}

fn demo_limbs_vec_mul_limb_in_place(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut xs, y) in unsigned_vec_unsigned_pair_gen()
        .get(gm, &config)
        .take(limit)
    {
        let xs_old = xs.clone();
        limbs_vec_mul_limb_in_place(&mut xs, y);
        println!(
            "xs := {:?}; limbs_vec_mul_limb_in_place(&mut xs, {}); xs = {:?}",
            xs_old, y, xs
        );
    }
}

fn demo_limbs_mul_greater(gm: GenMode, config: GenConfig, limit: usize) {
    for (xs, ys) in unsigned_vec_pair_gen_var_1().get(gm, &config).take(limit) {
        println!(
            "limbs_mul_greater({:?}, {:?}) = {:?}",
            xs,
            ys,
            limbs_mul_greater(&xs, &ys)
        );
    }
}

fn demo_limbs_mul(gm: GenMode, config: GenConfig, limit: usize) {
    for (xs, ys) in unsigned_vec_pair_gen_var_2().get(gm, &config).take(limit) {
        println!("limbs_mul({:?}, {:?}) = {:?}", xs, ys, limbs_mul(&xs, &ys));
    }
}

fn demo_limbs_mul_same_length_to_out(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut out, xs, ys) in unsigned_vec_triple_gen_var_1().get(gm, &config).take(limit) {
        let out_old = out.clone();
        limbs_mul_same_length_to_out(&mut out, &xs, &ys);
        println!(
            "out := {:?}; limbs_mul_same_length_to_out(&mut out, {:?}, {:?}); \
             out = {:?}",
            out_old, xs, ys, out
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

fn benchmark_limbs_mul_limb_with_carry_to_out(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mul_limb_with_carry_to_out(&mut [Limb], &[Limb], Limb, Limb)",
        BenchmarkType::Single,
        large_type_gen_var_1().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &quadruple_2_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut out, xs, y, carry)| {
            no_out!(limbs_mul_limb_with_carry_to_out(&mut out, &xs, y, carry))
        })],
    );
}

fn benchmark_limbs_mul_limb_to_out(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_mul_limb_to_out(&mut [Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut out, xs, y)| {
            no_out!(limbs_mul_limb_to_out(&mut out, &xs, y))
        })],
    );
}

fn benchmark_limbs_slice_mul_limb_with_carry_in_place(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_slice_mul_limb_with_carry_in_place(&mut [Limb], Limb, Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_unsigned_triple_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, y, carry)| {
            no_out!(limbs_slice_mul_limb_with_carry_in_place(&mut xs, y, carry))
        })],
    );
}

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

fn benchmark_limbs_mul_greater(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_mul_greater(&[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_1().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_sum_vec_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(xs, ys)| {
            no_out!(limbs_mul_greater(&xs, &ys))
        })],
    );
}

fn benchmark_limbs_mul(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_mul(&[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_2().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_sum_vec_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(xs, ys)| no_out!(limbs_mul(&xs, &ys)))],
    );
}

fn benchmark_limbs_mul_same_length_to_out(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mul_same_length_to_out(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_triple_gen_var_1().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut out, xs, ys)| {
            limbs_mul_same_length_to_out(&mut out, &xs, &ys)
        })],
    );
}
