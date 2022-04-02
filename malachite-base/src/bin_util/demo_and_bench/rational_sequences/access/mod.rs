use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    get::register(runner);
    mutate::register(runner);
}

mod get;
mod mutate;
