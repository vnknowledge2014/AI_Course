/** Cây kỹ năng và luật mở khoá.
 *
 *  Mở khoá theo ĐỒ THỊ TIỀN ĐỀ, không theo thứ tự trong danh sách. Một bài mở
 *  khi mọi skill nó `requires` đã được một bài ĐÃ XONG `teaches`.
 *
 *  Khác biệt này quan trọng: nó cho phép người học rẽ nhánh. Realm 2 (Toán)
 *  không phụ thuộc Realm 1, nên ai muốn học toán trước thì học được ngay — mà
 *  vẫn không vào được bài `list` khi chưa qua bài `vòng lặp`. Xếp thứ tự cứng
 *  thì hoặc chặn nhầm, hoặc cho qua nhầm.
 */
import type { MucLuc } from './noi_dung';
import type { TienDo } from './tien_do';

export interface Realm {
  id: string;
  ten: string;
  track: string[];
}

export interface BaiTrenBanDo extends MucLuc {
  /** Mọi tiền đề đã được dạy ở một bài đã xong. */
  mo: boolean;
  xong: boolean;
  /** Skill còn thiếu, kèm tên bài dạy nó — để nói cho người học biết đi đâu. */
  con_thieu: { skill: string; hoc_o: string | null }[];
}

export interface TrackTrenBanDo {
  id: string;
  bai: BaiTrenBanDo[];
  xong: number;
  tong: number;
}

export interface RealmTrenBanDo {
  id: string;
  ten: string;
  track: TrackTrenBanDo[];
  xong: number;
  tong: number;
  /** Realm có ít nhất một bài mở. */
  mo: boolean;
}

export function dung_ban_do(
  ds: MucLuc[],
  thu_tu: Realm[],
  tien_do: TienDo,
): RealmTrenBanDo[] {
  const da_xong = new Set(tien_do.da_xong);

  // Skill đã thạo = skill được dạy bởi một bài đã xong.
  const thao = new Set<string>();
  for (const l of ds) {
    if (da_xong.has(l.id)) for (const s of l.teaches ?? []) thao.add(s);
  }
  // Skill -> bài dạy nó, để chỉ đường khi còn thiếu.
  const day_o = new Map<string, string>();
  for (const l of ds) {
    for (const s of l.teaches ?? []) if (!day_o.has(s)) day_o.set(s, l.title);
  }

  const theo_khoa = new Map<string, MucLuc[]>();
  for (const l of ds) {
    const k = `${l.track}/${l.module}`;
    (theo_khoa.get(k) ?? theo_khoa.set(k, []).get(k)!).push(l);
  }

  return thu_tu.map((r) => {
    const track = r.track.map((t) => {
      const bai = (theo_khoa.get(`${r.id}/${t}`) ?? [])
        .slice()
        .sort((a, b) => a.order - b.order)
        .map((l): BaiTrenBanDo => {
          const con_thieu = (l.requires ?? [])
            .filter((s) => !thao.has(s))
            .map((s) => ({ skill: s, hoc_o: day_o.get(s) ?? null }));
          return { ...l, xong: da_xong.has(l.id), mo: con_thieu.length === 0, con_thieu };
        });
      return {
        id: t,
        bai,
        xong: bai.filter((b) => b.xong).length,
        tong: bai.length,
      };
    });
    const xong = track.reduce((s, t) => s + t.xong, 0);
    const tong = track.reduce((s, t) => s + t.tong, 0);
    return {
      id: r.id,
      ten: r.ten,
      track,
      xong,
      tong,
      mo: track.some((t) => t.bai.some((b) => b.mo && !b.xong)) || xong > 0,
    };
  });
}

/** Bài tiếp theo nên học: bài mở, chưa xong, sớm nhất theo thứ tự. */
export function bai_ke_tiep(ban_do: RealmTrenBanDo[]): BaiTrenBanDo | null {
  for (const r of ban_do) {
    for (const t of r.track) {
      for (const b of t.bai) {
        if (b.mo && !b.xong) return b;
      }
    }
  }
  return null;
}

/** Số skill đã thạo — con số "điểm kinh nghiệm" thật, không phải điểm bịa.
 *
 *  Cố tình đếm skill chứ không đếm bài: người học học vì biết làm được thêm
 *  một việc, không phải vì lấp đầy một thanh. */
export function so_ky_nang(ds: MucLuc[], tien_do: TienDo): number {
  const da = new Set(tien_do.da_xong);
  const s = new Set<string>();
  for (const l of ds) if (da.has(l.id)) for (const k of l.teaches ?? []) s.add(k);
  return s.size;
}
