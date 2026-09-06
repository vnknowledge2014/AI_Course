---
id: ky-nghe-ung-dung-ai.giao-thuc-mcp-a2a.ma-loi-json-rpc-day-du
title: "Mã lỗi JSON-RPC đầy đủ — parse error, invalid request, internal error"
summary: "xuLyYeuCauDayDu(raw) nhận một CHUỖI thô (chưa parse) và trả JsonRpcResponse<number>, đi qua NĂM cửa theo đúng thứ tự: JSON.parse thất bại → error.code=-32700 (id luôn null, vì id CHƯA đọc được); parse được nhưng thiếu jsonrpc/method hoặc sai kiểu → -32600; method lạ → -32601 (đã dạy ở bài 1); tham số sai (kiemTraThamSoTheoSchema-style) → -32602 (đã dạy ở bài 1); tool THẬT ném exception (method 'chia' với b=0) → catch bắt được, trả -32603 kèm data là message gốc của exception, KHÔNG để lỗi rò ra ngoài hàm. Trên năm chuỗi đầu vào cụ thể: JSON hỏng '{a:1' → -32700, id=null; thiếu jsonrpc → -32600, id=3 (id VẪN đọc được dù jsonrpc thiếu, vì id hợp lệ độc lập với field khác); method 'tru' không tồn tại → -32601, id=4; 'cong' thiếu tham số b → -32602, id=5; 'chia' với {a:10,b:0} → -32603 kèm data='chia cho 0', id=6; 'chia' với {a:10,b:2} → result=5, id=7. demGoiChiaThat.soLan=2 sau CẢ sáu lời gọi (chỉ hai lần dispatch chạm tới nhánh chia, một ném exception một không) — không có exception nào thoát ra ngoài xuLyYeuCauDayDu."
locale: vi
track: ky-nghe-ung-dung-ai
module: giao-thuc-mcp-a2a
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [kna.ma-loi-json-rpc-day-du]
requires: [kna.boss-agent-a-goi-agent-b-qua-mcp]
concepts: [kna.ma-loi-json-rpc-day-du]
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
`q9.6a` dạy đúng hai mã lỗi: method lạ (`-32601`) VÀ tham số sai
(`-32602`) — cả hai xảy ra SAU KHI request ĐÃ parse được VÀ ĐÃ đúng hình
dạng. Nhưng một server THẬT nhận CHUỖI thô qua mạng: chuỗi đó CÓ THỂ
không hề LÀ JSON hợp lệ, CÓ THỂ thiếu field bắt buộc, VÀ tool bên trong
CÓ THỂ tự ném exception. Ba chuyện đó cần BA mã lỗi khác — VÀ một hàng
rào không được để bất kỳ exception nào rò ra ngoài.
::::

::::explain{#ba_ma_loi_moi}
`xuLyYeuCauDayDu` nhận `raw: string` — KHÔNG PHẢI một `JsonRpcRequest` đã
parse sẵn như bài `1`. Ba mã lỗi MỚI xử lý ba tầng lỗi khác nhau:
`-32700` (Parse error — `JSON.parse` tự nó NÉM exception, `id` LUÔN LÀ
`null` vì request CHƯA từng được đọc thành object để lấy `id`), `-32600`
(Invalid Request — parse được NHƯNG thiếu `jsonrpc`/`method` hoặc sai
kiểu; `id` VẪN có thể đọc được NẾU nó hợp lệ, độc lập với field khác),
VÀ `-32603` (Internal error — request ĐÚNG hình dạng, method VÀ tham số
ĐỀU hợp lệ, nhưng CHÍNH tool bên trong ném exception lúc thực thi):

```typescript title=readonly
type JsonRpcResponse<TResult> =
  | { jsonrpc: "2.0"; id: string | number; result: TResult }
  | { jsonrpc: "2.0"; id: string | number | null; error: { code: number; message: string; data?: string } };

function layIdNeuCo(parsed: unknown): string | number | null {
  if (typeof parsed !== "object" || parsed === null) return null;
  const p = parsed as Record<string, unknown>;
  const id = p["id"];
  if (typeof id === "string") return id;
  if (typeof id === "number") return id;
  return null;
}

function taoLoiGiaoThuc(id: string | number | null, code: number, message: string): JsonRpcResponse<never> {
  return { jsonrpc: "2.0", id, error: { code, message } };
}

const parseHong = "{a:1";
try {
  JSON.parse(parseHong);
} catch {
  console.log(JSON.stringify(taoLoiGiaoThuc(null, -32700, "Parse error")));
}

const thieuJsonrpc = JSON.parse('{"id":3,"method":"cong","params":{"a":1,"b":2}}');
console.log(JSON.stringify(taoLoiGiaoThuc(layIdNeuCo(thieuJsonrpc), -32600, "Invalid Request")));
```

```text title=readonly
{"jsonrpc":"2.0","id":null,"error":{"code":-32700,"message":"Parse error"}}
{"jsonrpc":"2.0","id":3,"error":{"code":-32600,"message":"Invalid Request"}}
```

Chú ý `id` Ở dòng đầu LÀ `null` (chưa từng đọc được object), nhưng `id`
Ở dòng hai LÀ `3` — request thiếu `jsonrpc` NHƯNG `id` vẫn đọc được VÌ
`layIdNeuCo` chỉ cần đúng MỘT field (`id` hợp lệ), không cần CẢ request
hợp lệ.
::::

::::example{#day_du_nam_cong}
`xuLyYeuCauDayDu` ráp CẢ NĂM cửa thành một hàng rào DUY NHẤT, theo đúng
THỨ TỰ: parse → hình dạng → method → tham số → thực thi (bọc `try/catch`
để bắt exception CỦA TOOL). `demGoiChiaThat` đếm số lần tool `chia` THẬT
sự chạy, kể cả những lần nó ném exception:

```typescript title=readonly
type JsonRpcResponse<TResult> =
  | { jsonrpc: "2.0"; id: string | number; result: TResult }
  | { jsonrpc: "2.0"; id: string | number | null; error: { code: number; message: string; data?: string } };

function layIdNeuCo(parsed: unknown): string | number | null {
  if (typeof parsed !== "object" || parsed === null) return null;
  const p = parsed as Record<string, unknown>;
  const id = p["id"];
  if (typeof id === "string") return id;
  if (typeof id === "number") return id;
  return null;
}

function laYeuCauHopLe(
  parsed: unknown,
): parsed is { jsonrpc: "2.0"; id: string | number; method: string; params?: Record<string, unknown> } {
  if (typeof parsed !== "object" || parsed === null) return false;
  const p = parsed as Record<string, unknown>;
  if (p["jsonrpc"] !== "2.0") return false;
  if (typeof p["method"] !== "string") return false;
  if (typeof p["id"] !== "string" && typeof p["id"] !== "number") return false;
  return true;
}

function laThamSoHopLe(params: unknown): params is { a: number; b: number } {
  if (typeof params !== "object" || params === null) return false;
  const p = params as Record<string, unknown>;
  return typeof p["a"] === "number" && typeof p["b"] === "number";
}

const demGoiChiaThat = { soLan: 0 };
function thucThiChiaThat(a: number, b: number): number {
  demGoiChiaThat.soLan++;
  if (b === 0) throw new Error("chia cho 0");
  return a / b;
}

function xuLyYeuCauDayDu(raw: string): JsonRpcResponse<number> {
  let parsed: unknown;
  try {
    parsed = JSON.parse(raw);
  } catch {
    return { jsonrpc: "2.0", id: null, error: { code: -32700, message: "Parse error" } };
  }
  if (!laYeuCauHopLe(parsed)) {
    return { jsonrpc: "2.0", id: layIdNeuCo(parsed), error: { code: -32600, message: "Invalid Request" } };
  }
  if (parsed.method !== "cong" && parsed.method !== "chia") {
    return { jsonrpc: "2.0", id: parsed.id, error: { code: -32601, message: `method khong ton tai: ${parsed.method}` } };
  }
  if (!laThamSoHopLe(parsed.params)) {
    return { jsonrpc: "2.0", id: parsed.id, error: { code: -32602, message: "tham so khong hop le" } };
  }
  try {
    const ketQua = parsed.method === "cong" ? parsed.params.a + parsed.params.b : thucThiChiaThat(parsed.params.a, parsed.params.b);
    return { jsonrpc: "2.0", id: parsed.id, result: ketQua };
  } catch (e) {
    return { jsonrpc: "2.0", id: parsed.id, error: { code: -32603, message: "Internal error", data: (e as Error).message } };
  }
}

console.log(JSON.stringify(xuLyYeuCauDayDu("{a:1")));
console.log(JSON.stringify(xuLyYeuCauDayDu('{"id":3,"method":"cong","params":{"a":1,"b":2}}')));
console.log(JSON.stringify(xuLyYeuCauDayDu('{"jsonrpc":"2.0","id":4,"method":"tru","params":{"a":3,"b":4}}')));
console.log(JSON.stringify(xuLyYeuCauDayDu('{"jsonrpc":"2.0","id":5,"method":"cong","params":{"a":3}}')));
console.log(JSON.stringify(xuLyYeuCauDayDu('{"jsonrpc":"2.0","id":6,"method":"chia","params":{"a":10,"b":0}}')));
console.log(JSON.stringify(xuLyYeuCauDayDu('{"jsonrpc":"2.0","id":7,"method":"chia","params":{"a":10,"b":2}}')));
console.log(demGoiChiaThat.soLan);
```

```text title=readonly
{"jsonrpc":"2.0","id":null,"error":{"code":-32700,"message":"Parse error"}}
{"jsonrpc":"2.0","id":3,"error":{"code":-32600,"message":"Invalid Request"}}
{"jsonrpc":"2.0","id":4,"error":{"code":-32601,"message":"method khong ton tai: tru"}}
{"jsonrpc":"2.0","id":5,"error":{"code":-32602,"message":"tham so khong hop le"}}
{"jsonrpc":"2.0","id":6,"error":{"code":-32603,"message":"Internal error","data":"chia cho 0"}}
{"jsonrpc":"2.0","id":7,"result":5}
2
```

Sáu lời gọi, NĂM mã lỗi khác nhau CỘNG một thành công — VÀ
`demGoiChiaThat.soLan` LÀ `2` (chỉ hai lời gọi CÓ method `"chia"` VÀ
tham số hợp lệ mới chạm tới `thucThiChiaThat`; lời gọi `id=6` ném
exception nhưng exception đó bị `catch` NGAY BÊN TRONG `xuLyYeuCauDayDu`
— không hề thoát ra ngoài làm crash tiến trình gọi).
::::

::::predict{#doan-id-parse-error commitOnce}
Gọi `xuLyYeuCauDayDu('{"id": 99, method cong')` — một chuỗi CHỨA số
`99` trông giống một `id`, nhưng bản thân chuỗi KHÔNG PHẢI JSON hợp lệ
(thiếu dấu ngoặc kép quanh `method`, thiếu dấu hai chấm sau nó). `id`
Ở response trả về LÀ gì?

:::opt{correct}
`null` — `JSON.parse` NÉM exception NGAY (chuỗi không phải JSON hợp lệ),
nên nhánh `catch` chạy TRƯỚC KHI bất kỳ dòng nào đọc được `id`; `-32700`
LUÔN đi kèm `id: null`, bất kể chuỗi gốc trông "giống" có `id` hay không
:::
:::opt
`99` — vì con số `99` CÓ xuất hiện trong chuỗi gốc, nên hàm vẫn "nhìn
thấy" VÀ trích ra được
::why
Nhầm rằng `layIdNeuCo` có thể đọc `id` bằng cách QUÉT chuỗi thô tìm số —
nhưng `layIdNeuCo` chỉ nhận MỘT object ĐÃ PARSE (`unknown`), không hề
đọc chuỗi gốc. Ở nhánh `-32700`, `JSON.parse` ném exception TRƯỚC KHI có
bất kỳ object nào tồn tại để `layIdNeuCo` đọc.

Chỗ lệch: nhánh `catch` của `-32700` trả thẳng `id: null`, KHÔNG hề gọi
`layIdNeuCo` — không có con đường nào từ chuỗi gốc tới `id: 99` trong
trường hợp NÀY.
::
:::
:::opt
Lỗi biên dịch — TypeScript sẽ từ chối gọi hàm với một chuỗi không phải
JSON hợp lệ
::why
Nhầm rằng "hợp lệ JSON hay không" LÀ một điều kiểm được LÚC BIÊN DỊCH —
nhưng `raw: string` LÀ kiểu `string` THUẦN; TypeScript không hề biết
(và không thể biết) nội dung một chuỗi CÓ PHẢI JSON hợp lệ không cho
tới khi chương trình THẬT SỰ chạy `JSON.parse` LÚC RUNTIME.

Chỗ lệch: hàm CHẠY bình thường VÀ trả về response `-32700` LÚC RUNTIME —
không có gì ngăn TypeScript biên dịch lời gọi này.
::
:::
::::

::::code{#viet_xu_ly_yeu_cau_day_du}
Hoàn thiện `laYeuCauHopLe` — type-guard kiểm `parsed` LÀ object khác
`null`, CÓ `jsonrpc === "2.0"`, CÓ `method` kiểu `string`, VÀ CÓ `id`
kiểu `string` hoặc `number`. Hoàn thiện `xuLyYeuCauDayDu` — theo ĐÚNG
THỨ TỰ: `try/catch` quanh `JSON.parse` (thất bại → `-32700`, `id: null`);
NẾU `!laYeuCauHopLe(parsed)` → `-32600` (dùng `layIdNeuCo(parsed)` LÀM
`id`); NẾU `method` khác `"cong"` VÀ khác `"chia"` → `-32601`; NẾU
`!laThamSoHopLe(parsed.params)` → `-32602`; NGƯỢC LẠI bọc phần TÍNH
TOÁN (gọi `thucThiChiaThat` khi `method` LÀ `"chia"`) trong MỘT
`try/catch` KHÁC, bắt exception thành `-32603` kèm `data` LÀ
`(e as Error).message`.

```typescript title=starter
type JsonRpcResponse<TResult> =
  | { jsonrpc: "2.0"; id: string | number; result: TResult }
  | { jsonrpc: "2.0"; id: string | number | null; error: { code: number; message: string; data?: string } };

function layIdNeuCo(parsed: unknown): string | number | null {
  if (typeof parsed !== "object" || parsed === null) return null;
  const p = parsed as Record<string, unknown>;
  const id = p["id"];
  if (typeof id === "string") return id;
  if (typeof id === "number") return id;
  return null;
}

function laYeuCauHopLe(
  parsed: unknown,
): parsed is { jsonrpc: "2.0"; id: string | number; method: string; params?: Record<string, unknown> } {
  ___
}

function laThamSoHopLe(params: unknown): params is { a: number; b: number } {
  if (typeof params !== "object" || params === null) return false;
  const p = params as Record<string, unknown>;
  return typeof p["a"] === "number" && typeof p["b"] === "number";
}

const demGoiChiaThat = { soLan: 0 };
function thucThiChiaThat(a: number, b: number): number {
  demGoiChiaThat.soLan++;
  if (b === 0) throw new Error("chia cho 0");
  return a / b;
}

function xuLyYeuCauDayDu(raw: string): JsonRpcResponse<number> {
  ___
}

const ketQuaChia = xuLyYeuCauDayDu('{"jsonrpc":"2.0","id":7,"method":"chia","params":{"a":10,"b":2}}');
console.log(JSON.stringify(ketQuaChia));
```

```typescript title=solution
type JsonRpcResponse<TResult> =
  | { jsonrpc: "2.0"; id: string | number; result: TResult }
  | { jsonrpc: "2.0"; id: string | number | null; error: { code: number; message: string; data?: string } };

function layIdNeuCo(parsed: unknown): string | number | null {
  if (typeof parsed !== "object" || parsed === null) return null;
  const p = parsed as Record<string, unknown>;
  const id = p["id"];
  if (typeof id === "string") return id;
  if (typeof id === "number") return id;
  return null;
}

function laYeuCauHopLe(
  parsed: unknown,
): parsed is { jsonrpc: "2.0"; id: string | number; method: string; params?: Record<string, unknown> } {
  if (typeof parsed !== "object" || parsed === null) return false;
  const p = parsed as Record<string, unknown>;
  if (p["jsonrpc"] !== "2.0") return false;
  if (typeof p["method"] !== "string") return false;
  if (typeof p["id"] !== "string" && typeof p["id"] !== "number") return false;
  return true;
}

function laThamSoHopLe(params: unknown): params is { a: number; b: number } {
  if (typeof params !== "object" || params === null) return false;
  const p = params as Record<string, unknown>;
  return typeof p["a"] === "number" && typeof p["b"] === "number";
}

const demGoiChiaThat = { soLan: 0 };
function thucThiChiaThat(a: number, b: number): number {
  demGoiChiaThat.soLan++;
  if (b === 0) throw new Error("chia cho 0");
  return a / b;
}

function xuLyYeuCauDayDu(raw: string): JsonRpcResponse<number> {
  let parsed: unknown;
  try {
    parsed = JSON.parse(raw);
  } catch {
    return { jsonrpc: "2.0", id: null, error: { code: -32700, message: "Parse error" } };
  }
  if (!laYeuCauHopLe(parsed)) {
    return { jsonrpc: "2.0", id: layIdNeuCo(parsed), error: { code: -32600, message: "Invalid Request" } };
  }
  if (parsed.method !== "cong" && parsed.method !== "chia") {
    return { jsonrpc: "2.0", id: parsed.id, error: { code: -32601, message: `method khong ton tai: ${parsed.method}` } };
  }
  if (!laThamSoHopLe(parsed.params)) {
    return { jsonrpc: "2.0", id: parsed.id, error: { code: -32602, message: "tham so khong hop le" } };
  }
  try {
    const ketQua = parsed.method === "cong" ? parsed.params.a + parsed.params.b : thucThiChiaThat(parsed.params.a, parsed.params.b);
    return { jsonrpc: "2.0", id: parsed.id, result: ketQua };
  } catch (e) {
    return { jsonrpc: "2.0", id: parsed.id, error: { code: -32603, message: "Internal error", data: (e as Error).message } };
  }
}

const ketQuaChia = xuLyYeuCauDayDu('{"jsonrpc":"2.0","id":7,"method":"chia","params":{"a":10,"b":2}}');
console.log(JSON.stringify(ketQuaChia));
```

```typescript title=test
if (!("result" in ketQuaChia)) throw new Error("chia 10 cho 2 phai tra ve result, khong phai error");
if ((ketQuaChia as { result: number }).result !== 5) throw new Error("10 / 2 phai bang 5");
if (ketQuaChia.id !== 7) throw new Error("id cua response phai khop id cua request (7)");

const parseError = xuLyYeuCauDayDu("{a:1");
if (!("error" in parseError)) throw new Error("JSON hong phai tra ve error");
if ((parseError as { error: { code: number } }).error.code !== -32700) throw new Error("JSON hong phai co error.code la -32700");
if (parseError.id !== null) throw new Error("loi parse error phai co id la null (chua doc duoc id nao)");

const invalidRequest = xuLyYeuCauDayDu('{"id":3,"method":"cong","params":{"a":1,"b":2}}');
if (!("error" in invalidRequest)) throw new Error("thieu jsonrpc phai tra ve error");
if ((invalidRequest as { error: { code: number } }).error.code !== -32600) throw new Error("thieu jsonrpc phai co error.code la -32600");
if (invalidRequest.id !== 3) throw new Error("id van doc duoc (3) du thieu jsonrpc");

const noiBoLoi = xuLyYeuCauDayDu('{"jsonrpc":"2.0","id":6,"method":"chia","params":{"a":10,"b":0}}');
if (!("error" in noiBoLoi)) throw new Error("chia cho 0 phai tra ve error");
if ((noiBoLoi as { error: { code: number } }).error.code !== -32603) throw new Error("chia cho 0 phai co error.code la -32603");
if ((noiBoLoi as { error: { data?: string } }).error.data !== "chia cho 0") throw new Error("error.data phai la message goc cua exception ('chia cho 0')");
if (demGoiChiaThat.soLan !== 2) throw new Error("thucThiChiaThat phai duoc goi DUNG 2 lan (id=6 va id=7)");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (laYeuCauHopLe): bon dieu kien lien tiep -- la object khac null, jsonrpc dung '2.0', method la string, id la string hoac number. Cho hai (xuLyYeuCauDayDu): NAM buoc theo thu tu -- try/catch quanh JSON.parse, kiem hinh dang, kiem method, kiem tham so, roi try/catch quanh phan tinh toan."
- kind: strategy
  body: "Cho dau: if (typeof parsed !== 'object' || parsed === null) return false; const p = parsed as Record<string, unknown>; if (p['jsonrpc'] !== '2.0') return false; if (typeof p['method'] !== 'string') return false; if (typeof p['id'] !== 'string' && typeof p['id'] !== 'number') return false; return true; Cho hai: let parsed: unknown; try { parsed = JSON.parse(raw); } catch { return { jsonrpc: '2.0', id: null, error: { code: -32700, message: 'Parse error' } }; } if (!laYeuCauHopLe(parsed)) return { jsonrpc: '2.0', id: layIdNeuCo(parsed), error: { code: -32600, message: 'Invalid Request' } }; if (parsed.method !== 'cong' && parsed.method !== 'chia') return { jsonrpc: '2.0', id: parsed.id, error: { code: -32601, message: `method khong ton tai: ${parsed.method}` } }; if (!laThamSoHopLe(parsed.params)) return { jsonrpc: '2.0', id: parsed.id, error: { code: -32602, message: 'tham so khong hop le' } }; try { const ketQua = parsed.method === 'cong' ? parsed.params.a + parsed.params.b : thucThiChiaThat(parsed.params.a, parsed.params.b); return { jsonrpc: '2.0', id: parsed.id, result: ketQua }; } catch (e) { return { jsonrpc: '2.0', id: parsed.id, error: { code: -32603, message: 'Internal error', data: (e as Error).message } }; }"
- kind: one-line
  body: "Sao chep dung hai khoi o phan Strategy vao dung vi tri tuong ung, GIU NGUYEN thu tu nam buoc trong xuLyYeuCauDayDu."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "{\"jsonrpc\":\"2.0\",\"id\":7,\"result\":5}"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Năm mã lỗi, một hàng rào — VÀ không exception nào rò ra ngoài. Bài sau
mở rộng MCP sang một khả năng q9.6a chưa hề đụng tới: `resources` — đọc
dữ liệu, không phải gọi hành động.
::::

::::reflect{#nghi-lai}
Ba mã lỗi mới Ở bài này xếp thành đúng BA tầng: `-32700` xảy ra TRƯỚC
KHI có bất kỳ object nào tồn tại (còn LÀ một chuỗi thô); `-32600` xảy ra
SAU KHI có object NHƯNG hình dạng sai; `-32603` xảy ra SAU KHI mọi thứ Ở
tầng GIAO THỨC đều đúng — lỗi nằm Ở TẦNG NGHIỆP VỤ, bên trong chính tool.
Thứ tự kiểm (`parse` → `hình dạng` → `method` → `tham số` → `thực thi`)
không phải tuỳ ý: mỗi bước sau CHỈ có ý nghĩa khi bước trước đã qua —
đọc `method` của một request KHÔNG hợp lệ hình dạng LÀ vô nghĩa, y hệt
gọi tool THẬT trước khi biết tham số có đúng kiểu hay không.
::::

::::checkpoint{mastery=0.86}
::::
