/**
 * Phân tích directive dạng container theo thang colon.
 *
 * # Vì sao thang colon quan trọng
 *
 * Cú pháp `:::` của remark-directive có một luật dễ vấp: container NGOÀI phải
 * có **nhiều dấu hai chấm HƠN** container trong. Viết `:::challenge` bọc
 * `:::hints` cùng ba dấu thì `:::hints` **đóng luôn** `:::challenge`, và mọi
 * thứ sau đó rơi ra ngoài — im lặng, không lỗi.
 *
 * Thang chốt cứng của Byte Academy:
 * - Cấp Step: `::::` (4 dấu)
 * - Cấp con (`hints`, `validate`, `opt`, `why`, `solution`): `:::` (3 dấu)
 *
 * Parser này **fail cứng** khi con có số colon ≥ cha, thay vì im lặng nuốt nội
 * dung. Một bài học bị nuốt mất phần chấm điểm là bài học chấm sai.
 */

export interface Directive {
  /** Tên directive, ví dụ `explain`, `code`, `hints`. */
  ten: string;
  /** Số dấu hai chấm mở. */
  bac: number;
  /** Thuộc tính trong `{...}`: `{trigger=enter mood=curious}` hoặc cờ `{commitOnce}`. */
  thuoc_tinh: Record<string, string | true>;
  /** `#id` viết tắt trong `{...}`. */
  id?: string;
  /** Nội dung thô bên trong, chưa parse. */
  than: string;
  /** Directive con trực tiếp. */
  con: Directive[];
  /** Dòng bắt đầu trong tệp nguồn, đếm từ 1 — để chẩn đoán chỉ đúng chỗ. */
  dong: number;
}

export class LoiDirective extends Error {
  constructor(
    message: string,
    readonly dong: number,
    readonly goi_y?: string,
  ) {
    super(message);
    this.name = 'LoiDirective';
  }
}

// Tối thiểu HAI dấu, không phải ba: thang colon có ba cấp — Step `::::`,
// con của Step `:::`, con của `opt` là `::why`. Bản đầu dùng `{3,}` nên `::why`
// không được nhận là directive và mọi giải thích cho đáp án sai biến mất
// LẶNG LẼ — bài học vẫn biên dịch, chỉ là mất đúng phần dạy học.
const RE_MO = /^(:{2,})([a-zA-Z][\w-]*)(?:\{([^}]*)\})?\s*$/;
const RE_DONG = /^(:{2,})\s*$/;

function doc_thuoc_tinh(raw: string | undefined): {
  thuoc_tinh: Record<string, string | true>;
  id?: string;
} {
  const thuoc_tinh: Record<string, string | true> = {};
  let id: string | undefined;
  if (!raw) return { thuoc_tinh, id };

  // Tách theo khoảng trắng nhưng giữ nguyên "chuỗi có dấu nháy".
  const phan = raw.match(/(?:[^\s"]+|"[^"]*")+/g) ?? [];
  for (const p of phan) {
    if (p.startsWith('#')) {
      id = p.slice(1);
      continue;
    }
    const bang = p.indexOf('=');
    if (bang === -1) {
      thuoc_tinh[p] = true;
    } else {
      const k = p.slice(0, bang);
      let v = p.slice(bang + 1);
      if (v.startsWith('"') && v.endsWith('"')) v = v.slice(1, -1);
      thuoc_tinh[k] = v;
    }
  }
  return { thuoc_tinh, id };
}

/**
 * Phân tích thân bài học thành cây directive.
 *
 * Chỉ nhận directive ở **đầu dòng**, và bỏ qua mọi dòng nằm trong code fence —
 * nếu không, một ví dụ minh hoạ cú pháp directive trong bài học sẽ phá cấu trúc.
 */
export function phan_tich_directive(nguon: string, dong_bat_dau = 1): Directive[] {
  const dong = nguon.split('\n');
  const goc: Directive[] = [];
  const ngan_xep: Directive[] = [];
  let trong_fence: string | null = null;

  for (let i = 0; i < dong.length; i++) {
    const l = dong[i] ?? '';
    const so_dong = dong_bat_dau + i;

    // Code fence: mọi thứ bên trong là nội dung thô.
    const fence = /^\s*(`{3,}|~{3,})/.exec(l);
    if (fence) {
      const dau = fence[1]!;
      if (trong_fence === null) trong_fence = dau;
      else if (dau.startsWith(trong_fence[0]!) && dau.length >= trong_fence.length)
        trong_fence = null;
      them_dong(ngan_xep, goc, l);
      continue;
    }
    if (trong_fence !== null) {
      them_dong(ngan_xep, goc, l);
      continue;
    }

    const mo = RE_MO.exec(l);
    if (mo) {
      const bac = mo[1]!.length;
      const cha = ngan_xep[ngan_xep.length - 1];
      if (cha && bac >= cha.bac) {
        throw new LoiDirective(
          `\`${':'.repeat(bac)}${mo[2]}\` nằm trong \`${':'.repeat(cha.bac)}${cha.ten}\` ` +
            `nhưng có số dấu hai chấm không ít hơn cha (${bac} ≥ ${cha.bac})`,
          so_dong,
          `Container ngoài phải NHIỀU dấu hai chấm hơn container trong. ` +
            `Cấp Step dùng \`::::\`, cấp con dùng \`:::\`. ` +
            `Viết sai thang này thì directive con sẽ ĐÓNG LUÔN directive cha — ` +
            `im lặng, và phần chấm điểm bị nuốt mất.`,
        );
      }
      const { thuoc_tinh, id } = doc_thuoc_tinh(mo[3]);
      const d: Directive = {
        ten: mo[2]!,
        bac,
        thuoc_tinh,
        ...(id !== undefined ? { id } : {}),
        than: '',
        con: [],
        dong: so_dong,
      };
      if (cha) cha.con.push(d);
      else goc.push(d);
      ngan_xep.push(d);
      continue;
    }

    const dong_dong = RE_DONG.exec(l);
    if (dong_dong) {
      const bac = dong_dong[1]!.length;
      const dang = ngan_xep[ngan_xep.length - 1];
      if (!dang) {
        throw new LoiDirective(
          `gặp \`${':'.repeat(bac)}\` đóng nhưng không có directive nào đang mở`,
          so_dong,
        );
      }
      if (bac !== dang.bac) {
        throw new LoiDirective(
          `\`${':'.repeat(bac)}\` không khớp với \`${':'.repeat(dang.bac)}${dang.ten}\` mở ở dòng ${dang.dong}`,
          so_dong,
        );
      }
      ngan_xep.pop();
      continue;
    }

    them_dong(ngan_xep, goc, l);
  }

  if (ngan_xep.length > 0) {
    const d = ngan_xep[ngan_xep.length - 1]!;
    throw new LoiDirective(
      `\`${':'.repeat(d.bac)}${d.ten}\` mở ở dòng ${d.dong} mà chưa đóng`,
      d.dong,
      `Thêm một dòng \`${':'.repeat(d.bac)}\` ở cuối phần này.`,
    );
  }
  return goc;
}

function them_dong(ngan_xep: Directive[], goc: Directive[], l: string): void {
  const dang = ngan_xep[ngan_xep.length - 1];
  if (dang) {
    dang.than += (dang.than === '' ? '' : '\n') + l;
  } else if (l.trim() !== '') {
    // Văn bản ngoài mọi directive: giữ lại như một khối `explain` ngầm.
    const cuoi = goc[goc.length - 1];
    if (cuoi && cuoi.ten === '_van_ban') {
      cuoi.than += '\n' + l;
    } else {
      goc.push({ ten: '_van_ban', bac: 0, thuoc_tinh: {}, than: l, con: [], dong: 0 });
    }
  }
}
