#!/usr/bin/env node
/**
 * CLI của content-compiler.
 *
 *   content build [thư-mục]     biên dịch mọi .lesson.md, ghi ra dist/content
 *   content new <track>/<module>/<slug>   sinh khung bài học đúng thang colon
 *
 * `new` tồn tại vì một lý do cụ thể: thang colon quá dễ gõ sai, và gõ sai thì
 * hỏng LẶNG LẼ. Tác giả nội dung không bao giờ nên phải tự đếm dấu hai chấm.
 */

import { readdir, readFile, mkdir, writeFile, rm } from 'node:fs/promises';
import { existsSync } from 'node:fs';
import { join, relative } from 'node:path';
import { bien_dich, LoiBienDich } from './lesson.js';
import { LoiDirective } from './directive.js';
import { LoiFrontmatter } from './frontmatter.js';

/** Bỏ tiền tố số thứ tự khỏi tên thư mục/file: `02-ra-lenh-cho-byte` →
 *  `ra-lenh-cho-byte`.
 *
 *  Số thứ tự thuộc về ĐƯỜNG DẪN, không thuộc về danh tính bài học. Chèn nó
 *  vào `id` nghĩa là mỗi lần chèn thêm một bài vào giữa mạch thì mọi bài sau
 *  đó đổi id — và mọi tiến độ người học đã lưu theo id ấy mất trắng. */
const bo_so = (s: string) => s.replace(/^\d+-/, '');

const KHUNG = (track: string, moduleId: string, slug: string) => `---
id: ${track}.${bo_so(moduleId)}.${bo_so(slug)}
title: TODO — tiêu đề ngắn, nói rõ người học sẽ LÀM được gì
summary: TODO — một câu
locale: vi
track: ${track}
module: ${bo_so(moduleId)}
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 10
teaches: []
requires: []
concepts: []
gradingMatrix:
  web-chrome: [static, run, tests, output]
provenance:
  authoredBy: human
  reviewed: false
---

::::byte{trigger=enter mood=curious pose=lean-in}
TODO — một câu của Byte, tối đa 90 ký tự, không dùng thuật ngữ chưa dạy.
::::

::::explain{#y-tuong}
TODO — giải thích. Ẩn dụ trước, thuật ngữ sau.
::::

::::example{#vi-du}
TODO — cho xem một ví dụ chạy được.

\`\`\`python title=readonly
print("xin chào")
\`\`\`
::::

::::predict{#doan commitOnce}
TODO — bắt người học CAM KẾT một dự đoán trước khi chạy.

:::opt{correct}
TODO — đáp án đúng
:::

:::opt
TODO — đáp án sai nhưng hợp lý
::why
TODO — vì sao nghĩ thế là tự nhiên, và chỗ nào lệch. Đây là chỗ dạy học
thật sự, không phải chỗ phạt.
::
:::
::::

::::code{#lam-thu}
TODO — yêu cầu, một câu, rõ ràng.

\`\`\`python title=starter
TODO
\`\`\`

\`\`\`python title=solution
TODO
\`\`\`

\`\`\`python title=test
TODO
\`\`\`

:::hints
- kind: attention
  body: TODO — hướng chú ý, KHÔNG cho đáp án.
- kind: strategy
  body: TODO — chiến lược, vẫn chưa cho đáp án.
- kind: one-line
  body: TODO — đúng một dòng code.
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
:::
::::

::::checkpoint{mastery=0.8}
::::
`;

/** Thứ tự học, đọc từ `content/curriculum/thu-tu.yaml`.
 *
 *  Đưa vào `index.json` để ứng dụng khỏi phải hiểu YAML. Tên thư mục KHÔNG
 *  quyết định được thứ tự — `nen-tang` (Realm 1) xếp trước `onboarding`
 *  (Realm 0) theo bảng chữ cái — nên thứ tự phải đi kèm nội dung, không suy
 *  ra được ở phía người đọc.
 *
 *  Đọc bằng tay thay vì kéo thêm một gói YAML: file này cố tình giữ hình dạng
 *  hai tầng đơn giản, và `tools/kiem_do_thi.py` cũng đọc nó y như vậy.
 */
async function doc_thu_tu(goc: string): Promise<{ id: string; ten: string; track: string[] }[]> {
  const tep = join(goc, 'curriculum', 'thu-tu.yaml');
  if (!existsSync(tep)) return [];
  const ra: { id: string; ten: string; track: string[] }[] = [];
  let hien: { id: string; ten: string; track: string[] } | null = null;
  let trong_track = false;
  for (const dong of (await readFile(tep, 'utf-8')).split('\n')) {
    const s = dong.trim();
    if (!s || s.startsWith('#')) continue;
    const m_id = /^- id:\s*(\S+)/.exec(s);
    if (m_id) {
      hien = { id: m_id[1]!, ten: '', track: [] };
      ra.push(hien);
      trong_track = false;
      continue;
    }
    const m_ten = /^ten:\s*"?([^"]*)"?/.exec(s);
    if (m_ten && hien) { hien.ten = m_ten[1]!.trim(); continue; }
    if (s === 'track:') { trong_track = true; continue; }
    if (trong_track && hien && s.startsWith('- ')) hien.track.push(s.slice(2).trim());
  }
  return ra;
}

async function tim_lesson(goc: string): Promise<string[]> {
  const ra: string[] = [];
  async function di(d: string): Promise<void> {
    for (const e of await readdir(d, { withFileTypes: true })) {
      if (e.name.startsWith('.') || e.name === 'node_modules') continue;
      const p = join(d, e.name);
      if (e.isDirectory()) await di(p);
      else if (e.name.endsWith('.lesson.md')) ra.push(p);
    }
  }
  await di(goc);
  return ra.sort();
}

async function build(goc: string, dich: string): Promise<number> {
  const tep = await tim_lesson(goc);

  // Dọn sạch thư mục đích trước khi ghi.
  //
  // Không dọn thì một bài bị xoá khỏi nguồn vẫn để lại file JSON của nó nằm
  // mãi ở đây, và ứng dụng vẫn nạp bài đó lên như thường — không có dấu hiệu
  // nào cho thấy nó đã chết. Đúng cùng một chế độ hỏng với khung `TODO —` lọt
  // cổng kiểm: thứ không còn tồn tại vẫn trông y như đang sống.
  await rm(dich, { recursive: true, force: true });
  let loi = 0;
  const tat_ca = [];
  for (const t of tep) {
    const rel = relative(goc, t);
    try {
      const l = bien_dich(rel, await readFile(t, 'utf-8'));
      tat_ca.push(l);
      await mkdir(dich, { recursive: true });
      await writeFile(join(dich, `${l.id}.json`), JSON.stringify(l, null, 2), 'utf-8');
    } catch (e) {
      loi++;
      if (e instanceof LoiBienDich || e instanceof LoiDirective || e instanceof LoiFrontmatter) {
        const dong = 'dong' in e ? e.dong : '?';
        console.error(`\n✖ ${rel}:${dong}\n  ${e.message}`);
        if ('goi_y' in e && e.goi_y) console.error(`  → ${e.goi_y}`);
      } else {
        console.error(`\n✖ ${rel}\n  ${(e as Error).message}`);
      }
    }
  }
  await mkdir(dich, { recursive: true });
  await writeFile(
    join(dich, 'index.json'),
    JSON.stringify(
      {
        generated: null,
        thuTu: await doc_thu_tu(goc),
        lessons: tat_ca.map((l) => ({
          id: l.id,
          track: l.track,
          module: l.module,
          order: l.order,
          title: l.title,
          tier: l.tier,
          estimatedMinutes: l.estimatedMinutes,
          teaches: l.teaches,
          requires: l.requires,
        })),
      },
      null,
      2,
    ),
    'utf-8',
  );
  console.log(`\n${tat_ca.length}/${tep.length} bài biên dịch được → ${dich}`);
  return loi;
}

const [, , lenh, ...dsl] = process.argv;

if (lenh === 'new') {
  const [duong_dan] = dsl;
  const phan = (duong_dan ?? '').split('/');
  if (phan.length !== 3) {
    console.error('Dùng: content new <track>/<module>/<slug>');
    process.exit(2);
  }
  const [track, mod, slug] = phan as [string, string, string];
  const thu_muc = join('content', track, mod);
  await mkdir(thu_muc, { recursive: true });
  const tep = join(thu_muc, `${slug}.lesson.md`);
  await writeFile(tep, KHUNG(track, mod, slug), { flag: 'wx' });
  console.log(`✓ ${tep}`);
} else if (lenh === 'build' || lenh === undefined) {
  const goc = dsl[0] ?? 'content';
  const dich = dsl[1] ?? 'dist/content';
  process.exit((await build(goc, dich)) > 0 ? 1 : 0);
} else {
  console.error(`Lệnh không hiểu: ${lenh}`);
  process.exit(2);
}
