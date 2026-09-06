---
id: ky-nghe-ung-dung-ai.ky-thuat-prompt.bo-kiem-thu-hoi-quy
title: "Bộ kiểm thử hồi quy: bắt lỗi khi \"sửa\" làm hỏng trường hợp khác"
summary: "BO_KIEM_THU_MO_RONG = [...BO_KIEM_THU (tai dung nguyen van tu q9.1a), CAU_MOI_1, CAU_MOI_2] them 2 cau moi: mot cau chua CA HAI tu khoa rong \"y kien\" VA \"ngon ngu khac\" cung luc (kiem tra tinh uu tien nhat quan -- phai la tom_tat vi 'y kien' duoc kiem TRUOC), mot cau chua CA HAI tu khoa cung \"tong\" VA \"tom tat\" cung luc (kiem tra uu tien tu khoa cung -- phai la tom_tat vi kiem TRUOC). chayBoKiemThuHoiQuy(taoMessages, boKiemThu) chay pass@1 tren bo BAT KY duoc truyen vao (tai dung llmMoPhongDayDu + goiToolPhanLoai nguyen van). Do: taoPromptChuan (bai truoc) dat 8/8 tren bo mo rong; taoPromptChuan_HONG (bien the CO Y bo mot trong hai vi du mau, lam demViDuMau tut xuong duoi nguong >=2, KHONG con chi thi schema bang loi nao bu lai) lam dinh dang LUON that bai -> pass@1 sut tu 1 xuong 0 -- bo hoi quy BAT DUOC su sut giam nay TRUOC khi 'cai tien' do duoc trien khai."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-prompt
order: 11
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [kna.bo-kiem-thu-hoi-quy]
requires: [kna.prompt-template-tai-su-dung]
concepts: [kna.bo-kiem-thu-hoi-quy]
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
`taoPromptChuan` đóng gói một cấu trúc đạt `6/6`. Nhưng "đóng gói" chỉ
LÀ một tổ chức code — không có gì NGĂN một người (kể cả chính bạn, sáu
tháng sau) "dọn dẹp" nó, ví dụ bớt một ví dụ mẫu để prompt gọn hơn. Một
bộ kiểm thử HỒI QUY tồn tại đúng để bắt CHÍNH loại lỗi đó: một thay đổi
tưởng chừng vô hại, âm thầm phá một trường hợp đã từng đúng.
::::

::::explain{#mo_rong_bo_kiem_thu}
Mở rộng `BO_KIEM_THU` (tái dùng nguyên văn từ `q9.1a`) thêm hai câu
MỚI, mỗi câu kiểm một dạng "khó" khác nhau: `CAU_MOI_1` chứa CẢ HAI từ
khoá MỀM `"y kien"` và `"ngon ngu khac"` cùng lúc — kiểm tra tính ưu
tiên NHẤT QUÁN (`"y kien"` được kiểm TRƯỚC trong
`phanLoaiNhiemVuCoRangBuoc`, nên nhãn đúng LÀ `"tom_tat"`, không phải
`"dich"`); `CAU_MOI_2` chứa CẢ HAI từ khoá CỨNG `"tong"` và `"tom tat"`
cùng lúc — kiểm tra ưu tiên từ khoá cứng (`"tom tat"` được kiểm TRƯỚC
`"dem"`/`"tong"`, nhãn đúng LÀ `"tom_tat"`):

```typescript title=readonly
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
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

const CAU_MOI_1: CauKiemThu = {
  prompt: "Cho toi y kien ve tai lieu nay, co the chuyen sang ngon ngu khac khong",
  nhanDung: "tom_tat",
};
const CAU_MOI_2: CauKiemThu = {
  prompt: "Toi muon biet tong so don hang va cung muon ban tom tat lai giup toi",
  nhanDung: "tom_tat",
};

const BO_KIEM_THU_MO_RONG: CauKiemThu[] = [...BO_KIEM_THU, CAU_MOI_1, CAU_MOI_2];
console.log(BO_KIEM_THU_MO_RONG.length);
```

```text title=readonly
8
```

Hai câu mới không kiểm tra logic MỚI — chúng kiểm tra CHẶT hơn cùng
một hành vi ưu tiên đã có sẵn Ở `q9.1a`, chỉ với những tổ hợp CHƯA từng
xuất hiện trong bộ gốc. Đây chính LÀ cách một bộ hồi quy LỚN LÊN theo
thời gian: mỗi lần phát hiện một tổ hợp mới đáng lo, thêm MỘT câu, giữ
NGUYÊN mọi câu cũ.
::::

::::example{#bo_hoi_quy_bat_duoc_su_hong}
`chayBoKiemThuHoiQuy` giống hệt `chayThucNghiemPromptDayDu` (bài trước)
nhưng nhận THÊM tham số `boKiemThu` — chạy được trên BẤT KỲ bộ kiểm thử
nào, không hard-code kích thước. `taoPromptChuan_HONG` LÀ một "cải
tiến" GIẢ ĐỊNH: bớt Đi ví dụ mẫu THỨ HAI, có vẻ như chỉ làm prompt gọn
hơn:

```typescript title=readonly
const CHI_THI_RANG_BUOC = "Chi duoc chon mot trong ba loai: tom_tat, dich, hoac dem_so. Khong duoc tra loi khong_ro.";
const VI_DU_1_USER = "Dem so nguoi tham gia buoi hop nay";
const VI_DU_1_ASSISTANT = JSON.stringify({ nhiemVu: "dem_so", doTinCay: 0.95 });

function taoPromptChuan(cauHoi: string): ChatMessage[] {
  return [
    { role: "system", content: CHI_THI_RANG_BUOC },
    { role: "user", content: VI_DU_1_USER },
    { role: "assistant", content: VI_DU_1_ASSISTANT },
    { role: "user", content: "Dich cau chao mung sang tieng Phap" },
    { role: "assistant", content: JSON.stringify({ nhiemVu: "dich", doTinCay: 0.95 }) },
    { role: "user", content: cauHoi },
  ];
}

function taoPromptChuan_HONG(cauHoi: string): ChatMessage[] {
  return [
    { role: "system", content: CHI_THI_RANG_BUOC },
    { role: "user", content: VI_DU_1_USER },
    { role: "assistant", content: VI_DU_1_ASSISTANT },
    // DA XOA cap vi du mau THU HAI o day -- "gon" hon, "toi uu" hon
    { role: "user", content: cauHoi },
  ];
}

function chayBoKiemThuHoiQuy(
  taoMessages: (prompt: string) => ChatMessage[],
  boKiemThu: CauKiemThu[],
): boolean[] {
  return boKiemThu.map((c) => {
    const chuoiTraVe = llmMoPhongDayDu(taoMessages(c.prompt));
    const ketQuaTool = goiToolPhanLoai(chuoiTraVe);
    return ketQuaTool.loai === "thanh_cong" && ketQuaTool.duLieu.nhiemVu === c.nhanDung;
  });
}

const ketQuaChuan = chayBoKiemThuHoiQuy(taoPromptChuan, BO_KIEM_THU_MO_RONG);
const ketQuaHong = chayBoKiemThuHoiQuy(taoPromptChuan_HONG, BO_KIEM_THU_MO_RONG);
console.log("chuan:", ketQuaChuan, tinhPassAt1(ketQuaChuan));
console.log("hong:", ketQuaHong, tinhPassAt1(ketQuaHong));
```

```text title=readonly
chuan: [true,true,true,true,true,true,true,true] 1
hong: [false,false,false,false,false,false,false,false] 0
```

`taoPromptChuan` (đúng) đạt `8/8` trên bộ MỞ RỘNG — hai câu mới cũng
được suy đúng. `taoPromptChuan_HONG` (bớt một ví dụ mẫu) làm
`demViDuMau` tụt xuống `1`, DƯỚI ngưỡng `>= 2` — và vì template này
KHÔNG có câu chỉ thị schema bằng lời nào bù lại (đúng thiết kế đã chọn
Ở bài trước: format chỉ dựa vào few-shot), nhánh JSON KHÔNG BAO GIỜ
được kích hoạt nữa — MỌI câu trả lời rơi về văn xuôi, `goiToolPhanLoai`
luôn thấy `"loi_parse"`. Pass@1 sụp từ `1` xuống `0` — TOÀN BỘ 8 câu
đều sai, dù giá trị phân loại (trục KHÁC) vẫn đúng y hệt bên trong.
Đây chính LÀ giá trị của bộ hồi quy: PHÁT HIỆN được sự sụt giảm NÀY
trước khi "cải tiến" kịp triển khai.
::::

::::predict{#doan_bo_goc_khong_mo_rong commitOnce}
Gọi `chayBoKiemThuHoiQuy(taoPromptChuan, BO_KIEM_THU)` — dùng bộ GỐC
(chỉ `6` câu của `q9.1a`, KHÔNG mở rộng) thay vì `BO_KIEM_THU_MO_RONG`.
`tinhPassAt1` của kết quả LÀ bao nhiêu?

:::opt{correct}
`1` (tức `6/6`) — `chayBoKiemThuHoiQuy` không hard-code kích thước bộ
test; nó CHỈ `.map` qua THAM SỐ `boKiemThu` được truyền vào. Với
`taoPromptChuan` (đúng) và bộ gốc `6` câu — đây CHÍNH LÀ phép đo đã
xác nhận Ở bài trước, không đổi
:::
:::opt
Hàm ném lỗi vì `chayBoKiemThuHoiQuy` được thiết kế RIÊNG cho bộ MỞ
RỘNG `8` câu, không chấp nhận bộ khác kích thước
::why
Giả định có một điều kiện kiểm tra ĐỘ DÀI của `boKiemThu` (ví dụ
`if (boKiemThu.length !== 8) throw ...`) — nhưng chữ ký hàm chỉ nhận
`boKiemThu: CauKiemThu[]`, một MẢNG bất kỳ, và thân hàm chỉ gọi
`.map(...)` trên nó.

Chỗ lệch: `.map` chạy được trên mảng CÓ BAO NHIÊU phần tử cũng được —
không có ràng buộc kích thước nào trong code.
::
:::
:::opt
`0.75` (tức `6/8`) — hai câu mới trong bộ mở rộng LUÔN được coi LÀ
"đã chạy" dù không có mặt trong mảng, làm giảm mẫu số
::why
Nhầm việc "định nghĩa" `CAU_MOI_1`/`CAU_MOI_2` Ở nơi khác trong code
VỚI việc chúng bị TỰ ĐỘNG gộp vào MỌI lời gọi `chayBoKiemThuHoiQuy`.

Chỗ lệch: chỉ NHỮNG phần tử THẬT SỰ có trong mảng `boKiemThu` truyền
vào mới được `.map` xử lý. Gọi hàm với `BO_KIEM_THU` (6 phần tử) thì
`tinhPassAt1` chia cho ĐÚNG `6`, không phải `8`.
::
:::
::::

::::code{#viet_bo_kiem_thu_hoi_quy}
Hoàn thiện: khai báo `BO_KIEM_THU_MO_RONG` bằng cách nối `BO_KIEM_THU`
với `CAU_MOI_1` và `CAU_MOI_2` (đã khai báo sẵn Ở trên). Hoàn thiện
thân `chayBoKiemThuHoiQuy` — với MỖI câu kiểm thử: gọi `llmMoPhongDayDu`
trên messages do `taoMessages` dựng, đưa qua `goiToolPhanLoai`, trả về
`true` CHỈ KHI vừa `thanh_cong` vừa đúng nhãn.

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

function taoPromptChuan_HONG(cauHoi: string): ChatMessage[] {
  return [
    { role: "system", content: CHI_THI_RANG_BUOC },
    { role: "user", content: VI_DU_1_USER },
    { role: "assistant", content: VI_DU_1_ASSISTANT },
    { role: "user", content: cauHoi },
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
const CAU_MOI_1: CauKiemThu = {
  prompt: "Cho toi y kien ve tai lieu nay, co the chuyen sang ngon ngu khac khong",
  nhanDung: "tom_tat",
};
const CAU_MOI_2: CauKiemThu = {
  prompt: "Toi muon biet tong so don hang va cung muon ban tom tat lai giup toi",
  nhanDung: "tom_tat",
};

___

function chayBoKiemThuHoiQuy(
  taoMessages: (prompt: string) => ChatMessage[],
  boKiemThu: CauKiemThu[],
): boolean[] {
  return boKiemThu.map((c) => {
    ___
  });
}

const ketQuaChuan = chayBoKiemThuHoiQuy(taoPromptChuan, BO_KIEM_THU_MO_RONG);
const ketQuaHong = chayBoKiemThuHoiQuy(taoPromptChuan_HONG, BO_KIEM_THU_MO_RONG);
console.log(tinhPassAt1(ketQuaChuan), tinhPassAt1(ketQuaHong));
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

function taoPromptChuan_HONG(cauHoi: string): ChatMessage[] {
  return [
    { role: "system", content: CHI_THI_RANG_BUOC },
    { role: "user", content: VI_DU_1_USER },
    { role: "assistant", content: VI_DU_1_ASSISTANT },
    { role: "user", content: cauHoi },
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
const CAU_MOI_1: CauKiemThu = {
  prompt: "Cho toi y kien ve tai lieu nay, co the chuyen sang ngon ngu khac khong",
  nhanDung: "tom_tat",
};
const CAU_MOI_2: CauKiemThu = {
  prompt: "Toi muon biet tong so don hang va cung muon ban tom tat lai giup toi",
  nhanDung: "tom_tat",
};

const BO_KIEM_THU_MO_RONG: CauKiemThu[] = [...BO_KIEM_THU, CAU_MOI_1, CAU_MOI_2];

function chayBoKiemThuHoiQuy(
  taoMessages: (prompt: string) => ChatMessage[],
  boKiemThu: CauKiemThu[],
): boolean[] {
  return boKiemThu.map((c) => {
    const chuoiTraVe = llmMoPhongDayDu(taoMessages(c.prompt));
    const ketQuaTool = goiToolPhanLoai(chuoiTraVe);
    return ketQuaTool.loai === "thanh_cong" && ketQuaTool.duLieu.nhiemVu === c.nhanDung;
  });
}

const ketQuaChuan = chayBoKiemThuHoiQuy(taoPromptChuan, BO_KIEM_THU_MO_RONG);
const ketQuaHong = chayBoKiemThuHoiQuy(taoPromptChuan_HONG, BO_KIEM_THU_MO_RONG);
console.log(tinhPassAt1(ketQuaChuan), tinhPassAt1(ketQuaHong));
```

```typescript title=test
if (tinhPassAt1(ketQuaChuan) !== 1) throw new Error("taoPromptChuan phai dat 8/8 tren bo mo rong");
if (tinhPassAt1(ketQuaHong) !== 0) throw new Error("taoPromptChuan_HONG (thieu 1 vi du) phai lam pass@1 sut ve 0 -- day la gia tri cua bo hoi quy: bat duoc su sut giam");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (mot khai bao const o cap module, TRUOC ham chayBoKiemThuHoiQuy): BO_KIEM_THU_MO_RONG = mang noi BO_KIEM_THU voi CAU_MOI_1 va CAU_MOI_2 bang toan tu spread (...). Cho hai (than callback cua .map): tinh chuoiTraVe qua llmMoPhongDayDu(taoMessages(c.prompt)), roi ketQuaTool qua goiToolPhanLoai(chuoiTraVe), return true CHI KHI ketQuaTool.loai === 'thanh_cong' VA ketQuaTool.duLieu.nhiemVu === c.nhanDung."
- kind: strategy
  body: "Cho dau: const BO_KIEM_THU_MO_RONG: CauKiemThu[] = [...BO_KIEM_THU, CAU_MOI_1, CAU_MOI_2]; Cho hai: const chuoiTraVe = llmMoPhongDayDu(taoMessages(c.prompt)); const ketQuaTool = goiToolPhanLoai(chuoiTraVe); return ketQuaTool.loai === \"thanh_cong\" && ketQuaTool.duLieu.nhiemVu === c.nhanDung;"
- kind: one-line
  body: "Sao chep dung logic o phan Strategy vao dung vi tri tuong ung."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 8000
- tier: output
  match: contains
  expect: "1 0"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`pass@1` từ `1` sụp xuống `0` — không cần chờ người dùng thật báo lỗi,
bộ hồi quy bắt được ngay khi "cải tiến" đó vừa được viết. Năm kỹ thuật
đã học — phân cấp chỉ thị, phòng thủ injection, ràng buộc phủ định,
template tái sử dụng, hồi quy — giờ sẵn sàng RÁP lại thành MỘT hệ
thống, đo đồng thời cả ba mặt: chính xác, an toàn, ổn định.
::::

::::reflect{#nghi-lai}
Một bộ kiểm thử hồi quy không NGĂN được người viết code phạm sai lầm —
nó chỉ đảm bảo sai lầm đó bị PHÁT HIỆN trước khi tới tay người dùng.
`taoPromptChuan_HONG` không sai Ở CÚ PHÁP, không sai Ở KIỂU — nó biên
dịch sạch, chạy được, và với NGƯỜI ĐỌC lướt qua, trông như một phiên
bản GỌN hơn. Chỉ có việc CHẠY LẠI toàn bộ `BO_KIEM_THU_MO_RONG` và so
`pass@1` MỚI lộ ra: mọi câu đều hỏng, vì một điều kiện ngưỡng
(`demViDuMau >= 2`) đã đủ dữ liệu vào để rẽ nhánh SAI. Giá trị của một
bộ hồi quy không nằm Ở việc nó THÔNG MINH — nó nằm Ở việc nó ĐẦY ĐỦ và
được CHẠY LẠI mỗi lần có thay đổi, dù thay đổi đó trông nhỏ tới đâu.
::::

::::checkpoint{mastery=0.85}
::::
