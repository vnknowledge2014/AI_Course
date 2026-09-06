---
id: ky-nghe-ung-dung-ai.ky-thuat-harness.boss-harness-production-grade
title: "BOSS q9.3b — ráp NĂM lớp phòng thủ thành một harness production-grade"
summary: "ToolCoTre {trangThai; thuMotBuoc(): KetQuaGoiTool|null} LA giao dien CHUNG cho moi tool trong bai nay (ke ca tool 'treo'). xuLyTacVuDayDu rap NAM lop THEO DUNG THU TU: (1) validateDoiSo (bai 9) chan doi so sai TRUOC KHI cham tool; (2) dinh tuyen (bai 10) chon toolChinhCoTre + circuit breaker RIENG theo loaiTacVu; (3) goiToolCoTimeout (bai 7, qua boToolCoTreThanhToolMoPhong) chan treo; (4) goiToolCoRetryCoGioiHan (bai 4, qua ghepRetryVaoTool) cuu loi tam thoi; (5) circuit breaker (bai 8) boc NGOAI CUNG ca hai lop tren, nho loi LIEN TIEP xuyen suot NHIEU tac vu CUNG loai; that bai qua breaker moi FALLBACK (bai 3) sang toolDuPhongCoTre. Tren 8 tac vu hon hop (3 treo, 2 loi tam thoi -- 1 co doi so SAI, 3 loi vinh vien): harnessDonGian (goi thuMotBuoc dung 1 lan, khong lop nao) hoan thanh 0/8 = 0. harnessDayDu hoan thanh 7/8 = 0.875 -- CHI that bai o tac vu co doi so sai (validate chan, khong fallback). Dong track T9.3 tai 12/12: moi lop giai quyet MOT che do hong KHAC nhau (treo/tam thoi/vinh vien/sai doi so/lap lai nhieu tac vu), khong lop nao thay the duoc lop khac."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-harness
order: 12
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [kna.boss-harness-production-grade]
requires: [kna.so-sanh-cau-hinh-harness-qua-bang-danh-gia]
concepts: [kna.boss-harness-production-grade]
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
Sáu bài Ở q9.3b: timeout, circuit breaker, validate, định tuyến, VÀ một
cách đo có hệ thống. Mỗi bài giải quyết ĐÚNG một chế độ hỏng, TÁCH
RIÊNG khỏi mọi chế độ khác. BOSS này ráp CẢ NĂM lớp phòng thủ (bốn của
q9.3b, cộng retry/fallback của q9.3a) thành MỘT harness DUY NHẤT, VÀ đo
nó trên một bộ tác vụ HỖN HỢP — một số tool treo, một số lỗi tạm thời,
một số lỗi vĩnh viễn, một số có đối số SAI — để đóng track `T9.3` Ở
`12/12`.
::::

::::explain{#giao_dien_chung_va_keo_dan}
Để ráp năm lớp vào MỘT đường ống, mọi tool trong bài này dùng CHUNG một
giao diện — `ToolCoTre` (bài `7`) — VÀ hai hàm "keo dán" nhỏ biến nó
thành `ToolMoPhong` (giao diện `goi()` mà retry/breaker/fallback đã
quen dùng suốt track):

```typescript title=readonly
type KetQua<T, E> =
  | { thanhCong: true; giaTri: T }
  | { thanhCong: false; loi: E };

type KetQuaGoiTool = KetQua<string, string>;
type TrangThaiTool = { soLanDaGoi: number };

interface ToolCoTre {
  trangThai: TrangThaiTool;
  thuMotBuoc(): KetQuaGoiTool | null;
}

function goiToolCoTimeout(tool: ToolCoTre, gioiHanBuoc: number): KetQuaGoiTool {
  for (let buoc = 0; buoc < gioiHanBuoc; buoc++) {
    const ketQua = tool.thuMotBuoc();
    if (ketQua !== null) return ketQua;
  }
  return { thanhCong: false, loi: "loi_timeout" };
}

interface ToolMoPhong {
  trangThai: TrangThaiTool;
  goi(): KetQuaGoiTool;
}

// Lop TIMEOUT: bien mot ToolCoTre thanh mot ToolMoPhong binh thuong.
function boToolCoTreThanhToolMoPhong(tool: ToolCoTre, gioiHanBuoc: number): ToolMoPhong {
  return {
    trangThai: tool.trangThai,
    goi(): KetQuaGoiTool {
      return goiToolCoTimeout(tool, gioiHanBuoc);
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

// Lop RETRY: ghep ngan sach thu lai vao TRUOC mot ToolMoPhong.
function ghepRetryVaoTool(tool: ToolMoPhong, soLanThuLaiToiDa: number): ToolMoPhong {
  return {
    trangThai: tool.trangThai,
    goi(): KetQuaGoiTool {
      return goiToolCoRetryCoGioiHan(tool, soLanThuLaiToiDa);
    },
  };
}
```

`boToolCoTreThanhToolMoPhong` LÀ lớp TIMEOUT: một tool "treo" (bài
`7`, `thuMotBuoc()` LUÔN `null`) sẽ khiến `goiToolCoTimeout` chạy hết
`gioiHanBuoc` bước rồi bỏ cuộc; một tool bình thường (đáp ứng NGAY, dù
thành công hay thất bại) sẽ khiến vòng `for` dừng Ở BƯỚC ĐẦU TIÊN, bất
kể `gioiHanBuoc` LÀ bao nhiêu. `ghepRetryVaoTool` LÀ lớp RETRY: nó bọc
MỘT `ToolMoPhong` (Ở đây LUÔN LÀ một tool đã qua timeout) bằng
`goiToolCoRetryCoGioiHan` (bài `4`) — nếu tool bên trong hỏng TẠM THỜI,
retry cứu được; nếu nó TREO hoặc hỏng VĨNH VIỄN, retry chỉ tốn thêm
công (bị chặn LẠI Ở timeout mỗi lần), không cứu được.
::::

::::example{#so_sanh_don_gian_vs_day_du_tren_bo_tac_vu_hon_hop}
Một bộ `8` tác vụ HỖN HỢP: `3` tác vụ định tuyến tới một tool TREO
(`tra_cuu_don_hang`), `2` tác vụ tới một tool lỗi TẠM THỜI (`doi_tra`,
MỘT trong hai có đối số SAI), `3` tác vụ tới một tool lỗi VĨNH VIỄN
(`khuyen_mai`). `harnessDonGian` gọi `thuMotBuoc()` ĐÚNG một lần, không
lớp phòng thủ nào. `harnessDayDu` ráp NĂM lớp: validate → định tuyến
(MỖI loại tác vụ có circuit breaker RIÊNG) → timeout → retry → circuit
breaker → fallback:

```typescript title=readonly
// (đầy đủ 5 lớp + tool + bộ tác vụ hỗn hợp — xem khối `solution` ở bước
// ::::code bên dưới để đọc TOÀN VĂN hàm `xuLyTacVuDayDu` và `harnessDayDu`)

function goiMotLanKhongTimeout(tool: ToolCoTre): KetQuaGoiTool {
  const ketQua = tool.thuMotBuoc();
  if (ketQua === null) return { thanhCong: false, loi: "chua_tra_loi" };
  return ketQua;
}

// harnessDonGian: goi mot lan, khong validate/retry/timeout/breaker/fallback.
// harnessDayDu: rap ca nam lop (xem giai thich o tren + khoi solution).

const ketQuaDonGian = harnessDonGian(DANH_SACH_TAC_VU);
const ketQuaDayDu = harnessDayDu(DANH_SACH_TAC_VU);
console.log("don gian:", JSON.stringify(ketQuaDonGian.map((k) => k.thanhCong)), tinhTyLeHoanThanh(ketQuaDonGian));
console.log("day du:  ", JSON.stringify(ketQuaDayDu.map((k) => k.thanhCong)), tinhTyLeHoanThanh(ketQuaDayDu));
```

```text title=readonly
don gian: [false,false,false,false,false,false,false,false] 0
day du:   [true,true,true,true,false,true,true,true] 0.875
```

`harnessDonGian` thất bại TOÀN BỘ `8`/`8` — một `thuMotBuoc()` DUY
NHẤT không đủ cho BẤT KỲ chế độ hỏng nào Ở đây: tool treo trả về
`null` (không cứu), tool lỗi tạm thời LUÔN thất bại Ở LẦN GỌI ĐẦU
(không có retry để chờ lần thứ hai), tool lỗi vĩnh viễn thất bại y hệt
retry hay không. `harnessDayDu` hoàn thành `7`/`8`: BA tác vụ treo đều
THÀNH CÔNG (qua fallback, sau khi timeout+breaker chặn được sự lãng
phí Ở tác vụ THỨ BA — mạch đã mở); tác vụ lỗi tạm thời VỚI đối số ĐÚNG
THÀNH CÔNG bằng giá trị THẬT (retry cứu được); BA tác vụ lỗi vĩnh viễn
đều THÀNH CÔNG qua fallback. DUY NHẤT một tác vụ thất bại: tác vụ lỗi
tạm thời VỚI đối số SAI — validate chặn nó TRƯỚC KHI bất kỳ tool nào
được chạm tới, VÀ (đúng thiết kế bài `9`) validate không có fallback
— một đối số sai LÀ một yêu cầu không rõ ràng, không phải một tool
hỏng.
::::

::::predict{#doan-tac-vu-tre-thu-ba commitOnce}
Tác vụ THỨ BA định tuyến tới `tra_cuu_don_hang` (tool TREO) — VÀO lúc
đó, mạch (breaker) của tuyến NÀY đã `"mo"` (sau hai tác vụ treo LIÊN
TIẾP trước đó). Tool TREO mới được tạo cho tác vụ thứ ba này CÓ bị gọi
thật (qua `thuMotBuoc()`) hay không?

:::opt{correct}
Không — `cb.goi(toolChinh)` thấy mạch đang `"mo"` VÀ chưa đủ
`soLuotChoHoiPhuc`, nên TỰ nó trả lỗi NGAY (`"mach_da_mo_tu_choi_ngay"`),
KHÔNG bao giờ gọi `toolChinh.goi()` — nghĩa LÀ `goiToolCoTimeout` VÀ
`thuMotBuoc()` của tool treo mới đều không được chạm tới; tác vụ vẫn
THÀNH CÔNG nhưng HOÀN TOÀN qua fallback, không tốn một bước timeout
nào
:::
:::opt
Có — `goiToolCoTimeout` luôn thử gọi tool thật `gioiHanBuoc` bước
TRƯỚC KHI breaker được xét tới
::why
Nhầm THỨ TỰ các lớp — breaker BỌC NGOÀI `toolChinh` (đã gồm timeout
VÀ retry Ở BÊN TRONG nó), nên breaker LÀ lớp QUYẾT ĐỊNH trước tiên có
đi vào bên trong hay không.

Chỗ lệch: `xuLyTacVuDayDu` gọi `cb.goi(toolChinh)` — KHÔNG gọi
`toolChinh.goi()` trực tiếp. Khi mạch `"mo"` VÀ chưa đủ lượt chờ,
`CircuitBreaker.goi` (bài `8`) `return` NGAY Ở CHÍNH nó, dòng
`tool.goi()` (dẫn tới timeout) nằm SAU, không bao giờ được thực thi.
::
:::
:::opt
Có — mỗi tác vụ LUÔN cần một lần thử tool CHÍNH trước khi được PHÉP
fallback, bất kể trạng thái mạch
::why
Nhầm VỚI `goiToolCoFallback` (bài `3`, q9.3a) — Ở đó retry LUÔN thử
tool chính TRỰC TIẾP (không qua breaker) trước khi fallback. NHƯNG
harness Ở BÀI NÀY bọc tool chính QUA circuit breaker TRƯỚC, nên khi
mạch đã `"mo"`, CHÍNH breaker (không phải retry) LÀ nơi quyết định
việc từ chối — VÀ nó từ chối MÀ KHÔNG gọi tool chính lần nào.

Chỗ lệch: thứ tự ráp Ở BOSS này LÀ breaker BỌC NGOÀI (retry+timeout đã
gộp sẵn Ở trong `toolChinh`) — khác VỚI `goiToolCoFallback` gốc Ở bài
`3`, nơi KHÔNG hề có breaker tham gia.
::
:::
::::

::::code{#viet_harness_day_du}
Hoàn thiện `xuLyTacVuDayDu` — validate `tacVu.doiSoTho` TRƯỚC (trả lỗi
NGAY nếu không `"hop_le"`); nếu hợp lệ, lấy `toolChinhCoTre` VÀ `cb`
đúng theo `tacVu.loaiTacVu`, ghép `toolChinh` qua
`ghepRetryVaoTool(boToolCoTreThanhToolMoPhong(...), soLanThuLaiToiDa)`;
gọi `cb.goi(toolChinh)`, trả về NGAY nếu thành công; nếu KHÔNG, fallback
sang `toolDuPhongCoTre` (cũng qua `boToolCoTreThanhToolMoPhong`).
Hoàn thiện `harnessDayDu` — VỚI MỖI tác vụ trong `danhSachTacVu`: gọi
`xuLyTacVuDayDu` VỚI `boDinhTuyen`, `boBreaker`, `toolDuPhongCoTre`,
ngân sách timeout `2` VÀ ngân sách retry `1`; SAU MỖI tác vụ, thay tool
Ở `boDinhTuyen` cho ĐÚNG loại tác vụ đó bằng một tool MỚI (mô phỏng
request TIẾP THEO tới CÙNG dịch vụ) — GIỮ NGUYÊN breaker (nó phải nhớ
xuyên suốt nhiều tác vụ, không phải một).

```typescript title=starter
type KetQua<T, E> =
  | { thanhCong: true; giaTri: T }
  | { thanhCong: false; loi: E };

type KetQuaGoiTool = KetQua<string, string>;
type TrangThaiTool = { soLanDaGoi: number };

interface ToolCoTre {
  trangThai: TrangThaiTool;
  thuMotBuoc(): KetQuaGoiTool | null;
}

function taoToolTre(): ToolCoTre {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return {
    trangThai,
    thuMotBuoc(): KetQuaGoiTool | null {
      trangThai.soLanDaGoi++;
      return null;
    },
  };
}

function taoToolLoiTamThoi(): ToolCoTre {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return {
    trangThai,
    thuMotBuoc(): KetQuaGoiTool | null {
      trangThai.soLanDaGoi++;
      if (trangThai.soLanDaGoi === 1) {
        return { thanhCong: false, loi: "loi_tam_thoi_rate_limit" };
      }
      return { thanhCong: true, giaTri: "ket_qua_that" };
    },
  };
}

function taoToolLuonThatBai(): ToolCoTre {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return {
    trangThai,
    thuMotBuoc(): KetQuaGoiTool | null {
      trangThai.soLanDaGoi++;
      return { thanhCong: false, loi: "loi_vinh_vien" };
    },
  };
}

function taoToolDuPhong(): ToolCoTre {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return {
    trangThai,
    thuMotBuoc(): KetQuaGoiTool | null {
      trangThai.soLanDaGoi++;
      return { thanhCong: true, giaTri: "gia_tri_mac_dinh" };
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

interface ToolMoPhong {
  trangThai: TrangThaiTool;
  goi(): KetQuaGoiTool;
}

function boToolCoTreThanhToolMoPhong(tool: ToolCoTre, gioiHanBuoc: number): ToolMoPhong {
  return {
    trangThai: tool.trangThai,
    goi(): KetQuaGoiTool {
      return goiToolCoTimeout(tool, gioiHanBuoc);
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

function ghepRetryVaoTool(tool: ToolMoPhong, soLanThuLaiToiDa: number): ToolMoPhong {
  return {
    trangThai: tool.trangThai,
    goi(): KetQuaGoiTool {
      return goiToolCoRetryCoGioiHan(tool, soLanThuLaiToiDa);
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

interface DoiSoTacVu {
  maDonHang: string;
}

type KetQuaValidate =
  | { loai: "hop_le"; doiSo: DoiSoTacVu }
  | { loai: "thieu_khoa"; khoaThieu: string[] }
  | { loai: "sai_kieu"; khoa: string; kieuThuc: string };

function validateDoiSo(doiSoTho: unknown): KetQuaValidate {
  if (typeof doiSoTho !== "object" || doiSoTho === null) {
    return { loai: "thieu_khoa", khoaThieu: ["maDonHang"] };
  }
  const ghi = doiSoTho as Record<string, unknown>;
  if (!("maDonHang" in ghi)) {
    return { loai: "thieu_khoa", khoaThieu: ["maDonHang"] };
  }
  if (typeof ghi.maDonHang !== "string") {
    return { loai: "sai_kieu", khoa: "maDonHang", kieuThuc: typeof ghi.maDonHang };
  }
  return { loai: "hop_le", doiSo: { maDonHang: ghi.maDonHang } };
}

type LoaiTacVu = "tra_cuu_don_hang" | "doi_tra" | "khuyen_mai";

interface TacVu {
  loaiTacVu: LoaiTacVu;
  doiSoTho: unknown;
}

function goiMotLanKhongTimeout(tool: ToolCoTre): KetQuaGoiTool {
  const ketQua = tool.thuMotBuoc();
  if (ketQua === null) return { thanhCong: false, loi: "chua_tra_loi" };
  return ketQua;
}

const TAO_TOOL_THEO_LOAI: Record<LoaiTacVu, () => ToolCoTre> = {
  tra_cuu_don_hang: taoToolTre,
  doi_tra: taoToolLoiTamThoi,
  khuyen_mai: taoToolLuonThatBai,
};

function harnessDonGian(danhSachTacVu: TacVu[]): KetQuaGoiTool[] {
  const boDinhTuyen: Record<LoaiTacVu, ToolCoTre> = {
    tra_cuu_don_hang: taoToolTre(),
    doi_tra: taoToolLoiTamThoi(),
    khuyen_mai: taoToolLuonThatBai(),
  };
  return danhSachTacVu.map((tv) => {
    const ketQua = goiMotLanKhongTimeout(boDinhTuyen[tv.loaiTacVu]);
    boDinhTuyen[tv.loaiTacVu] = TAO_TOOL_THEO_LOAI[tv.loaiTacVu]();
    return ketQua;
  });
}

function tinhTyLeHoanThanh(ketQua: KetQuaGoiTool[]): number {
  if (ketQua.length === 0) return 0;
  return ketQua.filter((k) => k.thanhCong).length / ketQua.length;
}

function xuLyTacVuDayDu(
  tacVu: TacVu,
  boDinhTuyen: Record<LoaiTacVu, ToolCoTre>,
  boBreaker: Record<LoaiTacVu, CircuitBreaker>,
  toolDuPhongCoTre: ToolCoTre,
  gioiHanBuocTimeout: number,
  soLanThuLaiToiDa: number,
): KetQuaGoiTool {
  ___
}

function harnessDayDu(danhSachTacVu: TacVu[]): KetQuaGoiTool[] {
  ___
}

const DANH_SACH_TAC_VU: TacVu[] = [
  { loaiTacVu: "tra_cuu_don_hang", doiSoTho: { maDonHang: "A1" } },
  { loaiTacVu: "tra_cuu_don_hang", doiSoTho: { maDonHang: "A2" } },
  { loaiTacVu: "tra_cuu_don_hang", doiSoTho: { maDonHang: "A3" } },
  { loaiTacVu: "doi_tra", doiSoTho: { maDonHang: "B1" } },
  { loaiTacVu: "doi_tra", doiSoTho: { ma: "B2" } },
  { loaiTacVu: "khuyen_mai", doiSoTho: { maDonHang: "C1" } },
  { loaiTacVu: "khuyen_mai", doiSoTho: { maDonHang: "C2" } },
  { loaiTacVu: "khuyen_mai", doiSoTho: { maDonHang: "C3" } },
];

const ketQuaDonGian = harnessDonGian(DANH_SACH_TAC_VU);
const ketQuaDayDu = harnessDayDu(DANH_SACH_TAC_VU);
console.log(JSON.stringify(ketQuaDayDu.map((k) => k.thanhCong)), tinhTyLeHoanThanh(ketQuaDonGian), tinhTyLeHoanThanh(ketQuaDayDu));
```

```typescript title=solution
type KetQua<T, E> =
  | { thanhCong: true; giaTri: T }
  | { thanhCong: false; loi: E };

type KetQuaGoiTool = KetQua<string, string>;
type TrangThaiTool = { soLanDaGoi: number };

interface ToolCoTre {
  trangThai: TrangThaiTool;
  thuMotBuoc(): KetQuaGoiTool | null;
}

function taoToolTre(): ToolCoTre {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return {
    trangThai,
    thuMotBuoc(): KetQuaGoiTool | null {
      trangThai.soLanDaGoi++;
      return null;
    },
  };
}

function taoToolLoiTamThoi(): ToolCoTre {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return {
    trangThai,
    thuMotBuoc(): KetQuaGoiTool | null {
      trangThai.soLanDaGoi++;
      if (trangThai.soLanDaGoi === 1) {
        return { thanhCong: false, loi: "loi_tam_thoi_rate_limit" };
      }
      return { thanhCong: true, giaTri: "ket_qua_that" };
    },
  };
}

function taoToolLuonThatBai(): ToolCoTre {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return {
    trangThai,
    thuMotBuoc(): KetQuaGoiTool | null {
      trangThai.soLanDaGoi++;
      return { thanhCong: false, loi: "loi_vinh_vien" };
    },
  };
}

function taoToolDuPhong(): ToolCoTre {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return {
    trangThai,
    thuMotBuoc(): KetQuaGoiTool | null {
      trangThai.soLanDaGoi++;
      return { thanhCong: true, giaTri: "gia_tri_mac_dinh" };
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

interface ToolMoPhong {
  trangThai: TrangThaiTool;
  goi(): KetQuaGoiTool;
}

function boToolCoTreThanhToolMoPhong(tool: ToolCoTre, gioiHanBuoc: number): ToolMoPhong {
  return {
    trangThai: tool.trangThai,
    goi(): KetQuaGoiTool {
      return goiToolCoTimeout(tool, gioiHanBuoc);
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

function ghepRetryVaoTool(tool: ToolMoPhong, soLanThuLaiToiDa: number): ToolMoPhong {
  return {
    trangThai: tool.trangThai,
    goi(): KetQuaGoiTool {
      return goiToolCoRetryCoGioiHan(tool, soLanThuLaiToiDa);
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

interface DoiSoTacVu {
  maDonHang: string;
}

type KetQuaValidate =
  | { loai: "hop_le"; doiSo: DoiSoTacVu }
  | { loai: "thieu_khoa"; khoaThieu: string[] }
  | { loai: "sai_kieu"; khoa: string; kieuThuc: string };

function validateDoiSo(doiSoTho: unknown): KetQuaValidate {
  if (typeof doiSoTho !== "object" || doiSoTho === null) {
    return { loai: "thieu_khoa", khoaThieu: ["maDonHang"] };
  }
  const ghi = doiSoTho as Record<string, unknown>;
  if (!("maDonHang" in ghi)) {
    return { loai: "thieu_khoa", khoaThieu: ["maDonHang"] };
  }
  if (typeof ghi.maDonHang !== "string") {
    return { loai: "sai_kieu", khoa: "maDonHang", kieuThuc: typeof ghi.maDonHang };
  }
  return { loai: "hop_le", doiSo: { maDonHang: ghi.maDonHang } };
}

type LoaiTacVu = "tra_cuu_don_hang" | "doi_tra" | "khuyen_mai";

interface TacVu {
  loaiTacVu: LoaiTacVu;
  doiSoTho: unknown;
}

function goiMotLanKhongTimeout(tool: ToolCoTre): KetQuaGoiTool {
  const ketQua = tool.thuMotBuoc();
  if (ketQua === null) return { thanhCong: false, loi: "chua_tra_loi" };
  return ketQua;
}

const TAO_TOOL_THEO_LOAI: Record<LoaiTacVu, () => ToolCoTre> = {
  tra_cuu_don_hang: taoToolTre,
  doi_tra: taoToolLoiTamThoi,
  khuyen_mai: taoToolLuonThatBai,
};

function harnessDonGian(danhSachTacVu: TacVu[]): KetQuaGoiTool[] {
  const boDinhTuyen: Record<LoaiTacVu, ToolCoTre> = {
    tra_cuu_don_hang: taoToolTre(),
    doi_tra: taoToolLoiTamThoi(),
    khuyen_mai: taoToolLuonThatBai(),
  };
  return danhSachTacVu.map((tv) => {
    const ketQua = goiMotLanKhongTimeout(boDinhTuyen[tv.loaiTacVu]);
    boDinhTuyen[tv.loaiTacVu] = TAO_TOOL_THEO_LOAI[tv.loaiTacVu]();
    return ketQua;
  });
}

function tinhTyLeHoanThanh(ketQua: KetQuaGoiTool[]): number {
  if (ketQua.length === 0) return 0;
  return ketQua.filter((k) => k.thanhCong).length / ketQua.length;
}

function xuLyTacVuDayDu(
  tacVu: TacVu,
  boDinhTuyen: Record<LoaiTacVu, ToolCoTre>,
  boBreaker: Record<LoaiTacVu, CircuitBreaker>,
  toolDuPhongCoTre: ToolCoTre,
  gioiHanBuocTimeout: number,
  soLanThuLaiToiDa: number,
): KetQuaGoiTool {
  const kqValidate = validateDoiSo(tacVu.doiSoTho);
  if (kqValidate.loai !== "hop_le") {
    return { thanhCong: false, loi: `loi_validate_${kqValidate.loai}` };
  }
  const toolChinhCoTre = boDinhTuyen[tacVu.loaiTacVu];
  const cb = boBreaker[tacVu.loaiTacVu];
  const toolChinh = ghepRetryVaoTool(
    boToolCoTreThanhToolMoPhong(toolChinhCoTre, gioiHanBuocTimeout),
    soLanThuLaiToiDa,
  );
  const ketQuaQuaBreaker = cb.goi(toolChinh);
  if (ketQuaQuaBreaker.thanhCong) return ketQuaQuaBreaker;
  const toolDuPhong = boToolCoTreThanhToolMoPhong(toolDuPhongCoTre, gioiHanBuocTimeout);
  return toolDuPhong.goi();
}

function harnessDayDu(danhSachTacVu: TacVu[]): KetQuaGoiTool[] {
  const GIOI_HAN_BUOC_TIMEOUT = 2;
  const SO_LAN_THU_LAI_TOI_DA = 1;
  const boDinhTuyen: Record<LoaiTacVu, ToolCoTre> = {
    tra_cuu_don_hang: taoToolTre(),
    doi_tra: taoToolLoiTamThoi(),
    khuyen_mai: taoToolLuonThatBai(),
  };
  const boBreaker: Record<LoaiTacVu, CircuitBreaker> = {
    tra_cuu_don_hang: taoCircuitBreaker(2, 2),
    doi_tra: taoCircuitBreaker(2, 2),
    khuyen_mai: taoCircuitBreaker(2, 2),
  };
  const toolDuPhongCoTre = taoToolDuPhong();
  return danhSachTacVu.map((tv) => {
    const ketQua = xuLyTacVuDayDu(tv, boDinhTuyen, boBreaker, toolDuPhongCoTre, GIOI_HAN_BUOC_TIMEOUT, SO_LAN_THU_LAI_TOI_DA);
    boDinhTuyen[tv.loaiTacVu] = TAO_TOOL_THEO_LOAI[tv.loaiTacVu]();
    return ketQua;
  });
}

const DANH_SACH_TAC_VU: TacVu[] = [
  { loaiTacVu: "tra_cuu_don_hang", doiSoTho: { maDonHang: "A1" } },
  { loaiTacVu: "tra_cuu_don_hang", doiSoTho: { maDonHang: "A2" } },
  { loaiTacVu: "tra_cuu_don_hang", doiSoTho: { maDonHang: "A3" } },
  { loaiTacVu: "doi_tra", doiSoTho: { maDonHang: "B1" } },
  { loaiTacVu: "doi_tra", doiSoTho: { ma: "B2" } },
  { loaiTacVu: "khuyen_mai", doiSoTho: { maDonHang: "C1" } },
  { loaiTacVu: "khuyen_mai", doiSoTho: { maDonHang: "C2" } },
  { loaiTacVu: "khuyen_mai", doiSoTho: { maDonHang: "C3" } },
];

const ketQuaDonGian = harnessDonGian(DANH_SACH_TAC_VU);
const ketQuaDayDu = harnessDayDu(DANH_SACH_TAC_VU);
console.log(JSON.stringify(ketQuaDayDu.map((k) => k.thanhCong)), tinhTyLeHoanThanh(ketQuaDonGian), tinhTyLeHoanThanh(ketQuaDayDu));
```

```typescript title=test
if (ketQuaDayDu.length !== 8) throw new Error("harnessDayDu phai tra ve mang dung 8 phan tu (dung so tac vu trong danh sach)");
if (JSON.stringify(ketQuaDayDu.map((k) => k.thanhCong)) !== JSON.stringify([true, true, true, true, false, true, true, true])) {
  throw new Error("ket qua harnessDayDu phai la [true,true,true,true,false,true,true,true] -- chi tac vu doi so SAI (vi tri thu 5) that bai");
}
if (Math.abs(tinhTyLeHoanThanh(ketQuaDayDu) - 0.875) > 1e-9) throw new Error("completion rate cua harnessDayDu tren 8 tac vu hon hop phai la 0.875 (7/8)");
if (tinhTyLeHoanThanh(ketQuaDonGian) !== 0) throw new Error("harnessDonGian (khong lop phong thu nao) phai hoan thanh 0/8 tren CHINH bo tac vu nay");

if (ketQuaDayDu[4]!.thanhCong !== false) throw new Error("tac vu thu 5 (doi so sai, thieu maDonHang) phai THAT BAI qua validate");
if (ketQuaDayDu[4]!.thanhCong === false && ketQuaDayDu[4]!.loi !== "loi_validate_thieu_khoa") {
  throw new Error("loi cua tac vu doi so sai phai la loi_validate_thieu_khoa, khong duoc di qua fallback");
}

if (ketQuaDayDu[3]!.thanhCong !== true || (ketQuaDayDu[3]!.thanhCong && ketQuaDayDu[3]!.giaTri !== "ket_qua_that")) {
  throw new Error("tac vu doi_tra co doi so DUNG (vi tri thu 4) phai THANH CONG bang gia tri THAT qua retry, khong phai fallback");
}

for (const i of [0, 1, 2, 5, 6, 7]) {
  const k = ketQuaDayDu[i]!;
  if (!k.thanhCong || k.giaTri !== "gia_tri_mac_dinh") {
    throw new Error(`tac vu vi tri ${i} (tool treo hoac loi vinh vien) phai THANH CONG qua FALLBACK, gia tri gia_tri_mac_dinh`);
  }
}

const mot = harnessDayDu([{ loaiTacVu: "tra_cuu_don_hang", doiSoTho: { maDonHang: "X" } }]);
if (mot.length !== 1) throw new Error("doi do dai danhSachTacVu phai doi do dai mang tra ve -- tham so phai duoc dung that");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (xuLyTacVuDayDu): validate truoc, tra loi loi_validate_<loai> ngay neu KHONG hop_le; lay toolChinhCoTre va cb dung loaiTacVu; ghep toolChinh = ghepRetryVaoTool(boToolCoTreThanhToolMoPhong(toolChinhCoTre, gioiHanBuocTimeout), soLanThuLaiToiDa) (dung DUNG hai tham so gioiHanBuocTimeout/soLanThuLaiToiDa cua CHINH ham xuLyTacVuDayDu); goi cb.goi(toolChinh), return NGAY neu thanh cong; nguoc lai fallback qua boToolCoTreThanhToolMoPhong(toolDuPhongCoTre, gioiHanBuocTimeout).goi(). Cho hai (harnessDayDu): khai bao hai hang so cuc bo (vi du GIOI_HAN_BUOC_TIMEOUT = 2, SO_LAN_THU_LAI_TOI_DA = 1) roi dung .map tren danhSachTacVu, moi phan tu goi xuLyTacVuDayDu voi day du sau tham so (boDinhTuyen, boBreaker, toolDuPhongCoTre, hai hang so vua khai bao), SAU DO thay tool o boDinhTuyen cho DUNG loai tac vu do bang mot tool MOI (GIU NGUYEN boBreaker)."
- kind: strategy
  body: "Cho dau: const kqValidate = validateDoiSo(tacVu.doiSoTho); if (kqValidate.loai !== \"hop_le\") { return { thanhCong: false, loi: `loi_validate_${kqValidate.loai}` }; } const toolChinhCoTre = boDinhTuyen[tacVu.loaiTacVu]; const cb = boBreaker[tacVu.loaiTacVu]; const toolChinh = ghepRetryVaoTool(boToolCoTreThanhToolMoPhong(toolChinhCoTre, gioiHanBuocTimeout), soLanThuLaiToiDa); const ketQuaQuaBreaker = cb.goi(toolChinh); if (ketQuaQuaBreaker.thanhCong) return ketQuaQuaBreaker; const toolDuPhong = boToolCoTreThanhToolMoPhong(toolDuPhongCoTre, gioiHanBuocTimeout); return toolDuPhong.goi(); Cho hai: const GIOI_HAN_BUOC_TIMEOUT = 2; const SO_LAN_THU_LAI_TOI_DA = 1; return danhSachTacVu.map((tv) => { const ketQua = xuLyTacVuDayDu(tv, boDinhTuyen, boBreaker, toolDuPhongCoTre, GIOI_HAN_BUOC_TIMEOUT, SO_LAN_THU_LAI_TOI_DA); boDinhTuyen[tv.loaiTacVu] = TAO_TOOL_THEO_LOAI[tv.loaiTacVu](); return ketQua; }); (voi boDinhTuyen/boBreaker/toolDuPhongCoTre da dung trong than ham nhu khoi solution)."
- kind: one-line
  body: "Sao chep dung hai khoi o phan Strategy vao dung vi tri tuong ung, dung goi tri boDinhTuyen/boBreaker/toolDuPhongCoTre da khoi tao dau ham harnessDayDu."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 10000
- tier: output
  match: contains
  expect: "[true,true,true,true,false,true,true,true] 0 0.875"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`0` so VỚI `0.875` — trên CÙNG một bộ `8` tác vụ, chỉ đổi chương trình
bao quanh model. Track `T9.3` "Kỹ thuật Harness" đóng lại Ở `12/12`:
`q9.3a` dựng nền (tool mô phỏng, retry, fallback, ngân sách, ROP);
`q9.3b` mở rộng SÂU hơn (timeout, circuit breaker, validate, định
tuyến, đo lường có hệ thống) VÀ ráp TẤT CẢ lại thành một BOSS DUY NHẤT.
::::

::::reflect{#nghi-lai}
Con số `0.875` không đến từ MỘT lớp phòng thủ nào LÀM VIỆC "giỏi hơn"
những lớp khác — nó đến từ việc NĂM lớp CÙNG có mặt, MỖI lớp cứu đúng
MỘT chế độ hỏng mà bốn lớp còn lại KHÔNG chạm tới: timeout cứu tool
TREO (biến "chờ vô hạn" thành "bỏ cuộc có kiểm soát"); retry cứu lỗi
TẠM THỜI (tác vụ `doi_tra` VỚI đối số đúng); circuit breaker không cứu
completion rate NÀO cả — nó chỉ ngăn LÃNG PHÍ khi MỘT tuyến ĐÃ biết
hỏng lặp lại (tác vụ thứ BA của `tra_cuu_don_hang`); fallback cứu
completion rate cho MỌI trường hợp còn lại, kể cả khi cả bốn lớp kia
đã bó tay (tool treo VĨNH VIỄN, tool hỏng VĨNH VIỄN); VÀ validate LÀ
lớp DUY NHẤT từ chối cứu — bởi vì đối số sai không phải LÀ MỘT tool
hỏng, mà LÀ một YÊU CẦU không rõ ràng, VÀ không lớp phòng thủ nào phía
sau nó có thể ĐOÁN đúng ý định thay cho một dữ liệu không hợp lệ. Đây
chính LÀ câu trả lời cho câu hỏi mở đầu `T9.3`: harness không phải LÀ
MỘT cơ chế, mà LÀ một TẬP HỢP CÓ CHỌN LỌC nhiều cơ chế, mỗi cơ chế
đứng gác cho đúng MỘT cách một hệ thống có thể hỏng — VÀ completion
rate của "model+prompt+context đều tốt" phụ thuộc HOÀN TOÀN vào việc
tập hợp đó có ĐẦY ĐỦ hay không, không phụ thuộc model chút nào.
::::

::::checkpoint{mastery=0.93}
::::
