/**
 * Lắp ráp `.lesson.md` thành đối tượng `Lesson` của schema v2.
 *
 * Nguyên tắc: **mọi thứ mơ hồ đều là lỗi build.** Một bài học thiếu đáp án,
 * thiếu gợi ý, hay có `estimatedMinutes` ngoài khoảng 7–15 sẽ chặn build chứ
 * không lặng lẽ đi tiếp. Nội dung sai chỉ lộ ra khi có người học đọc phải, và
 * lúc đó thì đã muộn.
 */

import type {
  ContentTier,
  LangId,
  Lesson,
  Level,
  LocaleId,
  RichNode,
  Step,
  TrackId,
} from '@byte/content-schema';
import { SCHEMA_VERSION } from '@byte/content-schema';
import { phan_tich_directive, type Directive } from './directive.js';
import { tach, type GiaTriYaml } from './frontmatter.js';
import { doc_richtext } from './richtext.js';

export class LoiBienDich extends Error {
  constructor(
    message: string,
    readonly tep: string,
    readonly dong: number,
    readonly goi_y?: string,
  ) {
    super(message);
    this.name = 'LoiBienDich';
  }
}

/** Ràng buộc "chậm mà chắc" — cưỡng chế ở đây chứ không chỉ ghi trong tài liệu. */
export const PHUT_TOI_THIEU = 7;
export const PHUT_TOI_DA = 15;

interface KhoiMa {
  lang: string;
  title: string;
  src: string;
}

/** Tách các code fence có `title=` khỏi phần văn xuôi. */
function tach_khoi_ma(than: string): { van_xuoi: string; ma: KhoiMa[] } {
  const dong = than.split('\n');
  const con_lai: string[] = [];
  const ma: KhoiMa[] = [];
  let i = 0;
  while (i < dong.length) {
    const m = /^\s*```([a-zA-Z0-9_+-]*)\s+title=(\S+)\s*$/.exec(dong[i] ?? '');
    if (m) {
      const src: string[] = [];
      i++;
      while (i < dong.length && !/^\s*```\s*$/.test(dong[i] ?? '')) {
        src.push(dong[i] ?? '');
        i++;
      }
      i++;
      ma.push({ lang: m[1]!.toLowerCase(), title: m[2]!, src: src.join('\n') });
      continue;
    }
    con_lai.push(dong[i] ?? '');
    i++;
  }
  return { van_xuoi: con_lai.join('\n'), ma };
}

function than_richtext(d: Directive): RichNode[] {
  return doc_richtext(tach_khoi_ma(d.than).van_xuoi);
}

function bat_buoc<T>(v: T | undefined, thong_diep: string, tep: string, dong: number, goi_y?: string): T {
  if (v === undefined || v === null || (Array.isArray(v) && v.length === 0)) {
    throw new LoiBienDich(thong_diep, tep, dong, goi_y);
  }
  return v;
}

/** Biên dịch một tệp `.lesson.md`. */
/** Dấu hiệu khung bài học do `content new` sinh ra và chưa ai điền vào.
 *
 *  Dùng đúng chuỗi `TODO —` (kèm gạch ngang dài) mà `cli.ts` phát ra, không
 *  phải chữ `TODO` trơn — để một bài học nói VỀ việc ghi chú TODO vẫn viết
 *  được bình thường. */
const KHUNG_CHUA_DIEN = /TODO\s+—/;

export function bien_dich(tep: string, nguon: string): Lesson {
  const { frontmatter: fm, than, dong_than } = tach(nguon);

  // Một khung rỗng biên dịch được là chế độ hỏng tệ nhất của cả đường ống:
  // nó khiến "40 bài đã xong" và "40 khung chưa ai viết" trông giống hệt nhau
  // trong CI. Chặn ở đây, trước mọi kiểm tra khác.
  const dong_todo = nguon.split('\n').findIndex((d) => KHUNG_CHUA_DIEN.test(d));
  if (dong_todo >= 0) {
    throw new LoiBienDich(
      'bài học còn chỗ `TODO —` chưa điền',
      tep,
      dong_todo + 1,
      'Đây là khung do `content new` sinh ra. Hoặc viết xong nội dung, hoặc xoá ' +
        'file đi — một khung rỗng lọt qua cổng kiểm sẽ bị đếm nhầm là bài đã viết.',
    );
  }
  const lay = <T extends GiaTriYaml>(k: string): T | undefined => fm[k] as T | undefined;

  const id = bat_buoc(lay<string>('id'), 'frontmatter thiếu `id`', tep, 1);
  const title = bat_buoc(lay<string>('title'), 'frontmatter thiếu `title`', tep, 1);
  const phut = bat_buoc(lay<number>('estimatedMinutes'), 'frontmatter thiếu `estimatedMinutes`', tep, 1);

  if (phut < PHUT_TOI_THIEU || phut > PHUT_TOI_DA) {
    throw new LoiBienDich(
      `\`estimatedMinutes\` là ${phut}, phải nằm trong khoảng ${PHUT_TOI_THIEU}–${PHUT_TOI_DA}`,
      tep,
      1,
      'Nguyên tắc "chậm mà chắc": bài dài hơn 15 phút thì tách đôi, ngắn hơn 7 phút ' +
        'thì gộp với bài kế. Người học bỏ dở giữa chừng là mất cả mạch.',
    );
  }

  const goc = phan_tich_directive(than, dong_than);
  const steps: Step[] = [];

  for (const d of goc) {
    const b = lam_step(d, tep);
    if (b) steps.push(b);
  }

  if (steps.length === 0) {
    throw new LoiBienDich('bài học không có bước nào', tep, dong_than);
  }

  const languages = (lay<string[]>('languages') ?? ['python']) as LangId[];
  return {
    schemaVersion: SCHEMA_VERSION,
    id,
    locale: (lay<string>('locale') ?? 'vi') as LocaleId,
    track: bat_buoc(lay<string>('track'), 'frontmatter thiếu `track`', tep, 1) as TrackId,
    module: bat_buoc(lay<string>('module'), 'frontmatter thiếu `module`', tep, 1),
    order: lay<number>('order') ?? 0,
    title,
    summary: lay<string>('summary') ?? title,
    level: (lay<string>('level') ?? 'beginner') as Level,
    tier: (lay<string>('tier') ?? 'A') as ContentTier,
    languages,
    defaultLanguage: (lay<string>('defaultLanguage') ?? languages[0]) as LangId,
    estimatedMinutes: phut,
    steps,
    teaches: lay<string[]>('teaches') ?? [],
    practices: lay<string[]>('practices') ?? [],
    requires: lay<string[]>('requires') ?? [],
    concepts: lay<string[]>('concepts') ?? [],
    assets: [],
    gradingMatrix: (lay('gradingMatrix') ?? {}) as Lesson['gradingMatrix'],
    // Xuất xứ là bắt buộc: nó cho biết bài này viết tay hay di trú từ sách cũ,
    // và đã có người rà lại chưa. Bài `authoredBy: 'llm-assisted'` mà
    // `reviewed: false` KHÔNG được cấp mastery mức Proficient (schema §10).
    provenance: {
      authoredBy: (fm['provenance'] as Record<string, GiaTriYaml> | undefined)?.['authoredBy'] ?? 'human',
      reviewed: (fm['provenance'] as Record<string, GiaTriYaml> | undefined)?.['reviewed'] ?? false,
      ...((fm['provenance'] as Record<string, GiaTriYaml> | undefined) ?? {}),
    },
  } as unknown as Lesson;
}

function lam_step(d: Directive, tep: string): Step | null {
  const id = d.id ?? `${d.ten}-${d.dong}`;
  const chung = { id, kind: d.ten } as Record<string, unknown>;

  switch (d.ten) {
    case '_van_ban':
      return { ...chung, id: `explain-${d.dong}`, kind: 'explain', body: doc_richtext(d.than) } as Step;

    case 'byte':
      // Byte beat không phải một Step; nó gắn vào step kế tiếp ở tầng biên dịch
      // sau. Ở đây bỏ qua để giữ cấu trúc đơn giản.
      return null;

    case 'explain':
      return { ...chung, body: than_richtext(d) } as Step;

    case 'example': {
      const { ma } = tach_khoi_ma(d.than);
      const chinh = ma[0];
      return {
        ...chung,
        body: than_richtext(d),
        code: chinh ? { lang: chinh.lang, starter: chinh.src } : undefined,
        runnable: d.thuoc_tinh['runnable'] !== undefined,
      } as unknown as Step;
    }

    case 'predict': {
      const opts = d.con.filter((c) => c.ten === 'opt');
      if (opts.length < 2) {
        throw new LoiBienDich(
          'bước `predict` cần ít nhất hai lựa chọn',
          tep,
          d.dong,
          'Mỗi lựa chọn là một `:::opt`; đánh dấu đáp án đúng bằng `:::opt{correct}`.',
        );
      }
      const dung = opts.filter((o) => o.thuoc_tinh['correct'] !== undefined);
      if (dung.length !== 1) {
        throw new LoiBienDich(
          `bước \`predict\` phải có ĐÚNG một \`:::opt{correct}\`, đang có ${dung.length}`,
          tep,
          d.dong,
        );
      }
      return {
        ...chung,
        body: than_richtext(d),
        commitOnce: true,
        answer: {
          kind: 'choice',
          options: opts.map((o) => ({
            id: `opt-${o.dong}`,
            label: doc_richtext(tach_khoi_ma(o.than).van_xuoi),
            correct: o.thuoc_tinh['correct'] !== undefined,
            why: o.con.find((c) => c.ten === 'why')
              ? doc_richtext(o.con.find((c) => c.ten === 'why')!.than)
              : undefined,
          })),
        },
      } as unknown as Step;
    }

    case 'code':
    case 'repair':
    case 'refactor':
    case 'assemble': {
      const { ma } = tach_khoi_ma(d.than);
      const starter = ma.find((m) => m.title === 'starter');
      const solution = ma.find((m) => m.title === 'solution');
      const test = ma.find((m) => m.title === 'test');
      if (!starter) {
        throw new LoiBienDich(
          `bước \`${d.ten}\` thiếu khối \`\`\`<lang> title=starter`,
          tep,
          d.dong,
          'Người học cần một điểm bắt đầu, không phải một trang trắng.',
        );
      }
      if (!solution) {
        throw new LoiBienDich(
          `bước \`${d.ten}\` thiếu khối \`title=solution\``,
          tep,
          d.dong,
          'Lời giải tham chiếu là bắt buộc: CI chạy nó qua đúng runtime sẽ ship ' +
            'để bảo đảm bài này thật sự giải được.',
        );
      }
      if (!test) {
        throw new LoiBienDich(
          `bước \`${d.ten}\` thiếu khối \`title=test\``,
          tep,
          d.dong,
          'Không có test thì không chấm được — và một bài không chấm được mà vẫn ' +
            'cấp huy hiệu chính là lỗi mà cả dự án này tồn tại để tránh.',
        );
      }
      const hints = d.con.find((c) => c.ten === 'hints');
      if (!hints) {
        throw new LoiBienDich(
          `bước \`${d.ten}\` thiếu \`:::hints\``,
          tep,
          d.dong,
          'Thang gợi ý là thứ giữ người học khỏi bỏ cuộc. Tối thiểu ba bậc: ' +
            'hướng chú ý → chiến lược → một dòng code.',
        );
      }
      return {
        ...chung,
        body: than_richtext(d),
        code: {
          lang: starter.lang,
          starter: starter.src,
          solution: solution.src,
          test: test.src,
        },
        hints: doc_thang_goi_y(hints),
        validation: doc_validate(d.con.find((c) => c.ten === 'validate')),
      } as unknown as Step;
    }

    case 'checkpoint':
      return {
        ...chung,
        mastery: Number.parseFloat(String(d.thuoc_tinh['mastery'] ?? '0.8')),
      } as unknown as Step;

    case 'reflect':
      return { ...chung, body: than_richtext(d) } as unknown as Step;

    default:
      throw new LoiBienDich(
        `chưa hỗ trợ bước \`${d.ten}\``,
        tep,
        d.dong,
        'Các bước dùng được: explain, example, predict, code, repair, refactor, ' +
          'assemble, checkpoint, reflect.',
      );
  }
}

function doc_thang_goi_y(d: Directive): unknown {
  // `:::hints` chứa danh sách YAML dạng `- kind: … / body: …`
  const rungs: { kind: string; body: RichNode[] }[] = [];
  let kind: string | null = null;
  let body: string[] = [];
  const xa = () => {
    if (kind !== null) {
      rungs.push({ kind, body: doc_richtext(body.join('\n').trim()) });
      kind = null;
      body = [];
    }
  };
  for (const l of d.than.split('\n')) {
    const m = /^\s*-\s*kind:\s*(\S+)\s*$/.exec(l);
    if (m) {
      xa();
      kind = m[1]!;
      continue;
    }
    const b = /^\s*body:\s*(.*)$/.exec(l);
    if (b) {
      body.push(b[1]!);
      continue;
    }
    if (l.trim() !== '') body.push(l.trim());
  }
  xa();
  return { rungs };
}

function doc_validate(d: Directive | undefined): unknown {
  if (!d) return { rules: [], stopOnFirstBlocking: true, advisoryAffectsBadge: 'none' };
  const rules: Record<string, unknown>[] = [];
  let cur: Record<string, unknown> | null = null;
  for (const l of d.than.split('\n')) {
    const m = /^\s*-\s*tier:\s*(\S+)\s*$/.exec(l);
    if (m) {
      if (cur) rules.push(cur);
      cur = { tier: m[1] };
      continue;
    }
    const kv = /^\s*([a-zA-Z]\w*):\s*(.*)$/.exec(l);
    if (kv && cur) cur[kv[1]!] = kv[2];
  }
  if (cur) rules.push(cur);
  return { rules, stopOnFirstBlocking: true, advisoryAffectsBadge: 'none' };
}
