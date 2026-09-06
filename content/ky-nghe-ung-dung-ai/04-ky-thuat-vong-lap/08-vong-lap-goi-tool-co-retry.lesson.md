---
id: ky-nghe-ung-dung-ai.ky-thuat-vong-lap.vong-lap-goi-tool-co-retry
title: "Loop gọi tool có retry — hai lớp ngân sách TÁCH BIỆT"
summary: "chayVongLapGoiToolCoRetry(soBuocCanDeXong, soBuocToiDaNgoai, soLanThuLaiToiDaTrongMoiBuoc) mỗi BƯỚC NGOÀI (của vòng lặp T9.4) THẬT SỰ tạo một ToolMoPhong MỚI kiểu taoToolLoiTamThoi (T9.3 bài 1: lần gọi đầu LUÔN thất bại, từ lần 2 LUÔN thành công) rồi gọi QUA goiToolCoRetryCoGioiHan (T9.3 bài 4) TRƯỚC KHI đếm bước ngoài LÀ tiến bộ. Với soBuocCanDeXong=3, soBuocToiDaNgoai=10: ngân sách retry-trong-bước=0 cho hetBuoc sau ĐÚNG 10 bước ngoài, ĐÚNG 10 lượt gọi tool THẬT (mỗi bước ngoài chỉ tốn 1 lượt, không bao giờ thành công); ngân sách=1 cho thanhCong sau ĐÚNG 3 bước ngoài NHƯNG 6 lượt gọi tool THẬT (mỗi bước ngoài âm thầm tốn 2 lượt: 1 gọi + 1 retry); ngân sách=2 cho KẾT QUẢ GIỐNG HỆT ngân sách=1 (3 bước ngoài, 6 lượt gọi) -- lỗi tạm thời chỉ cần đúng 1 lần thử lại, ngân sách retry-trong-bước dư ra không hề được dùng tới. Chứng minh bằng số: hai ngân sách (số bước NGOÀI của vòng lặp VÀ số lần thử lại TRONG một bước) độc lập hoàn toàn -- một bước 'thất bại' hay 'thành công' của vòng lặp ngoài có thể đã âm thầm tốn NHIỀU HƠN 1 lệnh gọi tool bên trong."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-vong-lap
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [kna.vong-lap-goi-tool-co-retry]
requires: [kna.do-chi-phi-moi-buoc]
concepts: [kna.vong-lap-goi-tool-co-retry]
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
Mọi tác vụ Ở q9.4a VÀ bài trước tự báo trạng thái trực tiếp — "chua_xong"
đến từ MỘT hàm tất định, không hề chạm tới tool nào. Nhưng T9.3 dạy một
sự thật khác: một agent THẬT không tự biết "xong" hay "chưa" — nó biết
qua GỌI TOOL, và tool CÓ THỂ lỗi tạm thời. Bài này ghép hai kỹ năng đã
học Ở hai track lại: mỗi BƯỚC của vòng lặp T9.4 giờ THẬT SỰ gọi một tool
T9.3, qua một lớp retry-trong-bước — VÀ lộ ra một ngân sách THỨ HAI.
::::

::::explain{#mot_buoc_ngoai_am_tham_ton_hai_luot_goi}
`goiToolCoRetryCoGioiHan` (T9.3 bài `4`) gọi tool một lần, rồi thử lại
LIÊN TỤC cho tới khi thành công HOẶC đã thử lại đủ `soLanThuLaiToiDa`
lần. Ghép hàm đó VÀO MỘT bước của vòng lặp T9.4: mỗi lần vòng lặp NGOÀI
lặp thêm một bước, nó tạo một `ToolMoPhong` MỚI (kiểu `taoToolLoiTamThoi`
— lần gọi ĐẦU luôn thất bại, từ lần THỨ HAI luôn thành công) rồi gọi
QUA lớp retry đó — TRƯỚC KHI biết bước NGOÀI này có LÀ tiến bộ hay không:

```typescript title=readonly
type KetQuaGoiTool =
  | { thanhCong: true; giaTri: string }
  | { thanhCong: false; loi: string };

type TrangThaiTool = { soLanDaGoi: number };

interface ToolMoPhong {
  trangThai: TrangThaiTool;
  goi(): KetQuaGoiTool;
}

function taoToolLoiTamThoi(): ToolMoPhong {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return {
    trangThai,
    goi(): KetQuaGoiTool {
      trangThai.soLanDaGoi++;
      if (trangThai.soLanDaGoi === 1) {
        return { thanhCong: false, loi: "loi_tam_thoi_rate_limit" };
      }
      return { thanhCong: true, giaTri: "ket_qua_that" };
    },
  };
}

function goiToolCoRetryCoGioiHan(tool: ToolMoPhong, soLanThuLaiToiDa: number): KetQuaGoiTool {
  let ketQua = tool.goi();
  let soLanDaThuLai = 0;
  while (!ketQua.thanhCong && soLanDaThuLai < soLanThuLaiToiDa) {
    ketQua = tool.goi();
    soLanDaThuLai++;
  }
  return ketQua;
}

// MOT buoc NGOAI cua vong lap T9.4: tao mot tool MOI, goi QUA retry-trong-buoc.
const toolBuocNay = taoToolLoiTamThoi();
const ketQuaBuocNay = goiToolCoRetryCoGioiHan(toolBuocNay, 1);
console.log(JSON.stringify(ketQuaBuocNay), "luot goi that:", toolBuocNay.trangThai.soLanDaGoi);
```

```text title=readonly
{"thanhCong":true,"giaTri":"ket_qua_that"} luot goi that: 2
```

MỘT bước NGOÀI (một lần "lặp thêm" của vòng lặp T9.4) đã âm thầm tốn
`2` lượt gọi tool THẬT — `1` gọi ban đầu (luôn thất bại) CỘNG `1` lần
retry (luôn thành công) — dù bên NGOÀI, vòng lặp T9.4 chỉ thấy ĐÚNG một
kết quả: "bước này thành công". Ngân sách retry-trong-bước (T9.3) VÀ
ngân sách bước-ngoài (T9.4) LÀ hai con số HOÀN TOÀN khác nhau.
::::

::::example{#ba_ngan_sach_retry_tren_cung_mot_tac_vu}
`chayVongLapGoiToolCoRetry` lặp tối đa `soBuocToiDaNgoai` bước NGOÀI;
mỗi bước tạo MỘT `taoToolLoiTamThoi()` mới, gọi qua
`goiToolCoRetryCoGioiHan(tool, soLanThuLaiToiDaTrongMoiBuoc)`; NẾU
thành công, bước đó tính LÀ tiến bộ THẬT — đủ `soBuocCanDeXong` lần tiến
bộ thì tác vụ `"thanhCong"`. So sánh BA ngân sách retry-trong-bước
(`0`, `1`, `2`) trên CÙNG `soBuocCanDeXong = 3`, `soBuocToiDaNgoai = 10`:

```typescript title=readonly
type KetQuaGoiTool =
  | { thanhCong: true; giaTri: string }
  | { thanhCong: false; loi: string };

type TrangThaiTool = { soLanDaGoi: number };

interface ToolMoPhong {
  trangThai: TrangThaiTool;
  goi(): KetQuaGoiTool;
}

function taoToolLoiTamThoi(): ToolMoPhong {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return {
    trangThai,
    goi(): KetQuaGoiTool {
      trangThai.soLanDaGoi++;
      if (trangThai.soLanDaGoi === 1) {
        return { thanhCong: false, loi: "loi_tam_thoi_rate_limit" };
      }
      return { thanhCong: true, giaTri: "ket_qua_that" };
    },
  };
}

function goiToolCoRetryCoGioiHan(tool: ToolMoPhong, soLanThuLaiToiDa: number): KetQuaGoiTool {
  let ketQua = tool.goi();
  let soLanDaThuLai = 0;
  while (!ketQua.thanhCong && soLanDaThuLai < soLanThuLaiToiDa) {
    ketQua = tool.goi();
    soLanDaThuLai++;
  }
  return ketQua;
}

type KetQuaVongLapNgoai =
  | { trangThai: "thanhCong"; giaTri: string; soBuocNgoaiDaDung: number; tongLuotGoiToolThat: number }
  | { trangThai: "hetBuoc"; soBuocNgoaiDaDung: number; tongLuotGoiToolThat: number };

function chayVongLapGoiToolCoRetry(
  soBuocCanDeXong: number,
  soBuocToiDaNgoai: number,
  soLanThuLaiToiDaTrongMoiBuoc: number,
): KetQuaVongLapNgoai {
  let soBuocNgoai = 0;
  let soBuocTienBoThat = 0;
  let tongLuotGoiToolThat = 0;
  while (soBuocNgoai < soBuocToiDaNgoai) {
    soBuocNgoai++;
    const tool = taoToolLoiTamThoi();
    const kq = goiToolCoRetryCoGioiHan(tool, soLanThuLaiToiDaTrongMoiBuoc);
    tongLuotGoiToolThat += tool.trangThai.soLanDaGoi;
    if (kq.thanhCong) {
      soBuocTienBoThat++;
      if (soBuocTienBoThat >= soBuocCanDeXong) {
        return { trangThai: "thanhCong", giaTri: "hoan_thanh", soBuocNgoaiDaDung: soBuocNgoai, tongLuotGoiToolThat };
      }
    }
  }
  return { trangThai: "hetBuoc", soBuocNgoaiDaDung: soBuocNgoai, tongLuotGoiToolThat };
}

for (const budget of [0, 1, 2]) {
  const kq = chayVongLapGoiToolCoRetry(3, 10, budget);
  console.log(`budget=${budget}:`, JSON.stringify(kq));
}
```

```text title=readonly
budget=0: {"trangThai":"hetBuoc","soBuocNgoaiDaDung":10,"tongLuotGoiToolThat":10}
budget=1: {"trangThai":"thanhCong","giaTri":"hoan_thanh","soBuocNgoaiDaDung":3,"tongLuotGoiToolThat":6}
budget=2: {"trangThai":"thanhCong","giaTri":"hoan_thanh","soBuocNgoaiDaDung":3,"tongLuotGoiToolThat":6}
```

Ngân sách `0` (không cho thử lại TRONG bước): MỌI bước ngoài chỉ tốn
đúng `1` lượt gọi — NHƯNG lượt gọi ĐÓ luôn LÀ "lần đầu" của một tool
MỚI, LUÔN thất bại — nên KHÔNG bước ngoài nào từng tính LÀ tiến bộ, vòng
lặp chạy hết `10` bước rồi `"hetBuoc"`, tốn ĐÚNG `10` lượt gọi tool
THẬT. Ngân sách `1`: MỖI bước ngoài THÀNH CÔNG (retry cứu được), nhưng
âm thầm tốn `2` lượt gọi — `3` bước ngoài × `2` lượt = `6`. Ngân sách
`2`: kết quả GIỐNG HỆT ngân sách `1` — lỗi tạm thời chỉ cần ĐÚNG `1` lần
thử lại LÀ đủ, ngân sách dư ra KHÔNG hề được dùng tới.
::::

::::predict{#doan-tang-ngan-sach-retry-trong-buoc commitOnce}
Nếu tăng `soLanThuLaiToiDaTrongMoiBuoc` từ `1` lên `5` (giữ NGUYÊN
`soBuocCanDeXong = 3`, `soBuocToiDaNgoai = 10`) — `soBuocNgoaiDaDung` VÀ
`tongLuotGoiToolThat` đổi ra sao so VỚI ngân sách `1`?

:::opt{correct}
KHÔNG đổi gì cả — VẪN `soBuocNgoaiDaDung = 3` VÀ `tongLuotGoiToolThat =
6`, giống HỆT ngân sách `1`: mỗi tool `taoToolLoiTamThoi()` MỚI luôn
thành công NGAY Ở lần retry ĐẦU TIÊN (lần gọi thứ hai), nên phần ngân
sách retry-trong-bước VƯỢT quá `1` không bao giờ được chạm tới, bất kể
con số đó LÀ `2`, `5`, hay lớn hơn nữa
:::
:::opt
`tongLuotGoiToolThat` sẽ giảm xuống, vì ngân sách retry cao hơn nghĩa
LÀ mỗi bước "tự tin" hơn, tốn ÍT lượt gọi hơn để thành công
::why
Nhầm rằng ngân sách retry CAO HƠN nghĩa LÀ ÍT lượt gọi hơn — nhưng
`goiToolCoRetryCoGioiHan` CHỈ dừng THỬ LẠI khi thành công HOẶC hết ngân
sách; nó không hề "biết trước" cần bao nhiêu lượt, VÀ với tool NÀY, số
lượt CẦN THIẾT ĐỂ THÀNH CÔNG luôn LÀ `2`, không phụ thuộc ngân sách tối
đa được PHÉP dùng.

Chỗ lệch: `taoToolLoiTamThoi` LUÔN thành công Ở ĐÚNG lần gọi THỨ HAI —
ngân sách retry-trong-bước chỉ LÀ một TRẦN trên, không phải một mục
tiêu cần đạt tới; tăng trần không hề thay đổi số lượt THẬT SỰ cần.
::
:::
:::opt
`soBuocNgoaiDaDung` sẽ giảm xuống dưới `3`, vì mỗi bước ngoài giờ "chắc
ăn" hơn nên vòng lặp NGOÀI hội tụ nhanh hơn
::why
Nhầm rằng ngân sách retry-trong-bước ảnh hưởng tới TỐC ĐỘ hội tụ CỦA
vòng lặp NGOÀI — nhưng vòng lặp ngoài CHỈ đếm `soBuocTienBoThat`, VÀ nó
tăng ĐÚNG `1` mỗi khi MỘT bước ngoài thành công, bất kể bước đó tốn `1`
hay `10` lượt gọi bên trong.

Chỗ lệch: `soBuocCanDeXong = 3` LÀ số bước NGOÀI cần để tác vụ
`"thanhCong"` — với BẤT KỲ ngân sách retry-trong-bước nào `>= 1`, mỗi
bước ngoài ĐỀU thành công (retry luôn cứu được lỗi tạm thời NÀY), nên
`soBuocNgoaiDaDung` LUÔN LÀ đúng `3`, không hơn không kém.
::
:::
::::

::::code{#viet_chay_vong_lap_goi_tool_co_retry}
Hoàn thiện `chayVongLapGoiToolCoRetry` — lặp `while` tối đa
`soBuocToiDaNgoai` bước NGOÀI; MỖI bước tạo `taoToolLoiTamThoi()` MỚI,
gọi qua `goiToolCoRetryCoGioiHan(tool, soLanThuLaiToiDaTrongMoiBuoc)`,
CỘNG `tool.trangThai.soLanDaGoi` (số lượt gọi THẬT của tool NÀY) VÀO
`tongLuotGoiToolThat`; NẾU kết quả thành công, tăng `soBuocTienBoThat`;
NẾU `soBuocTienBoThat` đạt `soBuocCanDeXong`, trả về `"thanhCong"` NGAY;
hết vòng lặp thì trả `"hetBuoc"`. Hoàn thiện `soSanhNhieuNganSachRetry`
— với MỖI ngân sách trong `dsNganSachRetry`, gọi
`chayVongLapGoiToolCoRetry(soBuocCanDeXong, soBuocToiDaNgoai, nganSach)`,
đẩy kết quả vào mảng trả về.

```typescript title=starter
type KetQuaGoiTool =
  | { thanhCong: true; giaTri: string }
  | { thanhCong: false; loi: string };

type TrangThaiTool = { soLanDaGoi: number };

interface ToolMoPhong {
  trangThai: TrangThaiTool;
  goi(): KetQuaGoiTool;
}

function taoToolLoiTamThoi(): ToolMoPhong {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return {
    trangThai,
    goi(): KetQuaGoiTool {
      trangThai.soLanDaGoi++;
      if (trangThai.soLanDaGoi === 1) {
        return { thanhCong: false, loi: "loi_tam_thoi_rate_limit" };
      }
      return { thanhCong: true, giaTri: "ket_qua_that" };
    },
  };
}

function goiToolCoRetryCoGioiHan(tool: ToolMoPhong, soLanThuLaiToiDa: number): KetQuaGoiTool {
  let ketQua = tool.goi();
  let soLanDaThuLai = 0;
  while (!ketQua.thanhCong && soLanDaThuLai < soLanThuLaiToiDa) {
    ketQua = tool.goi();
    soLanDaThuLai++;
  }
  return ketQua;
}

type KetQuaVongLapNgoai =
  | { trangThai: "thanhCong"; giaTri: string; soBuocNgoaiDaDung: number; tongLuotGoiToolThat: number }
  | { trangThai: "hetBuoc"; soBuocNgoaiDaDung: number; tongLuotGoiToolThat: number };

function chayVongLapGoiToolCoRetry(
  soBuocCanDeXong: number,
  soBuocToiDaNgoai: number,
  soLanThuLaiToiDaTrongMoiBuoc: number,
): KetQuaVongLapNgoai {
  ___
}

function soSanhNhieuNganSachRetry(
  soBuocCanDeXong: number,
  soBuocToiDaNgoai: number,
  dsNganSachRetry: number[],
): KetQuaVongLapNgoai[] {
  ___
}

const ketQuaBaCauHinh = soSanhNhieuNganSachRetry(3, 10, [0, 1, 2]);
console.log(JSON.stringify(ketQuaBaCauHinh));
```

```typescript title=solution
type KetQuaGoiTool =
  | { thanhCong: true; giaTri: string }
  | { thanhCong: false; loi: string };

type TrangThaiTool = { soLanDaGoi: number };

interface ToolMoPhong {
  trangThai: TrangThaiTool;
  goi(): KetQuaGoiTool;
}

function taoToolLoiTamThoi(): ToolMoPhong {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return {
    trangThai,
    goi(): KetQuaGoiTool {
      trangThai.soLanDaGoi++;
      if (trangThai.soLanDaGoi === 1) {
        return { thanhCong: false, loi: "loi_tam_thoi_rate_limit" };
      }
      return { thanhCong: true, giaTri: "ket_qua_that" };
    },
  };
}

function goiToolCoRetryCoGioiHan(tool: ToolMoPhong, soLanThuLaiToiDa: number): KetQuaGoiTool {
  let ketQua = tool.goi();
  let soLanDaThuLai = 0;
  while (!ketQua.thanhCong && soLanDaThuLai < soLanThuLaiToiDa) {
    ketQua = tool.goi();
    soLanDaThuLai++;
  }
  return ketQua;
}

type KetQuaVongLapNgoai =
  | { trangThai: "thanhCong"; giaTri: string; soBuocNgoaiDaDung: number; tongLuotGoiToolThat: number }
  | { trangThai: "hetBuoc"; soBuocNgoaiDaDung: number; tongLuotGoiToolThat: number };

function chayVongLapGoiToolCoRetry(
  soBuocCanDeXong: number,
  soBuocToiDaNgoai: number,
  soLanThuLaiToiDaTrongMoiBuoc: number,
): KetQuaVongLapNgoai {
  let soBuocNgoai = 0;
  let soBuocTienBoThat = 0;
  let tongLuotGoiToolThat = 0;
  while (soBuocNgoai < soBuocToiDaNgoai) {
    soBuocNgoai++;
    const tool = taoToolLoiTamThoi();
    const kq = goiToolCoRetryCoGioiHan(tool, soLanThuLaiToiDaTrongMoiBuoc);
    tongLuotGoiToolThat += tool.trangThai.soLanDaGoi;
    if (kq.thanhCong) {
      soBuocTienBoThat++;
      if (soBuocTienBoThat >= soBuocCanDeXong) {
        return { trangThai: "thanhCong", giaTri: "hoan_thanh", soBuocNgoaiDaDung: soBuocNgoai, tongLuotGoiToolThat };
      }
    }
  }
  return { trangThai: "hetBuoc", soBuocNgoaiDaDung: soBuocNgoai, tongLuotGoiToolThat };
}

function soSanhNhieuNganSachRetry(
  soBuocCanDeXong: number,
  soBuocToiDaNgoai: number,
  dsNganSachRetry: number[],
): KetQuaVongLapNgoai[] {
  const ketQua: KetQuaVongLapNgoai[] = [];
  for (const nganSach of dsNganSachRetry) {
    ketQua.push(chayVongLapGoiToolCoRetry(soBuocCanDeXong, soBuocToiDaNgoai, nganSach));
  }
  return ketQua;
}

const ketQuaBaCauHinh = soSanhNhieuNganSachRetry(3, 10, [0, 1, 2]);
console.log(JSON.stringify(ketQuaBaCauHinh));
```

```typescript title=test
if (ketQuaBaCauHinh.length !== 3) throw new Error("soSanhNhieuNganSachRetry phai tra ve mang dung 3 phan tu");

const [budget0, budget1, budget2] = ketQuaBaCauHinh;
if (budget0!.trangThai !== "hetBuoc") throw new Error("ngan sach retry 0: khong bao gio thanh cong, phai la hetBuoc");
if (budget0!.soBuocNgoaiDaDung !== 10) throw new Error("ngan sach retry 0: phai dung DUNG 10 buoc ngoai (het soBuocToiDaNgoai)");
if (budget0!.tongLuotGoiToolThat !== 10) throw new Error("ngan sach retry 0: moi buoc ngoai chi ton DUNG 1 luot goi tool that, tong phai la 10");

if (budget1!.trangThai !== "thanhCong") throw new Error("ngan sach retry 1: phai THANH CONG (retry cuu duoc loi tam thoi)");
if (budget1!.soBuocNgoaiDaDung !== 3) throw new Error("ngan sach retry 1: phai dung DUNG 3 buoc ngoai (khop soBuocCanDeXong)");
if (budget1!.tongLuotGoiToolThat !== 6) throw new Error("ngan sach retry 1: MOI buoc ngoai am tham ton 2 luot goi that (1 goi + 1 retry) -- tong phai la 6, KHONG phai 3");

if (budget2!.trangThai !== "thanhCong") throw new Error("ngan sach retry 2: phai THANH CONG");
if (budget2!.soBuocNgoaiDaDung !== 3) throw new Error("ngan sach retry 2: phai GIONG HET ngan sach 1, dung 3 buoc ngoai");
if (budget2!.tongLuotGoiToolThat !== 6) throw new Error("ngan sach retry 2: phai GIONG HET ngan sach 1 (6 luot goi that) -- 1 lan retry du de cuu loi tam thoi, du du them cung khong dung toi");

const kqRieng = chayVongLapGoiToolCoRetry(1, 5, 1);
if (kqRieng.trangThai !== "thanhCong") throw new Error("can 1 buoc tien bo, ngan sach retry 1, phai THANH CONG");
if (kqRieng.soBuocNgoaiDaDung !== 1) throw new Error("can 1 buoc tien bo phai dung DUNG 1 buoc ngoai");
if (kqRieng.tongLuotGoiToolThat !== 2) throw new Error("1 buoc ngoai voi retry cuu duoc phai ton DUNG 2 luot goi tool that");

const ketQuaMotCauHinh = soSanhNhieuNganSachRetry(3, 10, [1]);
if (ketQuaMotCauHinh.length !== 1) throw new Error("doi so danh sach ngan sach phai doi do dai mang tra ve -- tham so phai duoc dung that");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (chayVongLapGoiToolCoRetry): mot vong while chay toi da soBuocToiDaNgoai lan; moi lan tang soBuocNgoai, tao MOT taoToolLoiTamThoi() MOI, goi goiToolCoRetryCoGioiHan(tool, soLanThuLaiToiDaTrongMoiBuoc), cong tool.trangThai.soLanDaGoi vao tongLuotGoiToolThat; neu ket qua thanh cong thi tang soBuocTienBoThat, va neu soBuocTienBoThat dat soBuocCanDeXong thi return thanhCong NGAY; het vong lap tra hetBuoc. Cho hai (soSanhNhieuNganSachRetry): mot vong for-of qua dsNganSachRetry, goi chayVongLapGoiToolCoRetry voi tung ngan sach, day ket qua vao mot mang."
- kind: strategy
  body: "Cho dau: let soBuocNgoai = 0; let soBuocTienBoThat = 0; let tongLuotGoiToolThat = 0; while (soBuocNgoai < soBuocToiDaNgoai) { soBuocNgoai++; const tool = taoToolLoiTamThoi(); const kq = goiToolCoRetryCoGioiHan(tool, soLanThuLaiToiDaTrongMoiBuoc); tongLuotGoiToolThat += tool.trangThai.soLanDaGoi; if (kq.thanhCong) { soBuocTienBoThat++; if (soBuocTienBoThat >= soBuocCanDeXong) { return { trangThai: \"thanhCong\", giaTri: \"hoan_thanh\", soBuocNgoaiDaDung: soBuocNgoai, tongLuotGoiToolThat }; } } } return { trangThai: \"hetBuoc\", soBuocNgoaiDaDung: soBuocNgoai, tongLuotGoiToolThat }; Cho hai: const ketQua: KetQuaVongLapNgoai[] = []; for (const nganSach of dsNganSachRetry) { ketQua.push(chayVongLapGoiToolCoRetry(soBuocCanDeXong, soBuocToiDaNgoai, nganSach)); } return ketQua;"
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
  expect: "{\"trangThai\":\"hetBuoc\",\"soBuocNgoaiDaDung\":10,\"tongLuotGoiToolThat\":10}"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ngân sách retry `0` VÀ `1` cho ra HAI KẾT CỤC hoàn toàn khác — `hetBuoc`
VỚI `10` lượt gọi, so VỚI `thanhCong` VỚI `6` lượt gọi. Nhưng CHÚ Ý:
`6` lượt gọi, KHÔNG phải `3` — MỘT bước ngoài "thành công" đã âm thầm
tốn `2` lệnh gọi tool. Bài sau dùng ĐÚNG khoảng cách đó (giữa "một bước
NGOÀI" VÀ "một lệnh gọi tool BÊN TRONG") LÀM cầu nối sang lập trình
hàm: tool call LÀ một Kleisli arrow, retry/fallback LÀ Alternative.
::::

::::reflect{#nghi-lai}
Bài này không dạy một cơ chế MỚI — nó ghép HAI cơ chế đã học Ở HAI track
khác nhau (`goiToolCoRetryCoGioiHan` từ T9.3, hình dạng vòng lặp từ
T9.4) VÀ chỉ ra rằng chúng vận hành Ở HAI TẦNG hoàn toàn tách biệt. Một
kỹ sư đọc log "vòng lặp dùng `3` bước" có thể lầm tưởng chi phí THẬT LÀ
`3` đơn vị — nhưng nếu MỖI bước Ở BÊN TRONG có thể âm thầm retry, chi
phí THẬT có thể LÀ gấp đôi, gấp ba, hay hơn nữa. Ngân sách của vòng lặp
NGOÀI (số bước) VÀ ngân sách của MỖI bước (số lần thử lại) LÀ hai tham
số ĐỘC LẬP, VÀ một hệ thống đo lường đúng đắn phải theo dõi CẢ HAI,
không chỉ một.
::::

::::checkpoint{mastery=0.85}
::::
