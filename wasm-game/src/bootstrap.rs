//! 启动：画布、事件、主循环
use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, CanvasRenderingContext2d, Document, HtmlCanvasElement, KeyboardEvent};

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

#[wasm_bindgen]
pub fn run(mode: u8) -> Result<(), JsValue> {
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

    let game = Rc::new(RefCell::new(Game::new_with_mode(GameMode::from_u8(mode))));
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

    wire_buttons(&document, game.clone())?;

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

fn wire_buttons(document: &Document, game: Rc<RefCell<Game>>) -> Result<(), JsValue> {
    if let Some(el) = document.get_element_by_id("btn-hint") {
        let g = game.clone();
        let cb = Closure::wrap(Box::new(move |_e: web_sys::Event| {
            g.borrow_mut().hint();
        }) as Box<dyn FnMut(_)>);
        el.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }
    if let Some(el) = document.get_element_by_id("btn-skip-space") {
        let g = game.clone();
        let cb = Closure::wrap(Box::new(move |_e: web_sys::Event| {
            g.borrow_mut().try_char(' ');
        }) as Box<dyn FnMut(_)>);
        el.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }
    if let Some(el) = document.get_element_by_id("btn-next") {
        let g = game.clone();
        let cb = Closure::wrap(Box::new(move |_e: web_sys::Event| {
            g.borrow_mut().next_modal();
        }) as Box<dyn FnMut(_)>);
        el.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }
    Ok(())
}
