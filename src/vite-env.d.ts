/// <reference types="vite/client" />

interface ImportMetaEnv {
  /** 词表 API 根 URL，留空则用本地 mock */
  readonly VITE_LEVEL_API_BASE?: string;
  /** 设为 "false" 时使用内置嵌入式关卡 */
  readonly VITE_USE_LEVEL_API?: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
