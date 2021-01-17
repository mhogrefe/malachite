use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    slice_leading_zeros::register(runner);
    slice_move_left::register(runner);
    slice_set_zero::register(runner);
    slice_test_zero::register(runner);
    slice_trailing_zeros::register(runner);
    split_into_chunks::register(runner);
}

mod slice_leading_zeros;
mod slice_move_left;
mod slice_set_zero;
mod slice_test_zero;
mod slice_trailing_zeros;
mod split_into_chunks;
