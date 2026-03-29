//! 启动：画布、事件、主循环
use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};

use crate::game::{Game, GameMode};

thread_local! {
    static GAME_HOOK: RefCell<Option<Rc<RefCell<Game>>>> = RefCell::new(None);
}

/// 切换难度并重开本关（进度、生命、怪物重置）。
#[wasm_bindgen]
pub fn set_game_mode(mode: u8) {
    GAME_HOOK.with(|h| {
        if let Some(g) = h.borrow().as_ref() {
            g.borrow_mut().apply_mode(GameMode::from_u8(mode));
        }
    });
}

/// 由 TS 在分页拉取下一题后调用，重置战斗并切换句子。
#[wasm_bindgen]
pub fn apply_api_level(level_id: u32, sentence: &str, translation: &str) {
    GAME_HOOK.with(|h| {
        if let Some(g) = h.borrow().as_ref() {
            g.borrow_mut()
                .start_api_level(level_id, sentence.to_string(), translation.to_string());
        }
    });
}

/// 弹窗「下一关 / 再试」：0=未打开；1=游戏结束已重开；2=嵌入式已进下一关；3=API 已关弹窗，需 TS 拉题并 `apply_api_level`。
#[wasm_bindgen]
pub fn handle_modal_next() -> u8 {
    GAME_HOOK.with(|h| {
        if let Some(g) = h.borrow().as_ref() {
            g.borrow_mut().handle_modal_next()
        } else {
            0
        }
    })
}

#[wasm_bindgen]
pub fn is_modal_game_over() -> bool {
    GAME_HOOK.with(|h| {
        h.borrow()
            .as_ref()
            .map(|g| {
                let g = g.borrow();
                g.modal_open && g.modal_game_over
            })
            .unwrap_or(false)
    })
}

#[wasm_bindgen]
pub fn run(
    mode: u8,
    api_sentence: Option<String>,
    api_translation: Option<String>,
    api_level_id: Option<u32>,
) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let win = window().ok_or("no window")?;
    let document = win.document().ok_or("no document")?;
    let canvas = document
        .get_element_by_id("game")
        .ok_or("no #game")?
        .dyn_into::<HtmlCanvasElement>()?;
    canvas.set_width(960);
    canvas.set_height(540);
    let ctx = canvas
        .get_context("2d")?
        .ok_or("no 2d")?
        .dyn_into::<CanvasRenderingContext2d>()?;

    let mut game_inner = Game::new_unstarted(GameMode::from_u8(mode));
    match (api_sentence, api_translation) {
        (Some(s), Some(t)) => {
            let id = api_level_id.unwrap_or(1);
            game_inner.start_api_level(id, s, t);
        }
        _ => game_inner.start_embedded_level(0),
    }
    let game = Rc::new(RefCell::new(game_inner));
    GAME_HOOK.with(|h| {
        *h.borrow_mut() = Some(game.clone());
    });
    let game_key = game.clone();

    let key_cb = Closure::wrap(Box::new(move |e: KeyboardEvent| {
        if game_key.borrow().level_locked {
            return;
        }
        let key = e.key();
        if key.len() == 1 && !e.ctrl_key() && !e.meta_key() && !e.alt_key() {
            e.prevent_default();
            game_key.borrow_mut().try_char(key.chars().next().unwrap_or(' '));
        } else if key == " " {
            e.prevent_default();
            game_key.borrow_mut().try_char(' ');
        }
    }) as Box<dyn FnMut(_)>);
    win.add_event_listener_with_callback("keydown", key_cb.as_ref().unchecked_ref())?;
    key_cb.forget();

    let f = Rc::new(RefCell::new(None::<Closure<dyn FnMut(f64)>>));
    let g = f.clone();
    let game_loop = game.clone();
    let doc_loop = document.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |time: f64| {
        {
            let mut gm = game_loop.borrow_mut();
            let dt = {
                let last = gm.last_frame_ms;
                gm.last_frame_ms = time;
                if last <= 0.0 {
                    1.0 / 60.0
                } else {
                    ((time - last) / 1000.0).min(0.05)
                }
            };
            gm.tick(dt);
            gm.draw(&ctx);
            gm.sync_dom(&doc_loop);
        }
        let w = window().expect("window");
        let cb = f.borrow();
        let c = cb.as_ref().unwrap();
        w.request_animation_frame(c.as_ref().unchecked_ref()).unwrap();
    }) as Box<dyn FnMut(f64)>));

    win.request_animation_frame(
        g.borrow()
            .as_ref()
            .unwrap()
            .as_ref()
            .unchecked_ref(),
    )?;

    Ok(())
}
