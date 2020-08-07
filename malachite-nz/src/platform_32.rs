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

pub const MULLO_BASECASE_THRESHOLD: usize = 0;
pub const MULLO_DC_THRESHOLD: usize = 216;
pub const MULLO_MUL_N_THRESHOLD: usize = 100_000;

pub const BINV_NEWTON_THRESHOLD: usize = 3_264;
pub const DC_BDIV_QR_THRESHOLD: usize = 329;
pub const MU_BDIV_QR_THRESHOLD: usize = 50_000;
pub const DC_BDIV_Q_THRESHOLD: usize = 373;
pub const MU_BDIV_Q_THRESHOLD: usize = 2_390;

pub const MOD_1_NORM_THRESHOLD: usize = 0;
pub const MOD_1_UNNORM_THRESHOLD: usize = 0;
pub const MOD_1_1P_METHOD: bool = true;
pub const MOD_1N_TO_MOD_1_1_THRESHOLD: usize = 3;
pub const MOD_1U_TO_MOD_1_1_THRESHOLD: usize = 3;
pub const MOD_1_1_TO_MOD_1_2_THRESHOLD: usize = 15;
pub const MOD_1_2_TO_MOD_1_4_THRESHOLD: usize = 43;

pub const BMOD_1_TO_MOD_1_THRESHOLD: usize = 31;

pub const SQR_TOOM2_THRESHOLD: usize = 222;
pub const SQR_TOOM3_THRESHOLD: usize = 205;
pub const SQR_TOOM4_THRESHOLD: usize = 1170;
pub const SQR_TOOM6_THRESHOLD: usize = 512;
pub const SQR_TOOM8_THRESHOLD: usize = 644;
