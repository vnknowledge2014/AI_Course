/**
 * Schema nội dung v2 — HỢP ĐỒNG TRUNG TÂM.
 *
 * Mọi package khác phụ thuộc vào file này. **ĐÓNG BĂNG** trước khi viết lesson
 * đầu tiên: sửa schema sau đó nghĩa là sửa lại mọi bài đã viết.
 *
 * Nguồn: `MASTERPLAN.md` §5. Trích tự động bởi `tools/trich_schema.py` —
 * nếu cần đổi, sửa MASTERPLAN rồi trích lại, đừng sửa trực tiếp ở đây.
 */

export const SCHEMA_VERSION = '2.0.0' as const;

/* ═══════════════ 1. ĐỊNH DANH ═══════════════ */

/** Mở rộng được: track DB cần surrealql/cql, không phá hợp đồng về sau. */
export type LangId =
  | 'python' | 'typescript' | 'rust'
  | 'surrealql' | 'cql' | 'sql' | 'text';

export type LocaleId = 'vi' | 'en';

export type Platform =
  | 'web-chrome' | 'web-firefox'
  | 'macos' | 'windows' | 'linux'
  | 'android' | 'ios';

export type TrackId =
  | 'onboarding' | 'programming-101' | 'math-discrete' | 'cs-core'
  | 'fp-core' | 'software-eng' | 'database-scratch'
  | 'system-design' | 'ai-genai-rag' | 'ai-applied';

export type Level = 'intro' | 'beginner' | 'intermediate' | 'advanced' | 'expert';

export type ConceptId = string;  // 'fp.functor.map'  — dotted, ổn định, không dịch
export type SkillId   = string;  // 'math.set.subset' — đơn vị đo mastery
export type LessonId  = string;  // '<track>.<module>.<slug>'
export type StepId    = string;  // '<lessonId>#<slug>'
export type ModuleId  = string;

/* ═══════════════ 2. RICHTEXT (compile sẵn, runtime không parse markdown) ═══════════════ */

export type RichNode =
  | { t: 'p';  c: Inline[] }
  | { t: 'h';  lvl: 2 | 3 | 4; c: Inline[] }
  | { t: 'ul' | 'ol'; items: RichNode[][] }
  | { t: 'code'; lang: LangId | 'bash' | 'json'; src: string;
      highlight?: [number, number][]; annotations?: { line: number; note: Inline[] }[] }
  | { t: 'callout'; variant: 'tip'|'warning'|'important'|'note'|'pitfall'; c: RichNode[] }
  | { t: 'table'; head: Inline[][]; rows: Inline[][][] }
  | { t: 'math'; tex: string; display: boolean }   // render: SVG pre-render lúc build
  | { t: 'img'; assetId: string; alt: string }     // alt BẮT BUỘC
  | { t: 'figure'; world: WorldRef; caption?: Inline[] };

export type Inline =
  | { t: 'txt'; v: string }
  | { t: 'b' | 'i' | 'code'; c: Inline[] | string }
  | { t: 'link'; href: string; c: Inline[] }
  | { t: 'concept'; id: ConceptId; c: Inline[] }   // tooltip + "ôn lại"
  | { t: 'kbd'; v: string }
  | { t: 'math'; tex: string };

export type RichText = RichNode[];

/* ═══════════════ 3. BYTE ═══════════════ */

export type ByteMood =
  | 'idle' | 'thinking' | 'happy' | 'blocked' | 'dizzy' | 'curious';
// CỐ Ý KHÔNG CÓ 'sad' / 'disappointed'. Sai là dữ liệu, không phải thất bại.

export type BytePose =
  | 'idle' | 'lean-in' | 'point-editor' | 'point-stage' | 'jump';

export type ByteTrigger =
  | { on: 'enter' }
  | { on: 'success' }
  | { on: 'fail-same-error'; nth: 2 }        // CHỈ khi lỗi GIỐNG NHAU lần thứ 2
  | { on: 'idle'; afterMs: 90_000 }
  | { on: 'suboptimal'; ratioOverOptimal: number }
  | { on: 'hint'; rung: number };

export interface ByteBeat {
  trigger: ByteTrigger;
  mood: ByteMood;
  pose?: BytePose;
  line: RichText;                    // ≤ 90 ký tự, đã là tiếng Việt
  spotlight?: 'editor' | 'stage' | 'console' | 'hint-button';
  /** Không bao giờ dùng thuật ngữ chưa dạy. Director lọc theo mastery. */
  requiresConcepts?: ConceptId[];
  cooldownMs: number;
  maxPerPage: 1;
}

/* ═══════════════ 4. WORLD & LIVE VIEW ═══════════════ */

export type RendererId = 'pixi' | 'svg';

export type WorldFamily =
  | 'grid-bot'        // lập trình nền tảng
  | 'machine-belt'    // FP: hàm thuần, compose, functor/monad
  | 'memory-city'     // CS + ownership Rust
  | 'set-garden'      // tập hợp, Venn, số học
  | 'graph-lab'       // đồ thị, quan hệ, ma trận kề
  | 'proof-bench'     // logic, luật suy diễn (bước sai → dây KHÔNG nối được)
  | 'number-line'     // số, phân số, toạ độ
  | 'blueprint-board' // DDD: sum type hở → bug BÒ RA
  | 'storage'         // page 4KB, B+Tree split/merge
  | 'lsm'             // memtable → flush → level, đếm amplification
  | 'ring'            // token ring, vnode, shard lane, quorum
  | 'ledger'          // double-entry, pending vs posted
  | 'vsr'             // replica, message arrow, view number, partition
  | 'traffic-city'    // system design: p50/p95/p99 + $/tháng
  | 'context-window'  // cửa sổ token như thanh nhiên liệu, thấy block bị evict
  | 'agent-trace';    // vòng lặp/graph agent chạy thật

export interface WorldRef {
  family: WorldFamily;
  params: Record<string, unknown>;
  seed: number;
}

export interface LiveViewConfig {
  world: WorldRef;
  renderer: RendererId;
  /** Số biểu diễn LIÊN KẾT ĐỘNG. >=2 ⇒ mobile BẮT BUỘC stack dọc, CẤM tab. */
  linkedViews: number;
  /** Sàn theo renderer: pixi >= 320x240 ; svg >= 360x420 (chiều CAO là ràng buộc thật) */
  minViewport: { w: number; h: number };
  onSuccess: 'pulse' | 'confetti' | 'flow' | 'none';
  onFailure: 'freeze-at-step' | 'shake' | 'none';
}

/** Hợp đồng code-học-viên → world. Code KHÔNG BAO GIỜ gọi PixiJS trực tiếp. */
export interface TraceEvent {
  t: number;                          // step counter, KHÔNG phải wall clock
  kind: 'call' | 'return' | 'alloc' | 'drop' | 'move' | 'borrow' | 'release'
      | 'print' | 'assign' | 'panic' | 'flush' | 'compact' | 'send' | 'recv'
      | 'tool-call' | 'evict' | 'retrieve';
  payload: Record<string, unknown>;   // ĐÃ CHUẨN HOÁ + SẮP XẾP (xem probe)
  depth: number;
}

/* ═══════════════ 5. CODE & PROBE ═══════════════ */

export type PolyCode = Partial<Record<LangId, string>>;

export interface CodeSlot {
  language: LangId;
  starter: string;
  /** BẮT BUỘC. Compiler dùng để self-test trong runtime shipped. */
  solution: string;
  test?: string;
  prelude?: string;                   // chạy trước, ẩn khỏi editor
  epilogue?: string;
  /** Chế độ "thử-đi": chỉ vùng này sửa được, phần còn lại readonly */
  editableRegions?: [number, number][];
  suggestions: string[];              // snippet cho suggestion bar (mobile)
  /** Biến global mà probe chiếu ra WorldPatch. Chuẩn hoá + SORT trước khi serialize. */
  probeVars?: string[];
  needsDom?: boolean;
}

/* ═══════════════ 6. VALIDATION NHIỀU TẦNG ═══════════════ */

export type ValidationTier =
  | 'static'     // AST query (KHÔNG regex trên source)
  | 'compile'    // tsc --noEmit / byte-rust typecheck / mypy
  | 'run'        // chạy được, không throw
  | 'tests'      // assertion trong test fence pass
  | 'output'     // stdout khớp (exact|trim|regex|json-deep)
  | 'trace'      // invariant/property trên TraceEvent[] của CHÍNH học viên
  | 'property'   // ∀-property + shrinking, seed đổi mỗi lần nộp
  | 'complexity' // hồi quy log-log trên step counter
  | 'style';     // lint/idiom — advisory, không chặn

export type Severity = 'blocking' | 'advisory';
export type HostReq  = 'wasm' | 'native-toolchain';
export type CapReq   = 'sab' | 'webgpu' | 'ts-checker' | 'surreal-wasm' | 'embed-model';

/** Discriminated theo NGÔN NGỮ — chống lỗi "no-unwrap trong lesson Python". */
export type AstQuery =
  | { lang: 'python'; kind: PyAstKind; target?: string; min?: number }
  | { lang: 'rust';   kind: RsAstKind; target?: string; min?: number }
  | { lang: 'typescript'; kind: TsAstKind; target?: string; min?: number };

export type PyAstKind =
  | 'match-stmt' | 'comprehension' | 'lambda' | 'recursion'
  | 'frozen-dataclass' | 'no-mutation' | 'pure-fn' | 'no-global'
  | 'uses-generator' | 'no-import';
export type RsAstKind =
  | 'match-expr' | 'enum-def' | 'impl-trait' | 'no-unwrap'
  | 'no-clone-in-loop' | 'uses-iterator-chain' | 'trait-bound' | 'no-mut-binding';
export type TsAstKind =
  | 'discriminated-union' | 'readonly-modifier' | 'no-let'
  | 'no-any' | 'exhaustive-switch' | 'uses-pipe' | 'no-nonnull-assert';

export type ValidationRule =
  | { tier: 'static'; id: string; severity?: Severity;
      requireAst?: AstQuery[]; forbidAst?: AstQuery[]; onFail: string }
  | { tier: 'compile'; id: string; severity?: Severity; strict?: boolean;
      requiresHost?: HostReq; requiresCaps?: CapReq[];
      /** Bài "phải làm compiler báo lỗi" — dạy được nhờ trường này. */
      expectDiagnostics?: { code: string; line?: number }[]; onFail?: string }
  | { tier: 'run'; id: string; timeoutMs: number; maxSteps?: number;
      requiresHost?: HostReq; onFail?: string }
  | { tier: 'tests'; id: string; testCode?: PolyCode; timeoutMs: number;
      requiresHost?: HostReq; onFail?: string }
  | { tier: 'output'; id: string; expected: string;
      match: 'exact' | 'trim' | 'regex' | 'json-deep';
      /** R-B: expected do CI sinh bằng rustc/CPython THẬT, lưu như DỮ LIỆU. */
      generatedBy?: { toolchain: string; version: string; at: string };
      onFail?: string }
  | { tier: 'trace'; id: string; assertions: TraceAssertion[]; onFail?: string }
  | { tier: 'property'; id: string; generator: PolyCode; runs: number;
      invariant: PolyCode; onFail?: string }
  | { tier: 'complexity'; id: string;
      /** Hồi quy log(steps) theo log(n) trên n ∈ ns; so alpha kỳ vọng ±0.35.
       *  CẤM dùng để phân biệt O(n) với O(n log n) — hai lớp đó KHÔNG tách được. */
      ns: number[]; expectedAlpha: number; tolerance: 0.35;
      /** Chỉ đếm thao tác MIỀN do std phát ra, không đếm step nội bộ interpreter. */
      countOnly: 'domain-ops'; severity?: Severity; onFail?: string }
  | { tier: 'style'; id: string; lint: 'ruff' | 'clippy' | 'eslint';
      maxWarnings: number; severity: 'advisory'; onFail?: string };

/** Vị từ trên chuỗi TraceEvent — KHÔNG so khớp bằng nhau với trace hệ thật. */
export interface TraceAssertion {
  id: string;
  desc: string;                          // tiếng Việt, hiện cho học viên
  kind: 'invariant' | 'ordering' | 'count' | 'ratio' | 'absence';
  expr: string;                          // DSL nhỏ, đánh giá trong sandbox
  tolerance?: number;
}

export interface ValidationSpec {
  rules: ValidationRule[];
  stopOnFirstBlocking: boolean;
  /** Rule advisory fail ⇒ mất huy hiệu `Tinh gọn`, KHÔNG chặn qua bài. */
  advisoryAffectsBadge: 'tinh-gon' | 'none';
}

/* ═══════════════ 7. TOÁN — Khan Academy / New Math ═══════════════ */

export type AnswerSpec =
  | { kind: 'numeric'; value: number; tolerance: number; unit?: string }
  | { kind: 'math-expr'; canonical: string; vars: string[];
      /** Tương đương ký hiệu bằng sampling: đánh giá 2 biểu thức tại N điểm ngẫu
       *  nhiên theo seed. Thuần TS, offline, không cần CAS. */
      equivalence: 'symbolic-sampling'; samples: number }
  | { kind: 'choice'; correct: string[]; multi: boolean }
  | { kind: 'ordering'; correct: string[] }
  | { kind: 'matching'; pairs: [string, string][] }
  | { kind: 'set-membership'; correct: string[] }   // "kéo vật vào rổ"
  | { kind: 'free-text'; gradable: false };         // KHÔNG BAO GIỜ cấp mastery

/** Sinh đề tham số hoá tất định theo seed — cùng seed = cùng đề. */
export interface GeneratorSpec {
  id: string;
  params: Record<string, { min: number; max: number; step?: number }>;
  /** Sinh {đề, đáp án, lời giải từng bước} — code TS thuần, có test riêng. */
  moduleId: string;
}

/* ═══════════════ 8. HINT (thang 5 bậc, thay hints[] phẳng) ═══════════════ */

export type HintKind =
  | 'attention'     // "Nhìn xem Byte đang quay mặt về hướng nào"
  | 'strategy'      // "Cần lặp 4 lần — có cách nào không phải chép 4 dòng?"
  | 'watch-byte'    // ★ MỚI: phát ANIMATION lời giải, KHÔNG hiện code
  | 'one-line'      // một dòng code cụ thể
  | 'solution';     // đầy đủ — BẮT BUỘC kèm task `trace` ngay sau

export interface HintRung {
  kind: HintKind;
  body: RichText;
  autoRevealAfterFailures?: number;   // chống bế tắc; solution KHÔNG BAO GIỜ auto
  byte?: ByteBeat;
}

export interface HintLadder {
  rungs: HintRung[];                  // index 0 = ít spoiler nhất
  /** Sau bậc `solution`, học viên PHẢI qua một step `trace` mới đi tiếp. */
  requireTraceAfterSolution: true;
}

/* ═══════════════ 9. STEP — union 12 nhánh ═══════════════ */

interface StepBase {
  id: StepId;
  title?: string;
  byte?: ByteBeat[];
  liveView?: LiveViewConfig;
  teaches?: SkillId[];
  requires?: SkillId[];
  concepts?: ConceptId[];
  estimatedSeconds: number;
  /** Item ôn tập FSRS chỉ lấy từ step có cờ này + có >=3 biến thể. */
  reviewable?: boolean;
  variantId?: string;
  determinism?: boolean;              // bật determinism harness (clock ảo + PRNG seed)
}

interface GradableBase extends StepBase {
  validation: ValidationSpec;
  hints: HintLadder;
  /** Số hành động tối ưu — dùng cho huy hiệu `Tinh gọn` VÀ chống brute-force. */
  optimalActions?: number;
  timeoutMs: number;
  maxSteps?: number;
}

export interface ExplainStep    extends StepBase { kind: 'explain'; body: RichText }
export interface ExampleStep    extends StepBase { kind: 'example'; body?: RichText;
                                                   code: CodeSlot; runnable: boolean }
export interface AssembleStep   extends GradableBase { kind: 'assemble';  // kéo block, không gõ
                                                   body: RichText; blocks: string[];
                                                   code: CodeSlot }
export interface CodeStep       extends GradableBase { kind: 'code';
                                                   body: RichText; code: CodeSlot }
export interface PredictStep    extends GradableBase { kind: 'predict';   // cam kết TRƯỚC khi chạy
                                                   body: RichText; answer: AnswerSpec;
                                                   code?: CodeSlot;
                                                   commitOnce: true }     // chỉ lần đầu tính điểm
export interface ManipulateStep extends GradableBase { kind: 'manipulate';
                                                   body: RichText; answer: AnswerSpec }
export interface TuneStep       extends GradableBase { kind: 'tune';
                                                   body: RichText;
                                                   knobs: { id: string; min: number;
                                                            max: number; step: number }[] }
export interface TraceStep      extends GradableBase { kind: 'trace';     // điền bảng trạng thái
                                                   body: RichText; code: CodeSlot;
                                                   rows: { at: number; vars: string[] }[] }
export interface RepairStep     extends GradableBase { kind: 'repair';
                                                   body: RichText; code: CodeSlot }
export interface RefactorStep   extends GradableBase { kind: 'refactor';  // đúng rồi, làm gọn hơn
                                                   body: RichText; code: CodeSlot }
export interface ClassifyStep   extends GradableBase { kind: 'classify';  // ĐỊNH NGHĨA ĐƯỢC PHÁT HIỆN
                                                   body: RichText;
                                                   items: { id: string; label: RichText;
                                                            correct: boolean }[];
                                                   requireWhy: boolean }
export interface CheckpointStep extends StepBase { kind: 'checkpoint';
                                                   quizzes: (PredictStep | ClassifyStep)[];
                                                   masteryThreshold: number;
                                                   gatesProgress: boolean }
export interface SandboxStep    extends StepBase { kind: 'sandbox'; code: CodeSlot }
export interface ReflectStep    extends StepBase { kind: 'reflect'; prompt: RichText;
                                                   rubric: RichText; gradable: false }

export type Step =
  | ExplainStep | ExampleStep | AssembleStep | CodeStep | PredictStep
  | ManipulateStep | TuneStep | TraceStep | RepairStep | RefactorStep
  | ClassifyStep | CheckpointStep | SandboxStep | ReflectStep;

/* ═══════════════ 10. LESSON ═══════════════ */

export type ContentTier = 'A' | 'B' | 'C';
// A = tương tác đầy đủ, viết tay, chấm thật, cấp mastery
// B = prose từ legacy + 2-3 task predict/trace sinh bán tự động, reviewed=false
//     ⇒ KHÔNG cấp Proficient/Mastered
// C = chế độ đọc thuần + glossary, mastery mức chương ⇒ KHÔNG lĩnh vực nào trống

export interface Lesson {
  schemaVersion: typeof SCHEMA_VERSION;
  id: LessonId;
  locale: LocaleId;
  track: TrackId;
  module: ModuleId;
  order: number;
  title: string;
  summary: string;
  level: Level;
  tier: ContentTier;
  languages: LangId[];
  defaultLanguage: LangId;
  /** Ngôn ngữ CHỦ của khái niệm này. 2 ngôn ngữ kia chỉ có Bước Rosetta. */
  homeLanguage?: LangId;
  rosettaOf?: LessonId;
  estimatedMinutes: number;            // 7..15, lint cưỡng chế
  steps: Step[];
  teaches: SkillId[];
  practices: SkillId[];
  requires: SkillId[];
  concepts: ConceptId[];
  assets: string[];
  /** Nền tảng nào chấm được tier nào. Thiếu ⇒ BUILD FAIL cho lesson Rust. */
  gradingMatrix: Partial<Record<Platform, ValidationTier[]>>;
  /** Bắt buộc khi lesson có step requiresHost:'native-toolchain'. */
  fallback?: { kind: 'recorded-trace' | 'reading-only'; ref?: string };
  requiresCaps?: CapReq[];
  provenance: {
    migratedFrom?: string;             // 'fp/Rust_Books/.../chapter_12_*.md'
    anchorId?: string;                 // neo ổn định, KHÔNG phải dải dòng
    sourceHash?: string;               // sha256 khối gốc — phát hiện drift
    authoredBy: 'human' | 'migration' | 'llm-assisted';
    reviewed: boolean;                 // false ⇒ chỉ vào bundle `beta`
  };
}

/* ═══════════════ 11. SKILL / CONCEPT GRAPH ═══════════════ */

export interface Skill {
  id: SkillId;
  label: string;
  /** ĐỊNH NGHĨA CHUẨN TIẾNG VIỆT DUY NHẤT. >1 định nghĩa ⇒ BUILD FAIL. */
  definition: RichText;
  track: TrackId;
  requires: SkillId[];
  languageAgnostic: boolean;
  bloom: 'remember' | 'understand' | 'apply' | 'analyze' | 'create';
  /** FSRS chỉ bật cho skill có >=3 biến thể review. Lint cưỡng chế. */
  reviewVariants: string[];
}

export interface SkillGraph { version: string; skills: Skill[] } // compiler đảm bảo DAG

/* ═══════════════ 12. MASTERY & TIẾN ĐỘ ═══════════════ */

export type MasteryLevel = 'not-started' | 'attempted' | 'familiar' | 'proficient' | 'mastered';
export type Badge = 'hoan-thanh' | 'tu-luc' | 'tinh-gon' | 'giai-thich-duoc';

export interface StepAttempt {
  stepId: StepId; lessonId: LessonId; at: number;
  /** ★ Chống đoán mò: mastery CHỈ tính lần trả lời đầu tiên. */
  firstAttemptCorrect: boolean;
  attemptsBeforeFirstPass: number;
  actionCount: number;
  variantId: string;
  hintsUsed: number;
  badges: Badge[];
  language: LangId;
  durationMs: number;
  platform: Platform;
}

export interface SkillMastery {
  skillId: SkillId;
  level: MasteryLevel;
  score: 0 | 50 | 80 | 100;
  /** proficient: firstAttemptCorrect trên variantId CHƯA GẶP + actionCount <= 2*optimal.
   *  mastered:   đúng skill đó trong "Chuyến tuần tra", >= 16h sau khi đạt proficient. */
  proficientAt?: number;
  // FSRS-5
  stability: number; difficulty: number; due: number; lapses: number;
}
// KHÔNG CÓ MŨI TÊN ĐI XUỐNG. Sai chỉ khiến FSRS xếp lịch ôn sớm hơn.

export interface LessonProgress {
  lessonId: LessonId; currentStep: number; completedSteps: StepId[];
  badges: Badge[]; stars: 0|1|2|3;      // suy diễn = số badge, giữ tương thích Dexie cũ
  lastAccessed: number;
}

/* ═══════════════ 13. PACK / INDEX ═══════════════ */

export interface PackMeta {
  id: string; locale: LocaleId;
  proseUrl: string; execUrl: string;
  proseBytes: number; execBytes: number;
  proseSha256: string; execSha256: string;
  signature: string;                    // Ed25519
  lessonCount: number;
  requiresRuntime: ('pyodide'|'byte-rust'|'ts-checker'|'surreal-wasm'|'embed-model')[];
  maxPackKb: number;                    // vượt ⇒ BUILD FAIL, buộc tách module
}

export interface ContentIndex {
  schemaVersion: typeof SCHEMA_VERSION;
  builtAt: string; appBuildId: string;
  locale: LocaleId;
  tracks: TrackMeta[];
  skills: SkillGraph;
  packs: PackMeta[];
}

export interface TrackMeta {
  id: TrackId; order: number; title: string; summary: string;
  coverEmoji: string; coverColor: string;
  languages: LangId[]; modules: ModuleMeta[]; requiresTracks: TrackId[];
  totalPackBytes: number;               // để UI báo "Tải cả track: 48 MB"
}

export interface ModuleMeta {
  id: ModuleId; track: TrackId; order: number;
  title: string; summary: string; coverEmoji: string;
  lessonIds: LessonId[]; packId: string; estimatedMinutes: number;
}

/* ═══════════════ 14. KẾT QUẢ THỰC THI ═══════════════ */

export type ExecStatus =
  | 'ok' | 'parse_error' | 'type_error' | 'move_error'
  | 'runtime_panic' | 'assertion_failed'
  | 'timeout' | 'step_limit' | 'oom' | 'output_limit'
  | 'forbidden_api'
  | 'unsupported_feature'      // ★ ChuaHoTro — KHÔNG BAO GIỜ được coi là pass
  | 'engine_crash';

export interface Diagnostic {
  severity: 'error' | 'warning' | 'help' | 'note';
  code?: string;                        // 'E0382', 'TS2345', 'TypeError'
  messageEn: string;                    // nguyên văn — để học viên Google được
  messageVi: string;                    // từ bảng ánh xạ theo MÃ LỖI, không dịch máy
  primarySpan?: { file: string; startLine: number; startCol: number;
                  endLine: number; endCol: number };
  labels: { span: NonNullable<Diagnostic['primarySpan']>; text: string }[];
  suggestion?: { span: NonNullable<Diagnostic['primarySpan']>;
                 replacement: string; label: string };   // quick-fix cho CodeMirror 6
}

export interface StepResult {
  status: ExecStatus;
  passed: boolean;
  badges: Badge[];
  ruleResults: { id: string; tier: ValidationTier; passed: boolean;
                 severity: Severity; message?: string }[];
  diagnostics: Diagnostic[];
  stdout: string; stderr: string;
  trace?: TraceEvent[];
  metrics: { wallMs: number; steps?: number; domainOps?: number;
             peakHeapBytes?: number; allocCount?: number };
  engineId: string; engineVersion: string;
  revealHintRung?: number;
}

export interface EngineCapabilities {
  compileDiagnostics: boolean;
  typeCheck: boolean;
  moveCheck: 'none' | 'narrow' | 'full';
  stepMetering: boolean;
  deterministic: boolean;
  cancellation: 'interrupt' | 'terminate-only';   // ★ khai báo, không giả định
  coldStartMsEstimate: number;
  residentBytesEstimate: number;
  supportedFeatures: string[];
  unsupportedFeatures: string[];                   // hiện trong "Byte biết gì"
}
