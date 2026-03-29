/**
 * 词表分页 API（后端占位）。LLM 预解析后的结构化字段与游戏对齐。
 */

export type LevelToken = {
  text: string;
  ipa?: string;
  pos_zh?: string;
  role_zh?: string;
  start?: number;
  end?: number;
};

/** 短语块内逐词的音标 / 词性（显示在红框上方） */
export type AnalysisPhraseWord = {
  text: string;
  ipa?: string;
  pos_zh?: string;
};

/**
 * 独立词卡片：音标 → 词性胶囊 → 词面 → 分隔线 → 句法角色（与设计稿一致）
 * theme：卡片底色倾向，也可由渲染层按词性推断
 */
export type AnalysisWordCard = {
  kind: "word";
  text: string;
  ipa?: string;
  pos_zh?: string;
  role_zh?: string;
  theme?: "violet" | "slate" | "teal" | "rose" | "maroon" | "amber";
};

/**
 * 短语成分块：上方多列 ipa+词性，中间整段短语+强调线，下方居中角色（如「主语」）
 */
export type AnalysisPhraseBlock = {
  kind: "phrase";
  phrase_text: string;
  role_zh: string;
  words: AnalysisPhraseWord[];
  /** 中间短语区域底色主题 */
  theme?: "maroon" | "wine" | "slate";
};

export type AnalysisSegment = AnalysisWordCard | AnalysisPhraseBlock;

export type LevelAnalysis = {
  /** 有则按顺序渲染词卡与短语块；无则回退为 tokens 词卡横排 */
  segments?: AnalysisSegment[];
  tokens?: LevelToken[];
  /** 句子成分简述（可选，显示在可视化下方） */
  constituents_zh?: string[];
  notes?: string;
};

export type LevelItem = {
  id: number;
  sentence: string;
  translation: string;
  analysis?: LevelAnalysis;
};

export type LevelPageResponse = {
  items: LevelItem[];
  next_cursor: string | null;
  has_more: boolean;
};

const MOCK_PAGE_A: LevelPageResponse = {
  items: [
    {
      id: 101,
      sentence: "What's your favourite animal?",
      translation: "你最喜爱的动物是什么？",
      analysis: {
        segments: [
          {
            kind: "word",
            text: "What",
            ipa: "/wɒt/",
            pos_zh: "代词",
            role_zh: "表语",
            theme: "violet",
          },
          {
            kind: "word",
            text: "'s",
            ipa: "/z/",
            pos_zh: "助动词",
            role_zh: "系动词",
            theme: "slate",
          },
          {
            kind: "phrase",
            phrase_text: "your favourite animal",
            role_zh: "主语",
            theme: "maroon",
            words: [
              { text: "your", ipa: "/jɔː(r)/", pos_zh: "代词" },
              { text: "favourite", ipa: "/ˈfeɪvərɪt/", pos_zh: "形容词" },
              { text: "animal", ipa: "/ˈænɪml/", pos_zh: "名词" },
            ],
          },
        ],
      },
    },
    {
      id: 104,
      sentence: "The quick brown fox jumps over the lazy dog.",
      translation: "敏捷的棕色狐狸跳过懒狗。",
      analysis: {
        tokens: [
          { text: "The", pos_zh: "冠词", role_zh: "限定名词" },
          { text: "quick", pos_zh: "形容词", ipa: "/kwɪk/" },
          { text: "brown", pos_zh: "形容词" },
          { text: "fox", pos_zh: "名词", role_zh: "主语核心" },
          { text: "jumps", pos_zh: "动词", role_zh: "谓语" },
          { text: "over", pos_zh: "介词" },
          { text: "the lazy dog", pos_zh: "名词短语", role_zh: "宾语" },
        ],
        constituents_zh: ["主语：The quick brown fox", "谓语：jumps over the lazy dog"],
      },
    },
    {
      id: 102,
      sentence: "She sells seashells by the seashore.",
      translation: "她在海边卖贝壳。",
      analysis: {
        tokens: [
          { text: "She", pos_zh: "代词", role_zh: "主语" },
          { text: "sells", pos_zh: "动词", role_zh: "谓语" },
          { text: "seashells", pos_zh: "名词", role_zh: "宾语" },
          { text: "by the seashore", pos_zh: "介词短语", role_zh: "状语" },
        ],
      },
    },
    {
      id: 103,
      sentence: "How much wood would a woodchuck chuck?",
      translation: "一只土拨鼠能扔多少木头？",
      analysis: {
        tokens: [
          { text: "How much wood", pos_zh: "名词短语", role_zh: "宾语（问句）" },
          { text: "would", pos_zh: "助动词" },
          { text: "a woodchuck", pos_zh: "名词短语", role_zh: "主语" },
          { text: "chuck", pos_zh: "动词", role_zh: "谓语" },
        ],
      },
    },
  ],
  next_cursor: "mock-page-2",
  has_more: true,
};

const MOCK_PAGE_B: LevelPageResponse = {
  items: [
    {
      id: 201,
      sentence: "Practice makes perfect.",
      translation: "熟能生巧。",
      analysis: {
        tokens: [
          { text: "Practice", pos_zh: "名词", role_zh: "主语" },
          { text: "makes", pos_zh: "动词", role_zh: "谓语" },
          { text: "perfect", pos_zh: "形容词", role_zh: "补语" },
        ],
      },
    },
  ],
  next_cursor: null,
  has_more: false,
};

function mockFetchPage(cursor: string | null): LevelPageResponse {
  if (cursor === "mock-page-2") {
    return MOCK_PAGE_B;
  }
  return MOCK_PAGE_A;
}

/**
 * GET `${base}/levels?cursor=...` 期望 JSON：`LevelPageResponse`。
 * 网络失败或未配置 base 时使用本地 mock（可循环分页）。
 */
export async function fetchLevelPage(cursor: string | null): Promise<LevelPageResponse> {
  const base = (import.meta.env.VITE_LEVEL_API_BASE as string | undefined)?.replace(/\/$/, "") ?? "";
  if (!base) {
    return Promise.resolve(mockFetchPage(cursor));
  }
  const url = new URL(`${base}/levels`);
  if (cursor) url.searchParams.set("cursor", cursor);
  try {
    const res = await fetch(url.toString(), { headers: { Accept: "application/json" } });
    if (!res.ok) throw new Error(String(res.status));
    return (await res.json()) as LevelPageResponse;
  } catch (e) {
    console.warn("[levelApi] fetch failed, using mock", e);
    return mockFetchPage(cursor);
  }
}
