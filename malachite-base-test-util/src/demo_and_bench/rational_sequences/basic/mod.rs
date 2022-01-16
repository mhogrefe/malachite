use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    component_len::register(runner);
    is_empty::register(runner);
    is_finite::register(runner);
    iter::register(runner);
    len::register(runner);
}

mod component_len;
mod is_empty;
mod is_finite;
mod iter;
mod len;
