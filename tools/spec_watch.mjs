#!/usr/bin/env node
/**
 * Spec-watch — đối chiếu bề mặt MCP/A2A mà khoá học GHIM (packages/mcp-kit/
 * src/spec-2026-06.ts) với spec upstream THẬT.
 *
 * MASTERPLAN §9 ra lệnh: "Conformance test chạy CI hàng tuần; test đỏ → sửa 1
 * file, không phải 18 lesson." Script này là cổng đó: chạy trong
 * .github/workflows/spec-watch.yml mỗi tuần (không nằm trong cong.sh — trôi
 * spec là chuyện BẢO TRÌ theo lịch, không phải cổng merge).
 *
 * Hai chiều kiểm:
 *  - GHIM ⊆ UPSTREAM: những gì khoá học dạy (method, field, trạng thái) phải
 *    còn tồn tại upstream. Thiếu → drift.
 *  - UPSTREAM ⊆ GHIM (chỉ với tập bài tuyên bố liệt kê hết — TaskState của
 *    A2A): upstream thêm trạng thái mới → drift, vì `laA2aTaskKetThuc` tuyên
 *    bố biết hết trạng thái kết thúc.
 *
 * Drift không tự sửa được theo cơ học: đổi `initialize` thành `discover` là
 * quyết định NỘI DUNG (bài T9.6-02 dạy trên ngữ nghĩa "bắt buộc" mà upstream
 * 2026-07-28 đã nới thành tuỳ chọn). Vì thế drift được quản bằng baseline
 * `tools/spec-watch-bo-qua.json` — MỖI mục phải kèm lý do, giống luật của
 * `content/curriculum/dot-bien-bo-qua.yaml`:
 *
 *  - drift MỚI không có trong baseline        → ĐỎ
 *  - mục baseline không còn khớp drift nào    → ĐỎ (ép dọn danh sách)
 *  - mục baseline thiếu `lyDo`                → ĐỎ
 *
 * Exit code: 0 xanh · 1 có drift chưa được khai · 2 không đo được (mạng/parse).
 */

import { readFileSync } from 'node:fs';

const MCP_API_SCHEMAS =
  'https://api.github.com/repos/modelcontextprotocol/specification/contents/schema?ref=main';
const MCP_RAW = (phienBan) =>
  `https://raw.githubusercontent.com/modelcontextprotocol/specification/main/schema/${phienBan}/schema.json`;
const A2A_PROTO =
  'https://raw.githubusercontent.com/a2aproject/A2A/main/specification/a2a.proto';

const UA = { 'User-Agent': 'byte-spec-watch', Accept: 'application/vnd.github+json' };

/** Thứ khoá học ghim dạy — phải khớp với spec-2026-06.ts và các bài T9.6. */
const MCP_METHOD_GHIM = ['initialize', 'notifications/initialized', 'tools/list', 'tools/call'];
const MCP_FIELD_GHIM = {
  CallToolResult: ['isError'],
  Tool: ['name', 'inputSchema'],
};
const A2A_TRANG_THAI_GHIM = [
  'TASK_STATE_SUBMITTED',
  'TASK_STATE_WORKING',
  'TASK_STATE_COMPLETED',
  'TASK_STATE_FAILED',
  'TASK_STATE_CANCELED',
  'TASK_STATE_INPUT_REQUIRED',
];
const A2A_FIELD_GHIM = {
  AgentCard: ['name', 'description', 'url', 'capabilities', 'skills'],
  AgentSkill: ['id', 'name', 'description'],
};

async function tai(url, laJson = true) {
  const r = await fetch(url, { headers: UA, redirect: 'follow' });
  if (!r.ok) throw new Error(`HTTP ${r.status} khi tải ${url}`);
  return laJson ? r.json() : r.text();
}

/** Gom mọi `"const"` của property `method` ở mọi độ sâu trong schema. */
function gomMethodConst(nut, ra = new Set()) {
  if (Array.isArray(nut)) {
    for (const x of nut) gomMethodConst(x, ra);
  } else if (nut && typeof nut === 'object') {
    const m = nut.properties?.method?.const;
    if (typeof m === 'string') ra.add(m);
    for (const v of Object.values(nut)) gomMethodConst(v, ra);
  }
  return ra;
}

/** Tách thân của một khai báo `enum X { ... }` hoặc `message X { ... }` trong proto. */
function thanKhaiBao(proto, loai, ten) {
  const dau = proto.search(new RegExp(`${loai}\\s+${ten}\\s*\\{`));
  if (dau < 0) return null;
  const i = proto.indexOf('{', dau);
  let doSau = 0;
  for (let j = i; j < proto.length; j++) {
    if (proto[j] === '{') doSau++;
    else if (proto[j] === '}') {
      doSau--;
      if (doSau === 0) return proto.slice(i + 1, j);
    }
  }
  return null;
}

/** Tên field trong một thân message proto (bỏ comment và annotation). */
function tenFieldProto(than) {
  if (than === null) return null;
  const ra = new Set();
  for (const dong of than.split('\n')) {
    const s = dong.replace(/\/\/.*$/, '').trim();
    const khop = /^(?:repeated\s+|optional\s+|map\s*<[^>]+>\s+)?[\w.]+\s+(\w+)\s*=\s*\d+/.exec(s);
    if (khop) ra.add(khop[1]);
  }
  return ra;
}

function docBaseline() {
  const duong = new URL('./spec-watch-bo-qua.json', import.meta.url);
  const tep = JSON.parse(readFileSync(duong, 'utf8'));
  if (!Array.isArray(tep.boQua)) throw new Error('spec-watch-bo-qua.json: thiếu mảng "boQua"');
  return tep.boQua;
}

async function main() {
  // ── MCP: chọn phiên bản mới nhất ────────────────────────────────────────
  const mucLuc = await tai(MCP_API_SCHEMAS);
  const phienBans = mucLuc
    .map((x) => x.name)
    .filter((n) => /^\d{4}-\d{2}-\d{2}$/.test(n))
    .sort();
  if (phienBans.length === 0) throw new Error('không tìm thấy phiên bản schema MCP nào');
  const moiNhat = phienBans[phienBans.length - 1];
  const schema = await tai(MCP_RAW(moiNhat));

  const methodUpstream = gomMethodConst(schema);
  const defs = schema.$defs ?? {};

  const drift = [];
  const thongTin = [`MCP upstream mới nhất: ${moiNhat} (${Object.keys(defs).length} definition)`];

  for (const m of MCP_METHOD_GHIM) {
    if (!methodUpstream.has(m)) drift.push(`mcp:method:${m}`);
  }
  for (const [def, cacField] of Object.entries(MCP_FIELD_GHIM)) {
    const props = defs[def]?.properties;
    if (!props) {
      drift.push(`mcp:def:${def}`);
      continue;
    }
    for (const f of cacField) {
      if (!(f in props)) drift.push(`mcp:field:${def}.${f}`);
    }
  }

  // ── A2A: proto là nguồn chuẩn (a2a.json chỉ là build artifact, không commit) ──
  const proto = await tai(A2A_PROTO, false);

  const thanState = thanKhaiBao(proto, 'enum', 'TaskState');
  if (!thanState) throw new Error('không tách được enum TaskState trong a2a.proto');
  const stateUpstream = new Set(
    [...thanState.matchAll(/\b(TASK_STATE_[A-Z_]+)\s*=\s*\d+/g)].map((m) => m[1]),
  );
  thongTin.push(`A2A TaskState upstream: ${[...stateUpstream].join(', ')}`);
  const ghimTap = new Set(A2A_TRANG_THAI_GHIM);
  for (const s of stateUpstream) {
    // Hai chiều một lượt: ghim mà upstream bỏ, hoặc upstream thêm mà ghim chưa
    // dạy (khoá học tuyên bố liệt kê HẾT trạng thái kết thúc).
    // TASK_STATE_UNSPECIFIED là giá trị rỗng bắt buộc của proto3, không phải
    // một trạng thái task thật.
    if (!ghimTap.has(s) && s !== 'TASK_STATE_UNSPECIFIED') drift.push(`a2a:task-state:${s}`);
  }
  for (const s of ghimTap) {
    if (!stateUpstream.has(s)) drift.push(`a2a:task-state:${s}`);
  }

  for (const [ten, cacField] of Object.entries(A2A_FIELD_GHIM)) {
    const cacTen = tenFieldProto(thanKhaiBao(proto, 'message', ten));
    if (cacTen === null) {
      drift.push(`a2a:message:${ten}`);
      continue;
    }
    for (const f of cacField) {
      if (!cacTen.has(f)) drift.push(`a2a:field:${ten}.${f}`);
    }
  }

  // ── Đối chiếu baseline ───────────────────────────────────────────────────
  const baseline = docBaseline();
  const daKhai = new Map(baseline.map((b) => [b.doiTuong, b]));

  let soDo = 0;
  for (const d of [...new Set(drift)].sort()) {
    const khai = daKhai.get(d);
    if (!khai) {
      console.log(`✗ DRIFT MỚI chưa khai: ${d}`);
      soDo++;
    } else if (!khai.lyDo || !String(khai.lyDo).trim()) {
      console.log(`✗ DRIFT khai thiếu lý do: ${d}`);
      soDo++;
    } else {
      console.log(`… drift đã khai (có lý do): ${d}`);
    }
  }
  const daThay = new Set(drift);
  for (const b of baseline) {
    if (!daThay.has(b.doiTuong)) {
      console.log(`✗ Miễn trừ HẾT HIỆU LỰC (upstream khớp lại rồi) — xoá khỏi baseline: ${b.doiTuong}`);
      soDo++;
    }
  }

  console.log('');
  for (const t of thongTin) console.log(`ℹ ${t}`);

  if (soDo > 0) {
    console.log(`
ĐỎ: ${soDo} vấn đề. Sửa ở packages/mcp-kit/src/spec-2026-06.ts (đúng triết lý
"sửa 1 file, không phải 18 lesson") hoặc khai kèm LÝ DO vào
tools/spec-watch-bo-qua.json.`);
    process.exit(1);
  }
  console.log(`\nXANH: ${[...new Set(drift)].length} drift, tất cả đã khai kèm lý do trong baseline.`);
}

main().catch((loi) => {
  console.error(`KHÔNG ĐO ĐƯỢC (mạng/parse), coi như đỏ để không âm thầm lọt: ${loi.message}`);
  process.exit(2);
});
