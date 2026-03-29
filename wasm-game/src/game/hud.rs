//! DOM HUD、弹窗、打字行 HTML
use super::constants::STARTING_LIVES;
use super::mode::GameMode;
use super::state::Game;
use crate::levels::LEVELS;
use crate::util::escape_html;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement};

impl Game {
    pub fn sync_dom(&self, doc: &Document) {
        if let Some(el) = doc.get_element_by_id("level-num") {
            el.set_text_content(Some(&self.level_display_id.to_string()));
        }
        if let Some(el) = doc.get_element_by_id("score") {
            el.set_text_content(Some(&self.score.to_string()));
        }
        if let Some(el) = doc.get_element_by_id("combo-num") {
            el.set_text_content(Some(&self.combo.to_string()));
        }
        if let Some(el) = doc.get_element_by_id("combo-mult") {
            el.set_text_content(Some(&format!("{:.2}", self.combo_mult())));
        }
        if let Some(el) = doc.get_element_by_id("combo-pill") {
            if let Ok(he) = el.dyn_into::<HtmlElement>() {
                he.set_hidden(self.combo == 0);
            }
        }
        if let Some(el) = doc.get_element_by_id("fever-badge") {
            if let Ok(he) = el.dyn_into::<HtmlElement>() {
                he.set_hidden(!self.fever());
            }
        }
        if let Some(el) = doc.get_element_by_id("lives") {
            let mut s = String::new();
            for i in 0..STARTING_LIVES {
                s.push(if i < self.lives { '♥' } else { '♡' });
            }
            el.set_text_content(Some(&s));
        }
        if let Some(el) = doc.get_element_by_id("prompt-line") {
            let inner = match self.mode {
                GameMode::Hell => String::from(
                    "<span class=\"prompt-hint\">只听音频，不显示原文。请听写听到的英文。</span>",
                ),
                GameMode::Hard | GameMode::Easy => format!(
                    "<span class=\"prompt-cn-label\">中文</span> {}",
                    escape_html(&self.level_translation)
                ),
            };
            el.set_inner_html(&inner);
        }
        if let Some(el) = doc.get_element_by_id("typed-line") {
            el.set_inner_html(&self.render_typed_html());
        }
        let Some(modal) = doc.get_element_by_id("modal-overlay") else {
            return;
        };
        let Some(html_el) = modal.dyn_ref::<HtmlElement>() else {
            return;
        };
        html_el.set_class_name(if self.modal_open {
            "modal-overlay is-open"
        } else {
            "modal-overlay"
        });
        html_el
            .set_attribute("aria-hidden", &(!self.modal_open).to_string())
            .ok();
        if self.modal_open {
            if let Some(title) = doc.get_element_by_id("modal-title") {
                if self.modal_game_over {
                    title.set_text_content(Some("游戏结束"));
                } else if self.use_api_levels {
                    title.set_text_content(Some("通关！"));
                } else if self.level_index >= LEVELS.len().saturating_sub(1).max(0) {
                    title.set_text_content(Some("全部通关！"));
                } else {
                    title.set_text_content(Some("通关！"));
                }
            }
            if let Some(body) = doc.get_element_by_id("modal-body-wasm") {
                let inner = if self.modal_game_over {
                    format!(
                        "<p class=\"modal-stats\">本局积分 <strong>{}</strong></p><p class=\"breakdown-trans\">再试一次，守住防线！</p>",
                        self.score
                    )
                } else {
                    format!(
                        "<p class=\"modal-stats\">积分 <strong>{}</strong> · 连击 <strong>{}</strong></p>\
                         <div class=\"breakdown-sentence\">{}</div>\
                         <div class=\"breakdown-trans\"><strong>翻译：</strong>{}</div>",
                        self.score,
                        self.combo,
                        escape_html(&self.target),
                        escape_html(&self.level_translation)
                    )
                };
                body.set_inner_html(&inner);
            }
            if let Some(btn) = doc.get_element_by_id("btn-next") {
                btn.set_text_content(Some(if self.modal_game_over {
                    "再试本关"
                } else if self.use_api_levels {
                    "下一关"
                } else if self.level_index >= LEVELS.len().saturating_sub(1).max(0) {
                    "再玩一次"
                } else {
                    "下一关"
                }));
            }
        }
    }

    fn render_typed_html(&self) -> String {
        if self.mode.shows_english() {
            self.render_typed_html_easy()
        } else {
            self.render_typed_html_masked()
        }
    }

    fn render_typed_html_easy(&self) -> String {
        let mut out = String::new();
        let t: Vec<char> = self.target.chars().collect();
        let plen = self.progress.chars().count();
        for (i, ch) in t.iter().enumerate() {
            let c = escape_html(&ch.to_string());
            if i < plen {
                out.push_str(&format!("<span class=\"ok\">{}</span>", c));
            } else if i == plen {
                out.push_str(&format!("<span class=\"pending\">{}</span>", c));
            } else {
                out.push_str(&format!(
                    "<span class=\"pending\" style=\"opacity:0.35\">{}</span>",
                    c
                ));
            }
        }
        if out.is_empty() {
            out.push_str("<span class=\"muted\">开始输入…</span>");
        }
        out
    }

    fn render_typed_html_masked(&self) -> String {
        let mut out = String::new();
        let t: Vec<char> = self.target.chars().collect();
        let pchars: Vec<char> = self.progress.chars().collect();
        let plen = pchars.len();
        for (i, ch) in t.iter().enumerate() {
            if i < plen {
                let c = escape_html(&pchars[i].to_string());
                out.push_str(&format!("<span class=\"ok\">{}</span>", c));
            } else if i == plen {
                let sym = if ch.is_whitespace() {
                    "<span class=\"pending mask-space\">␣</span>"
                } else {
                    "<span class=\"pending mask-slot\">_</span>"
                };
                out.push_str(sym);
            } else if ch.is_whitespace() {
                out.push_str("<span class=\"pending mask-space-dim\">·</span>");
            } else {
                out.push_str("<span class=\"pending mask-slot-dim\">_</span>");
            }
        }
        if out.is_empty() {
            out.push_str("<span class=\"muted\">听写中…</span>");
        }
        out
    }
}
