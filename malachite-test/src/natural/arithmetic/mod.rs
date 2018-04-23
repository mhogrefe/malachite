use common::DemoBenchRegistry;

pub mod add;
pub mod add_mul;
pub mod add_mul_u32;
pub mod add_u32;
pub mod divisible_by_power_of_two;
pub mod even_odd;
pub mod is_power_of_two;
pub mod log_two;
pub mod mod_power_of_two;
pub mod mul;
pub mod mul_u32;
pub mod neg;
pub mod shl_i32;
pub mod shl_u32;
pub mod shr_i32;
pub mod shr_u32;
pub mod sub;
pub mod sub_mul;
pub mod sub_mul_u32;
pub mod sub_u32;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    add::register(registry);
    add_u32::register(registry);
    add_mul::register(registry);
    add_mul_u32::register(registry);
    divisible_by_power_of_two::register(registry);
    even_odd::register(registry);
    is_power_of_two::register(registry);
    log_two::register(registry);
    mod_power_of_two::register(registry);
    mul::register(registry);
    mul_u32::register(registry);
    neg::register(registry);
    shl_i32::register(registry);
    shl_u32::register(registry);
    shr_i32::register(registry);
    shr_u32::register(registry);
    sub::register(registry);
    sub_u32::register(registry);
    sub_mul::register(registry);
    sub_mul_u32::register(registry);
}
