use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    primes::register(runner);
}

mod primes;
