use common::DemoBenchRegistry;

pub mod add;
pub mod add_mul;
pub mod checked_sub;
pub mod checked_sub_mul;
pub mod div;
pub mod div_exact;
pub mod div_mod;
pub mod div_round;
pub mod divisible_by;
pub mod divisible_by_power_of_two;
pub mod eq_mod;
pub mod eq_mod_power_of_two;
pub mod is_power_of_two;
pub mod log_two;
pub mod mod_is_reduced;
pub mod mod_neg;
pub mod mod_op;
pub mod mod_power_of_two;
pub mod mod_power_of_two_add;
pub mod mod_power_of_two_is_reduced;
pub mod mod_power_of_two_neg;
pub mod mod_power_of_two_sub;
pub mod mul;
pub mod neg;
pub mod next_power_of_two;
pub mod parity;
pub mod power_of_two;
pub mod saturating_sub;
pub mod saturating_sub_mul;
pub mod shl_i;
pub mod shl_u;
pub mod shr_i;
pub mod shr_u;
pub mod sign;
pub mod sub;
pub mod sub_mul;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    add::register(registry);
    add_mul::register(registry);
    checked_sub::register(registry);
    checked_sub_mul::register(registry);
    div::register(registry);
    div_exact::register(registry);
    div_mod::register(registry);
    div_round::register(registry);
    divisible_by::register(registry);
    divisible_by_power_of_two::register(registry);
    eq_mod::register(registry);
    eq_mod_power_of_two::register(registry);
    is_power_of_two::register(registry);
    log_two::register(registry);
    mod_is_reduced::register(registry);
    mod_neg::register(registry);
    mod_op::register(registry);
    mod_power_of_two::register(registry);
    mod_power_of_two_add::register(registry);
    mod_power_of_two_is_reduced::register(registry);
    mod_power_of_two_neg::register(registry);
    mod_power_of_two_sub::register(registry);
    mul::register(registry);
    neg::register(registry);
    next_power_of_two::register(registry);
    parity::register(registry);
    power_of_two::register(registry);
    saturating_sub::register(registry);
    saturating_sub_mul::register(registry);
    shl_i::register(registry);
    shl_u::register(registry);
    shr_i::register(registry);
    shr_u::register(registry);
    sign::register(registry);
    sub::register(registry);
    sub_mul::register(registry);
}
