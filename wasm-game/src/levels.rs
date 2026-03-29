//! 关卡静态数据
pub struct LevelData {
    pub id: u32,
    pub sentence: &'static str,
    pub translation: &'static str,
}

pub const LEVELS: &[LevelData] = &[
    LevelData {
        id: 1,
        sentence: "The cat sleeps.",
        translation: "猫在睡觉。",
    },
    LevelData {
        id: 2,
        sentence: "I love toy English.",
        translation: "我喜欢玩具英语。",
    },
    LevelData {
        id: 3,
        sentence: "She reads a book every day.",
        translation: "她每天读一本书。",
    },
];
