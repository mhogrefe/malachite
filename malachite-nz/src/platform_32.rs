pub type Limb = u32;
pub type HalfLimb = u16;
pub type DoubleLimb = u64;
pub type SignedLimb = i32;
pub type SignedHalfLimb = i16;
pub type SignedDoubleLimb = i64;
pub type FloatWithLimbWidth = f32;

pub const AORSMUL_FASTER_2AORSLSH: bool = true;
pub const AORSMUL_FASTER_3AORSLSH: bool = true;
pub const AORSMUL_FASTER_AORS_AORSLSH: bool = true;
pub const AORSMUL_FASTER_AORS_2AORSLSH: bool = true;

pub const MUL_TOOM22_THRESHOLD: usize = 106;
pub const MUL_TOOM33_THRESHOLD: usize = 77;
pub const MUL_TOOM44_THRESHOLD: usize = 544;
pub const MUL_TOOM6H_THRESHOLD: usize = 737;
pub const MUL_TOOM8H_THRESHOLD: usize = 1074;

pub const MUL_TOOM32_TO_TOOM43_THRESHOLD: usize = 91;
pub const MUL_TOOM32_TO_TOOM53_THRESHOLD: usize = 307;
pub const MUL_TOOM42_TO_TOOM53_THRESHOLD: usize = 353;
pub const MUL_TOOM42_TO_TOOM63_THRESHOLD: usize = 109;

pub const MUL_FFT_THRESHOLD: usize = 5608;

pub const DC_DIV_QR_THRESHOLD: usize = 6;
pub const DC_DIVAPPR_Q_THRESHOLD: usize = 109;
pub const MAYBE_DCP1_DIVAPPR: bool = true;
pub const INV_NEWTON_THRESHOLD: usize = 323;
pub const MU_DIV_QR_THRESHOLD: usize = 2248;
pub const INV_MULMOD_BNM1_THRESHOLD: usize = 68;
pub const MU_DIV_QR_SKEW_THRESHOLD: usize = 190;
