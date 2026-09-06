---
id: ky-nghe-ung-dung-ai.ky-thuat-harness.circuit-breaker-giua-nhieu-yeu-cau
title: "Circuit breaker — nhớ chuỗi thất bại GIỮA nhiều yêu cầu"
summary: "taoCircuitBreaker(nguongLoiLienTiep, soLuotChoHoiPhuc) dựng một breaker có BA trạng thái ('dong'/'mo'/'nua-mo') SỐNG XUYÊN QUA nhiều lần gọi cb.goi(tool) -- khác retry (bài 2, 4): retry chỉ sống TRONG một lần gọi, breaker sống GIỮA nhiều lần gọi liên tiếp. Ở 'dong': gọi tool thật, đếm soLoiLienTiep; đủ nguongLoiLienTiep lần thất bại LIÊN TIẾP thì chuyển 'mo'. Ở 'mo': từ chối NGAY, KHÔNG gọi tool thật, đếm soLuotDaBoQua; đủ soLuotChoHoiPhuc lượt thì chuyển 'nua-mo' và THỬ gọi tool thật đúng một lần. 'nua-mo' thành công -> 'dong' (reset); 'nua-mo' thất bại (dù chỉ MỘT lần) -> 'mo' lại NGAY, không cần đủ nguongLoiLienTiep. Kịch bản taoToolHoiPhucSauNLan(3) qua taoCircuitBreaker(3,2), 6 lần gọi: kết quả [false,false,false,false,true,true], NHƯNG tool THẬT chỉ bị gọi 5 lần (không phải 6) -- một lần bị chặn ở 'mo' không tốn lệnh gọi thật. Đối chứng taoToolLuonThatBai qua taoCircuitBreaker(2,2), 5 lần gọi: tool THẬT chỉ bị gọi 3 lần, mạch kết thúc ở 'mo' vĩnh viễn."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-harness
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [kna.circuit-breaker-giua-nhieu-yeu-cau]
requires: [kna.timeout-buoc-mo-phong]
concepts: [kna.circuit-breaker-giua-nhieu-yeu-cau]
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
Retry (bài `2`, `4`) VÀ timeout (bài `7`) đều sống TRONG một lần gọi
duy nhất — hàm được gọi, thử vài lần, rồi trả về, VÀ mọi trạng thái
đó biến mất khi hàm kết thúc. Nhưng một hệ thống thật thường gọi MỘT
tool hàng trăm lần MỖI GIỜ — VÀ nếu tool đó vừa hỏng liên tiếp `10`
lần, gọi lại lần thứ `11` ngay lập tức thường LÀ lãng phí (tốn tiền,
tốn thời gian) trước khi nó kịp hồi phục. Circuit breaker LÀ cơ chế
đầu tiên trong track này có trạng thái SỐNG XUYÊN QUA nhiều lần gọi —
nó NHỚ.
::::

::::explain{#ba_trang_thai_cua_mach}
`taoCircuitBreaker(nguongLoiLienTiep, soLuotChoHoiPhuc)` dựng một
breaker VỚI ba trạng thái: `"dong"` (bình thường, gọi tool thật),
`"mo"` (từ chối NGAY, không đụng tới tool thật), VÀ `"nua-mo"` (thử
LẠI đúng một lần để xem tool đã khỏi chưa). Trạng thái NÀY sống trong
một closure, giữ nguyên giữa các lần gọi `cb.goi(tool)` khác nhau —
khác HẲN `goiToolCoRetryCoGioiHan` (bài `4`), nơi mọi biến đếm bị xoá
sạch ngay khi hàm kết thúc:

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

function taoToolHoiPhucSauNLan(soLanLoiDau: number): ToolMoPhong {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return {
    trangThai,
    goi(): KetQuaGoiTool {
      trangThai.soLanDaGoi++;
      if (trangThai.soLanDaGoi <= soLanLoiDau) {
        return { thanhCong: false, loi: "loi_tam_thoi" };
      }
      return { thanhCong: true, giaTri: "ket_qua_that" };
    },
  };
}

type TrangThaiMach = "dong" | "mo" | "nua-mo";

interface CircuitBreaker {
  trangThai(): TrangThaiMach;
  goi(tool: ToolMoPhong): KetQuaGoiTool;
}

function taoCircuitBreaker(nguongLoiLienTiep: number, soLuotChoHoiPhuc: number): CircuitBreaker {
  let trangThaiHienTai: TrangThaiMach = "dong";
  let soLoiLienTiep = 0;
  let soLuotDaBoQua = 0;

  return {
    trangThai(): TrangThaiMach {
      return trangThaiHienTai;
    },
    goi(tool: ToolMoPhong): KetQuaGoiTool {
      if (trangThaiHienTai === "mo") {
        soLuotDaBoQua++;
        if (soLuotDaBoQua < soLuotChoHoiPhuc) {
          return { thanhCong: false, loi: "mach_da_mo_tu_choi_ngay" };
        }
        trangThaiHienTai = "nua-mo";
      }

      const ketQua = tool.goi();
      if (ketQua.thanhCong) {
        trangThaiHienTai = "dong";
        soLoiLienTiep = 0;
        soLuotDaBoQua = 0;
        return ketQua;
      }

      soLoiLienTiep++;
      if (trangThaiHienTai === "nua-mo" || soLoiLienTiep >= nguongLoiLienTiep) {
        trangThaiHienTai = "mo";
        soLuotDaBoQua = 0;
      }
      return ketQua;
    },
  };
}

function chayCircuitBreakerTrenChuoiGoi(cb: CircuitBreaker, tool: ToolMoPhong, soLanGoi: number): KetQuaGoiTool[] {
  const ketQua: KetQuaGoiTool[] = [];
  for (let i = 0; i < soLanGoi; i++) {
    ketQua.push(cb.goi(tool));
  }
  return ketQua;
}

const cb = taoCircuitBreaker(3, 2);
const tool = taoToolHoiPhucSauNLan(3);
const ketQua6 = chayCircuitBreakerTrenChuoiGoi(cb, tool, 6);
console.log(JSON.stringify(ketQua6.map((k) => k.thanhCong)), tool.trangThai.soLanDaGoi);
```

```text title=readonly
[false,false,false,false,true,true] 5
```

`taoToolHoiPhucSauNLan(3)` thất bại đúng `3` lần đầu, thành công từ
lần thứ `4` — tổng quát hoá `taoToolLoiTamThoi` (bài `1`, chỉ thất bại
đúng lần đầu). Chuỗi `6` lần gọi qua breaker (`nguongLoiLienTiep=3`,
`soLuotChoHoiPhuc=2`): ba lần đầu thất bại LIÊN TIẾP mở mạch; lần thứ
`4` bị TỪ CHỐI NGAY (mạch `"mo"`, không đụng tool thật); lần thứ `5`
chuyển sang `"nua-mo"`, THỬ tool thật — LÚC NÀY tool đã hồi phục
(đây LÀ lần gọi thật thứ `4` của nó), thành công, mạch đóng lại; lần
thứ `6` gọi bình thường, cũng thành công. Tool THẬT chỉ bị gọi `5`
lần — KHÔNG phải `6` — vì lần thứ `4` bị breaker chặn HOÀN TOÀN.
::::

::::example{#mach_khong_bao_gio_dong_lai_duoc}
Đối chứng: tool hỏng VĨNH VIỄN (bài `3`) qua một breaker khác
(`nguongLoiLienTiep=2`, `soLuotChoHoiPhuc=2`), `5` lần gọi:

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

function taoToolLuonThatBai(): ToolMoPhong {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return {
    trangThai,
    goi(): KetQuaGoiTool {
      trangThai.soLanDaGoi++;
      return { thanhCong: false, loi: "loi_vinh_vien" };
    },
  };
}

type TrangThaiMach = "dong" | "mo" | "nua-mo";

interface CircuitBreaker {
  trangThai(): TrangThaiMach;
  goi(tool: ToolMoPhong): KetQuaGoiTool;
}

function taoCircuitBreaker(nguongLoiLienTiep: number, soLuotChoHoiPhuc: number): CircuitBreaker {
  let trangThaiHienTai: TrangThaiMach = "dong";
  let soLoiLienTiep = 0;
  let soLuotDaBoQua = 0;

  return {
    trangThai(): TrangThaiMach {
      return trangThaiHienTai;
    },
    goi(tool: ToolMoPhong): KetQuaGoiTool {
      if (trangThaiHienTai === "mo") {
        soLuotDaBoQua++;
        if (soLuotDaBoQua < soLuotChoHoiPhuc) {
          return { thanhCong: false, loi: "mach_da_mo_tu_choi_ngay" };
        }
        trangThaiHienTai = "nua-mo";
      }

      const ketQua = tool.goi();
      if (ketQua.thanhCong) {
        trangThaiHienTai = "dong";
        soLoiLienTiep = 0;
        soLuotDaBoQua = 0;
        return ketQua;
      }

      soLoiLienTiep++;
      if (trangThaiHienTai === "nua-mo" || soLoiLienTiep >= nguongLoiLienTiep) {
        trangThaiHienTai = "mo";
        soLuotDaBoQua = 0;
      }
      return ketQua;
    },
  };
}

function chayCircuitBreakerTrenChuoiGoi(cb: CircuitBreaker, tool: ToolMoPhong, soLanGoi: number): KetQuaGoiTool[] {
  const ketQua: KetQuaGoiTool[] = [];
  for (let i = 0; i < soLanGoi; i++) {
    ketQua.push(cb.goi(tool));
  }
  return ketQua;
}

const cb2 = taoCircuitBreaker(2, 2);
const toolHong = taoToolLuonThatBai();
const ketQua2 = chayCircuitBreakerTrenChuoiGoi(cb2, toolHong, 5);
console.log(JSON.stringify(ketQua2.map((k) => k.thanhCong)));
console.log("so lan goi tool THAT:", toolHong.trangThai.soLanDaGoi);
console.log("trang thai mach cuoi cung:", cb2.trangThai());
```

```text title=readonly
[false,false,false,false,false]
so lan goi tool THAT: 3
trang thai mach cuoi cung: mo
```

`5` lần gọi qua breaker, nhưng tool THẬT chỉ bị gọi `3` lần: hai lần
đầu (thất bại liên tiếp, mở mạch), MỘT lần thử Ở `"nua-mo"` (lần gọi
thứ `4` qua breaker — thất bại NGAY LẬP TỨC đóng mạch lại, không cần
đủ `nguongLoiLienTiep` lần liên tiếp như trạng thái `"dong"`), VÀ lần
gọi thứ `3`/thứ `5` qua breaker bị từ chối NGAY (mạch `"mo"`). Mạch
kết thúc Ở `"mo"` — tool không bao giờ khỏi, nên nó không bao giờ có
cơ hội đóng lại vĩnh viễn. So VỚI fallback (bài `3`, luôn có một tool
dự phòng thành công), circuit breaker KHÔNG cứu completion rate — nó
chỉ NGỪNG tốn lệnh gọi thật vào một tool đã biết LÀ vô dụng.
::::

::::predict{#doan-nua-mo-that-bai commitOnce}
Ở lần gọi thứ `4` qua `cb2` (mạch đang Ở trạng thái `"nua-mo"`, thử
gọi tool thật đúng một lần) — tool đó thất bại. Mạch chuyển sang
trạng thái gì NGAY SAU đó: `"dong"`, `"mo"`, hay VẪN `"nua-mo"`?

:::opt{correct}
`"mo"` — MỘT lần thất bại DUY NHẤT Ở trạng thái `"nua-mo"` LÀ đủ để
đóng mạch lại ngay; điều kiện `trangThaiHienTai === "nua-mo"` trong
nhánh xử lý thất bại LUÔN đúng khi đang Ở đó, không cần đợi đủ
`nguongLoiLienTiep` lần liên tiếp như trạng thái `"dong"`
:::
:::opt
VẪN `"nua-mo"` — breaker cho thử thêm một lần nữa trước khi quyết
định hẳn
::why
Nhầm rằng `"nua-mo"` cho phép NHIỀU lần thử trước khi quyết định —
nhưng code chỉ cho ĐÚNG một lần thử: ngay khi vào `"nua-mo"`, dòng kế
tiếp LÀ `const ketQua = tool.goi();` — một lần gọi DUY NHẤT, rồi rẽ
nhánh thành công/thất bại NGAY LẬP TỨC, không có vòng lặp thử lại
nào Ở đây.

Chỗ lệch: `trangThaiHienTai === "nua-mo"` trong nhánh thất bại LÀ một
điều kiện, không phải một BỘ ĐẾM — nó không "cho phép vài lần" rồi mới
quyết định, nó quyết định NGAY Ở lần thất bại đầu tiên (và cũng LÀ duy
nhất) đó.
::
:::
:::opt
`"dong"` — vì tool "chỉ" thất bại một lần, chưa đủ `nguongLoiLienTiep`
(`2`) lần LIÊN TIẾP để coi LÀ hỏng thật
::why
Nhầm áp dụng ngưỡng của trạng thái `"dong"` (cần đủ `nguongLoiLienTiep`
lần liên tiếp) VÀO trạng thái `"nua-mo"` — nhưng `"nua-mo"` có luật
RIÊNG, khắt khe HƠN NHIỀU: bất kỳ thất bại nào (dù chỉ `1` lần) đều đủ
để đóng mạch lại NGAY, không cần đạt ngưỡng nào cả.

Chỗ lệch: dòng `if (trangThaiHienTai === "nua-mo" || soLoiLienTiep >=
nguongLoiLienTiep)` LÀ một phép toán `||` — CHỈ CẦN một Ở HAI điều
kiện đúng LÀ đủ đóng mạch, VÀ điều kiện đầu (đang Ở `"nua-mo"`) không
hề phụ thuộc `soLoiLienTiep`.
::
:::
::::

::::code{#viet_circuit_breaker}
Hoàn thiện thân hàm `goi` bên trong `taoCircuitBreaker` — NẾU đang
`"mo"`: tăng `soLuotDaBoQua`, nếu CHƯA đủ `soLuotChoHoiPhuc` thì từ
chối NGAY (không gọi `tool.goi()`); nếu ĐÃ đủ, chuyển sang `"nua-mo"`
(rồi đi tiếp xuống dưới). Sau đó (đang `"dong"` hoặc vừa vào
`"nua-mo"`): gọi `tool.goi()` thật; NẾU thành công, chuyển `"dong"` VÀ
reset cả hai bộ đếm; NẾU thất bại, tăng `soLoiLienTiep`, VÀ nếu đang
`"nua-mo"` HOẶC đã đủ `nguongLoiLienTiep` lần liên tiếp thì chuyển
`"mo"` (reset `soLuotDaBoQua`). Hoàn thiện
`chayCircuitBreakerTrenChuoiGoi` — gọi `cb.goi(tool)` đúng `soLanGoi`
lần LIÊN TIẾP trên CÙNG một `cb` VÀ CÙNG một `tool`, đẩy từng kết quả
vào mảng trả về.

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

function taoToolHoiPhucSauNLan(soLanLoiDau: number): ToolMoPhong {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return {
    trangThai,
    goi(): KetQuaGoiTool {
      trangThai.soLanDaGoi++;
      if (trangThai.soLanDaGoi <= soLanLoiDau) {
        return { thanhCong: false, loi: "loi_tam_thoi" };
      }
      return { thanhCong: true, giaTri: "ket_qua_that" };
    },
  };
}

function taoToolLuonThatBai(): ToolMoPhong {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return {
    trangThai,
    goi(): KetQuaGoiTool {
      trangThai.soLanDaGoi++;
      return { thanhCong: false, loi: "loi_vinh_vien" };
    },
  };
}

type TrangThaiMach = "dong" | "mo" | "nua-mo";

interface CircuitBreaker {
  trangThai(): TrangThaiMach;
  goi(tool: ToolMoPhong): KetQuaGoiTool;
}

function taoCircuitBreaker(nguongLoiLienTiep: number, soLuotChoHoiPhuc: number): CircuitBreaker {
  let trangThaiHienTai: TrangThaiMach = "dong";
  let soLoiLienTiep = 0;
  let soLuotDaBoQua = 0;

  return {
    trangThai(): TrangThaiMach {
      return trangThaiHienTai;
    },
    goi(tool: ToolMoPhong): KetQuaGoiTool {
      ___
    },
  };
}

function chayCircuitBreakerTrenChuoiGoi(cb: CircuitBreaker, tool: ToolMoPhong, soLanGoi: number): KetQuaGoiTool[] {
  ___
}

const cb = taoCircuitBreaker(3, 2);
const tool = taoToolHoiPhucSauNLan(3);
const ketQua6 = chayCircuitBreakerTrenChuoiGoi(cb, tool, 6);
console.log(JSON.stringify(ketQua6.map((k) => k.thanhCong)), tool.trangThai.soLanDaGoi);
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

function taoToolHoiPhucSauNLan(soLanLoiDau: number): ToolMoPhong {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return {
    trangThai,
    goi(): KetQuaGoiTool {
      trangThai.soLanDaGoi++;
      if (trangThai.soLanDaGoi <= soLanLoiDau) {
        return { thanhCong: false, loi: "loi_tam_thoi" };
      }
      return { thanhCong: true, giaTri: "ket_qua_that" };
    },
  };
}

function taoToolLuonThatBai(): ToolMoPhong {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return {
    trangThai,
    goi(): KetQuaGoiTool {
      trangThai.soLanDaGoi++;
      return { thanhCong: false, loi: "loi_vinh_vien" };
    },
  };
}

type TrangThaiMach = "dong" | "mo" | "nua-mo";

interface CircuitBreaker {
  trangThai(): TrangThaiMach;
  goi(tool: ToolMoPhong): KetQuaGoiTool;
}

function taoCircuitBreaker(nguongLoiLienTiep: number, soLuotChoHoiPhuc: number): CircuitBreaker {
  let trangThaiHienTai: TrangThaiMach = "dong";
  let soLoiLienTiep = 0;
  let soLuotDaBoQua = 0;

  return {
    trangThai(): TrangThaiMach {
      return trangThaiHienTai;
    },
    goi(tool: ToolMoPhong): KetQuaGoiTool {
      if (trangThaiHienTai === "mo") {
        soLuotDaBoQua++;
        if (soLuotDaBoQua < soLuotChoHoiPhuc) {
          return { thanhCong: false, loi: "mach_da_mo_tu_choi_ngay" };
        }
        trangThaiHienTai = "nua-mo";
      }

      const ketQua = tool.goi();
      if (ketQua.thanhCong) {
        trangThaiHienTai = "dong";
        soLoiLienTiep = 0;
        soLuotDaBoQua = 0;
        return ketQua;
      }

      soLoiLienTiep++;
      if (trangThaiHienTai === "nua-mo" || soLoiLienTiep >= nguongLoiLienTiep) {
        trangThaiHienTai = "mo";
        soLuotDaBoQua = 0;
      }
      return ketQua;
    },
  };
}

function chayCircuitBreakerTrenChuoiGoi(cb: CircuitBreaker, tool: ToolMoPhong, soLanGoi: number): KetQuaGoiTool[] {
  const ketQua: KetQuaGoiTool[] = [];
  for (let i = 0; i < soLanGoi; i++) {
    ketQua.push(cb.goi(tool));
  }
  return ketQua;
}

const cb = taoCircuitBreaker(3, 2);
const tool = taoToolHoiPhucSauNLan(3);
const ketQua6 = chayCircuitBreakerTrenChuoiGoi(cb, tool, 6);
console.log(JSON.stringify(ketQua6.map((k) => k.thanhCong)), tool.trangThai.soLanDaGoi);
```

```typescript title=test
if (ketQua6.length !== 6) throw new Error("chayCircuitBreakerTrenChuoiGoi phai tra ve mang dung soLanGoi (6) phan tu");
if (JSON.stringify(ketQua6.map((k) => k.thanhCong)) !== JSON.stringify([false, false, false, false, true, true])) {
  throw new Error("chuoi 6 lan goi phai la [false,false,false,false,true,true] -- 3 that bai lien tiep mo mach, 1 lan bi tu choi, roi 2 lan thanh cong qua nua-mo va dong");
}
if (tool.trangThai.soLanDaGoi !== 5) {
  throw new Error("tool THAT chi duoc goi 5 lan (khong phai 6) -- mach mo phai chan MOT lan goi that, tu choi ngay khong ton lenh goi that");
}
if (cb.trangThai() !== "dong") throw new Error("sau khi tool hoi phuc thanh cong qua nua-mo, mach phai dong lai hoan toan");

const cbRieng = taoCircuitBreaker(2, 2);
const toolHong = taoToolLuonThatBai();
const ketQuaRieng = chayCircuitBreakerTrenChuoiGoi(cbRieng, toolHong, 5);
if (JSON.stringify(ketQuaRieng.map((k) => k.thanhCong)) !== JSON.stringify([false, false, false, false, false])) {
  throw new Error("tool hong vinh vien thi CA 5 lan goi qua mach phai la false");
}
if (toolHong.trangThai.soLanDaGoi !== 3) {
  throw new Error("voi nguong 2 va cho-hoi-phuc 2, tool THAT chi duoc goi dung 3 lan trong 5 lan goi qua mach (2 lan dau + 1 lan thu nua-mo that bai) -- 2 lan con lai bi tu choi ngay");
}
if (cbRieng.trangThai() !== "mo") throw new Error("tool hong vinh vien thi mach phai KET THUC o trang thai mo, khong bao gio dong lai duoc");

const cbNgan = taoCircuitBreaker(3, 2);
const toolNgan = taoToolHoiPhucSauNLan(3);
const ketQuaNgan = chayCircuitBreakerTrenChuoiGoi(cbNgan, toolNgan, 4);
if (ketQuaNgan.length !== 4) throw new Error("doi soLanGoi tu 6 sang 4 phai doi do dai mang tra ve -- tham so phai duoc dung that");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (than ham goi ben trong taoCircuitBreaker): neu dang 'mo', tang soLuotDaBoQua, chua du soLuotChoHoiPhuc thi tu choi ngay (khong goi tool.goi()); du roi thi chuyen 'nua-mo' va DI TIEP xuong duoi (khong return o day). Sau do luon goi tool.goi() that: thanh cong thi chuyen 'dong' va reset ca hai bo dem; that bai thi tang soLoiLienTiep, va neu dang 'nua-mo' HOAC du nguongLoiLienTiep thi chuyen 'mo' (reset soLuotDaBoQua). Cho hai (chayCircuitBreakerTrenChuoiGoi): mot vong for goi cb.goi(tool) dung soLanGoi lan tren CUNG mot cb va CUNG mot tool, day tung ket qua vao mang."
- kind: strategy
  body: "Cho dau: if (trangThaiHienTai === \"mo\") { soLuotDaBoQua++; if (soLuotDaBoQua < soLuotChoHoiPhuc) { return { thanhCong: false, loi: \"mach_da_mo_tu_choi_ngay\" }; } trangThaiHienTai = \"nua-mo\"; } const ketQua = tool.goi(); if (ketQua.thanhCong) { trangThaiHienTai = \"dong\"; soLoiLienTiep = 0; soLuotDaBoQua = 0; return ketQua; } soLoiLienTiep++; if (trangThaiHienTai === \"nua-mo\" || soLoiLienTiep >= nguongLoiLienTiep) { trangThaiHienTai = \"mo\"; soLuotDaBoQua = 0; } return ketQua; Cho hai: const ketQua: KetQuaGoiTool[] = []; for (let i = 0; i < soLanGoi; i++) { ketQua.push(cb.goi(tool)); } return ketQua;"
- kind: one-line
  body: "Sao chep dung hai khoi o phan Strategy vao dung vi tri tuong ung, GIU NGUYEN thu tu cac nhanh if."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "[false,false,false,false,true,true] 5"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`5` lần gọi tool THẬT, không phải `6` — breaker tự chặn đúng MỘT lần
gọi lẽ ra sẽ lãng phí. Nhưng breaker Ở bài này chỉ TỪ CHỐI, nó không
hề THAY thế bằng một kết quả nào khác — khi mạch `"mo"`, tác vụ đó
đơn giản LÀ thất bại. Bài sau quay lại một câu hỏi đã gặp Ở bài `3`
(schema tool-call), nhưng từ góc harness: TRƯỚC khi gọi bất kỳ tool
nào — kể cả qua breaker — làm sao biết đối số gửi vào nó có ĐÚNG hình
dạng hay không?
::::

::::reflect{#nghi-lai}
Circuit breaker KHÔNG phải LÀ "retry mạnh hơn" — nó LÀ một TRỤC khác
hẳn. Retry (bài `2`, `4`) VÀ timeout (bài `7`) đều hỏi: "trong LẦN GỌI
NÀY, tôi nên thử lại/chờ bao lâu?" — VÀ câu trả lời biến mất ngay khi
hàm return. Circuit breaker hỏi một câu KHÁC: "qua NHIỀU lần gọi liên
tiếp, tool này có đáng tin không?" — VÀ câu trả lời đó PHẢI sống sót
qua ranh giới của từng lần gọi, nằm trong một closure dùng chung. Đây
chính LÀ lý do MASTERPLAN gọi retry LÀ "trong-một-yêu-cầu" VÀ circuit
breaker LÀ "giữa-nhiều-yêu-cầu": không phải khác nhau Ở CƠ CHẾ (cả hai
đều đếm VÀ so sánh VỚI một ngưỡng), mà khác nhau Ở TRẠNG THÁI đó SỐNG
được BAO LÂU. Luật "một thất bại Ở nửa-mở LÀ đủ để đóng mạch lại
ngay" — khắt khe HƠN NHIỀU so VỚI ngưỡng của trạng thái đóng — không
phải LÀ chi tiết vụn vặt: nó LÀ lý do circuit breaker không bao giờ
đánh cược nhiều hơn MỘT lần gọi thật vào một tool vừa mới nghi ngờ LÀ
đã khỏi.
::::

::::checkpoint{mastery=0.88}
::::
