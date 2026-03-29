declare module "../wasm-game/pkg/toy_english_wasm.js" {
  export default function init(module_or_path?: unknown): Promise<void>;
  /** 失败时可能抛错（见 wasm-bindgen Result） */
  export function run(): void;
}
