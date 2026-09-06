---
id: ky-nghe-ung-dung-ai.giao-thuc-mcp-a2a.json-rpc-la-gi-va-vi-sao-can-chuan-hoa
title: "JSON-RPC 2.0 là gì, và vì sao cần chuẩn hoá"
summary: "xuLyYeuCauJsonRpc(req) nhận JsonRpcRequest<{a,b}> và trả JsonRpcResponse<number> đúng MỘT trong hai hình dạng: {jsonrpc:\"2.0\",id,result} khi method là \"cong\" và tham số hợp lệ (qua type-guard laThamSoHopLe), hoặc {jsonrpc:\"2.0\",id,error:{code,message}} khi method không tồn tại (code -32601) hoặc tham số sai kiểu/thiếu (code -32602). Trên ba lời gọi id=1 (cong 3 và 4), id=2 (method 'tru' không tồn tại), id=3 (thiếu tham số b): kết quả lần lượt result=7, error.code=-32601, error.code=-32602 — id của response LUÔN khớp đúng id của request tương ứng dù nội dung hoàn toàn khác nhau. Đối chiếu ToolMoPhong.goi() ở T9.3 (trả thẳng KetQuaGoiTool {thanhCong,giaTri|loi}, không có jsonrpc, không có id): JSON-RPC thêm đúng hai thứ ToolMoPhong không có — id để khớp request↔response qua một kênh không giữ thứ tự, và error{code,message} tách lỗi TẦNG GIAO THỨC khỏi lỗi nghiệp vụ."
locale: vi
track: ky-nghe-ung-dung-ai
module: giao-thuc-mcp-a2a
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [kna.json-rpc-la-gi-va-vi-sao-can-chuan-hoa]
requires: [kna.boss-do-thi-production-grade-vs-don-gian]
concepts: [kna.json-rpc-la-gi-va-vi-sao-can-chuan-hoa]
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
Năm track vừa qua, agent gọi tool bằng hàm TypeScript ngay trong CÙNG một
tiến trình — gọi xong biết ngay kết quả, không cần tên nào khớp với ai.
Track này bắt đầu một chuyện khác: khi agent VÀ tool nằm Ở HAI TIẾN TRÌNH
riêng, gửi qua lại từng gói tin, làm sao BIẾT được gói trả lời NÀO ứng
với gói hỏi NÀO? JSON-RPC LÀ câu trả lời — nền của MCP mà cả track này
sẽ xây trên đó.
::::

::::explain{#hinh_dang_json_rpc}
Một lời gọi JSON-RPC 2.0 LÀ một object CÓ ĐÚNG bốn trường: `jsonrpc`
(luôn LÀ chuỗi `"2.0"` — số phiên bản GIAO THỨC, không phải phiên bản
ứng dụng), `id` (định danh của LẦN GỌI này), `method` (tên hành động
muốn gọi), VÀ `params` (đối số, tuỳ chọn). Phản hồi LUÔN LÀ MỘT trong
HAI hình dạng — CÓ `result` (thành công) HOẶC CÓ `error` (thất bại Ở
TẦNG GIAO THỨC), KHÔNG BAO GIỜ có cả hai, VÀ KHÔNG BAO GIỜ thiếu cả hai:

```typescript title=readonly
type JsonRpcRequest<TParams> = {
  jsonrpc: "2.0";
  id: string | number;
  method: string;
  params?: TParams;
};

type JsonRpcError = {
  code: number;
  message: string;
};

type JsonRpcResponse<TResult> =
  | { jsonrpc: "2.0"; id: string | number; result: TResult }
  | { jsonrpc: "2.0"; id: string | number | null; error: JsonRpcError };

const yeuCau: JsonRpcRequest<{ a: number; b: number }> = {
  jsonrpc: "2.0",
  id: 1,
  method: "cong",
  params: { a: 3, b: 4 },
};

const phanHoiOk: JsonRpcResponse<number> = { jsonrpc: "2.0", id: 1, result: 7 };
const phanHoiLoi: JsonRpcResponse<number> = {
  jsonrpc: "2.0",
  id: 1,
  error: { code: -32601, message: "method khong ton tai" },
};
console.log(JSON.stringify(yeuCau));
console.log(JSON.stringify(phanHoiOk));
console.log(JSON.stringify(phanHoiLoi));
```

```text title=readonly
{"jsonrpc":"2.0","id":1,"method":"cong","params":{"a":3,"b":4}}
{"jsonrpc":"2.0","id":1,"result":7}
{"jsonrpc":"2.0","id":1,"error":{"code":-32601,"message":"method khong ton tai"}}
```

So với `ToolMoPhong.goi()` Ở T9.3 (bài `1`, module `ky-thuat-harness`) —
hàm đó trả THẲNG `KetQuaGoiTool { thanhCong: true; giaTri } | { thanhCong:
false; loi }`, không có `jsonrpc`, không có `id` — JSON-RPC không thay
đổi Ý TƯỞNG "thành công hoặc thất bại". Nó thêm đúng HAI thứ: một `id` để
khớp lại đúng request VỚI đúng response (`ToolMoPhong` không cần vì gọi
hàm VÀ nhận kết quả xảy ra NGAY LẬP TỨC, cùng một chỗ trong mã — không hề
có khoảng trống nào để hai lời gọi bị LẪN VÀO NHAU), VÀ một hình dạng
`error` CHUẨN (`code` SỐ, `message` chuỗi) tách biệt khỏi bất kỳ lỗi
NGHIỆP VỤ nào tool có thể trả trong chính `result` của nó.
::::

::::example{#xu_ly_yeu_cau_json_rpc}
`xuLyYeuCauJsonRpc` LÀ một "server" tối giản: nó hiểu ĐÚNG MỘT method
(`"cong"`), kiểm tham số bằng type-guard `laThamSoHopLe`, VÀ trả về ĐÚNG
hình dạng `JsonRpcResponse` cho mọi trường hợp — kể cả khi thất bại:

```typescript title=readonly
type JsonRpcRequest<TParams> = {
  jsonrpc: "2.0";
  id: string | number;
  method: string;
  params?: TParams;
};

type JsonRpcError = { code: number; message: string };

type JsonRpcResponse<TResult> =
  | { jsonrpc: "2.0"; id: string | number; result: TResult }
  | { jsonrpc: "2.0"; id: string | number | null; error: JsonRpcError };

function laThamSoHopLe(params: unknown): params is { a: number; b: number } {
  if (typeof params !== "object" || params === null) return false;
  const p = params as Record<string, unknown>;
  return typeof p["a"] === "number" && typeof p["b"] === "number";
}

function xuLyYeuCauJsonRpc(
  req: JsonRpcRequest<{ a: number; b: number }>,
): JsonRpcResponse<number> {
  if (req.method !== "cong") {
    return {
      jsonrpc: "2.0",
      id: req.id,
      error: { code: -32601, message: `method khong ton tai: ${req.method}` },
    };
  }
  if (!laThamSoHopLe(req.params)) {
    return { jsonrpc: "2.0", id: req.id, error: { code: -32602, message: "tham so khong hop le" } };
  }
  return { jsonrpc: "2.0", id: req.id, result: req.params.a + req.params.b };
}

const ok = xuLyYeuCauJsonRpc({ jsonrpc: "2.0", id: 1, method: "cong", params: { a: 3, b: 4 } });
const saiMethod = xuLyYeuCauJsonRpc({ jsonrpc: "2.0", id: 2, method: "tru", params: { a: 3, b: 4 } });
console.log(JSON.stringify(ok));
console.log(JSON.stringify(saiMethod));
```

```text title=readonly
{"jsonrpc":"2.0","id":1,"result":7}
{"jsonrpc":"2.0","id":2,"error":{"code":-32601,"message":"method khong ton tai: tru"}}
```

`ok` VÀ `saiMethod` LÀ hai response HOÀN TOÀN khác hình dạng (một có
`result`, một có `error`) — nhưng cả hai VẪN LÀ `JsonRpcResponse` hợp lệ,
VÀ `id` của mỗi response (`1` VÀ `2`) khớp ĐÚNG `id` của request đã gửi
nó. Đó chính LÀ điều `ToolMoPhong` không làm được: nếu hai lời gọi Ở hai
NƠI khác nhau (qua mạng, qua hàng đợi) trả lời KHÔNG theo đúng thứ tự đã
gửi, `id` LÀ CÁCH DUY NHẤT biết response nào ứng với request nào.
::::

::::predict{#doan-id-khop-request commitOnce}
Gọi `xuLyYeuCauJsonRpc` hai lần, CÙNG method `"cong"`, CÙNG params
`{a:2,b:2}`, chỉ khác `id`: `{id:10,...}` VÀ `{id:20,...}`. Hai response
trả về chắc chắn GIỐNG HỆT nhau Ở trường nào, VÀ khác nhau Ở trường nào?

:::opt{correct}
`jsonrpc` VÀ `result` giống hệt nhau (`"2.0"` VÀ `4`); CHỈ `id` khác nhau
(`10` so với `20`) — vì `xuLyYeuCauJsonRpc` LUÔN gán `id: req.id` vào
response, bất kể nội dung tính toán bên trong có giống hệt request khác
hay không
:::
:::opt
Cả ba trường đều giống hệt nhau, vì `params` giống hệt nhau nên response
cũng phải giống hệt nhau hoàn toàn
::why
Nhầm rằng response không phụ thuộc gì vào `id` của chính request đã gửi
nó — nhưng `xuLyYeuCauJsonRpc` luôn gán `id: req.id` vào MỌI nhánh trả
về (cả `result` lẫn `error`).

Chỗ lệch: hai request có `id` khác nhau (`10` VÀ `20`) LUÔN sinh ra hai
response có `id` khác nhau, dù `params` VÀ `result` bên trong giống hệt
nhau tuyệt đối.
::
:::
:::opt
Cả `id` LẪN `result` đều khác nhau, vì mỗi lần gọi hàm LÀ một lần tính
toán MỚI nên kết quả không lặp lại
::why
Nhầm rằng có yếu tố ngẫu nhiên nào đó trong hàm — nhưng `xuLyYeuCauJsonRpc`
hoàn toàn tất định: không `Math.random()`, không `Date.now()`.

Chỗ lệch: CÙNG `params` `{a:2,b:2}` LUÔN cho `result` LÀ `4`, bất kể
`id` của request LÀ gì — chỉ `id` phản ánh lại đúng request nào đã gửi
nó, `result` thì không.
::
:::
::::

::::code{#viet_xu_ly_yeu_cau_json_rpc}
Hoàn thiện `laThamSoHopLe` — type-guard kiểm `params` CÓ phải một object
khác `null`, VÀ CÓ đúng hai khoá `a`/`b` kiểu `number` hay không. Hoàn
thiện `xuLyYeuCauJsonRpc` — NẾU `req.method` khác `"cong"`, trả `error`
mã `-32601`; NẾU `laThamSoHopLe(req.params)` LÀ `false`, trả `error` mã
`-32602`; NGƯỢC LẠI trả `result` LÀ `req.params.a + req.params.b`. MỌI
nhánh đều phải gán ĐÚNG `id: req.id`.

```typescript title=starter
type JsonRpcRequest<TParams> = {
  jsonrpc: "2.0";
  id: string | number;
  method: string;
  params?: TParams;
};

type JsonRpcError = { code: number; message: string };

type JsonRpcResponse<TResult> =
  | { jsonrpc: "2.0"; id: string | number; result: TResult }
  | { jsonrpc: "2.0"; id: string | number | null; error: JsonRpcError };

function laThamSoHopLe(params: unknown): params is { a: number; b: number } {
  ___
}

function xuLyYeuCauJsonRpc(
  req: JsonRpcRequest<{ a: number; b: number }>,
): JsonRpcResponse<number> {
  ___
}

const ketQua1 = xuLyYeuCauJsonRpc({ jsonrpc: "2.0", id: 1, method: "cong", params: { a: 3, b: 4 } });
console.log(JSON.stringify(ketQua1));
```

```typescript title=solution
type JsonRpcRequest<TParams> = {
  jsonrpc: "2.0";
  id: string | number;
  method: string;
  params?: TParams;
};

type JsonRpcError = { code: number; message: string };

type JsonRpcResponse<TResult> =
  | { jsonrpc: "2.0"; id: string | number; result: TResult }
  | { jsonrpc: "2.0"; id: string | number | null; error: JsonRpcError };

function laThamSoHopLe(params: unknown): params is { a: number; b: number } {
  if (typeof params !== "object" || params === null) return false;
  const p = params as Record<string, unknown>;
  return typeof p["a"] === "number" && typeof p["b"] === "number";
}

function xuLyYeuCauJsonRpc(
  req: JsonRpcRequest<{ a: number; b: number }>,
): JsonRpcResponse<number> {
  if (req.method !== "cong") {
    return {
      jsonrpc: "2.0",
      id: req.id,
      error: { code: -32601, message: `method khong ton tai: ${req.method}` },
    };
  }
  if (!laThamSoHopLe(req.params)) {
    return { jsonrpc: "2.0", id: req.id, error: { code: -32602, message: "tham so khong hop le" } };
  }
  return { jsonrpc: "2.0", id: req.id, result: req.params.a + req.params.b };
}

const ketQua1 = xuLyYeuCauJsonRpc({ jsonrpc: "2.0", id: 1, method: "cong", params: { a: 3, b: 4 } });
console.log(JSON.stringify(ketQua1));
```

```typescript title=test
if (!("result" in ketQua1)) throw new Error("cong 3 va 4 phai tra ve result, khong phai error");
if (ketQua1.jsonrpc !== "2.0") throw new Error("jsonrpc phai la '2.0'");
if (ketQua1.id !== 1) throw new Error("id cua response phai KHOP dung id cua request (1)");
if ((ketQua1 as { result: number }).result !== 7) throw new Error("3 + 4 phai bang 7");

const ketQuaSaiMethod = xuLyYeuCauJsonRpc({ jsonrpc: "2.0", id: 2, method: "tru", params: { a: 3, b: 4 } });
if (!("error" in ketQuaSaiMethod)) throw new Error("method 'tru' khong ton tai, phai tra ve error");
if (ketQuaSaiMethod.id !== 2) throw new Error("id cua response loi van phai KHOP id cua request (2)");
if ((ketQuaSaiMethod as { error: { code: number } }).error.code !== -32601) {
  throw new Error("method khong ton tai phai co error.code la -32601");
}

const thamSoThieu = { a: 3 } as unknown as { a: number; b: number };
const ketQuaSaiThamSo = xuLyYeuCauJsonRpc({ jsonrpc: "2.0", id: 3, method: "cong", params: thamSoThieu });
if (!("error" in ketQuaSaiThamSo)) throw new Error("thieu tham so b phai tra ve error, khong duoc tra ve result");
if ((ketQuaSaiThamSo as { error: { code: number } }).error.code !== -32602) {
  throw new Error("tham so khong hop le phai co error.code la -32602");
}
if (ketQuaSaiThamSo.id !== 3) throw new Error("id cua response loi tham so van phai KHOP id cua request (3)");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (laThamSoHopLe): kiem typeof params co phai 'object' VA khac null, roi ep kieu de doc hai khoa a/b. Cho hai (xuLyYeuCauJsonRpc): ba nhanh theo thu tu -- method sai, tham so sai, roi moi tinh ket qua."
- kind: strategy
  body: "Cho dau: if (typeof params !== 'object' || params === null) return false; const p = params as Record<string, unknown>; return typeof p['a'] === 'number' && typeof p['b'] === 'number'; Cho hai: if (req.method !== 'cong') return { jsonrpc: '2.0', id: req.id, error: { code: -32601, message: `method khong ton tai: ${req.method}` } }; if (!laThamSoHopLe(req.params)) return { jsonrpc: '2.0', id: req.id, error: { code: -32602, message: 'tham so khong hop le' } }; return { jsonrpc: '2.0', id: req.id, result: req.params.a + req.params.b };"
- kind: one-line
  body: "Sao chep dung hai khoi o phan Strategy vao dung vi tri tuong ung, GIU NGUYEN thu tu ba nhanh trong xuLyYeuCauJsonRpc."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":7}"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba lời gọi, ba `id`, ba hình dạng response khác nhau — nhưng CẢ ba đều
LÀ `JsonRpcResponse` hợp lệ, VÀ `id` LUÔN khớp đúng. Đây LÀ nền của MCP:
mọi method (`"initialize"`, `"tools/call"`, …) mà track này sắp học đều
đi qua ĐÚNG lớp JSON-RPC này trước.
::::

::::reflect{#nghi-lai}
`ToolMoPhong` Ở T9.3 dạy đúng một Ý TƯỞNG — "thành công hoặc thất bại LÀ
một giá trị" — trong bối cảnh MỘT tiến trình gọi một hàm rồi nhận kết
quả NGAY LẬP TỨC. JSON-RPC không thay Ý TƯỞNG đó — nó thêm đúng những gì
cần Ở bối cảnh KHÁC: khi lời gọi VÀ phản hồi phải đi qua một kênh CÓ THỂ
không giữ đúng thứ tự (`id` để khớp lại), VÀ khi nhiều bên khác nhau
cùng đọc chung một giao thức nên lỗi TẦNG GIAO THỨC (method sai, tham số
sai) cần một hình dạng CHUẨN, tách biệt khỏi bất kỳ lỗi nghiệp vụ nào
tool tự định nghĩa bên trong `result` của nó.
::::

::::checkpoint{mastery=0.85}
::::
