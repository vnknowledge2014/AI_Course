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
