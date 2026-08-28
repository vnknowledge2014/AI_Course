import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { dung_ban_do, bai_ke_tiep, so_ky_nang } from '../dist-test/cay_ky_nang.js';

const IDX = JSON.parse(readFileSync(new URL('../public/noi-dung/index.json', import.meta.url), 'utf-8'));
const DS = IDX.lessons;
const THU_TU = IDX.thuTu;
const rong = { buoc_xong: {}, da_xong: [] };

test('người mới: đúng những bài không có tiền đề mới mở', () => {
  const bd = dung_ban_do(DS, THU_TU, rong);
  const mo = bd.flatMap((r) => r.track.flatMap((t) => t.bai.filter((b) => b.mo)));
  assert.ok(mo.length > 0, 'phải có ít nhất một bài mở, không thì không ai vào được');
  for (const b of mo) {
    assert.deepEqual(b.requires ?? [], [], `\`${b.id}\` mở nhưng vẫn khai requires`);
  }
});

test('bài đầu tiên của Realm 0 luôn mở', () => {
  const bd = dung_ban_do(DS, THU_TU, rong);
  const r0 = bd.find((r) => r.id === 'onboarding');
  assert.ok(r0, 'phải có Realm 0');
  assert.equal(r0.track[0].bai[0].mo, true);
});

test('học xong một bài thì mở đúng những bài chỉ chờ skill của nó', () => {
  const bd0 = dung_ban_do(DS, THU_TU, rong);
  const dau = bd0[0].track[0].bai[0];
  const truoc = bd0.flatMap((r) => r.track.flatMap((t) => t.bai.filter((b) => b.mo))).length;

  const sau_bd = dung_ban_do(DS, THU_TU, { buoc_xong: {}, da_xong: [dau.id] });
  const sau = sau_bd.flatMap((r) => r.track.flatMap((t) => t.bai.filter((b) => b.mo))).length;
  assert.ok(sau >= truoc, 'học xong một bài không bao giờ được làm ĐÓNG bớt bài nào');
});

test('bài bị khoá nói được nó đang chờ gì', () => {
  const bd = dung_ban_do(DS, THU_TU, rong);
  const khoa = bd.flatMap((r) => r.track.flatMap((t) => t.bai.filter((b) => !b.mo)));
  assert.ok(khoa.length > 0, 'phải có bài bị khoá, không thì đồ thị tiền đề vô nghĩa');
  for (const b of khoa) {
    assert.ok(b.con_thieu.length > 0, `\`${b.id}\` bị khoá mà không nêu thiếu gì`);
    // Ít nhất một tiền đề chỉ được đúng tên bài dạy nó — nếu không, người học
    // bị chặn mà không biết đi đâu.
    assert.ok(b.con_thieu.some((c) => c.hoc_o), `\`${b.id}\` chặn người học mà không chỉ đường`);
  }
});

test('học hết thì mọi bài đều mở và bai_ke_tiep trả null', () => {
  const het = { buoc_xong: {}, da_xong: DS.map((l) => l.id) };
  const bd = dung_ban_do(DS, THU_TU, het);
  const con_khoa = bd.flatMap((r) => r.track.flatMap((t) => t.bai.filter((b) => !b.mo)));
  assert.deepEqual(con_khoa.map((b) => b.id), [], 'học hết mà vẫn còn bài khoá = đồ thị có vòng hoặc skill treo');
  assert.equal(bai_ke_tiep(bd), null);
});

test('đếm kỹ năng, không đếm bài', () => {
  assert.equal(so_ky_nang(DS, rong), 0);
  const mot = DS.find((l) => (l.teaches ?? []).length > 0);
  assert.equal(so_ky_nang(DS, { buoc_xong: {}, da_xong: [mot.id] }), new Set(mot.teaches).size);
});

test('track chưa viết bài nào vẫn hiện, với tổng 0', () => {
  // Dựng một track RỖNG tổng hợp, thay vì trông chờ repo còn track chưa viết.
  //
  // Bản trước lọc `tong === 0` trên dữ liệu THẬT rồi khẳng định phải tìm được
  // ít nhất một. Nó xanh suốt nhiều tháng vì lúc nào cũng còn track dở dang —
  // rồi ĐỎ đúng vào ngày viết xong bài cuối của v1.0. Một test chỉ đúng khi
  // công việc CHƯA xong thì nó không kiểm hành vi, nó kiểm tiến độ.
  //
  // Hành vi cần giữ vẫn thật và vẫn quan trọng: track đã có mạch mà chưa viết
  // bài nào PHẢI hiện ra trên bản đồ — không thì người học không thấy đường
  // trước mặt, và tiến độ trông cao hơn thực tế.
  const track_rong = { id: 'chua-viet', ten: 'Track chưa viết', module: ['module-khong-co-bai'] };
  const thu_tu_them_track_rong = [
    ...THU_TU,
    { id: 'realm-thu-nghiem', ten: 'Realm thử nghiệm', track: [track_rong] },
  ];

  const bd = dung_ban_do(DS, thu_tu_them_track_rong, rong);
  const trong = bd.flatMap((r) => r.track).filter((t) => t.tong === 0);
  assert.ok(trong.length > 0, 'track đã có mạch nhưng chưa viết phải hiện ra');
  for (const t of trong) assert.equal(t.xong, 0);
});
