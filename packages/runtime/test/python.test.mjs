import { test } from 'node:test';
import assert from 'node:assert/strict';
import { chanDoanTuTraceback } from '../dist/index.js';

test('NameError -> chẩn đoán ba tầng', () => {
  const d = chanDoanTuTraceback(
    'Traceback (most recent call last):\n  File "<exec>", line 3, in <module>\nNameError: name \'tong\' is not defined',
  );
  assert.equal(d.ma, 'PY0501');
  assert.equal(d.dong, 3, 'phải lấy số dòng của mã người học');
  assert.ok(d.viSao);
  assert.ok(d.cachSua.length > 0);
  assert.equal(d.khaiNiem, 'biến');
});

test('lấy số dòng CUỐI CÙNG, bỏ qua khung của Pyodide', () => {
  const tb = [
    'Traceback (most recent call last):',
    '  File "/lib/python3.12/site-packages/pyodide/_base.py", line 468, in eval_code',
    '  File "<exec>", line 7, in <module>',
    'ZeroDivisionError: division by zero',
  ].join('\n');
  const d = chanDoanTuTraceback(tb);
  assert.equal(d.ma, 'PY0505');
  assert.equal(d.dong, 7, 'phải là dòng 7 chứ không phải 468 của Pyodide');
});

test('IndentationError giải thích đúng đặc thù Python', () => {
  const d = chanDoanTuTraceback('  File "<exec>", line 2\nIndentationError: expected an indented block');
  assert.equal(d.ma, 'PY0506');
  assert.equal(d.khaiNiem, 'thụt lề');
  assert.ok(d.cachSua.some((s) => s.includes('Tab')), 'phải cảnh báo trộn Tab với dấu cách');
});

test('TypeError gợi ý đúng cách nối chuỗi của Python', () => {
  const d = chanDoanTuTraceback('TypeError: can only concatenate str (not "int") to str');
  assert.equal(d.ma, 'PY0502');
  assert.ok(d.cachSua.some((s) => s.includes('f"')), 'phải gợi ý f-string');
});

test('mọi loại lỗi đều ra chẩn đoán hợp lệ, không ném', () => {
  const cac = [
    'KeyError: \'ten\'', 'IndexError: list index out of range',
    'AttributeError: no attribute', 'RecursionError: maximum recursion depth exceeded',
    'AssertionError', 'SyntaxError: invalid syntax', 'TabError: inconsistent use of tabs',
    'ValueError: gì đó lạ', '', 'không phải traceback gì cả',
  ];
  for (const tb of cac) {
    const d = chanDoanTuTraceback(tb);
    assert.equal(typeof d.ma, 'string');
    assert.equal(d.muc, 'loi');
    assert.ok(d.dong >= 1);
    assert.equal(typeof d.thongDiep, 'string');
    assert.ok(Array.isArray(d.cachSua));
  }
});

test('tiếng Việt trong thông báo lỗi không bị hỏng', () => {
  const d = chanDoanTuTraceback('ValueError: giá trị không hợp lệ cho tuổi');
  assert.ok(d.thongDiep.includes('giá trị không hợp lệ'));
});
