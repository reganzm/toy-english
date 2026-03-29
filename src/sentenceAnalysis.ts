import type {
  AnalysisPhraseBlock,
  AnalysisSegment,
  AnalysisWordCard,
  LevelAnalysis,
  LevelToken,
} from "./levelApi";

function esc(s: string): string {
  return s
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}

/** 词性 → 胶囊配色（与设计稿紫/粉/青/红一致） */
function posPillModifier(pos?: string): string {
  if (!pos) return "analysis-pos-pill--muted";
  if (pos.includes("代")) return "analysis-pos-pill--pron";
  if (pos.includes("助动") || (pos.includes("动") && pos.length <= 4)) return "analysis-pos-pill--aux";
  if (pos.includes("形容")) return "analysis-pos-pill--adj";
  if (pos.includes("名")) return "analysis-pos-pill--noun";
  if (pos.includes("动")) return "analysis-pos-pill--verb";
  if (pos.includes("冠") || pos.includes("限定")) return "analysis-pos-pill--det";
  if (pos.includes("介")) return "analysis-pos-pill--prep";
  return "analysis-pos-pill--muted";
}

function tokenToWordCard(t: LevelToken): AnalysisWordCard {
  const pos = t.pos_zh ?? "";
  let theme: AnalysisWordCard["theme"] = "slate";
  if (pos.includes("代")) theme = "violet";
  else if (pos.includes("形容")) theme = "teal";
  else if (pos.includes("名")) theme = "rose";
  else if (pos.includes("动") || pos.includes("助")) theme = "amber";
  return {
    kind: "word",
    text: t.text,
    ipa: t.ipa,
    pos_zh: t.pos_zh,
    role_zh: t.role_zh,
    theme,
  };
}

function resolveSegments(a: LevelAnalysis): AnalysisSegment[] | null {
  if (a.segments?.length) return a.segments;
  if (a.tokens?.length) return a.tokens.map(tokenToWordCard);
  return null;
}

function renderPosPill(pos?: string): string {
  if (!pos) return "";
  const mod = posPillModifier(pos);
  return `<span class="analysis-pos-pill ${mod}">${esc(pos)}</span>`;
}

/** 句子成分（主语/谓语等）→ 与词性胶囊同风格的色块 */
function roleBadgeModifier(role: string): string {
  if (role.includes("主语")) return "analysis-role-badge--subject";
  if (role.includes("谓语")) return "analysis-role-badge--predicate";
  if (role.includes("宾语")) return "analysis-role-badge--object";
  if (role.includes("表语")) return "analysis-role-badge--predicative";
  if (role.includes("系动")) return "analysis-role-badge--linking";
  if (role.includes("状语")) return "analysis-role-badge--adverbial";
  if (role.includes("定语")) return "analysis-role-badge--attribute";
  if (role.includes("补语")) return "analysis-role-badge--complement";
  if (role.includes("同位")) return "analysis-role-badge--appositive";
  return "analysis-role-badge--default";
}

function renderRoleBadge(roleZh: string): string {
  const mod = roleBadgeModifier(roleZh);
  return `<span class="analysis-role-badge ${mod}">${esc(roleZh)}</span>`;
}

function renderWordCard(c: AnalysisWordCard): string {
  const theme = c.theme ?? "slate";
  const ipa = c.ipa ? `<span class="analysis-w-ipa">${esc(c.ipa)}</span>` : "";
  const role = c.role_zh ? renderRoleBadge(c.role_zh) : "";
  return `<div class="analysis-w-card analysis-w-card--${theme}">
    ${ipa}
    ${renderPosPill(c.pos_zh)}
    <span class="analysis-w-word">${esc(c.text)}</span>
    <div class="analysis-w-rule" aria-hidden="true"></div>
    ${role}
  </div>`;
}

function renderPhraseBlock(p: AnalysisPhraseBlock): string {
  const theme = p.theme ?? "maroon";
  const hasWords = p.words.length > 0;
  const body = hasWords
    ? `<div class="analysis-ph-units">${p.words
        .map((w) => {
          const ipa = w.ipa ? `<span class="analysis-ph-ipa">${esc(w.ipa)}</span>` : "";
          return `<div class="analysis-ph-unit">
            ${ipa}
            ${renderPosPill(w.pos_zh)}
            <span class="analysis-ph-lex">${esc(w.text)}</span>
          </div>`;
        })
        .join("")}</div>`
    : `<span class="analysis-ph-text">${esc(p.phrase_text)}</span>`;
  return `<div class="analysis-phrase-block analysis-phrase-block--${theme}">
    <div class="analysis-ph-box">
      ${body}
      <div class="analysis-ph-accentline" aria-hidden="true"></div>
      <div class="analysis-ph-role">${renderRoleBadge(p.role_zh)}</div>
    </div>
  </div>`;
}

function renderSegment(s: AnalysisSegment): string {
  if (s.kind === "phrase") return renderPhraseBlock(s);
  return renderWordCard(s);
}

/**
 * 通关弹窗：句子解析可视化（词卡 + 短语块；译文见弹窗上半部分，此处不重复）
 */
export function renderSentenceAnalysisHtml(a: LevelAnalysis | undefined): string {
  if (!a) {
    return '<p class="analysis-muted">（本题暂无解析数据）</p>';
  }

  const segments = resolveSegments(a);
  const flow =
    segments && segments.length > 0
      ? `<div class="analysis-flow">${segments.map(renderSegment).join("")}</div>`
      : "";

  const hasPhrase = segments?.some((s) => s.kind === "phrase");
  const constituents = a.constituents_zh?.length
    ? hasPhrase
      ? `<div class="analysis-constituents analysis-constituents--compact"><ul class="analysis-list">${a.constituents_zh.map((c) => `<li>${esc(c)}</li>`).join("")}</ul></div>`
      : `<div class="analysis-constituents"><div class="analysis-label">句子成分</div><ul class="analysis-list">${a.constituents_zh.map((c) => `<li>${esc(c)}</li>`).join("")}</ul></div>`
    : "";

  const notes = a.notes ? `<p class="analysis-notes-line">${esc(a.notes)}</p>` : "";

  if (!flow && !constituents && !notes) {
    return '<p class="analysis-muted">（本题暂无解析数据）</p>';
  }

  return `<div class="analysis-visual">${flow}${constituents}${notes}</div>`;
}
