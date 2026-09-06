---
id: ky-nghe-ung-dung-ai.giao-thuc-mcp-a2a.tool-schema-qua-mcp
title: "Tool schema qua MCP — \"tools/list\" và \"tools/call\""
summary: "kiemTraThamSoTheoSchema(schema, thamSo) validate một object tham số THEO ĐÚNG McpJsonSchema (required[] + properties[ten].type) TRƯỚC KHI gọi tool thật — dùng lại được CHO MỌI tool công bố inputSchema, không phải viết tay một hàm kiểm riêng cho từng tool như T9.3 bài 9. goiToolCoValidateSchema ráp validate + thực thi: trên tool 'lay_thoi_tiet' (McpToolDefinition yêu cầu bắt buộc thanhPho:string) — gọi hợp lệ {thanhPho:'Hanoi'} THỰC THI tool thật (demGoiToolThat.soLan=1, trả 'Hanoi: 22C'); gọi thiếu thanhPho VÀ gọi sai kiểu (thanhPho:123) đều bị CHẶN (isError:true) TRƯỚC khi chạm tool thật — demGoiToolThat.soLan vẫn giữ nguyên 1 sau CẢ BA lần gọi, chứng minh bằng số rằng tool thật chỉ chạy đúng 1 trong 3 lần."
locale: vi
track: ky-nghe-ung-dung-ai
module: giao-thuc-mcp-a2a
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [kna.tool-schema-qua-mcp]
requires: [kna.capability-negotiation-truoc-khi-dung]
concepts: [kna.tool-schema-qua-mcp]
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

::::byte{trigger=enter mood=curious pose=lean-in}
`"initialize"` xác nhận server CÓ hỗ trợ tools nói chung. Nhưng agent
còn cần biết CHÍNH XÁC tool nào tồn tại, VÀ tool đó đòi tham số hình
dạng gì — đó LÀ việc của `"tools/list"`. VÀ trước khi gọi `"tools/call"`
thật sự, có một bước KHÔNG được bỏ qua: kiểm tham số ĐÚNG hình dạng
`inputSchema` đã công bố.
::::

::::explain{#schema_va_validate}
`McpToolDefinition` công bố `inputSchema` — một JSON Schema TỐI GIẢN:
`required` (những khoá BẮT BUỘC) VÀ `properties` (kiểu của MỖI khoá).
`kiemTraThamSoTheoSchema` đọc ĐÚNG hai phần đó để validate: thiếu MỘT
khoá Ở `required` → `false`; CÓ khoá nhưng SAI kiểu Ở `properties` →
`false`; NGOÀI hai lỗi đó → `true`:

```typescript title=readonly
type McpJsonSchema = {
  type: "object";
  properties: Record<string, { type: "string" | "number" | "boolean" }>;
  required?: string[];
};

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

const schema: McpJsonSchema = {
  type: "object",
  properties: { thanhPho: { type: "string" } },
  required: ["thanhPho"],
};

console.log(kiemTraThamSoTheoSchema(schema, { thanhPho: "Hanoi" }));
console.log(kiemTraThamSoTheoSchema(schema, {}));
console.log(kiemTraThamSoTheoSchema(schema, { thanhPho: 123 }));
```

```text title=readonly
true
false
false
```

Cùng MỘT hàm `kiemTraThamSoTheoSchema` validate được CHO BẤT KỲ
`McpJsonSchema` nào — khác hẳn `validateDoiSo` Ở T9.3 bài `9`
(`ky-thuat-harness`), vốn viết TAY riêng cho hình dạng
`DoiSoTraCuuDonHang` cụ thể. Ở đây, hình dạng NẰM TRONG dữ liệu
(`schema`), không nằm trong mã — thêm một tool MỚI với `inputSchema`
khác chỉ cần một object mới, không cần viết lại logic validate.
::::

::::example{#goi_tool_co_validate}
`goiToolCoValidateSchema` ráp validate VÀ thực thi thành MỘT hàng rào:
validate trước, CHỈ khi hợp lệ mới chạm tới `thucThiToolThat` — VÀ
`demGoiToolThat` đếm số lần tool THẬT thật sự chạy, để chứng minh hàng
rào có chặn ĐÚNG hay không:

```typescript title=readonly
type McpJsonSchema = {
  type: "object";
  properties: Record<string, { type: "string" | "number" | "boolean" }>;
  required?: string[];
};

type McpToolDefinition = { name: string; description: string; inputSchema: McpJsonSchema };
type McpContentBlock = { type: "text"; text: string };
type McpToolCallResult = { content: McpContentBlock[]; isError?: boolean };

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

const demGoiToolThat = { soLan: 0 };
const BANG_THOI_TIET: Record<string, string> = { Hanoi: "22C", "Ho Chi Minh": "31C" };

function thucThiToolThat(thanhPho: string): McpToolCallResult {
  demGoiToolThat.soLan++;
  const nhietDo = BANG_THOI_TIET[thanhPho];
  if (nhietDo === undefined) {
    return { content: [{ type: "text", text: `khong tim thay thanh pho: ${thanhPho}` }], isError: true };
  }
  return { content: [{ type: "text", text: `${thanhPho}: ${nhietDo}` }] };
}

const dinhNghiaLayThoiTiet: McpToolDefinition = {
  name: "lay_thoi_tiet",
  description: "Tra cuu nhiet do hien tai cua mot thanh pho",
  inputSchema: {
    type: "object",
    properties: { thanhPho: { type: "string" } },
    required: ["thanhPho"],
  },
};

function goiToolCoValidateSchema(
  dinhNghia: McpToolDefinition,
  thamSo: Record<string, unknown>,
): McpToolCallResult {
  if (!kiemTraThamSoTheoSchema(dinhNghia.inputSchema, thamSo)) {
    return { content: [{ type: "text", text: "tham so khong hop schema" }], isError: true };
  }
  return thucThiToolThat(thamSo["thanhPho"] as string);
}

const hopLe = goiToolCoValidateSchema(dinhNghiaLayThoiTiet, { thanhPho: "Hanoi" });
console.log(JSON.stringify(hopLe), demGoiToolThat.soLan);

const thieuKhoa = goiToolCoValidateSchema(dinhNghiaLayThoiTiet, {});
console.log(JSON.stringify(thieuKhoa), demGoiToolThat.soLan);

const saiKieu = goiToolCoValidateSchema(dinhNghiaLayThoiTiet, { thanhPho: 123 });
console.log(JSON.stringify(saiKieu), demGoiToolThat.soLan);
```

```text title=readonly
{"content":[{"type":"text","text":"Hanoi: 22C"}]} 1
{"content":[{"type":"text","text":"tham so khong hop schema"}],"isError":true} 1
{"content":[{"type":"text","text":"tham so khong hop schema"}],"isError":true} 1
```

Ba lời gọi, VÀ `demGoiToolThat.soLan` VẪN LÀ `1` sau CẢ BA — tool thật
chỉ chạy đúng MỘT lần (lần hợp lệ). Hai lần còn lại bị CHẶN Ở
`kiemTraThamSoTheoSchema`, KHÔNG BAO GIỜ chạm tới `thucThiToolThat`.
::::

::::predict{#doan-goi-them-truong commitOnce}
Gọi `goiToolCoValidateSchema(dinhNghiaLayThoiTiet, { thanhPho: "Hanoi",
doAmDoi: true })` — CÓ đúng `thanhPho` kiểu `string`, NHƯNG có THÊM một
khoá `doAmDoi` mà `inputSchema` không hề khai. Lời gọi này CÓ bị chặn
(`isError: true`) không?

:::opt{correct}
Không — `kiemTraThamSoTheoSchema` chỉ kiểm các khoá NẰM TRONG
`required`/`properties` của schema; một khoá THỪA không được khai
không hề bị vòng `for` nào chạm tới, nên tool THẬT vẫn được gọi bình
thường
:::
:::opt
Có — vì schema chỉ khai `thanhPho`, mọi khoá LẠ đều LÀ vi phạm schema
::why
Nhầm rằng schema Ở đây CẤM khoá lạ (kiểu `additionalProperties: false`
trong JSON Schema đầy đủ) — nhưng `kiemTraThamSoTheoSchema` không hề có
bước nào kiểm "có khoá THỪA không", nó chỉ LẶP qua `required` VÀ
`properties` đã khai.

Chỗ lệch: vòng `for (const ten of Object.keys(schema.properties))` chỉ
duyệt các khoá SCHEMA ĐÃ KHAI (Ở đây chỉ `thanhPho`) — `doAmDoi` không
nằm trong `schema.properties` nên không bao giờ được xét tới.
::
:::
:::opt
Không xác định — phụ thuộc tool THẬT có đọc `doAmDoi` hay không
::why
Nhầm giai đoạn VALIDATE (trước khi gọi tool) với giai đoạn THỰC THI (bên
trong tool) — `kiemTraThamSoTheoSchema` trả `true`/`false` HOÀN TOÀN dựa
trên chính `thamSo` VÀ `schema`, không hề gọi tới `thucThiToolThat`.

Chỗ lệch: kết quả CÓ bị chặn hay không được quyết định XONG trước khi
`goiToolCoValidateSchema` gọi `thucThiToolThat` — tool thật (đọc gì bên
trong nó) không ảnh hưởng gì tới quyết định đó.
::
:::
::::

::::code{#viet_goi_tool_co_validate_schema}
Hoàn thiện `kiemTraThamSoTheoSchema` — LẶP qua `schema.required ?? []`,
trả `false` NẾU thiếu khoá nào; LẶP qua `schema.properties`, trả `false`
NẾU khoá CÓ mặt Ở `thamSo` nhưng SAI kiểu; NGƯỢC LẠI trả `true`. Hoàn
thiện `goiToolCoValidateSchema` — NẾU `kiemTraThamSoTheoSchema` trả
`false`, trả về `{ content: [...], isError: true }` KHÔNG gọi
`thucThiToolThat`; NGƯỢC LẠI gọi `thucThiToolThat(thamSo["thanhPho"] as
string)`.

```typescript title=starter
type McpJsonSchema = {
  type: "object";
  properties: Record<string, { type: "string" | "number" | "boolean" }>;
  required?: string[];
};

type McpToolDefinition = { name: string; description: string; inputSchema: McpJsonSchema };
type McpContentBlock = { type: "text"; text: string };
type McpToolCallResult = { content: McpContentBlock[]; isError?: boolean };

function kiemTraThamSoTheoSchema(schema: McpJsonSchema, thamSo: Record<string, unknown>): boolean {
  ___
}

const demGoiToolThat = { soLan: 0 };
const BANG_THOI_TIET: Record<string, string> = { Hanoi: "22C", "Ho Chi Minh": "31C" };

function thucThiToolThat(thanhPho: string): McpToolCallResult {
  demGoiToolThat.soLan++;
  const nhietDo = BANG_THOI_TIET[thanhPho];
  if (nhietDo === undefined) {
    return { content: [{ type: "text", text: `khong tim thay thanh pho: ${thanhPho}` }], isError: true };
  }
  return { content: [{ type: "text", text: `${thanhPho}: ${nhietDo}` }] };
}

const dinhNghiaLayThoiTiet: McpToolDefinition = {
  name: "lay_thoi_tiet",
  description: "Tra cuu nhiet do hien tai cua mot thanh pho",
  inputSchema: {
    type: "object",
    properties: { thanhPho: { type: "string" } },
    required: ["thanhPho"],
  },
};

function goiToolCoValidateSchema(
  dinhNghia: McpToolDefinition,
  thamSo: Record<string, unknown>,
): McpToolCallResult {
  ___
}

const hopLe = goiToolCoValidateSchema(dinhNghiaLayThoiTiet, { thanhPho: "Hanoi" });
console.log(JSON.stringify(hopLe));
```

```typescript title=solution
type McpJsonSchema = {
  type: "object";
  properties: Record<string, { type: "string" | "number" | "boolean" }>;
  required?: string[];
};

type McpToolDefinition = { name: string; description: string; inputSchema: McpJsonSchema };
type McpContentBlock = { type: "text"; text: string };
type McpToolCallResult = { content: McpContentBlock[]; isError?: boolean };

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

const demGoiToolThat = { soLan: 0 };
const BANG_THOI_TIET: Record<string, string> = { Hanoi: "22C", "Ho Chi Minh": "31C" };

function thucThiToolThat(thanhPho: string): McpToolCallResult {
  demGoiToolThat.soLan++;
  const nhietDo = BANG_THOI_TIET[thanhPho];
  if (nhietDo === undefined) {
    return { content: [{ type: "text", text: `khong tim thay thanh pho: ${thanhPho}` }], isError: true };
  }
  return { content: [{ type: "text", text: `${thanhPho}: ${nhietDo}` }] };
}

const dinhNghiaLayThoiTiet: McpToolDefinition = {
  name: "lay_thoi_tiet",
  description: "Tra cuu nhiet do hien tai cua mot thanh pho",
  inputSchema: {
    type: "object",
    properties: { thanhPho: { type: "string" } },
    required: ["thanhPho"],
  },
};

function goiToolCoValidateSchema(
  dinhNghia: McpToolDefinition,
  thamSo: Record<string, unknown>,
): McpToolCallResult {
  if (!kiemTraThamSoTheoSchema(dinhNghia.inputSchema, thamSo)) {
    return { content: [{ type: "text", text: "tham so khong hop schema" }], isError: true };
  }
  return thucThiToolThat(thamSo["thanhPho"] as string);
}

const hopLe = goiToolCoValidateSchema(dinhNghiaLayThoiTiet, { thanhPho: "Hanoi" });
console.log(JSON.stringify(hopLe));
```

```typescript title=test
if (hopLe.isError !== undefined) throw new Error("goi hop le KHONG duoc co isError");
if (hopLe.content[0]?.text !== "Hanoi: 22C") throw new Error("goi hop le phai tra ve dung noi dung that cua tool");
if (demGoiToolThat.soLan !== 1) throw new Error("goi hop le phai THUC THI tool that dung 1 lan");

const thieuKhoa = goiToolCoValidateSchema(dinhNghiaLayThoiTiet, {});
if (thieuKhoa.isError !== true) throw new Error("thieu truong bat buoc thanhPho phai bi CHAN (isError true)");
if (demGoiToolThat.soLan !== 1) throw new Error("thieu truong bat buoc KHONG duoc goi tool that -- soLan phai VAN la 1");

const saiKieu = goiToolCoValidateSchema(dinhNghiaLayThoiTiet, { thanhPho: 123 });
if (saiKieu.isError !== true) throw new Error("sai kieu (so thay vi chuoi) phai bi CHAN (isError true)");
if (demGoiToolThat.soLan !== 1) throw new Error("sai kieu KHONG duoc goi tool that -- soLan phai VAN la 1");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (kiemTraThamSoTheoSchema): mot vong for tren required kiem thieu khoa, mot vong for tren properties kiem sai kieu. Cho hai (goiToolCoValidateSchema): if khong hop schema thi tra ve isError true NGAY, khong goi thucThiToolThat; nguoc lai moi goi thucThiToolThat."
- kind: strategy
  body: "Cho dau: for (const truong of schema.required ?? []) { if (!(truong in thamSo)) return false; } for (const ten of Object.keys(schema.properties)) { const dinhNghia = schema.properties[ten]; if (dinhNghia === undefined) continue; if (ten in thamSo && typeof thamSo[ten] !== dinhNghia.type) return false; } return true; Cho hai: if (!kiemTraThamSoTheoSchema(dinhNghia.inputSchema, thamSo)) return { content: [{ type: 'text', text: 'tham so khong hop schema' }], isError: true }; return thucThiToolThat(thamSo['thanhPho'] as string);"
- kind: one-line
  body: "Sao chep dung hai khoi o phan Strategy vao dung vi tri tuong ung."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "{\"content\":[{\"type\":\"text\",\"text\":\"Hanoi: 22C\"}]}"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba lời gọi, một tool thật chạy đúng một lần. `kiemTraThamSoTheoSchema`
đứng gác y hệt tinh thần `validateDoiSo` Ở T9.3 — chỉ khác giờ hàng rào
ĐỌC hình dạng từ dữ liệu (`inputSchema`) thay vì được viết cứng cho một
tool. Bài sau chuyển sang phía BÊN KIA của agent — làm sao biết một
agent KHÁC có hiểu được yêu cầu mình sắp gửi hay không.
::::

::::reflect{#nghi-lai}
`validateDoiSo` Ở T9.3 giải quyết đúng MỘT tool, MỘT hình dạng tham số —
viết tay từng nhánh `if` cho từng khoá. `kiemTraThamSoTheoSchema` giải
quyết CẢ MỘT LỚP tool: bất kỳ `McpToolDefinition` nào công bố
`inputSchema` đều validate được bằng ĐÚNG một hàm, không cần viết thêm
dòng nào khi có tool mới. Đó chính LÀ lý do MCP bắt tool công bố schema
CHUẨN thay vì để mỗi tool tự định nghĩa luật kiểm tra riêng — schema LÀ
dữ liệu, VÀ dữ liệu thì DÙNG LẠI được, còn logic viết tay thì không.
::::

::::checkpoint{mastery=0.85}
::::
