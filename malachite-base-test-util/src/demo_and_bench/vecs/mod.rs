use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    vec_delete_left::register(runner);
    vec_pad_left::register(runner);
}

mod vec_delete_left;
mod vec_pad_left;
