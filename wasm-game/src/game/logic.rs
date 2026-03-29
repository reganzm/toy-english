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
    pub fn new_unstarted(mode: GameMode) -> Self {
        Self {
            mode,
            level_index: 0,
            level_display_id: 1,
            use_api_levels: false,
            score: 0,
            target: String::new(),
            progress: String::new(),
            monsters: Vec::new(),
            bullets: Vec::new(),
            particles: Vec::new(),
            shockwaves: Vec::new(),
            combo: 0,
            lives: STARTING_LIVES,
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
            level_translation: String::new(),
        }
    }

    pub fn apply_mode(&mut self, mode: GameMode) {
        self.mode = mode;
        if self.use_api_levels {
            let s = self.target.clone();
            let t = self.level_translation.clone();
            let id = self.level_display_id;
            self.start_api_level(id, s, t);
        } else {
            self.start_embedded_level(self.level_index);
        }
    }

    pub fn start_embedded_level(&mut self, idx: usize) {
        self.use_api_levels = false;
        self.level_index = idx.min(LEVELS.len().saturating_sub(1).max(0));
        let lv = &LEVELS[self.level_index.min(LEVELS.len() - 1)];
        self.level_display_id = lv.id;
        self.target = lv.sentence.to_string();
        self.level_translation = lv.translation.to_string();
        self.reset_level_combat();
    }

    pub fn start_api_level(&mut self, level_id: u32, sentence: String, translation: String) {
        self.use_api_levels = true;
        self.level_display_id = level_id;
        self.target = sentence;
        self.level_translation = translation;
        self.reset_level_combat();
    }

    fn reset_level_combat(&mut self) {
        self.level_locked = false;
        self.progress.clear();
        self.combo = 0;
        self.lives = STARTING_LIVES;
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
        self.spawn_monsters();
        self.rebuild_tts_plan();
        if self.mode.dictation_listen() {
            dom_bridge::tts_speak_full_sentence(&self.target);
        }
    }

    pub fn restart_current_after_game_over(&mut self) {
        self.modal_open = false;
        self.modal_game_over = false;
        self.reset_level_combat();
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
        let span = (SPAWN_BASE_MAX - SPAWN_BASE_MIN + 1) as f64;
        let base = SPAWN_BASE_MIN + (js_rand() * span).floor() as usize;
        let ch = self.target.chars().count();
        let len_bonus = ch
            .saturating_sub(SPAWN_LEN_START)
            .saturating_mul(SPAWN_EXTRA_PER_CHAR)
            .min(SPAWN_LEN_BONUS_CAP);
        let lv_bonus = self.level_index.saturating_mul(SPAWN_PER_LEVEL_INDEX);
        let count = (base + len_bonus + lv_bonus).min(SPAWN_HARD_CAP);
        for i in 0..count {
            let col = (i % 8) as f64;
            let row = (i / 8) as f64;
            self.monsters.push(Monster {
                x: 80.0 + col * 100.0 + (js_rand() * 24.0 - 12.0),
                y: -60.0 - row * 55.0 - js_rand() * 40.0,
                w: 44.0 + js_rand() * 16.0,
                h: 36.0 + js_rand() * 12.0,
                speed: 28.0 + js_rand() * 45.0,
                hue: js_rand() * 360.0,
                sat: 50.0 + js_rand() * 44.0,
                lit: 44.0 + js_rand() * 26.0,
                phase: js_rand() * PI * 2.0,
                animal: ((js_rand() * 20.0).floor() as u8).min(19),
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

    fn gun_muzzle_world(now_secs: f64) -> (f64, f64) {
        let bob = (now_secs * GUN_BOB_FREQ).sin() * GUN_BOB_AMP;
        (
            GUN_ANCHOR_X,
            GUN_ANCHOR_Y + bob + GUN_MUZZLE_LOCAL_Y,
        )
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
        let want = self.bullets_for_combo();
        let room = (MAX_BULLETS as usize).saturating_sub(self.bullets.len());
        let n = want.min(room as u32);
        let now_secs = crate::util::js_now_ms() / 1000.0;
        let (mx, my) = Self::gun_muzzle_world(now_secs);
        let spread = 5.0;
        let speed = if self.fever() {
            BULLET_SPEED_FEVER
        } else {
            BULLET_SPEED
        };
        for i in 0..n {
            let k = if n == 1 {
                0.0
            } else {
                i as f64 - (n as f64 - 1.0) / 2.0
            };
            let sx = mx + k * spread;
            let sy = my;
            let (vx, vy) = Self::bullet_initial_velocity(sx, sy, speed, &self.monsters);
            self.bullets.push(Bullet {
                x: sx,
                y: sy,
                vx,
                vy,
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
                let c = if fever {
                    format!(
                        "hsl({:.0}, {:.0}%, {:.0}%)",
                        m.hue,
                        (m.sat + 12.0).min(100.0),
                        (m.lit + 18.0).min(82.0)
                    )
                } else {
                    format!(
                        "hsl({:.0}, {:.0}%, {:.0}%)",
                        m.hue,
                        (m.sat + 6.0).min(100.0),
                        (m.lit + 10.0).min(76.0)
                    )
                };
                let n = if fever { 22 } else { 16 };
                self.explode(m.x + m.w / 2.0, m.y + m.h / 2.0, &c, n);
                return true;
            }
        }
        false
    }

    fn nearest_monster_center(monsters: &[Monster], x: f64, y: f64) -> Option<(f64, f64)> {
        if monsters.is_empty() {
            return None;
        }
        let mut best_d = f64::INFINITY;
        let mut best = (0.0_f64, 0.0_f64);
        for m in monsters {
            let mx = m.x + m.w / 2.0;
            let my = m.y + m.h / 2.0;
            let d = ((x - mx).powi(2) + (y - my).powi(2)).sqrt();
            if d < best_d {
                best_d = d;
                best = (mx, my);
            }
        }
        Some(best)
    }

    fn bullet_initial_velocity(x: f64, y: f64, speed: f64, monsters: &[Monster]) -> (f64, f64) {
        if let Some((tx, ty)) = Self::nearest_monster_center(monsters, x, y) {
            let dx = tx - x;
            let dy = ty - y;
            let d = (dx * dx + dy * dy).sqrt().max(1e-9);
            (dx / d * speed, dy / d * speed)
        } else {
            (0.0, -speed)
        }
    }

    fn steer_bullet_toward_nearest(b: &mut Bullet, monsters: &[Monster], dt: f64) {
        let speed = if b.fever {
            BULLET_SPEED_FEVER
        } else {
            BULLET_SPEED
        };
        let blend = (BULLET_HOMING * dt).min(1.0);
        if let Some((tx, ty)) = Self::nearest_monster_center(monsters, b.x, b.y) {
            let dx = tx - b.x;
            let dy = ty - b.y;
            let d = (dx * dx + dy * dy).sqrt().max(1e-9);
            let tvx = dx / d * speed;
            let tvy = dy / d * speed;
            b.vx += (tvx - b.vx) * blend;
            b.vy += (tvy - b.vy) * blend;
        } else {
            b.vx += (0.0 - b.vx) * blend;
            b.vy += (-speed - b.vy) * blend;
        }
        let cur = (b.vx * b.vx + b.vy * b.vy).sqrt();
        if cur > 1e-9 {
            b.vx *= speed / cur;
            b.vy *= speed / cur;
        }
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
                m.y += m.speed * MONSTER_DESCENT_SCALE * dt * fever_slow;
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
                Self::steer_bullet_toward_nearest(&mut b, &self.monsters, dt);
                b.x += b.vx * dt;
                b.y += b.vy * dt;
                if b.x < -48.0 || b.x > 1008.0 || b.y < -48.0 || b.y > 588.0 {
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

    /// 嵌入式关卡用：与旧 `next_modal` 行为一致（由 TS 在不需要 API 时也可调用）
    pub fn next_modal_embedded_only(&mut self) {
        if !self.modal_open {
            return;
        }
        if self.modal_game_over {
            self.restart_current_after_game_over();
            return;
        }
        if self.level_index >= LEVELS.len().saturating_sub(1).max(0) {
            self.level_index = 0;
            self.score = 0;
        } else {
            self.level_index += 1;
        }
        self.start_embedded_level(self.level_index);
    }

    /// 返回：0=未处理；1=已处理游戏结束重开；2=已切嵌入式下一关；3=API 模式已关弹窗，需 TS 拉下一题并 `apply_api_level`
    pub fn handle_modal_next(&mut self) -> u8 {
        if !self.modal_open {
            return 0;
        }
        if self.modal_game_over {
            self.restart_current_after_game_over();
            return 1;
        }
        if self.use_api_levels {
            self.modal_open = false;
            return 3;
        }
        self.next_modal_embedded_only();
        2
    }
}
