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

pub const MUL_TOOM22_THRESHOLD: usize = 106;
pub const MUL_TOOM33_THRESHOLD: usize = 62;
pub const MUL_TOOM44_THRESHOLD: usize = 165;
pub const MUL_TOOM6H_THRESHOLD: usize = 624;
pub const MUL_TOOM8H_THRESHOLD: usize = 348;

pub const MUL_TOOM32_TO_TOOM43_THRESHOLD: usize = 96;
pub const MUL_TOOM32_TO_TOOM53_THRESHOLD: usize = 307;
pub const MUL_TOOM42_TO_TOOM53_THRESHOLD: usize = 314;
pub const MUL_TOOM42_TO_TOOM63_THRESHOLD: usize = 100;

pub const MUL_FFT_THRESHOLD: usize = 4505;

pub const DC_DIV_QR_THRESHOLD: usize = 105;
pub const DC_DIVAPPR_Q_THRESHOLD: usize = 211;
pub const MAYBE_DCP1_DIVAPPR: bool = true;
pub const INV_NEWTON_THRESHOLD: usize = 789;
pub const MU_DIV_QR_THRESHOLD: usize = 2211;
pub const INV_MULMOD_BNM1_THRESHOLD: usize = 58;
pub const MU_DIV_QR_SKEW_THRESHOLD: usize = 233;

pub const MU_DIVAPPR_Q_THRESHOLD: usize = 2494;
pub const FUDGE: usize = 280;

pub const MULLO_BASECASE_THRESHOLD: usize = 0;
pub const MULLO_DC_THRESHOLD: usize = 62;
pub const MULLO_MUL_N_THRESHOLD: usize = 8_907;

pub const BINV_NEWTON_THRESHOLD: usize = 224;
pub const DC_BDIV_QR_THRESHOLD: usize = 44;
pub const DC_BDIV_Q_THRESHOLD: usize = 104;
pub const MU_BDIV_Q_THRESHOLD: usize = 1_442;
