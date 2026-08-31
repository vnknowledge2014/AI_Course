/**
 * Chạy THẬT lời giải của mọi bài học, qua chính engine mà người học sẽ dùng.
 *
 * Trình biên dịch kiểm CẤU TRÚC bài (có đủ solution/test/hints không).
 * `kiem_bai_hoc.py` kiểm SƯ PHẠM (mọi đáp án sai có `::why` không).
 * Không cái nào kiểm được điều quan trọng nhất: **mã trong bài có chạy được
 * không, và khối test có thật sự đạt không**.
 *
 * Thiếu cổng này thì một bài dạy code sai vẫn xanh hết mọi cửa. Chuyện đó đã
 * xảy ra một lần trong dự án này — một test tự viết dạy Rust không hợp lệ mà
 * `rustc` từ chối — nên đây không phải phòng xa lý thuyết.
 *
 *     node tools/kiem_ma_bai_hoc.mjs [thư-mục-json]
 *
 * Mặc định đọc `dist/content`. Chạy `content-compiler build` trước.
 */
import { readdirSync, readFileSync, statSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { createRequire } from 'node:module';
import { pathToFileURL } from 'node:url';
import { Worker } from 'node:worker_threads';

const THU_MUC = process.argv[2] ?? 'dist/content';

// `pyodide` là phụ thuộc của `packages/exec-python`, không của thư mục gốc —
// nên phải phân giải qua đó thay vì import trần.
const { kiemAst } = await import(new URL('../packages/exec-python/dist/kiem-ast.js', import.meta.url).href);
const require = createRequire(new URL('../packages/exec-python/package.json', import.meta.url));
const { loadPyodide } = await import(pathToFileURL(require.resolve('pyodide/pyodide.mjs')).href);
const py = await loadPyodide();

// ── TypeScript: cùng nguyên tắc, engine khác ──────────────────────────────
//
// Trước bản vá này, mọi bước `c.lang !== 'python'` bị BỎ QUA HOÀN TOÀN
// (`if (!c || c.lang !== 'python') continue;`) — nghĩa là bài TypeScript
// đầu tiên của khoá (R4.T4.0a) sẽ được viết mà KHÔNG cổng nào thật sự chạy
// thử nó. Đúng chế độ hỏng dự án này coi là kẻ thù: một cổng xanh không đo
// được cái nó nói đang đo — chỉ khác lần này cổng không đo GÌ CẢ, không
// phải đo sai.
const { BoThucThiTypeScript, kiemKieu } = await import(new URL('../packages/exec-typescript/dist/index.js', import.meta.url).href);
const require_ts = createRequire(new URL('../packages/exec-typescript/package.json', import.meta.url));
const TS = require_ts('typescript');
const TS_LIB_DIR = dirname(require_ts.resolve('typescript'));
const doc_lib_ts = (ten) => {
  try {
    return readFileSync(join(TS_LIB_DIR, ten), 'utf-8');
  } catch {
    return undefined;
  }
};
const kiem_kieu_truoc = (ma) => kiemKieu(TS, ma, doc_lib_ts);

const TS_WORKER_URL = new URL('../packages/exec-typescript/node-worker.mjs', import.meta.url);
function tao_cong_ts() {
  const w = new Worker(TS_WORKER_URL);
  let handler = null;
  w.on('message', (m) => handler?.(m));
  w.on('error', () => {});
  return {
    gui: (tin) => w.postMessage(tin),
    khiNhan: (f) => { handler = f; },
    giet: () => { w.terminate(); },
  };
}
// Một engine DÙNG CHUNG cho cả lượt chạy — `BoThucThiTypeScript` tự dựng
// lại worker mới nếu worker cũ bị giết (đã xác nhận qua test riêng của
// package), nên không cần một worker mới cho mỗi lời giải.
const ts_engine = new BoThucThiTypeScript(tao_cong_ts, kiem_kieu_truoc);

/** Chạy một đoạn TypeScript, trả về {ok, xuat, loi} — cùng hình dạng chay(). */
async function chay_ts(ma, ma_kiem_tra) {
  const r = await ts_engine.chay(ma, ma_kiem_tra ? { maKiemTra: ma_kiem_tra } : {});
  return {
    ok: r.ok,
    xuat: r.xuat,
    loi: r.chanDoan[0]?.vanBan ?? (r.ok ? null : 'lỗi không rõ'),
  };
}

// ── Rust: cùng nguyên tắc, engine THỨ BA ──────────────────────────────────
//
// Đơn giản hơn TypeScript: module WASM (`byte_rust.wasm`) tự nó có ngân sách
// nhiên liệu và giới hạn độ sâu BÊN TRONG (xem ADR-001) — không cần Worker
// để chặn vòng lặp vô hạn, vì Rust không nhường quyền cho host.
//
// BẪY VỪA GẶP THẬT lúc vá: `byte_rust.wasm` là mã đã BIÊN DỊCH SẴN, y hệt
// `dist/content` — sửa `crates/byte-rust/src/*.rs` mà không `pnpm run wasm`
// lại thì cổng này chấm bằng một trình thông dịch Rust CŨ, đúng lớp lỗi
// "bản dịch cũ hơn nguồn" đã có sẵn cảnh báo cho `dist/content` ở dưới —
// giờ thêm một bản kiểm tương tự cho wasm.
const WASM_RUST = join(process.cwd(), 'apps/byte/public/wasm/byte_rust.wasm');
{
  const wasm_cu = (() => {
    try {
      return statSync(WASM_RUST).mtimeMs;
    } catch {
      return 0;
    }
  })();
  const rust_moi_nhat = (() => {
    let t = 0;
    const di = (d) => {
      for (const e of readdirSync(d, { withFileTypes: true })) {
        const p = join(d, e.name);
        if (e.isDirectory()) di(p);
        else if (e.name.endsWith('.rs')) t = Math.max(t, statSync(p).mtimeMs);
      }
    };
    di('crates/byte-rust/src');
    return t;
  })();
  if (rust_moi_nhat > wasm_cu) {
    console.error(
      `❌ ${WASM_RUST} cũ hơn crates/byte-rust/src — cổng này sẽ chấm bằng một\n`
        + '   trình thông dịch Rust không còn khớp mã nguồn.\n'
        + '   Biên dịch lại trước: cd apps/byte && pnpm run wasm',
    );
    process.exit(1);
  }
}
const { BoThucThiRust } = await import(new URL('../packages/exec-rust/dist/index.js', import.meta.url).href);
const rust_engine = new BoThucThiRust(async () => readFileSync(WASM_RUST));

/** Chạy một đoạn Rust, trả về {ok, xuat, loi} — cùng hình dạng chay(). */
async function chay_rust(ma, ma_kiem_tra) {
  const r = await rust_engine.chay(ma, ma_kiem_tra ? { maKiemTra: ma_kiem_tra } : {});
  return {
    ok: r.ok,
    xuat: r.xuat,
    loi: r.chanDoan[0]?.vanBan ?? (r.ok ? null : 'lỗi không rõ'),
  };
}

/** Trần số dòng in ra cho MỘT lần chạy.
 *
 *  Cần thật, không phải phòng xa: luật #5 điền `1` vào chỗ trống, và
 *  `while 1:` là vòng lặp vô hạn in ra vô tận. Không có trần này thì bộ dò
 *  nuốt sạch bộ nhớ rồi chết trước khi kịp báo cáo gì.
 *
 *  Ném từ trong `batched` là cách duy nhất dừng được vòng lặp: `runPython`
 *  không nhận timeout, nên phải để chính Python nổ ra ngoài.
 */
const TRAN_DONG = 5000;

// TRAN_DONG chỉ chặn vòng lặp CÓ IN. T3.2 bài `mang-vs-lien-ket-danh-doi-gi`
// lộ ra lớp còn lại: chỗ trống là nguyên thân một `while so_buoc < k:`, điền
// bừa `1` (luật #5 dưới) làm thân vòng thành no-op — `so_buoc` không bao giờ
// tăng, vòng lặp không dừng, và KHÔNG IN GÌ CẢ nên `TRAN_DONG` không chạm
// tới. `HET_HAN_MS` từng khai báo ở đây định làm việc này nhưng chưa bao giờ
// được nối vào đâu — một hạn mức không thật, giống hệt lớp lỗi AST-kind-giả
// đã sửa ở `kiem-ast.ts`. `py.runPython` không nhận timeout (đồng bộ, một
// luồng), nên cách duy nhất chặn được là để CHÍNH PYTHON đếm bước và tự ném,
// đúng mẫu `chay_co_han` đã dùng ở `kiem_dot_bien.mjs`.
const TRAN_BUOC = 300_000;
py.runPython(`
import sys

def _chay_co_han(ma, tran, qua_dong):
    dem = [0]
    def theo_doi(frame, su_kien, gt):
        dem[0] += 1
        if dem[0] > tran:
            raise RuntimeError('vượt hạn mức bước — vòng lặp không dừng')
        if qua_dong():
            raise RuntimeError('vượt hạn mức dòng in — vòng lặp không dừng')
        return theo_doi
    sys.settrace(theo_doi)
    try:
        exec(ma, {'__name__': '__main__'})
    finally:
        sys.settrace(None)
`);
const chay_co_han = py.globals.get('_chay_co_han');

/** Chạy một đoạn Python, trả về {ok, xuat, loi}. */
function chay(ma) {
  const dong = [];
  // QUAN TRỌNG: `batched` KHÔNG BAO GIỜ được ném. Bản trước ném thẳng từ đây
  // khi chạm TRAN_DONG — và một lần chạy thật lộ ra hậu quả: cú ném từ TRONG
  // callback `batched` cắt ngang đúng lúc Pyodide đang xả một lần `write()`
  // ở tầng WASM/emscripten, biến thành `OSError: [Errno 29] I/O error` phía
  // Python thay vì thông điệp ta ném, VÀ để kẹt lại đúng MỘT dòng in dở
  // trong bộ đệm nội bộ của nó — dòng ấy không thuộc `dong` này, mà trôi
  // sang lần gọi `chay()` KẾ TIẾP (bài nào cũng được, không liên quan gì
  // tới bài đang chạy), làm xuất hiện một dòng lạ ở ĐẦU output của bài sau.
  // Cờ boolean này chỉ ĐÁNH DẤU đã chạm trần; việc NÉM chuyển hẳn sang
  // `theo_doi` bên dưới — cùng cơ chế `sys.settrace` đã dùng cho TRAN_BUOC,
  // nổ ra GIỮA HAI LỆNH của Python chứ không phải giữa một lời gọi write()
  // cấp thấp, nên không đụng tới bộ đệm stdio nội bộ.
  let qua_han = false;
  const gom = {
    batched: (s) => {
      dong.push(s);
      if (dong.length >= TRAN_DONG) qua_han = true;
    },
  };
  py.setStdout(gom);
  py.setStderr(gom);
  try {
    chay_co_han(ma, TRAN_BUOC, () => qua_han);
    return { ok: true, xuat: dong.join('\n'), loi: null };
  } catch (e) {
    return { ok: false, xuat: dong.join('\n'), loi: e instanceof Error ? e.message : String(e) };
  }
}

/** Đối chiếu output theo đúng kiểu `match` mà luật chấm khai báo. */
function khop(thuc, luat) {
  const mong = String(luat.expected ?? '');
  switch (luat.match ?? 'contains') {
    case 'exact':
      return thuc === mong;
    case 'trim':
      return thuc.trim() === mong.trim();
    case 'regex':
      return new RegExp(mong).test(thuc);
    case 'json-deep':
      try {
        return JSON.stringify(JSON.parse(thuc)) === JSON.stringify(JSON.parse(mong));
      } catch {
        return false;
      }
    default:
      // `contains` — mặc định. Xem chú thích ở `chuan_hoa_rule` trong
      // content-compiler: `expect:` nêu một dòng, không nêu cả output.
      return thuc.includes(mong.trim());
  }
}

// Cổng này đọc BẢN ĐÃ BIÊN DỊCH, không đọc `.lesson.md`. Nếu nguồn mới hơn
// bản dịch thì mọi thứ nó nói ra là về một bài học không còn tồn tại — và nó
// nói bằng màu xanh.
//
// Tôi tự dính đúng bẫy này: sửa bài rồi chạy thẳng cổng, thấy xanh, tưởng đã
// kiểm. Hai đột biến cố ý bẻ gãy luật chấm cũng xanh nốt. Trong `cong.sh` thì
// không sao vì bước biên dịch chạy ngay trước; chạy tay một mình mới chết.
const nguon_moi_nhat = (() => {
  let t = 0;
  const di = (d) => {
    for (const e of readdirSync(d, { withFileTypes: true })) {
      const p = join(d, e.name);
      if (e.isDirectory()) di(p);
      else if (e.name.endsWith('.lesson.md')) t = Math.max(t, statSync(p).mtimeMs);
    }
  };
  di('content');
  return t;
})();
const dich_cu_nhat = Math.min(
  ...readdirSync(THU_MUC)
    .filter((f) => f.endsWith('.json'))
    .map((f) => statSync(join(THU_MUC, f)).mtimeMs),
);
if (nguon_moi_nhat > dich_cu_nhat) {
  console.error(
    `❌ ${THU_MUC} cũ hơn content/ — cổng này sẽ chấm một bài học không còn tồn tại.\n`
      + '   Biên dịch lại trước:\n'
      + '   node packages/content-compiler/dist/cli.js build content dist/content',
  );
  process.exit(1);
}

const tep = readdirSync(THU_MUC).filter((f) => f.endsWith('.json') && f !== 'index.json');
const hong = [];
let da_chay = 0;

for (const f of tep) {
  const bai = JSON.parse(readFileSync(join(THU_MUC, f), 'utf-8'));
  for (const b of bai.steps) {
    const c = b.code;
    if (!c) continue;
    const la_py = c.lang === 'python';
    const la_ts = c.lang === 'typescript';
    const la_rust = c.lang === 'rust';
    if (!la_py && !la_ts && !la_rust) continue;

    // Chạy một đoạn mã ĐÚNG NGÔN NGỮ của bước này, {ok, xuat, loi}.
    //
    // Route Python (`chay_py`) giữ nguyên 100% đồng bộ, không một chữ
    // `await` nào chạm vào nó — mọi lời gọi dùng ternary tường minh
    // (`la_py ? chay_py(...) : await ...`), KHÔNG gộp qua một hàm `async`
    // dùng chung. Đây từng là nghi phạm sai của một lỗi có thật: một lần
    // chạy full-corpus lộ ra output của bài TRƯỚC trôi sang bài SAU, và ban
    // đầu bị đổ oan cho điểm nhường microtask của `await`. Gỡ hết `await`
    // khỏi route Python KHÔNG sửa được lỗi đó — cô lập bằng debug log mới
    // tìm ra thủ phạm thật: `TRAN_DONG` (xem `chay()` ở trên) từng NÉM một
    // `Error` từ TRONG callback `batched`, cắt ngang đúng lúc Pyodide xả
    // `write()` ở tầng WASM, làm kẹt lại một dòng in dở trôi sang lần gọi
    // KẾ TIẾP — bất kể `await` có mặt hay không. Đã vá tại gốc (`batched`
    // không còn ném). Ternary tường minh ở đây giờ chỉ còn là phòng thủ
    // theo chiều sâu, không phải điều kiện đủ để tránh lỗi đó.
    const chay_py = (ma, ma_test) => chay(ma_test ? `${ma}\n${ma_test}` : ma);
    // Rust và TypeScript đều async (Worker/WASM); gộp chung một nhánh cho
    // hai cái đó là an toàn — chỉ Python mới cần tách riêng như trên.
    const chay_khac_py = (ma, ma_test) => (la_ts ? chay_ts(ma, ma_test) : chay_rust(ma, ma_test));

    // 1. Lời giải tham chiếu phải chạy được.
    //
    // RUST LÀ TRƯỜNG HỢP RIÊNG, đã đo thật: Rust không cho lệnh trần ở
    // ngoài cùng chương trình (BR0103 "chỉ được khai báo ở ngoài cùng") —
    // nên idiom ĐÚNG cho bài Rust có `test` là để `solution` CHỈ chứa
    // `fn`/`struct` (không `fn main`), và `test` cung cấp `fn main` gọi vào
    // đó kèm `assert_eq!`. Với idiom này, "chạy solution MỘT MÌNH" không áp
    // dụng được — nó THẬT SỰ không có cách chạy độc lập, không phải một
    // lỗi. Khi `la_rust && c.test`, gộp bước 1 và bước 2 làm một: chạy
    // solution+test cùng lúc, và ĐÓ là phép kiểm "chạy được".
    let r_solution = null;
    if (c.solution) {
      da_chay++;
      if (la_rust && c.test) {
        r_solution = await chay_khac_py(c.solution, c.test);
        if (!r_solution.ok) {
          hong.push({
            bai: bai.id,
            buoc: b.id,
            loai: 'lời giải (ghép cùng test cung cấp fn main) không chạy được',
            chi_tiet: r_solution.loi,
          });
          continue;
        }
      } else {
        r_solution = la_py ? chay_py(c.solution) : await chay_khac_py(c.solution);
        if (!r_solution.ok) {
          hong.push({ bai: bai.id, buoc: b.id, loai: 'lời giải không chạy được', chi_tiet: r_solution.loi });
          continue;
        }
        // 2. Khối test phải ĐẠT khi chạy trên lời giải. Nếu không thì hoặc
        //    test sai, hoặc lời giải sai — cách nào cũng khiến người học
        //    không bao giờ qua được bài.
        if (c.test) {
          const rt = la_py ? chay_py(c.solution, c.test) : await chay_khac_py(c.solution, c.test);
          if (!rt.ok) {
            hong.push({ bai: bai.id, buoc: b.id, loai: 'test TRƯỢT trên chính lời giải', chi_tiet: rt.loi });
          }
        }
      }
    }

    // 3. Lời giải phải cho ra ĐÚNG output mà bài đã hứa.
    //
    // 31/40 bài Realm 0 chấm bằng tier `output` chứ không bằng assert. Bỏ qua
    // tier này nghĩa là bỏ qua cách chấm chính của học liệu.
    // MỌI luật `output`, không phải luật đầu tiên.
    //
    // Bản cũ dùng `.find`, nên với 46/236 bước khai nhiều hơn một luật output
    // thì luật thứ hai trở đi không bao giờ được kiểm. Bước `ghep-ca-may` của
    // Realm 0 khai hai luật; đổi `gia_mot_to * so_to` thành `+` phá vỡ luật
    // THỨ HAI, mà cổng vẫn xanh — đúng cái hình dạng lỗi mà chính cổng này
    // sinh ra để chặn.
    const luat_out_ds = (b.validation?.rules ?? []).filter((r) => r.tier === 'output');
    const luat_out = luat_out_ds[0];
    const khop_het = (xuat) => luat_out_ds.every((r) => khop(xuat, r));
    if (c.solution && luat_out && r_solution) {
      const sai = luat_out_ds.find((lo) => !khop(r_solution.xuat, lo));
      if (r_solution.ok && sai) {
        hong.push({
          bai: bai.id,
          buoc: b.id,
          loai: 'lời giải KHÔNG ra output mà bài đã hứa',
          chi_tiet: `hứa ${JSON.stringify(sai.expected)}, thật ra ${JSON.stringify(r_solution.xuat)}`,
        });
      }
    }

    // 5. Chỗ trống `___` không được điền bừa mà vẫn qua.
    //
    // Một bài dạy dấu ngoặc, chấm bằng đúng MỘT bộ dữ liệu, thì `if True:`
    // cũng cho ra đúng câu mà bài mong đợi — người học gõ bừa vẫn xanh, và
    // cái bẫy mà bài dựng lên cả trang để nói tới thì không bao giờ sập.
    //
    // Chỉ báo khi câu điền bừa THẬT SỰ qua được; điền bừa mà chương trình nổ
    // thì không sao, đó là hành vi đúng.
    // Thử điền bừa phải chạy ĐỦ CÁC TẦNG, kể cả `static` — CHỈ với Python,
    // vì TypeScript CHƯA có tầng static (xem mục 3b dưới).
    //
    // Bản trước chỉ chạy `tests` và `output`, nên nó báo động GIẢ: người viết
    // T2.3 gặp một bước có chỗ trống là điều kiện `if ___:` — điền `0` vào thì
    // thân `if` không chạy lần nào, danh sách gom được rỗng đúng bằng đáp án,
    // nên hai tầng ấy cho qua. Luật `static` chặn được, nhưng cổng không hỏi
    // tới nó, và người viết phải dựng lại cả bước code cho một lỗi không có
    // thật.
    //
    // Một cổng kêu oan ăn mòn lòng tin đúng như một cổng báo xanh sai: sau vài
    // lần, người ta bắt đầu bỏ qua nó.
    const luat_static_som = (b.validation?.rules ?? []).filter((r) => r.tier === 'static');
    const qua_static = (ma) =>
      !la_py || luat_static_som.every((r) => kiemAst(py, ma, r.requireAst ?? [], r.forbidAst ?? []).dat);

    // Bừa Python dùng literal Python (True/1/0); bừa TypeScript VÀ Rust dùng
    // chung một bộ literal hợp cú pháp ở HẦU HẾT vị trí biểu thức của cả hai
    // (0/'x'/true — `'x'` là char literal hợp lệ trong CẢ HAI ngôn ngữ) —
    // KHÔNG phủ được vị trí chú thích KIỂU (`: ___`), vì đó không phải một
    // biểu thức. Bài có chỗ trống nằm ở vị trí kiểu phải tự kiểm thêm bằng
    // tay lúc viết.
    const cac_bua = la_py ? ['True', '1', '0'] : ['0', "'x'", 'true'];

    if (b.kind === 'code' && c.starter?.includes('___') && (luat_out || c.test)) {
      for (const bua of cac_bua) {
        const thu = c.starter.replaceAll('___', bua);
        const r = la_py ? chay_py(thu, c.test) : await chay_khac_py(thu, c.test);
        const qua = r.ok && (luat_out ? khop_het(r.xuat) : true) && qua_static(thu);
        if (qua) {
          hong.push({
            bai: bai.id,
            buoc: b.id,
            loai: 'điền bừa vẫn QUA bài',
            chi_tiet: `thay \`___\` bằng \`${bua}\` là đạt — cách chấm không phân biệt được đúng với sai`,
          });
          break;
        }
      }
    }

    // 3b. Luật `tier: static` phải ĐẠT trên lời giải và TRƯỢT trên mã khởi đầu.
    //
    // Một luật static đạt trên cả hai thì nó không kiểm gì cả; trượt trên lời
    // giải thì bài không thể qua được. Kiểm cả hai chiều là cách duy nhất biết
    // luật ấy có thật sự phân biệt hay không.
    //
    // CHỈ ÁP DỤNG CHO PYTHON. Cả `TsAstKind` lẫn `RsAstKind` đã khai trong
    // content-schema (discriminated-union, no-any, … / borrow-move,
    // exhaustive-match, …) nhưng KHÔNG file nào trong `packages/exec-typescript`
    // hay `packages/exec-rust` cài nhánh xử lý cho chúng — một bài TS/Rust
    // khai `tier: static` sẽ không có cách nào được kiểm, và IM LẶNG bỏ qua
    // nó đúng là chế độ hỏng đã fix cho `kind` Python lạ (ném lỗi, không trả
    // 0 câm). Ở đây fix tương đương: NÉM LỖI ngay khi gặp, đừng chờ người
    // viết tự phát hiện lúc đọc kỹ.
    const luat_static = (b.validation?.rules ?? []).filter((r) => r.tier === 'static');
    if ((la_ts || la_rust) && luat_static.length > 0) {
      const goi_y = la_ts
        ? 'TsAstKind chưa cài trong packages/exec-typescript'
        : 'RsAstKind chưa cài trong packages/exec-rust (borrow-check của byte-rust rất hẹp, xem ADR-002)';
      console.error(
        `❌ ${bai.id} · bước ${b.id}: khai \`tier: static\` cho bước ${c.lang}, nhưng `
          + `chưa có hạ tầng kiểm (${goi_y}). `
          + `Xoá luật static này, hoặc cài kiemAst cho ${c.lang} trước.`,
      );
      process.exit(1);
    }
    let static_phan_biet = false;
    if (la_py) {
      for (const r of luat_static) {
        const yeu = r.requireAst ?? [];
        const cam = r.forbidAst ?? [];
        if (c.solution) {
          const kq = kiemAst(py, c.solution, yeu, cam);
          if (!kq.dat) {
            hong.push({
              bai: bai.id,
              buoc: b.id,
              loai: `luật static \`${r.id}\` TRƯỢT trên chính lời giải`,
              chi_tiet: kq.loi_cu_phap
                ? `lời giải không parse được: ${kq.loi_cu_phap}`
                : `thiếu ${JSON.stringify(kq.thieu)} · cấm mà vẫn có ${JSON.stringify(kq.cam)}`,
            });
          }
        }
        if (c.starter && c.starter.includes('___')) {
          // Điền bừa vào chỗ trống rồi kiểm: luật static phải chặn được.
          const bua = c.starter.replaceAll('___', 'True');
          const kq = kiemAst(py, bua, yeu, cam);
          if (kq.dat) {
            hong.push({
              bai: bai.id,
              buoc: b.id,
              loai: `luật static \`${r.id}\` cho qua cả đáp án điền bừa`,
              chi_tiet: 'thay `___` bằng `True` vẫn thoả — luật không phân biệt được gì',
            });
          } else {
            // Luật này CHẶN được đáp án điền bừa, tức nó là một cách trượt thật.
            // Mục 4 ngay dưới cần biết điều đó.
            static_phan_biet = true;
          }
        }
      }
    }

    // 4. Cách chấm phải PHÂN BIỆT được đúng với sai.
    //
    // Một bước `code` mà khối test chỉ có `pass` và không có tier `output`
    // thì không có cách nào trượt: người học gõ gì cũng xanh. Bài vẫn qua
    // trình biên dịch (có đủ solution/test/hints) và qua cổng sư phạm (có đủ
    // ::why, đủ ba nấc gợi ý) — nhưng nó không dạy được gì, vì không có phản
    // hồi nào phụ thuộc vào thứ người học viết ra.
    //
    // Tầng `static` CŨNG là một cách trượt — nhưng chỉ khi nó phân biệt được
    // thật, và điều đó mục 3b vừa đo xong bằng cách điền bừa vào chỗ trống
    // (chỉ áp dụng khi có static — Python). Không tính nó thì mấy bài cố ý
    // không in gì ra màn hình (bài dạy gán lại một cái tên, ở chỗ người học
    // còn chưa được biết `print` một biến) không có đường nào hợp lệ để chấm.
    if (b.kind === 'code' && c.solution) {
      // `throw` cho TypeScript, `assert`/`raise` cho Python, `assert!`/
      // `assert_eq!`/`assert_ne!`/`panic!` cho Rust — cùng vai trò: khối
      // test tự nổ khi sai. `\bassert` (không có `\b` sau) khớp cả
      // `assert_eq!`/`assert_ne!`, vì `_` là ký tự "chữ" nên `\bassert\b`
      // không khớp qua dấu gạch dưới.
      const co_assert = c.test ? /\bassert|\braise\b|\bthrow\b|\bpanic!/.test(c.test) : false;
      const co_output = luat_out && String(luat_out.expected ?? '').trim() !== '';
      if (!co_assert && !co_output && !static_phan_biet) {
        hong.push({
          bai: bai.id,
          buoc: b.id,
          loai: 'không có cách nào TRƯỢT bài này',
          chi_tiet:
            'khối test không có assert/throw, không có tier output, và không luật static nào'
            + ' chặn nổi đáp án điền bừa — gõ gì cũng xanh',
        });
      }
    }

    // 4. Mã `starter` KHÔNG được vô tình đã đạt sẵn — nếu đạt thì bài không
    //    yêu cầu người học làm gì cả.
    if (c.starter && c.solution && !c.starter.includes('___')) {
      // Rust + có test: cùng lý do ở mục 1 — `starter` một mình (không
      // `fn main`) không chạy được, nên phải ghép với `test` NGAY TỪ ĐẦU,
      // không chạy "một mình" trước rồi mới ghép sau.
      const rs = la_rust && c.test
        ? await chay_khac_py(c.starter, c.test)
        : la_py ? chay_py(c.starter) : await chay_khac_py(c.starter);
      const dat_san = luat_out
        ? rs.ok && khop_het(rs.xuat)
        : c.test
          ? (la_rust ? rs : la_py ? chay_py(c.starter, c.test) : await chay_khac_py(c.starter, c.test)).ok
          : false;
      if (dat_san) {
        hong.push({
          bai: bai.id,
          buoc: b.id,
          loai: 'mã khởi đầu đã ĐẠT sẵn',
          chi_tiet: 'người học không phải làm gì mà bài vẫn xanh',
        });
      }
    }
  }
}

console.log(`Đã chạy ${da_chay} lời giải trong ${tep.length} bài.`);
if (hong.length === 0) {
  console.log('✅ Mọi lời giải chạy được và mọi khối test đều đạt.');
  process.exit(0);
}
console.log(`❌ ${hong.length} vấn đề:\n`);
for (const h of hong) {
  console.log(`  ✗ ${h.bai} · bước ${h.buoc}`);
  console.log(`     ${h.loai}`);
  if (h.chi_tiet) console.log(`     ${String(h.chi_tiet).split('\n').slice(-3).join(' | ').slice(0, 240)}`);
}
process.exit(1);
