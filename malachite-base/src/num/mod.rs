pub fn get_lower(val: u64) -> u32 {
    val as u32
}

pub fn get_upper(val: u64) -> u32 {
    (val >> 32) as u32
}

pub fn make_u64(upper: u32, lower: u32) -> u64 {
    u64::from(upper) << 32 | u64::from(lower)
}
