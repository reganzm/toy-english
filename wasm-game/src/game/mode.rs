//! 难度：听写遮罩与 TTS 策略
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum GameMode {
    /// 仅 TTS，不显示中英文
    Hell = 0,
    /// TTS + 中文提示，英文听写
    Hard = 1,
    /// TTS + 中文 + 英文（跟打）
    Easy = 2,
}

impl GameMode {
    pub fn from_u8(v: u8) -> Self {
        match v % 3 {
            0 => GameMode::Hell,
            1 => GameMode::Hard,
            _ => GameMode::Easy,
        }
    }

    pub fn shows_english(&self) -> bool {
        matches!(self, GameMode::Easy)
    }

    /// 关卡开始时整句朗读，且打字过程中不再按词跟读
    pub fn dictation_listen(&self) -> bool {
        matches!(self, GameMode::Hell | GameMode::Hard)
    }
}
