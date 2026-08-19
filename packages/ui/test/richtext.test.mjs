import { test } from 'node:test';
import assert from 'node:assert/strict';
import { ket_xuat, ket_xuat_inline } from '../dist/index.js';

test('nội dung được thoát HTML — không bao giờ coi là mã', () => {
  const h = ket_xuat([{ t: 'p', c: [{ t: 'txt', v: '<script>alert(1)</script>' }] }]);
  assert.ok(!h.includes('<script>'), 'thẻ script phải bị thoát');
  assert.ok(h.includes('&lt;script&gt;'));
});

test('code block giữ nguyên văn và thoát đúng', () => {
  const h = ket_xuat([{ t: 'code', lang: 'python', src: 'if a < b:\n    print("&")' }]);
  assert.ok(h.includes('data-ngon-ngu="python"'));
  assert.ok(h.includes('a &lt; b'));
  assert.ok(h.includes('&amp;'));
});

test('liên kết ra ngoài luôn có rel an toàn', () => {
  const h = ket_xuat_inline([{ t: 'link', href: 'https://a.b', c: [{ t: 'txt', v: 'x' }] }]);
  assert.ok(h.includes('rel="noopener noreferrer"'));
});

test('bảng tự cuộn ngang — trang không bao giờ cuộn ngang', () => {
  const h = ket_xuat([{ t: 'table', head: [[{ t: 'txt', v: 'a' }]], rows: [[[{ t: 'txt', v: '1' }]]] }]);
  assert.ok(h.includes('class="bang-cuon"'));
});

test('callout có nhãn tiếng Việt', () => {
  const h = ket_xuat([{ t: 'callout', variant: 'pitfall', c: [{ t: 'p', c: [{ t: 'txt', v: 'x' }] }] }]);
  assert.ok(h.includes('Chỗ hay vấp'));
});

test('tiếng Việt có dấu giữ nguyên', () => {
  const h = ket_xuat([{ t: 'p', c: [{ t: 'txt', v: 'Nguyễn Văn A — 25 tuổi' }] }]);
  assert.ok(h.includes('Nguyễn Văn A — 25 tuổi'));
});

test('khái niệm thành nút bấm được để mở giải thích', () => {
  const h = ket_xuat_inline([{ t: 'concept', id: 'fp.functor', c: [{ t: 'txt', v: 'functor' }] }]);
  assert.ok(h.includes('data-khai-niem="fp.functor"'));
});
