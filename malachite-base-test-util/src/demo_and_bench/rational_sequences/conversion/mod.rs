use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    clone::register(runner);
    from_vec::register(runner);
    from_vecs::register(runner);
    to_vecs::register(runner);
}

mod clone;
mod from_vec;
mod from_vecs;
mod to_vecs;
