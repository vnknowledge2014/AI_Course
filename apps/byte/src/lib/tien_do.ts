/** Tiến độ người học, lưu trên máy.
 *
 *  Khoá lưu là `id` của bài — chính vì vậy `id` không được mang số thứ tự
 *  (xem `content-compiler/src/cli.ts`): chèn thêm một bài vào giữa mạch mà làm
 *  đổi id thì toàn bộ tiến độ đã lưu trỏ vào hư không.
 */
const KHOA = 'byte.tien-do.v1';

export interface TienDo {
  /** id bài -> số bước đã hoàn thành */
  buoc_xong: Record<string, number>;
  /** id bài đã học hết */
  da_xong: string[];
}

const rong = (): TienDo => ({ buoc_xong: {}, da_xong: [] });

export function doc(): TienDo {
  try {
    const s = localStorage.getItem(KHOA);
    if (!s) return rong();
    const d = JSON.parse(s) as Partial<TienDo>;
    return { buoc_xong: d.buoc_xong ?? {}, da_xong: d.da_xong ?? [] };
  } catch {
    // localStorage có thể bị chặn (chế độ riêng tư trên Safari). Mất tiến độ
    // thì tiếc, nhưng không được làm sập ứng dụng vì chuyện đó.
    return rong();
  }
}

export function ghi(t: TienDo): void {
  try {
    localStorage.setItem(KHOA, JSON.stringify(t));
  } catch {
    /* im lặng: xem chú thích ở `doc` */
  }
}

export function danh_dau_buoc(id: string, so_buoc: number): TienDo {
  const t = doc();
  t.buoc_xong[id] = Math.max(t.buoc_xong[id] ?? 0, so_buoc);
  ghi(t);
  return t;
}

export function danh_dau_xong(id: string): TienDo {
  const t = doc();
  if (!t.da_xong.includes(id)) t.da_xong.push(id);
  ghi(t);
  return t;
}
