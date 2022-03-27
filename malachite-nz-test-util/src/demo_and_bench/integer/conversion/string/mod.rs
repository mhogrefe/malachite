use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    from_sci_string::register(runner);
    from_string::register(runner);
    to_sci::register(runner);
    to_string::register(runner);
}

mod from_sci_string;
mod from_string;
mod to_sci;
mod to_string;
