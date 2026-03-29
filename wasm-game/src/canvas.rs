//! Canvas 2D 样式（绕过 web-sys 已弃用的 set_fill_style API）
use js_sys::Reflect;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;

#[inline]
pub fn set_fill_style(ctx: &CanvasRenderingContext2d, color: &str) {
    let _ = Reflect::set(
        ctx.unchecked_ref(),
        &JsValue::from_str("fillStyle"),
        &JsValue::from_str(color),
    );
}

#[inline]
pub fn set_stroke_style(ctx: &CanvasRenderingContext2d, color: &str) {
    let _ = Reflect::set(
        ctx.unchecked_ref(),
        &JsValue::from_str("strokeStyle"),
        &JsValue::from_str(color),
    );
}
