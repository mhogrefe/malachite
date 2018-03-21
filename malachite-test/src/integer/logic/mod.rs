use common::DemoBenchRegistry;

pub mod assign_bit;
pub mod clear_bit;
pub mod flip_bit;
pub mod from_twos_complement_limbs;
pub mod get_bit;
pub mod not;
pub mod set_bit;
pub mod significant_bits;
pub mod trailing_zeros;
pub mod twos_complement_limbs;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    assign_bit::register(registry);
    clear_bit::register(registry);
    flip_bit::register(registry);
    from_twos_complement_limbs::register(registry);
    get_bit::register(registry);
    not::register(registry);
    set_bit::register(registry);
    significant_bits::register(registry);
    trailing_zeros::register(registry);
    twos_complement_limbs::register(registry);
}
