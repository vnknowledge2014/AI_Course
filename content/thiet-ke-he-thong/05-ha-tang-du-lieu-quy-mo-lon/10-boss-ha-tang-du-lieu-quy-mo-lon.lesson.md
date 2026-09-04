---
id: thiet-ke-he-thong.ha-tang-du-lieu-quy-mo-lon.boss-ha-tang-du-lieu-quy-mo-lon
title: "BOSS — chia khối, khử trùng, sao lưu, rồi báo tin"
summary: "nhanKhoiFile(ht, idFile, chiSoKhoi, noiDungKhoi) rap DUNG bon mang: luu khoi qua kho khu trung theo hash (bai 6) VOI moi khoi MOI duoc sao thanh 3 ban tren cac dia (bai 7), danh dau khoi da nhan cho file DO (bai 5) -- CHI khi DU tat ca khoi (laHoanTat), day mot tin nhan 'file_san_sang' vao hang doi phan vung theo idFile (bai 1) VA tra ve 'hoan_tat_da_thong_bao'; con thieu khoi thi tra ve 'dang_cho_them_khoi', KHONG gui thong bao nao. video-a VA video-b cung mot khoi mo dau TRUNG noi dung -- khoi do chi luu 1 lan (kho.khoi.size khong tang gap doi) nhung CA HAI file van duoc thong bao rieng khi hoan tat."
locale: vi
track: thiet-ke-he-thong
module: ha-tang-du-lieu-quy-mo-lon
order: 10
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [sd.boss-ha-tang-du-lieu-quy-mo-lon]
requires: [sd.hang-doi-gui-thu-va-bounce]
concepts: [sd.boss-ha-tang-du-lieu-quy-mo-lon]
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
Chín bài — phân vùng, ACK VÀ giao lại (Message Queue); gom theo khung,
cảnh báo có độ trễ (Metrics); chia khối, khử trùng, sao lưu (lưu trữ
theo khối). Giờ ráp bốn mảnh CUỐI vào một luồng upload thật sự: khối
tới, được lưu, VÀ khi file ĐỦ khối — một tin nhắn bay đi báo cho hệ
thống KHÁC biết "sẵn sàng rồi".
::::

::::explain{#rap_nhan_khoi_file}
Mỗi khối tới đi qua ĐÚNG ba trạm, theo thứ TỰ: trạm MỘT — lưu khối vào
kho khử trùng theo hash (bài 6), mỗi khối MỚI được sao thành `3` bản
trên các đĩa khác nhau (bài 7). Trạm HAI — đánh dấu chỉ số khối đó ĐÃ
nhận cho đúng file (bài 5). Trạm BA — CHỈ khi file đã ĐỦ tất cả khối,
đẩy một tin nhắn `"file_san_sang"` vào hàng đợi phân vùng THEO
`idFile` (bài 1) VÀ báo hoàn tất; còn thiếu khối thì KHÔNG gửi gì cả:

```typescript title=readonly
function bamNoiDung(noiDung: string): string {
  let h = 0;
  for (let i = 0; i < noiDung.length; i++) {
    h = (h * 31 + noiDung.charCodeAt(i)) % 1000000007;
  }
  return "h" + h.toString(16);
}
function bam(khoa: string): number {
  let h = 0;
  for (let i = 0; i < khoa.length; i++) {
    h = (h * 31 + khoa.charCodeAt(i)) % 1000000007;
  }
  return h;
}

interface KhoiLuuTru { noiDung: string; soThamChieu: number; dsDia: string[]; }
interface KhoLuuTruKhoi { khoi: Map<string, KhoiLuuTru>; }
function taoKhoLuuTruKhoi(): KhoLuuTruKhoi { return { khoi: new Map() }; }
const DIA_CO_SAN = ["dia-1", "dia-2", "dia-3", "dia-4"];
function chonDiaSaoLuu(kho: KhoLuuTruKhoi): string[] {
  const batDau = kho.khoi.size % DIA_CO_SAN.length;
  const a = DIA_CO_SAN[batDau]!;
  const b = DIA_CO_SAN[(batDau + 1) % DIA_CO_SAN.length]!;
  const c = DIA_CO_SAN[(batDau + 2) % DIA_CO_SAN.length]!;
  return [a, b, c];
}
function luuKhoi(kho: KhoLuuTruKhoi, noiDung: string): string {
  const hash = bamNoiDung(noiDung);
  const hienCo = kho.khoi.get(hash);
  if (hienCo !== undefined) {
    hienCo.soThamChieu += 1;
  } else {
    kho.khoi.set(hash, { noiDung, soThamChieu: 1, dsDia: chonDiaSaoLuu(kho) });
  }
  return hash;
}

interface TinNhanHangDoi { khoa: string; noiDung: string; }
interface HangDoiThongBao { soPhanVung: number; phanVung: TinNhanHangDoi[][]; }
function taoHangDoiThongBao(soPhanVung: number): HangDoiThongBao {
  const phanVung: TinNhanHangDoi[][] = [];
  for (let i = 0; i < soPhanVung; i++) phanVung.push([]);
  return { soPhanVung, phanVung };
}
function guiThongBao(hd: HangDoiThongBao, khoa: string, noiDung: string): void {
  const idx = bam(khoa) % hd.soPhanVung;
  hd.phanVung[idx]!.push({ khoa, noiDung });
}
function demSoTinNhan(hd: HangDoiThongBao): number {
  return hd.phanVung.reduce((t, p) => t + p.length, 0);
}

interface TrangThaiUpload { idFile: string; tongSoKhoi: number; khoiDaNhan: Set<number>; }
function taoTrangThaiUpload(idFile: string, tongSoKhoi: number): TrangThaiUpload {
  return { idFile, tongSoKhoi, khoiDaNhan: new Set() };
}
function laHoanTat(tt: TrangThaiUpload): boolean {
  return tt.khoiDaNhan.size === tt.tongSoKhoi;
}

interface HeThongUpload {
  upload: Map<string, TrangThaiUpload>;
  kho: KhoLuuTruKhoi;
  hangDoi: HangDoiThongBao;
}
function taoHeThongUpload(soPhanVung: number): HeThongUpload {
  return { upload: new Map(), kho: taoKhoLuuTruKhoi(), hangDoi: taoHangDoiThongBao(soPhanVung) };
}
function batDauUpload(ht: HeThongUpload, idFile: string, tongSoKhoi: number): void {
  ht.upload.set(idFile, taoTrangThaiUpload(idFile, tongSoKhoi));
}
type KetQuaNhanKhoi = "dang_cho_them_khoi" | "hoan_tat_da_thong_bao";
function nhanKhoiFile(ht: HeThongUpload, idFile: string, chiSoKhoi: number, noiDungKhoi: string): KetQuaNhanKhoi {
  const tt = ht.upload.get(idFile)!;
  luuKhoi(ht.kho, noiDungKhoi);
  tt.khoiDaNhan.add(chiSoKhoi);
  if (!laHoanTat(tt)) return "dang_cho_them_khoi";
  guiThongBao(ht.hangDoi, idFile, "file_san_sang");
  return "hoan_tat_da_thong_bao";
}

const ht = taoHeThongUpload(4);
batDauUpload(ht, "video-a.mp4", 2);
batDauUpload(ht, "video-b.mp4", 2);

console.log("video-a khoi 0:", nhanKhoiFile(ht, "video-a.mp4", 0, "noi-dung-chung-mo-dau"));
console.log("video-a khoi 1:", nhanKhoiFile(ht, "video-a.mp4", 1, "noi-dung-rieng-cua-a"));
console.log("so tin nhan trong hang doi (sau khi video-a xong):", demSoTinNhan(ht.hangDoi));

console.log("video-b khoi 0 (TRUNG NOI DUNG voi video-a khoi 0):", nhanKhoiFile(ht, "video-b.mp4", 0, "noi-dung-chung-mo-dau"));
console.log("video-b khoi 1:", nhanKhoiFile(ht, "video-b.mp4", 1, "noi-dung-rieng-cua-b"));
console.log("so tin nhan trong hang doi (sau khi video-b xong):", demSoTinNhan(ht.hangDoi));

console.log("so khoi THAT SU luu trong kho (khoi chung chi luu 1 lan):", ht.kho.khoi.size);
```

```text title=readonly
video-a khoi 0: dang_cho_them_khoi
video-a khoi 1: hoan_tat_da_thong_bao
so tin nhan trong hang doi (sau khi video-a xong): 1
video-b khoi 0 (TRUNG NOI DUNG voi video-a khoi 0): dang_cho_them_khoi
video-b khoi 1: hoan_tat_da_thong_bao
so tin nhan trong hang doi (sau khi video-b xong): 2
so khoi THAT SU luu trong kho (khoi chung chi luu 1 lan): 3
```

`nhanKhoiFile` không phát MINH gì mới — nó gọi đúng thứ TỰ các mảnh đã
xây RIÊNG lẻ. `video-a` VÀ `video-b` cùng chia sẻ một khối mở đầu
GIỐNG hệt nhau — `luuKhoi` chỉ lưu nó ĐÚNG một lần (`kho.khoi.size`
LÀ `3`, không phải `4`) — nhưng CẢ hai file vẫn nhận đúng thông báo
RIÊNG của mình khi ĐỦ khối, vì việc "đủ khối cho file NÀO" (`bài 5`)
tách BIỆT hoàn toàn khỏi việc "khối này đã lưu CHƯA" (`bài 6`).
::::

::::example{#khoi_toi_khong_theo_thu_tu_van_hoan_tat}
Khối vẫn có thể tới KHÔNG theo thứ tự — file chỉ hoàn tất, VÀ thông
báo chỉ được gửi, đúng NGAY tại lần gọi khiến khối CUỐI cùng còn thiếu
xuất hiện:

```typescript title=readonly
function bamNoiDung(noiDung: string): string {
  let h = 0;
  for (let i = 0; i < noiDung.length; i++) {
    h = (h * 31 + noiDung.charCodeAt(i)) % 1000000007;
  }
  return "h" + h.toString(16);
}
function bam(khoa: string): number {
  let h = 0;
  for (let i = 0; i < khoa.length; i++) {
    h = (h * 31 + khoa.charCodeAt(i)) % 1000000007;
  }
  return h;
}

interface KhoiLuuTru { noiDung: string; soThamChieu: number; dsDia: string[]; }
interface KhoLuuTruKhoi { khoi: Map<string, KhoiLuuTru>; }
function taoKhoLuuTruKhoi(): KhoLuuTruKhoi { return { khoi: new Map() }; }
const DIA_CO_SAN = ["dia-1", "dia-2", "dia-3", "dia-4"];
function chonDiaSaoLuu(kho: KhoLuuTruKhoi): string[] {
  const batDau = kho.khoi.size % DIA_CO_SAN.length;
  const a = DIA_CO_SAN[batDau]!;
  const b = DIA_CO_SAN[(batDau + 1) % DIA_CO_SAN.length]!;
  const c = DIA_CO_SAN[(batDau + 2) % DIA_CO_SAN.length]!;
  return [a, b, c];
}
function luuKhoi(kho: KhoLuuTruKhoi, noiDung: string): string {
  const hash = bamNoiDung(noiDung);
  const hienCo = kho.khoi.get(hash);
  if (hienCo !== undefined) {
    hienCo.soThamChieu += 1;
  } else {
    kho.khoi.set(hash, { noiDung, soThamChieu: 1, dsDia: chonDiaSaoLuu(kho) });
  }
  return hash;
}

interface TinNhanHangDoi { khoa: string; noiDung: string; }
interface HangDoiThongBao { soPhanVung: number; phanVung: TinNhanHangDoi[][]; }
function taoHangDoiThongBao(soPhanVung: number): HangDoiThongBao {
  const phanVung: TinNhanHangDoi[][] = [];
  for (let i = 0; i < soPhanVung; i++) phanVung.push([]);
  return { soPhanVung, phanVung };
}
function guiThongBao(hd: HangDoiThongBao, khoa: string, noiDung: string): void {
  const idx = bam(khoa) % hd.soPhanVung;
  hd.phanVung[idx]!.push({ khoa, noiDung });
}
function demSoTinNhan(hd: HangDoiThongBao): number {
  return hd.phanVung.reduce((t, p) => t + p.length, 0);
}

interface TrangThaiUpload { idFile: string; tongSoKhoi: number; khoiDaNhan: Set<number>; }
function taoTrangThaiUpload(idFile: string, tongSoKhoi: number): TrangThaiUpload {
  return { idFile, tongSoKhoi, khoiDaNhan: new Set() };
}
function laHoanTat(tt: TrangThaiUpload): boolean {
  return tt.khoiDaNhan.size === tt.tongSoKhoi;
}

interface HeThongUpload {
  upload: Map<string, TrangThaiUpload>;
  kho: KhoLuuTruKhoi;
  hangDoi: HangDoiThongBao;
}
function taoHeThongUpload(soPhanVung: number): HeThongUpload {
  return { upload: new Map(), kho: taoKhoLuuTruKhoi(), hangDoi: taoHangDoiThongBao(soPhanVung) };
}
function batDauUpload(ht: HeThongUpload, idFile: string, tongSoKhoi: number): void {
  ht.upload.set(idFile, taoTrangThaiUpload(idFile, tongSoKhoi));
}
type KetQuaNhanKhoi = "dang_cho_them_khoi" | "hoan_tat_da_thong_bao";
function nhanKhoiFile(ht: HeThongUpload, idFile: string, chiSoKhoi: number, noiDungKhoi: string): KetQuaNhanKhoi {
  const tt = ht.upload.get(idFile)!;
  luuKhoi(ht.kho, noiDungKhoi);
  tt.khoiDaNhan.add(chiSoKhoi);
  if (!laHoanTat(tt)) return "dang_cho_them_khoi";
  guiThongBao(ht.hangDoi, idFile, "file_san_sang");
  return "hoan_tat_da_thong_bao";
}

// tai lap dung trang thai tu khoi truoc, KHONG in lai: video-a VA video-b da
// hoan tat (2 thong bao trong hang doi), kho co 3 khoi that su
const ht = taoHeThongUpload(4);
batDauUpload(ht, "video-a.mp4", 2);
batDauUpload(ht, "video-b.mp4", 2);
nhanKhoiFile(ht, "video-a.mp4", 0, "noi-dung-chung-mo-dau");
nhanKhoiFile(ht, "video-a.mp4", 1, "noi-dung-rieng-cua-a");
nhanKhoiFile(ht, "video-b.mp4", 0, "noi-dung-chung-mo-dau");
nhanKhoiFile(ht, "video-b.mp4", 1, "noi-dung-rieng-cua-b");

batDauUpload(ht, "video-c.mp4", 3);
console.log("video-c khoi 2 (toi TRUOC):", nhanKhoiFile(ht, "video-c.mp4", 2, "phan-c-2"));
console.log("video-c khoi 0:", nhanKhoiFile(ht, "video-c.mp4", 0, "phan-c-0"));
console.log("so tin nhan hang doi (video-c CHUA xong):", demSoTinNhan(ht.hangDoi));
console.log("video-c khoi 1 (toi CUOI cung, khoi con thieu):", nhanKhoiFile(ht, "video-c.mp4", 1, "phan-c-1"));
console.log("so tin nhan hang doi (video-c XONG):", demSoTinNhan(ht.hangDoi));
```

```text title=readonly
video-c khoi 2 (toi TRUOC): dang_cho_them_khoi
video-c khoi 0: dang_cho_them_khoi
so tin nhan hang doi (video-c CHUA xong): 2
video-c khoi 1 (toi CUOI cung, khoi con thieu): hoan_tat_da_thong_bao
so tin nhan hang doi (video-c XONG): 3
```

Khối `2` tới TRƯỚC cả khối `0` — `laHoanTat` vẫn tính đúng dựa trên
`Set` (bài 5), không quan tâm THỨ tự tới. Thông báo CHỈ được đẩy vào
hàng đợi đúng LÚC khối `1` (khối còn thiếu cuối cùng) xuất hiện — số
tin nhắn nhảy từ `2` lên `3` chính XÁC tại lần gọi đó.
::::

::::predict{#doan_file_1_khoi_trung_noi_dung commitOnce}
Một file MỚI, `"video-e.mp4"`, chỉ có ĐÚNG `1` khối. Nội dung khối đó
TRÙNG hệt với khối chung đã lưu Ở trên (`"noi-dung-chung-mo-dau"`) —
NGHĨA là `luuKhoi` sẽ không tạo bản ghi khối MỚI nào, chỉ tăng
`soThamChieu`. Gọi `nhanKhoiFile` cho khối DUY nhất đó — hàm trả về
gì?

:::opt{correct}
`"hoan_tat_da_thong_bao"` — việc khối bị khử trùng (không lưu bản MỚI)
xảy ra Ở trạm MỘT, hoàn toàn tách biệt VỚI trạm hai (đánh dấu đã nhận
CHO file này); file chỉ cần `1/1` khối, VÀ khối đó ĐÃ được đánh dấu
nhận — đủ điều kiện hoàn tất
:::
:::opt
`"dang_cho_them_khoi"` — vì khối này không hề được lưu MỚI (bị khử
trùng), nó không tính LÀ một khối "thật sự" đã nhận cho file
::why
Nhầm "khối bị khử trùng, không lưu thêm bản MỚI" VỚI "khối đó không
tính LÀ đã nhận" — nhưng hai việc này nằm Ở hai TRẠM tách biệt hoàn
toàn trong `nhanKhoiFile`.

Chỗ lệch: dòng `tt.khoiDaNhan.add(chiSoKhoi)` chạy VÔ điều kiện, ngay
SAU `luuKhoi`, bất kể `luuKhoi` có tạo bản ghi MỚI hay chỉ tăng
`soThamChieu`. Với `"video-e.mp4"` (`tongSoKhoi = 1`), chỉ số `0` được
đánh dấu nhận NGAY — `laHoanTat` trả về `true` VÀ hàm đi tới trạm ba,
gửi thông báo VÀ trả về `"hoan_tat_da_thong_bao"`.
::
:::
::::

::::code{#viet_nhan_khoi_file}
Hoàn thiện `nhanKhoiFile` — trạm một (lưu khối) VÀ trạm hai (đánh dấu
đã nhận) đã có sẵn. Còn thiếu trạm BA: nếu file CHƯA đủ khối thì trả
về `"dang_cho_them_khoi"`; ngược lại gửi thông báo `"file_san_sang"`
VÀO hàng đợi (theo `idFile`) VÀ trả về `"hoan_tat_da_thong_bao"`.

```typescript title=starter
function bamNoiDung(noiDung: string): string {
  let h = 0;
  for (let i = 0; i < noiDung.length; i++) {
    h = (h * 31 + noiDung.charCodeAt(i)) % 1000000007;
  }
  return "h" + h.toString(16);
}
function bam(khoa: string): number {
  let h = 0;
  for (let i = 0; i < khoa.length; i++) {
    h = (h * 31 + khoa.charCodeAt(i)) % 1000000007;
  }
  return h;
}

interface KhoiLuuTru { noiDung: string; soThamChieu: number; dsDia: string[]; }
interface KhoLuuTruKhoi { khoi: Map<string, KhoiLuuTru>; }
function taoKhoLuuTruKhoi(): KhoLuuTruKhoi { return { khoi: new Map() }; }
const DIA_CO_SAN = ["dia-1", "dia-2", "dia-3", "dia-4"];
function chonDiaSaoLuu(kho: KhoLuuTruKhoi): string[] {
  const batDau = kho.khoi.size % DIA_CO_SAN.length;
  const a = DIA_CO_SAN[batDau]!;
  const b = DIA_CO_SAN[(batDau + 1) % DIA_CO_SAN.length]!;
  const c = DIA_CO_SAN[(batDau + 2) % DIA_CO_SAN.length]!;
  return [a, b, c];
}
function luuKhoi(kho: KhoLuuTruKhoi, noiDung: string): string {
  const hash = bamNoiDung(noiDung);
  const hienCo = kho.khoi.get(hash);
  if (hienCo !== undefined) {
    hienCo.soThamChieu += 1;
  } else {
    kho.khoi.set(hash, { noiDung, soThamChieu: 1, dsDia: chonDiaSaoLuu(kho) });
  }
  return hash;
}

interface TinNhanHangDoi { khoa: string; noiDung: string; }
interface HangDoiThongBao { soPhanVung: number; phanVung: TinNhanHangDoi[][]; }
function taoHangDoiThongBao(soPhanVung: number): HangDoiThongBao {
  const phanVung: TinNhanHangDoi[][] = [];
  for (let i = 0; i < soPhanVung; i++) phanVung.push([]);
  return { soPhanVung, phanVung };
}
function guiThongBao(hd: HangDoiThongBao, khoa: string, noiDung: string): void {
  const idx = bam(khoa) % hd.soPhanVung;
  hd.phanVung[idx]!.push({ khoa, noiDung });
}
function demSoTinNhan(hd: HangDoiThongBao): number {
  return hd.phanVung.reduce((t, p) => t + p.length, 0);
}

interface TrangThaiUpload { idFile: string; tongSoKhoi: number; khoiDaNhan: Set<number>; }
function taoTrangThaiUpload(idFile: string, tongSoKhoi: number): TrangThaiUpload {
  return { idFile, tongSoKhoi, khoiDaNhan: new Set() };
}
function laHoanTat(tt: TrangThaiUpload): boolean {
  return tt.khoiDaNhan.size === tt.tongSoKhoi;
}

interface HeThongUpload {
  upload: Map<string, TrangThaiUpload>;
  kho: KhoLuuTruKhoi;
  hangDoi: HangDoiThongBao;
}
function taoHeThongUpload(soPhanVung: number): HeThongUpload {
  return { upload: new Map(), kho: taoKhoLuuTruKhoi(), hangDoi: taoHangDoiThongBao(soPhanVung) };
}
function batDauUpload(ht: HeThongUpload, idFile: string, tongSoKhoi: number): void {
  ht.upload.set(idFile, taoTrangThaiUpload(idFile, tongSoKhoi));
}
type KetQuaNhanKhoi = "dang_cho_them_khoi" | "hoan_tat_da_thong_bao";
function nhanKhoiFile(ht: HeThongUpload, idFile: string, chiSoKhoi: number, noiDungKhoi: string): KetQuaNhanKhoi {
  const tt = ht.upload.get(idFile)!;
  luuKhoi(ht.kho, noiDungKhoi);
  tt.khoiDaNhan.add(chiSoKhoi);
  ___
}

const htX = taoHeThongUpload(4);
batDauUpload(htX, "fx", 1);
console.log(nhanKhoiFile(htX, "fx", 0, "noi-dung-x"));
```

```typescript title=solution
function bamNoiDung(noiDung: string): string {
  let h = 0;
  for (let i = 0; i < noiDung.length; i++) {
    h = (h * 31 + noiDung.charCodeAt(i)) % 1000000007;
  }
  return "h" + h.toString(16);
}
function bam(khoa: string): number {
  let h = 0;
  for (let i = 0; i < khoa.length; i++) {
    h = (h * 31 + khoa.charCodeAt(i)) % 1000000007;
  }
  return h;
}

interface KhoiLuuTru { noiDung: string; soThamChieu: number; dsDia: string[]; }
interface KhoLuuTruKhoi { khoi: Map<string, KhoiLuuTru>; }
function taoKhoLuuTruKhoi(): KhoLuuTruKhoi { return { khoi: new Map() }; }
const DIA_CO_SAN = ["dia-1", "dia-2", "dia-3", "dia-4"];
function chonDiaSaoLuu(kho: KhoLuuTruKhoi): string[] {
  const batDau = kho.khoi.size % DIA_CO_SAN.length;
  const a = DIA_CO_SAN[batDau]!;
  const b = DIA_CO_SAN[(batDau + 1) % DIA_CO_SAN.length]!;
  const c = DIA_CO_SAN[(batDau + 2) % DIA_CO_SAN.length]!;
  return [a, b, c];
}
function luuKhoi(kho: KhoLuuTruKhoi, noiDung: string): string {
  const hash = bamNoiDung(noiDung);
  const hienCo = kho.khoi.get(hash);
  if (hienCo !== undefined) {
    hienCo.soThamChieu += 1;
  } else {
    kho.khoi.set(hash, { noiDung, soThamChieu: 1, dsDia: chonDiaSaoLuu(kho) });
  }
  return hash;
}

interface TinNhanHangDoi { khoa: string; noiDung: string; }
interface HangDoiThongBao { soPhanVung: number; phanVung: TinNhanHangDoi[][]; }
function taoHangDoiThongBao(soPhanVung: number): HangDoiThongBao {
  const phanVung: TinNhanHangDoi[][] = [];
  for (let i = 0; i < soPhanVung; i++) phanVung.push([]);
  return { soPhanVung, phanVung };
}
function guiThongBao(hd: HangDoiThongBao, khoa: string, noiDung: string): void {
  const idx = bam(khoa) % hd.soPhanVung;
  hd.phanVung[idx]!.push({ khoa, noiDung });
}
function demSoTinNhan(hd: HangDoiThongBao): number {
  return hd.phanVung.reduce((t, p) => t + p.length, 0);
}

interface TrangThaiUpload { idFile: string; tongSoKhoi: number; khoiDaNhan: Set<number>; }
function taoTrangThaiUpload(idFile: string, tongSoKhoi: number): TrangThaiUpload {
  return { idFile, tongSoKhoi, khoiDaNhan: new Set() };
}
function laHoanTat(tt: TrangThaiUpload): boolean {
  return tt.khoiDaNhan.size === tt.tongSoKhoi;
}

interface HeThongUpload {
  upload: Map<string, TrangThaiUpload>;
  kho: KhoLuuTruKhoi;
  hangDoi: HangDoiThongBao;
}
function taoHeThongUpload(soPhanVung: number): HeThongUpload {
  return { upload: new Map(), kho: taoKhoLuuTruKhoi(), hangDoi: taoHangDoiThongBao(soPhanVung) };
}
function batDauUpload(ht: HeThongUpload, idFile: string, tongSoKhoi: number): void {
  ht.upload.set(idFile, taoTrangThaiUpload(idFile, tongSoKhoi));
}
type KetQuaNhanKhoi = "dang_cho_them_khoi" | "hoan_tat_da_thong_bao";
function nhanKhoiFile(ht: HeThongUpload, idFile: string, chiSoKhoi: number, noiDungKhoi: string): KetQuaNhanKhoi {
  const tt = ht.upload.get(idFile)!;
  luuKhoi(ht.kho, noiDungKhoi);
  tt.khoiDaNhan.add(chiSoKhoi);
  if (!laHoanTat(tt)) return "dang_cho_them_khoi";
  guiThongBao(ht.hangDoi, idFile, "file_san_sang");
  return "hoan_tat_da_thong_bao";
}

const htX = taoHeThongUpload(4);
batDauUpload(htX, "fx", 1);
console.log(nhanKhoiFile(htX, "fx", 0, "noi-dung-x"));
```

```typescript title=test
const htT = taoHeThongUpload(4);
batDauUpload(htT, "video-a.mp4", 2);
const r1 = nhanKhoiFile(htT, "video-a.mp4", 0, "chung");
if (r1 !== "dang_cho_them_khoi") throw new Error("con thieu 1 khoi thi phai la dang_cho_them_khoi");
if (demSoTinNhan(htT.hangDoi) !== 0) throw new Error("chua hoan tat thi khong duoc gui thong bao nao");

const r2 = nhanKhoiFile(htT, "video-a.mp4", 1, "rieng-a");
if (r2 !== "hoan_tat_da_thong_bao") throw new Error("du het khoi thi phai la hoan_tat_da_thong_bao");
if (demSoTinNhan(htT.hangDoi) !== 1) throw new Error("hoan tat phai gui DUNG 1 thong bao vao hang doi");

batDauUpload(htT, "video-b.mp4", 2);
nhanKhoiFile(htT, "video-b.mp4", 0, "chung");
const r3 = nhanKhoiFile(htT, "video-b.mp4", 1, "rieng-b");
if (r3 !== "hoan_tat_da_thong_bao") throw new Error("video-b cung phai hoan tat va duoc thong bao");
if (demSoTinNhan(htT.hangDoi) !== 2) throw new Error("phai co THEM 1 thong bao nua, tong 2");
if (htT.kho.khoi.size !== 3) throw new Error("chi 3 khoi THAT SU (chung, rieng-a, rieng-b) -- khoi trung khong luu them");

batDauUpload(htT, "video-c.mp4", 3);
nhanKhoiFile(htT, "video-c.mp4", 2, "c2");
nhanKhoiFile(htT, "video-c.mp4", 0, "c0");
if (demSoTinNhan(htT.hangDoi) !== 2) throw new Error("video-c moi nhan 2/3 khoi, KHONG duoc gui thong bao");
const r4 = nhanKhoiFile(htT, "video-c.mp4", 1, "c1");
if (r4 !== "hoan_tat_da_thong_bao") throw new Error("khoi cuoi cung toi (du khong theo thu tu) phai kich hoat hoan tat");
if (demSoTinNhan(htT.hangDoi) !== 3) throw new Error("video-c hoan tat phai them thong bao thu 3");
```

:::hints
- kind: attention
  body: "Con thieu dung hai buoc: (1) neu !laHoanTat(tt) thi return 'dang_cho_them_khoi'; (2) nguoc lai goi guiThongBao(ht.hangDoi, idFile, 'file_san_sang') roi return 'hoan_tat_da_thong_bao'."
- kind: strategy
  body: "if (!laHoanTat(tt)) return 'dang_cho_them_khoi'; guiThongBao(ht.hangDoi, idFile, 'file_san_sang'); return 'hoan_tat_da_thong_bao';"
- kind: one-line
  body: "if (!laHoanTat(tt)) return \"dang_cho_them_khoi\"; guiThongBao(ht.hangDoi, idFile, \"file_san_sang\"); return \"hoan_tat_da_thong_bao\";"
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 7000
- tier: output
  match: contains
  expect: "hoan_tat_da_thong_bao"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Khối chia nhỏ, khử trùng theo hash, sao lưu bội số, đánh dấu đã nhận,
rồi báo tin qua hàng đợi phân vùng — năm mảnh RÁP đúng thứ tự thành
một luồng upload hoàn chỉnh. Quest "Hạ tầng dữ liệu quy mô lớn" đã
xong.
::::

::::reflect{#nghi-lai}
`nhanKhoiFile` không hề PHÁT minh gì mới — nó gọi đúng thứ tự các mảnh
đã xây RIÊNG lẻ Ở chín bài trước, VÀ giữ chúng tách BIỆT đúng mức cần
thiết: lưu khối (LUÔN chạy, có thể bị khử trùng ÂM thầm), đánh dấu đã
nhận (LUÔN chạy, độc lập với việc khối có MỚI hay không), VÀ báo tin
(chỉ chạy đúng MỘT lần, tại đúng thời điểm file chuyển từ "thiếu" sang
"đủ") — ba mối quan tâm khác nhau, không GỘP làm một chỉ vì chúng cùng
chạy trong một hàm.
::::

::::checkpoint{mastery=0.85}
::::
