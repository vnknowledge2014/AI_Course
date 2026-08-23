import { test } from 'node:test';
import assert from 'node:assert/strict';
import { doc_richtext } from '../dist/richtext.js';

/** Mục danh sách dài hơn một dòng nguồn.
 *
 *  Bản đầu tiên của bộ đọc thoát khỏi vòng lặp danh sách ngay khi gặp một
 *  dòng không mang dấu gạch — nên phần đuôi của mỗi mục rơi ra ngoài thành
 *  `<p>` mồ côi, và mục kế tiếp mở hẳn một `<ul>` mới. Lỗi này không nhìn
 *  thấy được từ phía người viết bài (họ viết Markdown đúng chuẩn) và đã âm
 *  thầm làm hỏng danh sách trong 21/40 bài Realm 0 trước khi bị bắt.
 */
test('mục danh sách xuống dòng vẫn nằm trong cùng một mục', () => {
  const cay = doc_richtext(
    ['- **Nói miệng.** Nghe xong là xong. Người nói phải', '  có mặt.', '- **Viết ra giấy.** Tờ giấy nằm đó.'].join(
      '\n',
    ),
  );
  assert.equal(cay.length, 1, 'phải gom thành đúng MỘT danh sách, không phải hai');
  assert.equal(cay[0].t, 'ul');
  assert.equal(cay[0].items.length, 2);

  const van = (item) => item[0].c.map((x) => x.v ?? x.c?.map((y) => y.v).join('') ?? '').join('');
  assert.match(van(cay[0].items[0]), /Người nói phải có mặt\./, 'phần đuôi phải được nối vào mục');
  assert.match(van(cay[0].items[1]), /Tờ giấy nằm đó\./);
});

test('dòng không thụt lề sau danh sách vẫn là đoạn văn riêng', () => {
  const cay = doc_richtext(['- một mục', 'Đoạn văn mới, không thuộc danh sách.'].join('\n'));
  assert.equal(cay.length, 2);
  assert.equal(cay[0].t, 'ul');
  assert.equal(cay[1].t, 'p');
});

test('danh sách đánh số cũng nối được dòng tiếp', () => {
  const cay = doc_richtext(['1. việc đầu tiên, còn', '   dài nữa', '2. việc thứ hai'].join('\n'));
  assert.equal(cay.length, 1);
  assert.equal(cay[0].t, 'ol');
  assert.equal(cay[0].items.length, 2);
});

/** Dấu nháy của YAML không được lọt vào chữ người học nhìn thấy.
 *
 *  YAML bắt đặt nháy khi giá trị chứa dấu hai chấm — mà gợi ý thì hay chứa
 *  (`Viết {x:,} vào chỗ trống`). Bản đầu giữ nguyên cặp nháy ấy, nên 73 gợi ý
 *  trong repo hiện ra kèm hai dấu nháy thừa, và những gợi ý DẠY VỀ dấu nháy
 *  còn kèm cả dấu gạch chéo ngược.
 */
import { bien_dich } from '../dist/lesson.js';

const KHUNG = (than) => `---
id: t.m.s
title: Thử
summary: Một câu
locale: vi
track: t
module: m
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 10
---

::::predict{#p commitOnce}
Đoán đi.

:::opt{correct}
đúng
:::

:::opt
sai
::why
Gần đúng ở chỗ bạn thử.
::
:::
::::

::::code{#c}
Làm đi.

\`\`\`python title=starter
x = 1
\`\`\`

\`\`\`python title=solution
x = 1
\`\`\`

\`\`\`python title=test
assert x == 1
\`\`\`

:::hints
${than}
:::

:::validate
- tier: run
  timeoutMs: 4000
:::
::::

::::reflect{#r}
Nghĩ lại.
::::
`;

const van = (bai) => {
  const h = bai.steps.find((s) => s.kind === 'code').hints.rungs;
  return h.map((r) => r.body.flatMap((n) => n.c ?? []).map((c) => c.v ?? '').join(''));
};

test('bóc cặp nháy YAML bao trọn dòng', () => {
  const b = bien_dich('t.md', KHUNG(
    `- kind: attention
  body: "Nhìn vào chỗ có dấu hai chấm: ngay sau tên biến."
- kind: strategy
  body: Không có nháy thì giữ nguyên
- kind: one-line
  body: "Viết x = 1"`,
  ));
  const t = van(b);
  assert.ok(!t[0].startsWith('"'), `còn nháy thừa: ${t[0]}`);
  assert.match(t[0], /^Nhìn vào chỗ/);
  assert.equal(t[1], 'Không có nháy thì giữ nguyên');
  assert.ok(!t[2].startsWith('"'));
});

test('gỡ escape cho nháy nằm bên trong', () => {
  const b = bien_dich('t.md', KHUNG(
    `- kind: attention
  body: "Viết \\"Xin chào\\" vào chỗ trống."
- kind: strategy
  body: s
- kind: one-line
  body: s`,
  ));
  const t = van(b)[0];
  assert.ok(!t.includes('\\'), `còn dấu gạch chéo ngược: ${t}`);
  assert.equal(t, 'Viết "Xin chào" vào chỗ trống.');
});

test('nháy KHÔNG ôm trọn dòng thì giữ nguyên', () => {
  // `"Đủ tiền" nghĩa là…` — ở đây cặp nháy là một phần của điều đang dạy.
  const b = bien_dich('t.md', KHUNG(
    `- kind: attention
  body: "Đủ tiền" nghĩa là khách đưa nhiều hơn giá.
- kind: strategy
  body: s
- kind: one-line
  body: s`,
  ));
  assert.match(van(b)[0], /^"Đủ tiền" nghĩa là/);
});

/** `target: "*"` phải khớp toán tử nhân, không phải chuỗi ba ký tự.
 *
 *  Thiếu bước bóc nháy thì luật `static` im lặng không chạy — cùng chế độ
 *  hỏng với `requireAst` thành mảng rỗng, và với khung `TODO —` biên dịch
 *  được: một luật có mặt trong file nhưng không đo gì cả.
 */
test('bóc nháy cho giá trị trong mục danh sách của validate', () => {
  const b = bien_dich('t.md', KHUNG(
    `- kind: attention
  body: a
- kind: strategy
  body: b
- kind: one-line
  body: c`,
  ).replace(
    `:::validate
- tier: run
  timeoutMs: 4000
:::`,
    `:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: phải dùng phép nhân
  requireAst:
  - kind: uses-operator, target: "*"
  - kind: uses-call, target: print
:::`,
  ));
  const r = b.steps.find((s) => s.kind === 'code').validation.rules.find((x) => x.tier === 'static');
  assert.equal(r.requireAst[0].target, '*', `còn nháy: ${JSON.stringify(r.requireAst[0].target)}`);
  assert.equal(r.requireAst[1].target, 'print');
});
