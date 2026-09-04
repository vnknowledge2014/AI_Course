#!/usr/bin/env node
/**
 * Tải wheel numpy (WASM) khớp ĐÚNG bản Pyodide đã ghim trong package.json,
 * xác thực bằng sha256 lấy từ CHÍNH `pyodide-lock.json` của gói `pyodide` đó
 * — không hardcode version/tên file/hash, để luôn tự khớp bản đã ghim dù
 * pyodide có nâng cấp version sau này.
 *
 * VÌ SAO CẦN FILE NÀY: Realm 8 (AI · Generative AI · RAG, xem MASTERPLAN.md
 * §9 "R8") dạy machine learning/mạng nơ-ron/transformer bằng "numpy thuần
 * trong Pyodide (KHÔNG torch)". Nhưng gói npm `pyodide` (script `pyodide` ở
 * package.json này) chỉ mang theo CPython lõi + stdlib — KHÔNG mang numpy.
 * numpy phải tải riêng từ CDN chính thức của dự án Pyodide, xác thực
 * sha256, rồi đặt CẠNH các file runtime khác trong `public/pyodide/` — nhờ
 * vậy `py.loadPackage('numpy')` tự tìm thấy nó qua `indexURL` (giống hệt
 * cách nó tìm `pyodide.asm.wasm`), KHÔNG cần sửa bất kỳ code runtime nào
 * khác. Cơ chế này đã được verify THẬT (không suy luận) — xem lịch sử
 * phiên làm việc: tải wheel, đặt cạnh `pyodide-lock.json` sẵn có, gọi
 * `loadPyodide({indexURL}).loadPackage('numpy')` bằng TÊN GÓI trần (không
 * URL thủ công) — numpy nạp đúng, `np.linalg.det`/`@`/`np.random` chạy
 * đúng kết quả.
 */
import { createRequire } from 'node:module';
import { readFileSync, existsSync, mkdirSync, writeFileSync } from 'node:fs';
import { createHash } from 'node:crypto';

const require = createRequire(import.meta.url);
const pyodidePkgDir = require.resolve('pyodide/package.json').replace(/package\.json$/, '');
const pyodidePkg = JSON.parse(readFileSync(pyodidePkgDir + 'package.json', 'utf8'));
const lock = JSON.parse(readFileSync(pyodidePkgDir + 'pyodide-lock.json', 'utf8'));
const numpyEntry = lock.packages && lock.packages.numpy;
if (!numpyEntry) {
  throw new Error('pyodide-lock.json không có gói numpy — bản pyodide đã đổi cấu trúc lock, cần xem lại tay.');
}

const version = pyodidePkg.version; // vd "0.29.4" — dùng ĐÚNG bản đã cài, không hardcode.
const fileName = numpyEntry.file_name;
const expectedSha256 = numpyEntry.sha256;
const outDir = 'public/pyodide';
const outPath = `${outDir}/${fileName}`;

mkdirSync(outDir, { recursive: true });

function sha256(buf) {
  return createHash('sha256').update(buf).digest('hex');
}

if (existsSync(outPath)) {
  const cur = sha256(readFileSync(outPath));
  if (cur === expectedSha256) {
    console.log(`numpy ${numpyEntry.version} đã có sẵn ở ${outPath}, sha256 khớp — bỏ qua tải lại.`);
    process.exit(0);
  }
  console.log(`numpy đã có ở ${outPath} nhưng sha256 lệch (${cur} != ${expectedSha256}) — tải lại.`);
}

const url = `https://cdn.jsdelivr.net/pyodide/v${version}/full/${fileName}`;
console.log(`Tải ${url} ...`);
const res = await fetch(url);
if (!res.ok) {
  throw new Error(`Tải numpy thất bại: HTTP ${res.status} ${res.statusText} — ${url}`);
}
const buf = Buffer.from(await res.arrayBuffer());
const gotSha256 = sha256(buf);
if (gotSha256 !== expectedSha256) {
  throw new Error(
    `sha256 KHÔNG khớp sau khi tải — nghi ngờ file bị hỏng hoặc CDN trả nhầm bản.\n` +
      `  mong đợi: ${expectedSha256}\n  thực tế:  ${gotSha256}\n  url: ${url}`,
  );
}
writeFileSync(outPath, buf);
console.log(
  `OK — numpy ${numpyEntry.version} (${fileName}, ${(buf.length / 1024 / 1024).toFixed(2)} MB) → ${outPath}, sha256 xác thực khớp pyodide-lock.json.`,
);
