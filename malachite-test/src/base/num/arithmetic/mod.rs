use malachite_test::common::DemoBenchRegistry;

pub mod shl_round;
pub mod shr_round;
pub mod sign;
pub mod square;
pub mod wrapping_abs;
pub mod wrapping_add;
pub mod wrapping_add_mul;
pub mod wrapping_div;
pub mod wrapping_mul;
pub mod wrapping_neg;
pub mod wrapping_pow;
pub mod wrapping_square;
pub mod wrapping_sub;
pub mod wrapping_sub_mul;
pub mod xx_add_yy_is_zz;
pub mod xx_div_mod_y_is_qr;
pub mod xx_sub_yy_is_zz;
pub mod xxx_add_yyy_is_zzz;
pub mod xxx_sub_yyy_is_zzz;
pub mod xxxx_add_yyyy_is_zzzz;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    shl_round::register(registry);
    shr_round::register(registry);
    sign::register(registry);
    square::register(registry);
    wrapping_abs::register(registry);
    wrapping_add::register(registry);
    wrapping_add_mul::register(registry);
    wrapping_div::register(registry);
    wrapping_mul::register(registry);
    wrapping_neg::register(registry);
    wrapping_pow::register(registry);
    wrapping_square::register(registry);
    wrapping_sub::register(registry);
    wrapping_sub_mul::register(registry);
    xx_add_yy_is_zz::register(registry);
    xx_div_mod_y_is_qr::register(registry);
    xx_sub_yy_is_zz::register(registry);
    xxx_add_yyy_is_zzz::register(registry);
    xxx_sub_yyy_is_zzz::register(registry);
    xxxx_add_yyyy_is_zzzz::register(registry);
}
