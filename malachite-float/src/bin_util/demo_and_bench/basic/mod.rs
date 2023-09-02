use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    classification::register(runner);
    complexity::register(runner);
    constants::register(runner);
    get_and_set::register(runner);
    ulp::register(runner);
}

mod classification;
mod complexity;
mod constants;
mod get_and_set;
mod ulp;
