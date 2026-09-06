---
id: ky-nghe-ung-dung-ai.ky-thuat-prompt.rang-buoc-phu-dinh
title: "Ràng buộc phủ định: \"không được phân loại là X\""
summary: "coBiCam(messages: ChatMessage[], loaiBiCam: \"tom_tat\"|\"dich\"|\"dem_so\"): boolean do system message chua cum \"khong duoc phan loai la \" + loaiBiCam. phanLoaiCoRangBuocPhuDinh(messages) GOI phanLoaiNhiemVuCoRangBuoc (q9.1a) lay ket qua THO; neu ket qua do trung MOT loai bi cam, chuyen sang loai theo THU TU uu tien co dinh (\"tom_tat\" -> neu cam thi \"dem_so\" -> neu cam luon thi \"dich\"), lap qua CA BA neu can. Do: mot prompt RO RANG la \"dich\" (tu khoa cung khop) nhung he thong cam \"dich\" -- ket qua doi sang \"tom_tat\" (dau tien trong thu tu uu tien khong bi cam), KHONG con tra ve \"dich\". Rieng biet: mot cau mo ho \"y kien\" ra tom_tat nhung tom_tat bi cam CA HAI (tom_tat VA dem_so bi cam cung luc) -- ket qua phai nhay qua ca hai, dung o \"dich\" (loai DUY NHAT con lai)."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-prompt
order: 9
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [kna.rang-buoc-phu-dinh]
requires: [kna.phong-thu-prompt-injection]
concepts: [kna.rang-buoc-phu-dinh]
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
Mọi ràng buộc học được cho tới giờ đều LÀ ràng buộc DƯƠNG: "phải chọn
một trong ba". Nhưng một hệ thống production thường cần điều NGƯỢC
lại: "được chọn gì cũng được, TRỪ loại này" — ví dụ một tính năng bị
tắt tạm thời, hoặc một chính sách cấm một loại phản hồi cụ thể. Ràng
buộc PHỦ ĐỊNH đòi một cơ chế khác: không chỉ ĐỌC ràng buộc, mà còn phải
SỬA kết quả khi nó vi phạm.
::::

::::explain{#rang-buoc-phu-dinh-va-thu-tu-uu-tien}
`coBiCam` dò system message chứa đúng cụm
`"khong duoc phan loai la " + loaiBiCam` — tham số hoá theo TỪNG loại
cụ thể, không gộp chung một cụm cấm-tất-cả:

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

function coBiCam(messages: ChatMessage[], loaiBiCam: "tom_tat" | "dich" | "dem_so"): boolean {
  return messages.some(
    (tin) => tin.role === "system" && tin.content.toLowerCase().includes("khong duoc phan loai la " + loaiBiCam),
  );
}
```

`phanLoaiCoRangBuocPhuDinh` gọi `phanLoaiNhiemVuCoRangBuoc` (hàm THẬT
của `q9.1a`, tái dùng nguyên văn) để lấy kết quả THÔ. Nếu kết quả đó
trùng một loại bị cấm, nó DUYỆT một danh sách ưu tiên CỐ ĐỊNH —
`"tom_tat"` trước, `"dem_so"` kế tiếp, `"dich"` cuối cùng — và trả về
loại ĐẦU TIÊN trong danh sách đó KHÔNG bị cấm:

```typescript title=readonly
function phanLoaiCoRangBuocPhuDinh(messages: ChatMessage[]): "tom_tat" | "dich" | "dem_so" | "khong_ro" {
  const ketQuaTho = phanLoaiNhiemVuCoRangBuoc(messages);
  if (ketQuaTho !== "khong_ro" && coBiCam(messages, ketQuaTho)) {
    const thuTuUuTien: Array<"tom_tat" | "dem_so" | "dich"> = ["tom_tat", "dem_so", "dich"];
    for (const loai of thuTuUuTien) {
      if (!coBiCam(messages, loai)) return loai;
    }
  }
  return ketQuaTho;
}

const heThongCamDich: ChatMessage = {
  role: "system",
  content: "Chi duoc chon mot trong ba loai: tom_tat, dich, hoac dem_so. Khong duoc tra loi khong_ro. Khong duoc phan loai la dich.",
};
const msgsRoDichBiCam: ChatMessage[] = [
  heThongCamDich,
  { role: "user", content: "Dich cau sau sang tieng Anh: Xin chao ban" },
];

console.log("tho (truoc phu dinh):", phanLoaiNhiemVuCoRangBuoc(msgsRoDichBiCam));
console.log("sau phu dinh:", phanLoaiCoRangBuocPhuDinh(msgsRoDichBiCam));
```

```text title=readonly
tho (truoc phu dinh): dich
sau phu dinh: tom_tat
```

Prompt CHỨA từ khoá cứng `"dich"` — `phanLoaiNhiemVuCoRangBuoc` trả về
đúng `"dich"`, không hề biết Ở đây có ràng buộc phủ định nào. NHƯNG hệ
thống cấm chính xác `"dich"` — `phanLoaiCoRangBuocPhuDinh` phát hiện
điều đó, duyệt danh sách ưu tiên, thấy `"tom_tat"` KHÔNG bị cấm, và trả
về `"tom_tat"` thay vì loại bị cấm.
::::

::::predict{#doan_cam_hai_loai_cung_luc commitOnce}
Hệ thống cấm CẢ HAI: `"khong duoc phan loai la tom_tat"` VÀ
`"khong duoc phan loai la dem_so"` — cùng lúc, trong CÙNG một message
`system`. Câu hỏi mơ hồ `"Cho toi y kien ve bai viet nay"` (khiến kết
quả THÔ LÀ `"tom_tat"` qua nhánh `"y kien"`). `phanLoaiCoRangBuocPhuDinh`
trả về gì?

:::opt{correct}
`"dich"` — kết quả thô `"tom_tat"` bị cấm, vòng lặp ưu tiên bắt đầu
kiểm: `"tom_tat"` bị cấm (bỏ qua), `"dem_so"` CŨNG bị cấm (bỏ qua),
`"dich"` KHÔNG bị cấm — trả về `"dich"`, dù kết quả thô ban đầu không
hề liên quan tới `"dich"`
:::
:::opt
`"tom_tat"` — giữ nguyên kết quả thô, vì ràng buộc phủ định chỉ áp dụng
khi CHỈ MỘT loại bị cấm, không áp dụng khi nhiều loại bị cấm cùng lúc
::why
Giả định vòng lặp `for` có một điều kiện đặc biệt "dừng lại nếu có
nhiều hơn một loại bị cấm" — nhưng vòng lặp không hề đếm SỐ loại bị
cấm; nó chỉ kiểm TỪNG loại theo thứ tự, bỏ qua loại nào bị cấm, tới khi
gặp loại ĐẦU TIÊN không bị cấm.

Chỗ lệch: `"tom_tat"` bị cấm nên nhánh `if (!coBiCam(messages, loai))
return loai;` KHÔNG chạy tại bước đầu — vòng lặp tiếp tục sang
`"dem_so"`, cũng bị cấm, tiếp tục sang `"dich"`.
::
:::
:::opt
`"khong_ro"` — khi CẢ HAI lựa chọn ưu tiên đầu tiên đều bị cấm, hệ
thống nên rơi về trạng thái "không xác định được" thay vì đoán bừa
::why
Gần đúng Ở việc bạn nhận ra tình huống này "khó" hơn trường hợp cấm
một loại — quan sát đó đúng.

Chỗ lệch: `thuTuUuTien` liệt kê ĐỦ BA loại (`"tom_tat"`, `"dem_so"`,
`"dich"`), và vòng lặp duyệt tới hết nếu cần — không có nhánh nào trả
về `"khong_ro"` bên trong khối `if` này; giá trị đó CHỈ xuất hiện Ở
`return ketQuaTho;` cuối hàm, và `ketQuaTho` Ở đây LÀ `"tom_tat"`,
không phải `"khong_ro"`.
::
:::
::::

::::code{#viet_rang_buoc_phu_dinh}
Hoàn thiện `coBiCam` — trả về `true` nếu CÓ message `system` chứa cụm
`"khong duoc phan loai la " + loaiBiCam` (đã lowercase). Hoàn thiện
nhánh phủ định trong `phanLoaiCoRangBuocPhuDinh` — khi kết quả thô bị
cấm, duyệt `thuTuUuTien` VÀ trả về loại ĐẦU TIÊN không bị cấm.

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

function coBiCam(messages: ChatMessage[], loaiBiCam: "tom_tat" | "dich" | "dem_so"): boolean {
  ___
}

function phanLoaiCoRangBuocPhuDinh(messages: ChatMessage[]): "tom_tat" | "dich" | "dem_so" | "khong_ro" {
  const ketQuaTho = phanLoaiNhiemVuCoRangBuoc(messages);
  if (ketQuaTho !== "khong_ro" && coBiCam(messages, ketQuaTho)) {
    const thuTuUuTien: Array<"tom_tat" | "dem_so" | "dich"> = ["tom_tat", "dem_so", "dich"];
    ___
  }
  return ketQuaTho;
}

const heThongCamDich: ChatMessage = {
  role: "system",
  content: "Chi duoc chon mot trong ba loai: tom_tat, dich, hoac dem_so. Khong duoc tra loi khong_ro. Khong duoc phan loai la dich.",
};
const msgsRoDichBiCam: ChatMessage[] = [
  heThongCamDich,
  { role: "user", content: "Dich cau sau sang tieng Anh: Xin chao ban" },
];
console.log(phanLoaiNhiemVuCoRangBuoc(msgsRoDichBiCam), phanLoaiCoRangBuocPhuDinh(msgsRoDichBiCam));
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

function coBiCam(messages: ChatMessage[], loaiBiCam: "tom_tat" | "dich" | "dem_so"): boolean {
  return messages.some(
    (tin) => tin.role === "system" && tin.content.toLowerCase().includes("khong duoc phan loai la " + loaiBiCam),
  );
}

function phanLoaiCoRangBuocPhuDinh(messages: ChatMessage[]): "tom_tat" | "dich" | "dem_so" | "khong_ro" {
  const ketQuaTho = phanLoaiNhiemVuCoRangBuoc(messages);
  if (ketQuaTho !== "khong_ro" && coBiCam(messages, ketQuaTho)) {
    const thuTuUuTien: Array<"tom_tat" | "dem_so" | "dich"> = ["tom_tat", "dem_so", "dich"];
    for (const loai of thuTuUuTien) {
      if (!coBiCam(messages, loai)) return loai;
    }
  }
  return ketQuaTho;
}

const heThongCamDich: ChatMessage = {
  role: "system",
  content: "Chi duoc chon mot trong ba loai: tom_tat, dich, hoac dem_so. Khong duoc tra loi khong_ro. Khong duoc phan loai la dich.",
};
const msgsRoDichBiCam: ChatMessage[] = [
  heThongCamDich,
  { role: "user", content: "Dich cau sau sang tieng Anh: Xin chao ban" },
];
console.log(phanLoaiNhiemVuCoRangBuoc(msgsRoDichBiCam), phanLoaiCoRangBuocPhuDinh(msgsRoDichBiCam));
```

```typescript title=test
if (coBiCam([heThongCamDich], "dich") !== true) throw new Error("dich phai duoc nhan dien la bi cam");
if (coBiCam([heThongCamDich], "tom_tat") !== false) throw new Error("tom_tat khong bi cam trong vi du nay");

if (phanLoaiCoRangBuocPhuDinh(msgsRoDichBiCam) !== "tom_tat") throw new Error("dich bi cam thi phai chuyen sang tom_tat (dau tien trong thu tu uu tien)");

const heThongCamTomTat: ChatMessage = {
  role: "system",
  content: "Chi duoc chon mot trong ba loai: tom_tat, dich, hoac dem_so. Khong duoc tra loi khong_ro. Khong duoc phan loai la tom_tat.",
};
const msgsYKienCamTomTat: ChatMessage[] = [heThongCamTomTat, { role: "user", content: "Cho toi y kien ve bai viet nay" }];
if (phanLoaiNhiemVuCoRangBuoc(msgsYKienCamTomTat) !== "tom_tat") throw new Error("truoc phu dinh, cau mo ho 'y kien' phai la tom_tat (hanh vi cu khong doi)");
if (phanLoaiCoRangBuocPhuDinh(msgsYKienCamTomTat) !== "dem_so") throw new Error("tom_tat bi cam thi phai chuyen sang dem_so (thu hai trong thu tu uu tien)");

const heThongKhongCam: ChatMessage = {
  role: "system",
  content: "Chi duoc chon mot trong ba loai: tom_tat, dich, hoac dem_so. Khong duoc tra loi khong_ro.",
};
const msgsDemSo: ChatMessage[] = [heThongKhongCam, { role: "user", content: "Dem so don hang thang nay" }];
if (phanLoaiCoRangBuocPhuDinh(msgsDemSo) !== "dem_so") throw new Error("khong bi cam gi thi ket qua phai giu nguyen");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (than coBiCam): return messages.some voi dieu kien tin.role === 'system' VA tin.content.toLowerCase().includes cum ghep tu 'khong duoc phan loai la ' + loaiBiCam. Cho hai (ben trong khoi if cua phanLoaiCoRangBuocPhuDinh, SAU dong khai bao thuTuUuTien): mot vong for-of duyet thuTuUuTien, neu (!coBiCam(messages, loai)) thi return loai."
- kind: strategy
  body: "Cho dau: return messages.some((tin) => tin.role === \"system\" && tin.content.toLowerCase().includes(\"khong duoc phan loai la \" + loaiBiCam)); Cho hai: for (const loai of thuTuUuTien) { if (!coBiCam(messages, loai)) return loai; }"
- kind: one-line
  body: "Sao chep dung logic o phan Strategy vao dung vi tri tuong ung."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "dich tom_tat"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một prompt RÕ dịch, một hệ thống cấm dịch — kết quả đổi sang loại KHÁC
trong danh sách ưu tiên, không bao giờ trả về loại bị cấm. Ba kỹ
thuật đã học — phân cấp chỉ thị, phòng thủ injection, ràng buộc phủ
định — đều LÀ những mảnh RỜI. Bài sau đóng gói chúng thành MỘT lệnh
gọi duy nhất, để người dùng của hàm không cần biết chi tiết bên trong.
::::

::::reflect{#nghi-lai}
Ràng buộc phủ định đòi hỏi một bước mà ràng buộc dương không cần: SỬA
kết quả sau khi đã có, không chỉ MỞ RỘNG cách suy luận ra kết quả đó.
`phanLoaiCoRangBuocPhuDinh` không "hiểu" LÝ DO một loại bị cấm — nó chỉ
kiểm tra CÓ bị cấm hay không, rồi máy móc duyệt một danh sách ưu tiên
cố định cho tới khi tìm được một lựa chọn hợp lệ. Thứ tự ưu tiên đó
(`tom_tat` → `dem_so` → `dich`) LÀ một quyết định kỹ nghệ CỤ THỂ — thay
đổi thứ tự này thì các câu bị cấm sẽ đổi sang một loại KHÁC, chứng minh
thứ tự đó thật sự LÀ dữ liệu điều khiển hành vi, không phải chi tiết
trang trí.
::::

::::checkpoint{mastery=0.81}
::::
