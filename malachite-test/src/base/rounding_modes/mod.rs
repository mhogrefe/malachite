use malachite_test::common::DemoBenchRegistry;

pub mod from_str;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    from_str::register(registry);
}
