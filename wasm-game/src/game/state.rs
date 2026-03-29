//! 游戏状态聚合
use crate::entities::{Bullet, Monster, Particle, Shockwave};
use super::mode::GameMode;

pub struct Game {
    pub mode: GameMode,
    pub level_index: usize,
    /// HUD 关卡号（嵌入式用 levels.id；API 用后端 id）
    pub level_display_id: u32,
    pub use_api_levels: bool,
    pub score: u32,
    pub target: String,
    pub progress: String,
    /// 当前句中文翻译（HUD / 弹窗）
    pub level_translation: String,
    pub monsters: Vec<Monster>,
    pub bullets: Vec<Bullet>,
    pub particles: Vec<Particle>,
    pub shockwaves: Vec<Shockwave>,
    pub combo: u32,
    pub lives: u32,
    pub level_locked: bool,
    pub bomb_active: bool,
    pub bomb_t: f64,
    pub wrong_flash: f64,
    pub last_frame_ms: f64,
    pub modal_open: bool,
    pub modal_game_over: bool,
    pub hint_used: bool,
    pub wrong_count: u32,
    pub tts_words: Vec<String>,
    pub tts_word_end_chars: Vec<usize>,
    pub tts_spoken_idx: usize,
}
