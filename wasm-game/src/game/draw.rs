//! Canvas 绘制
use std::f64::consts::PI;

use super::state::Game;
use crate::canvas::{set_fill_style, set_stroke_style};
use crate::util::js_now_ms;
use web_sys::CanvasRenderingContext2d;

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
            let ax = (now * 4.0 + m.phase).sin() * 4.0;
            set_stroke_style(ctx, &format!("hsl({:.0}, 70%, 45%)", m.hue));
            ctx.set_line_width(3.0);
            ctx.begin_path();
            ctx.move_to(-m.w * 0.08 + ax, -m.h * 0.55);
            ctx.line_to(-m.w * 0.2 + ax, -m.h * 0.95);
            ctx.stroke();
            ctx.begin_path();
            ctx.move_to(m.w * 0.08 + ax, -m.h * 0.55);
            ctx.line_to(m.w * 0.2 + ax, -m.h * 0.95);
            ctx.stroke();
            set_fill_style(ctx, &format!("hsl({:.0}, 78%, 48%)", m.hue));
            ctx.begin_path();
            let _ = ctx.ellipse(0.0, 0.0, m.w / 2.0, m.h / 2.0, 0.0, 0.0, PI * 2.0);
            ctx.fill();
            ctx.restore();
        }

        for b in &self.bullets {
            set_fill_style(ctx, if b.fever { "#fff8f0" } else { "#9ef5ff" });
            ctx.begin_path();
            let _ = ctx.arc(b.x, b.y, b.r, 0.0, PI * 2.0);
            ctx.fill();
        }

        set_fill_style(ctx, "#3d5a80");
        ctx.begin_path();
        ctx.move_to(480.0, 540.0 - 46.0);
        ctx.line_to(480.0 + 22.0, 540.0 - 14.0);
        ctx.line_to(480.0 - 22.0, 540.0 - 14.0);
        ctx.close_path();
        ctx.fill();

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
}
