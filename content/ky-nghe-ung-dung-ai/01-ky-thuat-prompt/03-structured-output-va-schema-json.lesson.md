---
id: ky-nghe-ung-dung-ai.ky-thuat-prompt.structured-output-va-schema-json
title: "Structured output: bắt lỗi JSON không khớp schema bằng TypeScript"
summary: "interface KetQuaPhanLoai { nhiemVu: \"tom_tat\"|\"dich\"|\"dem_so\"; doTinCay: number } dinh nghia CHINH XAC hinh dang response mong doi. goiModelJSON(messages) mo phong model tra ve CHUOI (khong phai object): neu prompt CHI RO 'tra ve JSON voi khoa nhiemVu va doTinCay' (kiem tra tren system/user, KHONG tren cac message assistant la vi du mau) thi tra JSON DUNG schema; neu CHI nhac 'json' chung chung thi tra JSON nhung SAI ten khoa; nguoc lai tra PROSE tu nhien. thuParseKetQua(chuoi): KetQuaPhanLoai | null dung JSON.parse trong try/catch + type-guard rieng (khong dung any) de kiem tra dung hinh dang runtime. Tren bo 6 prompt: KHONG chi ro schema, ty le parse thanh cong = 0/6; CO chi ro schema, ty le = 6/6."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-prompt
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [kna.structured-output-va-schema-json]
requires: [kna.rang-buoc-ro-rang-sua-mo-ho]
concepts: [kna.structured-output-va-schema-json]
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
Hai bài trước đo ĐÚNG một trục: model có phân loại đúng LOẠI nhiệm vụ
không. Nhưng một hệ thống production không chỉ ĐỌC câu trả lời bằng
mắt — nó ĐỌC bằng CODE, thường LÀ `JSON.parse`. Python (Realm 8) không
có cách nào SẠCH để định nghĩa "hình dạng response mong đợi" bằng
chính ngôn ngữ — TypeScript thì có: `interface`.
::::

::::explain{#interface-la-schema}
`interface KetQuaPhanLoai` định nghĩa CHÍNH XÁC hình dạng một response
hợp lệ: đúng hai trường, `nhiemVu` LÀ một trong ba giá trị cố định,
`doTinCay` LÀ `number`. Đây LÀ "schema" — nhưng viết bằng chính hệ
thống kiểu của TypeScript, không phải một tài liệu mô tả riêng biệt dễ
lệch khỏi code thật.

`goiModelJSON` mô phỏng một model LUÔN trả về CHUỖI (giống hệt LLM
thật — không có API nào trả thẳng một object JavaScript, luôn LÀ text)
— RULE-BASED theo BA mức chỉ thị: nếu prompt CHỈ RÕ đúng tên hai khoá
(`"nhiemvu"` VÀ `"dotincay"`, không phân biệt hoa/thường), trả JSON
ĐÚNG schema; nếu CHỈ nhắc chung chung tới `"json"` (không nêu tên
khoá), trả JSON nhưng dùng SAI tên khoá; nếu KHÔNG nhắc gì tới JSON,
trả một câu PROSE tự nhiên. Cả ba mức CHỈ soi vào message `system`/
`user` — KHÔNG soi vào message `assistant` (ví dụ mẫu, sẽ dùng Ở bài
sau), tránh model "tự khớp" schema chỉ vì đọc lại chính câu trả lời mẫu
của nó:

```typescript title=readonly
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}

function layNoiDungUserCuoi(messages: ChatMessage[]): string {
  let noiDung = "";
  for (const tin of messages) {
    if (tin.role === "user") noiDung = tin.content;
  }
  return noiDung;
}

interface KetQuaPhanLoai {
  nhiemVu: "tom_tat" | "dich" | "dem_so";
  doTinCay: number;
}

function phanLoaiNhiemVuDonGian(prompt: string): "tom_tat" | "dich" | "dem_so" {
  const p = prompt.toLowerCase();
  if (p.includes("dich")) return "dich";
  if (p.includes("dem") || p.includes("tong")) return "dem_so";
  return "tom_tat";
}

function doTinCayGiaLap(prompt: string): number {
  const p = prompt.toLowerCase();
  if (p.includes("dich") || p.includes("dem") || p.includes("tong") || p.includes("tom tat")) return 0.9;
  return 0.5;
}

function coChiRoSchemaJson(messages: ChatMessage[]): boolean {
  const noi = messages
    .filter((m) => m.role !== "assistant")
    .map((m) => m.content.toLowerCase())
    .join(" ");
  return noi.includes("nhiemvu") && noi.includes("dotincay");
}

function coNhacToiJsonChungChung(messages: ChatMessage[]): boolean {
  const noi = messages
    .filter((m) => m.role !== "assistant")
    .map((m) => m.content.toLowerCase())
    .join(" ");
  return noi.includes("json");
}

function goiModelJSON(messages: ChatMessage[]): string {
  const prompt = layNoiDungUserCuoi(messages);
  const nhiemVu = phanLoaiNhiemVuDonGian(prompt);
  const doTinCay = doTinCayGiaLap(prompt);
  if (coChiRoSchemaJson(messages)) {
    return JSON.stringify({ nhiemVu, doTinCay });
  }
  if (coNhacToiJsonChungChung(messages)) {
    return JSON.stringify({ loaiNhiemVu: nhiemVu, mucTinCay: doTinCay });
  }
  return `Toi nghi day la nhiem vu ${nhiemVu}, do tin cay khoang ${doTinCay}.`;
}

const msgsKhong: ChatMessage[] = [{ role: "user", content: "Hay tom tat doan van nay" }];
const msgsSchema: ChatMessage[] = [
  { role: "system", content: "Tra loi bang JSON co dung hai khoa nhiemVu va doTinCay, khong viet gi them." },
  { role: "user", content: "Hay tom tat doan van nay" },
];

console.log("khong chi ro schema:", goiModelJSON(msgsKhong));
console.log("co chi ro schema:", goiModelJSON(msgsSchema));
```

```text title=readonly
khong chi ro schema: Toi nghi day la nhiem vu tom_tat, do tin cay khoang 0.9.
co chi ro schema: {"nhiemVu":"tom_tat","doTinCay":0.9}
```

CÙNG một câu hỏi thật (`"Hay tom tat doan van nay"`), hai chuỗi trả về
KHÁC hẳn hình dạng: một câu PROSE tự nhiên (không có cách nào
`JSON.parse` được), một chuỗi JSON hợp lệ đúng hai khoá `nhiemVu`/
`doTinCay`. Sự khác biệt DUY NHẤT LÀ có hay không một chỉ thị chỉ RÕ
tên hai khoá.
::::

::::example{#thu-parse-va-do-ty-le}
`thuParseKetQua` là hàng rào ĐỌC AN TOÀN: gọi `JSON.parse` trong
`try/catch` (chuỗi PROSE làm nó ném `SyntaxError`, bị bắt VÀ trả về
`null`), rồi kiểm tra ĐÚNG hình dạng bằng một type-guard RIÊNG — không
dùng `any` Ở bất kỳ đâu:

```typescript title=readonly
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}

interface KetQuaPhanLoai {
  nhiemVu: "tom_tat" | "dich" | "dem_so";
  doTinCay: number;
}

function laNhiemVuHopLe(x: unknown): x is "tom_tat" | "dich" | "dem_so" {
  return x === "tom_tat" || x === "dich" || x === "dem_so";
}

function thuParseKetQua(chuoi: string): KetQuaPhanLoai | null {
  let giaTri: unknown;
  try {
    giaTri = JSON.parse(chuoi);
  } catch {
    return null;
  }
  if (typeof giaTri !== "object" || giaTri === null) return null;
  const ghi = giaTri as Record<string, unknown>;
  if (laNhiemVuHopLe(ghi.nhiemVu) && typeof ghi.doTinCay === "number") {
    return { nhiemVu: ghi.nhiemVu, doTinCay: ghi.doTinCay };
  }
  return null;
}

console.log("chuoi prose:", thuParseKetQua("Toi nghi day la tom_tat."));
console.log("chuoi 'null':", thuParseKetQua("null"));
console.log("sai ten khoa:", thuParseKetQua(JSON.stringify({ loaiNhiemVu: "dich", mucTinCay: 0.9 })));
console.log("dung schema:", thuParseKetQua(JSON.stringify({ nhiemVu: "dich", doTinCay: 0.9 })));
```

```text title=readonly
chuoi prose: null
chuoi 'null': null
sai ten khoa: null
dung schema: {"nhiemVu":"dich","doTinCay":0.9}
```

Bốn chuỗi ĐẦU vào, ba giá trị `null`: chuỗi PROSE làm `JSON.parse` ném
lỗi ngay từ đầu; chuỗi `"null"` LÀ JSON HỢP LỆ (parse thành công) NHƯNG
giá trị parse ra chính LÀ `null` — nếu KHÔNG kiểm tra `giaTri === null`
trước khi ép kiểu, dòng `ghi.nhiemVu` sẽ ném lỗi runtime; chuỗi thứ ba
parse THÀNH object nhưng dùng TÊN khoá khác (`loaiNhiemVu` thay vì
`nhiemVu`) — vượt qua `JSON.parse` nhưng bị type-guard chặn Ở bước
kiểm tra HÌNH DẠNG. Chỉ chuỗi ĐÚNG cả hai khoá LẪN đúng kiểu mới lọt
qua.
::::

::::predict{#doan-mang-json commitOnce}
Gọi `thuParseKetQua("[1,2,3]")` — một mảng JSON HỢP LỆ (parse được
bằng `JSON.parse`), nhưng KHÔNG có trường `nhiemVu` hay `doTinCay` nào.
Kết quả LÀ gì?

:::opt{correct}
`null` — mảng vượt qua `typeof giaTri !== "object"` (mảng CŨNG LÀ
`"object"` trong JavaScript) VÀ vượt qua `giaTri === null` (mảng không
phải `null`), nhưng `ghi.nhiemVu` VÀ `ghi.doTinCay` đều LÀ `undefined`
trên một mảng — `laNhiemVuHopLe(undefined)` cho `false`, hàm rơi vào
nhánh cuối, trả về `null`
:::
:::opt
Hàm sẽ ném lỗi runtime, vì mảng KHÔNG có các thuộc tính `nhiemVu`/
`doTinCay` để truy cập
::why
Gần đúng Ở việc bạn nhận ra mảng THIẾU hai trường mong đợi — quan sát
đó đúng.

Chỗ lệch: truy cập một thuộc tính KHÔNG tồn tại trên object (hay mảng)
trong JavaScript KHÔNG ném lỗi — nó trả về `undefined`. `ghi.nhiemVu`
trên mảng `[1,2,3]` đơn giản LÀ `undefined`, VÀ `laNhiemVuHopLe`
kiểm tra `undefined` một cách AN TOÀN, trả `false`, không hề crash.
::
:::
:::opt
`{ nhiemVu: undefined, doTinCay: undefined }` — hàm vẫn tạo VÀ trả về
một `KetQuaPhanLoai`, chỉ Là hai trường của nó ĐANG rỗng
::why
Gần đúng Ở việc bạn nhớ ĐÚNG rằng truy cập thuộc tính thiếu cho ra
`undefined`, không crash.

Chỗ lệch: hàm không hề trả về một object CÓ trường rỗng — điều kiện
`if (laNhiemVuHopLe(ghi.nhiemVu) && typeof ghi.doTinCay === "number")`
kiểm tra TRƯỚC khi tạo kết quả, VÀ điều kiện đó LÀ `false` (vì
`undefined` không nằm trong ba giá trị hợp lệ). Nhánh `if` không chạy,
hàm rơi thẳng xuống `return null;` Ở cuối — không hề có object nào
được tạo ra với trường rỗng.
::
:::
::::

::::code{#viet_thu_parse_ket_qua}
Hoàn thiện `thuParseKetQua` — bắt lỗi `JSON.parse` bằng `try/catch`
(trả về `null` nếu ném lỗi), rồi kiểm tra ĐÚNG hình dạng: phải LÀ một
object khác `null`, có `nhiemVu` hợp lệ (dùng `laNhiemVuHopLe`) VÀ
`doTinCay` LÀ `number`.

```typescript title=starter
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}

interface KetQuaPhanLoai {
  nhiemVu: "tom_tat" | "dich" | "dem_so";
  doTinCay: number;
}

function laNhiemVuHopLe(x: unknown): x is "tom_tat" | "dich" | "dem_so" {
  return x === "tom_tat" || x === "dich" || x === "dem_so";
}

function thuParseKetQua(chuoi: string): KetQuaPhanLoai | null {
  ___
  ___
}

const chuoiX = JSON.stringify({ nhiemVu: "dem_so", doTinCay: 0.9 });
console.log(thuParseKetQua(chuoiX));
```

```typescript title=solution
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}

interface KetQuaPhanLoai {
  nhiemVu: "tom_tat" | "dich" | "dem_so";
  doTinCay: number;
}

function laNhiemVuHopLe(x: unknown): x is "tom_tat" | "dich" | "dem_so" {
  return x === "tom_tat" || x === "dich" || x === "dem_so";
}

function thuParseKetQua(chuoi: string): KetQuaPhanLoai | null {
  let giaTri: unknown;
  try {
    giaTri = JSON.parse(chuoi);
  } catch {
    return null;
  }
  if (typeof giaTri !== "object" || giaTri === null) return null;
  const ghi = giaTri as Record<string, unknown>;
  if (laNhiemVuHopLe(ghi.nhiemVu) && typeof ghi.doTinCay === "number") {
    return { nhiemVu: ghi.nhiemVu, doTinCay: ghi.doTinCay };
  }
  return null;
}

const chuoiX = JSON.stringify({ nhiemVu: "dem_so", doTinCay: 0.9 });
console.log(thuParseKetQua(chuoiX));
```

```typescript title=test
const okX = thuParseKetQua(chuoiX);
if (okX === null || okX.nhiemVu !== "dem_so" || okX.doTinCay !== 0.9) throw new Error("JSON hop le phai parse dung ca hai khoa");

if (thuParseKetQua("khong phai JSON gi ca") !== null) throw new Error("chuoi khong phai JSON phai tra ve null (JSON.parse nem loi, bat bang catch)");
if (thuParseKetQua('{"loaiNhiemVu":"dich","mucTinCay":0.9}') !== null) throw new Error("JSON dung nhung SAI ten khoa phai tra ve null");
if (thuParseKetQua('{"nhiemVu":"linh_tinh","doTinCay":0.9}') !== null) throw new Error("nhiemVu ngoai ba gia tri hop le phai tra ve null");
if (thuParseKetQua('{"nhiemVu":"tom_tat","doTinCay":"cao"}') !== null) throw new Error("doTinCay khong phai number phai tra ve null");
if (thuParseKetQua("[1,2,3]") !== null) throw new Error("mot mang JSON hop le nhung khong co hai khoa can thiet phai tra ve null");
if (thuParseKetQua("null") !== null) throw new Error("chuoi 'null' parse thanh JS null, phai duoc nhan dien va tra ve null, khong duoc nem loi runtime");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau: khai bao let giaTri: unknown; roi try { giaTri = JSON.parse(chuoi); } catch { return null; } -- bat loi parse. Cho hai: kiem tra typeof giaTri !== 'object' hoac giaTri === null thi return null; sau do ep kieu giaTri as Record<string, unknown>, kiem tra laNhiemVuHopLe(ghi.nhiemVu) VA typeof ghi.doTinCay === 'number' truoc khi tra ve ket qua, nguoc lai return null."
- kind: strategy
  body: "Cho dau: let giaTri: unknown; try { giaTri = JSON.parse(chuoi); } catch { return null; } Cho hai: if (typeof giaTri !== \"object\" || giaTri === null) return null; const ghi = giaTri as Record<string, unknown>; if (laNhiemVuHopLe(ghi.nhiemVu) && typeof ghi.doTinCay === \"number\") { return { nhiemVu: ghi.nhiemVu, doTinCay: ghi.doTinCay }; } return null;"
- kind: one-line
  body: "Sao chep dung logic o phan Strategy, DUNG THU TU: try/catch parse truoc, kiem tra hinh dang sau."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "{\"nhiemVu\":\"dem_so\",\"doTinCay\":0.9}"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không chỉ rõ schema: tỉ lệ parse thành công `0/6`. Chỉ rõ schema:
`6/6`. Không cần một LLM thật để thấy structured output THẤT BẠI ra
sao — chỉ cần thiếu MỘT chỉ thị. Nhưng chỉ thị bằng LỜI không phải
cách DUY NHẤT để dạy model một định dạng — bài sau dùng VÍ DỤ MẪU
thay vì lời.
::::

::::reflect{#nghi-lai}
`interface KetQuaPhanLoai` KHÔNG chỉ LÀ một chú thích kiểu cho người
đọc — nó LÀ hợp đồng mà `thuParseKetQua` PHẢI kiểm tra bằng CODE RUNTIME
trước khi tin bất kỳ chuỗi nào từ model. Python không có cách nào viết
type-guard vừa NGẮN vừa được TRÌNH BIÊN DỊCH xác nhận đúng shape như
`x is "tom_tat" | "dich" | "dem_so"` — TypeScript có. Điểm mấu chốt:
"structured output" không phải LÀ hy vọng model trả JSON đúng — nó LÀ
CODE kiểm tra runtime xác nhận điều đó, ĐỘC LẬP với việc model có hợp
tác hay không.
::::

::::checkpoint{mastery=0.80}
::::
