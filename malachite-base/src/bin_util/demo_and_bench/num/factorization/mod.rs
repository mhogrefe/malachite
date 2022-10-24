use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    primes::register(runner);
    prime_sieve::register(runner);
}

mod prime_sieve;
mod primes;
