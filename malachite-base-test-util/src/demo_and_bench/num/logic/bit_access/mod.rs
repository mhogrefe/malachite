use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    assign_bit::register(runner);
    clear_bit::register(runner);
    flip_bit::register(runner);
    get_bit::register(runner);
    set_bit::register(runner);
}

mod assign_bit;
mod clear_bit;
mod flip_bit;
mod get_bit;
mod set_bit;
