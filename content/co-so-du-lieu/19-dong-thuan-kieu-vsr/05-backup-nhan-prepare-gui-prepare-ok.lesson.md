---
id: co-so-du-lieu.dong-thuan-kieu-vsr.backup-nhan-prepare-gui-prepare-ok
title: "Backup nhận Prepare, gửi PrepareOk"
summary: "xuLyPrepare ghi MỘT LogEntry{opNumber,bt} lấy từ ThongDiep vào backup.log, cập nhật backup.opNumber, RỒI gửi lại đúng một PrepareOk (tu=chiSo của backup, den=tu của Prepare gốc, viewNumber=CỦA CHÍNH BACKUP chứ không phải của Prepare) sau độ trễ ngẫu-nhiên-nhưng-tất-định. seed=9n: backup nhận Prepare tại t=0, PrepareOk tới primary tại t=5 -- backup.log.length=1 ngay sau khi xử lý."
locale: vi
track: co-so-du-lieu
module: dong-thuan-kieu-vsr
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [db.backup-nhan-prepare-gui-prepare-ok]
requires: [db.gui-prepare-toi-backup]
concepts: [db.backup-nhan-prepare-gui-prepare-ok]
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
Prepare vừa TỚI backup (bài trước). Backup KHÔNG được im lặng — nó
phải GHI lại VÀ báo cho primary BIẾT nó đã nhận.
::::

::::explain{#xu-ly-prepare}
`xuLyPrepare` ghi MỘT `LogEntry` (LẤY `opNumber`/`bt` từ `ThongDiep`
nhận được) VÀO `backup.log`, cập nhật `backup.opNumber`, RỒI gửi lại
đúng MỘT `PrepareOk` — `tu` LÀ chỉ số CỦA chính backup, `den` LÀ `tu`
CỦA Prepare gốc (tức LÀ primary), `viewNumber` LẤY từ `backup.
viewNumber` CỦA CHÍNH backup (không phải VIEW của Prepare nhận được —
backup LUÔN nói bằng VIEW của riêng nó):

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

function xuLyPrepare(hd: HangDoiMang, tt: TrangThai, dh: DongHoAo, backup: Replica, td: ThongDiep): void {
  const entry: LogEntry = { opNumber: td.opNumber!, bt: td.bt! };
  backup.log.push(entry);
  backup.opNumber = td.opNumber!;
  const doTre = soNguyenTrongKhoang(tt, 1, 20);
  guiThongDiep(hd, dh.hienTai + doTre, { loai: 'PrepareOk', tu: backup.chiSo, den: td.tu, viewNumber: backup.viewNumber, opNumber: td.opNumber! });
}

const tt = gieoHat(9n); const dh = taoDongHoAo(); const hd = taoHangDoiMang();
const backup1 = taoReplica(1, 3);
xuLyPrepare(hd, tt, dh, backup1, { loai: 'Prepare', tu: 0, den: 1, viewNumber: 0, opNumber: 1, bt: { id: 1, debitAccountId: 0, creditAccountId: 1, amount: 1000 } });
console.log("backup1.log.length:", backup1.log.length);
console.log("phan hoi:", JSON.stringify(hd.danhSach[0]));
```

```text title=readonly
backup1.log.length: 1
phan hoi: {"thoiDiem":5,"thuTuChen":0,"td":{"loai":"PrepareOk","tu":1,"den":0,"viewNumber":0,"opNumber":1}}
```

`backup1.log` GIỜ có ĐÚNG một entry (`opNumber=1`) — VÀ hàng đợi CÓ
một `PrepareOk` gửi VỀ, đến lúc `t=5` (seed `9n`), `tu=1` (backup),
`den=0` (primary).
::::

::::example{#viewnumber-cua-ai}
`viewNumber` trong `PrepareOk` LẤY từ `backup.viewNumber`, KHÔNG
phải `td.viewNumber` — vì SAU này (bài 9-12), một backup CÓ thể ĐÃ
chuyển sang view MỚI trong khi MỘT Prepare cũ (view TRƯỚC) vẫn còn
đang "bay" trên mạng. Backup PHẢI trả lời BẰNG view CỦA CHÍNH nó —
đây LÀ cách primary (sau NÀY) phát hiện nó ĐÃ lạc hậu view.
::::

::::predict{#doan-hai-prepare-lien-tiep commitOnce}
Gọi `xuLyPrepare` HAI lần liên TIẾP trên CÙNG một `backup` (CÙNG
`hd`, `tt`, `dh`), với HAI `Prepare` khác nhau (`opNumber=1` RỒI
`opNumber=2`). SAU cả hai lần, `backup.log.length` LÀ bao NHIÊU?
:::opt{correct}
`2` — MỖI lần gọi ĐỀU đẩy thêm MỘT entry mới vào `backup.log` (dòng
`backup.log.push(entry)`), không hề kiểm tra TRÙNG lặp — gọi hai lần
với hai `opNumber` khác nhau cho HAI entry riêng biệt
:::
:::opt
`1` — `xuLyPrepare` chỉ nên GHI đè entry mới nhất, không tích LŨY
::why
Trực giác NÀY hợp LÝ về mặt Ý nghĩa CUỐI cùng (log chỉ CẦN trạng thái
mới nhất) — nhưng SAI VỚI code THẬT Ở bài NÀY: `log` LÀ một MẢNG các
bút toán THEO thứ tự, không phải MỘT giá trị duy nhất bị ghi ĐÈ.

Chỗ lệch: `backup.log.push(entry)` LUÔN thêm PHẦN tử mới, không bao
GIỜ thay thế phần tử CŨ. Đây chính LÀ Ý nghĩa của "log": một LỊCH sử
đầy đủ các bút toán ĐÃ nhận, theo đúng thứ tự `opNumber` — nếu ghi đè,
sẽ MẤT khả năng phát hiện KHOẢNG trống (bài 13, State Transfer).
::
:::
::::

::::code{#viet_xu_ly_prepare}
Hoàn thiện `xuLyPrepare` — sau khi ghi log VÀ cập nhật `opNumber`,
gửi lại một `PrepareOk` tới `td.tu` (primary).

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

function xuLyPrepare(hd: HangDoiMang, tt: TrangThai, dh: DongHoAo, backup: Replica, td: ThongDiep): void {
  const entry: LogEntry = { opNumber: td.opNumber!, bt: td.bt! };
  backup.log.push(entry);
  backup.opNumber = td.opNumber!;
  const doTre = soNguyenTrongKhoang(tt, 1, 20);
  ___
}

const tt = gieoHat(9n); const dh = taoDongHoAo(); const hd = taoHangDoiMang();
const backup1 = taoReplica(1, 3);
xuLyPrepare(hd, tt, dh, backup1, { loai: 'Prepare', tu: 0, den: 1, viewNumber: 0, opNumber: 1, bt: { id: 1, debitAccountId: 0, creditAccountId: 1, amount: 1000 } });
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

function xuLyPrepare(hd: HangDoiMang, tt: TrangThai, dh: DongHoAo, backup: Replica, td: ThongDiep): void {
  const entry: LogEntry = { opNumber: td.opNumber!, bt: td.bt! };
  backup.log.push(entry);
  backup.opNumber = td.opNumber!;
  const doTre = soNguyenTrongKhoang(tt, 1, 20);
  guiThongDiep(hd, dh.hienTai + doTre, { loai: 'PrepareOk', tu: backup.chiSo, den: td.tu, viewNumber: backup.viewNumber, opNumber: td.opNumber! });
}

const tt = gieoHat(9n); const dh = taoDongHoAo(); const hd = taoHangDoiMang();
const backup1 = taoReplica(1, 3);
xuLyPrepare(hd, tt, dh, backup1, { loai: 'Prepare', tu: 0, den: 1, viewNumber: 0, opNumber: 1, bt: { id: 1, debitAccountId: 0, creditAccountId: 1, amount: 1000 } });
console.log(hd.danhSach.length);
```

```typescript title=test
const tt2 = gieoHat(9n); const dh2 = taoDongHoAo(); const hd2 = taoHangDoiMang();
const backup2 = taoReplica(2, 3);
backup2.viewNumber = 5; // backup dang o view RIENG cua no (5) -- khac view cua Prepare goc (0)
const btGoc: ButToan = { id: 1, debitAccountId: 0, creditAccountId: 1, amount: 1000 };
xuLyPrepare(hd2, tt2, dh2, backup2, { loai: 'Prepare', tu: 0, den: 2, viewNumber: 0, opNumber: 1, bt: btGoc });
if (backup2.log.length !== 1) throw new Error("backup phai ghi DUNG mot entry vao log cua CHINH no");
if (backup2.log[0]!.opNumber !== 1) throw new Error("entry ghi vao log phai co dung opNumber cua Prepare");
if (backup2.log[0]!.bt !== btGoc) throw new Error("entry phai chua dung ButToan da nhan");
if (backup2.opNumber !== 1) throw new Error("backup.opNumber phai duoc CAP NHAT theo opNumber vua nhan");
if (hd2.danhSach.length !== 1) throw new Error("phai gui DUNG MOT thong diep PrepareOk phan hoi");
const phanHoi = hd2.danhSach[0]!;
if (phanHoi.td.loai !== 'PrepareOk') throw new Error("thong diep phan hoi phai co loai='PrepareOk'");
if (phanHoi.td.tu !== 2) throw new Error("tu phai la chiSo cua BACKUP (nguoi gui phan hoi), khong phai primary");
if (phanHoi.td.den !== 0) throw new Error("den phai la chiSo cua PRIMARY (td.tu cua Prepare goc)");
if (phanHoi.td.opNumber !== 1) throw new Error("PrepareOk phai mang dung opNumber cua Prepare da xu ly");
if (phanHoi.td.viewNumber !== 5) throw new Error("viewNumber cua PrepareOk phai la viewNumber CUA BACKUP (backup.viewNumber), khong phai cua Prepare goc");
if (phanHoi.thoiDiem !== 5) throw new Error("thoi diem phan hoi phai la dh.hienTai + do tre (seed=9n: 5)");
```

:::hints
- kind: attention
  body: "Goi guiThongDiep de gui PrepareOk: tu=backup.chiSo, den=td.tu, viewNumber=backup.viewNumber, opNumber=td.opNumber -- mot loi goi."
- kind: strategy
  body: "guiThongDiep(hd, dh.hienTai + doTre, { loai: 'PrepareOk', tu: backup.chiSo, den: td.tu, viewNumber: backup.viewNumber, opNumber: td.opNumber! });"
- kind: one-line
  body: "guiThongDiep(hd, dh.hienTai + doTre, { loai: 'PrepareOk', tu: backup.chiSo, den: td.tu, viewNumber: backup.viewNumber, opNumber: td.opNumber! });"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
PrepareOk đang bay VỀ primary. Nhưng MỘT PrepareOk chưa đủ — primary
cần ĐẾM tới quorum trước khi dám commit.
::::

::::reflect{#nghi-lai}
`xuLyPrepare` LÀ nửa BÊN kia của bài 4 — cùng hạ tầng `HangDoiMang`,
đổi hướng "PHÁT" thành "NHẬN RỒI phát lại". Chi tiết `backup.viewNumber`
(không phải `td.viewNumber`) LÀ mầm mống của TOÀN bộ cơ chế phát hiện
lạc HẬU sẽ khai thác Ở bài 9-12 — một backup LUÔN "nói" bằng góc nhìn
của CHÍNH nó.
::::

::::checkpoint{mastery=0.85}
::::
