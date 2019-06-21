pub type Limb = u64;
pub type HalfLimb = u32;
pub type DoubleLimb = u128;
pub type SignedLimb = i64;
pub type SignedHalfLimb = i32;
pub type SignedDoubleLimb = i128;
pub type FloatWithLimbWidth = f64;

pub const AORSMUL_FASTER_2AORSLSH: bool = true;
pub const AORSMUL_FASTER_3AORSLSH: bool = true;
pub const AORSMUL_FASTER_AORS_AORSLSH: bool = true;
pub const AORSMUL_FASTER_AORS_2AORSLSH: bool = true;

pub const MUL_TOOM22_THRESHOLD: usize = 98;
pub const MUL_TOOM33_THRESHOLD: usize = 62;
pub const MUL_TOOM44_THRESHOLD: usize = 387;
pub const MUL_TOOM6H_THRESHOLD: usize = 526;
pub const MUL_TOOM8H_THRESHOLD: usize = 172;

pub const MUL_TOOM32_TO_TOOM43_THRESHOLD: usize = 63;
pub const MUL_TOOM32_TO_TOOM53_THRESHOLD: usize = 73;
pub const MUL_TOOM42_TO_TOOM53_THRESHOLD: usize = 249;
pub const MUL_TOOM42_TO_TOOM63_THRESHOLD: usize = 79;

pub const MUL_FFT_THRESHOLD: usize = 4480;
