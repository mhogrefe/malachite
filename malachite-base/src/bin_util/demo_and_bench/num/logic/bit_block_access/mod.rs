use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    assign_bits::register(runner);
    get_bits::register(runner);
}

mod assign_bits;
mod get_bits;
