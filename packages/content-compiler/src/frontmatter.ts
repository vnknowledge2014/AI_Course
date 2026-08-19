/**
 * Đọc frontmatter YAML của `.lesson.md`.
 *
 * Tự viết thay vì kéo một thư viện YAML đầy đủ: frontmatter của bài học có
 * **hình dạng đóng** (đã chốt ở schema v2), nên một parser hẹp vừa đủ dùng lại
 * cho thông báo lỗi tốt hơn nhiều — nó biết trường nào là hợp lệ nên gợi ý được
 * "có phải bạn định viết `estimatedMinutes`?" khi tác giả gõ nhầm.
 */

export type GiaTriYaml = string | number | boolean | GiaTriYaml[] | { [k: string]: GiaTriYaml };

export class LoiFrontmatter extends Error {
  constructor(message: string, readonly dong: number, readonly goi_y?: string) {
    super(message);
    this.name = 'LoiFrontmatter';
  }
}

export interface KetQuaTach {
  frontmatter: Record<string, GiaTriYaml>;
  than: string;
  /** Dòng (đếm từ 1) mà thân bài bắt đầu — để chẩn đoán chỉ đúng chỗ. */
  dong_than: number;
}

export function tach(nguon: string): KetQuaTach {
  const dong = nguon.split('\n');
  if (dong[0]?.trim() !== '---') {
    throw new LoiFrontmatter(
      'tệp bài học phải mở đầu bằng một dòng `---`',
      1,
      'Frontmatter chứa id, title, track… Chạy `content:new` để sinh khung đúng.',
    );
  }
  const ket = dong.findIndex((l, i) => i > 0 && l.trim() === '---');
  if (ket === -1) {
    throw new LoiFrontmatter('frontmatter mở bằng `---` mà không có `---` đóng', 1);
  }
  return {
    frontmatter: doc_yaml(dong.slice(1, ket), 2),
    than: dong.slice(ket + 1).join('\n'),
    dong_than: ket + 2,
  };
}

/** Parser YAML hẹp: khoá-giá trị, danh sách nội tuyến, danh sách gạch đầu dòng, lồng theo thụt lề. */
function doc_yaml(dong: string[], dong_dau: number): Record<string, GiaTriYaml> {
  const goc: Record<string, GiaTriYaml> = {};
  // Ngăn xếp (thụt lề, đối tượng đang ghi vào)
  const ngan_xep: { thut: number; obj: Record<string, GiaTriYaml> }[] = [
    { thut: -1, obj: goc },
  ];

  for (let i = 0; i < dong.length; i++) {
    const raw = dong[i] ?? '';
    if (raw.trim() === '' || raw.trim().startsWith('#')) continue;
    const thut = raw.length - raw.trimStart().length;
    const l = raw.trim();
    const so_dong = dong_dau + i;

    while (ngan_xep.length > 1 && thut <= ngan_xep[ngan_xep.length - 1]!.thut) {
      ngan_xep.pop();
    }
    const hien = ngan_xep[ngan_xep.length - 1]!.obj;

    // Mục danh sách gạch đầu dòng
    if (l.startsWith('- ')) {
      // Chỉ hỗ trợ danh sách các bản ghi phẳng (`- kind: … / body: …`).
      // Danh sách chuỗi dùng dạng nội tuyến `[a, b]`.
      throw new LoiFrontmatter(
        'danh sách gạch đầu dòng chưa hỗ trợ trong frontmatter',
        so_dong,
        'Dùng dạng nội tuyến: `languages: [python, rust]`.',
      );
    }

    const hai_cham = l.indexOf(':');
    if (hai_cham === -1) {
      throw new LoiFrontmatter(`dòng không phải dạng \`khoá: giá trị\``, so_dong);
    }
    const khoa = l.slice(0, hai_cham).trim();
    const phan_gia_tri = l.slice(hai_cham + 1).trim();

    if (phan_gia_tri === '') {
      // Bắt đầu một khối lồng.
      const con: Record<string, GiaTriYaml> = {};
      hien[khoa] = con;
      ngan_xep.push({ thut, obj: con });
      continue;
    }
    hien[khoa] = doc_gia_tri(phan_gia_tri);
  }
  return goc;
}

function doc_gia_tri(s: string): GiaTriYaml {
  if (s.startsWith('[') && s.endsWith(']')) {
    const trong = s.slice(1, -1).trim();
    if (trong === '') return [];
    return trong.split(',').map((x) => doc_gia_tri(x.trim()));
  }
  if ((s.startsWith('"') && s.endsWith('"')) || (s.startsWith("'") && s.endsWith("'"))) {
    return s.slice(1, -1);
  }
  if (s === 'true') return true;
  if (s === 'false') return false;
  if (/^-?\d+$/.test(s)) return Number.parseInt(s, 10);
  if (/^-?\d*\.\d+$/.test(s)) return Number.parseFloat(s);
  return s;
}
