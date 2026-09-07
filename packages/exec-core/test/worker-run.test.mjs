import { test } from 'node:test';
import assert from 'node:assert/strict';
import { chayCoHetGio } from '../dist/worker.js';

// Cổng giả lập trong bộ nhớ — không cần Worker thật để kiểm hành vi timeout/
// khớp id, vốn là phần LÕI mà exec-python và exec-typescript trước đây mỗi
// bên tự viết lại một bản gần như y hệt.
function taoCongGia({ treo = false, idPhanHoi } = {}) {
  let nhan = null;
  let daGiet = false;
  const goiDaGui = [];
  return {
    cong: {
      gui: (tin) => {
        goiDaGui.push(tin);
        if (!treo) {
          // Trả lời NGAY (đồng bộ giả lập) với id đúng như worker thật.
          queueMicrotask(() => nhan?.({ loai: 'xong', id: idPhanHoi ?? tin.id, ok: true, xuat: 'ok', loi: null }));
        }
      },
      khiNhan: (f) => { nhan = f; },
      giet: () => { daGiet = true; },
    },
    guiLai: () => nhan,
    daGietChua: () => daGiet,
    goiDaGui,
  };
}

test('chayCoHetGio: worker trả lời kịp — trả về phản hồi, KHÔNG giết worker', async () => {
  const { cong, daGietChua } = taoCongGia();
  const ket = await chayCoHetGio(cong, { id: 1, ma: 'print(1)' }, 1000, () => {});
  assert.ok(ket !== null);
  assert.equal(ket.loai, 'xong');
  assert.equal(ket.id, 1);
  assert.equal(daGietChua(), false);
});

test('chayCoHetGio: hết giờ — giết worker, gọi khiHetGio, trả về null', async () => {
  const { cong, daGietChua } = taoCongGia({ treo: true });
  let goiHetGio = false;
  const ket = await chayCoHetGio(cong, { id: 1, ma: 'while True: pass' }, 30, () => { goiHetGio = true; });
  assert.equal(ket, null);
  assert.equal(daGietChua(), true);
  assert.equal(goiHetGio, true);
});

test('chayCoHetGio: BỎ QUA phản hồi có id KHÁC (còn sót lại từ lượt gọi trước)', async () => {
  const { cong } = taoCongGia({ idPhanHoi: 999 }); // worker trả lời id=999, ta chờ id=1
  const ket = await chayCoHetGio(cong, { id: 1, ma: 'x' }, 30, () => {});
  // id không khớp nên KHÔNG resolve theo nhánh khớp — hết giờ mới resolve null.
  assert.equal(ket, null);
});

test('chayCoHetGio: gửi ĐÚNG hình dạng YeuCauChay qua cong.gui, gồm cả maKiemTra/luoi', async () => {
  const { cong, goiDaGui } = taoCongGia();
  await chayCoHetGio(cong, { id: 5, ma: 'x = 1', maKiemTra: 'assert x == 1', luoi: { family: 'grid-bot' } }, 1000, () => {});
  assert.equal(goiDaGui.length, 1);
  assert.deepEqual(goiDaGui[0], { loai: 'chay', id: 5, ma: 'x = 1', maKiemTra: 'assert x == 1', luoi: { family: 'grid-bot' } });
});
