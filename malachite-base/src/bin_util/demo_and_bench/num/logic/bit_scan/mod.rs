use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    index_of_next_false_bit::register(runner);
    index_of_next_true_bit::register(runner);
}

mod index_of_next_false_bit;
mod index_of_next_true_bit;
