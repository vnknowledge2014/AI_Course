---
id: ky-nghe-ung-dung-ai.giao-thuc-mcp-a2a.boss-agent-a-goi-agent-b-qua-mcp
title: "BOSS q9.6a — Agent A gọi Agent B qua MCP, chuẩn hoá vs không chuẩn hoá"
summary: "Kịch bản đầy đủ ráp NĂM bài trước: chayKichBanChuanHoa(agentBCard,'tinh_tong',{a:3,b:4}) kiểm AgentCard (bài 4, kiemAgentCoHoTroKyNang) → tạo task 'submitted' → 'working' (bài 5, chuyenTrangThaiTask) → agent B xử lý qua MCP: xác nhận capability 'tools' (bài 2, rút gọn) → validate tham số theo inputSchema của tool 'cong' (bài 3, kiemTraThamSoTheoSchema) → gọi tool thật → task 'completed' kèm ketQua=7, tool thật CHẠY ĐÚNG 1 lần (demGoiToolThat.soLan=1). Hai lần gọi chuẩn hoá KHÁC minh hoạ đúng lớp lỗi mỗi bước chặn: gọi kỹ năng 'dich_thuat' (agent B KHÔNG công bố) → task 'failed' NGAY, tool thật KHÔNG chạy (soLan vẫn 1); gọi 'tinh_tong' nhưng thiếu tham số b → task 'failed' do schema chặn, tool thật CŨNG không chạy (soLan vẫn 1). Đối chiếu luồng KHÔNG chuẩn hoá (bỏ qua CẢ BA gate) trên CÙNG hai đầu vào lỗi: gọi 'dich_thuat' vẫn ÂM THẦM chạy phép 'cong' và trả về 7 — SAI HOÀN TOÀN loại yêu cầu, không lỗi nào báo; gọi thiếu tham số b vẫn THỰC THI tool thật (soLan tăng lên 2 rồi 3 qua hai lần gọi không chuẩn hoá) và in ra null (NaN qua JSON.stringify) thay vì báo lỗi rõ ràng. Đóng q9.6a tại 6/6 (KHÔNG đóng cả track T9.6)."
locale: vi
track: ky-nghe-ung-dung-ai
module: giao-thuc-mcp-a2a
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [kna.boss-agent-a-goi-agent-b-qua-mcp]
requires: [kna.a2a-vong-doi-task-sau-trang-thai]
concepts: [kna.boss-agent-a-goi-agent-b-qua-mcp]
gradingMatrix:
  web-chrome: [run, tests, output]
  web-firefox: [run, tests, output]
  macos: [run, tests, output]
  windows: [run, tests, output]
  linux: [run, tests, output]
  android: [run, tests, output]
  ios: [run, tests, output]
provenance:
  authoredBy: llm-assisted
  reviewed: false
---

::::byte{trigger=enter mood=happy pose=jump}
Năm bài: JSON-RPC (`1`), capability negotiation (`2`), tool schema (`3`),
Agent Card (`4`), vòng đời task (`5`). BOSS này ráp NGUYÊN VĂN cả năm
thành MỘT kịch bản: agent A muốn agent B tính `3 + 4`. A kiểm Card của B
TRƯỚC, gửi task qua vòng đời chuẩn, B xử lý bằng CHÍNH MCP đã học — rồi
so với một luồng bỏ qua HẾT ba cổng đó, trên CÙNG hai đầu vào lỗi.
::::

::::explain{#rap_lai_duong_hanh_phuc}
Agent B (`agentBCard`) công bố ĐÚNG một kỹ năng — `"tinh_tong"` — và
đứng sau kỹ năng đó LÀ một tool MCP thật: `dinhNghiaToolCong`, đòi hai
tham số `a`/`b` kiểu `number`. `chayKichBanChuanHoa` nối BA cổng đã học
thành một đường ống: kiểm Agent Card (bài `4`) → vòng đời task (bài `5`)
→ agent B tự xử lý bằng capability check (bài `2`, rút gọn) + validate
schema (bài `3`) trước khi chạm tool thật:

```typescript title=readonly
type A2aAgentSkill = { id: string; name: string; description: string };
type A2aAgentCard = {
  name: string;
  description: string;
  url: string;
  capabilities: { streaming?: boolean; pushNotifications?: boolean };
  skills: A2aAgentSkill[];
};

type A2aTaskState = "submitted" | "working" | "input-required" | "completed" | "canceled" | "failed";
type A2aTask = { id: string; state: A2aTaskState; ketQua?: number };

type McpJsonSchema = {
  type: "object";
  properties: Record<string, { type: "string" | "number" | "boolean" }>;
  required?: string[];
};
type McpToolDefinition = { name: string; description: string; inputSchema: McpJsonSchema };
type McpCapabilities = { tools?: Record<string, never> };

const CHUYEN_HOP_LE: Record<A2aTaskState, A2aTaskState[]> = {
  submitted: ["working", "canceled"],
  working: ["input-required", "completed", "failed", "canceled"],
  "input-required": ["working", "canceled"],
  completed: [],
  canceled: [],
  failed: [],
};

function chuyenTrangThaiTask(hienTai: A2aTaskState, moi: A2aTaskState): A2aTaskState {
  if (!CHUYEN_HOP_LE[hienTai].includes(moi)) {
    throw new Error(`chuyen trang thai khong hop le: ${hienTai} -> ${moi}`);
  }
  return moi;
}

function capNhatTrangThai(task: A2aTask, moi: A2aTaskState): A2aTask {
  return { ...task, state: chuyenTrangThaiTask(task.state, moi) };
}

function kiemAgentCoHoTroKyNang(card: A2aAgentCard, tenKyNang: string): boolean {
  return card.skills.some((sk) => sk.id === tenKyNang);
}

function kiemTraThamSoTheoSchema(schema: McpJsonSchema, thamSo: Record<string, unknown>): boolean {
  for (const truong of schema.required ?? []) {
    if (!(truong in thamSo)) return false;
  }
  for (const ten of Object.keys(schema.properties)) {
    const dinhNghia = schema.properties[ten];
    if (dinhNghia === undefined) continue;
    if (ten in thamSo && typeof thamSo[ten] !== dinhNghia.type) return false;
  }
  return true;
}

const agentBCard: A2aAgentCard = {
  name: "agent-tinh-toan",
  description: "Agent thuc hien phep tinh so hoc qua MCP",
  url: "https://vi-du.test/agent-tinh-toan",
  capabilities: {},
  skills: [{ id: "tinh_tong", name: "Tinh tong hai so", description: "Cong hai so nguyen" }],
};

const dinhNghiaToolCong: McpToolDefinition = {
  name: "cong",
  description: "Cong hai so",
  inputSchema: {
    type: "object",
    properties: { a: { type: "number" }, b: { type: "number" } },
    required: ["a", "b"],
  },
};

const demGoiToolThat = { soLan: 0 };
function thucThiToolCongThat(a: number, b: number): number {
  demGoiToolThat.soLan++;
  return a + b;
}

function agentBXuLyTaskChuanHoa(
  thamSo: Record<string, unknown>,
): { thanhCong: true; ketQua: number } | { thanhCong: false; loi: string } {
  const serverCapabilities: McpCapabilities = { tools: {} };
  if (serverCapabilities.tools === undefined) {
    return { thanhCong: false, loi: "server khong ho tro tools" };
  }
  if (!kiemTraThamSoTheoSchema(dinhNghiaToolCong.inputSchema, thamSo)) {
    return { thanhCong: false, loi: "tham so khong hop schema cua tool 'cong'" };
  }
  const ketQua = thucThiToolCongThat(thamSo["a"] as number, thamSo["b"] as number);
  return { thanhCong: true, ketQua };
}

function chayKichBanChuanHoa(
  card: A2aAgentCard,
  tenKyNang: string,
  thamSo: Record<string, unknown>,
): A2aTask {
  if (!kiemAgentCoHoTroKyNang(card, tenKyNang)) {
    return { id: "task-1", state: "failed" };
  }
  let task: A2aTask = { id: "task-1", state: "submitted" };
  task = capNhatTrangThai(task, "working");
  const ketQuaXuLy = agentBXuLyTaskChuanHoa(thamSo);
  if (!ketQuaXuLy.thanhCong) {
    return capNhatTrangThai(task, "failed");
  }
  task = capNhatTrangThai(task, "completed");
  return { ...task, ketQua: ketQuaXuLy.ketQua };
}

const kb1 = chayKichBanChuanHoa(agentBCard, "tinh_tong", { a: 3, b: 4 });
console.log(JSON.stringify(kb1), demGoiToolThat.soLan);
```

```text title=readonly
{"id":"task-1","state":"completed","ketQua":7} 1
```

Đường hành phúc: task đi ĐÚNG `submitted → working → completed`, `ketQua`
LÀ `7`, VÀ tool thật (`thucThiToolCongThat`) chạy ĐÚNG một lần
(`demGoiToolThat.soLan === 1`).
::::

::::example{#doi_chieu_hai_kich_ban_loi}
Thêm HAI lời gọi chuẩn hoá SAI (sai kỹ năng, thiếu tham số) VÀ một luồng
KHÔNG chuẩn hoá bỏ qua CẢ BA cổng — chạy TRÊN CÙNG một `agentBCard` VÀ
CÙNG hai đầu vào lỗi đó, để thấy CHÍNH XÁC lớp lỗi nào biến mất khi bỏ
qua từng cổng:

```typescript title=readonly
type A2aAgentSkill = { id: string; name: string; description: string };
type A2aAgentCard = {
  name: string;
  description: string;
  url: string;
  capabilities: { streaming?: boolean; pushNotifications?: boolean };
  skills: A2aAgentSkill[];
};

type A2aTaskState = "submitted" | "working" | "input-required" | "completed" | "canceled" | "failed";
type A2aTask = { id: string; state: A2aTaskState; ketQua?: number };

type McpJsonSchema = {
  type: "object";
  properties: Record<string, { type: "string" | "number" | "boolean" }>;
  required?: string[];
};
type McpToolDefinition = { name: string; description: string; inputSchema: McpJsonSchema };
type McpCapabilities = { tools?: Record<string, never> };

const CHUYEN_HOP_LE: Record<A2aTaskState, A2aTaskState[]> = {
  submitted: ["working", "canceled"],
  working: ["input-required", "completed", "failed", "canceled"],
  "input-required": ["working", "canceled"],
  completed: [],
  canceled: [],
  failed: [],
};

function chuyenTrangThaiTask(hienTai: A2aTaskState, moi: A2aTaskState): A2aTaskState {
  if (!CHUYEN_HOP_LE[hienTai].includes(moi)) {
    throw new Error(`chuyen trang thai khong hop le: ${hienTai} -> ${moi}`);
  }
  return moi;
}

function capNhatTrangThai(task: A2aTask, moi: A2aTaskState): A2aTask {
  return { ...task, state: chuyenTrangThaiTask(task.state, moi) };
}

function kiemAgentCoHoTroKyNang(card: A2aAgentCard, tenKyNang: string): boolean {
  return card.skills.some((sk) => sk.id === tenKyNang);
}

function kiemTraThamSoTheoSchema(schema: McpJsonSchema, thamSo: Record<string, unknown>): boolean {
  for (const truong of schema.required ?? []) {
    if (!(truong in thamSo)) return false;
  }
  for (const ten of Object.keys(schema.properties)) {
    const dinhNghia = schema.properties[ten];
    if (dinhNghia === undefined) continue;
    if (ten in thamSo && typeof thamSo[ten] !== dinhNghia.type) return false;
  }
  return true;
}

const agentBCard: A2aAgentCard = {
  name: "agent-tinh-toan",
  description: "Agent thuc hien phep tinh so hoc qua MCP",
  url: "https://vi-du.test/agent-tinh-toan",
  capabilities: {},
  skills: [{ id: "tinh_tong", name: "Tinh tong hai so", description: "Cong hai so nguyen" }],
};

const dinhNghiaToolCong: McpToolDefinition = {
  name: "cong",
  description: "Cong hai so",
  inputSchema: {
    type: "object",
    properties: { a: { type: "number" }, b: { type: "number" } },
    required: ["a", "b"],
  },
};

const demGoiToolThat = { soLan: 0 };
function thucThiToolCongThat(a: number, b: number): number {
  demGoiToolThat.soLan++;
  return a + b;
}

function agentBXuLyTaskChuanHoa(
  thamSo: Record<string, unknown>,
): { thanhCong: true; ketQua: number } | { thanhCong: false; loi: string } {
  const serverCapabilities: McpCapabilities = { tools: {} };
  if (serverCapabilities.tools === undefined) {
    return { thanhCong: false, loi: "server khong ho tro tools" };
  }
  if (!kiemTraThamSoTheoSchema(dinhNghiaToolCong.inputSchema, thamSo)) {
    return { thanhCong: false, loi: "tham so khong hop schema cua tool 'cong'" };
  }
  const ketQua = thucThiToolCongThat(thamSo["a"] as number, thamSo["b"] as number);
  return { thanhCong: true, ketQua };
}

function chayKichBanChuanHoa(
  card: A2aAgentCard,
  tenKyNang: string,
  thamSo: Record<string, unknown>,
): A2aTask {
  if (!kiemAgentCoHoTroKyNang(card, tenKyNang)) {
    return { id: "task-1", state: "failed" };
  }
  let task: A2aTask = { id: "task-1", state: "submitted" };
  task = capNhatTrangThai(task, "working");
  const ketQuaXuLy = agentBXuLyTaskChuanHoa(thamSo);
  if (!ketQuaXuLy.thanhCong) {
    return capNhatTrangThai(task, "failed");
  }
  task = capNhatTrangThai(task, "completed");
  return { ...task, ketQua: ketQuaXuLy.ketQua };
}

// KHONG chuan hoa: khong kiem AgentCard, khong initialize, khong validate schema
function agentBXuLyTaskKhongChuanHoa(thamSo: Record<string, unknown>): number {
  return thucThiToolCongThat(thamSo["a"] as number, thamSo["b"] as number);
}

function chayKichBanKhongChuanHoa(
  card: A2aAgentCard,
  tenKyNang: string,
  thamSo: Record<string, unknown>,
): number {
  return agentBXuLyTaskKhongChuanHoa(thamSo);
}

// -- Kich ban 1: chuan hoa, duong hanh phuc --
const kb1 = chayKichBanChuanHoa(agentBCard, "tinh_tong", { a: 3, b: 4 });
console.log("KB1", JSON.stringify(kb1), demGoiToolThat.soLan);

// -- Kich ban 2: chuan hoa, sai ky nang -- AgentCard chan TRUOC khi xu ly --
const kb2 = chayKichBanChuanHoa(agentBCard, "dich_thuat", { a: 3, b: 4 });
console.log("KB2", JSON.stringify(kb2), demGoiToolThat.soLan);

// -- Kich ban 3: chuan hoa, thieu tham so -- schema chan TRUOC khi goi tool that --
const kb3 = chayKichBanChuanHoa(agentBCard, "tinh_tong", { a: 3 });
console.log("KB3", JSON.stringify(kb3), demGoiToolThat.soLan);

// -- Kich ban 4: KHONG chuan hoa, sai ky nang -- khong ai chan, tra ve so SAI --
const kb4 = chayKichBanKhongChuanHoa(agentBCard, "dich_thuat", { a: 3, b: 4 });
console.log("KB4", kb4, demGoiToolThat.soLan);

// -- Kich ban 5: KHONG chuan hoa, thieu tham so -- tool goi that voi tham so thieu --
const kb5 = chayKichBanKhongChuanHoa(agentBCard, "tinh_tong", { a: 3 });
console.log("KB5", kb5, demGoiToolThat.soLan);
```

```text title=readonly
KB1 {"id":"task-1","state":"completed","ketQua":7} 1
KB2 {"id":"task-1","state":"failed"} 1
KB3 {"id":"task-1","state":"failed"} 1
KB4 7 2
KB5 null 3
```

Đọc kỹ CẢ năm dòng. KB1-KB3 LÀ luồng chuẩn hoá: KB2 VÀ KB3 đều dừng Ở
`"failed"`, VÀ `demGoiToolThat.soLan` VẪN LÀ `1` sau CẢ BA — tool thật
KHÔNG hề chạy thêm lần nào cho hai kịch bản lỗi. KB4 VÀ KB5 LÀ luồng
KHÔNG chuẩn hoá TRÊN CÙNG hai đầu vào lỗi đó: KB4 gọi kỹ năng
`"dich_thuat"` (agent B không hề công bố) nhưng `chayKichBanKhongChuanHoa`
KHÔNG hề đọc `tenKyNang` để quyết định gì — nó ÂM THẦM chạy phép cộng VÀ
trả về `7`, một con số nghe có vẻ hợp lý cho một yêu cầu DỊCH THUẬT,
KHÔNG hề có lỗi nào báo. KB5 gọi tool thật với `b` bị thiếu
(`thamSo["b"]` LÀ `undefined`) — tool VẪN chạy (`soLan` tăng từ `2` lên
`3`), phép cộng `3 + undefined` ra `NaN`, VÀ khi in qua `JSON.stringify`
(như MỌI output console.log trong track này) `NaN` hoá thành `null` —
một kết quả MƠ HỒ, không hề có thông báo lỗi nào giải thích VÌ SAO.
::::

::::predict{#doan-vi-sao-kb4-tra-ve-7 commitOnce}
Ở kịch bản KB4, `chayKichBanKhongChuanHoa(agentBCard, "dich_thuat", {a:
3, b: 4})` trả về `7` — một con số, cho một yêu cầu đáng lẽ LÀ dịch
thuật. VÌ SAO?

:::opt{correct}
Vì `chayKichBanKhongChuanHoa` VÀ `agentBXuLyTaskKhongChuanHoa` không hề
ĐỌC tham số `tenKyNang` Ở BẤT KỲ đâu trong thân hàm — chúng LUÔN gọi
thẳng `thucThiToolCongThat(thamSo["a"], thamSo["b"])` bất kể `tenKyNang`
được yêu cầu LÀ gì, nên một yêu cầu `"dich_thuat"` bị xử lý Y HỆT một
yêu cầu `"tinh_tong"`
:::
:::opt
Vì `agentBCard` khai NHẦM kỹ năng `"dich_thuat"` trỏ tới CÙNG một tool
`"cong"` như `"tinh_tong"`
::why
Nhầm rằng lỗi nằm Ở DỮ LIỆU (`agentBCard`) — nhưng `agentBCard` CHỈ khai
đúng MỘT kỹ năng (`"tinh_tong"`), không hề nhắc tới `"dich_thuat"` Ở bất
kỳ đâu (đã xác nhận qua `kiemAgentCoHoTroKyNang` Ở bài `4`).

Chỗ lệch: lỗi nằm Ở CHÍNH `chayKichBanKhongChuanHoa` — nó không hề gọi
`kiemAgentCoHoTroKyNang` để kiểm `tenKyNang` TRƯỚC khi xử lý, nên
`agentBCard` khai gì không hề ảnh hưởng tới kết quả.
::
:::
:::opt
Vì `3 + 4` tình cờ bằng `7`, VÀ đây LÀ một sự trùng hợp không liên quan
gì tới việc có chuẩn hoá hay không
::why
Nhầm rằng đây LÀ ngẫu nhiên — nhưng CHÍNH VÌ luồng không chuẩn hoá
KHÔNG BAO GIỜ đọc `tenKyNang`, nên BẤT KỲ giá trị nào của `tenKyNang`
(kể cả `"dich_thuat"`, `"tinh_tong"`, hay bất kỳ chuỗi nào khác) đều dẫn
tới CÙNG một phép cộng trên CÙNG `thamSo` — không phải một lần trùng
hợp, mà LÀ hành vi LUÔN LUÔN như vậy.

Chỗ lệch: đổi `tenKyNang` thành BẤT KỲ chuỗi nào khác (giữ nguyên
`thamSo`) vẫn cho ra ĐÚNG `7` — chứng minh kết quả không phụ thuộc
`tenKyNang` chút nào.
::
:::
::::

::::code{#viet_kich_ban_chuan_hoa}
Hoàn thiện `agentBXuLyTaskChuanHoa` — kiểm `serverCapabilities.tools`
(nếu `undefined`, trả `thanhCong: false`); validate tham số theo
`dinhNghiaToolCong.inputSchema` bằng `kiemTraThamSoTheoSchema` (nếu
không hợp, trả `thanhCong: false`); NGƯỢC LẠI gọi `thucThiToolCongThat`
VÀ trả `thanhCong: true` kèm `ketQua`. Hoàn thiện `chayKichBanChuanHoa`
— kiểm `kiemAgentCoHoTroKyNang` TRƯỚC (nếu không có kỹ năng, trả task
`"failed"` NGAY); NGƯỢC LẠI tạo task `"submitted"` → `"working"` →
gọi `agentBXuLyTaskChuanHoa` → NẾU thất bại chuyển `"failed"`, NẾU
thành công chuyển `"completed"` kèm `ketQua`.

```typescript title=starter
type A2aAgentSkill = { id: string; name: string; description: string };
type A2aAgentCard = {
  name: string;
  description: string;
  url: string;
  capabilities: { streaming?: boolean; pushNotifications?: boolean };
  skills: A2aAgentSkill[];
};

type A2aTaskState = "submitted" | "working" | "input-required" | "completed" | "canceled" | "failed";
type A2aTask = { id: string; state: A2aTaskState; ketQua?: number };

type McpJsonSchema = {
  type: "object";
  properties: Record<string, { type: "string" | "number" | "boolean" }>;
  required?: string[];
};
type McpToolDefinition = { name: string; description: string; inputSchema: McpJsonSchema };
type McpCapabilities = { tools?: Record<string, never> };

const CHUYEN_HOP_LE: Record<A2aTaskState, A2aTaskState[]> = {
  submitted: ["working", "canceled"],
  working: ["input-required", "completed", "failed", "canceled"],
  "input-required": ["working", "canceled"],
  completed: [],
  canceled: [],
  failed: [],
};

function chuyenTrangThaiTask(hienTai: A2aTaskState, moi: A2aTaskState): A2aTaskState {
  if (!CHUYEN_HOP_LE[hienTai].includes(moi)) {
    throw new Error(`chuyen trang thai khong hop le: ${hienTai} -> ${moi}`);
  }
  return moi;
}

function capNhatTrangThai(task: A2aTask, moi: A2aTaskState): A2aTask {
  return { ...task, state: chuyenTrangThaiTask(task.state, moi) };
}

function kiemAgentCoHoTroKyNang(card: A2aAgentCard, tenKyNang: string): boolean {
  return card.skills.some((sk) => sk.id === tenKyNang);
}

function kiemTraThamSoTheoSchema(schema: McpJsonSchema, thamSo: Record<string, unknown>): boolean {
  for (const truong of schema.required ?? []) {
    if (!(truong in thamSo)) return false;
  }
  for (const ten of Object.keys(schema.properties)) {
    const dinhNghia = schema.properties[ten];
    if (dinhNghia === undefined) continue;
    if (ten in thamSo && typeof thamSo[ten] !== dinhNghia.type) return false;
  }
  return true;
}

const agentBCard: A2aAgentCard = {
  name: "agent-tinh-toan",
  description: "Agent thuc hien phep tinh so hoc qua MCP",
  url: "https://vi-du.test/agent-tinh-toan",
  capabilities: {},
  skills: [{ id: "tinh_tong", name: "Tinh tong hai so", description: "Cong hai so nguyen" }],
};

const dinhNghiaToolCong: McpToolDefinition = {
  name: "cong",
  description: "Cong hai so",
  inputSchema: {
    type: "object",
    properties: { a: { type: "number" }, b: { type: "number" } },
    required: ["a", "b"],
  },
};

const demGoiToolThat = { soLan: 0 };
function thucThiToolCongThat(a: number, b: number): number {
  demGoiToolThat.soLan++;
  return a + b;
}

function agentBXuLyTaskChuanHoa(
  thamSo: Record<string, unknown>,
): { thanhCong: true; ketQua: number } | { thanhCong: false; loi: string } {
  ___
}

function chayKichBanChuanHoa(
  card: A2aAgentCard,
  tenKyNang: string,
  thamSo: Record<string, unknown>,
): A2aTask {
  ___
}

const kb1 = chayKichBanChuanHoa(agentBCard, "tinh_tong", { a: 3, b: 4 });
console.log(JSON.stringify(kb1));
```

```typescript title=solution
type A2aAgentSkill = { id: string; name: string; description: string };
type A2aAgentCard = {
  name: string;
  description: string;
  url: string;
  capabilities: { streaming?: boolean; pushNotifications?: boolean };
  skills: A2aAgentSkill[];
};

type A2aTaskState = "submitted" | "working" | "input-required" | "completed" | "canceled" | "failed";
type A2aTask = { id: string; state: A2aTaskState; ketQua?: number };

type McpJsonSchema = {
  type: "object";
  properties: Record<string, { type: "string" | "number" | "boolean" }>;
  required?: string[];
};
type McpToolDefinition = { name: string; description: string; inputSchema: McpJsonSchema };
type McpCapabilities = { tools?: Record<string, never> };

const CHUYEN_HOP_LE: Record<A2aTaskState, A2aTaskState[]> = {
  submitted: ["working", "canceled"],
  working: ["input-required", "completed", "failed", "canceled"],
  "input-required": ["working", "canceled"],
  completed: [],
  canceled: [],
  failed: [],
};

function chuyenTrangThaiTask(hienTai: A2aTaskState, moi: A2aTaskState): A2aTaskState {
  if (!CHUYEN_HOP_LE[hienTai].includes(moi)) {
    throw new Error(`chuyen trang thai khong hop le: ${hienTai} -> ${moi}`);
  }
  return moi;
}

function capNhatTrangThai(task: A2aTask, moi: A2aTaskState): A2aTask {
  return { ...task, state: chuyenTrangThaiTask(task.state, moi) };
}

function kiemAgentCoHoTroKyNang(card: A2aAgentCard, tenKyNang: string): boolean {
  return card.skills.some((sk) => sk.id === tenKyNang);
}

function kiemTraThamSoTheoSchema(schema: McpJsonSchema, thamSo: Record<string, unknown>): boolean {
  for (const truong of schema.required ?? []) {
    if (!(truong in thamSo)) return false;
  }
  for (const ten of Object.keys(schema.properties)) {
    const dinhNghia = schema.properties[ten];
    if (dinhNghia === undefined) continue;
    if (ten in thamSo && typeof thamSo[ten] !== dinhNghia.type) return false;
  }
  return true;
}

const agentBCard: A2aAgentCard = {
  name: "agent-tinh-toan",
  description: "Agent thuc hien phep tinh so hoc qua MCP",
  url: "https://vi-du.test/agent-tinh-toan",
  capabilities: {},
  skills: [{ id: "tinh_tong", name: "Tinh tong hai so", description: "Cong hai so nguyen" }],
};

const dinhNghiaToolCong: McpToolDefinition = {
  name: "cong",
  description: "Cong hai so",
  inputSchema: {
    type: "object",
    properties: { a: { type: "number" }, b: { type: "number" } },
    required: ["a", "b"],
  },
};

const demGoiToolThat = { soLan: 0 };
function thucThiToolCongThat(a: number, b: number): number {
  demGoiToolThat.soLan++;
  return a + b;
}

function agentBXuLyTaskChuanHoa(
  thamSo: Record<string, unknown>,
): { thanhCong: true; ketQua: number } | { thanhCong: false; loi: string } {
  const serverCapabilities: McpCapabilities = { tools: {} };
  if (serverCapabilities.tools === undefined) {
    return { thanhCong: false, loi: "server khong ho tro tools" };
  }
  if (!kiemTraThamSoTheoSchema(dinhNghiaToolCong.inputSchema, thamSo)) {
    return { thanhCong: false, loi: "tham so khong hop schema cua tool 'cong'" };
  }
  const ketQua = thucThiToolCongThat(thamSo["a"] as number, thamSo["b"] as number);
  return { thanhCong: true, ketQua };
}

function chayKichBanChuanHoa(
  card: A2aAgentCard,
  tenKyNang: string,
  thamSo: Record<string, unknown>,
): A2aTask {
  if (!kiemAgentCoHoTroKyNang(card, tenKyNang)) {
    return { id: "task-1", state: "failed" };
  }
  let task: A2aTask = { id: "task-1", state: "submitted" };
  task = capNhatTrangThai(task, "working");
  const ketQuaXuLy = agentBXuLyTaskChuanHoa(thamSo);
  if (!ketQuaXuLy.thanhCong) {
    return capNhatTrangThai(task, "failed");
  }
  task = capNhatTrangThai(task, "completed");
  return { ...task, ketQua: ketQuaXuLy.ketQua };
}

const kb1 = chayKichBanChuanHoa(agentBCard, "tinh_tong", { a: 3, b: 4 });
console.log(JSON.stringify(kb1));
```

```typescript title=test
if (kb1.state !== "completed") throw new Error("tinh_tong voi tham so hop le phai hoan tat task o trang thai 'completed'");
if (kb1.ketQua !== 7) throw new Error("3 + 4 phai bang 7");
if (demGoiToolThat.soLan !== 1) throw new Error("duong hanh phuc phai goi tool that DUNG 1 lan");

const kb2 = chayKichBanChuanHoa(agentBCard, "dich_thuat", { a: 3, b: 4 });
if (kb2.state !== "failed") throw new Error("agent KHONG co ky nang 'dich_thuat' -- task phai 'failed' NGAY, khong duoc xu ly");
if (demGoiToolThat.soLan !== 1) throw new Error("sai ky nang phai bi chan TRUOC KHI cham toi tool that -- soLan phai VAN la 1");

const kb3 = chayKichBanChuanHoa(agentBCard, "tinh_tong", { a: 3 });
if (kb3.state !== "failed") throw new Error("thieu tham so b -- schema phai chan, task phai 'failed'");
if (demGoiToolThat.soLan !== 1) throw new Error("thieu tham so phai bi schema chan TRUOC KHI goi tool that -- soLan phai VAN la 1");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (agentBXuLyTaskChuanHoa): ba buoc theo thu tu -- kiem capabilities.tools, kiem schema, roi moi goi tool that. Cho hai (chayKichBanChuanHoa): kiem AgentCard TRUOC TIEN (tra ve failed ngay neu khong co), sau do moi tao task va di qua submitted -> working -> (failed | completed)."
- kind: strategy
  body: "Cho dau: const serverCapabilities: McpCapabilities = { tools: {} }; if (serverCapabilities.tools === undefined) return { thanhCong: false, loi: 'server khong ho tro tools' }; if (!kiemTraThamSoTheoSchema(dinhNghiaToolCong.inputSchema, thamSo)) return { thanhCong: false, loi: \"tham so khong hop schema cua tool 'cong'\" }; const ketQua = thucThiToolCongThat(thamSo['a'] as number, thamSo['b'] as number); return { thanhCong: true, ketQua }; Cho hai: if (!kiemAgentCoHoTroKyNang(card, tenKyNang)) return { id: 'task-1', state: 'failed' }; let task: A2aTask = { id: 'task-1', state: 'submitted' }; task = capNhatTrangThai(task, 'working'); const ketQuaXuLy = agentBXuLyTaskChuanHoa(thamSo); if (!ketQuaXuLy.thanhCong) return capNhatTrangThai(task, 'failed'); task = capNhatTrangThai(task, 'completed'); return { ...task, ketQua: ketQuaXuLy.ketQua };"
- kind: one-line
  body: "Sao chep dung hai khoi o phan Strategy vao dung vi tri tuong ung, GIU NGUYEN thu tu cac buoc kiem tra."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "{\"id\":\"task-1\",\"state\":\"completed\",\"ketQua\":7}"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba cổng — Agent Card, vòng đời task, validate schema — chặn đúng ba lớp
lỗi khác nhau, VÀ luồng không chuẩn hoá cho thấy CHÍNH XÁC điều gì biến
mất khi bỏ qua chúng: một con số SAI không hề báo lỗi, VÀ một `null` mơ
hồ thay cho một thông báo rõ ràng. `q9.6a` khép Ở đây, 6/6 — track
`giao-thuc-mcp-a2a` (T9.6) còn tiếp tục Ở những phần sau, CHƯA đóng.
::::

::::reflect{#nghi-lai}
Cả sáu bài `q9.6a` xoay quanh đúng MỘT ý: chuẩn hoá không phải LÀ thêm
thủ tục cho vui — mỗi cổng (JSON-RPC, capability negotiation, tool
schema, Agent Card, vòng đời task) đứng Ở đúng MỘT chỗ có THỂ hỏng, VÀ
biến một lỗi ÂM THẦM (con số sai không báo, `null` không giải thích)
thành một lỗi CÓ TIẾNG (task `"failed"` kèm lý do, TRƯỚC khi bất kỳ tool
thật nào kịp chạy). MCP chuẩn hoá biên dọc (agent↔tool); A2A chuẩn hoá
biên ngang (agent↔agent) — nhưng cả hai cùng LÀM một việc: thay "hy vọng
bên kia hiểu đúng" bằng "xác nhận được bên kia hiểu đúng, TRƯỚC KHI có
gì chạy thật".
::::

::::checkpoint{mastery=0.95}
::::
