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

pub const MUL_TOOM22_THRESHOLD: usize = 118;
pub const MUL_TOOM33_THRESHOLD: usize = 101;
pub const MUL_TOOM44_THRESHOLD: usize = 530;
pub const MUL_TOOM6H_THRESHOLD: usize = 738;
pub const MUL_TOOM8H_THRESHOLD: usize = 984;

pub const MUL_TOOM32_TO_TOOM43_THRESHOLD: usize = 315;
pub const MUL_TOOM32_TO_TOOM53_THRESHOLD: usize = 307;
pub const MUL_TOOM42_TO_TOOM53_THRESHOLD: usize = 328;
pub const MUL_TOOM42_TO_TOOM63_THRESHOLD: usize = 295;

pub const MUL_FFT_THRESHOLD: usize = 5608;

pub const DC_DIV_QR_THRESHOLD: usize = 7;
pub const DC_DIVAPPR_Q_THRESHOLD: usize = 151;
pub const MAYBE_DCP1_DIVAPPR: bool = true;
pub const INV_NEWTON_THRESHOLD: usize = 618;
pub const MU_DIV_QR_THRESHOLD: usize = 2243;
pub const INV_MULMOD_BNM1_THRESHOLD: usize = 68;
pub const MU_DIV_QR_SKEW_THRESHOLD: usize = 233;

pub const MU_DIVAPPR_Q_THRESHOLD: usize = 2297;
pub const FUDGE: usize = 261;
