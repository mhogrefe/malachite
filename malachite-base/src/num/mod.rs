pub mod arithmetic {
    pub mod abs;
    pub mod checked_abs;
    pub mod checked_neg;
    pub mod integers;
    pub mod mod_add;
    pub mod mod_is_reduced;
    pub mod mod_mul;
    pub mod mod_neg;
    pub mod mod_power_of_two_add;
    pub mod mod_power_of_two_is_reduced;
    pub mod mod_power_of_two_mul;
    pub mod mod_power_of_two_neg;
    pub mod mod_power_of_two_sub;
    pub mod mod_sub;
    pub mod neg;
    pub mod overflowing_abs;
    pub mod overflowing_add;
    pub mod overflowing_mul;
    pub mod overflowing_neg;
    pub mod overflowing_sub;
    pub mod power_of_two;
    pub mod signeds;
    pub mod traits;
    pub mod unsigneds;
}
pub mod basic {
    pub mod integers;
    pub mod signeds;
    pub mod traits;
    pub mod unsigneds;
}
pub mod comparison {
    pub mod integers;
    pub mod signeds;
    pub mod traits;
    pub mod unsigneds;
}
pub mod conversion {
    pub mod integers;
    pub mod traits;
    pub mod unsigneds;
}
pub mod floats;
pub mod logic {
    pub mod integers;
    pub mod signeds;
    pub mod traits;
    pub mod unsigneds;
}
