---
id: co-so-du-lieu.dong-thuan-kieu-vsr.gui-prepare-toi-backup
title: "Primary gửi Prepare tới backup"
summary: "guiPrepareToiCacBackup dùng HangDoiMang -- HangDoi (q17) đổi trường nhan:string thành td:ThongDiep (thông điệp đầy đủ: loai/tu/den/viewNumber/opNumber/bt) -- gửi MỖI Prepare tới MỘT backup với độ trễ ngẫu-nhiên-nhưng-tất-định (soNguyenTrongKhoang, q17). seed=5n, primary=replica0 gửi tới backup1 và backup2: thời điểm đến lần lượt là 9 và 18 -- hai con số này lặp lại y hệt mỗi lần chạy, đúng tinh thần q17 bài 7 áp cho 'gói tin' giữa các replica."
locale: vi
track: co-so-du-lieu
module: dong-thuan-kieu-vsr
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [db.gui-prepare-toi-backup]
requires: [db.client-gui-yeu-cau-toi-primary]
concepts: [db.gui-prepare-toi-backup]
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
Primary đã ghi bút toán CỤC bộ (bài trước). Nhưng backup Ở một tiến
trình KHÁC — làm sao nó BIẾT? q18 bài 7 đã dùng `HangDoi` (q17) để mô
phỏng gói tin ĐẾN trễ khác nhau. Giờ dùng LẠI chính nó cho "mạng ẢO"
giữa các replica.
::::

::::explain{#hang-doi-mang}
`HangDoi` mà q17 dùng cho "nhãn sự kiện" (`nhan: string`) giờ mang
một THÔNG ĐIỆP đầy đủ (`td: ThongDiep` — nơi gửi `tu`, nơi nhận
`den`, `loai`, `viewNumber`, VÀ nội dung `opNumber`/`bt`) — CÙNG cấu
trúc `HangDoi`/`SuKien` (thời điểm + `thuTuChen` để hoà tất định,
q17 bài 6), chỉ khác kiểu dữ liệu MANG theo. `guiPrepareToiCacBackup`
gửi một `Prepare` tới MỖI backup, độ trễ NGẪU-NHIÊN-NHƯNG-TẤT-ĐỊNH
(`soNguyenTrongKhoang`, q17):

```typescript title=readonly
const MASK64 = (1n << 64n) - 1n;
interface TrangThai { s0: bigint; s1: bigint; }
function gieoHat(seed: bigint): TrangThai {
  let s0 = seed & MASK64;
  if (s0 === 0n) s0 = 0x9e3779b97f4a7c15n;
  let s1 = (seed * 6364136223846793005n + 1442695040888963407n) & MASK64;
  if (s1 === 0n) s1 = 0xbf58476d1ce4e5b9n;
  return { s0, s1 };
}
function soTiepTheo(tt: TrangThai): bigint {
  let s1 = tt.s0;
  const s0 = tt.s1;
  const ketQua = (s1 + s0) & MASK64;
  tt.s0 = s0;
  s1 ^= (s1 << 23n) & MASK64;
  s1 ^= s1 >> 17n;
  s1 ^= s0 ^ (s0 >> 26n);
  tt.s1 = s1 & MASK64;
  return ketQua;
}
function soNguyenTrongKhoang(tt: TrangThai, min: number, max: number): number {
  return min + Number(soTiepTheo(tt) % BigInt(max - min));
}
interface DongHoAo { hienTai: number; }
function taoDongHoAo(): DongHoAo { return { hienTai: 0 }; }

interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
interface LogEntry { opNumber: number; bt: ButToan; }
interface Replica {
  chiSo: number; tongSo: number; viewNumber: number;
  status: 'normal' | 'view-change' | 'recovering';
  log: LogEntry[]; opNumber: number; commitNumber: number;
}
function taoReplica(chiSo: number, tongSo: number): Replica {
  return { chiSo, tongSo, viewNumber: 0, status: 'normal', log: [], opNumber: 0, commitNumber: 0 };
}

interface ThongDiep {
  loai: 'Prepare' | 'PrepareOk' | 'Commit' | 'StartViewChange' | 'DoViewChange' | 'StartView' | 'YeuCauDongBo' | 'PhanHoiDongBo';
  tu: number; den: number; viewNumber: number;
  opNumber?: number; bt?: ButToan; commitNumber?: number; log?: LogEntry[];
}
interface SuKienMang { thoiDiem: number; thuTuChen: number; td: ThongDiep; }
interface HangDoiMang { danhSach: SuKienMang[]; demChen: number; }
function taoHangDoiMang(): HangDoiMang { return { danhSach: [], demChen: 0 }; }
function guiThongDiep(hd: HangDoiMang, thoiDiem: number, td: ThongDiep): void {
  hd.danhSach.push({ thoiDiem, thuTuChen: hd.demChen, td });
  hd.demChen++;
}

function guiPrepareToiCacBackup(
  hd: HangDoiMang, tt: TrangThai, dh: DongHoAo,
  primary: Replica, entry: LogEntry, cacChiSoBackup: number[],
): void {
  for (const denChiSo of cacChiSoBackup) {
    const doTre = soNguyenTrongKhoang(tt, 1, 20);
    guiThongDiep(hd, dh.hienTai + doTre, {
      loai: 'Prepare', tu: primary.chiSo, den: denChiSo,
      viewNumber: primary.viewNumber, opNumber: entry.opNumber, bt: entry.bt,
    });
  }
}

const tt = gieoHat(5n);
const dh = taoDongHoAo();
const hd = taoHangDoiMang();
const primary = taoReplica(0, 3);
const entry: LogEntry = { opNumber: 1, bt: { id: 1, debitAccountId: 0, creditAccountId: 1, amount: 1000 } };
primary.log.push(entry); primary.opNumber = 1;
guiPrepareToiCacBackup(hd, tt, dh, primary, entry, [1, 2]);
for (const sk of hd.danhSach) console.log(`den=replica${sk.td.den} thoiDiem=${sk.thoiDiem} opNumber=${sk.td.opNumber}`);
```

```text title=readonly
den=replica1 thoiDiem=9 opNumber=1
den=replica2 thoiDiem=18 opNumber=1
```

Hai `Prepare` được gửi Ở CÙNG thời điểm (`dh.hienTai = 0`), nhưng
mỗi backup NHẬN được một ĐỘ trễ riêng RÚT từ `tt` — `backup1` ĐẾN
lúc `9`, `backup2` đến lúc `18`. `seed=5n` LUÔN cho lại ĐÚNG hai con
số NÀY.
::::

::::example{#khong-phai-goi-tin-that}
Bài 7 (q18) dùng `HangDoi` cho "gói tin" MANG một nhãn `string`
(`"goi0"`). Ở ĐÂY, "gói tin" LÀ một THÔNG điệp VSR thật — mang đủ
`opNumber` VÀ `bt` để backup có thể GHI log của chính nó KHI nhận
được (bài 5). Hạ tầng KHÔNG đổi — chỉ NỘI dung mang theo phong PHÚ
hơn.
::::

::::predict{#doan-doi-thu-tu-backup commitOnce}
Nếu ĐỔI `cacChiSoBackup` từ `[1, 2]` thành `[2, 1]` (CÙNG seed
`5n`) — thời điểm ĐẾN của `replica2` VÀ `replica1` CÓ đổi CHỖ cho
nhau không (`replica2` giờ đến LÚC `9`, `replica1` đến lúc `18`)?
:::opt{correct}
CÓ — `soNguyenTrongKhoang` RÚT số THEO đúng thứ tự vòng `for`, không
quan tâm "TÊN" của backup; `denChiSo` ĐẦU tiên trong mảng LUÔN nhận
độ trễ ĐẦU tiên rút RA (`9`), bất kể ĐÓ LÀ chỉ số NÀO
:::
:::opt
KHÔNG — độ trễ PHẢI gắn liền với TỪNG `chiSo` cụ thể (backup2 LUÔN
nhận `18`, dù đứng Ở vị trí NÀO trong mảng)
::why
Trực giác NÀY giả định `soNguyenTrongKhoang` "biết" NÓ đang tính CHO
ai — nhưng hàm KHÔNG hề nhận `denChiSo` LÀM tham số ẢNH hưởng tới kết
quả, nó chỉ đọc TỪ `tt` theo đúng THỨ tự gọi.

Chỗ lệch: độ trễ gắn VỚI thứ tự DUYỆT vòng `for`, không gắn với GIÁ
trị `denChiSo`. Đây LÀ lý do THỨ tự phần tử trong `cacChiSoBackup`
(hay `cacBackupConSong` Ở BOSS, bài 14) có thể ẢNH hưởng dấu vết thực
thi — một chi tiết cần NHỚ khi so sánh hai lần chạy.
::
:::
::::

::::code{#viet_gui_prepare}
Hoàn thiện `guiPrepareToiCacBackup` — với MỖI backup, gửi một thông
điệp `Prepare` tới thời điểm `dh.hienTai + doTre`.

```typescript title=starter
const MASK64 = (1n << 64n) - 1n;
interface TrangThai { s0: bigint; s1: bigint; }
function gieoHat(seed: bigint): TrangThai {
  let s0 = seed & MASK64;
  if (s0 === 0n) s0 = 0x9e3779b97f4a7c15n;
  let s1 = (seed * 6364136223846793005n + 1442695040888963407n) & MASK64;
  if (s1 === 0n) s1 = 0xbf58476d1ce4e5b9n;
  return { s0, s1 };
}
function soTiepTheo(tt: TrangThai): bigint {
  let s1 = tt.s0;
  const s0 = tt.s1;
  const ketQua = (s1 + s0) & MASK64;
  tt.s0 = s0;
  s1 ^= (s1 << 23n) & MASK64;
  s1 ^= s1 >> 17n;
  s1 ^= s0 ^ (s0 >> 26n);
  tt.s1 = s1 & MASK64;
  return ketQua;
}
function soNguyenTrongKhoang(tt: TrangThai, min: number, max: number): number {
  return min + Number(soTiepTheo(tt) % BigInt(max - min));
}
interface DongHoAo { hienTai: number; }
function taoDongHoAo(): DongHoAo { return { hienTai: 0 }; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
interface LogEntry { opNumber: number; bt: ButToan; }
interface Replica {
  chiSo: number; tongSo: number; viewNumber: number;
  status: 'normal' | 'view-change' | 'recovering';
  log: LogEntry[]; opNumber: number; commitNumber: number;
}
function taoReplica(chiSo: number, tongSo: number): Replica {
  return { chiSo, tongSo, viewNumber: 0, status: 'normal', log: [], opNumber: 0, commitNumber: 0 };
}
interface ThongDiep {
  loai: 'Prepare' | 'PrepareOk' | 'Commit' | 'StartViewChange' | 'DoViewChange' | 'StartView' | 'YeuCauDongBo' | 'PhanHoiDongBo';
  tu: number; den: number; viewNumber: number;
  opNumber?: number; bt?: ButToan; commitNumber?: number; log?: LogEntry[];
}
interface SuKienMang { thoiDiem: number; thuTuChen: number; td: ThongDiep; }
interface HangDoiMang { danhSach: SuKienMang[]; demChen: number; }
function taoHangDoiMang(): HangDoiMang { return { danhSach: [], demChen: 0 }; }
function guiThongDiep(hd: HangDoiMang, thoiDiem: number, td: ThongDiep): void {
  hd.danhSach.push({ thoiDiem, thuTuChen: hd.demChen, td });
  hd.demChen++;
}

function guiPrepareToiCacBackup(
  hd: HangDoiMang, tt: TrangThai, dh: DongHoAo,
  primary: Replica, entry: LogEntry, cacChiSoBackup: number[],
): void {
  for (const denChiSo of cacChiSoBackup) {
    const doTre = soNguyenTrongKhoang(tt, 1, 20);
    ___
  }
}

const tt = gieoHat(5n); const dh = taoDongHoAo(); const hd = taoHangDoiMang();
const primary = taoReplica(0, 3);
const entry: LogEntry = { opNumber: 1, bt: { id: 1, debitAccountId: 0, creditAccountId: 1, amount: 1000 } };
guiPrepareToiCacBackup(hd, tt, dh, primary, entry, [1, 2]);
console.log(hd.danhSach.length);
```

```typescript title=solution
const MASK64 = (1n << 64n) - 1n;
interface TrangThai { s0: bigint; s1: bigint; }
function gieoHat(seed: bigint): TrangThai {
  let s0 = seed & MASK64;
  if (s0 === 0n) s0 = 0x9e3779b97f4a7c15n;
  let s1 = (seed * 6364136223846793005n + 1442695040888963407n) & MASK64;
  if (s1 === 0n) s1 = 0xbf58476d1ce4e5b9n;
  return { s0, s1 };
}
function soTiepTheo(tt: TrangThai): bigint {
  let s1 = tt.s0;
  const s0 = tt.s1;
  const ketQua = (s1 + s0) & MASK64;
  tt.s0 = s0;
  s1 ^= (s1 << 23n) & MASK64;
  s1 ^= s1 >> 17n;
  s1 ^= s0 ^ (s0 >> 26n);
  tt.s1 = s1 & MASK64;
  return ketQua;
}
function soNguyenTrongKhoang(tt: TrangThai, min: number, max: number): number {
  return min + Number(soTiepTheo(tt) % BigInt(max - min));
}
interface DongHoAo { hienTai: number; }
function taoDongHoAo(): DongHoAo { return { hienTai: 0 }; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
interface LogEntry { opNumber: number; bt: ButToan; }
interface Replica {
  chiSo: number; tongSo: number; viewNumber: number;
  status: 'normal' | 'view-change' | 'recovering';
  log: LogEntry[]; opNumber: number; commitNumber: number;
}
function taoReplica(chiSo: number, tongSo: number): Replica {
  return { chiSo, tongSo, viewNumber: 0, status: 'normal', log: [], opNumber: 0, commitNumber: 0 };
}
interface ThongDiep {
  loai: 'Prepare' | 'PrepareOk' | 'Commit' | 'StartViewChange' | 'DoViewChange' | 'StartView' | 'YeuCauDongBo' | 'PhanHoiDongBo';
  tu: number; den: number; viewNumber: number;
  opNumber?: number; bt?: ButToan; commitNumber?: number; log?: LogEntry[];
}
interface SuKienMang { thoiDiem: number; thuTuChen: number; td: ThongDiep; }
interface HangDoiMang { danhSach: SuKienMang[]; demChen: number; }
function taoHangDoiMang(): HangDoiMang { return { danhSach: [], demChen: 0 }; }
function guiThongDiep(hd: HangDoiMang, thoiDiem: number, td: ThongDiep): void {
  hd.danhSach.push({ thoiDiem, thuTuChen: hd.demChen, td });
  hd.demChen++;
}

function guiPrepareToiCacBackup(
  hd: HangDoiMang, tt: TrangThai, dh: DongHoAo,
  primary: Replica, entry: LogEntry, cacChiSoBackup: number[],
): void {
  for (const denChiSo of cacChiSoBackup) {
    const doTre = soNguyenTrongKhoang(tt, 1, 20);
    guiThongDiep(hd, dh.hienTai + doTre, {
      loai: 'Prepare', tu: primary.chiSo, den: denChiSo,
      viewNumber: primary.viewNumber, opNumber: entry.opNumber, bt: entry.bt,
    });
  }
}

const tt = gieoHat(5n); const dh = taoDongHoAo(); const hd = taoHangDoiMang();
const primary = taoReplica(0, 3);
const entry: LogEntry = { opNumber: 1, bt: { id: 1, debitAccountId: 0, creditAccountId: 1, amount: 1000 } };
guiPrepareToiCacBackup(hd, tt, dh, primary, entry, [1, 2]);
console.log(hd.danhSach.length);
```

```typescript title=test
const tt2 = gieoHat(5n); const dh2 = taoDongHoAo(); const hd2 = taoHangDoiMang();
const primary2 = taoReplica(0, 3);
primary2.opNumber = 7; // co tinh KHAC opNumber cua entry
const entry2: LogEntry = { opNumber: 1, bt: { id: 1, debitAccountId: 0, creditAccountId: 1, amount: 1000 } };
guiPrepareToiCacBackup(hd2, tt2, dh2, primary2, entry2, [1, 2]);
if (hd2.danhSach.length !== 2) throw new Error("phai gui Prepare toi DUNG so backup trong danh sach");
if (hd2.danhSach[0]!.thoiDiem !== 9) throw new Error("thoi diem den phai la dh.hienTai + do tre (seed=5n, backup dau tien: 9)");
if (hd2.danhSach[1]!.thoiDiem !== 18) throw new Error("thoi diem den phai la dh.hienTai + do tre (seed=5n, backup thu hai: 18)");
for (const sk of hd2.danhSach) {
  if (sk.td.loai !== 'Prepare') throw new Error("moi thong diep gui di phai co loai='Prepare'");
  if (sk.td.tu !== 0) throw new Error("tu phai la chiSo cua primary");
  if (sk.td.opNumber !== 1) throw new Error("opNumber phai lay TU entry (1), KHONG PHAI tu primary.opNumber (7)");
  if (sk.td.viewNumber !== 0) throw new Error("viewNumber phai lay tu primary.viewNumber");
}
if (hd2.danhSach[0]!.td.den !== 1 || hd2.danhSach[1]!.td.den !== 2) throw new Error("den phai dung chiSo tung backup trong danh sach");
```

:::hints
- kind: attention
  body: "Goi guiThongDiep voi thoiDiem = dh.hienTai + doTre, loai='Prepare', tu=primary.chiSo, den=denChiSo, opNumber/bt lay TU entry -- mot loi goi guiThongDiep."
- kind: strategy
  body: "guiThongDiep(hd, dh.hienTai + doTre, { loai: 'Prepare', tu: primary.chiSo, den: denChiSo, viewNumber: primary.viewNumber, opNumber: entry.opNumber, bt: entry.bt });"
- kind: one-line
  body: "guiThongDiep(hd, dh.hienTai + doTre, { loai: 'Prepare', tu: primary.chiSo, den: denChiSo, viewNumber: primary.viewNumber, opNumber: entry.opNumber, bt: entry.bt });"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "2"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Prepare đã LÊN đường. Bên kia, backup NHẬN nó — VÀ phải làm HAI việc:
ghi log CỦA CHÍNH nó, RỒI báo lại cho primary.
::::

::::reflect{#nghi-lai}
`guiPrepareToiCacBackup` KHÔNG giới thiệu cấu TRÚC mới — nó tái sử
dụng `HangDoi` (q17) VÀ `soNguyenTrongKhoang` (q17) y HỆT bài 7 (q18)
đã làm cho gói tin, chỉ đổi Ý nghĩa "nhãn sự kiện" thành "thông điệp
đồng thuận". Đây chính LÀ điều làm q19 khả THI trong khuôn khổ mô
phỏng thuần TypeScript: KHÔNG cần mạng thật, chỉ cần MỘT hàng đợi
tất định.
::::

::::checkpoint{mastery=0.8}
::::
