import { test } from 'node:test';
import assert from 'node:assert/strict';
import { Worker } from 'node:worker_threads';
import { BoThucThiTypeScript, chanDoanTuLoiJs } from '../dist/index.js';

const WORKER = new URL('./ts-worker.mjs', import.meta.url);

/** Adapter Node cho giao diện CongWorker. */
function taoCong() {
  const w = new Worker(WORKER);
  let handler = null;
  w.on('message', (m) => handler?.(m));
  w.on('error', () => {});
  return {
    gui: (tin) => w.postMessage(tin),
    khiNhan: (f) => { handler = f; },
    giet: () => { w.terminate(); },
  };
}

const engine = new BoThucThiTypeScript(taoCong);

test('chạy chương trình đúng và thu được output', async () => {
  const r = await engine.chay('const a: number = 2; console.log(a * 21);');
  assert.equal(r.ok, true, r.chanDoan[0]?.vanBan);
  assert.equal(r.xuat, '42');
});

test('type annotation bị strip đúng', async () => {
  const r = await engine.chay(
    'interface P { x: number }\nconst p: P = { x: 7 };\nconsole.log(p.x);',
  );
  assert.equal(r.ok, true);
  assert.equal(r.xuat, '7');
});

test('VÒNG LẶP VÔ HẠN bị cắt, không treo tiến trình', async () => {
  const t0 = Date.now();
  const r = await engine.chay('while (true) {}', { hetHanMs: 800 });
  const troi = Date.now() - t0;
  assert.equal(r.biNgat, true, 'phải đánh dấu bị ngắt');
  assert.equal(r.ok, false);
  // Mã EX05xx: chẩn đoán chung của tầng exec-core, không riêng ngôn ngữ nào.
  assert.equal(r.chanDoan[0].ma, 'EX0500');
  assert.ok(troi < 3000, `mất ${troi}ms — phải cắt gần mốc 800ms`);
});

test('sau khi giết worker vẫn chạy tiếp được', async () => {
  await engine.chay('while (true) {}', { hetHanMs: 500 });
  const r = await engine.chay('console.log("vẫn sống");');
  assert.equal(r.ok, true, 'engine phải tự dựng lại worker mới');
  assert.equal(r.xuat, 'vẫn sống');
});

test('maKiemTra phát hiện lời giải sai', async () => {
  const dung = await engine.chay('function cong(a: number, b: number) { return a + b; }', {
    maKiemTra: 'if (cong(2,3) !== 5) throw new Error("sai"); console.log("dat");',
  });
  assert.equal(dung.ok, true);

  const sai = await engine.chay('function cong(a: number, b: number) { return a - b; }', {
    maKiemTra: 'if (cong(2,3) !== 5) throw new Error("sai");',
  });
  assert.equal(sai.ok, false, 'lời giải sai KHÔNG được báo pass');
});

test('tiếng Việt trong output nguyên vẹn', async () => {
  const r = await engine.chay('console.log("Nguyễn Văn A — 25 tuổi");');
  assert.equal(r.xuat, 'Nguyễn Văn A — 25 tuổi');
});

test('lỗi JS được đổi thành chẩn đoán ba tầng', () => {
  const d = chanDoanTuLoiJs('ReferenceError: x is not defined');
  assert.equal(d.ma, 'TS0501');
  assert.ok(d.viSao);
  assert.ok(d.cachSua.length > 0);
  assert.equal(d.khaiNiem, 'biến');

  const d2 = chanDoanTuLoiJs("TypeError: Cannot read properties of undefined (reading 'x')");
  assert.equal(d2.ma, 'TS0503');
  assert.ok(d2.cachSua.some((s) => s.includes('?.')));
});

test('lỗi runtime thật đi qua đủ đường', async () => {
  const r = await engine.chay('const v: any = undefined; console.log(v.x);');
  assert.equal(r.ok, false);
  assert.equal(r.chanDoan[0].ma, 'TS0503');
});

test('dọn dẹp', () => { engine.dong(); });
