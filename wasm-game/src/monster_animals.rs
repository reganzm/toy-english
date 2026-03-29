//! 10 种小动物外形。局部坐标约 x,y ∈ [-1,1]，调用前已处于怪物体中心且可再 scale(sx,sy)。
use std::f64::consts::PI;

use web_sys::CanvasRenderingContext2d;

use crate::canvas::{set_fill_style, set_stroke_style};

pub const ANIMAL_KINDS: u8 = 20;

fn stroke_close(ctx: &CanvasRenderingContext2d, stroke: &str) {
    set_stroke_style(ctx, stroke);
    let _ = ctx.stroke();
}

fn fill_shape(ctx: &CanvasRenderingContext2d, fill: &str) {
    set_fill_style(ctx, fill);
    ctx.fill();
}

fn cat(ctx: &CanvasRenderingContext2d, fill: &str, stroke: &str, wx: f64) {
    ctx.begin_path();
    let _ = ctx.ellipse(0.0, 0.12, 0.42, 0.35, 0.0, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    ctx.move_to(-0.55 + wx * 0.04, -0.15);
    ctx.line_to(-0.28 + wx * 0.04, -0.95);
    ctx.line_to(-0.05 + wx * 0.02, -0.35);
    ctx.close_path();
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    ctx.move_to(0.55 + wx * 0.04, -0.15);
    ctx.line_to(0.28 + wx * 0.04, -0.95);
    ctx.line_to(0.05 + wx * 0.02, -0.35);
    ctx.close_path();
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.arc(0.0, -0.22, 0.38, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    set_fill_style(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.arc(-0.14, -0.28, 0.07, 0.0, PI * 2.0);
    ctx.fill();
    ctx.begin_path();
    let _ = ctx.arc(0.14, -0.28, 0.07, 0.0, PI * 2.0);
    ctx.fill();
    ctx.begin_path();
    let _ = ctx.arc(0.0, -0.12, 0.05, 0.0, PI * 2.0);
    ctx.fill();
}

fn dog(ctx: &CanvasRenderingContext2d, fill: &str, stroke: &str, wx: f64) {
    ctx.begin_path();
    let _ = ctx.ellipse(0.0, 0.1, 0.48, 0.32, 0.0, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    ctx.move_to(-0.62 + wx * 0.03, -0.05);
    ctx.quadratic_curve_to(-0.85 + wx * 0.05, 0.35, -0.35, 0.45);
    ctx.line_to(-0.25, 0.12);
    ctx.close_path();
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    ctx.move_to(0.62 + wx * 0.03, -0.05);
    ctx.quadratic_curve_to(0.85 + wx * 0.05, 0.35, 0.35, 0.45);
    ctx.line_to(0.25, 0.12);
    ctx.close_path();
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.ellipse(0.0, -0.28, 0.36, 0.34, 0.0, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    ctx.move_to(0.32, -0.22);
    ctx.line_to(0.75, -0.18);
    ctx.line_to(0.35, -0.05);
    ctx.close_path();
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    set_fill_style(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.arc(-0.12, -0.32, 0.06, 0.0, PI * 2.0);
    ctx.fill();
    ctx.begin_path();
    let _ = ctx.arc(0.12, -0.32, 0.06, 0.0, PI * 2.0);
    ctx.fill();
}

fn rabbit(ctx: &CanvasRenderingContext2d, fill: &str, stroke: &str, wx: f64) {
    ctx.begin_path();
    let _ = ctx.ellipse(-0.22 + wx * 0.02, -0.72, 0.14, 0.42, -0.15, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.ellipse(0.22 + wx * 0.02, -0.72, 0.14, 0.42, 0.15, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.arc(0.0, -0.08, 0.45, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.ellipse(0.0, 0.38, 0.38, 0.28, 0.0, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    set_fill_style(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.arc(-0.12, -0.12, 0.06, 0.0, PI * 2.0);
    ctx.fill();
    ctx.begin_path();
    let _ = ctx.arc(0.12, -0.12, 0.06, 0.0, PI * 2.0);
    ctx.fill();
    ctx.begin_path();
    let _ = ctx.ellipse(0.08, 0.02, 0.06, 0.04, 0.2, 0.0, PI * 2.0);
    ctx.fill();
}

fn bird(ctx: &CanvasRenderingContext2d, fill: &str, stroke: &str, wx: f64) {
    ctx.begin_path();
    let _ = ctx.ellipse(0.05 + wx * 0.02, 0.0, 0.42, 0.5, 0.0, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    ctx.move_to(-0.52 + wx * 0.02, 0.0);
    ctx.line_to(-0.88 + wx * 0.02, 0.08);
    ctx.line_to(-0.52 + wx * 0.02, 0.18);
    ctx.close_path();
    set_fill_style(ctx, stroke);
    ctx.fill();
    stroke_close(ctx, stroke);
    set_fill_style(ctx, fill);
    ctx.begin_path();
    ctx.move_to(0.35, 0.05);
    ctx.quadratic_curve_to(0.75, -0.15, 0.55, 0.35);
    ctx.line_to(0.25, 0.25);
    ctx.close_path();
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    set_fill_style(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.arc(0.22, -0.18, 0.08, 0.0, PI * 2.0);
    ctx.fill();
}

fn pig(ctx: &CanvasRenderingContext2d, fill: &str, stroke: &str, wx: f64) {
    ctx.begin_path();
    let _ = ctx.arc(0.0, -0.05, 0.48, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.ellipse(-0.42 + wx * 0.03, -0.55, 0.16, 0.12, -0.4, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.ellipse(0.42 + wx * 0.03, -0.55, 0.16, 0.12, 0.4, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.ellipse(0.45, -0.02, 0.22, 0.18, 0.0, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.ellipse(0.0, 0.42, 0.35, 0.22, 0.0, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    set_fill_style(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.arc(0.52, -0.02, 0.04, 0.0, PI * 2.0);
    ctx.fill();
    ctx.begin_path();
    let _ = ctx.arc(0.62, -0.02, 0.04, 0.0, PI * 2.0);
    ctx.fill();
}

fn bear(ctx: &CanvasRenderingContext2d, fill: &str, stroke: &str, wx: f64) {
    ctx.begin_path();
    let _ = ctx.arc(-0.52 + wx * 0.02, -0.62, 0.18, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.arc(0.52 + wx * 0.02, -0.62, 0.18, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.arc(0.0, -0.15, 0.52, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.ellipse(0.0, 0.42, 0.45, 0.3, 0.0, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    set_fill_style(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.arc(-0.16, -0.22, 0.07, 0.0, PI * 2.0);
    ctx.fill();
    ctx.begin_path();
    let _ = ctx.arc(0.16, -0.22, 0.07, 0.0, PI * 2.0);
    ctx.fill();
    ctx.begin_path();
    let _ = ctx.arc(0.0, -0.02, 0.06, 0.0, PI * 2.0);
    ctx.fill();
}

fn fox(ctx: &CanvasRenderingContext2d, fill: &str, stroke: &str, wx: f64) {
    ctx.begin_path();
    ctx.move_to(-0.15 + wx * 0.02, -0.88);
    ctx.line_to(-0.55 + wx * 0.03, -0.25);
    ctx.line_to(-0.1, -0.15);
    ctx.close_path();
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    ctx.move_to(0.15 + wx * 0.02, -0.88);
    ctx.line_to(0.55 + wx * 0.03, -0.25);
    ctx.line_to(0.1, -0.15);
    ctx.close_path();
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    ctx.move_to(0.0, 0.55);
    ctx.line_to(-0.35, -0.05);
    ctx.line_to(0.0, -0.45);
    ctx.line_to(0.35, -0.05);
    ctx.close_path();
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.ellipse(0.0, 0.38, 0.32, 0.22, 0.0, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    set_fill_style(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.arc(-0.1, -0.18, 0.05, 0.0, PI * 2.0);
    ctx.fill();
    ctx.begin_path();
    let _ = ctx.arc(0.1, -0.18, 0.05, 0.0, PI * 2.0);
    ctx.fill();
}

fn panda(
    ctx: &CanvasRenderingContext2d,
    _fill: &str,
    stroke: &str,
    wx: f64,
    hue: f64,
) {
    let black = format!("hsl({:.0}, 8%, 18%)", hue);
    let white = "hsl(210, 25%, 96%)";
    ctx.begin_path();
    let _ = ctx.arc(-0.48 + wx * 0.02, -0.58, 0.2, 0.0, PI * 2.0);
    set_fill_style(ctx, &black);
    fill_shape(ctx, &black);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.arc(0.48 + wx * 0.02, -0.58, 0.2, 0.0, PI * 2.0);
    fill_shape(ctx, &black);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.arc(0.0, -0.08, 0.5, 0.0, PI * 2.0);
    set_fill_style(ctx, white);
    ctx.fill();
    stroke_close(ctx, stroke);
    set_fill_style(ctx, &black);
    ctx.begin_path();
    let _ = ctx.arc(-0.18, -0.12, 0.12, 0.0, PI * 2.0);
    ctx.fill();
    ctx.begin_path();
    let _ = ctx.arc(0.18, -0.12, 0.12, 0.0, PI * 2.0);
    ctx.fill();
    ctx.begin_path();
    let _ = ctx.ellipse(0.0, 0.05, 0.08, 0.06, 0.0, 0.0, PI * 2.0);
    ctx.fill();
    ctx.begin_path();
    let _ = ctx.ellipse(0.0, 0.45, 0.4, 0.28, 0.0, 0.0, PI * 2.0);
    set_fill_style(ctx, white);
    ctx.fill();
    stroke_close(ctx, stroke);
}

fn mouse(ctx: &CanvasRenderingContext2d, fill: &str, stroke: &str, wx: f64) {
    ctx.begin_path();
    let _ = ctx.arc(-0.55 + wx * 0.02, -0.35, 0.28, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.arc(0.55 + wx * 0.02, -0.35, 0.28, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.ellipse(0.0, 0.05, 0.38, 0.32, 0.0, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    ctx.move_to(0.35, 0.12);
    ctx.line_to(0.85 + wx * 0.02, 0.18);
    ctx.line_to(0.35, 0.22);
    ctx.close_path();
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    set_fill_style(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.arc(-0.1, -0.02, 0.06, 0.0, PI * 2.0);
    ctx.fill();
    ctx.begin_path();
    let _ = ctx.arc(0.1, -0.02, 0.06, 0.0, PI * 2.0);
    ctx.fill();
}

fn frog(ctx: &CanvasRenderingContext2d, fill: &str, stroke: &str, wx: f64) {
    ctx.begin_path();
    let _ = ctx.arc(-0.38 + wx * 0.02, -0.62, 0.22, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.arc(0.38 + wx * 0.02, -0.62, 0.22, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    set_fill_style(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.arc(-0.38 + wx * 0.02, -0.62, 0.1, 0.0, PI * 2.0);
    ctx.fill();
    ctx.begin_path();
    let _ = ctx.arc(0.38 + wx * 0.02, -0.62, 0.1, 0.0, PI * 2.0);
    ctx.fill();
    set_fill_style(ctx, fill);
    ctx.begin_path();
    let _ = ctx.ellipse(0.0, 0.08, 0.55, 0.38, 0.0, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.ellipse(-0.35, 0.42, 0.2, 0.12, 0.4, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.ellipse(0.35, 0.42, 0.2, 0.12, -0.4, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    set_fill_style(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.arc(0.0, 0.12, 0.06, 0.0, PI * 2.0);
    ctx.fill();
}

fn duck(ctx: &CanvasRenderingContext2d, fill: &str, stroke: &str, wx: f64) {
    ctx.begin_path();
    let _ = ctx.ellipse(0.0, 0.06, 0.44, 0.34, 0.0, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    ctx.move_to(-0.42 + wx * 0.02, 0.1);
    ctx.line_to(-0.78 + wx * 0.02, 0.14);
    ctx.line_to(-0.42 + wx * 0.02, 0.22);
    ctx.close_path();
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    set_fill_style(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.arc(0.14, -0.1, 0.07, 0.0, PI * 2.0);
    ctx.fill();
}

fn owl(ctx: &CanvasRenderingContext2d, fill: &str, stroke: &str, wx: f64) {
    ctx.begin_path();
    ctx.move_to(-0.12 + wx * 0.02, -0.82);
    ctx.line_to(-0.42 + wx * 0.02, -0.32);
    ctx.line_to(0.0, -0.22);
    ctx.close_path();
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    ctx.move_to(0.12 + wx * 0.02, -0.82);
    ctx.line_to(0.42 + wx * 0.02, -0.32);
    ctx.line_to(0.0, -0.22);
    ctx.close_path();
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.arc(0.0, 0.05, 0.5, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    set_fill_style(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.arc(-0.2, 0.08, 0.15, 0.0, PI * 2.0);
    ctx.fill();
    ctx.begin_path();
    let _ = ctx.arc(0.2, 0.08, 0.15, 0.0, PI * 2.0);
    ctx.fill();
    ctx.begin_path();
    let _ = ctx.arc(0.0, 0.28, 0.06, 0.0, PI * 2.0);
    ctx.fill();
}

fn sheep(ctx: &CanvasRenderingContext2d, fill: &str, stroke: &str, wx: f64) {
    ctx.begin_path();
    let _ = ctx.arc(0.0, -0.18, 0.32, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    for idx in 0..5i32 {
        let t = idx as f64 / 4.0 - 0.5;
        ctx.begin_path();
        let _ = ctx.arc(t * 0.72 + wx * 0.02, 0.24, 0.14, 0.0, PI * 2.0);
        fill_shape(ctx, fill);
        stroke_close(ctx, stroke);
    }
    set_fill_style(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.arc(-0.1, -0.22, 0.05, 0.0, PI * 2.0);
    ctx.fill();
    ctx.begin_path();
    let _ = ctx.arc(0.1, -0.22, 0.05, 0.0, PI * 2.0);
    ctx.fill();
}

fn cow(ctx: &CanvasRenderingContext2d, fill: &str, stroke: &str, wx: f64) {
    ctx.begin_path();
    let _ = ctx.arc(0.0, -0.08, 0.44, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.ellipse(0.4 + wx * 0.02, -0.46, 0.1, 0.16, 0.35, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.ellipse(-0.4 + wx * 0.02, -0.46, 0.1, 0.16, -0.35, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.ellipse(0.0, 0.42, 0.4, 0.24, 0.0, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    set_fill_style(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.ellipse(0.32, -0.02, 0.1, 0.08, 0.0, 0.0, PI * 2.0);
    ctx.fill();
}

fn chick(ctx: &CanvasRenderingContext2d, fill: &str, stroke: &str, wx: f64) {
    ctx.begin_path();
    let _ = ctx.arc(0.0, 0.02, 0.46, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    ctx.move_to(0.32 + wx * 0.02, 0.08);
    ctx.line_to(0.58 + wx * 0.02, 0.18);
    ctx.line_to(0.3 + wx * 0.02, 0.24);
    ctx.close_path();
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    set_fill_style(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.arc(-0.14, -0.1, 0.07, 0.0, PI * 2.0);
    ctx.fill();
    ctx.begin_path();
    let _ = ctx.arc(0.14, -0.1, 0.07, 0.0, PI * 2.0);
    ctx.fill();
}

fn deer(ctx: &CanvasRenderingContext2d, fill: &str, stroke: &str, wx: f64) {
    ctx.begin_path();
    ctx.move_to(0.0 + wx * 0.02, -0.92);
    ctx.line_to(-0.1 + wx * 0.02, -0.48);
    ctx.line_to(0.1 + wx * 0.02, -0.48);
    ctx.close_path();
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    ctx.move_to(-0.2 + wx * 0.02, -0.85);
    ctx.line_to(-0.32 + wx * 0.02, -0.52);
    ctx.line_to(-0.06 + wx * 0.02, -0.42);
    ctx.close_path();
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    ctx.move_to(0.2 + wx * 0.02, -0.85);
    ctx.line_to(0.32 + wx * 0.02, -0.52);
    ctx.line_to(0.06 + wx * 0.02, -0.42);
    ctx.close_path();
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.ellipse(0.0, 0.08, 0.38, 0.38, 0.0, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    set_fill_style(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.arc(-0.12, -0.02, 0.05, 0.0, PI * 2.0);
    ctx.fill();
    ctx.begin_path();
    let _ = ctx.arc(0.12, -0.02, 0.05, 0.0, PI * 2.0);
    ctx.fill();
}

fn hedgehog(ctx: &CanvasRenderingContext2d, fill: &str, stroke: &str, wx: f64) {
    ctx.begin_path();
    let _ = ctx.ellipse(0.0, 0.24, 0.36, 0.22, 0.0, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    set_stroke_style(ctx, stroke);
    ctx.set_line_width(0.12);
    ctx.begin_path();
    for idx in -5i32..=5 {
        let x = idx as f64 * 0.09 + wx * 0.02;
        ctx.move_to(x, -0.12);
        ctx.line_to(x + 0.03, -0.48);
    }
    let _ = ctx.stroke();
    ctx.begin_path();
    let _ = ctx.arc(0.32, 0.2, 0.09, 0.0, PI * 2.0);
    set_fill_style(ctx, fill);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
}

fn seal(ctx: &CanvasRenderingContext2d, fill: &str, stroke: &str, wx: f64) {
    ctx.begin_path();
    let _ = ctx.ellipse(0.0, 0.02, 0.52, 0.26, 0.0, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.ellipse(0.44 + wx * 0.02, -0.04, 0.16, 0.12, 0.45, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.ellipse(-0.32, 0.16, 0.12, 0.1, -0.45, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    set_fill_style(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.arc(0.54 + wx * 0.02, -0.06, 0.04, 0.0, PI * 2.0);
    ctx.fill();
}

fn snail(ctx: &CanvasRenderingContext2d, fill: &str, stroke: &str, wx: f64) {
    ctx.begin_path();
    let _ = ctx.arc(-0.06 + wx * 0.02, -0.38, 0.26, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    ctx.begin_path();
    let _ = ctx.ellipse(0.24 + wx * 0.02, 0.14, 0.4, 0.18, 0.0, 0.0, PI * 2.0);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    set_stroke_style(ctx, stroke);
    ctx.set_line_width(0.07);
    ctx.begin_path();
    let _ = ctx.arc(-0.06 + wx * 0.02, -0.38, 0.16, 0.9, PI * 2.55);
    let _ = ctx.stroke();
}

fn bee(ctx: &CanvasRenderingContext2d, fill: &str, stroke: &str, wx: f64) {
    set_fill_style(ctx, "rgba(255,252,255,0.5)");
    ctx.begin_path();
    let _ = ctx.ellipse(-0.36 + wx * 0.02, -0.18, 0.2, 0.11, -0.35, 0.0, PI * 2.0);
    ctx.fill();
    ctx.begin_path();
    let _ = ctx.ellipse(-0.34 + wx * 0.02, 0.12, 0.2, 0.11, 0.35, 0.0, PI * 2.0);
    ctx.fill();
    ctx.begin_path();
    let _ = ctx.ellipse(0.0, 0.0, 0.46, 0.3, 0.0, 0.0, PI * 2.0);
    set_fill_style(ctx, fill);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
    set_stroke_style(ctx, stroke);
    ctx.set_line_width(0.09);
    ctx.begin_path();
    ctx.move_to(-0.32, -0.06);
    ctx.line_to(0.32, -0.06);
    let _ = ctx.stroke();
    ctx.begin_path();
    ctx.move_to(-0.3, 0.06);
    ctx.line_to(0.3, 0.06);
    let _ = ctx.stroke();
    ctx.begin_path();
    ctx.move_to(-0.52 + wx * 0.02, -0.02);
    ctx.line_to(-0.82 + wx * 0.02, -0.12);
    ctx.line_to(-0.52 + wx * 0.02, -0.2);
    ctx.close_path();
    set_fill_style(ctx, stroke);
    fill_shape(ctx, fill);
    stroke_close(ctx, stroke);
}

pub fn draw_creature(
    ctx: &CanvasRenderingContext2d,
    kind: u8,
    w: f64,
    h: f64,
    hue: f64,
    sat: f64,
    lit: f64,
    wiggle: f64,
) {
    let k = kind % ANIMAL_KINDS;
    let stroke_sat = (sat - 4.0).max(32.0);
    let stroke_lit = (lit - 20.0).max(20.0);
    let fill = format!("hsl({:.0}, {:.0}%, {:.0}%)", hue, sat, lit);
    let stroke = format!("hsl({:.0}, {:.0}%, {:.0}%)", hue, stroke_sat, stroke_lit);
    let lw = (w / 22.0).clamp(1.15, 2.4);

    ctx.save();
    let sx = w * 0.48;
    let sy = h * 0.48;
    let _ = ctx.scale(sx, sy);
    let wx = wiggle / sx.max(1e-6);

    ctx.set_line_width(lw / sx.min(sy).max(0.01));

    match k {
        0 => cat(ctx, &fill, &stroke, wx),
        1 => dog(ctx, &fill, &stroke, wx),
        2 => rabbit(ctx, &fill, &stroke, wx),
        3 => bird(ctx, &fill, &stroke, wx),
        4 => pig(ctx, &fill, &stroke, wx),
        5 => bear(ctx, &fill, &stroke, wx),
        6 => fox(ctx, &fill, &stroke, wx),
        7 => panda(ctx, &fill, &stroke, wx, hue),
        8 => mouse(ctx, &fill, &stroke, wx),
        9 => frog(ctx, &fill, &stroke, wx),
        10 => duck(ctx, &fill, &stroke, wx),
        11 => owl(ctx, &fill, &stroke, wx),
        12 => sheep(ctx, &fill, &stroke, wx),
        13 => cow(ctx, &fill, &stroke, wx),
        14 => chick(ctx, &fill, &stroke, wx),
        15 => deer(ctx, &fill, &stroke, wx),
        16 => hedgehog(ctx, &fill, &stroke, wx),
        17 => seal(ctx, &fill, &stroke, wx),
        18 => snail(ctx, &fill, &stroke, wx),
        _ => bee(ctx, &fill, &stroke, wx),
    }

    ctx.restore();
}
