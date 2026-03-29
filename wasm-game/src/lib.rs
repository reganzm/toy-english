//! 玩具英语 · 打字守城 — Rust 核心，编译为 WebAssembly。
#![allow(clippy::too_many_arguments)]

mod bootstrap;
mod canvas;
mod dom_bridge;
mod entities;
mod game;
mod levels;
mod util;

pub use bootstrap::{run, set_game_mode};
