use common::DemoBenchRegistry;

pub mod char_to_contiguous_range;
pub mod contiguous_range_to_char;
pub mod crement;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    char_to_contiguous_range::register(registry);
    contiguous_range_to_char::register(registry);
    crement::register(registry);
}
