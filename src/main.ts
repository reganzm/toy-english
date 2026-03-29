import init, { run, set_game_mode } from "../wasm-game/pkg/toy_english_wasm.js";
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

async function boot() {
  initTts();
  await init();
  installTtsBridge();
  const modeSelect = document.getElementById("game-mode") as HTMLSelectElement | null;
  const initialMode = modeSelect ? Number(modeSelect.value) : 2;
  run(initialMode);
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
