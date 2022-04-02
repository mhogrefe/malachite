use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    string_is_subset::register(runner);
    string_sort::register(runner);
    string_unique::register(runner);
}

mod string_is_subset;
mod string_sort;
mod string_unique;
