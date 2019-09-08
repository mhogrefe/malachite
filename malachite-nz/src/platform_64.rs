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
pub const MUL_TOOM33_THRESHOLD: usize = 56;
pub const MUL_TOOM44_THRESHOLD: usize = 126;
pub const MUL_TOOM6H_THRESHOLD: usize = 594;
pub const MUL_TOOM8H_THRESHOLD: usize = 372;

pub const MUL_TOOM32_TO_TOOM43_THRESHOLD: usize = 64;
pub const MUL_TOOM32_TO_TOOM53_THRESHOLD: usize = 79;
pub const MUL_TOOM42_TO_TOOM53_THRESHOLD: usize = 102;
pub const MUL_TOOM42_TO_TOOM63_THRESHOLD: usize = 78;

pub const MUL_FFT_THRESHOLD: usize = 4060;

pub const DC_DIV_QR_THRESHOLD: usize = 13;
pub const DC_DIVAPPR_Q_THRESHOLD: usize = 221;
pub const MAYBE_DCP1_DIVAPPR: bool = true;
pub const INV_NEWTON_THRESHOLD: usize = 389;
pub const MU_DIV_QR_THRESHOLD: usize = 1956;
pub const INV_MULMOD_BNM1_THRESHOLD: usize = 56;
pub const MU_DIV_QR_SKEW_THRESHOLD: usize = 282;
