---
id: thiet-ke-he-thong.giao-dich-va-tien.boss-giao-dich-va-tien
title: "BOSS — khớp lệnh kích hoạt thanh toán, hoặc huỷ sạch cả hai"
summary: "xuLyLenhMua(sgd, lenhMua, khoaIdempotency) rap DUNG bon manh: kiem tra khoa idempotency da xu ly CHUA (bai 3) -- neu ROI, tra ve NGUYEN ket qua cu; khop voi lenh ban TOT nhat (bai 7-8), roi CHUYEN tien atomic tu vi nguoi mua sang nguoi ban (bai 6, dua tren kiem tra KHONG am cua bai 5); neu KHONG du tien, PHUC HOI lenh ban vao so lenh (rollback) VA tra ve trang thai huy, KHONG ghi so cai; neu du tien, ghi DUNG hai but toan doi ung vao so cai kep (bai 4). an mua khop b1 (2x50000) thanh cong; binh (chi co 1000) mua khop b2 (can 60000) bi HUY hoan toan -- vi binh KHONG doi, b2 quay VE so lenh, so cai KHONG tang."
locale: vi
track: thiet-ke-he-thong
module: giao-dich-va-tien
order: 10
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [sd.boss-giao-dich-va-tien]
requires: [sd.khop-lenh-cung-gia-fifo]
concepts: [sd.boss-giao-dich-va-tien]
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
Chín bài — đặt phòng tránh trùng, huỷ giữ chỗ (Hotel Reservation);
idempotency key, sổ cái kép (Payment); ví không âm, chuyển nguyên tử
(Digital Wallet); sổ lệnh, khớp lệnh, FIFO (Stock Exchange). Giờ ráp
bốn mảnh CUỐI vào một luồng giao dịch thật: một lệnh khớp phải kích
hoạt thanh toán — VÀ nếu thanh toán thất bại, TOÀN bộ phải coi như
chưa từng xảy ra.
::::

::::explain{#rap_xu_ly_lenh_mua}
Mỗi lệnh mua đi qua ĐÚNG bốn trạm, theo thứ TỰ: trạm MỘT — kiểm tra
khoá idempotency ĐÃ xử lý chưa (bài 3); nếu RỒI, trả về NGUYÊN kết
quả cũ NGAY. Trạm HAI — tìm lệnh bán tốt NHẤT (bài 7), khớp (bài 8).
Trạm BA — chuyển tiền NGUYÊN tử từ người mua sang người bán (bài 6,
dựa trên kiểm tra KHÔNG âm của bài 5); nếu THẤT bại, phục hồi lệnh
bán vào sổ (rollback) VÀ dừng, KHÔNG ghi gì thêm. Trạm BỐN — chỉ khi
chuyển tiền THÀNH công, ghi đúng hai bút toán vào sổ cái kép (bài 4):

```typescript title=readonly
interface Vi { chuSoHuu: string; soDu: number; }
interface HeThongVi { cacVi: Map<string, Vi>; }
function taoHeThongVi(): HeThongVi { return { cacVi: new Map() }; }
function moVi(ht: HeThongVi, chuSoHuu: string, soDuBanDau: number): void {
  ht.cacVi.set(chuSoHuu, { chuSoHuu, soDu: soDuBanDau });
}

type KetQuaChuyen = "da_chuyen" | "khong_du_so_du" | "vi_dich_khong_ton_tai";
function chuyenTien(ht: HeThongVi, tuChuSoHuu: string, denChuSoHuu: string, soTien: number): KetQuaChuyen {
  const viNguon = ht.cacVi.get(tuChuSoHuu)!;
  const viDich = ht.cacVi.get(denChuSoHuu);
  if (viDich === undefined) return "vi_dich_khong_ton_tai";
  if (viNguon.soDu < soTien) return "khong_du_so_du";
  viNguon.soDu -= soTien;
  viDich.soDu += soTien;
  return "da_chuyen";
}

type LoaiButToan = "no" | "co";
interface ButToan { taiKhoan: string; loai: LoaiButToan; soTien: number; }
interface SoCai { cacButToan: ButToan[]; }
function taoSoCai(): SoCai { return { cacButToan: [] }; }
function ghiGiaoDich(sc: SoCai, taiKhoanNo: string, taiKhoanCo: string, soTien: number): void {
  sc.cacButToan.push({ taiKhoan: taiKhoanNo, loai: "no", soTien });
  sc.cacButToan.push({ taiKhoan: taiKhoanCo, loai: "co", soTien });
}

type BenLenh = "mua" | "ban";
interface Lenh { id: string; ben: BenLenh; nguoiDat: string; gia: number; soLuong: number; }
interface SoLenh { lenhMua: Lenh[]; lenhBan: Lenh[]; }
function taoSoLenh(): SoLenh { return { lenhMua: [], lenhBan: [] }; }
function themLenh(sl: SoLenh, lenh: Lenh): void {
  if (lenh.ben === "mua") {
    sl.lenhMua.push(lenh);
    sl.lenhMua.sort((a, b) => b.gia - a.gia);
  } else {
    sl.lenhBan.push(lenh);
    sl.lenhBan.sort((a, b) => a.gia - b.gia);
  }
}

interface SanGiaoDich {
  soLenh: SoLenh;
  vi: HeThongVi;
  soCai: SoCai;
  ketQuaThanhToan: Map<string, KetQuaXuLy>;
}
function taoSanGiaoDich(): SanGiaoDich {
  return { soLenh: taoSoLenh(), vi: taoHeThongVi(), soCai: taoSoCai(), ketQuaThanhToan: new Map() };
}

type KetQuaXuLy = "khong_co_doi_ung" | "khop_va_thanh_toan" | "khop_nhung_huy_vi_khong_du_tien";

function xuLyLenhMua(sgd: SanGiaoDich, lenhMua: Lenh, khoaIdempotency: string): KetQuaXuLy {
  const daXuLy = sgd.ketQuaThanhToan.get(khoaIdempotency);
  if (daXuLy !== undefined) return daXuLy;

  const lenhBan = sgd.soLenh.lenhBan[0];
  if (lenhBan === undefined || lenhBan.soLuong !== lenhMua.soLuong) {
    sgd.ketQuaThanhToan.set(khoaIdempotency, "khong_co_doi_ung");
    return "khong_co_doi_ung";
  }

  sgd.soLenh.lenhBan.shift();
  const soTien = lenhBan.soLuong * lenhBan.gia;

  const ketQuaChuyen = chuyenTien(sgd.vi, lenhMua.nguoiDat, lenhBan.nguoiDat, soTien);
  if (ketQuaChuyen !== "da_chuyen") {
    themLenh(sgd.soLenh, lenhBan);
    sgd.ketQuaThanhToan.set(khoaIdempotency, "khop_nhung_huy_vi_khong_du_tien");
    return "khop_nhung_huy_vi_khong_du_tien";
  }

  ghiGiaoDich(sgd.soCai, lenhMua.nguoiDat, lenhBan.nguoiDat, soTien);
  sgd.ketQuaThanhToan.set(khoaIdempotency, "khop_va_thanh_toan");
  return "khop_va_thanh_toan";
}

const sgd = taoSanGiaoDich();
moVi(sgd.vi, "an", 100000);
moVi(sgd.vi, "cuaHang", 0);
themLenh(sgd.soLenh, { id: "b1", ben: "ban", nguoiDat: "cuaHang", gia: 50000, soLuong: 2 });

const m1: Lenh = { id: "m1", ben: "mua", nguoiDat: "an", gia: 50000, soLuong: 2 };
console.log("an mua khop voi b1 (2 x 50000 = 100000):", xuLyLenhMua(sgd, m1, "khoa-1"));
console.log("so du an sau khi mua:", sgd.vi.cacVi.get("an")!.soDu);
console.log("so du cuaHang sau khi ban:", sgd.vi.cacVi.get("cuaHang")!.soDu);
console.log("so but toan trong so cai:", sgd.soCai.cacButToan.length);

console.log("goi LAI voi CUNG khoa 'khoa-1' (mo phong gui lai):", xuLyLenhMua(sgd, m1, "khoa-1"));
console.log("so but toan KHONG tang them:", sgd.soCai.cacButToan.length);
```

```text title=readonly
an mua khop voi b1 (2 x 50000 = 100000): khop_va_thanh_toan
so du an sau khi mua: 0
so du cuaHang sau khi ban: 100000
so but toan trong so cai: 2
goi LAI voi CUNG khoa 'khoa-1' (mo phong gui lai): khop_va_thanh_toan
so but toan KHONG tang them: 2
```

`xuLyLenhMua` không phát MINH gì mới — nó gọi đúng thứ TỰ các mảnh đã
xây RIÊNG lẻ. `an` mua khớp ĐÚNG `b1`, tiền chuyển ĐÚNG `100000`, sổ
cái ghi ĐÚNG `2` bút toán. Gọi LẠI với cùng khoá `"khoa-1"` trả về
NGUYÊN kết quả cũ — sổ cái KHÔNG ghi thêm gì, dù hàm được gọi hai lần.
::::

::::example{#huy_hoan_toan_khi_khong_du_tien}
Khi chuyển tiền THẤT bại VÌ không đủ số dư, TOÀN bộ giao dịch bị huỷ —
không chỉ ví không bị trừ, mà lệnh bán CŨNG quay VỀ sổ lệnh y hệt
trước khi khớp, VÀ sổ cái không hề biết GÌ về lần thử này:

```typescript title=readonly
interface Vi { chuSoHuu: string; soDu: number; }
interface HeThongVi { cacVi: Map<string, Vi>; }
function taoHeThongVi(): HeThongVi { return { cacVi: new Map() }; }
function moVi(ht: HeThongVi, chuSoHuu: string, soDuBanDau: number): void {
  ht.cacVi.set(chuSoHuu, { chuSoHuu, soDu: soDuBanDau });
}

type KetQuaChuyen = "da_chuyen" | "khong_du_so_du" | "vi_dich_khong_ton_tai";
function chuyenTien(ht: HeThongVi, tuChuSoHuu: string, denChuSoHuu: string, soTien: number): KetQuaChuyen {
  const viNguon = ht.cacVi.get(tuChuSoHuu)!;
  const viDich = ht.cacVi.get(denChuSoHuu);
  if (viDich === undefined) return "vi_dich_khong_ton_tai";
  if (viNguon.soDu < soTien) return "khong_du_so_du";
  viNguon.soDu -= soTien;
  viDich.soDu += soTien;
  return "da_chuyen";
}

type LoaiButToan = "no" | "co";
interface ButToan { taiKhoan: string; loai: LoaiButToan; soTien: number; }
interface SoCai { cacButToan: ButToan[]; }
function taoSoCai(): SoCai { return { cacButToan: [] }; }
function ghiGiaoDich(sc: SoCai, taiKhoanNo: string, taiKhoanCo: string, soTien: number): void {
  sc.cacButToan.push({ taiKhoan: taiKhoanNo, loai: "no", soTien });
  sc.cacButToan.push({ taiKhoan: taiKhoanCo, loai: "co", soTien });
}

type BenLenh = "mua" | "ban";
interface Lenh { id: string; ben: BenLenh; nguoiDat: string; gia: number; soLuong: number; }
interface SoLenh { lenhMua: Lenh[]; lenhBan: Lenh[]; }
function taoSoLenh(): SoLenh { return { lenhMua: [], lenhBan: [] }; }
function themLenh(sl: SoLenh, lenh: Lenh): void {
  if (lenh.ben === "mua") {
    sl.lenhMua.push(lenh);
    sl.lenhMua.sort((a, b) => b.gia - a.gia);
  } else {
    sl.lenhBan.push(lenh);
    sl.lenhBan.sort((a, b) => a.gia - b.gia);
  }
}

interface SanGiaoDich {
  soLenh: SoLenh;
  vi: HeThongVi;
  soCai: SoCai;
  ketQuaThanhToan: Map<string, KetQuaXuLy>;
}
function taoSanGiaoDich(): SanGiaoDich {
  return { soLenh: taoSoLenh(), vi: taoHeThongVi(), soCai: taoSoCai(), ketQuaThanhToan: new Map() };
}

type KetQuaXuLy = "khong_co_doi_ung" | "khop_va_thanh_toan" | "khop_nhung_huy_vi_khong_du_tien";

function xuLyLenhMua(sgd: SanGiaoDich, lenhMua: Lenh, khoaIdempotency: string): KetQuaXuLy {
  const daXuLy = sgd.ketQuaThanhToan.get(khoaIdempotency);
  if (daXuLy !== undefined) return daXuLy;

  const lenhBan = sgd.soLenh.lenhBan[0];
  if (lenhBan === undefined || lenhBan.soLuong !== lenhMua.soLuong) {
    sgd.ketQuaThanhToan.set(khoaIdempotency, "khong_co_doi_ung");
    return "khong_co_doi_ung";
  }

  sgd.soLenh.lenhBan.shift();
  const soTien = lenhBan.soLuong * lenhBan.gia;

  const ketQuaChuyen = chuyenTien(sgd.vi, lenhMua.nguoiDat, lenhBan.nguoiDat, soTien);
  if (ketQuaChuyen !== "da_chuyen") {
    themLenh(sgd.soLenh, lenhBan);
    sgd.ketQuaThanhToan.set(khoaIdempotency, "khop_nhung_huy_vi_khong_du_tien");
    return "khop_nhung_huy_vi_khong_du_tien";
  }

  ghiGiaoDich(sgd.soCai, lenhMua.nguoiDat, lenhBan.nguoiDat, soTien);
  sgd.ketQuaThanhToan.set(khoaIdempotency, "khop_va_thanh_toan");
  return "khop_va_thanh_toan";
}

// tai lap dung trang thai tu khoi truoc, KHONG in lai: an da mua khop b1
// (2 x 50000), so du an = 0, cuaHang = 100000, so cai co 2 but toan
const sgd = taoSanGiaoDich();
moVi(sgd.vi, "an", 100000);
moVi(sgd.vi, "cuaHang", 0);
themLenh(sgd.soLenh, { id: "b1", ben: "ban", nguoiDat: "cuaHang", gia: 50000, soLuong: 2 });
const m1: Lenh = { id: "m1", ben: "mua", nguoiDat: "an", gia: 50000, soLuong: 2 };
xuLyLenhMua(sgd, m1, "khoa-1");

moVi(sgd.vi, "binh", 1000);
themLenh(sgd.soLenh, { id: "b2", ben: "ban", nguoiDat: "cuaHang", gia: 60000, soLuong: 1 });
const m2: Lenh = { id: "m2", ben: "mua", nguoiDat: "binh", gia: 60000, soLuong: 1 };
console.log("binh (chi co 1000) mua khop b2 (can 60000):", xuLyLenhMua(sgd, m2, "khoa-2"));
console.log("so du binh KHONG doi (van 1000, khong bi tru):", sgd.vi.cacVi.get("binh")!.soDu);
console.log("so du cuaHang KHONG doi (van 100000, khong nhan them):", sgd.vi.cacVi.get("cuaHang")!.soDu);
console.log("b2 co con trong so lenh ban KHONG (da duoc PHUC HOI):", JSON.stringify(sgd.soLenh.lenhBan.map((l) => l.id)));
console.log("so but toan trong so cai KHONG tang (van 2, giao dich BI HUY khong duoc ghi):", sgd.soCai.cacButToan.length);
```

```text title=readonly
binh (chi co 1000) mua khop b2 (can 60000): khop_nhung_huy_vi_khong_du_tien
so du binh KHONG doi (van 1000, khong bi tru): 1000
so du cuaHang KHONG doi (van 100000, khong nhan them): 100000
b2 co con trong so lenh ban KHONG (da duoc PHUC HOI): ["b2"]
so but toan trong so cai KHONG tang (van 2, giao dich BI HUY khong duoc ghi): 2
```

`binh` chỉ có `1000` nhưng cần `60000` — `chuyenTien` trả về
`"khong_du_so_du"` (bài 5 phát huy tác dụng NGAY bên trong bài 6).
`xuLyLenhMua` phản ứng bằng cách `themLenh` lại `b2` vào `sgd.soLenh`
— y hệt như nó CHƯA từng bị khớp — VÀ KHÔNG gọi `ghiGiaoDich` một lần
nào. Lệnh khớp bị coi LÀ chưa hề xảy ra, trên CẢ ba mặt: ví, sổ lệnh,
sổ cái.
::::

::::predict{#doan_nap_tien_roi_goi_lai commitOnce}
`binh` vừa bị huỷ giao dịch VỚI khoá `"khoa-2"`. Admin NẠP thêm tiền
vào ví `binh` (bây giờ dư SỨC trả `60000`). Client, không biết chuyện
đó, tự động GỬI LẠI đúng yêu cầu CŨ — cùng `m2`, CÙNG khoá
`"khoa-2"`. Gọi `xuLyLenhMua(sgd, m2, "khoa-2")` một lần NỮA — kết
quả LÀ gì?

:::opt{correct}
`"khop_nhung_huy_vi_khong_du_tien"` — y hệt LẦN trước; dòng ĐẦU tiên
của hàm kiểm tra `sgd.ketQuaThanhToan.get(khoaIdempotency)` VÀ tìm
thấy kết quả ĐÃ lưu (dù đó LÀ một thất bại), trả về NGAY, không hề
chạm lại vào `chuyenTien` hay kiểm tra số dư MỚI của `binh`
:::
:::opt
`"khop_va_thanh_toan"` — lần này `binh` đã đủ tiền, nên logic chuyển
tiền sẽ chạy LẠI VÀ thành công, cập nhật kết quả đã lưu THÀNH công
::why
Nhầm "idempotency key chỉ cache kết quả THÀNH công" VỚI "idempotency
key cache MỌI kết quả, kể cả thất bại" — nhưng `xuLyLenhMua` lưu VÀO
`sgd.ketQuaThanhToan` Ở CẢ ba nhánh return, không riêng nhánh thành
công.

Chỗ lệch: dòng `if (daXuLy !== undefined) return daXuLy;` LÀ điều
kiện ĐẦU tiên trong thân hàm — nó không phân biệt kết quả cũ LÀ thành
công hay thất bại. Vì `"khoa-2"` ĐÃ có `"khop_nhung_huy_vi_khong_du_
tien"` lưu sẵn TỪ lần gọi trước, lần gọi NÀY dừng lại NGAY tại dòng
đó. Muốn thử LẠI thật sự, hệ thống PHẢI dùng một khoá idempotency
MỚI — đây chính LÀ lý do vì sao khoá thường gắn VỚI một Ý định giao
dịch CỤ thể, không phải tái sử dụng VÔ hạn.
::
:::
::::

::::code{#viet_xu_ly_lenh_mua}
Hoàn thiện `xuLyLenhMua` — kiểm tra idempotency (đã có sẵn) VÀ tìm lệnh
bán tốt nhất (đã có sẵn). Còn thiếu BA việc theo đúng thứ tự: xoá lệnh
bán khỏi sổ RỒI tính `soTien`; gọi `chuyenTien` — nếu KHÔNG thành công,
`themLenh` phục hồi lệnh bán, lưu VÀ trả về
`"khop_nhung_huy_vi_khong_du_tien"`; nếu thành công, `ghiGiaoDich` VÀO
sổ cái, lưu VÀ trả về `"khop_va_thanh_toan"`.

```typescript title=starter
interface Vi { chuSoHuu: string; soDu: number; }
interface HeThongVi { cacVi: Map<string, Vi>; }
function taoHeThongVi(): HeThongVi { return { cacVi: new Map() }; }
function moVi(ht: HeThongVi, chuSoHuu: string, soDuBanDau: number): void {
  ht.cacVi.set(chuSoHuu, { chuSoHuu, soDu: soDuBanDau });
}

type KetQuaChuyen = "da_chuyen" | "khong_du_so_du" | "vi_dich_khong_ton_tai";
function chuyenTien(ht: HeThongVi, tuChuSoHuu: string, denChuSoHuu: string, soTien: number): KetQuaChuyen {
  const viNguon = ht.cacVi.get(tuChuSoHuu)!;
  const viDich = ht.cacVi.get(denChuSoHuu);
  if (viDich === undefined) return "vi_dich_khong_ton_tai";
  if (viNguon.soDu < soTien) return "khong_du_so_du";
  viNguon.soDu -= soTien;
  viDich.soDu += soTien;
  return "da_chuyen";
}

type LoaiButToan = "no" | "co";
interface ButToan { taiKhoan: string; loai: LoaiButToan; soTien: number; }
interface SoCai { cacButToan: ButToan[]; }
function taoSoCai(): SoCai { return { cacButToan: [] }; }
function ghiGiaoDich(sc: SoCai, taiKhoanNo: string, taiKhoanCo: string, soTien: number): void {
  sc.cacButToan.push({ taiKhoan: taiKhoanNo, loai: "no", soTien });
  sc.cacButToan.push({ taiKhoan: taiKhoanCo, loai: "co", soTien });
}

type BenLenh = "mua" | "ban";
interface Lenh { id: string; ben: BenLenh; nguoiDat: string; gia: number; soLuong: number; }
interface SoLenh { lenhMua: Lenh[]; lenhBan: Lenh[]; }
function taoSoLenh(): SoLenh { return { lenhMua: [], lenhBan: [] }; }
function themLenh(sl: SoLenh, lenh: Lenh): void {
  if (lenh.ben === "mua") {
    sl.lenhMua.push(lenh);
    sl.lenhMua.sort((a, b) => b.gia - a.gia);
  } else {
    sl.lenhBan.push(lenh);
    sl.lenhBan.sort((a, b) => a.gia - b.gia);
  }
}

interface SanGiaoDich {
  soLenh: SoLenh;
  vi: HeThongVi;
  soCai: SoCai;
  ketQuaThanhToan: Map<string, KetQuaXuLy>;
}
function taoSanGiaoDich(): SanGiaoDich {
  return { soLenh: taoSoLenh(), vi: taoHeThongVi(), soCai: taoSoCai(), ketQuaThanhToan: new Map() };
}

type KetQuaXuLy = "khong_co_doi_ung" | "khop_va_thanh_toan" | "khop_nhung_huy_vi_khong_du_tien";

function xuLyLenhMua(sgd: SanGiaoDich, lenhMua: Lenh, khoaIdempotency: string): KetQuaXuLy {
  const daXuLy = sgd.ketQuaThanhToan.get(khoaIdempotency);
  if (daXuLy !== undefined) return daXuLy;

  const lenhBan = sgd.soLenh.lenhBan[0];
  if (lenhBan === undefined || lenhBan.soLuong !== lenhMua.soLuong) {
    sgd.ketQuaThanhToan.set(khoaIdempotency, "khong_co_doi_ung");
    return "khong_co_doi_ung";
  }

  ___
}

const sgdX = taoSanGiaoDich();
moVi(sgdX.vi, "p1", 500);
moVi(sgdX.vi, "p2", 0);
themLenh(sgdX.soLenh, { id: "sx", ben: "ban", nguoiDat: "p2", gia: 500, soLuong: 1 });
const mX: Lenh = { id: "mx", ben: "mua", nguoiDat: "p1", gia: 500, soLuong: 1 };
console.log(xuLyLenhMua(sgdX, mX, "kx"), sgdX.vi.cacVi.get("p1")!.soDu);
```

```typescript title=solution
interface Vi { chuSoHuu: string; soDu: number; }
interface HeThongVi { cacVi: Map<string, Vi>; }
function taoHeThongVi(): HeThongVi { return { cacVi: new Map() }; }
function moVi(ht: HeThongVi, chuSoHuu: string, soDuBanDau: number): void {
  ht.cacVi.set(chuSoHuu, { chuSoHuu, soDu: soDuBanDau });
}

type KetQuaChuyen = "da_chuyen" | "khong_du_so_du" | "vi_dich_khong_ton_tai";
function chuyenTien(ht: HeThongVi, tuChuSoHuu: string, denChuSoHuu: string, soTien: number): KetQuaChuyen {
  const viNguon = ht.cacVi.get(tuChuSoHuu)!;
  const viDich = ht.cacVi.get(denChuSoHuu);
  if (viDich === undefined) return "vi_dich_khong_ton_tai";
  if (viNguon.soDu < soTien) return "khong_du_so_du";
  viNguon.soDu -= soTien;
  viDich.soDu += soTien;
  return "da_chuyen";
}

type LoaiButToan = "no" | "co";
interface ButToan { taiKhoan: string; loai: LoaiButToan; soTien: number; }
interface SoCai { cacButToan: ButToan[]; }
function taoSoCai(): SoCai { return { cacButToan: [] }; }
function ghiGiaoDich(sc: SoCai, taiKhoanNo: string, taiKhoanCo: string, soTien: number): void {
  sc.cacButToan.push({ taiKhoan: taiKhoanNo, loai: "no", soTien });
  sc.cacButToan.push({ taiKhoan: taiKhoanCo, loai: "co", soTien });
}

type BenLenh = "mua" | "ban";
interface Lenh { id: string; ben: BenLenh; nguoiDat: string; gia: number; soLuong: number; }
interface SoLenh { lenhMua: Lenh[]; lenhBan: Lenh[]; }
function taoSoLenh(): SoLenh { return { lenhMua: [], lenhBan: [] }; }
function themLenh(sl: SoLenh, lenh: Lenh): void {
  if (lenh.ben === "mua") {
    sl.lenhMua.push(lenh);
    sl.lenhMua.sort((a, b) => b.gia - a.gia);
  } else {
    sl.lenhBan.push(lenh);
    sl.lenhBan.sort((a, b) => a.gia - b.gia);
  }
}

interface SanGiaoDich {
  soLenh: SoLenh;
  vi: HeThongVi;
  soCai: SoCai;
  ketQuaThanhToan: Map<string, KetQuaXuLy>;
}
function taoSanGiaoDich(): SanGiaoDich {
  return { soLenh: taoSoLenh(), vi: taoHeThongVi(), soCai: taoSoCai(), ketQuaThanhToan: new Map() };
}

type KetQuaXuLy = "khong_co_doi_ung" | "khop_va_thanh_toan" | "khop_nhung_huy_vi_khong_du_tien";

function xuLyLenhMua(sgd: SanGiaoDich, lenhMua: Lenh, khoaIdempotency: string): KetQuaXuLy {
  const daXuLy = sgd.ketQuaThanhToan.get(khoaIdempotency);
  if (daXuLy !== undefined) return daXuLy;

  const lenhBan = sgd.soLenh.lenhBan[0];
  if (lenhBan === undefined || lenhBan.soLuong !== lenhMua.soLuong) {
    sgd.ketQuaThanhToan.set(khoaIdempotency, "khong_co_doi_ung");
    return "khong_co_doi_ung";
  }

  sgd.soLenh.lenhBan.shift();
  const soTien = lenhBan.soLuong * lenhBan.gia;

  const ketQuaChuyen = chuyenTien(sgd.vi, lenhMua.nguoiDat, lenhBan.nguoiDat, soTien);
  if (ketQuaChuyen !== "da_chuyen") {
    themLenh(sgd.soLenh, lenhBan);
    sgd.ketQuaThanhToan.set(khoaIdempotency, "khop_nhung_huy_vi_khong_du_tien");
    return "khop_nhung_huy_vi_khong_du_tien";
  }

  ghiGiaoDich(sgd.soCai, lenhMua.nguoiDat, lenhBan.nguoiDat, soTien);
  sgd.ketQuaThanhToan.set(khoaIdempotency, "khop_va_thanh_toan");
  return "khop_va_thanh_toan";
}

const sgdX = taoSanGiaoDich();
moVi(sgdX.vi, "p1", 500);
moVi(sgdX.vi, "p2", 0);
themLenh(sgdX.soLenh, { id: "sx", ben: "ban", nguoiDat: "p2", gia: 500, soLuong: 1 });
const mX: Lenh = { id: "mx", ben: "mua", nguoiDat: "p1", gia: 500, soLuong: 1 };
console.log(xuLyLenhMua(sgdX, mX, "kx"), sgdX.vi.cacVi.get("p1")!.soDu);
```

```typescript title=test
const sgdT = taoSanGiaoDich();
moVi(sgdT.vi, "an", 100000);
moVi(sgdT.vi, "cuaHang", 0);
themLenh(sgdT.soLenh, { id: "b1", ben: "ban", nguoiDat: "cuaHang", gia: 50000, soLuong: 2 });
const t1: Lenh = { id: "t1", ben: "mua", nguoiDat: "an", gia: 50000, soLuong: 2 };
const kq1 = xuLyLenhMua(sgdT, t1, "k-1");
if (kq1 !== "khop_va_thanh_toan") throw new Error("khop du dieu kien va du tien phai thanh cong");
const soDuAnSau = sgdT.vi.cacVi.get("an")!.soDu;
if (soDuAnSau !== 0) throw new Error("an phai bi tru DUNG 100000 (2 x 50000)");
const soDuCuaHangSau = sgdT.vi.cacVi.get("cuaHang")!.soDu;
if (soDuCuaHangSau !== 100000) throw new Error("cuaHang phai nhan DUNG 100000");
const soButToanSau1 = sgdT.soCai.cacButToan.length;
if (soButToanSau1 !== 2) throw new Error("mot giao dich thanh cong phai ghi DUNG 2 but toan (no + co)");

const kq2 = xuLyLenhMua(sgdT, t1, "k-1");
if (kq2 !== "khop_va_thanh_toan") throw new Error("goi lai CUNG khoa phai tra ve CUNG ket qua da luu");
const soButToanSau2 = sgdT.soCai.cacButToan.length;
if (soButToanSau2 !== 2) throw new Error("goi lai CUNG khoa KHONG duoc ghi them but toan (khong xu ly lai)");

moVi(sgdT.vi, "binh", 100);
themLenh(sgdT.soLenh, { id: "b2", ben: "ban", nguoiDat: "cuaHang", gia: 90000, soLuong: 1 });
const t2: Lenh = { id: "t2", ben: "mua", nguoiDat: "binh", gia: 90000, soLuong: 1 };
const kq3 = xuLyLenhMua(sgdT, t2, "k-2");
if (kq3 !== "khop_nhung_huy_vi_khong_du_tien") throw new Error("binh khong du tien phai bi HUY giao dich");
const soDuBinhSau = sgdT.vi.cacVi.get("binh")!.soDu;
if (soDuBinhSau !== 100) throw new Error("binh KHONG duoc bi tru khi giao dich bi huy");
const soDuCuaHangSau2 = sgdT.vi.cacVi.get("cuaHang")!.soDu;
if (soDuCuaHangSau2 !== 100000) throw new Error("cuaHang KHONG duoc nhan them khi giao dich bi huy");
const soLuongBanSauHuy = sgdT.soLenh.lenhBan.length;
if (soLuongBanSauHuy !== 1) throw new Error("lenh ban b2 phai duoc PHUC HOI lai vao so lenh sau khi huy");
const soButToanSau3 = sgdT.soCai.cacButToan.length;
if (soButToanSau3 !== 2) throw new Error("giao dich bi huy KHONG duoc ghi vao so cai");

sgdT.vi.cacVi.get("binh")!.soDu = 999999;
const kq4 = xuLyLenhMua(sgdT, t2, "k-2");
if (kq4 !== "khop_nhung_huy_vi_khong_du_tien") throw new Error("goi lai CUNG khoa da bi huy phai tra ve CACHED ket qua cu, du vi da du tien");
```

:::hints
- kind: attention
  body: "Ba buoc con thieu: (1) sgd.soLenh.lenhBan.shift(); const soTien = lenhBan.soLuong * lenhBan.gia; (2) goi chuyenTien -- neu KHAC 'da_chuyen' thi themLenh phuc hoi lenhBan, luu va tra ve 'khop_nhung_huy_vi_khong_du_tien'; (3) neu 'da_chuyen' thi ghiGiaoDich, luu va tra ve 'khop_va_thanh_toan'."
- kind: strategy
  body: "sgd.soLenh.lenhBan.shift(); const soTien = lenhBan.soLuong * lenhBan.gia; const ketQuaChuyen = chuyenTien(sgd.vi, lenhMua.nguoiDat, lenhBan.nguoiDat, soTien); if (ketQuaChuyen !== 'da_chuyen') { themLenh(sgd.soLenh, lenhBan); sgd.ketQuaThanhToan.set(khoaIdempotency, 'khop_nhung_huy_vi_khong_du_tien'); return 'khop_nhung_huy_vi_khong_du_tien'; } ghiGiaoDich(sgd.soCai, lenhMua.nguoiDat, lenhBan.nguoiDat, soTien); sgd.ketQuaThanhToan.set(khoaIdempotency, 'khop_va_thanh_toan'); return 'khop_va_thanh_toan';"
- kind: one-line
  body: "Sao chep dung logic o phan Strategy, doi dau nhay don thanh dau nhay kep cho cac chuoi trang thai."
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 8000
- tier: output
  match: contains
  expect: "khop_va_thanh_toan"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Khớp lệnh, chuyển tiền nguyên tử, sổ cái kép, idempotency — bốn mảnh
RÁP đúng thứ tự thành một luồng giao dịch không có trạng thái nửa
vời: hoặc tiền đổi chủ VÀ ghi sổ ĐẦY đủ, hoặc mọi thứ y NGUYÊN như
chưa từng có gì xảy ra. Quest "Giao dịch VÀ tiền" đã xong — VÀ cùng
với đó, track "Thiết kế thực chiến" khép LẠI trọn vẹn.
::::

::::reflect{#nghi-lai}
`xuLyLenhMua` không hề PHÁT minh gì mới — nó gọi đúng thứ tự các mảnh
đã xây RIÊNG lẻ Ở chín bài trước, VÀ giữ chúng tách BIỆT đúng mức cần
thiết: khớp lệnh (LUÔN thử trước), chuyển tiền (có thể thất bại, VÀ
khi thất bại thì rollback đúng phần đã đổi), ghi sổ (CHỈ chạy khi mọi
thứ trước đó đã chắc chắn thành công). Suốt track NÀY — hotel, payment,
wallet, exchange — một sợi chỉ CHUNG xuyên suốt: đúng đắn không phải
LÀ một tính năng thêm VÀO sau cùng, nó LÀ thứ TỰ các bước được viết
ra NGAY từ đầu.
::::

::::checkpoint{mastery=0.85}
::::
