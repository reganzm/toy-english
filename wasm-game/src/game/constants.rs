//! 数值平衡常量
/// 每关初始生命（HUD 心形数量与此一致）
pub const STARTING_LIVES: u32 = 5;
/// 每关小怪数量：基础在 [SPAWN_BASE_MIN, SPAWN_BASE_MAX] 随机，再加句长与关卡加成
pub const SPAWN_BASE_MIN: usize = 50;
pub const SPAWN_BASE_MAX: usize = 100;
/// 超过该字符数后，每多 1 字符增加的小怪数（含空格）
pub const SPAWN_EXTRA_PER_CHAR: usize = 2;
pub const SPAWN_LEN_START: usize = 10;
pub const SPAWN_LEN_BONUS_CAP: usize = 55;
/// 每往后一关额外增加的数量
pub const SPAWN_PER_LEVEL_INDEX: usize = 8;
pub const SPAWN_HARD_CAP: usize = 220;
/// 小怪竖直下落速度倍率（`spawn` 里的 `speed` 乘此系数再用于每帧位移）
pub const MONSTER_DESCENT_SCALE: f64 = 0.5;
pub const SCORE_PER_CHAR: u32 = 10;
pub const SCORE_CLEAR: u32 = 80;
pub const PERFECT_BONUS: u32 = 120;
pub const FEVER_COMBO: u32 = 12;
pub const MAX_MULT: f64 = 2.25;
pub const COMBO_STEP: u32 = 5;
/// 单次发射数量上限（同时场上子弹总数也受此限制）
pub const MAX_BULLETS: u32 = 2;
pub const BULLET_COMBO_EVERY: u32 = 5;
/// 与 `draw.rs` 中枪 SVG 的 `translate`、枪口局部 Y 一致
pub const GUN_ANCHOR_X: f64 = 480.0;
pub const GUN_ANCHOR_Y: f64 = 519.0;
pub const GUN_MUZZLE_LOCAL_Y: f64 = -44.0;
pub const GUN_BOB_FREQ: f64 = 2.6;
pub const GUN_BOB_AMP: f64 = 0.9;
/// 子弹速率（逻辑坐标 / 秒）
pub const BULLET_SPEED: f64 = 520.0;
pub const BULLET_SPEED_FEVER: f64 = 620.0;
/// 导向最近怪的转向强度（越大越贴目标，约等价于 1/s 的收敛）
pub const BULLET_HOMING: f64 = 14.0;
