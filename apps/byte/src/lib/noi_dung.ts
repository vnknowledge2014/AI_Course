/** Nạp bài học đã biên dịch từ `public/noi-dung/`.
 *
 *  Nội dung là file tĩnh, không phải API. Đó là lựa chọn có chủ đích: ứng dụng
 *  phải chạy được khi máy không có mạng — người học ở quán cà phê mất wifi
 *  không được mất bài đang học giữa chừng.
 */
import type { Lesson } from '@byte/content-schema';

export interface MucLuc {
  id: string;
  track: string;
  module: string;
  order: number;
  title: string;
  summary: string;
  estimatedMinutes: number;
  tier?: string;
  teaches?: string[];
  requires?: string[];
}

/** Thứ tự realm/track, do trình biên dịch chép từ `content/curriculum/thu-tu.yaml`. */
export interface Realm {
  id: string;
  ten: string;
  track: string[];
}

const GOC = `${import.meta.env.BASE_URL}noi-dung`;

let cache_muc_luc: MucLuc[] | null = null;
let cache_thu_tu: Realm[] = [];
const cache_bai = new Map<string, Lesson>();

export async function muc_luc(): Promise<MucLuc[]> {
  if (cache_muc_luc) return cache_muc_luc;
  const r = await fetch(`${GOC}/index.json`);
  if (!r.ok) throw new Error(`không nạp được mục lục (HTTP ${r.status})`);
  const d = (await r.json()) as { lessons: MucLuc[]; thuTu?: Realm[] };
  cache_thu_tu = d.thuTu ?? [];
  // Sắp theo module rồi theo `order` — thứ tự này là MẠCH học, không phải thứ
  // tự chữ cái. Đảo nó lên là phá vỡ chuỗi câu hỏi bỏ ngỏ nối giữa các bài.
  cache_muc_luc = [...d.lessons].sort(
    (a, b) => a.module.localeCompare(b.module) || a.order - b.order,
  );
  return cache_muc_luc;
}

export async function bai_hoc(id: string): Promise<Lesson> {
  const co = cache_bai.get(id);
  if (co) return co;
  const r = await fetch(`${GOC}/${id}.json`);
  if (!r.ok) throw new Error(`không nạp được bài \`${id}\` (HTTP ${r.status})`);
  const l = (await r.json()) as Lesson;
  cache_bai.set(id, l);
  return l;
}

/** Thứ tự realm. Gọi sau `muc_luc()` — nó được nạp cùng một lượt. */
export function thu_tu(): Realm[] {
  return cache_thu_tu;
}

/* ── Thư viện TIER-C ────────────────────────────────────────────────────── */

/** Một chương sách ở chế độ đọc. Không có bước, không chấm, không cấp mastery.
 *
 *  Tier C tồn tại để "không lĩnh vực nào trống": người vào tìm hiểu DDD hay
 *  parser combinator phải đọc được cái gì đó ngay, kể cả khi bài tương tác cho
 *  lĩnh vực ấy chưa được viết. */
export interface Chuong {
  id: string;
  lang: string;
  part: string;
  chapter: string;
  title: string;
  summary: string;
  lines: number;
  estimatedMinutes?: number;
  sourcePath: string;
}

const GOC_TV = `${import.meta.env.BASE_URL}thu-vien`;

let cache_tv: Chuong[] | null = null;
const cache_chuong = new Map<string, Chuong & { body: unknown }>();

export async function thu_vien(): Promise<Chuong[]> {
  if (cache_tv) return cache_tv;
  const r = await fetch(`${GOC_TV}/index.json`);
  if (!r.ok) throw new Error(`không nạp được thư viện (HTTP ${r.status})`);
  const d = (await r.json()) as { chapters: Chuong[] };
  cache_tv = d.chapters;
  return cache_tv;
}

export async function chuong(id: string): Promise<Chuong & { body: unknown }> {
  const co = cache_chuong.get(id);
  if (co) return co;
  const r = await fetch(`${GOC_TV}/${id}.json`);
  if (!r.ok) throw new Error(`không nạp được chương \`${id}\` (HTTP ${r.status})`);
  const c = (await r.json()) as Chuong & { body: unknown };
  cache_chuong.set(id, c);
  return c;
}
