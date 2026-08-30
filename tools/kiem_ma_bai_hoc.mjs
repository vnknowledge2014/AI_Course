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
import { join } from 'node:path';
import { createRequire } from 'node:module';
import { pathToFileURL } from 'node:url';

const THU_MUC = process.argv[2] ?? 'dist/content';
const HET_HAN_MS = 10_000;

// `pyodide` là phụ thuộc của `packages/exec-python`, không của thư mục gốc —
// nên phải phân giải qua đó thay vì import trần.
const { kiemAst } = await import(new URL('../packages/exec-python/dist/kiem-ast.js', import.meta.url).href);
const require = createRequire(new URL('../packages/exec-python/package.json', import.meta.url));
const { loadPyodide } = await import(pathToFileURL(require.resolve('pyodide/pyodide.mjs')).href);
const py = await loadPyodide();

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

/** Chạy một đoạn Python, trả về {ok, xuat, loi}. */
function chay(ma) {
  const dong = [];
  const gom = {
    batched: (s) => {
      if (dong.length >= TRAN_DONG) throw new Error('VUOT_TRAN_DONG');
      dong.push(s);
    },
  };
  py.setStdout(gom);
  py.setStderr(gom);
  // Pyodide in nguyên vết stack ra `console.error` khi cú ném từ `batched`
  // đi ngược qua tầng WASM của nó. Cú ném ấy là CỐ Ý (xem TRAN_DONG), nên
  // tắt tiếng trong đúng khoảng này thay vì để nó lấp mất báo cáo thật.
  const loi_goc = console.error;
  console.error = () => {};
  try {
    py.runPython(ma);
    return { ok: true, xuat: dong.join('\n'), loi: null };
  } catch (e) {
    return { ok: false, xuat: dong.join('\n'), loi: e instanceof Error ? e.message : String(e) };
  } finally {
    console.error = loi_goc;
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
    if (!c || c.lang !== 'python') continue;

    // 1. Lời giải tham chiếu phải chạy được.
    if (c.solution) {
      da_chay++;
      const r = chay(c.solution);
      if (!r.ok) {
        hong.push({ bai: bai.id, buoc: b.id, loai: 'lời giải không chạy được', chi_tiet: r.loi });
        continue;
      }
      // 2. Khối test phải ĐẠT khi chạy trên lời giải. Nếu không thì hoặc test
      //    sai, hoặc lời giải sai — cách nào cũng khiến người học không bao
      //    giờ qua được bài.
      if (c.test) {
        const rt = chay(`${c.solution}\n${c.test}`);
        if (!rt.ok) {
          hong.push({ bai: bai.id, buoc: b.id, loai: 'test TRƯỢT trên chính lời giải', chi_tiet: rt.loi });
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
    if (c.solution && luat_out) {
      const r = chay(c.solution);
      const sai = luat_out_ds.find((lo) => !khop(r.xuat, lo));
      if (r.ok && sai) {
        hong.push({
          bai: bai.id,
          buoc: b.id,
          loai: 'lời giải KHÔNG ra output mà bài đã hứa',
          chi_tiet: `hứa ${JSON.stringify(sai.expected)}, thật ra ${JSON.stringify(r.xuat)}`,
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
    // Thử điền bừa phải chạy ĐỦ CÁC TẦNG, kể cả `static`.
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
      luat_static_som.every((r) => kiemAst(py, ma, r.requireAst ?? [], r.forbidAst ?? []).dat);

    if (b.kind === 'code' && c.starter?.includes('___') && (luat_out || c.test)) {
      for (const bua of ['True', '1', '0']) {
        const thu = c.starter.replaceAll('___', bua);
        const r = chay(c.test ? `${thu}\n${c.test}` : thu);
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
    const luat_static = (b.validation?.rules ?? []).filter((r) => r.tier === 'static');
    let static_phan_biet = false;
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

    // 4. Cách chấm phải PHÂN BIỆT được đúng với sai.
    //
    // Một bước `code` mà khối test chỉ có `pass` và không có tier `output`
    // thì không có cách nào trượt: người học gõ gì cũng xanh. Bài vẫn qua
    // trình biên dịch (có đủ solution/test/hints) và qua cổng sư phạm (có đủ
    // ::why, đủ ba nấc gợi ý) — nhưng nó không dạy được gì, vì không có phản
    // hồi nào phụ thuộc vào thứ người học viết ra.
    //
    // Tầng `static` CŨNG là một cách trượt — nhưng chỉ khi nó phân biệt được
    // thật, và điều đó mục 3b vừa đo xong bằng cách điền bừa vào chỗ trống.
    // Không tính nó thì mấy bài cố ý không in gì ra màn hình (bài dạy gán lại
    // một cái tên, ở chỗ người học còn chưa được biết `print` một biến) không
    // có đường nào hợp lệ để chấm.
    if (b.kind === 'code' && c.solution) {
      const co_assert = c.test ? /\bassert\b|\braise\b/.test(c.test) : false;
      const co_output = luat_out && String(luat_out.expected ?? '').trim() !== '';
      if (!co_assert && !co_output && !static_phan_biet) {
        hong.push({
          bai: bai.id,
          buoc: b.id,
          loai: 'không có cách nào TRƯỢT bài này',
          chi_tiet:
            'khối test không có assert, không có tier output, và không luật static nào'
            + ' chặn nổi đáp án điền bừa — gõ gì cũng xanh',
        });
      }
    }

    // 4. Mã `starter` KHÔNG được vô tình đã đạt sẵn — nếu đạt thì bài không
    //    yêu cầu người học làm gì cả.
    if (c.starter && c.solution && !c.starter.includes('___')) {
      const rs = chay(c.starter);
      const dat_san = luat_out
        ? rs.ok && khop_het(rs.xuat)
        : c.test
          ? chay(`${c.starter}\n${c.test}`).ok
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
