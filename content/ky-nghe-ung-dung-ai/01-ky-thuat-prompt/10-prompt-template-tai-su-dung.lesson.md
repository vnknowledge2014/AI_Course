---
id: ky-nghe-ung-dung-ai.ky-thuat-prompt.prompt-template-tai-su-dung
title: "Prompt template tái sử dụng: đóng gói kỹ thuật thành một lệnh gọi"
summary: "taoPromptChuan(cauHoi: string): ChatMessage[] LUON lap dung cau truc da chung minh hieu qua o q9.1a: mot message system mang CHI_THI_RANG_BUOC (rang buoc chon-mot-trong-ba, tai dung nguyen van hang so tu q9.1a) + hai cap vi du mau user/assistant CO DINH (VI_DU_1_*, VI_DU_2_*, tai dung nguyen van) + cau hoi that o cuoi -- nguoi goi CHI can truyen MOT chuoi, khong can biet rang buoc hay few-shot la gi. Do: goi taoPromptChuan tren MOI cau trong BO_KIEM_THU (tai dung nguyen van tu q9.1a) qua chayThucNghiemPromptDayDu + llmMoPhongDayDu + goiToolPhanLoai (tai dung nguyen van) -- pass@1 = 6/6, dinh dang bao dam qua NGUONG demViDuMau>=2 (bai few-shot cua q9.1a), khong can nhac schema bang loi rieng."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-prompt
order: 10
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [kna.prompt-template-tai-su-dung]
requires: [kna.rang-buoc-phu-dinh]
concepts: [kna.prompt-template-tai-su-dung]
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
Ba bài vừa qua thêm ba lớp phòng thủ — phân cấp chỉ thị, chống
injection, ràng buộc phủ định. Nhưng MỘT người gọi hàm phân loại mỗi
ngày không nên phải tự tay lắp lại `system` + hai ví dụ mẫu + câu hỏi
mỗi lần. Kỹ nghệ hoá prompt nghĩa LÀ: đóng gói MỘT cấu trúc đã CHỨNG
MINH hiệu quả thành MỘT hàm — người gọi chỉ cần đưa câu hỏi.
::::

::::explain{#dong_goi_thanh_mot_ham}
`taoPromptChuan` LUÔN lắp đúng BA phần theo ĐÚNG một thứ tự cố định:
một message `system` mang `CHI_THI_RANG_BUOC` (ràng buộc chọn-một-
trong-ba, tái dùng nguyên văn hằng số từ `q9.1a`), hai cặp ví dụ mẫu
`user`/`assistant` CỐ ĐỊNH (đúng nội dung `VI_DU_1_*`/`VI_DU_2_*` đã
dùng Ở BOSS `q9.1a` — MỖI ví dụ mẫu ĐÃ LÀ một chuỗi JSON đúng schema,
tự nó truyền đạt "hình dạng response mong đợi" mà KHÔNG cần thêm một
câu chỉ thị schema riêng bằng lời — đúng tinh thần bài `few-shot-sua-
dinh-dang`), rồi câu hỏi thật Ở cuối:

```typescript title=readonly
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}

const CHI_THI_RANG_BUOC = "Chi duoc chon mot trong ba loai: tom_tat, dich, hoac dem_so. Khong duoc tra loi khong_ro.";
const VI_DU_1_USER = "Dem so nguoi tham gia buoi hop nay";
const VI_DU_1_ASSISTANT = JSON.stringify({ nhiemVu: "dem_so", doTinCay: 0.95 });
const VI_DU_2_USER = "Dich cau chao mung sang tieng Phap";
const VI_DU_2_ASSISTANT = JSON.stringify({ nhiemVu: "dich", doTinCay: 0.95 });

function taoPromptChuan(cauHoi: string): ChatMessage[] {
  return [
    { role: "system", content: CHI_THI_RANG_BUOC },
    { role: "user", content: VI_DU_1_USER },
    { role: "assistant", content: VI_DU_1_ASSISTANT },
    { role: "user", content: VI_DU_2_USER },
    { role: "assistant", content: VI_DU_2_ASSISTANT },
    { role: "user", content: cauHoi },
  ];
}
```

Người gọi `taoPromptChuan("Cho toi y kien ve bai viet nay")` không cần
biết BÊN TRONG có ràng buộc gì, hay VÌ SAO cần đúng hai ví dụ mẫu — chi
tiết kỹ thuật đã được ĐÓNG GÓI vào một lệnh gọi DUY NHẤT.
::::

::::example{#do_pass_at1_qua_template}
`chayThucNghiemPromptDayDu` (hàm THẬT của BOSS `q9.1a`, tái dùng nguyên
văn) chạy `taoPromptChuan` trên ĐÚNG bộ `6` prompt của `q9.1a`, qua
`llmMoPhongDayDu` + `goiToolPhanLoai` (cũng tái dùng nguyên văn) —
một câu chỉ tính LÀ đúng khi VỪA định dạng thành công VỪA đúng giá trị:

```typescript title=readonly
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

const ketQua = chayThucNghiemPromptDayDu(taoPromptChuan);
console.log(ketQua, tinhPassAt1(ketQua));
```

```text title=readonly
[true,true,true,true,true,true] 1
```

Người gọi chỉ cần một câu: `chayThucNghiemPromptDayDu(taoPromptChuan)`.
Không cần biết bên trong `taoPromptChuan` có ràng buộc gì, few-shot ra
sao — pass@1 vẫn LÀ `6/6`, y hệt kết quả BOSS của `q9.1a`.
::::

::::predict{#doan_cau_hoi_rong commitOnce}
Gọi `taoPromptChuan("")` — CHUỖI RỖNG làm câu hỏi (không hề chứa "tom
tat", "dich", "dem", "tong", "y kien" hay "ngon ngu khac"). Đưa kết quả
qua `llmMoPhongDayDu` rồi `goiToolPhanLoai`. Trường `loai` VÀ
`duLieu.nhiemVu` của kết quả LÀ gì?

:::opt{correct}
`loai: "thanh_cong"`, `duLieu.nhiemVu: "tom_tat"` — không từ khoá nào
khớp, nhưng `buocChon` LÀ `true` (message `system` LUÔN mang ràng buộc)
nên `phanLoaiNhiemVuCoRangBuoc` rơi vào nhánh mặc định `"tom_tat"`;
định dạng vẫn LÀ JSON hợp lệ vì `demViDuMau(messages) = 2` LUÔN đúng,
bất kể `cauHoi` LÀ gì
:::
:::opt
`loai: "loi_parse"` — một câu hỏi RỖNG khiến `llmMoPhongDayDu` không
biết trả lời gì, nên trả về một chuỗi không hợp lệ
::why
Giả định `llmMoPhongDayDu` có một nhánh xử lý ĐẶC BIỆT cho input rỗng
— nhưng hàm không hề kiểm tra `cauHoi === ""` Ở bất kỳ đâu; nó luôn
chạy đúng NĂM bước đã viết: phân loại, tính độ tin cậy, kiểm định dạng.

Chỗ lệch: `demViDuMau(messages) >= 2` LUÔN đúng cho `taoPromptChuan`
(luôn có đúng hai message `assistant`), không phụ thuộc `cauHoi` — nên
nhánh JSON LUÔN được chọn, không bao giờ rơi vào nhánh lỗi.
::
:::
:::opt
Hàm ném lỗi vì `layNoiDungUserCuoi` không xử lý được chuỗi rỗng
::why
Gần đúng Ở việc bạn nghi ngờ một hàm xử lý chuỗi có thể "gãy" với input
rỗng — trực giác phòng thủ đó có ích trong nhiều ngôn ngữ.

Chỗ lệch: `layNoiDungUserCuoi` chỉ gán `noiDung = tin.content;` — một
chuỗi rỗng vẫn LÀ một `string` hợp lệ, phép gán này không hề ném lỗi.
Toàn bộ pipeline chạy bình thường với `prompt = ""`.
::
:::
::::

::::code{#viet_prompt_template}
Hoàn thiện `taoPromptChuan` — lắp ĐÚNG thứ tự: message `system` mang
`CHI_THI_RANG_BUOC` TRƯỚC, rồi hai cặp ví dụ mẫu (đã viết sẵn), rồi
message `user` mang CHÍNH `cauHoi` truyền vào Ở CUỐI CÙNG.

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
const VI_DU_1_USER = "Dem so nguoi tham gia buoi hop nay";
const VI_DU_1_ASSISTANT = JSON.stringify({ nhiemVu: "dem_so", doTinCay: 0.95 });
const VI_DU_2_USER = "Dich cau chao mung sang tieng Phap";
const VI_DU_2_ASSISTANT = JSON.stringify({ nhiemVu: "dich", doTinCay: 0.95 });

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

function taoPromptChuan(cauHoi: string): ChatMessage[] {
  return [
    ___,
    { role: "user", content: VI_DU_1_USER },
    { role: "assistant", content: VI_DU_1_ASSISTANT },
    { role: "user", content: VI_DU_2_USER },
    { role: "assistant", content: VI_DU_2_ASSISTANT },
    ___,
  ];
}

const ketQua = chayThucNghiemPromptDayDu(taoPromptChuan);
console.log(ketQua, tinhPassAt1(ketQua));
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
const VI_DU_1_USER = "Dem so nguoi tham gia buoi hop nay";
const VI_DU_1_ASSISTANT = JSON.stringify({ nhiemVu: "dem_so", doTinCay: 0.95 });
const VI_DU_2_USER = "Dich cau chao mung sang tieng Phap";
const VI_DU_2_ASSISTANT = JSON.stringify({ nhiemVu: "dich", doTinCay: 0.95 });

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

function taoPromptChuan(cauHoi: string): ChatMessage[] {
  return [
    { role: "system", content: CHI_THI_RANG_BUOC },
    { role: "user", content: VI_DU_1_USER },
    { role: "assistant", content: VI_DU_1_ASSISTANT },
    { role: "user", content: VI_DU_2_USER },
    { role: "assistant", content: VI_DU_2_ASSISTANT },
    { role: "user", content: cauHoi },
  ];
}

const ketQua = chayThucNghiemPromptDayDu(taoPromptChuan);
console.log(ketQua, tinhPassAt1(ketQua));
```

```typescript title=test
if (ketQua.length !== 6) throw new Error("phai co dung 6 ket qua");
if (JSON.stringify(ketQua) !== JSON.stringify([true, true, true, true, true, true])) throw new Error("taoPromptChuan phai dat pass@1 = 6/6 tren BO_KIEM_THU");
if (tinhPassAt1(ketQua) !== 1) throw new Error("pass@1 phai la 1 (6/6)");

const msgsCheck = taoPromptChuan("Cau hoi kiem tra");
if (msgsCheck.length !== 6) throw new Error("taoPromptChuan phai luon lap dung 6 message");
if (msgsCheck[0]!.role !== "system") throw new Error("message dau tien phai la system (rang buoc)");
if (msgsCheck[5]!.role !== "user" || msgsCheck[5]!.content !== "Cau hoi kiem tra") throw new Error("message cuoi cung phai la user chua DUNG cau hoi truyen vao");
```

:::hints
- kind: attention
  body: "Hai cho trong, ca hai la mot phan tu trong mang ma taoPromptChuan tra ve. Cho dau (phan tu DAU TIEN cua mang): { role: 'system', content: CHI_THI_RANG_BUOC }. Cho hai (phan tu CUOI CUNG cua mang): { role: 'user', content: cauHoi }."
- kind: strategy
  body: "Cho dau: { role: \"system\", content: CHI_THI_RANG_BUOC }, Cho hai: { role: \"user\", content: cauHoi },"
- kind: one-line
  body: "Sao chep dung hai phan tu o phan Strategy vao dung vi tri (dau va cuoi mang)."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "[true,true,true,true,true,true] 1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một hàm, một chuỗi truyền vào, `pass@1 = 6/6` LUÔN LUÔN — người gọi
không cần biết ràng buộc hay few-shot LÀ gì. Nhưng "luôn luôn" chỉ
đúng CHỪNG NÀO không ai vô tình "dọn dẹp" cấu trúc bên trong. Bài sau
xây một hàng rào CHỐNG LẠI chính rủi ro đó: bộ kiểm thử hồi quy.
::::

::::reflect{#nghi-lai}
`taoPromptChuan` không phát minh kỹ thuật MỚI nào — nó ĐÓNG GÓI ba kỹ
thuật đã CHỨNG MINH hiệu quả Ở `q9.1a` (ràng buộc sửa trục giá trị,
few-shot sửa trục định dạng) thành MỘT điểm gọi duy nhất. Giá trị kỹ
nghệ không nằm Ở việc phát minh — nó nằm Ở việc giấu ĐI sự phức tạp đã
được kiểm chứng, để người gọi hàm không phải LẶP LẠI quyết định thiết
kế đó mỗi lần, và không thể VÔ TÌNH quên một phần nào của nó. Bài sau
sẽ cho thấy: đóng gói không đồng nghĩa với BẤT BIẾN — một "cải tiến"
tưởng chừng vô hại (bớt một ví dụ mẫu) vẫn có thể phá vỡ hợp đồng đó.
::::

::::checkpoint{mastery=0.83}
::::
