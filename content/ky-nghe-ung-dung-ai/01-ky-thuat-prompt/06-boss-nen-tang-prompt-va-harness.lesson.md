---
id: ky-nghe-ung-dung-ai.ky-thuat-prompt.boss-nen-tang-prompt-va-harness
title: "BOSS — ráp ràng buộc + structured output + few-shot, đo pass@1 tổng hợp"
summary: "chayThucNghiemPromptDayDu(taoMessages) rap NGUYEN VAN cac ham bai 1-5 (phanLoaiNhiemVuCoRangBuoc, layNoiDungUserCuoi, doTinCayGiaLap, coChiRoSchemaJson, coNhacToiJsonChungChung, demViDuMau, goiToolPhanLoai, tinhPassAt1 -- khong viet lai logic nao), chay tren CUNG bo 6 prompt cua bai 1. Mot cau duoc tinh 'dung' khi CA HAI dieu kien: goiToolPhanLoai tra ve loai=thanh_cong (dinh dang, bai 3+5) VA duLieu.nhiemVu === nhan dung (gia tri, bai 1+2). Prompt TRAN TRUI (khong ky thuat nao): pass@1 = 0/6. Prompt DAY DU (rang buoc + 2 vi du mau + chi ro schema): pass@1 = 6/6. Doi chung rieng le: CHI rang buoc (khong schema/few-shot) van la 0/6 vi dinh dang luon that bai; CHI schema+few-shot (khong rang buoc) la 5/6, mot cau mo ho van bi doan sai gia tri -- xac nhan dinh dang va gia tri la HAI truc doc lap, can CA BA ky thuat moi dat toi da. Dong q9.1a tai 6/6."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-prompt
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [kna.boss-nen-tang-prompt-va-harness]
requires: [kna.tool-schema-la-domain-modeling]
concepts: [kna.boss-nen-tang-prompt-va-harness]
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
Năm bài: phân loại nhiệm vụ, ràng buộc sửa mơ hồ, structured output,
few-shot sửa định dạng, tool schema qua discriminated union. Bài này
ráp TẤT CẢ lại thành MỘT prompt hoàn chỉnh — và đo bằng số xem nó cải
thiện được bao nhiêu so với một prompt trần trụi, trên ĐÚNG bộ `6`
prompt đã dùng xuyên suốt quest.
::::

::::explain{#rap_toan_bo_harness}
Một prompt hoàn chỉnh gồm ĐÚNG những gì năm bài trước đã xây, không
thêm gì mới:

> **Ràng buộc rõ ràng** (bài `2`) — một message `system` ép model chọn
> một trong ba loại, sửa được TRỤC GIÁ TRỊ (phân loại ĐÚNG hay sai).
>
> **Structured output + schema** (bài `3`) — chỉ rõ tên hai khoá
> `nhiemVu`/`doTinCay` trong message `system`, sửa được TRỤC ĐỊNH DẠNG.
>
> **Few-shot** (bài `4`) — hai cặp `user`/`assistant` mẫu, CŨNG sửa
> được TRỤC ĐỊNH DẠNG qua một con đường ĐỘC LẬP với schema.
>
> **Tool schema qua discriminated union** (bài `5`) — `goiToolPhanLoai`
> phân biệt `thanh_cong`/`loi_parse`/`thieu_khoa`, dùng LÀM tiêu chí đo
> ĐỊNH DẠNG có đúng không.

`llmMoPhongDayDu` ráp lại: dùng `phanLoaiNhiemVuCoRangBuoc` (bài `2`)
để quyết định GIÁ TRỊ (ánh xạ `"khong_ro"` sang mặc định `"tom_tat"` vì
`KetQuaPhanLoai` không cho phép giá trị đó), rồi dùng ĐÚNG điều kiện
`OR` của bài `4` (`coChiRoSchemaJson` HOẶC `demViDuMau >= 2`) để quyết
định ĐỊNH DẠNG:

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

function coRangBuocChonMotTrongBa(messages: ChatMessage[]): boolean {
  return messages.some(
    (tin) => tin.role === "system" && tin.content.toLowerCase().includes("khong duoc tra loi khong_ro"),
  );
}

function phanLoaiNhiemVuCoRangBuoc(messages: ChatMessage[]): "tom_tat" | "dich" | "dem_so" | "khong_ro" {
  const prompt = layNoiDungUserCuoi(messages).toLowerCase();
  const buocChon = coRangBuocChonMotTrongBa(messages);
  if (prompt.includes("tom tat")) return "tom_tat";
  if (prompt.includes("dich")) return "dich";
  if (prompt.includes("dem") || prompt.includes("tong")) return "dem_so";
  if (buocChon && prompt.includes("y kien")) return "tom_tat";
  if (buocChon && prompt.includes("ngon ngu khac")) return "dich";
  if (buocChon) return "tom_tat";
  return "khong_ro";
}

interface KetQuaPhanLoai {
  nhiemVu: "tom_tat" | "dich" | "dem_so";
  doTinCay: number;
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

function demViDuMau(messages: ChatMessage[]): number {
  return messages.filter((m) => m.role === "assistant").length;
}

function llmMoPhongDayDu(messages: ChatMessage[]): string {
  const nhiemVuTho = phanLoaiNhiemVuCoRangBuoc(messages);
  const nhiemVu = nhiemVuTho === "khong_ro" ? "tom_tat" : nhiemVuTho;
  const prompt = layNoiDungUserCuoi(messages);
  const doTinCay = doTinCayGiaLap(prompt);
  if (coChiRoSchemaJson(messages) || demViDuMau(messages) >= 2) {
    return JSON.stringify({ nhiemVu, doTinCay });
  }
  return `Toi nghi day la nhiem vu ${nhiemVu}, do tin cay khoang ${doTinCay}.`;
}

const promptTranTrui: ChatMessage[] = [{ role: "user", content: "Cho toi y kien ve bai viet nay" }];
console.log("tran trui:", llmMoPhongDayDu(promptTranTrui));

const promptDayDu: ChatMessage[] = [
  { role: "system", content: "Chi duoc chon mot trong ba loai: tom_tat, dich, hoac dem_so. Khong duoc tra loi khong_ro. Tra loi bang JSON co dung hai khoa nhiemVu va doTinCay, khong viet gi them." },
  { role: "user", content: "Dem so nguoi tham gia buoi hop nay" },
  { role: "assistant", content: JSON.stringify({ nhiemVu: "dem_so", doTinCay: 0.95 }) },
  { role: "user", content: "Dich cau chao mung sang tieng Phap" },
  { role: "assistant", content: JSON.stringify({ nhiemVu: "dich", doTinCay: 0.95 }) },
  { role: "user", content: "Cho toi y kien ve bai viet nay" },
];
console.log("day du:", llmMoPhongDayDu(promptDayDu));
```

```text title=readonly
tran trui: Toi nghi day la nhiem vu tom_tat, do tin cay khoang 0.5.
day du: {"nhiemVu":"tom_tat","doTinCay":0.5}
```

Prompt trần trụi cho một câu PROSE — `goiToolPhanLoai` (bài `5`) sẽ
phân loại đây LÀ `"loi_parse"`, THẤT BẠI Ở trục định dạng. Prompt đầy
đủ cho JSON đúng schema, VÀ giá trị `"tom_tat"` cũng ĐÚNG với nhãn thật
(`"Cho toi y kien..."` LÀ một yêu cầu tóm tắt/nhận xét) — nhờ ràng
buộc Ở bài `2` suy đúng từ khoá rộng `"y kien"`.
::::

::::example{#do_ca_bo_test}
Đo trên ĐÚNG bộ `6` prompt của bài `1`, dùng `goiToolPhanLoai` (bài
`5`) LÀM tiêu chí: một câu chỉ tính LÀ đúng khi VỪA parse thành công
VỪA đúng giá trị:

```typescript title=readonly
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}
interface KetQuaPhanLoai {
  nhiemVu: "tom_tat" | "dich" | "dem_so";
  doTinCay: number;
}
type KetQuaGoiTool =
  | { loai: "thanh_cong"; duLieu: KetQuaPhanLoai }
  | { loai: "loi_parse"; chuoiGoc: string }
  | { loai: "thieu_khoa"; khoaThieu: string[] };

function layNoiDungUserCuoi(messages: ChatMessage[]): string {
  let noiDung = "";
  for (const tin of messages) {
    if (tin.role === "user") noiDung = tin.content;
  }
  return noiDung;
}
function coRangBuocChonMotTrongBa(messages: ChatMessage[]): boolean {
  return messages.some(
    (tin) => tin.role === "system" && tin.content.toLowerCase().includes("khong duoc tra loi khong_ro"),
  );
}
function phanLoaiNhiemVuCoRangBuoc(messages: ChatMessage[]): "tom_tat" | "dich" | "dem_so" | "khong_ro" {
  const prompt = layNoiDungUserCuoi(messages).toLowerCase();
  const buocChon = coRangBuocChonMotTrongBa(messages);
  if (prompt.includes("tom tat")) return "tom_tat";
  if (prompt.includes("dich")) return "dich";
  if (prompt.includes("dem") || prompt.includes("tong")) return "dem_so";
  if (buocChon && prompt.includes("y kien")) return "tom_tat";
  if (buocChon && prompt.includes("ngon ngu khac")) return "dich";
  if (buocChon) return "tom_tat";
  return "khong_ro";
}
function doTinCayGiaLap(prompt: string): number {
  const p = prompt.toLowerCase();
  if (p.includes("dich") || p.includes("dem") || p.includes("tong") || p.includes("tom tat")) return 0.9;
  return 0.5;
}
function coChiRoSchemaJson(messages: ChatMessage[]): boolean {
  const noi = messages.filter((m) => m.role !== "assistant").map((m) => m.content.toLowerCase()).join(" ");
  return noi.includes("nhiemvu") && noi.includes("dotincay");
}
function demViDuMau(messages: ChatMessage[]): number {
  return messages.filter((m) => m.role === "assistant").length;
}
function llmMoPhongDayDu(messages: ChatMessage[]): string {
  const nhiemVuTho = phanLoaiNhiemVuCoRangBuoc(messages);
  const nhiemVu = nhiemVuTho === "khong_ro" ? "tom_tat" : nhiemVuTho;
  const prompt = layNoiDungUserCuoi(messages);
  const doTinCay = doTinCayGiaLap(prompt);
  if (coChiRoSchemaJson(messages) || demViDuMau(messages) >= 2) {
    return JSON.stringify({ nhiemVu, doTinCay });
  }
  return `Toi nghi day la nhiem vu ${nhiemVu}, do tin cay khoang ${doTinCay}.`;
}
function laNhiemVuHopLe(x: unknown): x is "tom_tat" | "dich" | "dem_so" {
  return x === "tom_tat" || x === "dich" || x === "dem_so";
}
function goiToolPhanLoai(chuoiTraVe: string): KetQuaGoiTool {
  let giaTri: unknown;
  try {
    giaTri = JSON.parse(chuoiTraVe);
  } catch {
    return { loai: "loi_parse", chuoiGoc: chuoiTraVe };
  }
  if (typeof giaTri !== "object" || giaTri === null) return { loai: "loi_parse", chuoiGoc: chuoiTraVe };
  const ghi = giaTri as Record<string, unknown>;
  const coNhiemVu = laNhiemVuHopLe(ghi.nhiemVu);
  const coDoTinCay = typeof ghi.doTinCay === "number";
  if (!coNhiemVu || !coDoTinCay) {
    const khoaThieu: string[] = [];
    if (!coNhiemVu) khoaThieu.push("nhiemVu");
    if (!coDoTinCay) khoaThieu.push("doTinCay");
    return { loai: "thieu_khoa", khoaThieu };
  }
  return { loai: "thanh_cong", duLieu: { nhiemVu: ghi.nhiemVu as "tom_tat" | "dich" | "dem_so", doTinCay: ghi.doTinCay as number } };
}
function tinhPassAt1(ketQua: boolean[]): number {
  if (ketQua.length === 0) return 0;
  const soDung = ketQua.filter((x) => x).length;
  return soDung / ketQua.length;
}

const CHI_THI_RANG_BUOC = "Chi duoc chon mot trong ba loai: tom_tat, dich, hoac dem_so. Khong duoc tra loi khong_ro.";
const CHI_THI_SCHEMA_JSON = "Tra loi bang JSON co dung hai khoa nhiemVu va doTinCay, khong viet gi them.";
const VI_DU_1_USER = "Dem so nguoi tham gia buoi hop nay";
const VI_DU_1_ASSISTANT = JSON.stringify({ nhiemVu: "dem_so", doTinCay: 0.95 });
const VI_DU_2_USER = "Dich cau chao mung sang tieng Phap";
const VI_DU_2_ASSISTANT = JSON.stringify({ nhiemVu: "dich", doTinCay: 0.95 });

function taoPromptTranTrui(prompt: string): ChatMessage[] {
  return [{ role: "user", content: prompt }];
}
function taoPromptDayDu(prompt: string): ChatMessage[] {
  return [
    { role: "system", content: `${CHI_THI_RANG_BUOC} ${CHI_THI_SCHEMA_JSON}` },
    { role: "user", content: VI_DU_1_USER },
    { role: "assistant", content: VI_DU_1_ASSISTANT },
    { role: "user", content: VI_DU_2_USER },
    { role: "assistant", content: VI_DU_2_ASSISTANT },
    { role: "user", content: prompt },
  ];
}

interface CauKiemThu {
  prompt: string;
  nhanDung: "tom_tat" | "dich" | "dem_so" | "khong_ro";
}
const BO_KIEM_THU: CauKiemThu[] = [
  { prompt: "Hay tom tat doan van ban sau day thanh mot doan ngan", nhanDung: "tom_tat" },
  { prompt: "Dich cau sau sang tieng Anh: Xin chao ban", nhanDung: "dich" },
  { prompt: "Dem so luong don hang thang nay giup toi", nhanDung: "dem_so" },
  { prompt: "Tinh tong doanh thu quy nay", nhanDung: "dem_so" },
  { prompt: "Cho toi y kien ve bai viet nay", nhanDung: "tom_tat" },
  { prompt: "Giup toi chuyen tai lieu nay sang ngon ngu khac", nhanDung: "dich" },
];

function chayThucNghiemPromptDayDu(taoMessages: (prompt: string) => ChatMessage[]): boolean[] {
  return BO_KIEM_THU.map((c) => {
    const chuoiTraVe = llmMoPhongDayDu(taoMessages(c.prompt));
    const ketQuaTool = goiToolPhanLoai(chuoiTraVe);
    return ketQuaTool.loai === "thanh_cong" && ketQuaTool.duLieu.nhiemVu === c.nhanDung;
  });
}

const ketQuaTranTrui = chayThucNghiemPromptDayDu(taoPromptTranTrui);
const ketQuaDayDu = chayThucNghiemPromptDayDu(taoPromptDayDu);
console.log("tran trui:", ketQuaTranTrui, "pass@1:", tinhPassAt1(ketQuaTranTrui));
console.log("day du:", ketQuaDayDu, "pass@1:", tinhPassAt1(ketQuaDayDu));
```

```text title=readonly
tran trui: [false,false,false,false,false,false] pass@1: 0
day du: [true,true,true,true,true,true] pass@1: 1
```

`pass@1` từ `0` LÊN `1` — từ KHÔNG câu nào đúng CẢ HAI trục, lên TOÀN
BỘ `6` câu đúng CẢ HAI trục, đo trên ĐÚNG bộ test đã dùng suốt quest.
::::

::::predict{#doan_doi_chung_rieng_le commitOnce}
Chạy `chayThucNghiemPromptDayDu` với một biến thể CHỈ có schema tường
minh VÀ hai ví dụ mẫu — KHÔNG có message ràng buộc `"khong duoc tra
loi khong_ro"`. `pass@1` của biến thể này LÀ bao nhiêu?

:::opt{correct}
`5/6` — định dạng LUÔN đúng (schema + few-shot đã đủ kích hoạt JSON
cho CẢ `6` câu), nhưng KHÔNG có ràng buộc, `phanLoaiNhiemVuCoRangBuoc`
CHỈ dùng được ba từ khoá cứng — hai câu mơ hồ (`"Cho toi y kien..."` VÀ
`"Giup toi chuyen tai lieu..."`) đều rơi vào nhánh MẶC ĐỊNH
`"tom_tat"`; câu ĐẦU trùng NHÃN thật (đúng, MAY MẮN), câu SAU (nhãn
thật LÀ `"dich"`) thì SAI — CHỈ MỘT câu sai trên tổng `6`
:::
:::opt
`0/6` — thiếu ràng buộc nghĩa LÀ hai câu mơ hồ đều bị phân loại sai,
VÀ định dạng cũng hỏng THEO vì `phanLoaiNhiemVuCoRangBuoc` trả về
`"khong_ro"` không nằm trong schema `KetQuaPhanLoai`
::why
Nhầm "phân loại GIÁ TRỊ có thể sai" VỚI "ĐỊNH DẠNG cũng hỏng theo" —
nhưng `llmMoPhongDayDu` LUÔN ánh xạ `"khong_ro"` sang một giá trị CỤ
THỂ (`"tom_tat"`) TRƯỚC khi đưa vào JSON, không bao giờ để lọt giá trị
KHÔNG hợp lệ vào chuỗi trả về.

Chỗ lệch: dòng `const nhiemVu = nhiemVuTho === "khong_ro" ? "tom_tat" :
nhiemVuTho;` xử lý CHÍNH xác trường hợp NÀY — GIÁ TRỊ có thể SAI (so
với nhãn thật), nhưng ĐỊNH DẠNG (một chuỗi JSON hợp lệ đúng hai khoá)
vẫn LUÔN đúng, vì schema+few-shot đã đủ kích hoạt nhánh JSON, không
phụ thuộc gì vào ràng buộc.
::
:::
:::opt
`6/6` — schema VÀ few-shot đã LÀ hai kỹ thuật MẠNH nhất, đủ để bù đắp
cho việc thiếu ràng buộc
::why
Gần đúng Ở việc bạn nhận ra schema+few-shot LÀ hai kỹ thuật MẠNH, sửa
được TRỤC ĐỊNH DẠNG hoàn toàn — quan sát đó đúng.

Chỗ lệch: schema VÀ few-shot chỉ chạm tới TRỤC ĐỊNH DẠNG (bài `3`,
`4`), KHÔNG chạm tới TRỤC GIÁ TRỊ — quyết định GIÁ TRỊ hoàn toàn nằm Ở
`phanLoaiNhiemVuCoRangBuoc`, VÀ hàm đó CHỈ mở rộng khả năng suy đoán
đúng khi `coRangBuocChonMotTrongBa(messages)` LÀ `true`. Thiếu message
ràng buộc, hàm rơi về ĐÚNG hành vi bài `1` (chỉ ba từ khoá cứng) cho
prompt mơ hồ — một trong hai câu mơ hồ VẪN sai giá trị.
::
:::
::::

::::code{#viet_chay_thuc_nghiem_prompt_day_du}
Hoàn thiện `chayThucNghiemPromptDayDu` — với MỖI câu kiểm thử: gọi
`llmMoPhongDayDu` trên messages do `taoMessages` dựng, đưa chuỗi trả về
qua `goiToolPhanLoai` (bài `5`), rồi trả về `true` CHỈ KHI kết quả VỪA
`loai === "thanh_cong"` VỪA `duLieu.nhiemVu` khớp ĐÚNG nhãn mong đợi.

```typescript title=starter
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}
interface KetQuaPhanLoai {
  nhiemVu: "tom_tat" | "dich" | "dem_so";
  doTinCay: number;
}
type KetQuaGoiTool =
  | { loai: "thanh_cong"; duLieu: KetQuaPhanLoai }
  | { loai: "loi_parse"; chuoiGoc: string }
  | { loai: "thieu_khoa"; khoaThieu: string[] };

function layNoiDungUserCuoi(messages: ChatMessage[]): string {
  let noiDung = "";
  for (const tin of messages) {
    if (tin.role === "user") noiDung = tin.content;
  }
  return noiDung;
}
function coRangBuocChonMotTrongBa(messages: ChatMessage[]): boolean {
  return messages.some(
    (tin) => tin.role === "system" && tin.content.toLowerCase().includes("khong duoc tra loi khong_ro"),
  );
}
function phanLoaiNhiemVuCoRangBuoc(messages: ChatMessage[]): "tom_tat" | "dich" | "dem_so" | "khong_ro" {
  const prompt = layNoiDungUserCuoi(messages).toLowerCase();
  const buocChon = coRangBuocChonMotTrongBa(messages);
  if (prompt.includes("tom tat")) return "tom_tat";
  if (prompt.includes("dich")) return "dich";
  if (prompt.includes("dem") || prompt.includes("tong")) return "dem_so";
  if (buocChon && prompt.includes("y kien")) return "tom_tat";
  if (buocChon && prompt.includes("ngon ngu khac")) return "dich";
  if (buocChon) return "tom_tat";
  return "khong_ro";
}
function doTinCayGiaLap(prompt: string): number {
  const p = prompt.toLowerCase();
  if (p.includes("dich") || p.includes("dem") || p.includes("tong") || p.includes("tom tat")) return 0.9;
  return 0.5;
}
function coChiRoSchemaJson(messages: ChatMessage[]): boolean {
  const noi = messages.filter((m) => m.role !== "assistant").map((m) => m.content.toLowerCase()).join(" ");
  return noi.includes("nhiemvu") && noi.includes("dotincay");
}
function demViDuMau(messages: ChatMessage[]): number {
  return messages.filter((m) => m.role === "assistant").length;
}
function llmMoPhongDayDu(messages: ChatMessage[]): string {
  const nhiemVuTho = phanLoaiNhiemVuCoRangBuoc(messages);
  const nhiemVu = nhiemVuTho === "khong_ro" ? "tom_tat" : nhiemVuTho;
  const prompt = layNoiDungUserCuoi(messages);
  const doTinCay = doTinCayGiaLap(prompt);
  if (coChiRoSchemaJson(messages) || demViDuMau(messages) >= 2) {
    return JSON.stringify({ nhiemVu, doTinCay });
  }
  return `Toi nghi day la nhiem vu ${nhiemVu}, do tin cay khoang ${doTinCay}.`;
}
function laNhiemVuHopLe(x: unknown): x is "tom_tat" | "dich" | "dem_so" {
  return x === "tom_tat" || x === "dich" || x === "dem_so";
}
function goiToolPhanLoai(chuoiTraVe: string): KetQuaGoiTool {
  let giaTri: unknown;
  try {
    giaTri = JSON.parse(chuoiTraVe);
  } catch {
    return { loai: "loi_parse", chuoiGoc: chuoiTraVe };
  }
  if (typeof giaTri !== "object" || giaTri === null) return { loai: "loi_parse", chuoiGoc: chuoiTraVe };
  const ghi = giaTri as Record<string, unknown>;
  const coNhiemVu = laNhiemVuHopLe(ghi.nhiemVu);
  const coDoTinCay = typeof ghi.doTinCay === "number";
  if (!coNhiemVu || !coDoTinCay) {
    const khoaThieu: string[] = [];
    if (!coNhiemVu) khoaThieu.push("nhiemVu");
    if (!coDoTinCay) khoaThieu.push("doTinCay");
    return { loai: "thieu_khoa", khoaThieu };
  }
  return { loai: "thanh_cong", duLieu: { nhiemVu: ghi.nhiemVu as "tom_tat" | "dich" | "dem_so", doTinCay: ghi.doTinCay as number } };
}
function tinhPassAt1(ketQua: boolean[]): number {
  if (ketQua.length === 0) return 0;
  const soDung = ketQua.filter((x) => x).length;
  return soDung / ketQua.length;
}

const CHI_THI_RANG_BUOC = "Chi duoc chon mot trong ba loai: tom_tat, dich, hoac dem_so. Khong duoc tra loi khong_ro.";
const CHI_THI_SCHEMA_JSON = "Tra loi bang JSON co dung hai khoa nhiemVu va doTinCay, khong viet gi them.";
const VI_DU_1_USER = "Dem so nguoi tham gia buoi hop nay";
const VI_DU_1_ASSISTANT = JSON.stringify({ nhiemVu: "dem_so", doTinCay: 0.95 });
const VI_DU_2_USER = "Dich cau chao mung sang tieng Phap";
const VI_DU_2_ASSISTANT = JSON.stringify({ nhiemVu: "dich", doTinCay: 0.95 });

function taoPromptTranTrui(prompt: string): ChatMessage[] {
  return [{ role: "user", content: prompt }];
}
function taoPromptDayDu(prompt: string): ChatMessage[] {
  return [
    { role: "system", content: `${CHI_THI_RANG_BUOC} ${CHI_THI_SCHEMA_JSON}` },
    { role: "user", content: VI_DU_1_USER },
    { role: "assistant", content: VI_DU_1_ASSISTANT },
    { role: "user", content: VI_DU_2_USER },
    { role: "assistant", content: VI_DU_2_ASSISTANT },
    { role: "user", content: prompt },
  ];
}
function taoPromptChiRangBuoc(prompt: string): ChatMessage[] {
  return [
    { role: "system", content: CHI_THI_RANG_BUOC },
    { role: "user", content: prompt },
  ];
}
function taoPromptChiSchemaVaFewShot(prompt: string): ChatMessage[] {
  return [
    { role: "system", content: CHI_THI_SCHEMA_JSON },
    { role: "user", content: VI_DU_1_USER },
    { role: "assistant", content: VI_DU_1_ASSISTANT },
    { role: "user", content: VI_DU_2_USER },
    { role: "assistant", content: VI_DU_2_ASSISTANT },
    { role: "user", content: prompt },
  ];
}

interface CauKiemThu {
  prompt: string;
  nhanDung: "tom_tat" | "dich" | "dem_so" | "khong_ro";
}
const BO_KIEM_THU: CauKiemThu[] = [
  { prompt: "Hay tom tat doan van ban sau day thanh mot doan ngan", nhanDung: "tom_tat" },
  { prompt: "Dich cau sau sang tieng Anh: Xin chao ban", nhanDung: "dich" },
  { prompt: "Dem so luong don hang thang nay giup toi", nhanDung: "dem_so" },
  { prompt: "Tinh tong doanh thu quy nay", nhanDung: "dem_so" },
  { prompt: "Cho toi y kien ve bai viet nay", nhanDung: "tom_tat" },
  { prompt: "Giup toi chuyen tai lieu nay sang ngon ngu khac", nhanDung: "dich" },
];

function chayThucNghiemPromptDayDu(taoMessages: (prompt: string) => ChatMessage[]): boolean[] {
  return BO_KIEM_THU.map((c) => {
    ___
    ___
  });
}

const ketQuaTranTrui = chayThucNghiemPromptDayDu(taoPromptTranTrui);
const ketQuaDayDu = chayThucNghiemPromptDayDu(taoPromptDayDu);
console.log(tinhPassAt1(ketQuaTranTrui), tinhPassAt1(ketQuaDayDu));
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
type KetQuaGoiTool =
  | { loai: "thanh_cong"; duLieu: KetQuaPhanLoai }
  | { loai: "loi_parse"; chuoiGoc: string }
  | { loai: "thieu_khoa"; khoaThieu: string[] };

function layNoiDungUserCuoi(messages: ChatMessage[]): string {
  let noiDung = "";
  for (const tin of messages) {
    if (tin.role === "user") noiDung = tin.content;
  }
  return noiDung;
}
function coRangBuocChonMotTrongBa(messages: ChatMessage[]): boolean {
  return messages.some(
    (tin) => tin.role === "system" && tin.content.toLowerCase().includes("khong duoc tra loi khong_ro"),
  );
}
function phanLoaiNhiemVuCoRangBuoc(messages: ChatMessage[]): "tom_tat" | "dich" | "dem_so" | "khong_ro" {
  const prompt = layNoiDungUserCuoi(messages).toLowerCase();
  const buocChon = coRangBuocChonMotTrongBa(messages);
  if (prompt.includes("tom tat")) return "tom_tat";
  if (prompt.includes("dich")) return "dich";
  if (prompt.includes("dem") || prompt.includes("tong")) return "dem_so";
  if (buocChon && prompt.includes("y kien")) return "tom_tat";
  if (buocChon && prompt.includes("ngon ngu khac")) return "dich";
  if (buocChon) return "tom_tat";
  return "khong_ro";
}
function doTinCayGiaLap(prompt: string): number {
  const p = prompt.toLowerCase();
  if (p.includes("dich") || p.includes("dem") || p.includes("tong") || p.includes("tom tat")) return 0.9;
  return 0.5;
}
function coChiRoSchemaJson(messages: ChatMessage[]): boolean {
  const noi = messages.filter((m) => m.role !== "assistant").map((m) => m.content.toLowerCase()).join(" ");
  return noi.includes("nhiemvu") && noi.includes("dotincay");
}
function demViDuMau(messages: ChatMessage[]): number {
  return messages.filter((m) => m.role === "assistant").length;
}
function llmMoPhongDayDu(messages: ChatMessage[]): string {
  const nhiemVuTho = phanLoaiNhiemVuCoRangBuoc(messages);
  const nhiemVu = nhiemVuTho === "khong_ro" ? "tom_tat" : nhiemVuTho;
  const prompt = layNoiDungUserCuoi(messages);
  const doTinCay = doTinCayGiaLap(prompt);
  if (coChiRoSchemaJson(messages) || demViDuMau(messages) >= 2) {
    return JSON.stringify({ nhiemVu, doTinCay });
  }
  return `Toi nghi day la nhiem vu ${nhiemVu}, do tin cay khoang ${doTinCay}.`;
}
function laNhiemVuHopLe(x: unknown): x is "tom_tat" | "dich" | "dem_so" {
  return x === "tom_tat" || x === "dich" || x === "dem_so";
}
function goiToolPhanLoai(chuoiTraVe: string): KetQuaGoiTool {
  let giaTri: unknown;
  try {
    giaTri = JSON.parse(chuoiTraVe);
  } catch {
    return { loai: "loi_parse", chuoiGoc: chuoiTraVe };
  }
  if (typeof giaTri !== "object" || giaTri === null) return { loai: "loi_parse", chuoiGoc: chuoiTraVe };
  const ghi = giaTri as Record<string, unknown>;
  const coNhiemVu = laNhiemVuHopLe(ghi.nhiemVu);
  const coDoTinCay = typeof ghi.doTinCay === "number";
  if (!coNhiemVu || !coDoTinCay) {
    const khoaThieu: string[] = [];
    if (!coNhiemVu) khoaThieu.push("nhiemVu");
    if (!coDoTinCay) khoaThieu.push("doTinCay");
    return { loai: "thieu_khoa", khoaThieu };
  }
  return { loai: "thanh_cong", duLieu: { nhiemVu: ghi.nhiemVu as "tom_tat" | "dich" | "dem_so", doTinCay: ghi.doTinCay as number } };
}
function tinhPassAt1(ketQua: boolean[]): number {
  if (ketQua.length === 0) return 0;
  const soDung = ketQua.filter((x) => x).length;
  return soDung / ketQua.length;
}

const CHI_THI_RANG_BUOC = "Chi duoc chon mot trong ba loai: tom_tat, dich, hoac dem_so. Khong duoc tra loi khong_ro.";
const CHI_THI_SCHEMA_JSON = "Tra loi bang JSON co dung hai khoa nhiemVu va doTinCay, khong viet gi them.";
const VI_DU_1_USER = "Dem so nguoi tham gia buoi hop nay";
const VI_DU_1_ASSISTANT = JSON.stringify({ nhiemVu: "dem_so", doTinCay: 0.95 });
const VI_DU_2_USER = "Dich cau chao mung sang tieng Phap";
const VI_DU_2_ASSISTANT = JSON.stringify({ nhiemVu: "dich", doTinCay: 0.95 });

function taoPromptTranTrui(prompt: string): ChatMessage[] {
  return [{ role: "user", content: prompt }];
}
function taoPromptDayDu(prompt: string): ChatMessage[] {
  return [
    { role: "system", content: `${CHI_THI_RANG_BUOC} ${CHI_THI_SCHEMA_JSON}` },
    { role: "user", content: VI_DU_1_USER },
    { role: "assistant", content: VI_DU_1_ASSISTANT },
    { role: "user", content: VI_DU_2_USER },
    { role: "assistant", content: VI_DU_2_ASSISTANT },
    { role: "user", content: prompt },
  ];
}
function taoPromptChiRangBuoc(prompt: string): ChatMessage[] {
  return [
    { role: "system", content: CHI_THI_RANG_BUOC },
    { role: "user", content: prompt },
  ];
}
function taoPromptChiSchemaVaFewShot(prompt: string): ChatMessage[] {
  return [
    { role: "system", content: CHI_THI_SCHEMA_JSON },
    { role: "user", content: VI_DU_1_USER },
    { role: "assistant", content: VI_DU_1_ASSISTANT },
    { role: "user", content: VI_DU_2_USER },
    { role: "assistant", content: VI_DU_2_ASSISTANT },
    { role: "user", content: prompt },
  ];
}

interface CauKiemThu {
  prompt: string;
  nhanDung: "tom_tat" | "dich" | "dem_so" | "khong_ro";
}
const BO_KIEM_THU: CauKiemThu[] = [
  { prompt: "Hay tom tat doan van ban sau day thanh mot doan ngan", nhanDung: "tom_tat" },
  { prompt: "Dich cau sau sang tieng Anh: Xin chao ban", nhanDung: "dich" },
  { prompt: "Dem so luong don hang thang nay giup toi", nhanDung: "dem_so" },
  { prompt: "Tinh tong doanh thu quy nay", nhanDung: "dem_so" },
  { prompt: "Cho toi y kien ve bai viet nay", nhanDung: "tom_tat" },
  { prompt: "Giup toi chuyen tai lieu nay sang ngon ngu khac", nhanDung: "dich" },
];

function chayThucNghiemPromptDayDu(taoMessages: (prompt: string) => ChatMessage[]): boolean[] {
  return BO_KIEM_THU.map((c) => {
    const chuoiTraVe = llmMoPhongDayDu(taoMessages(c.prompt));
    const ketQuaTool = goiToolPhanLoai(chuoiTraVe);
    return ketQuaTool.loai === "thanh_cong" && ketQuaTool.duLieu.nhiemVu === c.nhanDung;
  });
}

const ketQuaTranTrui = chayThucNghiemPromptDayDu(taoPromptTranTrui);
const ketQuaDayDu = chayThucNghiemPromptDayDu(taoPromptDayDu);
console.log(tinhPassAt1(ketQuaTranTrui), tinhPassAt1(ketQuaDayDu));
```

```typescript title=test
if (JSON.stringify(ketQuaTranTrui) !== JSON.stringify([false, false, false, false, false, false])) {
  throw new Error("prompt tran trui phai SAI CA 6 cau (khong ky thuat nao duoc ap dung)");
}
if (JSON.stringify(ketQuaDayDu) !== JSON.stringify([true, true, true, true, true, true])) {
  throw new Error("prompt day du (rang buoc + few-shot + schema) phai DUNG CA 6 cau");
}
if (tinhPassAt1(ketQuaTranTrui) !== 0) throw new Error("pass@1 tran trui phai la 0");
if (tinhPassAt1(ketQuaDayDu) !== 1) throw new Error("pass@1 day du phai la 1");

const ketQuaChiRangBuoc = chayThucNghiemPromptDayDu(taoPromptChiRangBuoc);
if (tinhPassAt1(ketQuaChiRangBuoc) !== 0) {
  throw new Error("chi rang buoc (khong schema, khong few-shot) van phai la 0 vi dinh dang luon that bai");
}

const ketQuaChiSchemaFewShot = chayThucNghiemPromptDayDu(taoPromptChiSchemaVaFewShot);
if (Math.abs(tinhPassAt1(ketQuaChiSchemaFewShot) - 5 / 6) > 1e-9) {
  throw new Error("chi schema+few-shot (khong rang buoc) phai la 5/6, mot cau mo ho van bi doan sai gia tri");
}
```

:::hints
- kind: attention
  body: "Hai cho trong, ca hai nam trong than cua BO_KIEM_THU.map((c) => {...}). Cho dau: goi llmMoPhongDayDu tren taoMessages(c.prompt) de duoc chuoiTraVe, roi goi goiToolPhanLoai(chuoiTraVe) de duoc ketQuaTool. Cho hai: return true CHI KHI ketQuaTool.loai === 'thanh_cong' VA ketQuaTool.duLieu.nhiemVu === c.nhanDung."
- kind: strategy
  body: "Cho dau: const chuoiTraVe = llmMoPhongDayDu(taoMessages(c.prompt)); const ketQuaTool = goiToolPhanLoai(chuoiTraVe); Cho hai: return ketQuaTool.loai === \"thanh_cong\" && ketQuaTool.duLieu.nhiemVu === c.nhanDung;"
- kind: one-line
  body: "Sao chep dung hai dong o phan Strategy, DUNG THU TU: tinh chuoiTraVe/ketQuaTool truoc, return dieu kien ket hop sau."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 8000
- tier: output
  match: contains
  expect: "0 1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Prompt trần trụi: `pass@1 = 0`. Prompt đầy đủ: `pass@1 = 1`. Hai đối
chứng riêng lẻ xác nhận CẢ HAI trục đều CẦN THIẾT — thiếu MỘT trong ba
kỹ thuật, `pass@1` không thể chạm `1`. Quest `q9.1a` — "Nền tảng prompt
& harness LLM mô phỏng" — khép lại tại `6/6`, VÀ với đó, bài học đầu
tiên của TOÀN BỘ Realm 9 cũng khép lại.
::::

::::reflect{#nghi-lai}
`chayThucNghiemPromptDayDu` không hề PHÁT MINH thêm khái niệm nào mới
— nó gọi ĐÚNG những hàm đã xây RIÊNG lẻ Ở năm bài trước, không viết
lại MỘT dòng logic nào: `phanLoaiNhiemVuCoRangBuoc` quyết định GIÁ
TRỊ, `coChiRoSchemaJson` VÀ `demViDuMau` cùng quyết định ĐỊNH DẠNG,
`goiToolPhanLoai` đo LẠI cả hai trục đó bằng một discriminated union
không gộp chung thất bại. Hai đối chứng — "chỉ ràng buộc" (`0/6`, vì
định dạng luôn hỏng) VÀ "chỉ schema+few-shot" (`5/6`, vì một câu mơ hồ
vẫn sai giá trị) — chứng minh điều quan trọng nhất của cả quest:
GIÁ TRỊ và ĐỊNH DẠNG LÀ hai trục HOÀN TOÀN độc lập trong một hệ thống
prompt engineering thật, VÀ chỉ khi cả ba kỹ thuật cộng lại, `pass@1`
mới chạm `1`.
::::

::::checkpoint{mastery=0.9}
::::
