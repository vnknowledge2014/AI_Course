---
id: thiet-ke-he-thong.chong-chiu-va-khoi-phuc.backup-restore-rpo-rto-va-xac-thuc
title: "Backup & restore: RPO/RTO, và verify backup THẬT SỰ khôi phục được"
summary: "xacThucBackup so sanh tinhChecksumDonGian(noiDungKhoiPhucDuoc) voi checksumGoc luu san trong BanSaoLuu -- mot ban ghi backup TON TAI DAY DU (khong undefined) VAN co the xac thuc THAT BAI (false) neu du lieu khoi phuc LECH mot ky tu (bit rot), chung minh 'file con' khac 'khoi phuc dung'. tinhRpoMs(cacBanSaoLuu, thoiDiemSuCoMs) tim ban sao GAN NHAT co thoiDiemMs <= thoiDiemSuCoMs, RPO = thoiDiemSuCoMs tru thoiDiem do -- chuoi backup moi gio (0,3600000,7200000,10800000ms), su co luc 9000000ms cho RPO=1800000ms (30 phut du lieu co the mat); su co TRUOC ca backup dau tien tra ve undefined (khong co gi de khoi phuc)."
locale: vi
track: thiet-ke-he-thong
module: chong-chiu-va-khoi-phuc
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [sd.backup-restore-rpo-rto-va-xac-thuc]
requires: [sd.rate-limit-bao-ve-downstream]
concepts: [sd.backup-restore-rpo-rto-va-xac-thuc]
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
Năm bài trước dạy cách CHỊU đựng một dịch vụ hạ nguồn lỗi tạm THỜI — mạch
mở, retry, cô lập, dự phòng, tự kìm tốc độ. Nhưng có một loại sự cố không
"tạm thời" chút nào: mất SẠCH dữ liệu. Khi ĐÓ, câu hỏi không còn LÀ "chịu
đựng được không" — mà LÀ "khôi phục lại được BAO NHIÊU, VÀ mất bao lâu".
::::

::::explain{#xac-thuc-backup}
Một bản sao lưu (`BanSaoLuu`) LƯU lại `checksumGoc` — dấu vân TAY của nội
dung TẠI thời điểm sao lưu, tính bằng `tinhChecksumDonGian` (tổng mã KÝ tự,
đơn giản NHƯNG đủ để phát hiện SAI lệch trong bài NÀY). `xacThucBackup`
THỬ khôi phục RỒI so sánh checksum — bản ghi backup TỒN tại KHÔNG hề đồng
nghĩa với khôi phục ĐÚNG:

```typescript title=readonly
function tinhChecksumDonGian(noiDung: string): number {
  let tong = 0;
  for (let i = 0; i < noiDung.length; i++) tong += noiDung.charCodeAt(i);
  return tong;
}
interface BanSaoLuu { thoiDiemMs: number; checksumGoc: number; }
function taoBanSaoLuu(thoiDiemMs: number, noiDungGoc: string): BanSaoLuu {
  return { thoiDiemMs, checksumGoc: tinhChecksumDonGian(noiDungGoc) };
}
function xacThucBackup(banSaoLuu: BanSaoLuu, noiDungKhoiPhucDuoc: string): boolean {
  return tinhChecksumDonGian(noiDungKhoiPhucDuoc) === banSaoLuu.checksumGoc;
}

const noiDungGoc = "don-hang-4471:tong=250000";
const backupTot = taoBanSaoLuu(1000, noiDungGoc);
console.log("backup TOT -- checksum goc:", backupTot.checksumGoc);
console.log("khoi phuc dung noi dung goc, xac thuc:", xacThucBackup(backupTot, noiDungGoc));

// backup HONG: BAN GHI van "ton tai" day du (co checksumGoc, khong undefined)
// nhung du lieu THUC SU khoi phuc duoc lai lech 1 ky tu (vi du bit rot tren dia)
const noiDungBiHong = "don-hang-4471:tong=250001";
const backupHong = taoBanSaoLuu(1000, noiDungGoc);
console.log("backup HONG -- ban ghi van 'ton tai' (co checksumGoc):", backupHong.checksumGoc !== undefined);
console.log("nhung khoi phuc RA du lieu lech 1 ky tu, xac thuc:", xacThucBackup(backupHong, noiDungBiHong));
```

```text title=readonly
backup TOT -- checksum goc: 1887
khoi phuc dung noi dung goc, xac thuc: true
backup HONG -- ban ghi van 'ton tai' (co checksumGoc): true
nhung khoi phuc RA du lieu lech 1 ky tu, xac thuc: false
```

`backupHong` KHÔNG hề khác `backupTot` VỀ mặt CẤU trúc — cả hai đều LÀ một
`BanSaoLuu` hoàn CHỈNH, `checksumGoc` đều LÀ một con số hợp LỆ. Sự khác
biệt CHỈ lộ ra khi THỰC SỰ thử khôi phục: `xacThucBackup` phát hiện dữ
liệu khôi phục được (`noiDungBiHong`) KHÔNG khớp checksum đã lưu — một
backup "có mặt" trên đĩa KHÔNG chứng minh được gì về khả năng dùng ĐƯỢC
nó khi cần.
::::

::::example{#rpo-tu-chuoi-backup}
`RPO` (Recovery Point Objective) LÀ lượng dữ liệu TỐI ĐA có thể mất, tính
bằng khoảng THỜI gian từ lần backup GẦN nhất tới lúc sự cố xảy ra.
`tinhRpoMs` tìm bản sao lưu GẦN nhất TRƯỚC (hoặc đúng lúc) sự cố, rồi trừ
mốc thời gian:

```typescript title=readonly
interface BanSaoLuuTheoThoiGian { thoiDiemMs: number; }
function tinhRpoMs(cacBanSaoLuu: BanSaoLuuTheoThoiGian[], thoiDiemSuCoMs: number): number | undefined {
  const truoc = cacBanSaoLuu.filter((b) => b.thoiDiemMs <= thoiDiemSuCoMs);
  if (truoc.length === 0) return undefined;
  let ganNhat = truoc[0]!;
  for (const b of truoc) if (b.thoiDiemMs > ganNhat.thoiDiemMs) ganNhat = b;
  return thoiDiemSuCoMs - ganNhat.thoiDiemMs;
}

// backup moi GIO, tinh bang mili-giay: 0h, 1h, 2h, 3h
const chuoiBackup: BanSaoLuuTheoThoiGian[] = [
  { thoiDiemMs: 0 },
  { thoiDiemMs: 3600000 },
  { thoiDiemMs: 7200000 },
  { thoiDiemMs: 10800000 },
];
console.log("su co luc 9000000ms (2h30p), RPO:", tinhRpoMs(chuoiBackup, 9000000));
console.log("su co DUNG luc co mot backup (10800000ms), RPO:", tinhRpoMs(chuoiBackup, 10800000));
console.log("su co TRUOC CA backup dau tien (-100ms):", tinhRpoMs(chuoiBackup, -100));

interface CauHinhKhoiPhuc { thoiGianPhatHienMs: number; thoiGianTaiBackupMs: number; thoiGianChayScriptMs: number; }
function tinhRtoUocLuongMs(c: CauHinhKhoiPhuc): number {
  return c.thoiGianPhatHienMs + c.thoiGianTaiBackupMs + c.thoiGianChayScriptMs;
}
const cauHinhRto: CauHinhKhoiPhuc = { thoiGianPhatHienMs: 60000, thoiGianTaiBackupMs: 300000, thoiGianChayScriptMs: 120000 };
console.log("RTO uoc luong (phat hien + tai backup + chay script):", tinhRtoUocLuongMs(cauHinhRto), "ms");
```

```text title=readonly
su co luc 9000000ms (2h30p), RPO: 1800000
su co DUNG luc co mot backup (10800000ms), RPO: 0
su co TRUOC CA backup dau tien (-100ms): undefined
RTO uoc luong (phat hien + tai backup + chay script): 480000 ms
```

Sự cố Ở `9000000`ms (`2h30`): bản sao lưu GẦN nhất TRƯỚC đó LÀ Ở
`7200000`ms (`2h`) — RPO LÀ `1800000`ms, đúng `30` phút dữ liệu CÓ thể đã
mất VĨNH viễn (mọi thay đổi SAU lần backup đó, TRƯỚC khi sự cố xảy ra).
`RTO` (Recovery Time Objective, KHÁC RPO) LÀ thời gian TỐI đa để khôi phục
XONG — Ở đây LÀ TỔNG ba giai đoạn: phát hiện sự cố, tải bản backup VỀ,
rồi chạy script khôi phục, cộng LẠI ra `480000`ms (`8` phút).
::::

::::predict{#doan-backup-ton-tai-khong-dam-bao-dung commitOnce}
Biến `backupHong` (kiểu `BanSaoLuu`) LÀ một object HOÀN chỉnh, KHÔNG phải
`undefined` — nó có `checksumGoc` hợp lệ, giống HỆT một backup tốt. Gọi
`xacThucBackup(backupHong, noiDungBiHong)` VỚI `noiDungBiHong` lệch đúng
MỘT ký tự so với nội dung LÚC sao lưu. Kết quả trả về LÀ gì?

:::opt{correct}
`false` — `xacThucBackup` so checksum của `noiDungBiHong` VỚI
`checksumGoc` đã lưu; một ký tự khác LÀM `tinhChecksumDonGian` ra một
tổng KHÁC, nên checksum KHÔNG khớp, DÙ `backupHong` bản thân nó vẫn LÀ
một object đầy đủ
:::
:::opt
`true` — `backupHong` LÀ một `BanSaoLuu` hợp LỆ (đã được tạo thành công,
không phải `undefined`), nên quá trình sao lưu chắc chắn đã THÀNH công,
VÀ khôi phục từ nó phải ĐÚNG
::why
Nhầm "bản ghi backup TỒN tại VỀ mặt cấu trúc" VỚI "nội dung khôi phục
được ĐÚNG với bản gốc" — nhưng ĐÂY chính LÀ hai điều `xacThucBackup` cố
tình TÁCH biệt: sự tồn tại của object không được `xacThucBackup` kiểm
tra, CHỈ checksum mới được so sánh.

Chỗ lệch: `xacThucBackup` chỉ có ĐÚNG một dòng —
`tinhChecksumDonGian(noiDungKhoiPhucDuoc) === banSaoLuu.checksumGoc` —
VÀ nó không hề quan tâm `banSaoLuu` có `undefined` hay không (đó LÀ việc
của người GỌI). Với `noiDungBiHong` lệch một ký TỰ, `tinhChecksumDonGian`
trả về một số KHÁC `checksumGoc`, nên phép so sánh LÀ `false` — "file còn
đó" không hề chứng minh được "khôi phục ra ĐÚNG dữ liệu".
::
:::
::::

::::code{#viet_tinh_rpo_ms}
Hoàn thiện `tinhRpoMs` — sau khi ĐÃ lọc ra các bản backup Ở TRƯỚC (hoặc
đúng lúc) sự cố VÀ xác nhận danh sách KHÔNG rỗng: tìm bản backup GẦN nhất
(có `thoiDiemMs` LỚN nhất trong danh sách đã lọc), rồi trả về hiệu số
giữa thời điểm sự cố VÀ thời điểm backup đó.

```typescript title=starter
interface BanSaoLuuTheoThoiGian { thoiDiemMs: number; }

function tinhRpoMs(cacBanSaoLuu: BanSaoLuuTheoThoiGian[], thoiDiemSuCoMs: number): number | undefined {
  const truoc = cacBanSaoLuu.filter((b) => b.thoiDiemMs <= thoiDiemSuCoMs);
  if (truoc.length === 0) return undefined;
  ___
}

const dsX: BanSaoLuuTheoThoiGian[] = [{ thoiDiemMs: 0 }, { thoiDiemMs: 1000 }, { thoiDiemMs: 2000 }];
console.log(String(tinhRpoMs(dsX, 2500)), String(tinhRpoMs(dsX, -1)));
```

```typescript title=solution
interface BanSaoLuuTheoThoiGian { thoiDiemMs: number; }

function tinhRpoMs(cacBanSaoLuu: BanSaoLuuTheoThoiGian[], thoiDiemSuCoMs: number): number | undefined {
  const truoc = cacBanSaoLuu.filter((b) => b.thoiDiemMs <= thoiDiemSuCoMs);
  if (truoc.length === 0) return undefined;
  let ganNhat = truoc[0]!;
  for (const b of truoc) if (b.thoiDiemMs > ganNhat.thoiDiemMs) ganNhat = b;
  return thoiDiemSuCoMs - ganNhat.thoiDiemMs;
}

const dsX: BanSaoLuuTheoThoiGian[] = [{ thoiDiemMs: 0 }, { thoiDiemMs: 1000 }, { thoiDiemMs: 2000 }];
console.log(String(tinhRpoMs(dsX, 2500)), String(tinhRpoMs(dsX, -1)));
```

```typescript title=test
const chuoiT: BanSaoLuuTheoThoiGian[] = [
  { thoiDiemMs: 0 },
  { thoiDiemMs: 3600000 },
  { thoiDiemMs: 7200000 },
  { thoiDiemMs: 10800000 },
];

const rpo1T = tinhRpoMs(chuoiT, 9000000);
if (rpo1T !== 1800000) throw new Error("su co luc 9000000, backup gan nhat truoc do la 7200000, RPO phai la 1800000");

const rpo2T = tinhRpoMs(chuoiT, 10800000);
if (rpo2T !== 0) throw new Error("su co DUNG luc co mot backup (10800000<=10800000), RPO phai la 0 -- khong mat gi ca");

const rpo3T = tinhRpoMs(chuoiT, -500);
if (rpo3T !== undefined) throw new Error("su co truoc CA backup dau tien phai tra ve undefined (khong co backup nao de khoi phuc)");

const rongT: BanSaoLuuTheoThoiGian[] = [];
if (tinhRpoMs(rongT, 1000) !== undefined) throw new Error("khong co backup nao ca phai tra ve undefined");

const motBanT: BanSaoLuuTheoThoiGian[] = [{ thoiDiemMs: 500 }];
if (tinhRpoMs(motBanT, 500) !== 0) throw new Error("mot backup DUNG luc su co, RPO phai la 0");
```

:::hints
- kind: attention
  body: "Duyet mang truoc, giu lai (bang mot bien ganNhat) phan tu co thoiDiemMs LON NHAT. Roi tra ve thoiDiemSuCoMs - ganNhat.thoiDiemMs."
- kind: strategy
  body: "let ganNhat = truoc[0]!; for (const b of truoc) if (b.thoiDiemMs > ganNhat.thoiDiemMs) ganNhat = b; return thoiDiemSuCoMs - ganNhat.thoiDiemMs;"
- kind: one-line
  body: "let ganNhat = truoc[0]!; for (const b of truoc) if (b.thoiDiemMs > ganNhat.thoiDiemMs) ganNhat = b; return thoiDiemSuCoMs - ganNhat.thoiDiemMs;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "500 undefined"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Backup giờ được đo bằng con số, VÀ được xác thực THẬT SỰ, không chỉ tin
vào "file còn đó". Nhưng backup/restore giả định TOÀN bộ khu vực hạ tầng
vẫn còn đứng vững — chỉ dữ liệu bị hỏng. Nếu CẢ một khu vực (region) sập
hoàn toàn, câu trả lời phải LÀ một chiến lược khác hẳn.
::::

::::reflect{#nghi-lai}
`xacThucBackup` VÀ `tinhRpoMs` giải quyết hai câu hỏi khác nhau nhưng bổ
sung cho nhau: một hàm hỏi "backup NÀY có DÙNG được không" (chất lượng),
hàm kia hỏi "mất bao nhiêu THỜI gian dữ liệu nếu dùng nó" (số lượng). Một
chiến lược sao lưu tốt cần cả HAI câu trả lời — một RPO ngắn (backup
thường xuyên) VÔ nghĩa nếu bản thân các bản backup ĐÓ chưa từng được xác
thực LÀ khôi phục ĐÚNG. Đây LÀ lý do "đã sao lưu" VÀ "có thể khôi phục"
không phải LÀ một mệnh đề DUY nhất, mà LÀ hai điều PHẢI kiểm tra riêng.
::::

::::checkpoint{mastery=0.83}
::::
