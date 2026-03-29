//! 调用页面 `window` 上由 TypeScript 注册的桥接函数（TTS 等）
use js_sys::{Function, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;

fn call_window_fn0(name: &str) {
    let Some(w) = window() else {
        return;
    };
    if let Ok(v) = Reflect::get(&w, &JsValue::from_str(name)) {
        if let Ok(f) = v.dyn_into::<Function>() {
            let _ = f.call0(&JsValue::NULL);
        }
    }
}

fn call_window_fn1_str(name: &str, arg: &str) {
    let Some(w) = window() else {
        return;
    };
    if let Ok(v) = Reflect::get(&w, &JsValue::from_str(name)) {
        if let Ok(f) = v.dyn_into::<Function>() {
            let _ = f.call1(&JsValue::NULL, &JsValue::from_str(arg));
        }
    }
}

pub fn tts_speak_word(text: &str) {
    call_window_fn1_str("__toyEnglishTtsSpeak", text);
}

pub fn tts_speak_full_sentence(text: &str) {
    call_window_fn1_str("__toyEnglishTtsSpeakFull", text);
}

pub fn tts_cancel() {
    call_window_fn0("__toyEnglishTtsCancel");
}
