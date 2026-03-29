//! 游戏实体（位置、渲染用状态）
#[derive(Clone)]
pub struct Monster {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub speed: f64,
    /// HSL 色相 0..360
    pub hue: f64,
    /// HSL 饱和度 %（约 50–95）
    pub sat: f64,
    /// HSL 亮度 %（约 44–70）
    pub lit: f64,
    pub phase: f64,
    /// 0..20：猫狗兔鸟猪熊狐熊猫鼠蛙 + 鸭猫头鹰羊牛鸡崽鹿刺猬海豹蜗牛蜜蜂
    pub animal: u8,
}

pub struct Bullet {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub r: f64,
    pub fever: bool,
}

pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub life: f64,
    pub color: String,
    pub size: f64,
    pub rot: f64,
    pub vr: f64,
}

pub struct Shockwave {
    pub x: f64,
    pub y: f64,
    pub r: f64,
    pub life: f64,
    pub max_life: f64,
    pub color: String,
}
