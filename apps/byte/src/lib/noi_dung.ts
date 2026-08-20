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
}

const GOC = `${import.meta.env.BASE_URL}noi-dung`;

let cache_muc_luc: MucLuc[] | null = null;
const cache_bai = new Map<string, Lesson>();

export async function muc_luc(): Promise<MucLuc[]> {
  if (cache_muc_luc) return cache_muc_luc;
  const r = await fetch(`${GOC}/index.json`);
  if (!r.ok) throw new Error(`không nạp được mục lục (HTTP ${r.status})`);
  const d = (await r.json()) as { lessons: MucLuc[] };
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
