/**
 * Test khoá hợp đồng schema.
 *
 * Schema v2 được **đóng băng** trước khi viết lesson đầu tiên. Lý do: mỗi thay
 * đổi sau đó là một lần sửa lại mọi bài đã viết. File này biến "đóng băng" từ
 * một lời hứa thành một thứ CI kiểm được.
 *
 * Test chạy bằng `tsc` chứ không bằng runtime: phần lớn hợp đồng là kiểu, và
 * kiểu thì runtime không thấy. Một Lesson thiếu trường bắt buộc phải làm
 * BIÊN DỊCH ĐỎ, không phải đợi tới lúc chạy.
 */

import {
  SCHEMA_VERSION,
  type ByteMood,
  type ContentTier,
  type Lesson,
  type Step,
} from '../src/index.js';

/* ── 1. Phiên bản schema phải cố định ───────────────────────────────────── */

const _phienBan: '2.0.0' = SCHEMA_VERSION;

/* ── 2. Byte KHÔNG được có tâm trạng tiêu cực ───────────────────────────── */
//
// Đây là quyết định sư phạm, không phải chi tiết cài đặt: "sai là dữ liệu,
// không phải thất bại". Nếu ai đó thêm 'sad' vào ByteMood, dòng dưới phải đỏ.

type MoodCam = 'sad' | 'disappointed' | 'angry' | 'annoyed';
type KhongCoMoodTieuCuc = Extract<ByteMood, MoodCam> extends never ? true : false;
const _byteKhongBuon: KhongCoMoodTieuCuc = true;

/* ── 3. Ba tầng nội dung, không hơn ─────────────────────────────────────── */

const _tang: ContentTier[] = ['A', 'B', 'C'];
// @ts-expect-error — 'D' không phải tầng hợp lệ
const _tangSai: ContentTier = 'D';

/* ── 4. Mọi Step phải có `kind` phân biệt được ──────────────────────────── */

function _phanBietStep(s: Step): string {
  switch (s.kind) {
    case 'explain':
    case 'example':
    case 'assemble':
    case 'code':
    case 'predict':
    case 'manipulate':
    case 'tune':
    case 'trace':
    case 'repair':
    case 'refactor':
    case 'classify':
    case 'checkpoint':
    case 'sandbox':
    case 'reflect':
      return s.kind;
    default: {
      // Thêm loại step mới mà quên xử lý ở đây ⇒ biên dịch đỏ.
      const _vetCan: never = s;
      return _vetCan;
    }
  }
}

/* ── 5. Lesson thiếu trường bắt buộc phải đỏ ────────────────────────────── */

// @ts-expect-error — thiếu `steps`, `teaches`, `tier`, … nên phải đỏ
const _lessonThieu: Lesson = {
  schemaVersion: SCHEMA_VERSION,
  id: 'onboarding.01.chao-byte',
};

/* ── 6. `estimatedMinutes` là số — ràng buộc 7..15 do lint cưỡng chế ────── */
//
// TypeScript không diễn đạt được khoảng số, nên giới hạn "chậm mà chắc" nằm ở
// `content-lint`. Ghi lại ở đây để người đọc biết đi tìm ở đâu.

export const RANG_BUOC_LINT = {
  estimatedMinutes: { min: 7, max: 15 },
  byteLine: { maxKyTu: 90 },
  byteBeat: { maxMoiTrang: 1 },
} as const;

export {};
