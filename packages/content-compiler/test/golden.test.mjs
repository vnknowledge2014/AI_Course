import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { bien_dich, phan_tich_directive, LoiDirective, LoiBienDich } from '../dist/index.js';

const GOLDEN = new URL(
  '../../../content/onboarding/01-may-tinh-noi-gi/01-byte-noi-gi.lesson.md',
  import.meta.url,
);

test('bài học vàng biên dịch được', async () => {
  const src = await readFile(GOLDEN, 'utf-8');
  const l = bien_dich('golden', src);
  assert.equal(l.id, 'onboarding.may-tinh-noi-gi.byte-noi-gi');
  assert.equal(l.locale, 'vi');
  assert.equal(l.tier, 'A');
  assert.equal(l.estimatedMinutes, 8);
  assert.ok(l.steps.length >= 6, `chỉ có ${l.steps.length} bước`);
});

test('mọi loại bước đều có mặt và đúng kind', async () => {
  const l = bien_dich('golden', await readFile(GOLDEN, 'utf-8'));
  const kinds = l.steps.map((s) => s.kind);
  for (const k of ['explain', 'example', 'predict', 'code', 'reflect', 'checkpoint']) {
    assert.ok(kinds.includes(k), `thiếu bước \`${k}\`, chỉ có: ${kinds.join(', ')}`);
  }
});

test('bước predict có đúng một đáp án đúng và có giải thích cho đáp án sai', async () => {
  const l = bien_dich('golden', await readFile(GOLDEN, 'utf-8'));
  const p = l.steps.find((s) => s.kind === 'predict');
  const opts = p.answer.options;
  assert.equal(opts.filter((o) => o.correct).length, 1);
  assert.ok(opts.length >= 3, 'nên có ít nhất 3 lựa chọn');
  // Đáp án SAI phải có `why` — đó là chỗ dạy học thật sự, không phải chỗ phạt.
  for (const o of opts.filter((x) => !x.correct)) {
    assert.ok(o.why, 'mọi lựa chọn sai phải có phần giải thích `why`');
  }
});

test('bước code có đủ starter/solution/test và thang gợi ý 3 bậc', async () => {
  const l = bien_dich('golden', await readFile(GOLDEN, 'utf-8'));
  const c = l.steps.find((s) => s.kind === 'code');
  assert.ok(c.code.starter.includes('___'), 'starter phải có chỗ trống');
  assert.ok(c.code.solution, 'phải có lời giải tham chiếu');
  assert.ok(c.code.test !== undefined, 'phải có khối test');
  assert.ok(c.hints.rungs.length >= 3, 'thang gợi ý phải có ít nhất 3 bậc');
  assert.deepEqual(
    c.hints.rungs.map((r) => r.kind),
    ['attention', 'strategy', 'one-line'],
    'thang gợi ý phải đi từ hướng chú ý → chiến lược → một dòng code',
  );
});

// ── Cổng chất lượng: compiler phải TỪ CHỐI bài học thiếu sót ────────────────

function phai_loi(src, chua) {
  try {
    bien_dich('thu', src);
    assert.fail('mong đợi biên dịch thất bại');
  } catch (e) {
    assert.ok(e instanceof LoiBienDich, `mong đợi LoiBienDich, gặp ${e.name}`);
    assert.ok(e.message.includes(chua) || e.goi_y?.includes(chua),
      `thông báo lỗi phải nhắc "${chua}", đang là: ${e.message} / ${e.goi_y}`);
  }
}

const FM = `---
id: t.m.s
title: Thử
track: onboarding
module: m
estimatedMinutes: 8
---
`;

test('từ chối bước code thiếu lời giải tham chiếu', () => {
  phai_loi(FM + '\n::::code{#x}\nLàm đi\n\n```python title=starter\n___\n```\n::::\n', 'solution');
});

test('từ chối bước code thiếu thang gợi ý', () => {
  phai_loi(
    FM + '\n::::code{#x}\nLàm đi\n\n```python title=starter\n___\n```\n```python title=solution\n1\n```\n```python title=test\npass\n```\n::::\n',
    'hints',
  );
});

test('từ chối predict không có đúng một đáp án đúng', () => {
  phai_loi(FM + '\n::::predict{#x}\nĐoán\n\n:::opt\na\n:::\n:::opt\nb\n:::\n::::\n', 'correct');
});

test('từ chối bài dài quá 15 phút — nguyên tắc chậm mà chắc', () => {
  phai_loi(FM.replace('estimatedMinutes: 8', 'estimatedMinutes: 40'), 'chậm mà chắc');
});

// ── Thang colon: lỗi im lặng nguy hiểm nhất của remark-directive ────────────

test('phát hiện directive con có số colon không ít hơn cha', () => {
  try {
    phan_tich_directive(':::code{#a}\n:::hints\nx\n:::\n:::\n');
    assert.fail('mong đợi lỗi thang colon');
  } catch (e) {
    assert.ok(e instanceof LoiDirective);
    assert.ok(e.goi_y.includes('ĐÓNG LUÔN'), 'phải giải thích hậu quả im lặng');
  }
});

test('thang colon đúng thì parse được và lồng đúng', () => {
  const d = phan_tich_directive('::::code{#a}\nthân\n:::hints\nx\n:::\n::::\n');
  assert.equal(d.length, 1);
  assert.equal(d[0].ten, 'code');
  assert.equal(d[0].id, 'a');
  assert.equal(d[0].con.length, 1);
  assert.equal(d[0].con[0].ten, 'hints');
});

test('directive trong code fence không phá cấu trúc', () => {
  const d = phan_tich_directive(
    '::::explain{#a}\n```md\n:::opt\nví dụ minh hoạ cú pháp\n:::\n```\n::::\n',
  );
  assert.equal(d.length, 1);
  assert.equal(d[0].con.length, 0, 'directive trong fence phải bị coi là văn bản thường');
});

test('directive mở mà không đóng bị bắt', () => {
  assert.throws(() => phan_tich_directive('::::code{#a}\nthân\n'), /chưa đóng/);
});

// ── Hai chế độ hỏng LẶNG LẼ, mỗi cái từng lọt qua cả chín cổng ────────────

test('khối `title=readonly` trong `predict` KHÔNG bị vứt', () => {
  const l = bien_dich('t.lesson.md', `---
id: t.m.a
title: T
summary: S
locale: vi
track: t
module: m
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 8
teaches: []
requires: []
concepts: []
gradingMatrix:
  web-chrome: [run]
provenance:
  authoredBy: human
  reviewed: false
---

::::predict{#p commitOnce}
Đoán xem màn hình in ra gì?

\`\`\`python title=readonly
print(2 + 3)
\`\`\`

:::opt{correct}
5
:::

:::opt
23
::why
Dấu cộng giữa hai SỐ là phép cộng, không phải phép nối chuỗi.
::
:::
::::

::::reflect{#r}
Xong.
::::

::::checkpoint{mastery=0.8}
::::
`);
  const p = l.steps.find((s) => s.kind === 'predict');
  const khoi = JSON.stringify(p.body).includes('"src":"print(2 + 3)"');
  assert.ok(khoi, `đoạn mã cần đoán biến mất khỏi thân bước: ${JSON.stringify(p.body)}`);
});

test('`title=solution` trong `predict` bị NÉM, không im lặng in đáp án', () => {
  const viet = (title) => `---
id: t.m.b
title: T
summary: S
locale: vi
track: t
module: m
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 8
teaches: []
requires: []
concepts: []
gradingMatrix:
  web-chrome: [run]
provenance:
  authoredBy: human
  reviewed: false
---

::::predict{#p commitOnce}
Đoán đi.

\`\`\`python title=${title}
print(5)
\`\`\`

:::opt{correct}
5
:::

:::opt
x
::why
Vì sao nghĩ thế là tự nhiên.
::
:::
::::

::::reflect{#r}
Xong.
::::

::::checkpoint{mastery=0.8}
::::
`;
  assert.throws(() => bien_dich('t.lesson.md', viet('solution')), LoiBienDich);
  assert.doesNotThrow(() => bien_dich('t.lesson.md', viet('readonly')));
});

test('`:::validate` bóc nháy khỏi giá trị', () => {
  const l = bien_dich('t.lesson.md', `---
id: t.m.c
title: T
summary: S
locale: vi
track: t
module: m
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 8
teaches: []
requires: []
concepts: []
gradingMatrix:
  web-chrome: [output]
provenance:
  authoredBy: human
  reviewed: false
---

::::code{#c}
Làm đi.

\`\`\`python title=starter
print("Đơn DH-4KM: xong")
\`\`\`

\`\`\`python title=solution
print("Đơn DH-4KM: xong")
\`\`\`

\`\`\`python title=test
assert True, "luôn đạt"
\`\`\`

:::hints
- kind: attention
  body: Nhìn dòng đầu.
- kind: strategy
  body: Nghĩ theo hướng này.
- kind: one-line
  body: Viết đúng một dòng.
:::

:::validate
- tier: output
  expect: 'Đơn DH-4KM: xong'
:::
::::

::::reflect{#r}
Xong.
::::

::::checkpoint{mastery=0.8}
::::
`);
  const v = l.steps.find((s) => s.kind === 'code').validation;
  const r = v.rules[0];
  const gt = JSON.stringify(r);
  assert.ok(!gt.includes("\\'Đơn"), `dấu nháy còn dính vào giá trị: ${gt}`);
  assert.ok(gt.includes('Đơn DH-4KM: xong'), gt);
});
