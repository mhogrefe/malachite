use common::DemoBenchRegistry;

pub mod abs;
pub mod log_two;
pub mod mod_is_reduced;
pub mod mod_neg;
pub mod mod_power_of_two_is_reduced;
pub mod mod_power_of_two_neg;
pub mod neg;
pub mod overflowing_abs;
pub mod overflowing_neg;
pub mod power_of_two;
pub mod saturating_abs;
pub mod saturating_neg;
pub mod sign;
pub mod wrapping_abs;
pub mod wrapping_neg;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    abs::register(registry);
    log_two::register(registry);
    mod_is_reduced::register(registry);
    mod_neg::register(registry);
    mod_power_of_two_neg::register(registry);
    mod_power_of_two_is_reduced::register(registry);
    neg::register(registry);
    overflowing_abs::register(registry);
    overflowing_neg::register(registry);
    power_of_two::register(registry);
    saturating_abs::register(registry);
    saturating_neg::register(registry);
    sign::register(registry);
    wrapping_abs::register(registry);
    wrapping_neg::register(registry);
}
