use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    convergents::register(runner);
    from_continued_fraction::register(runner);
    to_continued_fraction::register(runner);
}

mod convergents;
mod from_continued_fraction;
mod to_continued_fraction;
