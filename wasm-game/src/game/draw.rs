//! Canvas 绘制
use std::f64::consts::PI;

use super::constants::{GUN_ANCHOR_X, GUN_ANCHOR_Y, GUN_BOB_AMP, GUN_BOB_FREQ};
use super::state::Game;
use crate::canvas::{set_fill_style, set_stroke_style};
use crate::monster_animals;
use crate::util::js_now_ms;
use web_sys::{CanvasRenderingContext2d, Path2d};

impl Game {
    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        set_fill_style(ctx, "#121a30");
        ctx.fill_rect(0.0, 0.0, 960.0, 540.0);

        for s in &self.shockwaves {
            let t = (s.life / s.max_life).max(0.0);
            ctx.set_global_alpha(t * 0.55);
            set_stroke_style(ctx, &s.color);
            ctx.set_line_width(2.0 + (1.0 - t) * 5.0);
            ctx.begin_path();
            let _ = ctx.arc(s.x, s.y, s.r, 0.0, PI * 2.0);
            ctx.stroke();
            ctx.set_global_alpha(t * 0.22);
            ctx.set_line_width(8.0);
            let _ = ctx.arc(s.x, s.y, s.r, 0.0, PI * 2.0);
            ctx.stroke();
            ctx.set_global_alpha(1.0);
        }

        let now = js_now_ms() / 1000.0;
        for m in &self.monsters {
            let cx = m.x + m.w / 2.0;
            let cy = m.y + m.h / 2.0;
            let bob = (now * 2.2 + m.phase).sin() * 4.0;
            let squash = 1.0 + (now * 3.0 + m.phase).sin() * 0.07;
            ctx.save();
            let _ = ctx.translate(cx, cy + bob);
            let _ = ctx.scale(squash, 2.0 - squash);
            let wiggle = (now * 4.0 + m.phase).sin() * 4.0;
            monster_animals::draw_creature(ctx, m.animal, m.w, m.h, m.hue, m.sat, m.lit, wiggle);
            ctx.restore();
        }

        for b in &self.bullets {
            set_fill_style(ctx, if b.fever { "#fff8f0" } else { "#9ef5ff" });
            ctx.begin_path();
            let _ = ctx.arc(b.x, b.y, b.r, 0.0, PI * 2.0);
            ctx.fill();
        }

        Self::draw_player_gun_svg(ctx, now);

        for p in &self.particles {
            let a = (p.life * 2.2).min(1.0);
            ctx.set_global_alpha(a);
            ctx.save();
            let _ = ctx.translate(p.x, p.y);
            let _ = ctx.rotate(p.rot);
            set_fill_style(ctx, &p.color);
            if p.size < 2.2 {
                ctx.begin_path();
                let _ = ctx.arc(0.0, 0.0, p.size, 0.0, PI * 2.0);
                ctx.fill();
            } else {
                let s = p.size;
                ctx.fill_rect(-s, -s, s * 2.0, s * 2.0);
            }
            ctx.restore();
            ctx.set_global_alpha(1.0);
        }

        if self.bomb_active {
            let alpha = (1.0 - self.bomb_t / 0.9).max(0.0);
            set_fill_style(
                ctx,
                &format!("rgba(255, 209, 102, {:.2})", alpha * 0.55),
            );
            ctx.fill_rect(0.0, 0.0, 960.0, 540.0);
        }

        if self.wrong_flash > 0.0 {
            set_fill_style(
                ctx,
                &format!(
                    "rgba(255, 80, 120, {:.2})",
                    (self.wrong_flash * 2.0).min(0.35)
                ),
            );
            ctx.fill_rect(0.0, 0.0, 960.0, 540.0);
        }
    }

    /// 底部发射台：Canvas `Path2D(SVG d)`，炮口朝上，与 `logic` 中 `y = 540-36` 发射点对齐。
    fn draw_player_gun_svg(ctx: &CanvasRenderingContext2d, now_secs: f64) {
        const D: &str = "\
            M0,-44 L-2.2,-39 L-3.2,-33 L-3.2,-21.5 L-12.5,-16.5 L-15,1.5 L-12,14.5 \
            L-6.5,19 L6.5,19 L12,14.5 L15,1.5 L12.5,-16.5 L3.2,-21.5 L3.2,-33 L2.2,-39 Z";
        let Ok(path) = Path2d::new_with_path_string(D) else {
            return;
        };

        ctx.save();
        let _ = ctx.translate(GUN_ANCHOR_X, GUN_ANCHOR_Y);
        let bob = (now_secs * GUN_BOB_FREQ).sin() * GUN_BOB_AMP;
        let _ = ctx.translate(0.0, bob);

        set_fill_style(ctx, "#3a5582");
        let _ = ctx.fill_with_path_2d(&path);

        set_stroke_style(ctx, "#9ec9e8");
        ctx.set_line_width(1.35);
        let _ = ctx.stroke_with_path(&path);

        set_stroke_style(ctx, "#d4eefc");
        ctx.set_line_width(1.0);
        ctx.begin_path();
        ctx.move_to(-2.2, -44.6);
        ctx.line_to(2.2, -44.6);
        ctx.stroke();

        ctx.restore();
    }
}
