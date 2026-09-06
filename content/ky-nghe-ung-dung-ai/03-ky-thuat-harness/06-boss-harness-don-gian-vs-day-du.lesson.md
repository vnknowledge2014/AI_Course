---
id: ky-nghe-ung-dung-ai.ky-thuat-harness.boss-harness-don-gian-vs-day-du
title: "BOSS q9.3a — so sánh A/B harnessDonGian vs harnessDayDu, đóng q9.3a tại 6/6"
summary: "harnessDonGian ráp NGUYÊN VĂN bài 1 (goiToolKhongRetry, không retry/fallback). harnessDayDu ráp NGUYÊN VĂN goiToolCoRetryCoGioiHan(tool,1) (bài 4, ngân sách 1 lần == retry bài 2) + goiToolCoFallback (bài 3) + KetQua/andThen kiểu ROP (bài 5, qua dinhDangKetQuaCuoi). CẢ HAI chạy trên CÙNG taoToolLoiTamThoi (bài 1, lỗi tạm thời) và CÙNG 5 tác vụ: harnessDonGian completion rate = 0/5 = 0 (gọi 1 lần, luôn trúng lần đầu luôn fail); harnessDayDu completion rate = 5/5 = 1 (retry cứu được, fallback không cần dùng tới, giá trị vẫn là 'OK:ket_qua_that' -- KHÔNG phải giá trị fallback). Đối chứng riêng: goiToolCoFallback vẫn hoạt động đúng khi tool chính hỏng VĨNH VIỄN (taoToolHongVinhVien) -- trả về 'gia_tri_mac_dinh' qua fallback. Đóng q9.3a tại 6/6."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-harness
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [kna.boss-harness-don-gian-vs-day-du]
requires: [kna.loi-trong-agent-la-rop]
concepts: [kna.boss-harness-don-gian-vs-day-du]
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
Năm bài: tool mô phỏng có trạng thái, retry một lần, fallback sang tool
dự phòng, ràng buộc số lần thử lại, VÀ cách gọi tên chính thức cho
"thành công hoặc thất bại" — ROP. BOSS ráp TẤT CẢ thành HAI harness
hoàn chỉnh — một ĐƠN GIẢN, một ĐẦY ĐỦ — VÀ đo completion rate của cả
hai trên ĐÚNG CÙNG một model/tool, để chứng minh đúng luận điểm mở đầu
CẢ track T9.3: harness LÀ nơi quyết định một hệ thống sống hay chết,
không phải model.
::::

::::explain{#rap_hai_harness}
`harnessDonGian` LÀ đúng harness bài `1`: gọi tool đúng MỘT LẦN, không
retry, không fallback. `harnessDayDu` ráp NGUYÊN VĂN ba cơ chế:

> **Retry có ngân sách** (bài `4`) — `goiToolCoRetryCoGioiHan(tool, 1)`
> dùng ngân sách `1`, tương đương ĐÚNG "gọi lại một lần" của bài `2`,
> chỉ viết bằng phiên bản TỔNG QUÁT hơn.
>
> **Fallback** (bài `3`) — `goiToolCoFallback` chuyển sang tool dự
> phòng khi retry VẪN thất bại.
>
> **Biểu diễn kiểu ROP** (bài `5`) — `dinhDangKetQuaCuoi` dùng
> `andThen` để định dạng lại giá trị thành công, "trượt" qua nguyên
> vẹn khi thất bại — `KetQuaGoiTool` giờ được viết LÀ một BÍ DANH của
> `KetQua<string, string>`, xác nhận chúng LUÔN LÀ cùng một hình dạng.

```typescript title=readonly
type KetQua<T, E> =
  | { thanhCong: true; giaTri: T }
  | { thanhCong: false; loi: E };

type KetQuaGoiTool = KetQua<string, string>;

type TrangThaiTool = { soLanDaGoi: number };

interface ToolMoPhong {
  trangThai: TrangThaiTool;
  goi(): KetQuaGoiTool;
}

function andThen<T, U, E>(kq: KetQua<T, E>, buoc: (giaTri: T) => KetQua<U, E>): KetQua<U, E> {
  return kq.thanhCong ? buoc(kq.giaTri) : kq;
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

function taoToolDuPhong(): ToolMoPhong {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return {
    trangThai,
    goi(): KetQuaGoiTool {
      trangThai.soLanDaGoi++;
      return { thanhCong: true, giaTri: "gia_tri_mac_dinh" };
    },
  };
}

function goiToolKhongRetry(tool: ToolMoPhong): KetQuaGoiTool {
  return tool.goi();
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

function goiToolCoFallback(toolChinh: ToolMoPhong, toolDuPhong: ToolMoPhong): KetQuaGoiTool {
  const ketQuaChinh = goiToolCoRetryCoGioiHan(toolChinh, 1);
  if (ketQuaChinh.thanhCong) return ketQuaChinh;
  return toolDuPhong.goi();
}

function dinhDangKetQuaCuoi(kq: KetQuaGoiTool): KetQuaGoiTool {
  return andThen(kq, (giaTri) => ({ thanhCong: true, giaTri: `OK:${giaTri}` }));
}

const tool = taoToolLoiTamThoi();
console.log(JSON.stringify(dinhDangKetQuaCuoi(goiToolKhongRetry(tool))));
```

```text title=readonly
{"thanhCong":false,"loi":"loi_tam_thoi_rate_limit"}
```

`goiToolKhongRetry` gọi tool đúng MỘT lần — lần đó LUÔN thất bại theo
đúng quy luật `taoToolLoiTamThoi` — VÀ `dinhDangKetQuaCuoi` (qua
`andThen`) "trượt" qua NGUYÊN VẸN lỗi đó, không hề định dạng gì cả.
::::

::::example{#so_sanh_ab_hai_harness}
So sánh hai harness trên CÙNG `taoToolLoiTamThoi`, CÙNG `5` tác vụ —
CHỈ khác Ở harness:

```typescript title=readonly
type KetQua<T, E> =
  | { thanhCong: true; giaTri: T }
  | { thanhCong: false; loi: E };

type KetQuaGoiTool = KetQua<string, string>;

type TrangThaiTool = { soLanDaGoi: number };

interface ToolMoPhong {
  trangThai: TrangThaiTool;
  goi(): KetQuaGoiTool;
}

function andThen<T, U, E>(kq: KetQua<T, E>, buoc: (giaTri: T) => KetQua<U, E>): KetQua<U, E> {
  return kq.thanhCong ? buoc(kq.giaTri) : kq;
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

function taoToolDuPhong(): ToolMoPhong {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return {
    trangThai,
    goi(): KetQuaGoiTool {
      trangThai.soLanDaGoi++;
      return { thanhCong: true, giaTri: "gia_tri_mac_dinh" };
    },
  };
}

function goiToolKhongRetry(tool: ToolMoPhong): KetQuaGoiTool {
  return tool.goi();
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

function goiToolCoFallback(toolChinh: ToolMoPhong, toolDuPhong: ToolMoPhong): KetQuaGoiTool {
  const ketQuaChinh = goiToolCoRetryCoGioiHan(toolChinh, 1);
  if (ketQuaChinh.thanhCong) return ketQuaChinh;
  return toolDuPhong.goi();
}

function dinhDangKetQuaCuoi(kq: KetQuaGoiTool): KetQuaGoiTool {
  return andThen(kq, (giaTri) => ({ thanhCong: true, giaTri: `OK:${giaTri}` }));
}

function tinhTyLeHoanThanh(ketQua: KetQuaGoiTool[]): number {
  if (ketQua.length === 0) return 0;
  return ketQua.filter((k) => k.thanhCong).length / ketQua.length;
}

function harnessDonGian(soTacVu: number): KetQuaGoiTool[] {
  const ketQua: KetQuaGoiTool[] = [];
  for (let i = 0; i < soTacVu; i++) {
    const tool = taoToolLoiTamThoi();
    ketQua.push(dinhDangKetQuaCuoi(goiToolKhongRetry(tool)));
  }
  return ketQua;
}

function harnessDayDu(soTacVu: number): KetQuaGoiTool[] {
  const ketQua: KetQuaGoiTool[] = [];
  for (let i = 0; i < soTacVu; i++) {
    const toolChinh = taoToolLoiTamThoi();
    const toolDuPhong = taoToolDuPhong();
    ketQua.push(dinhDangKetQuaCuoi(goiToolCoFallback(toolChinh, toolDuPhong)));
  }
  return ketQua;
}

const donGian = harnessDonGian(5);
const dayDu = harnessDayDu(5);
console.log("don gian:", JSON.stringify(donGian.map((k) => k.thanhCong)), tinhTyLeHoanThanh(donGian));
console.log("day du:  ", JSON.stringify(dayDu.map((k) => k.thanhCong)), tinhTyLeHoanThanh(dayDu));
console.log(JSON.stringify(dayDu.map((k) => (k.thanhCong ? k.giaTri : null))));
```

```text title=readonly
don gian: [false,false,false,false,false] 0
day du:   [true,true,true,true,true] 1
["OK:ket_qua_that","OK:ket_qua_that","OK:ket_qua_that","OK:ket_qua_that","OK:ket_qua_that"]
```

CÙNG `taoToolLoiTamThoi`, CÙNG `5` tác vụ — completion rate LÀ `0` cho
harness ĐƠN GIẢN, `1` cho harness ĐẦY ĐỦ. VÀ giá trị trả về của
`harnessDayDu` LÀ `"OK:ket_qua_that"` — giá trị THẬT, không phải giá
trị fallback — vì retry (ngân sách `1`) ĐÃ ĐỦ để tool chính tự phục
hồi, fallback không hề cần tới.
::::

::::predict{#doan-tool-hong-vinh-vien commitOnce}
NẾU đổi tool trong CẢ HAI harness sang một tool HỎNG VĨNH VIỄN (mọi
lần gọi đều thất bại, không chỉ lần đầu) — completion rate của
`harnessDonGian` VÀ của `harnessDayDu` thay đổi thế nào?

:::opt{correct}
`harnessDonGian` VẪN LÀ `0` (không đổi — nó không có cơ chế phục hồi
nào, tạm thời hay vĩnh viễn cũng thất bại như nhau). `harnessDayDu`
VẪN LÀ `1` (không đổi TỈ LỆ, nhờ fallback) — nhưng giá trị trả về đổi
từ `"OK:ket_qua_that"` sang `"OK:gia_tri_mac_dinh"`, vì giờ nó phải đi
qua nhánh fallback
:::
:::opt
`harnessDayDu` giảm xuống `0` — tool chính hỏng vĩnh viễn thì retry
không cứu được, nên harness cũng thất bại theo
::why
Quên rằng fallback (bài `3`) CHÍNH LÀ để xử lý đúng trường hợp này —
khi retry (dù có ngân sách) vẫn thất bại, `goiToolCoFallback` chuyển
sang `toolDuPhong`, và `taoToolDuPhong` LUÔN thành công.

Chỗ lệch: `goiToolCoFallback` không dừng lại Ở kết quả thất bại của
`goiToolCoRetryCoGioiHan` — nó CÓ một nhánh `return toolDuPhong.goi();`
ngay sau đó, VÀ tool dự phòng không bao giờ thất bại.
::
:::
:::opt
Cả hai đều giữ NGUYÊN hoàn toàn — kể cả completion rate LẪN giá trị
trả về, không đổi gì cả
::why
Đúng phần "completion rate của `harnessDayDu` không đổi" (`1`) —
nhưng SAI phần "giá trị trả về không đổi".

Chỗ lệch: `harnessDayDu` SẼ đổi giá trị trả về — từ
`"OK:ket_qua_that"` (tool chính tự phục hồi) sang
`"OK:gia_tri_mac_dinh"` (đi qua fallback) — vì con đường XỬ LÝ bên
trong đã khác, dù con SỐ completion rate cuối cùng tình cờ giống nhau.
::
:::
::::

::::code{#viet_hai_harness}
Hoàn thiện `harnessDonGian` — với MỖI trong `soTacVu` tác vụ: tạo một
`taoToolLoiTamThoi()` MỚI, gọi `goiToolKhongRetry` rồi `dinhDangKetQuaCuoi`
trên kết quả đó, đẩy vào mảng trả về. Hoàn thiện `harnessDayDu` — với
MỖI trong `soTacVu` tác vụ: tạo MỘT `taoToolLoiTamThoi()` (tool chính)
VÀ MỘT `taoToolDuPhong()` (tool dự phòng) MỚI, gọi `goiToolCoFallback`
rồi `dinhDangKetQuaCuoi` trên kết quả đó, đẩy vào mảng trả về.

```typescript title=starter
type KetQua<T, E> =
  | { thanhCong: true; giaTri: T }
  | { thanhCong: false; loi: E };

type KetQuaGoiTool = KetQua<string, string>;

type TrangThaiTool = { soLanDaGoi: number };

interface ToolMoPhong {
  trangThai: TrangThaiTool;
  goi(): KetQuaGoiTool;
}

function andThen<T, U, E>(kq: KetQua<T, E>, buoc: (giaTri: T) => KetQua<U, E>): KetQua<U, E> {
  return kq.thanhCong ? buoc(kq.giaTri) : kq;
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

function taoToolDuPhong(): ToolMoPhong {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return {
    trangThai,
    goi(): KetQuaGoiTool {
      trangThai.soLanDaGoi++;
      return { thanhCong: true, giaTri: "gia_tri_mac_dinh" };
    },
  };
}

function taoToolHongVinhVien(): ToolMoPhong {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return {
    trangThai,
    goi(): KetQuaGoiTool {
      trangThai.soLanDaGoi++;
      return { thanhCong: false, loi: "hong_vinh_vien" };
    },
  };
}

function goiToolKhongRetry(tool: ToolMoPhong): KetQuaGoiTool {
  return tool.goi();
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

function goiToolCoFallback(toolChinh: ToolMoPhong, toolDuPhong: ToolMoPhong): KetQuaGoiTool {
  const ketQuaChinh = goiToolCoRetryCoGioiHan(toolChinh, 1);
  if (ketQuaChinh.thanhCong) return ketQuaChinh;
  return toolDuPhong.goi();
}

function dinhDangKetQuaCuoi(kq: KetQuaGoiTool): KetQuaGoiTool {
  return andThen(kq, (giaTri) => ({ thanhCong: true, giaTri: `OK:${giaTri}` }));
}

function tinhTyLeHoanThanh(ketQua: KetQuaGoiTool[]): number {
  if (ketQua.length === 0) return 0;
  return ketQua.filter((k) => k.thanhCong).length / ketQua.length;
}

function harnessDonGian(soTacVu: number): KetQuaGoiTool[] {
  ___
}

function harnessDayDu(soTacVu: number): KetQuaGoiTool[] {
  ___
}

const ketQuaDonGian = harnessDonGian(5);
const ketQuaDayDu = harnessDayDu(5);
console.log(tinhTyLeHoanThanh(ketQuaDonGian), tinhTyLeHoanThanh(ketQuaDayDu));
```

```typescript title=solution
type KetQua<T, E> =
  | { thanhCong: true; giaTri: T }
  | { thanhCong: false; loi: E };

type KetQuaGoiTool = KetQua<string, string>;

type TrangThaiTool = { soLanDaGoi: number };

interface ToolMoPhong {
  trangThai: TrangThaiTool;
  goi(): KetQuaGoiTool;
}

function andThen<T, U, E>(kq: KetQua<T, E>, buoc: (giaTri: T) => KetQua<U, E>): KetQua<U, E> {
  return kq.thanhCong ? buoc(kq.giaTri) : kq;
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

function taoToolDuPhong(): ToolMoPhong {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return {
    trangThai,
    goi(): KetQuaGoiTool {
      trangThai.soLanDaGoi++;
      return { thanhCong: true, giaTri: "gia_tri_mac_dinh" };
    },
  };
}

function taoToolHongVinhVien(): ToolMoPhong {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return {
    trangThai,
    goi(): KetQuaGoiTool {
      trangThai.soLanDaGoi++;
      return { thanhCong: false, loi: "hong_vinh_vien" };
    },
  };
}

function goiToolKhongRetry(tool: ToolMoPhong): KetQuaGoiTool {
  return tool.goi();
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

function goiToolCoFallback(toolChinh: ToolMoPhong, toolDuPhong: ToolMoPhong): KetQuaGoiTool {
  const ketQuaChinh = goiToolCoRetryCoGioiHan(toolChinh, 1);
  if (ketQuaChinh.thanhCong) return ketQuaChinh;
  return toolDuPhong.goi();
}

function dinhDangKetQuaCuoi(kq: KetQuaGoiTool): KetQuaGoiTool {
  return andThen(kq, (giaTri) => ({ thanhCong: true, giaTri: `OK:${giaTri}` }));
}

function tinhTyLeHoanThanh(ketQua: KetQuaGoiTool[]): number {
  if (ketQua.length === 0) return 0;
  return ketQua.filter((k) => k.thanhCong).length / ketQua.length;
}

function harnessDonGian(soTacVu: number): KetQuaGoiTool[] {
  const ketQua: KetQuaGoiTool[] = [];
  for (let i = 0; i < soTacVu; i++) {
    const tool = taoToolLoiTamThoi();
    ketQua.push(dinhDangKetQuaCuoi(goiToolKhongRetry(tool)));
  }
  return ketQua;
}

function harnessDayDu(soTacVu: number): KetQuaGoiTool[] {
  const ketQua: KetQuaGoiTool[] = [];
  for (let i = 0; i < soTacVu; i++) {
    const toolChinh = taoToolLoiTamThoi();
    const toolDuPhong = taoToolDuPhong();
    ketQua.push(dinhDangKetQuaCuoi(goiToolCoFallback(toolChinh, toolDuPhong)));
  }
  return ketQua;
}

const ketQuaDonGian = harnessDonGian(5);
const ketQuaDayDu = harnessDayDu(5);
console.log(tinhTyLeHoanThanh(ketQuaDonGian), tinhTyLeHoanThanh(ketQuaDayDu));
```

```typescript title=test
if (ketQuaDonGian.length !== 5) throw new Error("harnessDonGian(5) phai tra ve mang dung 5 phan tu");
if (ketQuaDayDu.length !== 5) throw new Error("harnessDayDu(5) phai tra ve mang dung 5 phan tu");

if (tinhTyLeHoanThanh(ketQuaDonGian) !== 0) {
  throw new Error("harnessDonGian (khong retry/fallback) tren tool loi-tam-thoi phai co completion rate 0 -- moi tac vu dung tool MOI, luon that bai o lan goi dau");
}
if (tinhTyLeHoanThanh(ketQuaDayDu) !== 1) {
  throw new Error("harnessDayDu (co retry+fallback) tren CUNG mot tool phai co completion rate 1 -- lan goi thu hai luon thanh cong");
}

for (const k of ketQuaDayDu) {
  if (!k.thanhCong) throw new Error("moi ket qua cua harnessDayDu phai thanh cong");
  if (k.giaTri !== "OK:ket_qua_that") {
    throw new Error("harnessDayDu phai tra ve gia tri THAT (khong phai fallback) vi retry ngan sach 1 da du de sua loi tam thoi");
  }
}
for (const k of ketQuaDonGian) {
  if (k.thanhCong) throw new Error("moi ket qua cua harnessDonGian phai that bai -- khong co co che phuc hoi nao");
}

if (harnessDonGian(3).length !== 3) throw new Error("doi so_tac_vu phai doi do dai mang harnessDonGian tra ve -- tham so phai duoc dung that");
if (harnessDayDu(2).length !== 2) throw new Error("doi so_tac_vu phai doi do dai mang harnessDayDu tra ve -- tham so phai duoc dung that");

const toolChinhHong = taoToolHongVinhVien();
const toolDuPhongRieng = taoToolDuPhong();
const ketQuaFallback = goiToolCoFallback(toolChinhHong, toolDuPhongRieng);
if (!ketQuaFallback.thanhCong) throw new Error("goiToolCoFallback phai fallback thanh cong khi tool chinh hong VINH VIEN, du retry roi van that bai");
if (ketQuaFallback.giaTri !== "gia_tri_mac_dinh") throw new Error("fallback phai tra ve dung gia_tri_mac_dinh");
if (toolChinhHong.trangThai.soLanDaGoi !== 2) throw new Error("tool chinh hong vinh vien phai duoc thu DUNG 2 lan (ngan sach 1: lan dau + 1 lan thu lai) truoc khi fallback");
```

:::hints
- kind: attention
  body: "Hai cho trong, CA HAI la than mot vong for. harnessDonGian: moi vong tao mot taoToolLoiTamThoi() MOI, goi dinhDangKetQuaCuoi(goiToolKhongRetry(tool)), day vao mang. harnessDayDu: moi vong tao CA HAI tool (taoToolLoiTamThoi() + taoToolDuPhong()) MOI, goi dinhDangKetQuaCuoi(goiToolCoFallback(toolChinh, toolDuPhong)), day vao mang."
- kind: strategy
  body: "harnessDonGian: const ketQua: KetQuaGoiTool[] = []; for (let i = 0; i < soTacVu; i++) { const tool = taoToolLoiTamThoi(); ketQua.push(dinhDangKetQuaCuoi(goiToolKhongRetry(tool))); } return ketQua; harnessDayDu: const ketQua: KetQuaGoiTool[] = []; for (let i = 0; i < soTacVu; i++) { const toolChinh = taoToolLoiTamThoi(); const toolDuPhong = taoToolDuPhong(); ketQua.push(dinhDangKetQuaCuoi(goiToolCoFallback(toolChinh, toolDuPhong))); } return ketQua;"
- kind: one-line
  body: "Sao chep dung hai khoi o phan Strategy vao dung ham tuong ung -- KHONG doi ten bien, KHONG doi thu tu tao tool."
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 8000
- tier: output
  match: contains
  expect: "0 1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`0` so với `1` — CÙNG model, CÙNG tool, CÙNG `5` tác vụ, CHỈ khác
harness. Quest `q9.3a` — "Harness tối giản: retry, fallback, ROP" —
khép lại tại `6/6`. Track `T9.3` "Kỹ thuật Harness" giờ có bằng chứng
bằng số cho đúng luận điểm mở đầu của nó: khi model+prompt+context đều
tốt mà hệ thống VẪN fail, chỗ cần sửa LÀ chương trình bao quanh model
— không phải model.
::::

::::reflect{#nghi-lai}
BOSS này không viết thêm MỘT dòng LOGIC mới nào — `harnessDonGian` VÀ
`harnessDayDu` chỉ RÁP LẠI đúng những hàm năm bài trước đã xây:
`goiToolKhongRetry` (bài `1`) quyết định vì sao harness đơn giản LUÔN
thất bại; `goiToolCoRetryCoGioiHan` VỚI ngân sách `1` (bài `4`, tổng
quát hoá đúng "gọi lại một lần" của bài `2`) VÀ `goiToolCoFallback`
(bài `3`) cùng quyết định vì sao harness đầy đủ LUÔN thành công trên
CHÍNH tool đó; `dinhDangKetQuaCuoi` qua `andThen` (bài `5`) xác nhận
toàn bộ chuỗi xử lý VẪN LÀ đường ray ROP, không hề có `throw` nào chen
vào. Con số `0` so với `1` không đến từ việc đổi model hay đổi tool —
nó đến từ việc đổi ĐÚNG một thứ: chương trình bao quanh chúng.
::::

::::checkpoint{mastery=0.9}
::::
