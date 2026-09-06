---
id: ky-nghe-ung-dung-ai.ky-thuat-harness.so-sanh-cau-hinh-harness-qua-bang-danh-gia
title: "So sánh nhiều cấu hình harness qua một bảng đánh giá"
summary: "CauHinhHarness {ten, soLanThuLaiToiDa, dungCircuitBreaker} mo ta MOT lua chon phong thu. chayMotCauHinh(cauHinh, taoTool, soTacVu) chay soTacVu tac vu qua DUNG MOT cau hinh, tra ve KetQuaDanhGia {ten, tyLeHoanThanh, soLanGoiTrungBinh} -- soLanGoiTrungBinh (tong so lan goi tool THAT chia so tac vu) LA proxy cho 'do tre/chi phi' mo phong (khong dung Date.now() that). soSanhNhieuCauHinh(cacCauHinh, taoTool, soTacVu) chay CUNG mot bo N tac vu qua NHIEU cau hinh, tra ve mang KetQuaDanhGia de xep thanh bang. Truc retry (tren taoToolLoiTamThoi, tool MOI moi tac vu): {retry:0} cho tyLeHoanThanh=0, soLanGoiTrungBinh=1; {retry:1} cho tyLeHoanThanh=1, soLanGoiTrungBinh=2 -- tang ca hai truc. Truc breaker (tren MOT taoToolLuonThatBai DUNG CHUNG qua taoNhaMayToolDungChung, su co he thong): {breaker:false} va {breaker:true} CUNG tyLeHoanThanh=0 (breaker KHONG cuu duoc loi he thong) nhung soLanGoiTrungBinh khac han (1 so voi 0.6) -- breaker chi giam CHI PHI, khong tang completion rate khi loi la vinh vien."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-harness
order: 11
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [kna.so-sanh-cau-hinh-harness-qua-bang-danh-gia]
requires: [kna.dinh-tuyen-nhieu-tool-theo-loai-tac-vu]
concepts: [kna.so-sanh-cau-hinh-harness-qua-bang-danh-gia]
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
Suốt mười bài, mỗi lần so sánh hai harness LÀ một cặp `console.log`
viết tay, đọc bằng mắt. Nhưng một đội kỹ sư thật không so sánh HAI
harness một lần — họ so sánh CHỤC cấu hình, LIÊN TỤC, mỗi khi đổi một
tham số. Bài này đóng gói phép so sánh đó thành một HÀM: đưa vào MỘT
danh sách cấu hình, nhận về một BẢNG kết quả — bước đầu tiên của một
hạ tầng đánh giá (evaluation infra) thật.
::::

::::explain{#cau_hinh_va_ket_qua_danh_gia}
`CauHinhHarness` gói TÊN một lựa chọn phòng thủ VÀ hai tham số:
`soLanThuLaiToiDa` (ngân sách retry, bài `4`) VÀ `dungCircuitBreaker`
(bật/tắt breaker, bài `8`). `chayMotCauHinh` chạy `soTacVu` tác vụ qua
ĐÚNG MỘT cấu hình đó, trả về `KetQuaDanhGia` — completion rate CỘNG
`soLanGoiTrungBinh` (tổng số lần gọi tool THẬT chia số tác vụ), một
PROXY cho độ trễ/chi phí mô phỏng (không dùng đồng hồ thật, chỉ đếm số
lần gọi):

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

function tinhTyLeHoanThanh(ketQua: KetQuaGoiTool[]): number {
  if (ketQua.length === 0) return 0;
  return ketQua.filter((k) => k.thanhCong).length / ketQua.length;
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

interface CauHinhHarness {
  ten: string;
  soLanThuLaiToiDa: number;
  dungCircuitBreaker: boolean;
}

interface KetQuaDanhGia {
  ten: string;
  tyLeHoanThanh: number;
  soLanGoiTrungBinh: number;
}

function chayMotCauHinh(
  cauHinh: CauHinhHarness,
  taoTool: () => ToolMoPhong,
  soTacVu: number,
): KetQuaDanhGia {
  const cb = taoCircuitBreaker(2, 2);
  const ketQua: KetQuaGoiTool[] = [];
  const congCuDaTao: ToolMoPhong[] = [];
  for (let i = 0; i < soTacVu; i++) {
    const tool = taoTool();
    if (!congCuDaTao.includes(tool)) congCuDaTao.push(tool);
    if (cauHinh.dungCircuitBreaker) {
      ketQua.push(cb.goi(tool));
    } else {
      ketQua.push(goiToolCoRetryCoGioiHan(tool, cauHinh.soLanThuLaiToiDa));
    }
  }
  const tongSoLanGoi = congCuDaTao.reduce((tong, t) => tong + t.trangThai.soLanDaGoi, 0);
  return {
    ten: cauHinh.ten,
    tyLeHoanThanh: tinhTyLeHoanThanh(ketQua),
    soLanGoiTrungBinh: tongSoLanGoi / soTacVu,
  };
}

function soSanhNhieuCauHinh(
  cacCauHinh: CauHinhHarness[],
  taoTool: () => ToolMoPhong,
  soTacVu: number,
): KetQuaDanhGia[] {
  return cacCauHinh.map((ch) => chayMotCauHinh(ch, taoTool, soTacVu));
}

const bangSoSanh1 = soSanhNhieuCauHinh(
  [
    { ten: "khong-retry", soLanThuLaiToiDa: 0, dungCircuitBreaker: false },
    { ten: "co-retry", soLanThuLaiToiDa: 1, dungCircuitBreaker: false },
  ],
  taoToolLoiTamThoi,
  5,
);
console.log(JSON.stringify(bangSoSanh1));
```

```text title=readonly
[{"ten":"khong-retry","tyLeHoanThanh":0,"soLanGoiTrungBinh":1},{"ten":"co-retry","tyLeHoanThanh":1,"soLanGoiTrungBinh":2}]
```

`taoToolLoiTamThoi` được truyền LÀM `taoTool` — MỘT tool MỚI cho MỖI
tác vụ (đúng quy ước bài `1`-`2`). Cấu hình `"khong-retry"` (ngân sách
`0`) cho completion rate `0`, trung bình `1` lần gọi MỖI tác vụ. Cấu
hình `"co-retry"` (ngân sách `1`) cho completion rate `1`, trung bình
`2` lần gọi. CÙNG một hàm `chayMotCauHinh`, CÙNG loại tool — chỉ khác
Ở CẤU HÌNH truyền vào, VÀ bảng kết quả LÀM RÕ đánh đổi: completion
rate tăng LÊN thì SỐ LẦN GỌI cũng tăng THEO, không phải MIỄN PHÍ.
::::

::::example{#truc_breaker_khong_cuu_duoc_loi_he_thong}
`taoNhaMayToolDungChung` bọc MỘT tool đã tạo sẵn thành một "factory"
LUÔN trả về ĐÚNG tool đó — dùng để mô phỏng một SỰ CỐ HỆ THỐNG DÙNG
CHUNG xuyên suốt cả `soTacVu` tác vụ CỦA MỘT cấu hình (khác VỚI
`taoToolLoiTamThoi` Ở trên, tool MỚI cho MỖI tác vụ):

```typescript title=readonly
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

function taoNhaMayToolDungChung(taoTool: () => ToolMoPhong): () => ToolMoPhong {
  const toolDuyNhat = taoTool();
  return () => toolDuyNhat;
}

const ktKhongBreaker = chayMotCauHinh(
  { ten: "khong-breaker", soLanThuLaiToiDa: 0, dungCircuitBreaker: false },
  taoNhaMayToolDungChung(taoToolLuonThatBai),
  5,
);
const ktCoBreaker = chayMotCauHinh(
  { ten: "co-breaker", soLanThuLaiToiDa: 0, dungCircuitBreaker: true },
  taoNhaMayToolDungChung(taoToolLuonThatBai),
  5,
);
console.log(JSON.stringify([ktKhongBreaker, ktCoBreaker]));
```

```text title=readonly
[{"ten":"khong-breaker","tyLeHoanThanh":0,"soLanGoiTrungBinh":1},{"ten":"co-breaker","tyLeHoanThanh":0,"soLanGoiTrungBinh":0.6}]
```

Mỗi cấu hình được gọi VỚI một `taoNhaMayToolDungChung(taoToolLuonThatBai)`
RIÊNG — hai tool "sự cố hệ thống" ĐỘC LẬP, không rò rỉ trạng thái giữa
hai cấu hình. `tyLeHoanThanh` GIỐNG NHAU HỆT (`0` cho cả hai) — breaker
KHÔNG cứu được một lỗi VĨNH VIỄN, đúng như bài `8` đã dạy. NHƯNG
`soLanGoiTrungBinh` khác HẲN: `1` so VỚI `0.6` — breaker chặn được một
phần lệnh gọi thật (`3` trên tổng `5` tác vụ, thay vì `5`/`5`). Bảng so
sánh LÀM RÕ điều mà một con số completion-rate ĐƠN LẺ không thấy được:
hai cấu hình GIỐNG NHAU Ở MỘT trục đo (tỉ lệ hoàn thành), nhưng khác
HẲN Ở trục KHÁC (chi phí).
::::

::::predict{#doan-breaker-tren-loi-he-thong commitOnce}
Dùng circuit breaker trên MỘT tool hỏng VĨNH VIỄN (như `ktCoBreaker`
Ở trên) — so VỚI KHÔNG dùng breaker (`ktKhongBreaker`) — `tyLeHoanThanh`
của hai cấu hình đó khác nhau thế nào?

:::opt{correct}
BẰNG NHAU — cả hai đều LÀ `0`. Breaker KHÔNG cải thiện completion rate
khi lỗi LÀ vĩnh viễn (không có tool dự phòng nào Ở bài này); nó CHỈ
làm giảm `soLanGoiTrungBinh` (số lần gọi tool THẬT), không hề làm
tăng số tác vụ thành công
:::
:::opt
`ktCoBreaker` CAO hơn — breaker giúp tool được "nghỉ" giữa các lần
gọi, VÀ nhờ đó có cơ hội tự hồi phục
::why
Nhầm rằng "nghỉ giữa các lần gọi" giúp một tool tự sửa lỗi — nhưng
`taoToolLuonThatBai` không phụ thuộc THỜI GIAN hay SỐ LẦN NGHỈ giữa
các lần gọi, nó LUÔN trả về thất bại BẤT KỂ được gọi khi nào hay cách
nhau bao lâu.

Chỗ lệch: `goi()` của `taoToolLuonThatBai` LUÔN `return { thanhCong:
false, ... }`, không có điều kiện nào phụ thuộc thời gian hay số lần
"nghỉ" — breaker CHẶN được lệnh gọi, nhưng không hề THAY ĐỔI được kết
quả của lệnh gọi đó khi nó THẬT SỰ xảy ra.
::
:::
:::opt
`ktCoBreaker` THẤP hơn — một số tác vụ bị mạch từ chối NGAY, không có
cơ hội thử, nên KÉM hơn cấu hình không breaker
::why
Đúng Ở việc một số tác vụ BỊ TỪ CHỐI NGAY (không chạm tool thật) —
nhưng những tác vụ ĐÓ vẫn được tính LÀ THẤT BẠI, giống HỆT kết quả nếu
KHÔNG có breaker (tool LUÔN thất bại Ở CẢ hai trường hợp).

Chỗ lệch: `tinhTyLeHoanThanh` đếm SỐ tác vụ THÀNH CÔNG — VÀ VỚI tool
hỏng vĩnh viễn, KHÔNG tác vụ nào thành công Ở CẤU HÌNH nào, dù đường đi
BÊN TRONG (từ chối ngay hay gọi thật rồi thất bại) khác nhau.
::
:::
::::

::::code{#viet_chay_va_so_sanh_cau_hinh}
Hoàn thiện `chayMotCauHinh` — chạy `soTacVu` tác vụ: mỗi tác vụ tạo
tool qua `taoTool()`, nếu tool đó CHƯA có trong `congCuDaTao` thì thêm
vào; NẾU `cauHinh.dungCircuitBreaker`, gọi QUA breaker (`cb.goi(tool)`);
NGƯỢC LẠI, gọi qua `goiToolCoRetryCoGioiHan(tool, cauHinh.soLanThuLaiToiDa)`;
cuối cùng trả về `KetQuaDanhGia` (tính `tyLeHoanThanh` VÀ
`soLanGoiTrungBinh` LÀ tổng `soLanDaGoi` của MỌI tool trong
`congCuDaTao`, chia `soTacVu`). Hoàn thiện `soSanhNhieuCauHinh` — chạy
`chayMotCauHinh` cho TỪNG cấu hình trong `cacCauHinh`, trả về mảng kết
quả.

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

function goiToolCoRetryCoGioiHan(tool: ToolMoPhong, soLanThuLaiToiDa: number): KetQuaGoiTool {
  let ketQua = tool.goi();
  let soLanDaThuLai = 0;
  while (!ketQua.thanhCong && soLanDaThuLai < soLanThuLaiToiDa) {
    ketQua = tool.goi();
    soLanDaThuLai++;
  }
  return ketQua;
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

function tinhTyLeHoanThanh(ketQua: KetQuaGoiTool[]): number {
  if (ketQua.length === 0) return 0;
  return ketQua.filter((k) => k.thanhCong).length / ketQua.length;
}

interface CauHinhHarness {
  ten: string;
  soLanThuLaiToiDa: number;
  dungCircuitBreaker: boolean;
}

interface KetQuaDanhGia {
  ten: string;
  tyLeHoanThanh: number;
  soLanGoiTrungBinh: number;
}

function taoNhaMayToolDungChung(taoTool: () => ToolMoPhong): () => ToolMoPhong {
  const toolDuyNhat = taoTool();
  return () => toolDuyNhat;
}

function chayMotCauHinh(
  cauHinh: CauHinhHarness,
  taoTool: () => ToolMoPhong,
  soTacVu: number,
): KetQuaDanhGia {
  ___
}

function soSanhNhieuCauHinh(
  cacCauHinh: CauHinhHarness[],
  taoTool: () => ToolMoPhong,
  soTacVu: number,
): KetQuaDanhGia[] {
  ___
}

const bangSoSanh1 = soSanhNhieuCauHinh(
  [
    { ten: "khong-retry", soLanThuLaiToiDa: 0, dungCircuitBreaker: false },
    { ten: "co-retry", soLanThuLaiToiDa: 1, dungCircuitBreaker: false },
  ],
  taoToolLoiTamThoi,
  5,
);
console.log(JSON.stringify(bangSoSanh1));
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

function goiToolCoRetryCoGioiHan(tool: ToolMoPhong, soLanThuLaiToiDa: number): KetQuaGoiTool {
  let ketQua = tool.goi();
  let soLanDaThuLai = 0;
  while (!ketQua.thanhCong && soLanDaThuLai < soLanThuLaiToiDa) {
    ketQua = tool.goi();
    soLanDaThuLai++;
  }
  return ketQua;
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

function tinhTyLeHoanThanh(ketQua: KetQuaGoiTool[]): number {
  if (ketQua.length === 0) return 0;
  return ketQua.filter((k) => k.thanhCong).length / ketQua.length;
}

interface CauHinhHarness {
  ten: string;
  soLanThuLaiToiDa: number;
  dungCircuitBreaker: boolean;
}

interface KetQuaDanhGia {
  ten: string;
  tyLeHoanThanh: number;
  soLanGoiTrungBinh: number;
}

function taoNhaMayToolDungChung(taoTool: () => ToolMoPhong): () => ToolMoPhong {
  const toolDuyNhat = taoTool();
  return () => toolDuyNhat;
}

function chayMotCauHinh(
  cauHinh: CauHinhHarness,
  taoTool: () => ToolMoPhong,
  soTacVu: number,
): KetQuaDanhGia {
  const cb = taoCircuitBreaker(2, 2);
  const ketQua: KetQuaGoiTool[] = [];
  const congCuDaTao: ToolMoPhong[] = [];
  for (let i = 0; i < soTacVu; i++) {
    const tool = taoTool();
    if (!congCuDaTao.includes(tool)) congCuDaTao.push(tool);
    if (cauHinh.dungCircuitBreaker) {
      ketQua.push(cb.goi(tool));
    } else {
      ketQua.push(goiToolCoRetryCoGioiHan(tool, cauHinh.soLanThuLaiToiDa));
    }
  }
  const tongSoLanGoi = congCuDaTao.reduce((tong, t) => tong + t.trangThai.soLanDaGoi, 0);
  return {
    ten: cauHinh.ten,
    tyLeHoanThanh: tinhTyLeHoanThanh(ketQua),
    soLanGoiTrungBinh: tongSoLanGoi / soTacVu,
  };
}

function soSanhNhieuCauHinh(
  cacCauHinh: CauHinhHarness[],
  taoTool: () => ToolMoPhong,
  soTacVu: number,
): KetQuaDanhGia[] {
  return cacCauHinh.map((ch) => chayMotCauHinh(ch, taoTool, soTacVu));
}

const bangSoSanh1 = soSanhNhieuCauHinh(
  [
    { ten: "khong-retry", soLanThuLaiToiDa: 0, dungCircuitBreaker: false },
    { ten: "co-retry", soLanThuLaiToiDa: 1, dungCircuitBreaker: false },
  ],
  taoToolLoiTamThoi,
  5,
);
console.log(JSON.stringify(bangSoSanh1));
```

```typescript title=test
if (bangSoSanh1.length !== 2) throw new Error("soSanhNhieuCauHinh phai tra ve dung 2 phan tu (dung so cau hinh trong danh sach)");
if (bangSoSanh1[0]!.ten !== "khong-retry" || bangSoSanh1[0]!.tyLeHoanThanh !== 0 || bangSoSanh1[0]!.soLanGoiTrungBinh !== 1) {
  throw new Error("cau hinh khong-retry (soLanThuLaiToiDa=0) phai co tyLeHoanThanh=0 va soLanGoiTrungBinh=1");
}
if (bangSoSanh1[1]!.ten !== "co-retry" || bangSoSanh1[1]!.tyLeHoanThanh !== 1 || bangSoSanh1[1]!.soLanGoiTrungBinh !== 2) {
  throw new Error("cau hinh co-retry (soLanThuLaiToiDa=1) phai co tyLeHoanThanh=1 va soLanGoiTrungBinh=2");
}

const bangSoSanhMotCauHinh = soSanhNhieuCauHinh(
  [{ ten: "retry-3", soLanThuLaiToiDa: 3, dungCircuitBreaker: false }],
  taoToolLoiTamThoi,
  4,
);
if (bangSoSanhMotCauHinh.length !== 1) throw new Error("doi so luong cau hinh trong danh sach phai doi do dai mang tra ve");
if (bangSoSanhMotCauHinh[0]!.soLanGoiTrungBinh !== 2) {
  throw new Error("ngan sach retry=3 tren tool chi can 1 lan thu lai de thanh cong -- soLanGoiTrungBinh VAN phai la 2, khong lang phi them cuoc goi nao");
}

const ktKhongBreaker = chayMotCauHinh(
  { ten: "khong-breaker", soLanThuLaiToiDa: 0, dungCircuitBreaker: false },
  taoNhaMayToolDungChung(taoToolLuonThatBai),
  5,
);
const ktCoBreaker = chayMotCauHinh(
  { ten: "co-breaker", soLanThuLaiToiDa: 0, dungCircuitBreaker: true },
  taoNhaMayToolDungChung(taoToolLuonThatBai),
  5,
);
if (ktKhongBreaker.tyLeHoanThanh !== 0 || ktCoBreaker.tyLeHoanThanh !== 0) {
  throw new Error("tool hong VINH VIEN thi CA HAI cau hinh phai co tyLeHoanThanh 0 -- khong cau hinh nao cuu duoc mot loi he thong");
}
if (ktKhongBreaker.soLanGoiTrungBinh !== 1) throw new Error("khong dung breaker, moi tac vu goi tool THAT dung 1 lan (ngan sach retry 0)");
if (ktCoBreaker.soLanGoiTrungBinh >= ktKhongBreaker.soLanGoiTrungBinh) {
  throw new Error("dung circuit breaker tren mot su co he thong PHAI giam so lan goi tool THAT trung binh so voi khong dung breaker");
}
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (chayMotCauHinh): mot vong for chay soTacVu lan; moi lan tao tool qua taoTool(), them vao congCuDaTao NEU chua co; neu cauHinh.dungCircuitBreaker thi goi qua cb.goi(tool), nguoc lai goi qua goiToolCoRetryCoGioiHan(tool, cauHinh.soLanThuLaiToiDa); sau vong for, cong don soLanDaGoi cua MOI tool trong congCuDaTao roi chia soTacVu; tra ve object KetQuaDanhGia. Cho hai (soSanhNhieuCauHinh): dung .map tren cacCauHinh, moi phan tu goi chayMotCauHinh voi CUNG taoTool va soTacVu."
- kind: strategy
  body: "Cho dau: const cb = taoCircuitBreaker(2, 2); const ketQua: KetQuaGoiTool[] = []; const congCuDaTao: ToolMoPhong[] = []; for (let i = 0; i < soTacVu; i++) { const tool = taoTool(); if (!congCuDaTao.includes(tool)) congCuDaTao.push(tool); if (cauHinh.dungCircuitBreaker) { ketQua.push(cb.goi(tool)); } else { ketQua.push(goiToolCoRetryCoGioiHan(tool, cauHinh.soLanThuLaiToiDa)); } } const tongSoLanGoi = congCuDaTao.reduce((tong, t) => tong + t.trangThai.soLanDaGoi, 0); return { ten: cauHinh.ten, tyLeHoanThanh: tinhTyLeHoanThanh(ketQua), soLanGoiTrungBinh: tongSoLanGoi / soTacVu }; Cho hai: return cacCauHinh.map((ch) => chayMotCauHinh(ch, taoTool, soTacVu));"
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
  expect: "[{\"ten\":\"khong-retry\",\"tyLeHoanThanh\":0,\"soLanGoiTrungBinh\":1},{\"ten\":\"co-retry\",\"tyLeHoanThanh\":1,\"soLanGoiTrungBinh\":2}]"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một hàm, một danh sách cấu hình, một bảng kết quả — VÀ hai trục đo
(completion rate, chi phí mô phỏng) tách RIÊNG, không gộp thành một con
số duy nhất. Track NÀY còn đúng MỘT bài: ráp NGUYÊN VĂN mọi lớp phòng
thủ đã học — timeout, circuit breaker, validate, định tuyến, retry,
fallback — thành MỘT harness production-grade DUY NHẤT, đo trên một bộ
tác vụ HỖN HỢP.
::::

::::reflect{#nghi-lai}
Bài này không thêm một cơ chế phòng thủ MỚI nào — nó thêm một CÁCH ĐO,
VÀ đó LÀ một loại tiến bộ khác hẳn. Chín bài trước đều trả lời "cơ chế
NÀY hoạt động ra sao"; bài này trả lời "làm sao BIẾT cơ chế nào ĐÁNG
dùng, cho tình huống NÀO". Hai trục — `tyLeHoanThanh` VÀ
`soLanGoiTrungBinh` — không thể gộp thành MỘT con số mà không MẤT
thông tin: kịch bản breaker chứng minh điều đó bằng số cụ thể, hai cấu
hình BẰNG NHAU Ở một trục nhưng khác HẲN Ở trục kia. Đây chính LÀ lý do
MASTERPLAN định nghĩa HARNESS bằng "completion rate của CÙNG model khi
đổi harness" — không phải MỘT con số, mà LÀ một PHÉP SO SÁNH có kiểm
soát, VÀ `soSanhNhieuCauHinh` LÀ hình dạng nhỏ nhất của một hạ tầng
đánh giá làm được đúng phép so sánh đó.
::::

::::checkpoint{mastery=0.88}
::::
