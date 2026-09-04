---
id: thiet-ke-he-thong.thoi-gian-thuc-va-vi-tri.boss-thoi-gian-thuc-va-vi-tri
title: "BOSS — điểm số, bảng xếp hạng, thông báo chỉ tới người online"
summary: "capNhatDiemVaThongBao rap DUNG ba mang: cap nhat diem (bai 8) -> kiem tra co VUA vao top-3 hay khong (laTrongTop TRUOC va SAU cap nhat, bai 9) -> neu CO, chi gui thong bao da kenh (bai 4) cho nguoi DANG ONLINE (bai 2); offline thi bo qua, KHONG retry. dung vao top-3 MOI, online -> 'push'. em vao top-3 MOI nhung chua tung heartbeat -> 'bo_qua_offline' (diem VAN duoc ghi binh thuong). an DA o san trong top-3 tu truoc -> 'khong_vao_top', khong thong bao lai."
locale: vi
track: thiet-ke-he-thong
module: thoi-gian-thuc-va-vi-tri
order: 10
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [sd.boss-thoi-gian-thuc-va-vi-tri]
requires: [sd.truy-van-xep-hang]
concepts: [sd.boss-thoi-gian-thuc-va-vi-tri]
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
Chín bài — thứ tự tin nhắn, online, đã đọc cả nhóm (Chat); đa kênh, gộp
thông báo (Notification); mã ô, lân cận (Proximity); chèn nhị phân, top-N
VÀ hạng (Leaderboard). Giờ ráp bốn mảnh CUỐI vào một trận đấu trực tiếp:
điểm số cập nhật, ai đó vào top-3 MỚI, thông báo bay đi — nhưng CHỈ tới
người còn Ở đó.
::::

::::explain{#rap_cap_nhat_diem_va_thong_bao}
Một lượt ghi điểm đi qua ĐÚNG ba trạm, theo thứ TỰ: trạm MỘT — cập nhật
điểm VÀO bảng xếp hạng (bài 8), giữ mảng LUÔN sắp xếp. Trạm HAI — so
sánh "có Ở trong top-3 hay KHÔNG" TRƯỚC và SAU khi cập nhật (bài 9); chỉ
khi chuyển từ NGOÀI top-3 sang TRONG top-3 (vào MỚI) mới đi tiếp. Trạm BA
— kiểm tra người đó có đang online (bài 2) hay không; CHỈ người online
mới nhận thông báo đa kênh (bài 4), người offline bị BỎ qua, không retry:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface NguoiChoi { ten: string; diem: number; }
type BangXepHang = NguoiChoi[];
function taoBangXepHang(): BangXepHang { return []; }
function timViTriChenNhiPhan(bxh: BangXepHang, diem: number): number {
  let trai = 0;
  let phai = bxh.length;
  while (trai < phai) {
    const giua = Math.floor((trai + phai) / 2);
    const pt = bxh[giua];
    if (pt !== undefined && pt.diem > diem) trai = giua + 1; else phai = giua;
  }
  return trai;
}
function capNhatDiem(bxh: BangXepHang, ten: string, diemMoi: number): void {
  const viTriCu = bxh.findIndex((ng) => ng.ten === ten);
  if (viTriCu !== -1) bxh.splice(viTriCu, 1);
  const viTriMoi = timViTriChenNhiPhan(bxh, diemMoi);
  bxh.splice(viTriMoi, 0, { ten, diem: diemMoi });
}

const NGUONG_TOP = 3;
function laTrongTop(bxh: BangXepHang, ten: string): boolean {
  const viTri = bxh.findIndex((ng) => ng.ten === ten);
  return viTri !== -1 && viTri < NGUONG_TOP;
}

interface TrangThaiPresence { lanCuoiHoatDong: Map<string, number>; }
function taoTrangThaiPresence(): TrangThaiPresence { return { lanCuoiHoatDong: new Map() }; }
function ghiNhanHeartbeat(tt: TrangThaiPresence, dh: DongHoMoPhong, nguoiDung: string): void {
  tt.lanCuoiHoatDong.set(nguoiDung, dh.thoiGianHienTai);
}
const NGUONG_ONLINE_MS = 30000;
function laDangOnline(tt: TrangThaiPresence, dh: DongHoMoPhong, nguoiDung: string): boolean {
  const lanCuoi = tt.lanCuoiHoatDong.get(nguoiDung);
  if (lanCuoi === undefined) return false;
  return dh.thoiGianHienTai - lanCuoi <= NGUONG_ONLINE_MS;
}

type Kenh = "push" | "email" | "sms";
function guiThongBaoDaKenh(dsUuTien: Kenh[], dsKenhLoi: Kenh[]): Kenh | undefined {
  for (const kenh of dsUuTien) {
    if (!dsKenhLoi.includes(kenh)) return kenh;
  }
  return undefined;
}
const THU_TU_UU_TIEN_KENH: Kenh[] = ["push", "email", "sms"];

type KetQuaCapNhat = "khong_vao_top" | "bo_qua_offline" | Kenh;

function capNhatDiemVaThongBao(
  bxh: BangXepHang,
  tt: TrangThaiPresence,
  dh: DongHoMoPhong,
  ten: string,
  diemMoi: number,
  dsKenhLoi: Kenh[],
): KetQuaCapNhat {
  const dangTrongTopTruoc = laTrongTop(bxh, ten);
  capNhatDiem(bxh, ten, diemMoi);
  const dangTrongTopSau = laTrongTop(bxh, ten);

  if (!dangTrongTopSau || dangTrongTopTruoc) return "khong_vao_top";

  if (!laDangOnline(tt, dh, ten)) return "bo_qua_offline";
  const kenhThanhCong = guiThongBaoDaKenh(THU_TU_UU_TIEN_KENH, dsKenhLoi);
  return kenhThanhCong ?? "bo_qua_offline";
}

const dh = taoDongHoMoPhong();
const bxh = taoBangXepHang();
const tt = taoTrangThaiPresence();

capNhatDiem(bxh, "an", 100);
capNhatDiem(bxh, "binh", 90);
capNhatDiem(bxh, "chi", 80);
console.log("top 3 ban dau:", bxh.map((ng) => ng.ten));

ghiNhanHeartbeat(tt, dh, "dung"); // dung online tu t=0
tienThoiGian(dh, 5000); // t=5000, dung van con online (<30000ms)

const kq1 = capNhatDiemVaThongBao(bxh, tt, dh, "dung", 95, []);
console.log("dung ghi 95 diem, vao top-3 MOI, dang online -> ket qua:", kq1);
console.log("bang xep hang sau khi dung vao:", bxh.map((ng) => ng.ten));

const kq2 = capNhatDiemVaThongBao(bxh, tt, dh, "em", 110, []);
console.log("em ghi 110 diem, vao top-3 MOI, nhung CHUA TUNG heartbeat -> ket qua:", kq2);
```

```text title=readonly
top 3 ban dau: [ 'an', 'binh', 'chi' ]
dung ghi 95 diem, vao top-3 MOI, dang online -> ket qua: push
bang xep hang sau khi dung vao: [ 'an', 'dung', 'binh', 'chi' ]
em ghi 110 diem, vao top-3 MOI, nhung CHUA TUNG heartbeat -> ket qua: bo_qua_offline
```

`capNhatDiemVaThongBao` không hề PHÁT minh gì mới — nó gọi đúng thứ tự
các mảnh đã xây RIÊNG lẻ Ở chín bài trước. `"dung"` vào top-3 mới VÀ
đang online → thử kênh, `push` thành công NGAY. `"em"` cũng vào top-3
mới, nhưng CHƯA từng gửi heartbeat NÊN `laDangOnline` trả `false` — trạm
BA dừng lại Ở đó, `"em"` KHÔNG hề nhận thông báo.
::::

::::example{#da_top3_khong_bao_lai_va_channel_du_phong}
Người ĐÃ Ở sẵn trong top-3 không được thông báo LẠI khi ghi thêm điểm —
VÀ điểm số vẫn được GHI bình thường cho người bị bỏ qua thông báo Ở trạm
BA, chỉ có bước GỬI là bị chặn:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface NguoiChoi { ten: string; diem: number; }
type BangXepHang = NguoiChoi[];
function taoBangXepHang(): BangXepHang { return []; }
function timViTriChenNhiPhan(bxh: BangXepHang, diem: number): number {
  let trai = 0;
  let phai = bxh.length;
  while (trai < phai) {
    const giua = Math.floor((trai + phai) / 2);
    const pt = bxh[giua];
    if (pt !== undefined && pt.diem > diem) trai = giua + 1; else phai = giua;
  }
  return trai;
}
function capNhatDiem(bxh: BangXepHang, ten: string, diemMoi: number): void {
  const viTriCu = bxh.findIndex((ng) => ng.ten === ten);
  if (viTriCu !== -1) bxh.splice(viTriCu, 1);
  const viTriMoi = timViTriChenNhiPhan(bxh, diemMoi);
  bxh.splice(viTriMoi, 0, { ten, diem: diemMoi });
}

const NGUONG_TOP = 3;
function laTrongTop(bxh: BangXepHang, ten: string): boolean {
  const viTri = bxh.findIndex((ng) => ng.ten === ten);
  return viTri !== -1 && viTri < NGUONG_TOP;
}

interface TrangThaiPresence { lanCuoiHoatDong: Map<string, number>; }
function taoTrangThaiPresence(): TrangThaiPresence { return { lanCuoiHoatDong: new Map() }; }
function ghiNhanHeartbeat(tt: TrangThaiPresence, dh: DongHoMoPhong, nguoiDung: string): void {
  tt.lanCuoiHoatDong.set(nguoiDung, dh.thoiGianHienTai);
}
const NGUONG_ONLINE_MS = 30000;
function laDangOnline(tt: TrangThaiPresence, dh: DongHoMoPhong, nguoiDung: string): boolean {
  const lanCuoi = tt.lanCuoiHoatDong.get(nguoiDung);
  if (lanCuoi === undefined) return false;
  return dh.thoiGianHienTai - lanCuoi <= NGUONG_ONLINE_MS;
}

type Kenh = "push" | "email" | "sms";
function guiThongBaoDaKenh(dsUuTien: Kenh[], dsKenhLoi: Kenh[]): Kenh | undefined {
  for (const kenh of dsUuTien) {
    if (!dsKenhLoi.includes(kenh)) return kenh;
  }
  return undefined;
}
const THU_TU_UU_TIEN_KENH: Kenh[] = ["push", "email", "sms"];

type KetQuaCapNhat = "khong_vao_top" | "bo_qua_offline" | Kenh;

function capNhatDiemVaThongBao(
  bxh: BangXepHang,
  tt: TrangThaiPresence,
  dh: DongHoMoPhong,
  ten: string,
  diemMoi: number,
  dsKenhLoi: Kenh[],
): KetQuaCapNhat {
  const dangTrongTopTruoc = laTrongTop(bxh, ten);
  capNhatDiem(bxh, ten, diemMoi);
  const dangTrongTopSau = laTrongTop(bxh, ten);

  if (!dangTrongTopSau || dangTrongTopTruoc) return "khong_vao_top";

  if (!laDangOnline(tt, dh, ten)) return "bo_qua_offline";
  const kenhThanhCong = guiThongBaoDaKenh(THU_TU_UU_TIEN_KENH, dsKenhLoi);
  return kenhThanhCong ?? "bo_qua_offline";
}

// tai lap dung trang thai tu khoi truoc, KHONG in lai: top-3 an/binh/chi, dung da
// vao top-3 (heartbeat t=0), em ghi 110 diem nhung bi bo qua thong bao vi offline
const dh = taoDongHoMoPhong();
const bxh = taoBangXepHang();
const tt = taoTrangThaiPresence();
capNhatDiem(bxh, "an", 100);
capNhatDiem(bxh, "binh", 90);
capNhatDiem(bxh, "chi", 80);
ghiNhanHeartbeat(tt, dh, "dung");
tienThoiGian(dh, 5000);
capNhatDiemVaThongBao(bxh, tt, dh, "dung", 95, []);
capNhatDiemVaThongBao(bxh, tt, dh, "em", 110, []);

console.log("bang xep hang hien tai:", bxh.map((ng) => ng.ten));
console.log("em (bi bo qua thong bao) co con duoc GHI DIEM binh thuong khong:", bxh.some((ng) => ng.ten === "em" && ng.diem === 110));

const kq3 = capNhatDiemVaThongBao(bxh, tt, dh, "an", 102, []);
console.log("an (DA o san trong top-3) ghi them diem -> ket qua:", kq3);

ghiNhanHeartbeat(tt, dh, "chi"); // chi vua online
const kq4 = capNhatDiemVaThongBao(bxh, tt, dh, "chi", 200, ["push"]);
console.log("chi bat dau ngoai top-3, ghi 200 diem, vao top-3 MOI, online, push LOI -> ket qua:", kq4);
```

```text title=readonly
bang xep hang hien tai: [ 'em', 'an', 'dung', 'binh', 'chi' ]
em (bi bo qua thong bao) co con duoc GHI DIEM binh thuong khong: true
an (DA o san trong top-3) ghi them diem -> ket qua: khong_vao_top
chi bat dau ngoai top-3, ghi 200 diem, vao top-3 MOI, online, push LOI -> ket qua: email
```

`"em"` vẫn CÓ mặt trong bảng với đúng điểm `110` dù thông báo của nó bị
bỏ qua — `capNhatDiem` LUÔN chạy trước khi trạm hai VÀ ba kịp quyết định
gì. `"an"` vốn ĐÃ ở top-3 từ trước (`dangTrongTopTruoc = true`), nên dù
ghi thêm điểm VÀ vẫn Ở top-3 sau đó, kết quả LÀ `"khong_vao_top"` — điều
kiện chỉ bắt sự kiện "MỚI vào", không phải "vẫn còn Ở trong". `"chi"`
vào top-3 mới, online, nhưng `push` LỖI (nằm trong `dsKenhLoi`) — trạm
đa kênh tự động chuyển sang `email`.
::::

::::predict{#doan_online_khong_du_neu_khong_vao_top commitOnce}
`"binh"` hiện KHÔNG nằm trong top-3. `"binh"` gửi heartbeat (đang
online), RỒI ghi thêm điểm — nhưng điểm mới vẫn quá THẤP, không đủ để
lọt vào top-3. `capNhatDiemVaThongBao` LÚC này trả về gì?

:::opt{correct}
`"khong_vao_top"` — điều kiện `!dangTrongTopSau || dangTrongTopTruoc` bị
chặn NGAY từ trạm hai (vì `dangTrongTopSau` LÀ `false`), hàm trả về VÀ
kết thúc TRƯỚC khi kịp xét `"binh"` có đang online hay không
:::
:::opt
Một kênh cụ thể (ví dụ `"push"`) — vì `"binh"` ĐANG online, hệ thống vẫn
gửi thông báo báo điểm mới, dù điểm đó chưa đủ vào top-3
::why
Nhầm "đang online" VỚI "luôn được thông báo mỗi lần ghi điểm" — nhưng
`capNhatDiemVaThongBao` chỉ xét chuyện online Ở trạm BA, mà trạm ba chỉ
chạy tới KHI trạm hai đã cho phép đi tiếp.

Chỗ lệch: dòng `if (!dangTrongTopSau || dangTrongTopTruoc) return
"khong_vao_top";` đứng TRƯỚC bất kỳ lời gọi nào tới `laDangOnline`. Với
`"binh"`, `dangTrongTopSau` LÀ `false` (điểm mới vẫn không đủ vào
top-3), NÊN hàm `return` ngay tại đó — trạng thái online của `"binh"`
không hề được ĐỌC tới trong lần gọi này.
::
:::
::::

::::code{#viet_cap_nhat_diem_va_thong_bao}
Phần cập nhật điểm VÀ kiểm tra "vào top-3 mới" đã CÓ sẵn. Hoàn thiện
phần CÒN lại: nếu người chơi ĐANG offline thì bỏ qua; ngược lại thử gửi
thông báo đa kênh VÀ trả về kênh thành công (hoặc `"bo_qua_offline"` nếu
không kênh nào chịu nhận).

```typescript title=starter
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface NguoiChoi { ten: string; diem: number; }
type BangXepHang = NguoiChoi[];
function taoBangXepHang(): BangXepHang { return []; }
function timViTriChenNhiPhan(bxh: BangXepHang, diem: number): number {
  let trai = 0;
  let phai = bxh.length;
  while (trai < phai) {
    const giua = Math.floor((trai + phai) / 2);
    const pt = bxh[giua];
    if (pt !== undefined && pt.diem > diem) trai = giua + 1; else phai = giua;
  }
  return trai;
}
function capNhatDiem(bxh: BangXepHang, ten: string, diemMoi: number): void {
  const viTriCu = bxh.findIndex((ng) => ng.ten === ten);
  if (viTriCu !== -1) bxh.splice(viTriCu, 1);
  const viTriMoi = timViTriChenNhiPhan(bxh, diemMoi);
  bxh.splice(viTriMoi, 0, { ten, diem: diemMoi });
}

const NGUONG_TOP = 3;
function laTrongTop(bxh: BangXepHang, ten: string): boolean {
  const viTri = bxh.findIndex((ng) => ng.ten === ten);
  return viTri !== -1 && viTri < NGUONG_TOP;
}

interface TrangThaiPresence { lanCuoiHoatDong: Map<string, number>; }
function taoTrangThaiPresence(): TrangThaiPresence { return { lanCuoiHoatDong: new Map() }; }
function ghiNhanHeartbeat(tt: TrangThaiPresence, dh: DongHoMoPhong, nguoiDung: string): void {
  tt.lanCuoiHoatDong.set(nguoiDung, dh.thoiGianHienTai);
}
const NGUONG_ONLINE_MS = 30000;
function laDangOnline(tt: TrangThaiPresence, dh: DongHoMoPhong, nguoiDung: string): boolean {
  const lanCuoi = tt.lanCuoiHoatDong.get(nguoiDung);
  if (lanCuoi === undefined) return false;
  return dh.thoiGianHienTai - lanCuoi <= NGUONG_ONLINE_MS;
}

type Kenh = "push" | "email" | "sms";
function guiThongBaoDaKenh(dsUuTien: Kenh[], dsKenhLoi: Kenh[]): Kenh | undefined {
  for (const kenh of dsUuTien) {
    if (!dsKenhLoi.includes(kenh)) return kenh;
  }
  return undefined;
}
const THU_TU_UU_TIEN_KENH: Kenh[] = ["push", "email", "sms"];

type KetQuaCapNhat = "khong_vao_top" | "bo_qua_offline" | Kenh;

function capNhatDiemVaThongBao(
  bxh: BangXepHang,
  tt: TrangThaiPresence,
  dh: DongHoMoPhong,
  ten: string,
  diemMoi: number,
  dsKenhLoi: Kenh[],
): KetQuaCapNhat {
  const dangTrongTopTruoc = laTrongTop(bxh, ten);
  capNhatDiem(bxh, ten, diemMoi);
  const dangTrongTopSau = laTrongTop(bxh, ten);

  if (!dangTrongTopSau || dangTrongTopTruoc) return "khong_vao_top";

  ___
}

const dhX = taoDongHoMoPhong();
const bxhX = taoBangXepHang();
const ttX = taoTrangThaiPresence();
capNhatDiem(bxhX, "x1", 10);
ghiNhanHeartbeat(ttX, dhX, "x2");
console.log(capNhatDiemVaThongBao(bxhX, ttX, dhX, "x2", 20, []));
```

```typescript title=solution
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface NguoiChoi { ten: string; diem: number; }
type BangXepHang = NguoiChoi[];
function taoBangXepHang(): BangXepHang { return []; }
function timViTriChenNhiPhan(bxh: BangXepHang, diem: number): number {
  let trai = 0;
  let phai = bxh.length;
  while (trai < phai) {
    const giua = Math.floor((trai + phai) / 2);
    const pt = bxh[giua];
    if (pt !== undefined && pt.diem > diem) trai = giua + 1; else phai = giua;
  }
  return trai;
}
function capNhatDiem(bxh: BangXepHang, ten: string, diemMoi: number): void {
  const viTriCu = bxh.findIndex((ng) => ng.ten === ten);
  if (viTriCu !== -1) bxh.splice(viTriCu, 1);
  const viTriMoi = timViTriChenNhiPhan(bxh, diemMoi);
  bxh.splice(viTriMoi, 0, { ten, diem: diemMoi });
}

const NGUONG_TOP = 3;
function laTrongTop(bxh: BangXepHang, ten: string): boolean {
  const viTri = bxh.findIndex((ng) => ng.ten === ten);
  return viTri !== -1 && viTri < NGUONG_TOP;
}

interface TrangThaiPresence { lanCuoiHoatDong: Map<string, number>; }
function taoTrangThaiPresence(): TrangThaiPresence { return { lanCuoiHoatDong: new Map() }; }
function ghiNhanHeartbeat(tt: TrangThaiPresence, dh: DongHoMoPhong, nguoiDung: string): void {
  tt.lanCuoiHoatDong.set(nguoiDung, dh.thoiGianHienTai);
}
const NGUONG_ONLINE_MS = 30000;
function laDangOnline(tt: TrangThaiPresence, dh: DongHoMoPhong, nguoiDung: string): boolean {
  const lanCuoi = tt.lanCuoiHoatDong.get(nguoiDung);
  if (lanCuoi === undefined) return false;
  return dh.thoiGianHienTai - lanCuoi <= NGUONG_ONLINE_MS;
}

type Kenh = "push" | "email" | "sms";
function guiThongBaoDaKenh(dsUuTien: Kenh[], dsKenhLoi: Kenh[]): Kenh | undefined {
  for (const kenh of dsUuTien) {
    if (!dsKenhLoi.includes(kenh)) return kenh;
  }
  return undefined;
}
const THU_TU_UU_TIEN_KENH: Kenh[] = ["push", "email", "sms"];

type KetQuaCapNhat = "khong_vao_top" | "bo_qua_offline" | Kenh;

function capNhatDiemVaThongBao(
  bxh: BangXepHang,
  tt: TrangThaiPresence,
  dh: DongHoMoPhong,
  ten: string,
  diemMoi: number,
  dsKenhLoi: Kenh[],
): KetQuaCapNhat {
  const dangTrongTopTruoc = laTrongTop(bxh, ten);
  capNhatDiem(bxh, ten, diemMoi);
  const dangTrongTopSau = laTrongTop(bxh, ten);

  if (!dangTrongTopSau || dangTrongTopTruoc) return "khong_vao_top";

  if (!laDangOnline(tt, dh, ten)) return "bo_qua_offline";
  const kenhThanhCong = guiThongBaoDaKenh(THU_TU_UU_TIEN_KENH, dsKenhLoi);
  return kenhThanhCong ?? "bo_qua_offline";
}

const dhX = taoDongHoMoPhong();
const bxhX = taoBangXepHang();
const ttX = taoTrangThaiPresence();
capNhatDiem(bxhX, "x1", 10);
ghiNhanHeartbeat(ttX, dhX, "x2");
console.log(capNhatDiemVaThongBao(bxhX, ttX, dhX, "x2", 20, []));
```

```typescript title=test
const dh = taoDongHoMoPhong();
const bxh = taoBangXepHang();
const tt = taoTrangThaiPresence();

capNhatDiem(bxh, "an", 100);
capNhatDiem(bxh, "binh", 90);
capNhatDiem(bxh, "chi", 80);

ghiNhanHeartbeat(tt, dh, "dung");
tienThoiGian(dh, 5000);
const kq1 = capNhatDiemVaThongBao(bxh, tt, dh, "dung", 95, []);
if (kq1 !== "push") throw new Error("dung vao top-3 MOI, dang online, khong kenh nao loi -> phai la 'push'");

const kq2 = capNhatDiemVaThongBao(bxh, tt, dh, "em", 110, []);
if (kq2 !== "bo_qua_offline") throw new Error("em vao top-3 MOI nhung CHUA TUNG heartbeat -> phai bo qua ('bo_qua_offline')");
if (!bxh.some((ng) => ng.ten === "em" && ng.diem === 110)) throw new Error("em van phai duoc GHI DIEM binh thuong du thong bao bi bo qua");

const kq3 = capNhatDiemVaThongBao(bxh, tt, dh, "an", 102, []);
if (kq3 !== "khong_vao_top") throw new Error("an DA o san trong top-3 tu truoc, khong duoc thong bao lai");

ghiNhanHeartbeat(tt, dh, "chi");
const kq4 = capNhatDiemVaThongBao(bxh, tt, dh, "chi", 200, ["push"]);
if (kq4 !== "email") throw new Error("chi vao top-3 MOI, online, push loi -> phai roi qua 'email'");

ghiNhanHeartbeat(tt, dh, "binh");
const kq5 = capNhatDiemVaThongBao(bxh, tt, dh, "binh", 91, []);
if (kq5 !== "khong_vao_top") throw new Error("binh online nhung diem moi van khong du vao top-3 -> phai 'khong_vao_top', KHONG duoc gui thong bao");
```

:::hints
- kind: attention
  body: "Con thieu dung hai buoc: (1) neu KHONG online (!laDangOnline(...)) thi return 'bo_qua_offline'; (2) nguoc lai goi guiThongBaoDaKenh va tra ve ket qua cua no (hoac 'bo_qua_offline' neu khong kenh nao thanh cong)."
- kind: strategy
  body: "if (!laDangOnline(tt, dh, ten)) return 'bo_qua_offline'; const kenhThanhCong = guiThongBaoDaKenh(THU_TU_UU_TIEN_KENH, dsKenhLoi); return kenhThanhCong ?? 'bo_qua_offline';"
- kind: one-line
  body: "if (!laDangOnline(tt, dh, ten)) return \"bo_qua_offline\"; const kenhThanhCong = guiThongBaoDaKenh(THU_TU_UU_TIEN_KENH, dsKenhLoi); return kenhThanhCong ?? \"bo_qua_offline\";"
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 7000
- tier: output
  match: contains
  expect: "push"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cập nhật điểm, kiểm tra vào top-3 mới, lọc theo online, thử đa kênh — bốn
mảnh ráp ĐÚNG thứ tự thành một luồng trận đấu hoàn chỉnh. Quest "Thời
gian thực VÀ vị trí" đã xong.
::::

::::reflect{#nghi-lai}
`capNhatDiemVaThongBao` không hề PHÁT minh gì mới — nó chỉ gọi đúng thứ
tự các mảnh đã xây RIÊNG lẻ trong chín bài trước, VÀ giữ chúng tách BIỆT
đúng mức cần thiết: ghi điểm (LUÔN chạy), phát hiện sự kiện "vào top-3
mới" (một phép so sánh trước/sau), VÀ lọc người NHẬN theo online (một
điều kiện độc lập) — ba mối quan tâm khác nhau, không GỘP làm một chỉ vì
chúng cùng chạy trong một hàm.
::::

::::checkpoint{mastery=0.85}
::::
