/** 浏览器 Web Speech API（与 Rust WASM 通过 window 桥接） */

const STORAGE_KEY = "toy-english-tts";

function refreshVoices(): void {
  if (typeof speechSynthesis === "undefined") return;
  speechSynthesis.getVoices();
}

export function initTts(): void {
  if (typeof speechSynthesis === "undefined") return;
  refreshVoices();
  speechSynthesis.addEventListener("voiceschanged", refreshVoices);
}

export function isTtsSupported(): boolean {
  return typeof speechSynthesis !== "undefined" && typeof SpeechSynthesisUtterance !== "undefined";
}

export function getTtsEnabled(): boolean {
  if (!isTtsSupported()) return false;
  try {
    return localStorage.getItem(STORAGE_KEY) !== "0";
  } catch {
    return true;
  }
}

export function setTtsEnabled(on: boolean): void {
  try {
    localStorage.setItem(STORAGE_KEY, on ? "1" : "0");
  } catch {
    /* ignore */
  }
}

/** 解除 Chrome/Edge 等浏览器里常见的 paused 状态，并触发语音列表加载 */
export function prepareSpeechSynthesis(): void {
  if (typeof speechSynthesis === "undefined") return;
  try {
    if (speechSynthesis.paused) speechSynthesis.resume();
  } catch {
    /* ignore */
  }
  speechSynthesis.getVoices();
}

function pickEnglishVoice(): SpeechSynthesisVoice | undefined {
  const list = speechSynthesis.getVoices();
  if (list.length === 0) return undefined;
  const prefer = (pred: (v: SpeechSynthesisVoice) => boolean) => list.find(pred);
  return (
    prefer((v) => /en-US/i.test(v.lang) && /Microsoft|Google|Natural|Zira|Mark/i.test(v.name)) ||
    prefer((v) => /^en(-[A-Z]{2})?$/i.test(v.lang)) ||
    prefer((v) => v.lang.toLowerCase().startsWith("en"))
  );
}

export function unlockAudio(): void {
  if (typeof AudioContext !== "undefined") {
    const c = new AudioContext();
    if (c.state === "suspended") void c.resume();
    void c.close();
  }
}

export function cancelSpeech(): void {
  if (typeof speechSynthesis === "undefined") return;
  speechSynthesis.cancel();
}

function doSpeakUtterance(
  text: string,
  opts?: { rate?: number; pitch?: number; cancel?: boolean }
): void {
  const t = text.trim();
  if (!t) return;
  unlockAudio();
  prepareSpeechSynthesis();
  if (opts?.cancel !== false) speechSynthesis.cancel();
  const u = new SpeechSynthesisUtterance(t);
  u.lang = "en-US";
  const voice = pickEnglishVoice();
  if (voice) u.voice = voice;
  u.rate = opts?.rate ?? (t.length <= 2 ? 0.85 : 0.92);
  u.pitch = opts?.pitch ?? 1;
  u.onerror = (ev) => {
    console.warn("[TTS] 朗读失败（可检查系统是否安装英语语音）:", ev.error, ev);
  };
  try {
    speechSynthesis.speak(u);
  } catch (e) {
    console.warn("[TTS] speak() 抛出:", e);
  }
}

/**
 * Chromium 系浏览器首次调用时 getVoices() 常为空，需等 voiceschanged 或短时重试。
 * 超时后仍尝试朗读（使用浏览器默认音，不绑定 voice）。
 */
export function speakEnglish(
  text: string,
  opts?: { rate?: number; pitch?: number; cancel?: boolean }
): void {
  if (!isTtsSupported() || !getTtsEnabled()) return;

  const run = () => doSpeakUtterance(text, opts);

  if (speechSynthesis.getVoices().length > 0) {
    run();
    return;
  }

  let finished = false;
  const once = () => {
    if (finished) return;
    finished = true;
    speechSynthesis.removeEventListener("voiceschanged", once);
    run();
  };
  speechSynthesis.addEventListener("voiceschanged", once);
  speechSynthesis.getVoices();
  setTimeout(once, 350);
}

export function speakFullSentence(sentence: string): void {
  speakEnglish(sentence, { rate: 0.88, cancel: true });
}

/** 用户点击「朗读开」或首次点页面时调用，满足部分浏览器的用户手势要求 */
export function primeTtsFromUserGesture(): void {
  prepareSpeechSynthesis();
  unlockAudio();
}

export function syncTtsButton(): void {
  const btn = document.getElementById("btn-tts");
  if (!btn) return;
  if (!isTtsSupported()) {
    btn.hidden = true;
    return;
  }
  btn.hidden = false;
  const on = getTtsEnabled();
  btn.textContent = on ? "朗读开" : "朗读关";
  btn.classList.toggle("tts-off", !on);
}
