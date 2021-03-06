use malachite_test::common::DemoBenchRegistry;

pub mod abs;
pub mod add;
pub mod add_mul;
pub mod div;
pub mod div_exact;
pub mod div_mod;
pub mod div_round;
pub mod divisible_by;
pub mod divisible_by_power_of_2;
pub mod eq_mod;
pub mod eq_mod_power_of_2;
pub mod mod_op;
pub mod mod_power_of_2;
pub mod mul;
pub mod parity;
pub mod pow;
pub mod power_of_2;
pub mod round_to_multiple;
pub mod round_to_multiple_of_power_of_2;
pub mod shl;
pub mod shl_round;
pub mod shr;
pub mod shr_round;
pub mod sign;
pub mod square;
pub mod sub;
pub mod sub_mul;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    abs::register(registry);
    add::register(registry);
    add_mul::register(registry);
    div::register(registry);
    div_exact::register(registry);
    div_mod::register(registry);
    div_round::register(registry);
    divisible_by::register(registry);
    divisible_by_power_of_2::register(registry);
    eq_mod::register(registry);
    eq_mod_power_of_2::register(registry);
    mod_op::register(registry);
    mod_power_of_2::register(registry);
    mul::register(registry);
    parity::register(registry);
    pow::register(registry);
    power_of_2::register(registry);
    round_to_multiple::register(registry);
    round_to_multiple_of_power_of_2::register(registry);
    shl::register(registry);
    shl_round::register(registry);
    shr::register(registry);
    shr_round::register(registry);
    sign::register(registry);
    square::register(registry);
    sub::register(registry);
    sub_mul::register(registry);
}
