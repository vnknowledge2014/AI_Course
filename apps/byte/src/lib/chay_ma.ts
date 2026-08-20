/** Cầu nối giữa giao diện và bộ thực thi.
 *
 *  Một worker DÙNG CHUNG cho cả phiên: Pyodide mất vài giây để nạp, tạo lại
 *  cho mỗi lần bấm chạy thì mỗi lần bấm là một lần chờ. Đổi lại, khi worker bị
 *  giết vì hết giờ thì lần chạy kế tiếp phải nạp lại — `BoThucThiPython` tự lo
 *  việc đó.
 */
import { BoThucThiPython } from '@byte/exec-python';
import type { CongWorker, KetQuaChay, TinNhanTuWorker } from '@byte/exec-core';

function tao_cong(): CongWorker {
  const w = new Worker(new URL('./python.worker.ts', import.meta.url), { type: 'module' });
  return {
    gui: (tin) => w.postMessage(tin),
    khiNhan: (xu_ly: (t: TinNhanTuWorker) => void) => {
      w.onmessage = (e: MessageEvent<TinNhanTuWorker>) => xu_ly(e.data);
    },
    giet: () => w.terminate(),
  };
}

let may: BoThucThiPython | null = null;

export function bo_thuc_thi(): BoThucThiPython {
  may ??= new BoThucThiPython(tao_cong);
  return may;
}

export async function chay_python(ma: string, ma_kiem_tra?: string): Promise<KetQuaChay> {
  const m = bo_thuc_thi();
  await m.sanSang();
  return ma_kiem_tra ? m.chay(ma, { maKiemTra: ma_kiem_tra }) : m.chay(ma);
}
