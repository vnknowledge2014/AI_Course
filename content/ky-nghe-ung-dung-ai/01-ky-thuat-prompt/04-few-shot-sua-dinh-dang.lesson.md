---
id: ky-nghe-ung-dung-ai.ky-thuat-prompt.few-shot-sua-dinh-dang
title: "Few-shot examples sửa lỗi định dạng, không cần nói bằng lời"
summary: "demViDuMau(messages: ChatMessage[]) dem so message role=\"assistant\" trong lich su. goiModelJSON_v2 giu nguyen logic bai truoc nhung THEM dieu kien OR: neu coChiRoSchemaJson(messages) HOAC demViDuMau(messages) >= 2 thi tra JSON dung schema -- model \"hoc\" dinh dang tu HAI vi du mau dat TRUOC cau hoi that, du prompt KHONG noi mot chu nao ve JSON. Voi DUNG 1 vi du (duoi nguong), hanh vi CHUA doi -- van la prose. Do lai ty le parse thanh cong tren cung mot prompt: 0 vi du va 1 vi du deu 0/1 (chua dat nguong), 2 vi du dat 1/1 (dat nguong, chuyen sang JSON dung schema)."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-prompt
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [kna.few-shot-sua-dinh-dang]
requires: [kna.structured-output-va-schema-json]
concepts: [kna.few-shot-sua-dinh-dang]
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
Bài trước sửa định dạng bằng LỜI: một chỉ thị hệ thống nêu tên hai
khoá. Nhưng đôi khi bạn KHÔNG MUỐN viết chỉ thị dài dòng — bạn muốn
CHỈ ra bằng ví dụ, giống hệt cách `zero-shot-va-few-shot` (Realm 8) đã
sửa GIÁ TRỊ tính toán. Lần này, few-shot sửa ĐỊNH DẠNG, không sửa giá
trị.
::::

::::explain{#dem-vi-du-mau}
`demViDuMau` đếm số message có `role === "assistant"` trong lịch sử —
mỗi message `assistant` LÀ một "câu trả lời mẫu" đặt TRƯỚC câu hỏi
thật, cho model thấy ĐỊNH DẠNG mong muốn qua VÍ DỤ, không qua lời mô
tả. `goiModelJSON_v2` giữ NGUYÊN toàn bộ logic ba mức của bài trước,
CHỈ THÊM một điều kiện `OR`: nếu ĐÃ chỉ rõ schema BẰNG LỜI **hoặc** đã
CÓ ĐỦ hai ví dụ mẫu trở lên, chuyển sang trả JSON đúng — hai con đường
ĐỘC LẬP dẫn tới CÙNG một hành vi đúng định dạng:

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

function demViDuMau(messages: ChatMessage[]): number {
  return messages.filter((m) => m.role === "assistant").length;
}

function goiModelJSON_v2(messages: ChatMessage[]): string {
  const prompt = layNoiDungUserCuoi(messages);
  const nhiemVu = phanLoaiNhiemVuDonGian(prompt);
  const doTinCay = doTinCayGiaLap(prompt);
  if (coChiRoSchemaJson(messages) || demViDuMau(messages) >= 2) {
    return JSON.stringify({ nhiemVu, doTinCay });
  }
  if (coNhacToiJsonChungChung(messages)) {
    return JSON.stringify({ loaiNhiemVu: nhiemVu, mucTinCay: doTinCay });
  }
  return `Toi nghi day la nhiem vu ${nhiemVu}, do tin cay khoang ${doTinCay}.`;
}

const promptThat = "Cho toi y kien ve bai viet nay";

const khongViDu: ChatMessage[] = [{ role: "user", content: promptThat }];
const haiViDu: ChatMessage[] = [
  { role: "user", content: "Dem so nguoi tham gia buoi hop nay" },
  { role: "assistant", content: JSON.stringify({ nhiemVu: "dem_so", doTinCay: 0.95 }) },
  { role: "user", content: "Dich cau chao mung sang tieng Phap" },
  { role: "assistant", content: JSON.stringify({ nhiemVu: "dich", doTinCay: 0.95 }) },
  { role: "user", content: promptThat },
];

console.log("0 vi du:", demViDuMau(khongViDu), goiModelJSON_v2(khongViDu));
console.log("2 vi du:", demViDuMau(haiViDu), goiModelJSON_v2(haiViDu));
```

```text title=readonly
0 vi du: 0 Toi nghi day la nhiem vu tom_tat, do tin cay khoang 0.5.
2 vi du: 2 {"nhiemVu":"tom_tat","doTinCay":0.5}
```

CÙNG một câu hỏi thật (`promptThat`), KHÔNG hề chứa chữ "json" hay tên
khoá nào — nhưng đặt TRƯỚC nó hai cặp `user`/`assistant` mẫu (mỗi cặp
LÀ một câu hỏi khác cùng CHUỖI JSON đúng định dạng) đủ để đổi hành vi:
từ PROSE tự nhiên sang JSON đúng schema, không cần một lời chỉ thị
nào.
::::

::::example{#nguong-hai-vi-du}
Đúng `1` ví dụ mẫu — dưới ngưỡng `>= 2` — KHÔNG đổi hành vi, giống hệt
tinh thần bài `zero-shot-va-few-shot` Ở Realm 8:

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

function demViDuMau(messages: ChatMessage[]): number {
  return messages.filter((m) => m.role === "assistant").length;
}

function goiModelJSON_v2(messages: ChatMessage[]): string {
  const prompt = layNoiDungUserCuoi(messages);
  const nhiemVu = phanLoaiNhiemVuDonGian(prompt);
  const doTinCay = doTinCayGiaLap(prompt);
  if (coChiRoSchemaJson(messages) || demViDuMau(messages) >= 2) {
    return JSON.stringify({ nhiemVu, doTinCay });
  }
  if (coNhacToiJsonChungChung(messages)) {
    return JSON.stringify({ loaiNhiemVu: nhiemVu, mucTinCay: doTinCay });
  }
  return `Toi nghi day la nhiem vu ${nhiemVu}, do tin cay khoang ${doTinCay}.`;
}

const promptThat = "Cho toi y kien ve bai viet nay";
const motViDu: ChatMessage[] = [
  { role: "user", content: "Dem so nguoi tham gia buoi hop nay" },
  { role: "assistant", content: JSON.stringify({ nhiemVu: "dem_so", doTinCay: 0.95 }) },
  { role: "user", content: promptThat },
];

console.log("1 vi du:", demViDuMau(motViDu), goiModelJSON_v2(motViDu));
```

```text title=readonly
1 vi du: 1 Toi nghi day la nhiem vu tom_tat, do tin cay khoang 0.5.
```

`demViDuMau` đếm được đúng `1` — nhưng `1 >= 2` LÀ `false`, VÀ
`coChiRoSchemaJson` cũng `false` (prompt không nhắc gì tới JSON) —
KHÔNG nhánh nào trong điều kiện `OR` khớp, hàm rơi về câu PROSE mặc
định, giống hệt trường hợp `0` ví dụ. Ngưỡng LÀ MỘT con số CỤ THỂ
(`2`), không phải "càng nhiều ví dụ càng tốt dần".
::::

::::predict{#doan_ba_vi_du commitOnce}
Giữ nguyên `haiViDu` (đã có `2` ví dụ mẫu, cho JSON đúng schema). Thêm
MỘT cặp `user`/`assistant` THỨ BA vào TRƯỚC câu hỏi thật, làm
`demViDuMau` tăng từ `2` lên `3`. Kết quả của `goiModelJSON_v2` có
KHÁC so với lúc chỉ có `2` ví dụ không?

:::opt{correct}
Không đổi — vẫn LÀ JSON đúng schema. Điều kiện LÀ `demViDuMau(messages)
>= 2`, một NGƯỠNG chứ không phải phép đếm tỉ lệ; `2` ví dụ hay `3` ví
dụ đều rơi vào CÙNG nhánh `if`, cho CÙNG một loại kết quả
:::
:::opt
Có — JSON trả về sẽ "chắc chắn hơn", ví dụ `doTinCay` sẽ tăng lên vì có
thêm bằng chứng ủng hộ
::why
Gần đúng Ở trực giác chung "thêm ví dụ mẫu thường cải thiện chất
lượng" — trực giác đó có cơ sở với một LLM thật trong nhiều tình
huống.

Chỗ lệch: `doTinCayGiaLap` chỉ đọc NỘI DUNG của prompt thật (có chứa
"dich"/"dem"/"tong"/"tom tat" hay không) — nó KHÔNG hề đọc
`demViDuMau(messages)`. Thêm ví dụ mẫu thứ `3` không đổi `doTinCay`,
càng không đổi kết quả `nhiemVu` — CHỈ đổi được liệu nhánh JSON có
được kích hoạt hay KHÔNG, một khi đã kích hoạt rồi thì thêm ví dụ nữa
không LÀM gì thêm.
::
:::
:::opt
Không xác định được nếu không chạy thử — hành vi few-shot phụ thuộc
NỘI DUNG cụ thể của từng ví dụ, không chỉ SỐ LƯỢNG
::why
Gần đúng VỚI một LLM THẬT — nội dung của từng ví dụ mẫu (không chỉ số
lượng) thường ảnh hưởng câu trả lời.

Chỗ lệch: đây LÀ một harness MÔ PHỎNG, luật của nó đọc thẳng trong
code — `demViDuMau` CHỈ đếm số message có `role === "assistant"`,
không hề đọc `content` của chúng. Điều kiện `demViDuMau(messages) >=
2` LÀ một phép đếm đơn thuần, hoàn toàn xác định TRƯỚC khi chạy, không
cần đoán.
::
:::
::::

::::code{#viet_dem_vi_du_va_nguong}
Hoàn thiện `demViDuMau` — đếm số message có `role === "assistant"`.
Hoàn thiện điều kiện trong `goiModelJSON_v2` — thêm nhánh `OR` để CŨNG
kích hoạt JSON đúng schema khi `demViDuMau(messages) >= 2`, dù prompt
không hề chỉ rõ schema bằng lời.

```typescript title=starter
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

function demViDuMau(messages: ChatMessage[]): number {
  ___
}

function goiModelJSON_v2(messages: ChatMessage[]): string {
  const prompt = layNoiDungUserCuoi(messages);
  const nhiemVu = phanLoaiNhiemVuDonGian(prompt);
  const doTinCay = doTinCayGiaLap(prompt);
  if (coChiRoSchemaJson(messages) || ___) {
    return JSON.stringify({ nhiemVu, doTinCay });
  }
  if (coNhacToiJsonChungChung(messages)) {
    return JSON.stringify({ loaiNhiemVu: nhiemVu, mucTinCay: doTinCay });
  }
  return `Toi nghi day la nhiem vu ${nhiemVu}, do tin cay khoang ${doTinCay}.`;
}

const vd1: ChatMessage = { role: "user", content: "Dem so nguoi tham gia" };
const vd1Dap: ChatMessage = { role: "assistant", content: JSON.stringify({ nhiemVu: "dem_so", doTinCay: 0.95 }) };
const vd2: ChatMessage = { role: "user", content: "Dich cau chao sang tieng Phap" };
const vd2Dap: ChatMessage = { role: "assistant", content: JSON.stringify({ nhiemVu: "dich", doTinCay: 0.95 }) };

const msgsX: ChatMessage[] = [vd1, vd1Dap, vd2, vd2Dap, { role: "user", content: "Cho toi y kien ve bai viet nay" }];
console.log(demViDuMau(msgsX), goiModelJSON_v2(msgsX));
```

```typescript title=solution
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

function demViDuMau(messages: ChatMessage[]): number {
  return messages.filter((m) => m.role === "assistant").length;
}

function goiModelJSON_v2(messages: ChatMessage[]): string {
  const prompt = layNoiDungUserCuoi(messages);
  const nhiemVu = phanLoaiNhiemVuDonGian(prompt);
  const doTinCay = doTinCayGiaLap(prompt);
  if (coChiRoSchemaJson(messages) || demViDuMau(messages) >= 2) {
    return JSON.stringify({ nhiemVu, doTinCay });
  }
  if (coNhacToiJsonChungChung(messages)) {
    return JSON.stringify({ loaiNhiemVu: nhiemVu, mucTinCay: doTinCay });
  }
  return `Toi nghi day la nhiem vu ${nhiemVu}, do tin cay khoang ${doTinCay}.`;
}

const vd1: ChatMessage = { role: "user", content: "Dem so nguoi tham gia" };
const vd1Dap: ChatMessage = { role: "assistant", content: JSON.stringify({ nhiemVu: "dem_so", doTinCay: 0.95 }) };
const vd2: ChatMessage = { role: "user", content: "Dich cau chao sang tieng Phap" };
const vd2Dap: ChatMessage = { role: "assistant", content: JSON.stringify({ nhiemVu: "dich", doTinCay: 0.95 }) };

const msgsX: ChatMessage[] = [vd1, vd1Dap, vd2, vd2Dap, { role: "user", content: "Cho toi y kien ve bai viet nay" }];
console.log(demViDuMau(msgsX), goiModelJSON_v2(msgsX));
```

```typescript title=test
if (demViDuMau([{ role: "user", content: "x" }]) !== 0) throw new Error("khong co message assistant nao thi demViDuMau phai la 0");

const vd1T: ChatMessage = { role: "user", content: "Dem so nguoi tham gia" };
const vd1DapT: ChatMessage = { role: "assistant", content: JSON.stringify({ nhiemVu: "dem_so", doTinCay: 0.95 }) };
if (demViDuMau([vd1T, vd1DapT]) !== 1) throw new Error("dung 1 message assistant thi demViDuMau phai la 1");

const khongViDu: ChatMessage[] = [{ role: "user", content: "Cho toi y kien ve bai viet nay" }];
const raKhongViDu = goiModelJSON_v2(khongViDu);
if (raKhongViDu.startsWith("{")) throw new Error("khong co vi du mau, khong chi ro schema -- phai la van xuoi, khong phai JSON");

const motViDu: ChatMessage[] = [vd1T, vd1DapT, { role: "user", content: "Cho toi y kien ve bai viet nay" }];
const raMotViDu = goiModelJSON_v2(motViDu);
if (raMotViDu.startsWith("{")) throw new Error("CHI 1 vi du mau (duoi nguong >=2) van phai la van xuoi, chua doi hanh vi");

const vd2T: ChatMessage = { role: "user", content: "Dich cau chao sang tieng Phap" };
const vd2DapT: ChatMessage = { role: "assistant", content: JSON.stringify({ nhiemVu: "dich", doTinCay: 0.95 }) };
const haiViDu: ChatMessage[] = [vd1T, vd1DapT, vd2T, vd2DapT, { role: "user", content: "Cho toi y kien ve bai viet nay" }];
if (demViDuMau(haiViDu) !== 2) throw new Error("dung 2 message assistant thi demViDuMau phai la 2");
const raHaiViDu = goiModelJSON_v2(haiViDu);
const parsedHaiViDu: unknown = JSON.parse(raHaiViDu);
const ghiHaiViDu = parsedHaiViDu as Record<string, unknown>;
if (ghiHaiViDu.nhiemVu !== "tom_tat" || ghiHaiViDu.doTinCay !== 0.5) throw new Error("DU 2 vi du mau (>=2) phai chuyen sang tra ve JSON dung schema, du prompt khong noi gi ve JSON");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (than ham demViDuMau): return messages.filter roi loc theo dieu kien m.role === 'assistant', lay .length. Cho hai (trong dieu kien if cua goiModelJSON_v2): them demViDuMau(messages) >= 2 noi voi coChiRoSchemaJson(messages) bang toan tu ||."
- kind: strategy
  body: "Cho dau: return messages.filter((m) => m.role === \"assistant\").length; Cho hai: coChiRoSchemaJson(messages) || demViDuMau(messages) >= 2"
- kind: one-line
  body: "Sao chep dung hai bieu thuc o phan Strategy vao dung vi tri tuong ung."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "2 {\"nhiemVu\":\"tom_tat\",\"doTinCay\":0.5}"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không lời chỉ thị nào, chỉ hai ví dụ mẫu — VÀ định dạng đổi hoàn toàn.
`demViDuMau` cùng ngưỡng `>= 2` sẽ còn xuất hiện lại Ở BOSS quý cuối
quest. Nhưng để RÁP nhiều kỹ thuật (ràng buộc, few-shot, schema) lại
với nhau một cách AN TOÀN, cần một cách mô hình hoá LỖI mà không gộp
chung mọi thất bại thành một khối — đó LÀ cầu nối sang lập trình hàm.
::::

::::reflect{#nghi-lai}
`goiModelJSON_v2` không hề "học" theo nghĩa cập nhật trọng số — nó CHỈ
thêm MỘT điều kiện `OR` vào ĐÚNG chỗ code đã viết sẵn Ở bài trước. Điều
đáng chú Ý LÀ: hai kỹ thuật hoàn toàn KHÁC nhau — một chỉ thị bằng lời
(bài trước) VÀ hai ví dụ mẫu (bài này) — dẫn tới CÙNG một hành vi đúng
định dạng, qua HAI nhánh code độc lập. Với một LLM thật, đây chính LÀ
lý do few-shot đôi khi được ưu tiên hơn chỉ thị dài dòng: ví dụ CỤ THỂ
thường truyền tải định dạng rõ hơn một câu mô tả trừu tượng.
::::

::::checkpoint{mastery=0.82}
::::
