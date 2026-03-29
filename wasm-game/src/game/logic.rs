//! 规则、物理、输入、关卡流程
use std::f64::consts::PI;

use super::constants::*;
use super::mode::GameMode;
use super::state::Game;
use crate::dom_bridge;
use crate::entities::{Bullet, Monster};
use crate::levels::LEVELS;
use crate::util::js_rand;

impl Game {
    pub fn new_with_mode(mode: GameMode) -> Self {
        let mut g = Self {
            mode,
            level_index: 0,
            score: 0,
            target: String::new(),
            progress: String::new(),
            monsters: Vec::new(),
            bullets: Vec::new(),
            particles: Vec::new(),
            shockwaves: Vec::new(),
            combo: 0,
            lives: 3,
            level_locked: false,
            bomb_active: false,
            bomb_t: 0.0,
            wrong_flash: 0.0,
            last_frame_ms: -1.0,
            modal_open: false,
            modal_game_over: false,
            hint_used: false,
            wrong_count: 0,
            tts_words: Vec::new(),
            tts_word_end_chars: Vec::new(),
            tts_spoken_idx: 0,
        };
        g.start_level(0);
        g
    }

    pub fn apply_mode(&mut self, mode: GameMode) {
        self.mode = mode;
        self.start_level(self.level_index);
    }

    pub fn level(&self) -> &crate::levels::LevelData {
        &LEVELS[self.level_index.min(LEVELS.len() - 1)]
    }

    fn start_level(&mut self, idx: usize) {
        self.level_index = idx.min(LEVELS.len() - 1);
        self.level_locked = false;
        self.progress.clear();
        self.combo = 0;
        self.lives = 3;
        self.bullets.clear();
        self.particles.clear();
        self.shockwaves.clear();
        self.bomb_active = false;
        self.bomb_t = 0.0;
        self.hint_used = false;
        self.wrong_count = 0;
        self.modal_open = false;
        self.modal_game_over = false;
        dom_bridge::tts_cancel();
        self.target = self.level().sentence.to_string();
        self.spawn_monsters();
        self.rebuild_tts_plan();
        if self.mode.dictation_listen() {
            dom_bridge::tts_speak_full_sentence(&self.target);
        }
    }

    fn rebuild_tts_plan(&mut self) {
        self.tts_words.clear();
        self.tts_word_end_chars.clear();
        self.tts_spoken_idx = 0;
        let target = self.target.as_str();
        let mut word_start_char: Option<usize> = None;
        let mut char_idx = 0usize;
        for ch in target.chars() {
            if ch.is_whitespace() {
                if let Some(sc) = word_start_char.take() {
                    if char_idx > sc {
                        let w: String = target.chars().skip(sc).take(char_idx - sc).collect();
                        self.tts_words.push(w);
                        self.tts_word_end_chars.push(char_idx);
                    }
                }
            } else if word_start_char.is_none() {
                word_start_char = Some(char_idx);
            }
            char_idx += 1;
        }
        if let Some(sc) = word_start_char {
            let w: String = target.chars().skip(sc).collect();
            if !w.is_empty() {
                self.tts_words.push(w);
                self.tts_word_end_chars.push(char_idx);
            }
        }
    }

    fn maybe_speak_completed_words(&mut self) {
        if self.mode.dictation_listen() {
            return;
        }
        let n_chars = self.progress.chars().count();
        let total_chars = self.target.chars().count();
        while self.tts_spoken_idx < self.tts_word_end_chars.len() {
            let end = self.tts_word_end_chars[self.tts_spoken_idx];
            if n_chars < end {
                break;
            }
            let is_last = self.tts_spoken_idx + 1 == self.tts_words.len();
            let sentence_done = n_chars >= total_chars;
            if is_last && sentence_done {
                self.tts_spoken_idx += 1;
                break;
            }
            let w = self.tts_words[self.tts_spoken_idx].clone();
            self.tts_spoken_idx += 1;
            dom_bridge::tts_speak_word(&w);
        }
    }

    fn spawn_monsters(&mut self) {
        self.monsters.clear();
        let letter_count = self.target.chars().filter(|c| !c.is_whitespace()).count();
        let count = BASE_MONSTERS.max(letter_count + 8);
        for i in 0..count {
            let col = (i % 8) as f64;
            let row = (i / 8) as f64;
            self.monsters.push(Monster {
                x: 80.0 + col * 100.0 + (js_rand() * 24.0 - 12.0),
                y: -60.0 - row * 55.0 - js_rand() * 40.0,
                w: 44.0 + js_rand() * 16.0,
                h: 36.0 + js_rand() * 12.0,
                speed: 28.0 + js_rand() * 45.0,
                hue: 280.0 + js_rand() * 60.0,
                phase: js_rand() * PI * 2.0,
                variant: (i % 3) as u8,
            });
        }
    }

    pub fn combo_mult(&self) -> f64 {
        if self.combo == 0 {
            return 1.0;
        }
        let steps = self.combo / COMBO_STEP;
        1.0 + (steps as f64 * 0.12).min(MAX_MULT - 1.0)
    }

    pub fn fever(&self) -> bool {
        self.combo >= FEVER_COMBO
    }

    fn bullets_for_combo(&self) -> u32 {
        (1 + self.combo / BULLET_COMBO_EVERY).min(MAX_BULLETS)
    }

    pub fn try_char(&mut self, ch: char) {
        if self.level_locked {
            return;
        }
        let expect = self
            .target
            .chars()
            .nth(self.progress.chars().count())
            .unwrap_or('\0');
        if expect == '\0' {
            return;
        }
        let ok = ch == expect
            || (expect == ' ' && ch == ' ')
            || (expect.is_ascii_alphabetic()
                && ch.to_ascii_lowercase() == expect.to_ascii_lowercase());
        if !ok {
            self.wrong_count += 1;
            self.combo = 0;
            self.wrong_flash = 0.25;
            return;
        }
        self.progress.push(expect);
        self.combo += 1;
        self.maybe_speak_completed_words();
        let n = self.bullets_for_combo();
        let cx = 480.0;
        let bottom = 540.0 - 36.0;
        let base_vy = if self.fever() { -620.0 } else { -520.0 };
        let spread = 13.0;
        for i in 0..n {
            let k = if n == 1 {
                0.0
            } else {
                i as f64 - (n as f64 - 1.0) / 2.0
            };
            self.bullets.push(Bullet {
                x: cx + k * spread,
                y: bottom,
                vy: base_vy + k * 18.0,
                r: if self.fever() { 7.0 } else { 5.0 },
                fever: self.fever(),
            });
        }
        let pts = (SCORE_PER_CHAR as f64 * self.combo_mult()).round() as u32;
        self.score += pts;
        if self.progress == self.target {
            self.level_locked = true;
            self.trigger_bomb();
        }
    }

    pub fn hint(&mut self) {
        if self.level_locked {
            return;
        }
        let plen = self.progress.chars().count();
        if plen >= self.target.chars().count() {
            return;
        }
        self.score = self.score.saturating_sub(25);
        self.hint_used = true;
        self.combo = 0;
        dom_bridge::tts_cancel();
        Self::speak_hint_at_cursor(&self.target, plen);
    }

    fn speak_hint_at_cursor(target: &str, plen: usize) {
        let t: Vec<char> = target.chars().collect();
        if plen >= t.len() {
            return;
        }
        let mut a = plen;
        while a > 0 && !t[a - 1].is_whitespace() {
            a -= 1;
        }
        let mut b = plen;
        while b < t.len() && !t[b].is_whitespace() {
            b += 1;
        }
        let slice: String = t[a..b].iter().collect();
        let slice = slice.trim();
        if !slice.is_empty() {
            dom_bridge::tts_speak_word(slice);
            return;
        }
        if plen < t.len() && t[plen].is_whitespace() {
            dom_bridge::tts_speak_word("space");
        }
    }

    fn trigger_bomb(&mut self) {
        dom_bridge::tts_speak_full_sentence(&self.target);
        self.bomb_active = true;
        self.bomb_t = 0.0;
        let cx = 480.0;
        let cy = 540.0 * 0.38;
        self.add_shockwave(cx, cy, "#ffffff", 0.45);
        self.add_shockwave(cx, cy, "#ffd166", 0.55);
        self.add_shockwave(cx, cy, "#ff6b8a", 0.65);
        let ms: Vec<_> = self.monsters.drain(..).collect();
        for m in ms {
            self.explode(m.x + m.w / 2.0, m.y + m.h / 2.0, "#ffd166", 14);
        }
        let mut bonus = SCORE_CLEAR;
        if self.wrong_count == 0 && !self.hint_used {
            bonus += PERFECT_BONUS;
        }
        self.score += bonus;
    }

    fn add_shockwave(&mut self, x: f64, y: f64, color: &str, max_life: f64) {
        self.shockwaves.push(crate::entities::Shockwave {
            x,
            y,
            r: 8.0,
            life: max_life,
            max_life,
            color: color.to_string(),
        });
    }

    fn explode(&mut self, x: f64, y: f64, color: &str, n: usize) {
        self.add_shockwave(x, y, color, 0.22);
        for i in 0..n {
            let a = PI * 2.0 * i as f64 / n as f64 + js_rand() * 0.55;
            let sp = 90.0 + js_rand() * 220.0;
            let sz = 2.0 + js_rand() * 5.0;
            self.particles.push(crate::entities::Particle {
                x,
                y,
                vx: a.cos() * sp,
                vy: a.sin() * sp,
                life: 0.5 + js_rand() * 0.4,
                color: color.to_string(),
                size: sz,
                rot: js_rand() * PI * 2.0,
                vr: (js_rand() - 0.5) * 10.0,
            });
        }
    }

    fn hit_monster(&mut self, bx: f64, by: f64, fever: bool) -> bool {
        let mut best_i: Option<usize> = None;
        let mut best_d = f64::INFINITY;
        for (i, m) in self.monsters.iter().enumerate() {
            let mx = m.x + m.w / 2.0;
            let my = m.y + m.h / 2.0;
            let d = ((bx - mx).powi(2) + (by - my).powi(2)).sqrt();
            if d < best_d {
                best_d = d;
                best_i = Some(i);
            }
        }
        if let Some(i) = best_i {
            if best_d < 130.0 {
                let m = self.monsters.remove(i);
                let c = if fever { "#ffb3c8" } else { "#6cf0c8" };
                let n = if fever { 22 } else { 16 };
                self.explode(m.x + m.w / 2.0, m.y + m.h / 2.0, c, n);
                return true;
            }
        }
        false
    }

    pub fn tick(&mut self, dt: f64) {
        if self.wrong_flash > 0.0 {
            self.wrong_flash -= dt;
        }
        if self.bomb_active {
            self.bomb_t += dt;
            if self.bomb_t > 0.85 {
                self.bomb_active = false;
                self.modal_open = true;
                self.modal_game_over = false;
            }
        }
        if !self.level_locked && !self.bomb_active {
            let lose_y = 540.0 - 50.0;
            let fever_slow = if self.fever() { 0.92 } else { 1.0 };
            for m in &mut self.monsters {
                m.y += m.speed * dt * fever_slow;
            }
            let breach: Vec<_> = self
                .monsters
                .iter()
                .filter(|m| m.y + m.h >= lose_y)
                .cloned()
                .collect();
            if !breach.is_empty() {
                self.monsters.retain(|m| m.y + m.h < lose_y);
                self.lives = self.lives.saturating_sub(1);
                self.combo = 0;
                self.score = self.score.saturating_sub(35);
                if self.lives == 0 {
                    self.level_locked = true;
                    self.modal_open = true;
                    self.modal_game_over = true;
                }
            }
        }
        if !self.level_locked {
            let old = std::mem::take(&mut self.bullets);
            let mut next = Vec::new();
            for mut b in old {
                b.y += b.vy * dt;
                if b.y < -10.0 {
                    continue;
                }
                if self.hit_monster(b.x, b.y, b.fever) {
                    continue;
                }
                next.push(b);
            }
            self.bullets = next;
        }
        for p in &mut self.particles {
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            p.vy += 320.0 * dt;
            p.vx *= 0.985;
            p.rot += p.vr * dt;
            p.life -= dt;
        }
        self.particles.retain(|p| p.life > 0.0);
        for s in &mut self.shockwaves {
            s.life -= dt;
            s.r += 420.0 * dt;
        }
        self.shockwaves.retain(|s| s.life > 0.0);
    }

    pub fn next_modal(&mut self) {
        if !self.modal_open {
            return;
        }
        if self.modal_game_over {
            self.start_level(self.level_index);
            return;
        }
        if self.level_index >= LEVELS.len() - 1 {
            self.level_index = 0;
            self.score = 0;
        } else {
            self.level_index += 1;
        }
        self.start_level(self.level_index);
    }
}
