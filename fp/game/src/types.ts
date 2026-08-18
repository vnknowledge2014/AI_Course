// ===== CONTENT BLOCKS =====
// Mỗi "page" trong lesson gồm nhiều blocks xen kẽ nhau

export interface ProseBlock {
  type: 'prose';
  content: string; // Markdown text, render thành HTML
}

export interface CodeReadonlyBlock {
  type: 'code-readonly';
  code: string;
  language: 'python' | 'typescript' | 'rust';
}

export interface CodeEditableBlock {
  type: 'code-editable';
  starterCode: string;
  language: 'python' | 'typescript' | 'rust';
}

export interface CalloutBlock {
  type: 'callout';
  variant: 'tip' | 'warning' | 'important' | 'note';
  content: string;
}

export type ContentBlock = ProseBlock | CodeReadonlyBlock | CodeEditableBlock | CalloutBlock;

// ===== VALIDATION =====
export interface PageValidation {
  testCode: string;          // Code chạy sau user code để validate
  expectedOutput?: string;   // Fallback: match stdout
  forbiddenPatterns?: string[];  // Regex patterns KHÔNG được có trong user code
  requiredPatterns?: string[];   // Regex patterns BẮT BUỘC có trong user code
}

// ===== LIVE VIEW =====
export type LiveScene = 'factory' | 'pipeline';
export type SuccessEffect = 'cube-moves' | 'pipeline-flows';
export type FailureEffect = 'explosion' | 'error-shake';

export interface LiveViewConfig {
  scene: LiveScene;
  onSuccess: SuccessEffect;
  onFailure: FailureEffect;
}

// ===== PAGE =====
// Tương đương 1 "page" trong Swift Playgrounds
export interface Page {
  id: string;                    // e.g. "py-ch11-p3"
  title: string;                 // e.g. "Lượt Của Bạn"
  blocks: ContentBlock[];
  suggestions: string[];         // Code snippets cho suggestion bar
  validation?: PageValidation;   // Chỉ có nếu page có code-editable block
  hints: string[];               // Progressive hints (index 0 = ít spoiler nhất)
  liveView: LiveViewConfig;
}

// ===== BOOK =====
// Tương đương 1 "book" trong Swift Playgrounds (= 1 chapter)
export interface Book {
  id: string;                    // e.g. "py-ch11"
  language: 'python' | 'typescript' | 'rust';
  world: string;                 // e.g. "World 2: Thinking Functionally"
  worldIndex: number;            // 0-6
  chapter: string;               // e.g. "Chapter 11"
  title: string;                 // e.g. "Immutability & Purity"
  description: string;           // 1-2 câu mô tả
  coverColor: string;            // CSS color cho book cover
  pages: Page[];
  prerequisites: string[];       // Book IDs cần hoàn thành trước
  totalPages: number;
}

// ===== MANIFEST =====
export interface BookMeta {
  id: string;
  language: 'python' | 'typescript' | 'rust';
  world: string;
  worldIndex: number;
  chapter?: string;
  title: string;
  description: string;
  coverColor: string;
  coverEmoji?: string;
  totalPages: number;
  prerequisites: string[];
}

declare global {
  interface Window {
    loadPyodide: any;
  }
}

export interface Manifest {
  version: string;
  generated: string;
  books: BookMeta[];
}

// ===== PROGRESS (IndexedDB) =====
export interface BookProgress {
  bookId: string;
  completedPages: string[];      // Page IDs
  currentPage: number;           // Page index
  stars: number;                 // 0-3 per book (max stars across pages)
  lastAccessed: Date;
}

export interface PageCode {
  pageId: string;
  code: string;                  // Last code user typed
  savedAt: Date;
}

// ===== RUNNER RESULTS =====
export interface RunResult {
  success: boolean;
  output: string;                // stdout
  error: string;                 // stderr
  executionTimeMs: number;
}

export interface ValidationResult {
  passed: boolean;
  stars: 1 | 2 | 3;             // 3=no hints, 2=1 hint, 1=2+ hints
  feedback: string;              // Feedback message
  output: string;                // Full output to show user
}
