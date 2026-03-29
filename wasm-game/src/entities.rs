//! 游戏实体（位置、渲染用状态）
#[derive(Clone)]
pub struct Monster {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub speed: f64,
    pub hue: f64,
    pub phase: f64,
    #[allow(dead_code)]
    pub variant: u8,
}

pub struct Bullet {
    pub x: f64,
    pub y: f64,
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
