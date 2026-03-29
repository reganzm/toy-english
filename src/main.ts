import init, {
  apply_api_level,
  handle_modal_next,
  is_modal_game_over,
  run,
  set_game_mode,
} from "../wasm-game/pkg/toy_english_wasm.js";
import { fetchLevelPage, type LevelItem } from "./levelApi";
import { renderSentenceAnalysisHtml } from "./sentenceAnalysis";
import {
  cancelSpeech,
  getTtsEnabled,
  initTts,
  primeTtsFromUserGesture,
  setTtsEnabled,
  speakEnglish,
  speakFullSentence,
  syncTtsButton,
} from "./tts";

function setupSettingsPanel(): void {
  const overlay = document.getElementById("settings-overlay");
  const btnOpen = document.getElementById("btn-settings");
  const btnClose = document.getElementById("btn-settings-close");
  const panel = overlay?.querySelector(".settings-panel");

  const open = () => {
    overlay?.classList.add("is-open");
    overlay?.setAttribute("aria-hidden", "false");
    btnOpen?.setAttribute("aria-expanded", "true");
    (panel as HTMLElement | undefined)?.focus();
  };

  const close = () => {
    overlay?.classList.remove("is-open");
    overlay?.setAttribute("aria-hidden", "true");
    btnOpen?.setAttribute("aria-expanded", "false");
    btnOpen?.focus();
  };

  btnOpen?.addEventListener("click", () => open());
  btnClose?.addEventListener("click", () => close());
  overlay?.addEventListener("click", (e) => {
    if (e.target === overlay) close();
  });
  document.addEventListener("keydown", (e) => {
    if (e.key === "Escape" && overlay?.classList.contains("is-open")) {
      e.preventDefault();
      close();
    }
  });
}

function installTtsBridge(): void {
  const w = window as Window & {
    __toyEnglishTtsSpeak?: (t: string) => void;
    __toyEnglishTtsSpeakFull?: (t: string) => void;
    __toyEnglishTtsCancel?: () => void;
  };
  w.__toyEnglishTtsSpeak = (t: string) => speakEnglish(t, { cancel: true });
  w.__toyEnglishTtsSpeakFull = (t: string) => speakFullSentence(t);
  w.__toyEnglishTtsCancel = () => cancelSpeech();
}

/** 当前页缓冲与游标（分页 API / mock） */
class LevelQueue {
  private buffer: LevelItem[] = [];
  private index = 0;
  private cursor: string | null = null;
  playing: LevelItem | null = null;

  async bootstrap(): Promise<void> {
    const page = await fetchLevelPage(null);
    this.buffer = page.items;
    this.index = 0;
    this.cursor = page.next_cursor;
    await this.ensureNonEmpty();
    this.playing = this.buffer[this.index] ?? null;
  }

  private async ensureNonEmpty(): Promise<void> {
    if (this.buffer.length > 0) return;
    const again = await fetchLevelPage(null);
    this.buffer = again.items;
    this.cursor = again.next_cursor;
  }

  /** 通关后进入下一题 */
  async advance(): Promise<void> {
    this.index += 1;
    if (this.index >= this.buffer.length) {
      const page = await fetchLevelPage(this.cursor);
      this.buffer = page.items;
      this.index = 0;
      this.cursor = page.next_cursor;
      await this.ensureNonEmpty();
    }
    this.playing = this.buffer[this.index] ?? null;
  }
}

function setupModalAnalysisObserver(queue: LevelQueue | null, useLevelApi: boolean): void {
  const overlay = document.getElementById("modal-overlay");
  const host = document.getElementById("modal-body-analysis");
  if (!overlay || !host) return;

  const sync = () => {
    if (!overlay.classList.contains("is-open")) {
      host.innerHTML = "";
      return;
    }
    if (!useLevelApi || !queue) {
      host.innerHTML = "";
      return;
    }
    if (is_modal_game_over()) {
      host.innerHTML = "";
      return;
    }
    host.innerHTML = renderSentenceAnalysisHtml(queue.playing?.analysis);
  };

  const mo = new MutationObserver(sync);
  mo.observe(overlay, { attributes: true, attributeFilter: ["class"] });
  sync();
}

async function boot() {
  initTts();
  await init();
  installTtsBridge();

  const useLevelApi = import.meta.env.VITE_USE_LEVEL_API !== "false";
  const queue = useLevelApi ? new LevelQueue() : null;
  if (queue) {
    await queue.bootstrap();
    if (!queue.playing) {
      throw new Error("词表为空：请检查 API 或 mock 数据");
    }
  }

  const modeSelect = document.getElementById("game-mode") as HTMLSelectElement | null;
  const initialMode = modeSelect ? Number(modeSelect.value) : 2;

  if (queue?.playing) {
    const p = queue.playing;
    run(initialMode, p.sentence, p.translation, p.id);
  } else {
    run(initialMode, undefined, undefined, undefined);
  }

  setupModalAnalysisObserver(queue, useLevelApi);

  modeSelect?.addEventListener("change", () => {
    set_game_mode(Number(modeSelect.value));
  });
  syncTtsButton();
  document.getElementById("btn-tts")?.addEventListener("click", () => {
    const next = !getTtsEnabled();
    setTtsEnabled(next);
    syncTtsButton();
    if (next) {
      primeTtsFromUserGesture();
      speakEnglish("Ready.", { rate: 1, cancel: true });
    }
  });

  setupSettingsPanel();

  document.getElementById("btn-next")?.addEventListener("click", async () => {
    const code = handle_modal_next();
    if (code === 3 && queue) {
      await queue.advance();
      const p = queue.playing;
      if (p) {
        apply_api_level(p.id, p.sentence, p.translation);
      }
    }
  });

  const primeOnce = () => {
    primeTtsFromUserGesture();
    window.removeEventListener("pointerdown", primeOnce, true);
    window.removeEventListener("keydown", primeOnce, true);
  };
  window.addEventListener("pointerdown", primeOnce, { capture: true, passive: true });
  window.addEventListener("keydown", primeOnce, { capture: true });
}

boot().catch((e) => {
  console.error(e);
  document.getElementById("app")!.innerHTML =
    "<p style='padding:1.5rem;font-family:sans-serif;line-height:1.6'>" +
      "<strong>未能加载 Rust / WebAssembly</strong><br/>" +
      "请先安装 Rust、<code>rustup target add wasm32-unknown-unknown</code> 与 <a href='https://rustwasm.github.io/wasm-pack/installer/'>wasm-pack</a>，" +
      "在项目根目录执行 <code>npm run build:wasm</code> 后再 <code>npm run dev</code>。" +
      "</p>";
});
