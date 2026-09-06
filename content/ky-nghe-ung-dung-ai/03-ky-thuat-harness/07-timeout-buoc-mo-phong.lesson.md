---
id: ky-nghe-ung-dung-ai.ky-thuat-harness.timeout-buoc-mo-phong
title: "Timeout — chặn tool treo bằng ngân sách bước mô phỏng"
summary: "taoToolTre() mô phỏng tool KHÔNG BAO GIỜ trả lời: trangThai.treo=true, mọi lần gọi thuMotBuoc() đều trả về null (chưa xong, không phải lỗi). goiToolCoTimeout(tool, gioiHanBuoc) đếm SỐ BƯỚC mô phỏng qua một vòng for (không phải mili-giây thật — sandbox không có real-time timer đáng tin): lặp tối đa gioiHanBuoc lần gọi thuMotBuoc(); NẾU có kết quả khác null thì trả về NGAY; NẾU hết ngân sách mà vẫn null thì trả lỗi 'loi_timeout' — KHÔNG đợi vô hạn. Trên 5 tác vụ dùng taoToolTre() MỚI mỗi tác vụ, gioiHanBuoc=3: completion rate = 0 (cả 5 timeout), MỖI tool bị hỏi ĐÚNG 3 lần trước khi bị bỏ cuộc. Đối chứng taoToolBinhThuong(): trả lời NGAY lần gọi đầu (soLanDaThu=1), không dùng hết ngân sách. Gotcha đo được: ngân sách 0 nghĩa là KHÔNG một lần thuMotBuoc() nào được gọi — khác hẳn quy ước 'ngân sách 0 vẫn gọi 1 lần' của retry-budget (bài 4), vì goiToolCoTimeout không có lệnh gọi nào NGOÀI vòng lặp."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-harness
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [kna.timeout-buoc-mo-phong]
requires: [kna.boss-harness-don-gian-vs-day-du]
concepts: [kna.timeout-buoc-mo-phong]
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
q9.3a đóng lại VỚI retry, fallback VÀ ROP — nhưng cả ba đều giả định
một điều: tool LUÔN trả lời, dù thành công hay thất bại. Có một chế
độ hỏng khác hẳn — tool KHÔNG BAO GIỜ trả lời gì cả, không thành công
cũng không thất bại, chỉ TREO mãi. q9.3b bắt đầu đúng ở đó: khi model
tốt, tool tốt, nhưng một LẦN GỌI cụ thể không bao giờ kết thúc, harness
phải tự đặt ra một GIỚI HẠN, thay vì đợi vô thời hạn.
::::

::::explain{#tool_treo_va_gioi_han_buoc}
Môi trường học này không có đồng hồ thật đáng tin (không
`setTimeout` thật, không `Date.now()` để đo mili-giây trôi qua) — nên
"treo" được mô phỏng bằng MỘT CỜ tất định, `trangThai.treo`, VÀ
"chờ" được mô phỏng bằng một BỘ ĐẾM BƯỚC, không phải thời gian thực.
`taoToolTre` dựng một tool có `treo: true`: MỖI lần `thuMotBuoc()`
được gọi, nó LUÔN trả về `null` — nghĩa LÀ "chưa xong", không phải
thành công, cũng không phải thất bại:

```typescript title=readonly
type KetQua<T, E> =
  | { thanhCong: true; giaTri: T }
  | { thanhCong: false; loi: E };

type KetQuaGoiTool = KetQua<string, string>;

type TrangThaiTre = { treo: boolean; soLanDaThu: number };

interface ToolCoTre {
  trangThai: TrangThaiTre;
  thuMotBuoc(): KetQuaGoiTool | null;
}

function taoToolTre(): ToolCoTre {
  const trangThai: TrangThaiTre = { treo: true, soLanDaThu: 0 };
  return {
    trangThai,
    thuMotBuoc(): KetQuaGoiTool | null {
      trangThai.soLanDaThu++;
      return null;
    },
  };
}

function taoToolBinhThuong(): ToolCoTre {
  const trangThai: TrangThaiTre = { treo: false, soLanDaThu: 0 };
  return {
    trangThai,
    thuMotBuoc(): KetQuaGoiTool | null {
      trangThai.soLanDaThu++;
      return { thanhCong: true, giaTri: "ket_qua_binh_thuong" };
    },
  };
}

function goiToolCoTimeout(tool: ToolCoTre, gioiHanBuoc: number): KetQuaGoiTool {
  for (let buoc = 0; buoc < gioiHanBuoc; buoc++) {
    const ketQua = tool.thuMotBuoc();
    if (ketQua !== null) return ketQua;
  }
  return { thanhCong: false, loi: "loi_timeout" };
}

const tre = taoToolTre();
const ketQuaTre = goiToolCoTimeout(tre, 3);
console.log(JSON.stringify(ketQuaTre), tre.trangThai.soLanDaThu);

const binhThuong = taoToolBinhThuong();
const ketQuaBinhThuong = goiToolCoTimeout(binhThuong, 3);
console.log(JSON.stringify(ketQuaBinhThuong), binhThuong.trangThai.soLanDaThu);
```

```text title=readonly
{"thanhCong":false,"loi":"loi_timeout"} 3
{"thanhCong":true,"giaTri":"ket_qua_binh_thuong"} 1
```

`goiToolCoTimeout` KHÔNG đợi tool trả lời vô thời hạn — nó hỏi tool
ĐÚNG `gioiHanBuoc` lần (Ở đây LÀ `3`), VÀ nếu vẫn `null` sau từng đó
lần hỏi, nó tự QUYẾT ĐỊNH dừng lại, trả về `"loi_timeout"`. Tool
`treo` bị hỏi ĐÚNG `3` lần (khớp `gioiHanBuoc`) rồi bị bỏ cuộc — KHÔNG
phải vòng lặp vô hạn. Tool bình thường trả lời NGAY lần hỏi đầu tiên
(`soLanDaThu` LÀ `1`, không phải `3`) — `goiToolCoTimeout` không hỏi
thêm khi đã có kết quả.
::::

::::example{#chay_tren_n_tac_vu_va_ngan_sach_0}
Chạy `goiToolCoTimeout` trên `5` tác vụ, MỖI tác vụ MỘT `taoToolTre()`
mới, ngân sách `3` bước — VÀ kiểm tra riêng trường hợp ngân sách LÀ
`0`:

```typescript title=readonly
type KetQua<T, E> =
  | { thanhCong: true; giaTri: T }
  | { thanhCong: false; loi: E };

type KetQuaGoiTool = KetQua<string, string>;

type TrangThaiTre = { treo: boolean; soLanDaThu: number };

interface ToolCoTre {
  trangThai: TrangThaiTre;
  thuMotBuoc(): KetQuaGoiTool | null;
}

function taoToolTre(): ToolCoTre {
  const trangThai: TrangThaiTre = { treo: true, soLanDaThu: 0 };
  return {
    trangThai,
    thuMotBuoc(): KetQuaGoiTool | null {
      trangThai.soLanDaThu++;
      return null;
    },
  };
}

function goiToolCoTimeout(tool: ToolCoTre, gioiHanBuoc: number): KetQuaGoiTool {
  for (let buoc = 0; buoc < gioiHanBuoc; buoc++) {
    const ketQua = tool.thuMotBuoc();
    if (ketQua !== null) return ketQua;
  }
  return { thanhCong: false, loi: "loi_timeout" };
}

function tinhTyLeHoanThanh(ketQua: KetQuaGoiTool[]): number {
  if (ketQua.length === 0) return 0;
  return ketQua.filter((k) => k.thanhCong).length / ketQua.length;
}

const SO_TAC_VU = 5;
const ketQuaTimeout: KetQuaGoiTool[] = [];
for (let i = 0; i < SO_TAC_VU; i++) {
  const tool = taoToolTre();
  ketQuaTimeout.push(goiToolCoTimeout(tool, 3));
}
console.log(JSON.stringify(ketQuaTimeout.map((k) => k.thanhCong)));
console.log(tinhTyLeHoanThanh(ketQuaTimeout));

const tool0 = taoToolTre();
goiToolCoTimeout(tool0, 0);
console.log("ngan sach 0:", tool0.trangThai.soLanDaThu);
```

```text title=readonly
[false,false,false,false,false]
0
ngan sach 0: 0
```

Completion rate LÀ `0` trên CẢ `5` tác vụ — tool `treo` không bao giờ
tự khỏi, VÀ đây LÀ bài duy nhất trong track chưa có cơ chế phục hồi
nào cho lớp lỗi này (bài BOSS cuối track sẽ ráp timeout VỚI fallback).
Dòng cuối đáng chú ý nhất: ngân sách `0` khiến `soLanDaThu` LÀ `0` —
KHÔNG một lần `thuMotBuoc()` nào được gọi. Khác VỚI
`goiToolCoRetryCoGioiHan` (bài `4`), nơi ngân sách `0` VẪN gọi tool
đúng `1` lần — vì Ở đó có MỘT lệnh gọi nằm NGOÀI vòng `while`
(`let ketQua = tool.goi();`). `goiToolCoTimeout` không có lệnh gọi
nào NGOÀI vòng `for` cả — MỌI lần gọi `thuMotBuoc()` đều LÀ một vòng
lặp của `for`, nên ngân sách `0` nghĩa LÀ đúng `0` lần gọi.
::::

::::predict{#doan-ngan-sach-0-buoc commitOnce}
Gọi `goiToolCoTimeout(taoToolTre(), 0)` — ngân sách LÀ `0` bước. Kết
quả trả về LÀ gì, VÀ `thuMotBuoc()` có được gọi lần nào không?

:::opt{correct}
Kết quả LÀ timeout (`{ thanhCong: false, loi: "loi_timeout" }`), VÀ
`thuMotBuoc()` KHÔNG được gọi lần nào (`soLanDaThu` vẫn LÀ `0`) — điều
kiện `buoc (0) < gioiHanBuoc (0)` LÀ `false` ngay từ đầu, vòng `for`
không chạy một vòng nào
:::
:::opt
Kết quả LÀ timeout, nhưng `thuMotBuoc()` VẪN được gọi đúng MỘT lần
trước khi bỏ cuộc — giống cách `goiToolCoRetryCoGioiHan` (bài `4`) vẫn
gọi tool đúng một lần dù ngân sách retry LÀ `0`
::why
Nhầm quy ước của bài `4` VỚI quy ước của bài NÀY — Ở bài `4`, lần gọi
ĐẦU TIÊN (`let ketQua = tool.goi();`) nằm NGOÀI vòng `while`, nên ngân
sách `0` vẫn gọi đúng `1` lần trước khi vòng lặp được xét tới.

Chỗ lệch: `goiToolCoTimeout` có thiết kế KHÁC — KHÔNG có lệnh gọi
`thuMotBuoc()` nào nằm NGOÀI vòng `for`, mọi lần gọi đều nằm BÊN TRONG
thân vòng lặp. Ngân sách `0` khiến điều kiện vòng lặp sai ngay từ vòng
đầu tiên, nên không một lần gọi nào xảy ra.
::
:::
:::opt
Không xác định được — `goiToolCoTimeout` không xử lý được ngân sách
bằng `0`, chương trình sẽ ném lỗi
::why
Nhầm rằng ngân sách `0` LÀ một giá trị bất hợp lệ cần kiểm tra trước —
nhưng `goiToolCoTimeout` không hề kiểm tra `gioiHanBuoc` có hợp lệ hay
không, nó chỉ dùng thẳng giá trị đó LÀM điều kiện dừng của vòng `for`.

Chỗ lệch: `buoc < gioiHanBuoc` VỚI `gioiHanBuoc = 0` đánh giá gọn gàng
thành `false` — một biểu thức boolean bình thường, không có nhánh nào
ném lỗi Ở đây cả.
::
:::
::::

::::code{#viet_goi_tool_co_timeout}
Hoàn thiện `goiToolCoTimeout` — lặp `for` tối đa `gioiHanBuoc` lần,
mỗi lần gọi `tool.thuMotBuoc()`; NẾU kết quả đó khác `null`, trả về
NGAY; nếu vòng lặp kết thúc mà chưa từng trả về (nghĩa LÀ tool vẫn
`treo` sau hết ngân sách), trả về `{ thanhCong: false, loi:
"loi_timeout" }`. Hoàn thiện `chayHarnessTimeoutTrenNTacVu` — với MỖI
trong `soTacVu` tác vụ: tạo một `taoToolTre()` MỚI, gọi
`goiToolCoTimeout(tool, gioiHanBuoc)`, đẩy kết quả vào mảng trả về.

```typescript title=starter
type KetQua<T, E> =
  | { thanhCong: true; giaTri: T }
  | { thanhCong: false; loi: E };

type KetQuaGoiTool = KetQua<string, string>;

type TrangThaiTre = { treo: boolean; soLanDaThu: number };

interface ToolCoTre {
  trangThai: TrangThaiTre;
  thuMotBuoc(): KetQuaGoiTool | null;
}

function taoToolTre(): ToolCoTre {
  const trangThai: TrangThaiTre = { treo: true, soLanDaThu: 0 };
  return {
    trangThai,
    thuMotBuoc(): KetQuaGoiTool | null {
      trangThai.soLanDaThu++;
      return null;
    },
  };
}

function taoToolBinhThuong(): ToolCoTre {
  const trangThai: TrangThaiTre = { treo: false, soLanDaThu: 0 };
  return {
    trangThai,
    thuMotBuoc(): KetQuaGoiTool | null {
      trangThai.soLanDaThu++;
      return { thanhCong: true, giaTri: "ket_qua_binh_thuong" };
    },
  };
}

function tinhTyLeHoanThanh(ketQua: KetQuaGoiTool[]): number {
  if (ketQua.length === 0) return 0;
  return ketQua.filter((k) => k.thanhCong).length / ketQua.length;
}

function goiToolCoTimeout(tool: ToolCoTre, gioiHanBuoc: number): KetQuaGoiTool {
  ___
}

function chayHarnessTimeoutTrenNTacVu(soTacVu: number, gioiHanBuoc: number): KetQuaGoiTool[] {
  ___
}

const ketQua5 = chayHarnessTimeoutTrenNTacVu(5, 3);
console.log(JSON.stringify(ketQua5.map((k) => k.thanhCong)), tinhTyLeHoanThanh(ketQua5));
```

```typescript title=solution
type KetQua<T, E> =
  | { thanhCong: true; giaTri: T }
  | { thanhCong: false; loi: E };

type KetQuaGoiTool = KetQua<string, string>;

type TrangThaiTre = { treo: boolean; soLanDaThu: number };

interface ToolCoTre {
  trangThai: TrangThaiTre;
  thuMotBuoc(): KetQuaGoiTool | null;
}

function taoToolTre(): ToolCoTre {
  const trangThai: TrangThaiTre = { treo: true, soLanDaThu: 0 };
  return {
    trangThai,
    thuMotBuoc(): KetQuaGoiTool | null {
      trangThai.soLanDaThu++;
      return null;
    },
  };
}

function taoToolBinhThuong(): ToolCoTre {
  const trangThai: TrangThaiTre = { treo: false, soLanDaThu: 0 };
  return {
    trangThai,
    thuMotBuoc(): KetQuaGoiTool | null {
      trangThai.soLanDaThu++;
      return { thanhCong: true, giaTri: "ket_qua_binh_thuong" };
    },
  };
}

function tinhTyLeHoanThanh(ketQua: KetQuaGoiTool[]): number {
  if (ketQua.length === 0) return 0;
  return ketQua.filter((k) => k.thanhCong).length / ketQua.length;
}

function goiToolCoTimeout(tool: ToolCoTre, gioiHanBuoc: number): KetQuaGoiTool {
  for (let buoc = 0; buoc < gioiHanBuoc; buoc++) {
    const ketQua = tool.thuMotBuoc();
    if (ketQua !== null) return ketQua;
  }
  return { thanhCong: false, loi: "loi_timeout" };
}

function chayHarnessTimeoutTrenNTacVu(soTacVu: number, gioiHanBuoc: number): KetQuaGoiTool[] {
  const ketQua: KetQuaGoiTool[] = [];
  for (let i = 0; i < soTacVu; i++) {
    const tool = taoToolTre();
    ketQua.push(goiToolCoTimeout(tool, gioiHanBuoc));
  }
  return ketQua;
}

const ketQua5 = chayHarnessTimeoutTrenNTacVu(5, 3);
console.log(JSON.stringify(ketQua5.map((k) => k.thanhCong)), tinhTyLeHoanThanh(ketQua5));
```

```typescript title=test
if (ketQua5.length !== 5) throw new Error("chayHarnessTimeoutTrenNTacVu(5, 3) phai tra ve mang dung 5 phan tu");
if (JSON.stringify(ketQua5.map((k) => k.thanhCong)) !== JSON.stringify([false, false, false, false, false])) {
  throw new Error("tool luon TREO, moi tac vu phai THAT BAI (timeout) -- ca 5 phai la false");
}
if (tinhTyLeHoanThanh(ketQua5) !== 0) throw new Error("completion rate tren 5 tac vu voi tool treo phai la 0");

const ketQua3 = chayHarnessTimeoutTrenNTacVu(3, 3);
if (ketQua3.length !== 3) throw new Error("doi so tac vu tu 5 sang 3 phai doi do dai mang tra ve -- tham so phai duoc dung that");

const toolRieng = taoToolTre();
const kq1 = goiToolCoTimeout(toolRieng, 4);
if (kq1.thanhCong !== false) throw new Error("tool treo phai LUON that bai du gioi han buoc la bao nhieu");
if (kq1.thanhCong === false && kq1.loi !== "loi_timeout") throw new Error("loi tra ve phai la 'loi_timeout', khong phai loi khac");
if (toolRieng.trangThai.soLanDaThu !== 4) throw new Error("goiToolCoTimeout phai thu DUNG gioiHanBuoc lan (4), khong hon khong kem, truoc khi bo cuoc");

const toolBinhThuongRieng = taoToolBinhThuong();
const kqOk = goiToolCoTimeout(toolBinhThuongRieng, 5);
if (kqOk.thanhCong !== true) throw new Error("tool binh thuong (khong treo) phai THANH CONG");
if (toolBinhThuongRieng.trangThai.soLanDaThu !== 1) throw new Error("tool binh thuong phai tra loi ngay lan thu 1, KHONG dung het ngan sach 5 buoc");

const toolNganSach0 = taoToolTre();
const kqNganSach0 = goiToolCoTimeout(toolNganSach0, 0);
if (kqNganSach0.thanhCong !== false) throw new Error("ngan sach 0 buoc phai la timeout ngay lap tuc");
if (toolNganSach0.trangThai.soLanDaThu !== 0) throw new Error("ngan sach 0 nghia la KHONG mot lan thuMotBuoc nao duoc goi -- khac voi retry-budget bai 4, o day KHONG co lan goi nao ngoai vong lap");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (goiToolCoTimeout): mot vong for chay tu buoc = 0 den truoc gioiHanBuoc, moi vong goi tool.thuMotBuoc() luu vao mot bien; neu bien do KHAC null thi return no NGAY; sau khi vong for ket thuc (khong con budget), return { thanhCong: false, loi: \"loi_timeout\" }. Cho hai (chayHarnessTimeoutTrenNTacVu): mot vong for chay dung soTacVu lan, moi lan TAO MOT taoToolTre() MOI, goi goiToolCoTimeout(tool, gioiHanBuoc), day ket qua vao mot mang, cuoi cung return mang do."
- kind: strategy
  body: "Cho dau: for (let buoc = 0; buoc < gioiHanBuoc; buoc++) { const ketQua = tool.thuMotBuoc(); if (ketQua !== null) return ketQua; } return { thanhCong: false, loi: \"loi_timeout\" }; Cho hai: const ketQua: KetQuaGoiTool[] = []; for (let i = 0; i < soTacVu; i++) { const tool = taoToolTre(); ketQua.push(goiToolCoTimeout(tool, gioiHanBuoc)); } return ketQua;"
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
  expect: "[false,false,false,false,false] 0"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đúng `3` lần hỏi rồi bỏ cuộc — không phải chờ vô hạn, không phải một
con số bí ẩn, mà đúng `gioiHanBuoc`. Nhưng bài này CHƯA có cách nào
cứu completion rate khi tool treo — nó chỉ CHẶN được vòng lặp vô hạn,
y hệt cách retry-budget (bài `4`) chỉ chặn vòng lặp chứ không cứu tool
hỏng vĩnh viễn. Bài sau thêm một lớp phòng thủ KHÁC: không chỉ nhớ MỘT
lần gọi thất bại, mà nhớ CẢ MỘT CHUỖI thất bại liên tiếp — VÀ tự động
ngừng gọi hoàn toàn khi chuỗi đó đủ dài.
::::

::::reflect{#nghi-lai}
Timeout giải quyết một câu hỏi mà retry (bài `2`, `4`) VÀ fallback
(bài `3`) chưa từng phải trả lời: NẾU tool không bao giờ nói gì cả —
không thành công, không thất bại, chỉ im lặng — thì cái gì quyết định
"đã đủ lâu"? Trong một hệ thống thật, đó LÀ mili-giây; Ở đây, VÌ không
có đồng hồ thật đáng tin, nó LÀ một NGÂN SÁCH BƯỚC tường minh — đúng
tinh thần "ngân sách" đã gặp Ở `goiToolCoRetryCoGioiHan` (bài `4`),
nhưng áp dụng cho một câu hỏi khác hẳn: không phải "thử lại bao nhiêu
lần" mà LÀ "chờ bao lâu trước khi từ bỏ MỘT lần gọi". Chi tiết ngân
sách `0` (không một lần gọi nào, khác quy ước bài `4`) không phải LÀ
một điểm khó chịu — nó LÀ bằng chứng rằng hai cơ chế NÀY, dù cùng dùng
từ "ngân sách", được LẮP RÁP theo hai cách khác nhau, VÀ đọc kỹ code
LÀ cách duy nhất biết chắc quy ước nào đang áp dụng.
::::

::::checkpoint{mastery=0.85}
::::
