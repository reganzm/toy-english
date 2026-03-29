//! 玩具英语 · 打字守城 — Rust 核心，编译为 WebAssembly。
#![allow(clippy::too_many_arguments)]

mod bootstrap;
mod canvas;
mod dom_bridge;
mod entities;
mod game;
mod levels;
mod monster_animals;
mod util;

pub use bootstrap::{
    apply_api_level, handle_modal_next, is_modal_game_over, run, set_game_mode,
};
