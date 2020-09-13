use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    char_to_contiguous_range::register(runner);
    contiguous_range_to_char::register(runner);
    crement::register(runner);
}

pub mod char_to_contiguous_range;
pub mod contiguous_range_to_char;
#[allow(clippy::module_inception)]
pub mod crement;
