import { test } from 'node:test';
import assert from 'node:assert/strict';
import { tab, tab_nguoc, enter, backspace } from '../dist-test/soan_thao.js';

const o = (van, dau, cuoi = dau) => ({ van, dau, cuoi });

test('Tab chèn cho TRÒN mốc lề, không phải luôn 4', () => {
  assert.equal(tab(o('', 0)).van, '    ');
  // Con trỏ ở cột 2 thì Tab đưa tới cột 4, giống mọi trình soạn thảo khác.
  assert.equal(tab(o('ab', 2)).van, 'ab  ');
  assert.equal(tab(o('abcd', 4)).van, 'abcd    ');
});

test('Tab khi chọn nhiều dòng thì thụt cả khối', () => {
  const r = tab(o('a\nb', 0, 3));
  assert.equal(r.van, '    a\n    b');
});

test('Shift+Tab giảm lề, và không giảm quá lề 0', () => {
  assert.equal(tab_nguoc(o('    a', 4)).van, 'a');
  assert.equal(tab_nguoc(o('a', 1)).van, 'a', 'đã sát lề thì giữ nguyên');
  assert.equal(tab_nguoc(o('  a', 3)).van, 'a', 'lề lẻ thì xoá hết phần lẻ');
});

/** Đây là chỗ Python khác hẳn mọi ngôn ngữ người học sẽ gặp sau này. */
test('Enter sau dấu hai chấm thì thụt thêm một mức', () => {
  const r = enter(o('if x:', 5));
  assert.equal(r.van, 'if x:\n    ');
  assert.equal(r.dau, 10);
});

test('Enter giữ nguyên lề của dòng trước', () => {
  const r = enter(o('if x:\n    a = 1', 15));
  assert.equal(r.van, 'if x:\n    a = 1\n    ');
});

test('Enter sau dòng thụt KẾT bằng hai chấm thì cộng dồn lề', () => {
  const r = enter(o('if x:\n    if y:', 15));
  assert.equal(r.van, 'if x:\n    if y:\n        ');
});

test('Backspace ở đầu dòng thụt xoá CẢ MỘT MỨC', () => {
  // Xoá từng dấu cách nghĩa là bấm bốn lần, và ba lần đầu để lại một mức lề
  // không hợp lệ mà Python sẽ từ chối.
  const r = backspace(o('    a', 4));
  assert.equal(r.van, 'a');
  assert.equal(r.dau, 0);
});

test('Backspace giữa chữ thì để trình duyệt tự lo', () => {
  assert.equal(backspace(o('abc', 2)), null);
  assert.equal(backspace(o('    abc', 6)), null, 'sau chữ thì không phải chuyện lề');
});

test('Backspace lề lẻ xoá về mốc gần nhất', () => {
  assert.equal(backspace(o('      a', 6)).van, '    a');
});
