---
id: co-so-du-lieu.byteledger-capstone.audit-byteledger-noi-doi-cho-nao
title: "Audit — ByteLedger nói dối chỗ nào"
summary: "Bốn điểm ByteLedger mô phỏng KHÁC TigerBeetle thật, mỗi điểm CHỨNG MINH bằng code chạy được (không chỉ liệt kê bằng lời): (1) KHÔNG Byzantine-an-toàn -- demPhieuGomCaKeGiaMao cho thấy một phiếu từ chiSo=99 (không tồn tại trong cụm N=3) VẪN được đếm, đạt đủ quorum(3)=2; (2) KHÔNG network partition thật -- doTreLuonHuuHan xác nhận độ trễ LUÔN nằm trong [1,20) qua 1000 lần rút, không có 'mất mạng vĩnh viễn'; (3) N CỐ ĐỊNH -- tongSoKhongDoiQuaThoiGian xác nhận không API nào đổi tongSo sau khi tạo replica; (4) thời điểm crash LÀ HẰNG SỐ (diemCrashCoDinhTheoKichBan luôn=2, không đọc seed) trong khi độ trễ mạng THẬT sự phụ thuộc seed (5 giá trị khác nhau: 8,15,13,13,11 cho seed 0n..4n)."
locale: vi
track: co-so-du-lieu
module: byteledger-capstone
order: 9
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [db.audit-byteledger-noi-doi-cho-nao]
requires: [db.mini-vopr-toan-he-thong]
concepts: [db.audit-byteledger-noi-doi-cho-nao]
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
Quét 30 seed, KHÔNG seed nào làm vỡ ByteLedger (bài trước). Trước khi
tin RẰNG điều đó có nghĩa "đáng tin cậy" — MASTERPLAN §9.2 yêu cầu
mọi quest mô phỏng phải TRẢ lời thành thật: mô phỏng NÀY khác TigerBeetle
THẬT ở đâu? KHÔNG chỉ nói bằng LỜI — chứng minh bằng CODE.
::::

::::explain{#bon-diem-khac-biet}
Bốn điểm ByteLedger MÔ phỏng KHÁC hẳn TigerBeetle thật — mỗi điểm
CHỨNG minh bằng code chạy ĐƯỢC, không chỉ liệt kê:

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

interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; debitsPending: number; creditsPending: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
function taoTaiKhoan(id: number): TaiKhoan { return { id, debitsPosted: 0, creditsPosted: 0, debitsPending: 0, creditsPending: 0 }; }
interface HeThongSoCai { cacTaiKhoan: Map<number, TaiKhoan>; cacPendingDangCho: Map<number, ButToan>; dsIdDaXuLy: Set<number>; }
function taoHeThongSoCai(cacId: number[]): HeThongSoCai {
  const cacTaiKhoan = new Map<number, TaiKhoan>();
  for (const id of cacId) cacTaiKhoan.set(id, taoTaiKhoan(id));
  return { cacTaiKhoan, cacPendingDangCho: new Map(), dsIdDaXuLy: new Set() };
}

interface LogEntry { opNumber: number; bt: ButToan; }
interface Replica {
  chiSo: number; tongSo: number; viewNumber: number;
  status: 'normal' | 'view-change' | 'recovering';
  log: LogEntry[]; opNumber: number; commitNumber: number;
  soCai: HeThongSoCai;
}
function taoReplica(chiSo: number, tongSo: number, cacIdTaiKhoan: number[]): Replica {
  return { chiSo, tongSo, viewNumber: 0, status: 'normal', log: [], opNumber: 0, commitNumber: 0, soCai: taoHeThongSoCai(cacIdTaiKhoan) };
}
function nguongQuorum(tongSo: number): number { return Math.floor(tongSo / 2) + 1; }
function nhanYeuCauTuClient(primary: Replica, bt: ButToan): LogEntry {
  primary.opNumber += 1;
  const entry: LogEntry = { opNumber: primary.opNumber, bt };
  primary.log.push(entry);
  return entry;
}

// (1) KHONG Byzantine-an-toan: he thong TIN TUYET DOI vao "tu" trong moi phieu
function demPhieuGomCaKeGiaMao(chiSoDaXacNhan: number[], tuGiaMao: number): number {
  const phieu = new Set<number>(chiSoDaXacNhan);
  phieu.add(tuGiaMao);
  return phieu.size;
}

// (2) KHONG mat mang "vinh vien": moi do tre LUON huu han, trong [1,20)
function doTreLuonHuuHan(seed: bigint, soLanThu: number): boolean {
  const tt = gieoHat(seed);
  for (let i = 0; i < soLanThu; i++) {
    const doTre = soNguyenTrongKhoang(tt, 1, 20);
    if (doTre < 1 || doTre >= 20) return false;
  }
  return true;
}

// (3) N CO DINH: khong API nao trong toan bo apparatus doi tongSo sau khi tao replica
function tongSoKhongDoiQuaThoiGian(): boolean {
  const r = taoReplica(0, 3, [0, 1, 2]);
  const tongSoBanDau = r.tongSo;
  nhanYeuCauTuClient(r, { id: 1, debitAccountId: 0, creditAccountId: 1, amount: 100 });
  r.viewNumber = 5;
  r.status = 'view-change';
  r.commitNumber = 1;
  return r.tongSo === tongSoBanDau;
}

// (4) Thoi diem crash LA HANG SO, khong doc seed -- khac han do tre mang (co doc seed)
function diemCrashCoDinhTheoKichBan(): number {
  return 2; // primary LUON "chet" sau dung 2 but toan -- khong nhan seed
}
function doTreDauTienTheoSeed(seed: bigint): number {
  const tt = gieoHat(seed);
  return soNguyenTrongKhoang(tt, 1, 20);
}

console.log("(1) phieu gom ke gia mao:", demPhieuGomCaKeGiaMao([0], 99), "(quorum(3)=" + nguongQuorum(3) + ")");
console.log("(2) do tre luon huu han qua 1000 lan rut:", doTreLuonHuuHan(1n, 1000));
console.log("(3) tongSo khong doi qua thoi gian:", tongSoKhongDoiQuaThoiGian());
console.log("(4) diem crash (5 seed khac nhau):", [0n, 1n, 2n, 3n, 4n].map(() => diemCrashCoDinhTheoKichBan()));
console.log("(4) do tre mang dau tien (5 seed khac nhau):", [0n, 1n, 2n, 3n, 4n].map((s) => doTreDauTienTheoSeed(s)));
```

```text title=readonly
(1) phieu gom ke gia mao: 2 (quorum(3)=2)
(2) do tre luon huu han qua 1000 lan rut: true
(3) tongSo khong doi qua thoi gian: true
(4) diem crash (5 seed khac nhau): [ 2, 2, 2, 2, 2 ]
(4) do tre mang dau tien (5 seed khac nhau): [ 8, 15, 13, 13, 11 ]
```

(1) `demPhieuGomCaKeGiaMao([0], 99)` — CHÍNH primary (`chiSo=0`) tự vote,
CỘNG thêm MỘT "phiếu" từ `chiSo=99` (KHÔNG tồn tại trong CỤM `N=3`
nào, VÀ chưa từng nhận `Prepare` gì) — hệ thống VẪN đếm đủ `2`, ĐẠT
`nguongQuorum(3)=2`. Đây LÀ bằng chứng TRỰC tiếp: `nhanPrepareOk`
(bài 2, q19) KHÔNG hề xác MINH danh tính hay CHỮ ký của `td.tu` —
CHỈ đếm SỐ giá trị PHÂN biệt trong một `Set`. (2) Qua `1000` lần rút,
độ trễ LUÔN nằm TRONG `[1,20)` — KHÔNG có khái niệm "gói tin KHÔNG
BAO giờ tới" (mất hẳn LÀ một quyết ĐỊNH RIÊNG, `matGoiTheoSeed`, KHÔNG
phải một độ TRỄ vô hạn). (3) `tongSo` giữ NGUYÊN `3` DÙ trải qua NHIỀU
thay đổi khác (`viewNumber`, `status`, `commitNumber`) — không CÓ API
"thêm/bớt replica ĐANG chạy". (4) Thời ĐIỂM crash LÀ hằng số `2`
(giống HỆT q19 BOSS's thiết kế) BẤT kể seed — trong khi độ trễ mạng
THẬT sự phụ thuộc seed (`5` giá trị KHÁC nhau cho `5` seed khác nhau).
::::

::::example{#tigerbeetle-that-khac-o-day}
TigerBeetle THẬT: (1) chạy giao THỨC chịu lỗi Byzantine MỘT phần
(kiểm tra checksum/chữ ký trên MỌI thông điệp, phát hiện replica bị
"hỏng" gửi dữ liệu SAI, không CHỈ replica bị chậm/mất kết nối); (2)
mạng THẬT có thể "chia cắt" (partition) — MỘT nhóm replica hoàn toàn
KHÔNG thể liên lạc với nhóm KIA trong một khoảng thời gian BẤT định,
không PHẢI một độ trễ có TRẦN; (3) hỗ trợ reconfiguration — thêm/bớt
replica MÀ không dừng cụm; (4) VOPR THẬT ngẫu-nhiên-hoá CẢ thời điểm
crash, KHÔNG chỉ hành vi mạng/đĩa. ByteLedger (q21) KHÔNG mô phỏng bất
kỳ điều NÀO trong bốn điều TRÊN — VÀ đó LÀ lựa chọn CÓ chủ đích (giữ
scope Ở "đủ để CHỨNG minh cơ chế đúng"), không phải một LỖ hổng bị bỏ
sót.
::::

::::predict{#doan-neu-co-hai-ke-gia-mao commitOnce}
Gọi `demPhieuGomCaKeGiaMao([0], 99)` HAI lần LIÊN tiếp — LẦN đầu như
Ở TRÊN, lần HAI truyền THÊM `chiSoDaXacNhan=[0, 99]` (đã "chứa" kẻ giả
mạo TỪ trước) và `tuGiaMao=99` (CÙNG kẻ giả mạo, gửi PHIẾU trùng lần
thứ hai). Kết quả LẦN gọi thứ hai LÀ bao nhiêu?
:::opt{correct}
`2` — `Set` khử TRÙNG tự động; `phieu.add(99)` khi `99` ĐÃ có TRONG
`Set` không làm `size` tăng thêm — kẻ giả mạo KHÔNG thể "nhân bản"
phiếu của chính NÓ để chiếm đa số
:::
:::opt
`3` — kẻ giả mạo gửi PHIẾU hai lần thì được TÍNH hai lần, giống MỘT
cuộc tấn công "double-vote"
::why
Trực giác NÀY đúng VỚI một số hệ thống KHÔNG dùng `Set` (VÍ dụ đếm
bằng mảng, KHÔNG khử trùng) — nhưng SAI với code Ở đây.

Chỗ lệch: `demPhieuGomCaKeGiaMao` DÙNG `Set<number>`, cấu trúc CHỈ
giữ mỗi GIÁ trị đúng MỘT lần. Đây chính LÀ LÝ do (đã học TỪ q19 bài
10) `nhanPrepareOk`/`nhanStartViewChange` DÙNG `Set` thay VÌ một bộ
đếm số nguyên: chống được "double-vote" TỪ MỘT nguồn — nhưng (như bài
NÀY chứng minh) KHÔNG chống được việc TIN một nguồn GIẢ MẠO chỉ gửi
đúng MỘT phiếu.
::
:::
::::

::::code{#viet_dem_phieu_gom_ke_gia_mao}
Hoàn thiện `demPhieuGomCaKeGiaMao` — thêm `tuGiaMao` VÀO `phieu`
(giống hệt CÁCH `nhanPrepareOk` thêm BẤT kỳ `td.tu` nào, KHÔNG xác
minh danh tính), rồi trả về SỐ lượng phiếu phân biệt.

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

interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; debitsPending: number; creditsPending: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
function taoTaiKhoan(id: number): TaiKhoan { return { id, debitsPosted: 0, creditsPosted: 0, debitsPending: 0, creditsPending: 0 }; }
interface HeThongSoCai { cacTaiKhoan: Map<number, TaiKhoan>; cacPendingDangCho: Map<number, ButToan>; dsIdDaXuLy: Set<number>; }
function taoHeThongSoCai(cacId: number[]): HeThongSoCai {
  const cacTaiKhoan = new Map<number, TaiKhoan>();
  for (const id of cacId) cacTaiKhoan.set(id, taoTaiKhoan(id));
  return { cacTaiKhoan, cacPendingDangCho: new Map(), dsIdDaXuLy: new Set() };
}

interface LogEntry { opNumber: number; bt: ButToan; }
interface Replica {
  chiSo: number; tongSo: number; viewNumber: number;
  status: 'normal' | 'view-change' | 'recovering';
  log: LogEntry[]; opNumber: number; commitNumber: number;
  soCai: HeThongSoCai;
}
function taoReplica(chiSo: number, tongSo: number, cacIdTaiKhoan: number[]): Replica {
  return { chiSo, tongSo, viewNumber: 0, status: 'normal', log: [], opNumber: 0, commitNumber: 0, soCai: taoHeThongSoCai(cacIdTaiKhoan) };
}
function nguongQuorum(tongSo: number): number { return Math.floor(tongSo / 2) + 1; }
function nhanYeuCauTuClient(primary: Replica, bt: ButToan): LogEntry {
  primary.opNumber += 1;
  const entry: LogEntry = { opNumber: primary.opNumber, bt };
  primary.log.push(entry);
  return entry;
}

function demPhieuGomCaKeGiaMao(chiSoDaXacNhan: number[], tuGiaMao: number): number {
  const phieu = new Set<number>(chiSoDaXacNhan);
  ___
  return phieu.size;
}

console.log(demPhieuGomCaKeGiaMao([0], 99));
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

interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; debitsPending: number; creditsPending: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
function taoTaiKhoan(id: number): TaiKhoan { return { id, debitsPosted: 0, creditsPosted: 0, debitsPending: 0, creditsPending: 0 }; }
interface HeThongSoCai { cacTaiKhoan: Map<number, TaiKhoan>; cacPendingDangCho: Map<number, ButToan>; dsIdDaXuLy: Set<number>; }
function taoHeThongSoCai(cacId: number[]): HeThongSoCai {
  const cacTaiKhoan = new Map<number, TaiKhoan>();
  for (const id of cacId) cacTaiKhoan.set(id, taoTaiKhoan(id));
  return { cacTaiKhoan, cacPendingDangCho: new Map(), dsIdDaXuLy: new Set() };
}

interface LogEntry { opNumber: number; bt: ButToan; }
interface Replica {
  chiSo: number; tongSo: number; viewNumber: number;
  status: 'normal' | 'view-change' | 'recovering';
  log: LogEntry[]; opNumber: number; commitNumber: number;
  soCai: HeThongSoCai;
}
function taoReplica(chiSo: number, tongSo: number, cacIdTaiKhoan: number[]): Replica {
  return { chiSo, tongSo, viewNumber: 0, status: 'normal', log: [], opNumber: 0, commitNumber: 0, soCai: taoHeThongSoCai(cacIdTaiKhoan) };
}
function nguongQuorum(tongSo: number): number { return Math.floor(tongSo / 2) + 1; }
function nhanYeuCauTuClient(primary: Replica, bt: ButToan): LogEntry {
  primary.opNumber += 1;
  const entry: LogEntry = { opNumber: primary.opNumber, bt };
  primary.log.push(entry);
  return entry;
}

function demPhieuGomCaKeGiaMao(chiSoDaXacNhan: number[], tuGiaMao: number): number {
  const phieu = new Set<number>(chiSoDaXacNhan);
  phieu.add(tuGiaMao);
  return phieu.size;
}

console.log(demPhieuGomCaKeGiaMao([0], 99));
```

```typescript title=test
if (demPhieuGomCaKeGiaMao([0], 99) !== 2) throw new Error("phieu tu ke gia mao (chiSo=99, khong ton tai trong cum N=3) VAN duoc dem la 1 phieu hop le -- tong phai la 2");
if (nguongQuorum(3) !== 2) throw new Error("nguongQuorum(3) phai la 2 (sanity check)");
if (demPhieuGomCaKeGiaMao([0], 99) < nguongQuorum(3)) throw new Error("2 phieu (mot THAT, mot GIA) van phai DU quorum(3)=2 -- chung minh he thong khong Byzantine-an-toan");
if (demPhieuGomCaKeGiaMao([0, 1], 1) !== 2) throw new Error("phieu TRUNG (chiSo=1 da co san trong danhSach) khong duoc dem hai lan -- Set khu trung, tong van la 2");

function doTreLuonHuuHan(seed: bigint, soLanThu: number): boolean {
  const tt = gieoHat(seed);
  for (let i = 0; i < soLanThu; i++) {
    const doTre = soNguyenTrongKhoang(tt, 1, 20);
    if (doTre < 1 || doTre >= 20) return false;
  }
  return true;
}
if (!doTreLuonHuuHan(1n, 1000)) throw new Error("qua 1000 lan rut, do tre PHAI luon nam trong [1,20) -- khong co 'mat mang vinh vien' trong mo hinh nay");
if (!doTreLuonHuuHan(999n, 1000)) throw new Error("dieu nay phai dung voi MOI seed, khong chi seed=1n");

function tongSoKhongDoiQuaThoiGian(): boolean {
  const r = taoReplica(0, 3, [0, 1, 2]);
  const tongSoBanDau = r.tongSo;
  nhanYeuCauTuClient(r, { id: 1, debitAccountId: 0, creditAccountId: 1, amount: 100 });
  r.viewNumber = 5;
  r.status = 'view-change';
  r.commitNumber = 1;
  return r.tongSo === tongSoBanDau;
}
if (!tongSoKhongDoiQuaThoiGian()) throw new Error("tongSo phai GIU NGUYEN xuyen suot moi thao tac -- khong co reconfiguration");

function diemCrashCoDinhTheoKichBan(): number { return 2; }
function doTreDauTienTheoSeed(seed: bigint): number {
  const tt = gieoHat(seed);
  return soNguyenTrongKhoang(tt, 1, 20);
}
const diemCrashCacSeed = [0n, 1n, 2n, 3n, 4n].map(() => diemCrashCoDinhTheoKichBan());
if (!diemCrashCacSeed.every((d) => d === 2)) throw new Error("diem crash phai la HANG SO 2 bat ke seed nao");
const tapHopDoTre = new Set([0n, 1n, 2n, 3n, 4n].map((s) => doTreDauTienTheoSeed(s)));
if (tapHopDoTre.size < 2) throw new Error("do tre MANG phai THAY DOI theo seed (khac han diem crash co dinh)");
```

:::hints
- kind: attention
  body: "Them tuGiaMao vao Set phieu (giong het cach nhanPrepareOk them BAT KY td.tu nao, khong xac minh) -- mot dong."
- kind: strategy
  body: "phieu.add(tuGiaMao);"
- kind: one-line
  body: "phieu.add(tuGiaMao);"
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 5000
- tier: output
  match: contains
  expect: "2"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn điểm khác biệt, đã thành thật. Giờ ráp TẤT CẢ mười bài — double-
entry, tất định, tiêm lỗi, VSR — vào một BOSS cuối cùng.
::::

::::reflect{#nghi-lai}
Cả bốn hàm bài NÀY ĐỀU bé — không hàm nào QUÁ năm dòng — nhưng CHÚNG
LÀ bằng chứng, không phải LỜI hứa. Đúng tinh THẦN "đếm thật thay VÌ
tin" xuyên suốt khoá học: thay VÌ viết một đoạn văn NÓI "hệ thống này
không chống Byzantine", `demPhieuGomCaKeGiaMao` CHỨNG minh điều đó
bằng một PHIẾU giả mạo THẬT sự lọt qua. Một mô phỏng ĐÁNG tin không
phải VÌ nó giống hệt hệ thống THẬT — mà VÌ nó BIẾT (và NÓI rõ) chính
xác nó khác Ở đâu.
::::

::::checkpoint{mastery=0.85}
::::
