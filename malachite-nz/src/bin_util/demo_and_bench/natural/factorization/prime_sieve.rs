use malachite_base::test_util::bench::bucketers::unsigned_direct_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_gen_var_26;
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::factorization::prime_sieve::{
    limbs_prime_sieve, limbs_prime_sieve_size,
};
use malachite_nz::test_util::natural::factorization::prime_sieve::{
    limbs_prime_sieve_naive_1, limbs_prime_sieve_naive_2,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_prime_sieve);
    register_bench!(runner, benchmark_limbs_prime_sieve_algorithms);
}

fn demo_limbs_prime_sieve(gm: GenMode, config: GenConfig, limit: usize) {
    for n in unsigned_gen_var_26().get(gm, &config).take(limit) {
        let len = limbs_prime_sieve_size(n);
        let mut sieve = vec![0; len];
        limbs_prime_sieve(&mut sieve, n);
        print!("limbs_prime_sieve({:}): ", n);
        let mut first = true;
        for s in sieve {
            if first {
                first = false;
            } else {
                print!(", ");
            }
            print!("{:b}", s);
        }
        println!();
    }
}

fn benchmark_limbs_prime_sieve_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_prime_sieve(&mut [Limb], u64)",
        BenchmarkType::Algorithms,
        unsigned_gen_var_26().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| {
                let len = limbs_prime_sieve_size(n);
                let mut sieve = vec![0; len];
                limbs_prime_sieve(&mut sieve, n);
            }),
            ("test each prime separately", &mut |n| {
                let len = limbs_prime_sieve_size(n);
                let mut sieve = vec![0; len];
                limbs_prime_sieve_naive_1(&mut sieve, n);
            }),
            ("naive sieve", &mut |n| {
                let len = limbs_prime_sieve_size(n);
                let mut sieve = vec![0; len];
                limbs_prime_sieve_naive_2(&mut sieve, n);
            }),
        ],
    );
}
