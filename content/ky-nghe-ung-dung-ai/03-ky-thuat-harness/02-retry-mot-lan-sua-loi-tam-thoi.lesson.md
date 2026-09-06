---
id: ky-nghe-ung-dung-ai.ky-thuat-harness.retry-mot-lan-sua-loi-tam-thoi
title: "Retry một lần sửa được lỗi tạm thời"
summary: "goiToolCoRetry(tool) gọi tool một lần; NẾU thất bại, thử lại ĐÚNG một lần nữa trước khi trả về kết quả cuối (không lặp thêm). chayHarnessCoRetryTrenNTacVu(soTacVu) chạy trên CÙNG taoToolLoiTamThoi (bài trước) và CÙNG N=5 tác vụ. Completion rate NHẢY từ 0 (bài trước, không retry) lên 1 (5/5) — CHỈ đổi harness (thêm một dòng thử lại), model/tool giữ NGUYÊN. Đối chứng bằng taoToolLuonThanhCong: nếu lần đầu ĐÃ thành công, goiToolCoRetry KHÔNG gọi thêm lần nào (soLanDaGoi vẫn là 1) — retry chỉ kích hoạt khi thật sự cần."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-harness
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 9
teaches: [kna.retry-mot-lan-sua-loi-tam-thoi]
requires: [kna.tool-mo-phong-loi-tam-thoi-khong-retry]
concepts: [kna.retry-mot-lan-sua-loi-tam-thoi]
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
Bài trước: completion rate `0`, dù tool THẬT SỰ thành công từ lần gọi
thứ hai. Bài này đổi ĐÚNG một điều — cho harness quyền gọi lại MỘT LẦN
khi lần đầu thất bại — VÀ đo lại trên CHÍNH tool đó, CHÍNH `5` tác vụ
đó. Không đổi model, không đổi tool, không đổi prompt. Chỉ đổi
harness.
::::

::::explain{#goi_tool_co_retry}
`goiToolCoRetry` gọi tool MỘT LẦN. Nếu thành công, trả về ngay — KHÔNG
gọi thêm. Nếu thất bại, thử lại ĐÚNG MỘT LẦN NỮA rồi trả về kết quả đó
(dù thành công hay vẫn thất bại) — không có vòng lặp, không thử lần
thứ ba:

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

function goiToolCoRetry(tool: ToolMoPhong): KetQuaGoiTool {
  const lanDau = tool.goi();
  if (lanDau.thanhCong) return lanDau;
  return tool.goi();
}

const mot = taoToolLoiTamThoi();
console.log(JSON.stringify(goiToolCoRetry(mot)), mot.trangThai.soLanDaGoi);
```

```text title=readonly
{"thanhCong":true,"giaTri":"ket_qua_that"} 2
```

Lần gọi ĐẦU của `mot` LUÔN thất bại (đúng quy luật `taoToolLoiTamThoi`
đã học Ở bài trước) — nhưng `goiToolCoRetry` không dừng lại Ở đó, nó
gọi lại NGAY, và lần gọi THỨ HAI LUÔN thành công. Kết quả cuối LÀ
thành công, VÀ tool bị gọi ĐÚNG `2` lần — không phải `1` (như bài
trước), cũng không phải `3` hay nhiều hơn.
::::

::::example{#harness_co_retry_tren_n_tac_vu}
Đo trên CÙNG cấu hình bài trước — `taoToolLoiTamThoi` MỚI cho mỗi tác
vụ, `5` tác vụ:

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

function goiToolCoRetry(tool: ToolMoPhong): KetQuaGoiTool {
  const lanDau = tool.goi();
  if (lanDau.thanhCong) return lanDau;
  return tool.goi();
}

function tinhTyLeHoanThanh(ketQua: KetQuaGoiTool[]): number {
  if (ketQua.length === 0) return 0;
  const soThanhCong = ketQua.filter((k) => k.thanhCong).length;
  return soThanhCong / ketQua.length;
}

const SO_TAC_VU = 5;
const ketQuaCoRetry: KetQuaGoiTool[] = [];
for (let i = 0; i < SO_TAC_VU; i++) {
  const tool = taoToolLoiTamThoi();
  ketQuaCoRetry.push(goiToolCoRetry(tool));
}
console.log(JSON.stringify(ketQuaCoRetry.map((k) => k.thanhCong)));
console.log(tinhTyLeHoanThanh(ketQuaCoRetry));
```

```text title=readonly
[true,true,true,true,true]
1
```

CÙNG `taoToolLoiTamThoi`, CÙNG `5` tác vụ Ở bài trước — completion
rate nhảy từ `0` lên `1`. KHÔNG một dòng nào của tool bị sửa. Sự khác
biệt DUY NHẤT LÀ harness: `goiToolKhongRetry` gọi một lần rồi dừng,
`goiToolCoRetry` gọi lại khi cần.
::::

::::predict{#doan-tool-luon-that-bai commitOnce}
Nếu `goiToolCoRetry` được gọi trên một tool mà LUÔN LUÔN thất bại
(không bao giờ thành công dù gọi bao nhiêu lần) — kết quả cuối cùng
LÀ gì, VÀ tool đó bị gọi tất cả bao nhiêu lần?

:::opt{correct}
Kết quả cuối LÀ thất bại (`thanhCong: false`), VÀ tool bị gọi ĐÚNG `2`
lần (lần đầu + đúng MỘT lần thử lại) — `goiToolCoRetry` không lặp
thêm, dù tool vẫn hỏng
:::
:::opt
Nó sẽ tiếp tục gọi lại nhiều lần nữa, cho tới khi tool thành công hoặc
chương trình bị treo
::why
Nhầm giữa "retry vô hạn" VỚI thiết kế THẬT của `goiToolCoRetry` — hàm
chỉ có ĐÚNG một cấu trúc `if`/`return` rồi một lần gọi lại, không có
vòng lặp nào Ở đây cả.

Chỗ lệch: `goiToolCoRetry` KHÔNG lặp — nó gọi tool đúng hai lần TỐI ĐA
rồi trả về BẤT KỂ kết quả LÀ gì. Ràng buộc TỔNG QUÁT hơn cho việc
"thử lại nhiều lần nhưng có giới hạn" LÀ chủ đề của một bài SAU trong
quest này.
::
:::
:::opt
Nó sẽ ném (throw) một exception báo lỗi retry thất bại
::why
Nhầm với try/catch — nhưng `goiToolCoRetry` (VÀ cả tool mô phỏng)
không throw exception nào Ở BẤT KỲ đâu.

Chỗ lệch: mọi lần gọi LUÔN trả về một GIÁ TRỊ `KetQuaGoiTool` (thành
công hoặc thất bại), không bao giờ ném lỗi — đúng tinh thần
Railway-Oriented Programming một bài SAU trong quest sẽ đặt tên chính
thức cho cách biểu diễn lỗi này.
::
:::
::::

::::code{#viet_goi_tool_co_retry}
Hoàn thiện `goiToolCoRetry` — gọi `tool.goi()` một lần; NẾU kết quả đó
đã thành công thì trả về NGAY; nếu KHÔNG, gọi `tool.goi()` một lần nữa
VÀ trả về kết quả đó (dù thành công hay thất bại). Hoàn thiện
`chayHarnessCoRetryTrenNTacVu` — với MỖI trong `soTacVu` tác vụ: tạo
một `taoToolLoiTamThoi()` MỚI, gọi `goiToolCoRetry` trên nó, đẩy kết
quả vào mảng trả về.

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

function taoToolLuonThanhCong(): ToolMoPhong {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return {
    trangThai,
    goi(): KetQuaGoiTool {
      trangThai.soLanDaGoi++;
      return { thanhCong: true, giaTri: "luon_ok" };
    },
  };
}

function tinhTyLeHoanThanh(ketQua: KetQuaGoiTool[]): number {
  if (ketQua.length === 0) return 0;
  const soThanhCong = ketQua.filter((k) => k.thanhCong).length;
  return soThanhCong / ketQua.length;
}

function goiToolCoRetry(tool: ToolMoPhong): KetQuaGoiTool {
  ___
}

function chayHarnessCoRetryTrenNTacVu(soTacVu: number): KetQuaGoiTool[] {
  ___
}

const ketQua5 = chayHarnessCoRetryTrenNTacVu(5);
console.log(JSON.stringify(ketQua5.map((k) => k.thanhCong)), tinhTyLeHoanThanh(ketQua5));
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

function taoToolLuonThanhCong(): ToolMoPhong {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return {
    trangThai,
    goi(): KetQuaGoiTool {
      trangThai.soLanDaGoi++;
      return { thanhCong: true, giaTri: "luon_ok" };
    },
  };
}

function tinhTyLeHoanThanh(ketQua: KetQuaGoiTool[]): number {
  if (ketQua.length === 0) return 0;
  const soThanhCong = ketQua.filter((k) => k.thanhCong).length;
  return soThanhCong / ketQua.length;
}

function goiToolCoRetry(tool: ToolMoPhong): KetQuaGoiTool {
  const lanDau = tool.goi();
  if (lanDau.thanhCong) return lanDau;
  return tool.goi();
}

function chayHarnessCoRetryTrenNTacVu(soTacVu: number): KetQuaGoiTool[] {
  const ketQua: KetQuaGoiTool[] = [];
  for (let i = 0; i < soTacVu; i++) {
    const tool = taoToolLoiTamThoi();
    ketQua.push(goiToolCoRetry(tool));
  }
  return ketQua;
}

const ketQua5 = chayHarnessCoRetryTrenNTacVu(5);
console.log(JSON.stringify(ketQua5.map((k) => k.thanhCong)), tinhTyLeHoanThanh(ketQua5));
```

```typescript title=test
if (ketQua5.length !== 5) throw new Error("chayHarnessCoRetryTrenNTacVu(5) phai tra ve mang dung 5 phan tu");
if (JSON.stringify(ketQua5.map((k) => k.thanhCong)) !== JSON.stringify([true, true, true, true, true])) {
  throw new Error("co retry, moi tac vu phai THANH CONG -- lan goi thu hai luon thanh cong dung quy luat taoToolLoiTamThoi");
}
if (tinhTyLeHoanThanh(ketQua5) !== 1) throw new Error("completion rate tren 5 tac vu CO retry phai la 1 (100%) -- doi voi bai truoc (0)");

const ketQua3 = chayHarnessCoRetryTrenNTacVu(3);
if (ketQua3.length !== 3) throw new Error("doi so tac vu tu 5 sang 3 phai doi do dai mang tra ve -- tham so phai duoc dung that");

const toolRieng = taoToolLoiTamThoi();
const kq1 = goiToolCoRetry(toolRieng);
if (kq1.thanhCong !== true) throw new Error("goiToolCoRetry tren tool loi tam thoi phai THANH CONG sau khi thu lai");
if (toolRieng.trangThai.soLanDaGoi !== 2) throw new Error("goiToolCoRetry phai goi tool DUNG 2 LAN (lan dau + dung 1 lan thu lai), khong hon");

const toolLuonOk = taoToolLuonThanhCong();
const kqOk = goiToolCoRetry(toolLuonOk);
if (kqOk.thanhCong !== true) throw new Error("neu lan dau da thanh cong thi ket qua phai thanh cong");
if (toolLuonOk.trangThai.soLanDaGoi !== 1) throw new Error("neu lan dau DA thanh cong, KHONG duoc goi lai them lan nao -- goiToolCoRetry phai dung lai ngay");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (goiToolCoRetry): goi tool.goi() luu vao mot bien, neu no thanh cong thi return NGAY bien do; nguoc lai goi tool.goi() THEM MOT LAN NUA va return ket qua do. Cho hai (chayHarnessCoRetryTrenNTacVu): giong het bai truoc nhung goi goiToolCoRetry thay vi goiToolKhongRetry."
- kind: strategy
  body: "Cho dau: const lanDau = tool.goi(); if (lanDau.thanhCong) return lanDau; return tool.goi(); Cho hai: const ketQua: KetQuaGoiTool[] = []; for (let i = 0; i < soTacVu; i++) { const tool = taoToolLoiTamThoi(); ketQua.push(goiToolCoRetry(tool)); } return ketQua;"
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
  expect: "[true,true,true,true,true] 1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Từ `0` lên `1` — chỉ bằng cách cho harness quyền gọi lại MỘT lần khi
lần đầu thất bại. Model VÀ tool y NGUYÊN như bài trước. Nhưng retry
CHỈ cứu được lỗi TẠM THỜI — nếu tool hỏng THẬT SỰ (không phải tạm
thời), gọi lại bao nhiêu lần cũng vô ích. Bài sau xử lý đúng trường
hợp đó: đổi sang một tool KHÁC khi tool chính đã hết cách.
::::

::::reflect{#nghi-lai}
Bài này chứng minh đúng luận điểm cốt lõi của T9.3 lần đầu tiên bằng
số: CÙNG model, CÙNG tool, CÙNG tập tác vụ — chỉ đổi HARNESS (thêm
đúng một lần thử lại) — completion rate đổi từ `0` sang `1`. Nhưng
retry không phải LÀ phép màu vạn năng: nó chỉ hoạt động vì
`taoToolLoiTamThoi` mô phỏng đúng một lớp lỗi cụ thể — lỗi TẠM THỜI,
tự khỏi Ở lần gọi sau. Đối chứng `taoToolLuonThanhCong` cũng quan
trọng không kém: retry phải BIẾT dừng lại khi không cần — gọi thêm
một lần vô ích vào một tool ĐÃ thành công là lãng phí, dù không sai.
::::

::::checkpoint{mastery=0.8}
::::
