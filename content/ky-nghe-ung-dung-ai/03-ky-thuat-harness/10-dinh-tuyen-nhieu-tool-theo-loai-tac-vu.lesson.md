---
id: ky-nghe-ung-dung-ai.ky-thuat-harness.dinh-tuyen-nhieu-tool-theo-loai-tac-vu
title: "Định tuyến — chọn ĐÚNG tool trong nhiều tool theo loại tác vụ"
summary: "LoaiTacVu = 'tra_cuu_don_hang' | 'doi_tra' | 'khuyen_mai' -- Record<LoaiTacVu, ToolMoPhong> LA bang dinh tuyen: moi loai tac vu ANH XA toi DUNG MOT tool xu ly no. goiToolTheoLoaiTacVu(loaiTacVu, boDinhTuyen, toolDuPhong) TRA CUU tool DUOC CHON qua boDinhTuyen[loaiTacVu], roi giao no cho goiToolCoFallback (TAI SU DUNG NGUYEN VAN tu bai 3/BOSS q9.3a, khong dinh nghia lai) -- fallback dung CHUNG cho MOI tuyen, khong rieng tung loai. chayNhieuTacVuTheoLoai(danhSachLoaiTacVu, boDinhTuyen, toolDuPhong) chay tren MOT danh sach loai tac vu (khong phai mot so dem N nhu cac bai truoc). Voi ba tuyen {tra_cuu_don_hang: taoToolLoiTamThoi, doi_tra: taoToolLuonThatBai, khuyen_mai: taoToolLuonThanhCong}: CA BA deu thanh cong nhung qua BA con duong khac nhau -- gia tri tra ve la ['ket_qua_that','gia_tri_mac_dinh','luon_ok'], VA toolDuPhong (dung chung) chi bi goi DUNG 1 lan (cho doi_tra) -- hai tuyen con lai khong can toi fallback."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-harness
order: 10
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [kna.dinh-tuyen-nhieu-tool-theo-loai-tac-vu]
requires: [kna.validate-doi-so-truoc-khi-thuc-thi]
concepts: [kna.dinh-tuyen-nhieu-tool-theo-loai-tac-vu]
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
Chín bài đầu quest NÀY luôn xử lý ĐÚNG MỘT tool, hoặc MỘT tool chính
cộng MỘT tool dự phòng cố định. Nhưng một harness thật thường phải
phục vụ NHIỀU loại tác vụ khác nhau — tra cứu đơn hàng, đổi trả, hỏi
khuyến mãi — VÀ mỗi loại có thể cần một tool KHÁC nhau. Bài này thêm
lớp quyết định ĐẦU TIÊN của một request: KHÔNG PHẢI "tool này có ổn
không", mà LÀ "tác vụ NÀY nên đi tới tool NÀO".
::::

::::explain{#bang_dinh_tuyen}
`LoaiTacVu` LÀ một union CHỈ có ba giá trị hợp lệ. `Record<LoaiTacVu,
ToolMoPhong>` LÀ BẢNG ĐỊNH TUYẾN — TypeScript BẮT BUỘC object đó phải
có ĐỦ CẢ BA khoá (thiếu MỘT khoá LÀ lỗi biên dịch, không phải lỗi
runtime). `goiToolTheoLoaiTacVu` tra cứu tool ĐÚNG cho loại tác vụ đó,
rồi giao thẳng cho `goiToolCoFallback` — TÁI SỬ DỤNG NGUYÊN VĂN hàm đã
viết Ở bài `3`/BOSS q9.3a, không viết lại logic fallback từ đầu:

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

type LoaiTacVu = "tra_cuu_don_hang" | "doi_tra" | "khuyen_mai";

function goiToolTheoLoaiTacVu(
  loaiTacVu: LoaiTacVu,
  boDinhTuyen: Record<LoaiTacVu, ToolMoPhong>,
  toolDuPhong: ToolMoPhong,
): KetQuaGoiTool {
  const toolDuocChon = boDinhTuyen[loaiTacVu];
  return goiToolCoFallback(toolDuocChon, toolDuPhong);
}
```

`goiToolTheoLoaiTacVu` KHÔNG chứa bất kỳ logic retry hay fallback MỚI
nào — nó chỉ LÀM một việc DUY NHẤT: TRA CỨU đúng tool cho đúng loại
tác vụ, rồi giao lại cho cơ chế phòng thủ ĐÃ CÓ. Đây LÀ điểm quan
trọng: định tuyến LÀ một lớp TÁCH BIỆT, đứng TRƯỚC mọi lớp phòng thủ
khác, không THAY THẾ chúng.
::::

::::example{#ba_tuyen_ba_ket_qua_khac_nhau}
Ba loại tác vụ, ba tool khác hẳn nhau (đã học Ở q9.3a), CÙNG một
`toolDuPhong` dùng chung:

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

type LoaiTacVu = "tra_cuu_don_hang" | "doi_tra" | "khuyen_mai";

function goiToolTheoLoaiTacVu(
  loaiTacVu: LoaiTacVu,
  boDinhTuyen: Record<LoaiTacVu, ToolMoPhong>,
  toolDuPhong: ToolMoPhong,
): KetQuaGoiTool {
  const toolDuocChon = boDinhTuyen[loaiTacVu];
  return goiToolCoFallback(toolDuocChon, toolDuPhong);
}

function chayNhieuTacVuTheoLoai(
  danhSachLoaiTacVu: LoaiTacVu[],
  boDinhTuyen: Record<LoaiTacVu, ToolMoPhong>,
  toolDuPhong: ToolMoPhong,
): KetQuaGoiTool[] {
  const ketQua: KetQuaGoiTool[] = [];
  for (const loai of danhSachLoaiTacVu) {
    ketQua.push(goiToolTheoLoaiTacVu(loai, boDinhTuyen, toolDuPhong));
  }
  return ketQua;
}

const boDinhTuyen: Record<LoaiTacVu, ToolMoPhong> = {
  tra_cuu_don_hang: taoToolLoiTamThoi(),
  doi_tra: taoToolLuonThatBai(),
  khuyen_mai: taoToolLuonThanhCong(),
};
const toolDuPhong = taoToolDuPhong();

const ketQua = chayNhieuTacVuTheoLoai(
  ["tra_cuu_don_hang", "doi_tra", "khuyen_mai"],
  boDinhTuyen,
  toolDuPhong,
);
console.log(JSON.stringify(ketQua.map((k) => k.thanhCong)));
console.log(JSON.stringify(ketQua.map((k) => (k.thanhCong ? k.giaTri : null))));
console.log("so lan goi toolDuPhong:", toolDuPhong.trangThai.soLanDaGoi);
```

```text title=readonly
[true,true,true]
["ket_qua_that","gia_tri_mac_dinh","luon_ok"]
so lan goi toolDuPhong: 1
```

CẢ BA tác vụ đều thành công — nhưng qua BA con đường HOÀN TOÀN khác
nhau: `tra_cuu_don_hang` tự phục hồi qua retry (giá trị `"ket_qua_that"`
— tool THẬT); `doi_tra` cần TỚI fallback (giá trị `"gia_tri_mac_dinh"`
— tool DỰ PHÒNG); `khuyen_mai` thành công NGAY, không cần retry lẫn
fallback (giá trị `"luon_ok"`). `toolDuPhong` DÙNG CHUNG cho cả ba
tuyến, nhưng chỉ bị gọi ĐÚNG `1` lần — CHỈ khi tuyến `doi_tra` thật sự
cần tới nó.
::::

::::predict{#doan-khuyen-mai-co-fallback-khong commitOnce}
Với tuyến `"khuyen_mai"` (định tuyến tới `taoToolLuonThanhCong()`,
LUÔN thành công NGAY lần gọi đầu) — `toolDuPhong` CÓ được gọi cho
tuyến NÀY không?

:::opt{correct}
Không — `goiToolCoFallback` chỉ gọi `toolDuPhong.goi()` khi
`goiToolCoRetryCoGioiHan(toolDuocChon, 1)` thất bại; `taoToolLuonThanhCong`
LUÔN thành công ngay Ở lần gọi đầu tiên, nên nhánh `return ketQuaChinh;`
chạy NGAY, dòng gọi `toolDuPhong.goi()` không bao giờ được thực thi
cho tuyến này
:::
:::opt
Có — mỗi tuyến nên gọi CẢ tool chính LẪN tool dự phòng để SO SÁNH kết
quả trước khi quyết định trả về cái nào
::why
Nhầm VỚI chiến lược "gọi song song rồi so sánh" — nhưng
`goiToolCoFallback` (tái sử dụng NGUYÊN VĂN từ bài `3`) dùng `if
(ketQuaChinh.thanhCong) return ketQuaChinh;` — một `return` SỚM, không
phải một phép so sánh giữa hai kết quả.

Chỗ lệch: khi tool chính đã thành công, hàm KẾT THÚC ngay Ở dòng đó —
dòng gọi `toolDuPhong.goi()` nằm SAU, không bao giờ được chạy tới
trong trường hợp này.
::
:::
:::opt
Không xác định được — phụ thuộc THỨ TỰ các khoá được khai trong
`boDinhTuyen`
::why
Nhầm rằng THỨ TỰ khai báo thuộc tính trong object ảnh hưởng tới hành
vi routing — nhưng `boDinhTuyen[loaiTacVu]` LÀ một phép TRA CỨU trực
tiếp theo TÊN khoá (`loaiTacVu`), không phải một phép DUYỆT tuần tự
qua các thuộc tính.

Chỗ lệch: dù `boDinhTuyen` được khai VỚI thứ tự khoá LÀ gì, tra cứu
`boDinhTuyen["khuyen_mai"]` LUÔN trả về ĐÚNG tool gán cho khoá đó,
không phụ thuộc vị trí khai báo.
::
:::
::::

::::code{#viet_dinh_tuyen}
Hoàn thiện `goiToolTheoLoaiTacVu` — tra cứu tool ĐÚNG cho `loaiTacVu`
qua `boDinhTuyen`, rồi giao cho `goiToolCoFallback` VỚI `toolDuPhong`.
Hoàn thiện `chayNhieuTacVuTheoLoai` — VỚI MỖI loại tác vụ trong
`danhSachLoaiTacVu` (một MẢNG, không phải một số đếm): gọi
`goiToolTheoLoaiTacVu`, đẩy kết quả vào mảng trả về.

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

type LoaiTacVu = "tra_cuu_don_hang" | "doi_tra" | "khuyen_mai";

function goiToolTheoLoaiTacVu(
  loaiTacVu: LoaiTacVu,
  boDinhTuyen: Record<LoaiTacVu, ToolMoPhong>,
  toolDuPhong: ToolMoPhong,
): KetQuaGoiTool {
  ___
}

function chayNhieuTacVuTheoLoai(
  danhSachLoaiTacVu: LoaiTacVu[],
  boDinhTuyen: Record<LoaiTacVu, ToolMoPhong>,
  toolDuPhong: ToolMoPhong,
): KetQuaGoiTool[] {
  ___
}

const boDinhTuyen: Record<LoaiTacVu, ToolMoPhong> = {
  tra_cuu_don_hang: taoToolLoiTamThoi(),
  doi_tra: taoToolLuonThatBai(),
  khuyen_mai: taoToolLuonThanhCong(),
};
const toolDuPhong = taoToolDuPhong();

const ketQua = chayNhieuTacVuTheoLoai(
  ["tra_cuu_don_hang", "doi_tra", "khuyen_mai"],
  boDinhTuyen,
  toolDuPhong,
);
console.log(JSON.stringify(ketQua.map((k) => k.thanhCong)), JSON.stringify(ketQua.map((k) => (k.thanhCong ? k.giaTri : null))));
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

type LoaiTacVu = "tra_cuu_don_hang" | "doi_tra" | "khuyen_mai";

function goiToolTheoLoaiTacVu(
  loaiTacVu: LoaiTacVu,
  boDinhTuyen: Record<LoaiTacVu, ToolMoPhong>,
  toolDuPhong: ToolMoPhong,
): KetQuaGoiTool {
  const toolDuocChon = boDinhTuyen[loaiTacVu];
  return goiToolCoFallback(toolDuocChon, toolDuPhong);
}

function chayNhieuTacVuTheoLoai(
  danhSachLoaiTacVu: LoaiTacVu[],
  boDinhTuyen: Record<LoaiTacVu, ToolMoPhong>,
  toolDuPhong: ToolMoPhong,
): KetQuaGoiTool[] {
  const ketQua: KetQuaGoiTool[] = [];
  for (const loai of danhSachLoaiTacVu) {
    ketQua.push(goiToolTheoLoaiTacVu(loai, boDinhTuyen, toolDuPhong));
  }
  return ketQua;
}

const boDinhTuyen: Record<LoaiTacVu, ToolMoPhong> = {
  tra_cuu_don_hang: taoToolLoiTamThoi(),
  doi_tra: taoToolLuonThatBai(),
  khuyen_mai: taoToolLuonThanhCong(),
};
const toolDuPhong = taoToolDuPhong();

const ketQua = chayNhieuTacVuTheoLoai(
  ["tra_cuu_don_hang", "doi_tra", "khuyen_mai"],
  boDinhTuyen,
  toolDuPhong,
);
console.log(JSON.stringify(ketQua.map((k) => k.thanhCong)), JSON.stringify(ketQua.map((k) => (k.thanhCong ? k.giaTri : null))));
```

```typescript title=test
if (ketQua.length !== 3) throw new Error("chayNhieuTacVuTheoLoai phai tra ve mang dung 3 phan tu (dung so luong loai tac vu trong danh sach)");
if (JSON.stringify(ketQua.map((k) => k.thanhCong)) !== JSON.stringify([true, true, true])) {
  throw new Error("ca ba tac vu deu phai THANH CONG (tra_cuu_don_hang tu phuc hoi qua retry, doi_tra can fallback, khuyen_mai thanh cong ngay)");
}
if (JSON.stringify(ketQua.map((k) => (k.thanhCong ? k.giaTri : null))) !== JSON.stringify(["ket_qua_that", "gia_tri_mac_dinh", "luon_ok"])) {
  throw new Error("gia tri phai la ['ket_qua_that','gia_tri_mac_dinh','luon_ok'] -- moi loai tac vu di qua mot con duong xu ly KHAC nhau");
}
if (toolDuPhong.trangThai.soLanDaGoi !== 1) {
  throw new Error("toolDuPhong CHI duoc goi dung 1 lan (cho doi_tra) -- tra_cuu_don_hang tu phuc hoi qua retry, khuyen_mai thanh cong ngay, ca hai KHONG can fallback");
}

const boDinhTuyenRieng: Record<LoaiTacVu, ToolMoPhong> = {
  tra_cuu_don_hang: taoToolLuonThanhCong(),
  doi_tra: taoToolLuonThanhCong(),
  khuyen_mai: taoToolLuonThatBai(),
};
const toolDuPhongRieng = taoToolDuPhong();
const ketQuaMotTacVu = chayNhieuTacVuTheoLoai(["khuyen_mai"], boDinhTuyenRieng, toolDuPhongRieng);
if (ketQuaMotTacVu.length !== 1) throw new Error("doi danh sach loai tac vu tu 3 phan tu sang 1 phan tu phai doi do dai mang tra ve -- tham so phai duoc dung that");
if (!ketQuaMotTacVu[0]!.thanhCong || ketQuaMotTacVu[0]!.giaTri !== "gia_tri_mac_dinh") {
  throw new Error("khuyen_mai dinh tuyen toi mot tool luon that bai thi phai CAN fallback, gia tri phai la gia_tri_mac_dinh");
}
if (toolDuPhongRieng.trangThai.soLanDaGoi !== 1) throw new Error("fallback rieng cho kich ban nay phai duoc goi dung 1 lan");

const kqTrucTiep = goiToolTheoLoaiTacVu("tra_cuu_don_hang", boDinhTuyen, toolDuPhong);
if (!kqTrucTiep.thanhCong) throw new Error("goi truc tiep goiToolTheoLoaiTacVu cho mot loai da dinh tuyen phai hoat dong dung");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (goiToolTheoLoaiTacVu): tra cuu toolDuocChon = boDinhTuyen[loaiTacVu], roi return goiToolCoFallback(toolDuocChon, toolDuPhong) -- KHONG viet lai logic retry/fallback, chi GOI ham da co. Cho hai (chayNhieuTacVuTheoLoai): mot vong for...of chay qua TUNG phan tu cua danhSachLoaiTacVu (MANG, khong phai so dem), moi lan goi goiToolTheoLoaiTacVu(loai, boDinhTuyen, toolDuPhong), day ket qua vao mang."
- kind: strategy
  body: "Cho dau: const toolDuocChon = boDinhTuyen[loaiTacVu]; return goiToolCoFallback(toolDuocChon, toolDuPhong); Cho hai: const ketQua: KetQuaGoiTool[] = []; for (const loai of danhSachLoaiTacVu) { ketQua.push(goiToolTheoLoaiTacVu(loai, boDinhTuyen, toolDuPhong)); } return ketQua;"
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
  expect: "[true,true,true] [\"ket_qua_that\",\"gia_tri_mac_dinh\",\"luon_ok\"]"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba loại tác vụ, một bảng định tuyến, VÀ mọi lớp phòng thủ CŨ (retry,
fallback) vẫn hoạt động Y NGUYÊN đằng sau nó — không cần viết lại một
dòng logic nào. Nhưng tới giờ, track NÀY vẫn đo completion rate bằng
`console.log` từng con số riêng lẻ. Bài sau đóng gói việc đo đó thành
một PHÉP SO SÁNH có hệ thống: chạy CÙNG một bộ tác vụ qua NHIỀU cấu
hình harness khác nhau, VÀ xếp kết quả thành một bảng.
::::

::::reflect{#nghi-lai}
Định tuyến LÀ một câu hỏi hoàn toàn khác VỚI mọi câu hỏi track này đã
hỏi trước đó. Timeout (bài `7`), circuit breaker (bài `8`), validate
(bài `9`) đều hỏi "tool NÀY có ổn không" — MỘT tool, đã biết trước.
Định tuyến hỏi "tác vụ NÀY nên tới tool NÀO" — một quyết định đứng
TRƯỚC mọi câu hỏi kia, VÀ hoàn toàn ĐỘC LẬP VỚI chúng: `Record<LoaiTacVu,
ToolMoPhong>` chỉ LÀ một phép tra cứu, không hề biết (VÀ không cần
biết) tool được chọn có retry hay fallback hay không. Đây LÀ lý do
`goiToolTheoLoaiTacVu` gọi thẳng `goiToolCoFallback` thay vì viết lại
nó: một lớp kiến trúc TỐT không chỉ LÀM ĐÚNG việc CỦA nó, mà còn ĐỂ
YÊN những lớp khác đã làm đúng việc CỦA chúng.
::::

::::checkpoint{mastery=0.87}
::::
