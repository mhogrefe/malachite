use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    floating_point_from_integer::register(runner);
    from_floating_point::register(runner);
    serde::register(runner);
    string::register(runner);
}

mod floating_point_from_integer;
mod from_floating_point;
mod serde;
mod string;
