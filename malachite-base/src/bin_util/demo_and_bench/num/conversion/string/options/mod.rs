use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    from_sci_string_options::register(runner);
    sci_size_options::register(runner);
    to_sci_options::register(runner);
}

mod from_sci_string_options;
mod sci_size_options;
mod to_sci_options;
