---
id: co-so-du-lieu.byteledger-capstone.mini-vopr-toan-he-thong
title: "Mini-VOPR cho cả hệ thống"
summary: "chayKichBanByteLedgerNhoGon(seed) chạy N=3, hai bút toán qua xuLyMotButToanQuaPrimary (q19 BOSS, áp dụng qua apDungQuaSoCai đầy đủ) VỚI retry+state-transfer tự động, CỘNG lỗi đĩa (apDungLoiDiaTheoChinhSach) cho mỗi backup -- TẤT CẢ rút từ ĐÚNG một ChinhSachLoi.tt. phanLoaiViPham kiểm tra HAI bất biến: heThongCanBang VÀ hội tụ giữa 3 replica. Quét 30 seed đầu tiên (0..29): timSeedLamVoMotTrongHaiBatBien trả về null -- KHÔNG seed nào làm vỡ, quorum(N=3)=2 đủ chịu mất gói lẻ tẻ VÀ lỗi đĩa (vốn không ảnh hưởng bộ nhớ) cùng lúc."
locale: vi
track: co-so-du-lieu
module: byteledger-capstone
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [db.mini-vopr-toan-he-thong]
requires: [db.idempotent-retry-xuyen-view-change]
concepts: [db.mini-vopr-toan-he-thong]
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
Idempotent xuyên suốt view change, đã chứng minh (bài trước) — bằng
TAY, một kịch bản CỤ thể. q18 BOSS đã dạy cách TỰ ĐỘNG quét seed thay
vì đoán. Áp dụng ĐIỀU đó cho toàn bộ ByteLedger — đĩa, mạng, VÀ VSR
CÙNG lúc.
::::

::::explain{#quet-toan-he-thong}
`chayKichBanByteLedgerNhoGon(seed)` chạy `N=3`, HAI bút toán qua
`xuLyMotButToanQuaPrimary` (q19 BOSS, nguyên xi — retry + State
Transfer tự động khi backup lạc HẬU — chỉ đổi CHỖ áp dụng THÀNH
`apDungQuaSoCai` đầy đủ), CỘNG một lỗi đĩa (`apDungLoiDiaTheoChinhSach`,
bài 4) cho MỖI backup SAU mỗi bút toán — TẤT cả rút TỪ đúng MỘT
`ChinhSachLoi.tt`. `phanLoaiViPham` kiểm tra HAI bất biến — `heThongCanBang`
VÀ hội tụ giữa `3` replica:

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
function matGoiTheoSeed(tt: TrangThai, tyLeMatPhanTram: number): boolean {
  return soNguyenTrongKhoang(tt, 0, 100) < tyLeMatPhanTram;
}
interface DongHoAo { hienTai: number; }
function taoDongHoAo(): DongHoAo { return { hienTai: 0 }; }
function tienToi(dh: DongHoAo, soMs: number): void { dh.hienTai += soMs; }

class SimDisk {
  readonly kichThuocSector: number;
  private readonly cache = new Map<number, Uint8Array>();
  private matFsyncKeTiep = false;
  private readonly tornSector = new Map<number, number>();
  constructor(kichThuocSector: number) { this.kichThuocSector = kichThuocSector; }
  write(sector: number, data: Uint8Array): void { this.cache.set(sector, data.slice()); }
  fsync(): void {
    if (this.matFsyncKeTiep) { this.matFsyncKeTiep = false; return; }
    this.tornSector.clear();
    this.cache.clear();
  }
  boQuaFsyncKeTiep(): void { this.matFsyncKeTiep = true; }
  danhDauTornGhi(sector: number, n: number): void { this.tornSector.set(sector, n); }
}
interface ChinhSachLoi { tt: TrangThai; }
function taoChinhSachLoi(seed: bigint): ChinhSachLoi { return { tt: gieoHat(seed) }; }
function apDungLoiDiaTheoChinhSach(cs: ChinhSachLoi, disk: SimDisk, sector: number): number {
  const kieu = soNguyenTrongKhoang(cs.tt, 0, 3);
  if (kieu === 1) disk.boQuaFsyncKeTiep();
  else if (kieu === 2) disk.danhDauTornGhi(sector, Math.floor(disk.kichThuocSector / 2));
  return kieu;
}

interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; debitsPending: number; creditsPending: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
function taoTaiKhoan(id: number): TaiKhoan { return { id, debitsPosted: 0, creditsPosted: 0, debitsPending: 0, creditsPending: 0 }; }
interface HeThongSoCai {
  cacTaiKhoan: Map<number, TaiKhoan>;
  cacPendingDangCho: Map<number, ButToan>;
  dsIdDaXuLy: Set<number>;
}
function taoHeThongSoCai(cacId: number[]): HeThongSoCai {
  const cacTaiKhoan = new Map<number, TaiKhoan>();
  for (const id of cacId) cacTaiKhoan.set(id, taoTaiKhoan(id));
  return { cacTaiKhoan, cacPendingDangCho: new Map(), dsIdDaXuLy: new Set() };
}
function heThongCanBang(soCai: HeThongSoCai): boolean {
  let no = 0, co = 0;
  for (const [, tk] of soCai.cacTaiKhoan) { no += tk.debitsPosted; co += tk.creditsPosted; }
  return no === co;
}
function apDungQuaSoCai(soCai: HeThongSoCai, bt: ButToan): boolean {
  if (soCai.dsIdDaXuLy.has(bt.id)) return true;
  soCai.dsIdDaXuLy.add(bt.id);
  soCai.cacTaiKhoan.get(bt.debitAccountId)!.debitsPosted += bt.amount;
  soCai.cacTaiKhoan.get(bt.creditAccountId)!.creditsPosted += bt.amount;
  return true;
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

interface ThongDiep {
  loai: 'Prepare' | 'PrepareOk' | 'Commit' | 'PhanHoiDongBo';
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
function soSanhSuKienMang(a: SuKienMang, b: SuKienMang): number {
  if (a.thoiDiem !== b.thoiDiem) return a.thoiDiem - b.thoiDiem;
  return a.thuTuChen - b.thuTuChen;
}
function layThongDiepTiepTheo(hd: HangDoiMang): SuKienMang | undefined {
  if (hd.danhSach.length === 0) return undefined;
  let idxNhoNhat = 0;
  for (let i = 1; i < hd.danhSach.length; i++) {
    if (soSanhSuKienMang(hd.danhSach[i]!, hd.danhSach[idxNhoNhat]!) < 0) idxNhoNhat = i;
  }
  return hd.danhSach.splice(idxNhoNhat, 1)[0];
}
function nhanYeuCauTuClient(primary: Replica, bt: ButToan): LogEntry {
  primary.opNumber += 1;
  const entry: LogEntry = { opNumber: primary.opNumber, bt };
  primary.log.push(entry);
  return entry;
}
function xuLyPrepare(hd: HangDoiMang, tt: TrangThai, dh: DongHoAo, backup: Replica, td: ThongDiep): void {
  const entry: LogEntry = { opNumber: td.opNumber!, bt: td.bt! };
  backup.log.push(entry);
  backup.opNumber = td.opNumber!;
  const doTre = soNguyenTrongKhoang(tt, 1, 20);
  guiThongDiep(hd, dh.hienTai + doTre, { loai: 'PrepareOk', tu: backup.chiSo, den: td.tu, viewNumber: backup.viewNumber, opNumber: td.opNumber! });
}
function nhanPrepareOk(primary: Replica, phieuTheoOp: Map<number, Set<number>>, td: ThongDiep): boolean {
  const opNumber = td.opNumber!;
  let phieu = phieuTheoOp.get(opNumber);
  if (!phieu) { phieu = new Set<number>([primary.chiSo]); phieuTheoOp.set(opNumber, phieu); }
  phieu.add(td.tu);
  if (phieu.size < nguongQuorum(primary.tongSo)) return false;
  if (opNumber !== primary.commitNumber + 1) return false;
  const entry = primary.log.find((e) => e.opNumber === opNumber)!;
  apDungQuaSoCai(primary.soCai, entry.bt);
  primary.commitNumber = opNumber;
  return true;
}
function xuLyCommit(backup: Replica, td: ThongDiep): void {
  const commitMoi = td.commitNumber!;
  for (let op = backup.commitNumber + 1; op <= commitMoi; op++) {
    const entry = backup.log.find((e) => e.opNumber === op)!;
    apDungQuaSoCai(backup.soCai, entry.bt);
  }
  backup.commitNumber = commitMoi;
}
function kiemTraCanGuiLai(dh: DongHoAo, thoiDiemGuiLanDau: number, nguongTimeoutMs: number): boolean {
  return dh.hienTai - thoiDiemGuiLanDau >= nguongTimeoutMs;
}
function kiemTraCanDongBo(backup: Replica, opNumberMoiNhatDaBiet: number): boolean {
  return backup.opNumber < opNumberMoiNhatDaBiet;
}
function xuLyPhanHoiDongBo(backup: Replica, td: ThongDiep): void {
  const entryConThieu = td.log!;
  for (const entry of entryConThieu) if (entry.opNumber > backup.opNumber) backup.log.push(entry);
  backup.opNumber = backup.log.length === 0 ? 0 : backup.log[backup.log.length - 1]!.opNumber;
}

function xuLyMotButToanQuaPrimary(
  hd: HangDoiMang, tt: TrangThai, dh: DongHoAo,
  primary: Replica, cacBackupConSong: Replica[],
  bt: ButToan, tyLeMatPhanTram: number,
): boolean {
  const entry = nhanYeuCauTuClient(primary, bt);
  const phieuTheoOp = new Map<number, Set<number>>();
  const daNhanPhanHoi = new Set<number>();
  let daCommit = false;
  const thoiDiemBatDau = dh.hienTai;
  const NGUONG_GUI_LAI = 30;
  const SO_LAN_THU_TOI_DA = 6;
  for (let lan = 0; lan < SO_LAN_THU_TOI_DA && !daCommit; lan++) {
    if (lan > 0 && !kiemTraCanGuiLai(dh, thoiDiemBatDau, NGUONG_GUI_LAI)) tienToi(dh, NGUONG_GUI_LAI);
    for (const backup of cacBackupConSong) {
      if (daNhanPhanHoi.has(backup.chiSo)) continue;
      if (matGoiTheoSeed(tt, tyLeMatPhanTram)) continue;
      if (entry.opNumber > backup.opNumber + 1) {
        const conThieu = primary.log.filter((e) => e.opNumber > backup.opNumber && e.opNumber < entry.opNumber);
        xuLyPhanHoiDongBo(backup, { loai: 'PhanHoiDongBo', tu: primary.chiSo, den: backup.chiSo, viewNumber: primary.viewNumber, log: conThieu });
      }
      if (backup.opNumber < entry.opNumber) {
        xuLyPrepare(hd, tt, dh, backup, { loai: 'Prepare', tu: primary.chiSo, den: backup.chiSo, viewNumber: primary.viewNumber, opNumber: entry.opNumber, bt: entry.bt });
      } else {
        const doTreLai = soNguyenTrongKhoang(tt, 1, 20);
        guiThongDiep(hd, dh.hienTai + doTreLai, { loai: 'PrepareOk', tu: backup.chiSo, den: primary.chiSo, viewNumber: backup.viewNumber, opNumber: entry.opNumber });
      }
    }
    let sk: SuKienMang | undefined;
    while ((sk = layThongDiepTiepTheo(hd)) !== undefined) {
      if (matGoiTheoSeed(tt, tyLeMatPhanTram)) continue;
      daNhanPhanHoi.add(sk.td.tu);
      if (nhanPrepareOk(primary, phieuTheoOp, sk.td)) daCommit = true;
    }
  }
  if (daCommit) {
    for (const backup of cacBackupConSong) {
      if (kiemTraCanDongBo(backup, primary.opNumber)) {
        const conThieu = primary.log.filter((e) => e.opNumber > backup.opNumber);
        xuLyPhanHoiDongBo(backup, { loai: 'PhanHoiDongBo', tu: primary.chiSo, den: backup.chiSo, viewNumber: primary.viewNumber, log: conThieu });
      }
      xuLyCommit(backup, { loai: 'Commit', tu: primary.chiSo, den: backup.chiSo, viewNumber: primary.viewNumber, commitNumber: primary.commitNumber });
    }
  }
  return daCommit;
}
function xacNhanHoiTu(a: HeThongSoCai, b: HeThongSoCai): boolean {
  return JSON.stringify([...a.cacTaiKhoan.entries()].sort()) === JSON.stringify([...b.cacTaiKhoan.entries()].sort());
}

function chayKichBanByteLedgerNhoGon(seed: bigint): { canBang: boolean; hoiTu: boolean } {
  const cs = taoChinhSachLoi(seed);
  const dh = taoDongHoAo();
  const hd = taoHangDoiMang();
  const N = 3;
  const TY_LE_MAT = 30;
  const NGUON_NGOAI = 0, TK1 = 1, TK2 = 2;
  const r0 = taoReplica(0, N, [NGUON_NGOAI, TK1, TK2]);
  const r1 = taoReplica(1, N, [NGUON_NGOAI, TK1, TK2]);
  const r2 = taoReplica(2, N, [NGUON_NGOAI, TK1, TK2]);
  const diskTheoChiSo = new Map<number, SimDisk>([[1, new SimDisk(4)], [2, new SimDisk(4)]]);

  xuLyMotButToanQuaPrimary(hd, cs.tt, dh, r0, [r1, r2], { id: 1, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 1000 }, TY_LE_MAT);
  for (const b of [r1, r2]) {
    const disk = diskTheoChiSo.get(b.chiSo)!;
    apDungLoiDiaTheoChinhSach(cs, disk, 0);
    disk.write(0, new Uint8Array([9, 9, 9, 9]));
    disk.fsync();
  }
  xuLyMotButToanQuaPrimary(hd, cs.tt, dh, r0, [r1, r2], { id: 2, debitAccountId: TK1, creditAccountId: TK2, amount: 400 }, TY_LE_MAT);
  for (const b of [r1, r2]) {
    const disk = diskTheoChiSo.get(b.chiSo)!;
    apDungLoiDiaTheoChinhSach(cs, disk, 0);
    disk.write(0, new Uint8Array([9, 9, 9, 9]));
    disk.fsync();
  }

  return {
    canBang: heThongCanBang(r0.soCai) && heThongCanBang(r1.soCai) && heThongCanBang(r2.soCai),
    hoiTu: xacNhanHoiTu(r0.soCai, r1.soCai) && xacNhanHoiTu(r0.soCai, r2.soCai),
  };
}

function phanLoaiViPham(kq: { canBang: boolean; hoiTu: boolean }): 'canBang' | 'hoiTu' | null {
  if (!kq.canBang) return 'canBang';
  if (!kq.hoiTu) return 'hoiTu';
  return null;
}
function timSeedLamVoMotTrongHaiBatBien(soLuongSeedThu: number): { seed: bigint; loaiVo: 'canBang' | 'hoiTu' } | null {
  for (let i = 0; i < soLuongSeedThu; i++) {
    const seed = BigInt(i);
    const loaiVo = phanLoaiViPham(chayKichBanByteLedgerNhoGon(seed));
    if (loaiVo !== null) return { seed, loaiVo };
  }
  return null;
}

console.log("seed=3n:", JSON.stringify(chayKichBanByteLedgerNhoGon(3n)));
console.log("quet 30 seed:", JSON.stringify(timSeedLamVoMotTrongHaiBatBien(30)));
```

```text title=readonly
seed=3n: {"canBang":true,"hoiTu":true}
quet 30 seed: null
```

`chayKichBanByteLedgerNhoGon` KHÔNG hề crash primary (khác BOSS Ở bài
10 sắp tới) — CHỈ tiêm lỗi mạng (mất Prepare/PrepareOk, retry TỰ động
qua `xuLyMotButToanQuaPrimary`) VÀ lỗi ĐĨA (`apDungLoiDiaTheoChinhSach`,
BÊN cạnh, KHÔNG ảnh hưởng consensus). Quét `30` seed đầu tiên: KHÔNG
seed nào làm `canBang` hay `hoiTu` sai — `null`, giống hệt phát hiện
CỦA q19 BOSS trên chính CHỪNG đó seed.
::::

::::example{#dia-khong-cham-toi-bo-nho}
Đây LÀ một phát hiện THẬT (không phải điều ĐOÁN trước được): lỗi ĐĨA KHÔNG hề
ảnh hưởng `canBang`/`hoiTu` Ở kịch bản NÀY — VÌ mô hình của quest CHƯA
wire "backup TỪ chối tham gia đồng thuận nếu ghi đĩa của NÓ thất
bại" (một tính năng THẬT có Ở TigerBeetle THẬT, nhưng NGOÀI phạm vi
q21). Hai bất biến ta ĐANG đo (`heThongCanBang`, hội tụ) chỉ nhìn VÀO
TRẠNG thái bộ nhớ (`soCai`) — đĩa LÀ một tầng HOÀN toàn tách BIỆT
trong mô hình NÀY. Đây chính LÀ điều bài SAU (audit) sẽ đào sâu:
"ByteLedger nói dối chỗ nào" so VỚI hệ thống thật.
::::

::::predict{#doan-neu-mat-100-phan-tram commitOnce}
NẾU đổi `TY_LE_MAT` (bên trong `chayKichBanByteLedgerNhoGon`) từ `30`
thành `100` (mọi Prepare/PrepareOk ĐỀU mất), quét `timSeedLamVoMotTrongHaiBatBien(30)`
CÓ tìm thấy MỘT seed làm vỡ `hoiTu` không?
:::opt{correct}
KHÔNG — `hoiTu` chỉ SO sánh giữa các replica ĐÃ tồn tại, VÀ `heThongCanBang`
chỉ kiểm tra tính đối XỨNG nợ/có; VỚI `100%` mất gói, `daCommit` mãi
mãi `false` (giống q19 BOSS's predict) — KHÔNG replica NÀO tiến thêm,
nhưng CẢ ba VẪN Ở đúng trạng thái BAN đầu (rỗng, cân bằng, giống hệt
nhau) — VẪN `canBang:true`, `hoiTu:true`, KHÔNG hề "vỡ"
:::
:::opt
CÓ — mất `100%` gói chắc chắn LÀM các replica lệch nhau, VÌ chúng
không CÒN cách nào đồng bộ
::why
Trực giác NÀY nhầm "KHÔNG tiến triển được" VỚI "LỆCH nhau" — nhưng
đứng YÊN Ở CÙNG một điểm xuất phát (rỗng) VẪN LÀ một dạng "hội tụ"
hợp LỆ theo đúng định nghĩa CỦA `xacNhanHoiTu`.

Chỗ lệch: `xacNhanHoiTu` chỉ so SÁNH nội dung `cacTaiKhoan` GIỮA các
replica — NẾU không có bút toán NÀO commit được (mất `100%` gói),
`cacTaiKhoan` của CẢ ba VẪN y hệt NHAU (đều Ở trạng thái khởi tạo,
mọi `debitsPosted`/`creditsPosted` LÀ `0`) — "hội tụ" Ở đây KHÔNG có
nghĩa LÀ "tiến bộ", chỉ có nghĩa LÀ "giống nhau". Một hệ thống ĐỨNG
YÊN hoàn toàn VẪN thoả cả hai bất biến — nó chỉ KHÔNG hữu ích, không
phải KHÔNG đúng.
::
:::
::::

::::code{#viet_phan_loai_vi_pham}
Hoàn thiện `phanLoaiViPham` — kiểm tra `canBang` TRƯỚC (trả `'canBang'`
nếu SAI), rồi `hoiTu` (trả `'hoiTu'` nếu SAI), CUỐI cùng `null` nếu
CẢ hai đều giữ vững.

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
function matGoiTheoSeed(tt: TrangThai, tyLeMatPhanTram: number): boolean {
  return soNguyenTrongKhoang(tt, 0, 100) < tyLeMatPhanTram;
}
interface DongHoAo { hienTai: number; }
function taoDongHoAo(): DongHoAo { return { hienTai: 0 }; }
function tienToi(dh: DongHoAo, soMs: number): void { dh.hienTai += soMs; }

class SimDisk {
  readonly kichThuocSector: number;
  private readonly cache = new Map<number, Uint8Array>();
  private matFsyncKeTiep = false;
  private readonly tornSector = new Map<number, number>();
  constructor(kichThuocSector: number) { this.kichThuocSector = kichThuocSector; }
  write(sector: number, data: Uint8Array): void { this.cache.set(sector, data.slice()); }
  fsync(): void {
    if (this.matFsyncKeTiep) { this.matFsyncKeTiep = false; return; }
    this.tornSector.clear();
    this.cache.clear();
  }
  boQuaFsyncKeTiep(): void { this.matFsyncKeTiep = true; }
  danhDauTornGhi(sector: number, n: number): void { this.tornSector.set(sector, n); }
}
interface ChinhSachLoi { tt: TrangThai; }
function taoChinhSachLoi(seed: bigint): ChinhSachLoi { return { tt: gieoHat(seed) }; }
function apDungLoiDiaTheoChinhSach(cs: ChinhSachLoi, disk: SimDisk, sector: number): number {
  const kieu = soNguyenTrongKhoang(cs.tt, 0, 3);
  if (kieu === 1) disk.boQuaFsyncKeTiep();
  else if (kieu === 2) disk.danhDauTornGhi(sector, Math.floor(disk.kichThuocSector / 2));
  return kieu;
}

interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; debitsPending: number; creditsPending: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
function taoTaiKhoan(id: number): TaiKhoan { return { id, debitsPosted: 0, creditsPosted: 0, debitsPending: 0, creditsPending: 0 }; }
interface HeThongSoCai {
  cacTaiKhoan: Map<number, TaiKhoan>;
  cacPendingDangCho: Map<number, ButToan>;
  dsIdDaXuLy: Set<number>;
}
function taoHeThongSoCai(cacId: number[]): HeThongSoCai {
  const cacTaiKhoan = new Map<number, TaiKhoan>();
  for (const id of cacId) cacTaiKhoan.set(id, taoTaiKhoan(id));
  return { cacTaiKhoan, cacPendingDangCho: new Map(), dsIdDaXuLy: new Set() };
}
function heThongCanBang(soCai: HeThongSoCai): boolean {
  let no = 0, co = 0;
  for (const [, tk] of soCai.cacTaiKhoan) { no += tk.debitsPosted; co += tk.creditsPosted; }
  return no === co;
}
function apDungQuaSoCai(soCai: HeThongSoCai, bt: ButToan): boolean {
  if (soCai.dsIdDaXuLy.has(bt.id)) return true;
  soCai.dsIdDaXuLy.add(bt.id);
  soCai.cacTaiKhoan.get(bt.debitAccountId)!.debitsPosted += bt.amount;
  soCai.cacTaiKhoan.get(bt.creditAccountId)!.creditsPosted += bt.amount;
  return true;
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

interface ThongDiep {
  loai: 'Prepare' | 'PrepareOk' | 'Commit' | 'PhanHoiDongBo';
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
function soSanhSuKienMang(a: SuKienMang, b: SuKienMang): number {
  if (a.thoiDiem !== b.thoiDiem) return a.thoiDiem - b.thoiDiem;
  return a.thuTuChen - b.thuTuChen;
}
function layThongDiepTiepTheo(hd: HangDoiMang): SuKienMang | undefined {
  if (hd.danhSach.length === 0) return undefined;
  let idxNhoNhat = 0;
  for (let i = 1; i < hd.danhSach.length; i++) {
    if (soSanhSuKienMang(hd.danhSach[i]!, hd.danhSach[idxNhoNhat]!) < 0) idxNhoNhat = i;
  }
  return hd.danhSach.splice(idxNhoNhat, 1)[0];
}
function nhanYeuCauTuClient(primary: Replica, bt: ButToan): LogEntry {
  primary.opNumber += 1;
  const entry: LogEntry = { opNumber: primary.opNumber, bt };
  primary.log.push(entry);
  return entry;
}
function xuLyPrepare(hd: HangDoiMang, tt: TrangThai, dh: DongHoAo, backup: Replica, td: ThongDiep): void {
  const entry: LogEntry = { opNumber: td.opNumber!, bt: td.bt! };
  backup.log.push(entry);
  backup.opNumber = td.opNumber!;
  const doTre = soNguyenTrongKhoang(tt, 1, 20);
  guiThongDiep(hd, dh.hienTai + doTre, { loai: 'PrepareOk', tu: backup.chiSo, den: td.tu, viewNumber: backup.viewNumber, opNumber: td.opNumber! });
}
function nhanPrepareOk(primary: Replica, phieuTheoOp: Map<number, Set<number>>, td: ThongDiep): boolean {
  const opNumber = td.opNumber!;
  let phieu = phieuTheoOp.get(opNumber);
  if (!phieu) { phieu = new Set<number>([primary.chiSo]); phieuTheoOp.set(opNumber, phieu); }
  phieu.add(td.tu);
  if (phieu.size < nguongQuorum(primary.tongSo)) return false;
  if (opNumber !== primary.commitNumber + 1) return false;
  const entry = primary.log.find((e) => e.opNumber === opNumber)!;
  apDungQuaSoCai(primary.soCai, entry.bt);
  primary.commitNumber = opNumber;
  return true;
}
function xuLyCommit(backup: Replica, td: ThongDiep): void {
  const commitMoi = td.commitNumber!;
  for (let op = backup.commitNumber + 1; op <= commitMoi; op++) {
    const entry = backup.log.find((e) => e.opNumber === op)!;
    apDungQuaSoCai(backup.soCai, entry.bt);
  }
  backup.commitNumber = commitMoi;
}
function kiemTraCanGuiLai(dh: DongHoAo, thoiDiemGuiLanDau: number, nguongTimeoutMs: number): boolean {
  return dh.hienTai - thoiDiemGuiLanDau >= nguongTimeoutMs;
}
function kiemTraCanDongBo(backup: Replica, opNumberMoiNhatDaBiet: number): boolean {
  return backup.opNumber < opNumberMoiNhatDaBiet;
}
function xuLyPhanHoiDongBo(backup: Replica, td: ThongDiep): void {
  const entryConThieu = td.log!;
  for (const entry of entryConThieu) if (entry.opNumber > backup.opNumber) backup.log.push(entry);
  backup.opNumber = backup.log.length === 0 ? 0 : backup.log[backup.log.length - 1]!.opNumber;
}
function xuLyMotButToanQuaPrimary(
  hd: HangDoiMang, tt: TrangThai, dh: DongHoAo,
  primary: Replica, cacBackupConSong: Replica[],
  bt: ButToan, tyLeMatPhanTram: number,
): boolean {
  const entry = nhanYeuCauTuClient(primary, bt);
  const phieuTheoOp = new Map<number, Set<number>>();
  const daNhanPhanHoi = new Set<number>();
  let daCommit = false;
  const thoiDiemBatDau = dh.hienTai;
  const NGUONG_GUI_LAI = 30;
  const SO_LAN_THU_TOI_DA = 6;
  for (let lan = 0; lan < SO_LAN_THU_TOI_DA && !daCommit; lan++) {
    if (lan > 0 && !kiemTraCanGuiLai(dh, thoiDiemBatDau, NGUONG_GUI_LAI)) tienToi(dh, NGUONG_GUI_LAI);
    for (const backup of cacBackupConSong) {
      if (daNhanPhanHoi.has(backup.chiSo)) continue;
      if (matGoiTheoSeed(tt, tyLeMatPhanTram)) continue;
      if (entry.opNumber > backup.opNumber + 1) {
        const conThieu = primary.log.filter((e) => e.opNumber > backup.opNumber && e.opNumber < entry.opNumber);
        xuLyPhanHoiDongBo(backup, { loai: 'PhanHoiDongBo', tu: primary.chiSo, den: backup.chiSo, viewNumber: primary.viewNumber, log: conThieu });
      }
      if (backup.opNumber < entry.opNumber) {
        xuLyPrepare(hd, tt, dh, backup, { loai: 'Prepare', tu: primary.chiSo, den: backup.chiSo, viewNumber: primary.viewNumber, opNumber: entry.opNumber, bt: entry.bt });
      } else {
        const doTreLai = soNguyenTrongKhoang(tt, 1, 20);
        guiThongDiep(hd, dh.hienTai + doTreLai, { loai: 'PrepareOk', tu: backup.chiSo, den: primary.chiSo, viewNumber: backup.viewNumber, opNumber: entry.opNumber });
      }
    }
    let sk: SuKienMang | undefined;
    while ((sk = layThongDiepTiepTheo(hd)) !== undefined) {
      if (matGoiTheoSeed(tt, tyLeMatPhanTram)) continue;
      daNhanPhanHoi.add(sk.td.tu);
      if (nhanPrepareOk(primary, phieuTheoOp, sk.td)) daCommit = true;
    }
  }
  if (daCommit) {
    for (const backup of cacBackupConSong) {
      if (kiemTraCanDongBo(backup, primary.opNumber)) {
        const conThieu = primary.log.filter((e) => e.opNumber > backup.opNumber);
        xuLyPhanHoiDongBo(backup, { loai: 'PhanHoiDongBo', tu: primary.chiSo, den: backup.chiSo, viewNumber: primary.viewNumber, log: conThieu });
      }
      xuLyCommit(backup, { loai: 'Commit', tu: primary.chiSo, den: backup.chiSo, viewNumber: primary.viewNumber, commitNumber: primary.commitNumber });
    }
  }
  return daCommit;
}
function xacNhanHoiTu(a: HeThongSoCai, b: HeThongSoCai): boolean {
  return JSON.stringify([...a.cacTaiKhoan.entries()].sort()) === JSON.stringify([...b.cacTaiKhoan.entries()].sort());
}

function chayKichBanByteLedgerNhoGon(seed: bigint): { canBang: boolean; hoiTu: boolean } {
  const cs = taoChinhSachLoi(seed);
  const dh = taoDongHoAo();
  const hd = taoHangDoiMang();
  const N = 3;
  const TY_LE_MAT = 30;
  const NGUON_NGOAI = 0, TK1 = 1, TK2 = 2;
  const r0 = taoReplica(0, N, [NGUON_NGOAI, TK1, TK2]);
  const r1 = taoReplica(1, N, [NGUON_NGOAI, TK1, TK2]);
  const r2 = taoReplica(2, N, [NGUON_NGOAI, TK1, TK2]);
  const diskTheoChiSo = new Map<number, SimDisk>([[1, new SimDisk(4)], [2, new SimDisk(4)]]);
  xuLyMotButToanQuaPrimary(hd, cs.tt, dh, r0, [r1, r2], { id: 1, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 1000 }, TY_LE_MAT);
  for (const b of [r1, r2]) {
    const disk = diskTheoChiSo.get(b.chiSo)!;
    apDungLoiDiaTheoChinhSach(cs, disk, 0);
    disk.write(0, new Uint8Array([9, 9, 9, 9]));
    disk.fsync();
  }
  xuLyMotButToanQuaPrimary(hd, cs.tt, dh, r0, [r1, r2], { id: 2, debitAccountId: TK1, creditAccountId: TK2, amount: 400 }, TY_LE_MAT);
  for (const b of [r1, r2]) {
    const disk = diskTheoChiSo.get(b.chiSo)!;
    apDungLoiDiaTheoChinhSach(cs, disk, 0);
    disk.write(0, new Uint8Array([9, 9, 9, 9]));
    disk.fsync();
  }
  return {
    canBang: heThongCanBang(r0.soCai) && heThongCanBang(r1.soCai) && heThongCanBang(r2.soCai),
    hoiTu: xacNhanHoiTu(r0.soCai, r1.soCai) && xacNhanHoiTu(r0.soCai, r2.soCai),
  };
}

function phanLoaiViPham(kq: { canBang: boolean; hoiTu: boolean }): 'canBang' | 'hoiTu' | null {
  ___
}
function timSeedLamVoMotTrongHaiBatBien(soLuongSeedThu: number): { seed: bigint; loaiVo: 'canBang' | 'hoiTu' } | null {
  for (let i = 0; i < soLuongSeedThu; i++) {
    const seed = BigInt(i);
    const loaiVo = phanLoaiViPham(chayKichBanByteLedgerNhoGon(seed));
    if (loaiVo !== null) return { seed, loaiVo };
  }
  return null;
}

console.log(JSON.stringify(timSeedLamVoMotTrongHaiBatBien(30)));
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
function matGoiTheoSeed(tt: TrangThai, tyLeMatPhanTram: number): boolean {
  return soNguyenTrongKhoang(tt, 0, 100) < tyLeMatPhanTram;
}
interface DongHoAo { hienTai: number; }
function taoDongHoAo(): DongHoAo { return { hienTai: 0 }; }
function tienToi(dh: DongHoAo, soMs: number): void { dh.hienTai += soMs; }

class SimDisk {
  readonly kichThuocSector: number;
  private readonly cache = new Map<number, Uint8Array>();
  private matFsyncKeTiep = false;
  private readonly tornSector = new Map<number, number>();
  constructor(kichThuocSector: number) { this.kichThuocSector = kichThuocSector; }
  write(sector: number, data: Uint8Array): void { this.cache.set(sector, data.slice()); }
  fsync(): void {
    if (this.matFsyncKeTiep) { this.matFsyncKeTiep = false; return; }
    this.tornSector.clear();
    this.cache.clear();
  }
  boQuaFsyncKeTiep(): void { this.matFsyncKeTiep = true; }
  danhDauTornGhi(sector: number, n: number): void { this.tornSector.set(sector, n); }
}
interface ChinhSachLoi { tt: TrangThai; }
function taoChinhSachLoi(seed: bigint): ChinhSachLoi { return { tt: gieoHat(seed) }; }
function apDungLoiDiaTheoChinhSach(cs: ChinhSachLoi, disk: SimDisk, sector: number): number {
  const kieu = soNguyenTrongKhoang(cs.tt, 0, 3);
  if (kieu === 1) disk.boQuaFsyncKeTiep();
  else if (kieu === 2) disk.danhDauTornGhi(sector, Math.floor(disk.kichThuocSector / 2));
  return kieu;
}

interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; debitsPending: number; creditsPending: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
function taoTaiKhoan(id: number): TaiKhoan { return { id, debitsPosted: 0, creditsPosted: 0, debitsPending: 0, creditsPending: 0 }; }
interface HeThongSoCai {
  cacTaiKhoan: Map<number, TaiKhoan>;
  cacPendingDangCho: Map<number, ButToan>;
  dsIdDaXuLy: Set<number>;
}
function taoHeThongSoCai(cacId: number[]): HeThongSoCai {
  const cacTaiKhoan = new Map<number, TaiKhoan>();
  for (const id of cacId) cacTaiKhoan.set(id, taoTaiKhoan(id));
  return { cacTaiKhoan, cacPendingDangCho: new Map(), dsIdDaXuLy: new Set() };
}
function heThongCanBang(soCai: HeThongSoCai): boolean {
  let no = 0, co = 0;
  for (const [, tk] of soCai.cacTaiKhoan) { no += tk.debitsPosted; co += tk.creditsPosted; }
  return no === co;
}
function apDungQuaSoCai(soCai: HeThongSoCai, bt: ButToan): boolean {
  if (soCai.dsIdDaXuLy.has(bt.id)) return true;
  soCai.dsIdDaXuLy.add(bt.id);
  soCai.cacTaiKhoan.get(bt.debitAccountId)!.debitsPosted += bt.amount;
  soCai.cacTaiKhoan.get(bt.creditAccountId)!.creditsPosted += bt.amount;
  return true;
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

interface ThongDiep {
  loai: 'Prepare' | 'PrepareOk' | 'Commit' | 'PhanHoiDongBo';
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
function soSanhSuKienMang(a: SuKienMang, b: SuKienMang): number {
  if (a.thoiDiem !== b.thoiDiem) return a.thoiDiem - b.thoiDiem;
  return a.thuTuChen - b.thuTuChen;
}
function layThongDiepTiepTheo(hd: HangDoiMang): SuKienMang | undefined {
  if (hd.danhSach.length === 0) return undefined;
  let idxNhoNhat = 0;
  for (let i = 1; i < hd.danhSach.length; i++) {
    if (soSanhSuKienMang(hd.danhSach[i]!, hd.danhSach[idxNhoNhat]!) < 0) idxNhoNhat = i;
  }
  return hd.danhSach.splice(idxNhoNhat, 1)[0];
}
function nhanYeuCauTuClient(primary: Replica, bt: ButToan): LogEntry {
  primary.opNumber += 1;
  const entry: LogEntry = { opNumber: primary.opNumber, bt };
  primary.log.push(entry);
  return entry;
}
function xuLyPrepare(hd: HangDoiMang, tt: TrangThai, dh: DongHoAo, backup: Replica, td: ThongDiep): void {
  const entry: LogEntry = { opNumber: td.opNumber!, bt: td.bt! };
  backup.log.push(entry);
  backup.opNumber = td.opNumber!;
  const doTre = soNguyenTrongKhoang(tt, 1, 20);
  guiThongDiep(hd, dh.hienTai + doTre, { loai: 'PrepareOk', tu: backup.chiSo, den: td.tu, viewNumber: backup.viewNumber, opNumber: td.opNumber! });
}
function nhanPrepareOk(primary: Replica, phieuTheoOp: Map<number, Set<number>>, td: ThongDiep): boolean {
  const opNumber = td.opNumber!;
  let phieu = phieuTheoOp.get(opNumber);
  if (!phieu) { phieu = new Set<number>([primary.chiSo]); phieuTheoOp.set(opNumber, phieu); }
  phieu.add(td.tu);
  if (phieu.size < nguongQuorum(primary.tongSo)) return false;
  if (opNumber !== primary.commitNumber + 1) return false;
  const entry = primary.log.find((e) => e.opNumber === opNumber)!;
  apDungQuaSoCai(primary.soCai, entry.bt);
  primary.commitNumber = opNumber;
  return true;
}
function xuLyCommit(backup: Replica, td: ThongDiep): void {
  const commitMoi = td.commitNumber!;
  for (let op = backup.commitNumber + 1; op <= commitMoi; op++) {
    const entry = backup.log.find((e) => e.opNumber === op)!;
    apDungQuaSoCai(backup.soCai, entry.bt);
  }
  backup.commitNumber = commitMoi;
}
function kiemTraCanGuiLai(dh: DongHoAo, thoiDiemGuiLanDau: number, nguongTimeoutMs: number): boolean {
  return dh.hienTai - thoiDiemGuiLanDau >= nguongTimeoutMs;
}
function kiemTraCanDongBo(backup: Replica, opNumberMoiNhatDaBiet: number): boolean {
  return backup.opNumber < opNumberMoiNhatDaBiet;
}
function xuLyPhanHoiDongBo(backup: Replica, td: ThongDiep): void {
  const entryConThieu = td.log!;
  for (const entry of entryConThieu) if (entry.opNumber > backup.opNumber) backup.log.push(entry);
  backup.opNumber = backup.log.length === 0 ? 0 : backup.log[backup.log.length - 1]!.opNumber;
}
function xuLyMotButToanQuaPrimary(
  hd: HangDoiMang, tt: TrangThai, dh: DongHoAo,
  primary: Replica, cacBackupConSong: Replica[],
  bt: ButToan, tyLeMatPhanTram: number,
): boolean {
  const entry = nhanYeuCauTuClient(primary, bt);
  const phieuTheoOp = new Map<number, Set<number>>();
  const daNhanPhanHoi = new Set<number>();
  let daCommit = false;
  const thoiDiemBatDau = dh.hienTai;
  const NGUONG_GUI_LAI = 30;
  const SO_LAN_THU_TOI_DA = 6;
  for (let lan = 0; lan < SO_LAN_THU_TOI_DA && !daCommit; lan++) {
    if (lan > 0 && !kiemTraCanGuiLai(dh, thoiDiemBatDau, NGUONG_GUI_LAI)) tienToi(dh, NGUONG_GUI_LAI);
    for (const backup of cacBackupConSong) {
      if (daNhanPhanHoi.has(backup.chiSo)) continue;
      if (matGoiTheoSeed(tt, tyLeMatPhanTram)) continue;
      if (entry.opNumber > backup.opNumber + 1) {
        const conThieu = primary.log.filter((e) => e.opNumber > backup.opNumber && e.opNumber < entry.opNumber);
        xuLyPhanHoiDongBo(backup, { loai: 'PhanHoiDongBo', tu: primary.chiSo, den: backup.chiSo, viewNumber: primary.viewNumber, log: conThieu });
      }
      if (backup.opNumber < entry.opNumber) {
        xuLyPrepare(hd, tt, dh, backup, { loai: 'Prepare', tu: primary.chiSo, den: backup.chiSo, viewNumber: primary.viewNumber, opNumber: entry.opNumber, bt: entry.bt });
      } else {
        const doTreLai = soNguyenTrongKhoang(tt, 1, 20);
        guiThongDiep(hd, dh.hienTai + doTreLai, { loai: 'PrepareOk', tu: backup.chiSo, den: primary.chiSo, viewNumber: backup.viewNumber, opNumber: entry.opNumber });
      }
    }
    let sk: SuKienMang | undefined;
    while ((sk = layThongDiepTiepTheo(hd)) !== undefined) {
      if (matGoiTheoSeed(tt, tyLeMatPhanTram)) continue;
      daNhanPhanHoi.add(sk.td.tu);
      if (nhanPrepareOk(primary, phieuTheoOp, sk.td)) daCommit = true;
    }
  }
  if (daCommit) {
    for (const backup of cacBackupConSong) {
      if (kiemTraCanDongBo(backup, primary.opNumber)) {
        const conThieu = primary.log.filter((e) => e.opNumber > backup.opNumber);
        xuLyPhanHoiDongBo(backup, { loai: 'PhanHoiDongBo', tu: primary.chiSo, den: backup.chiSo, viewNumber: primary.viewNumber, log: conThieu });
      }
      xuLyCommit(backup, { loai: 'Commit', tu: primary.chiSo, den: backup.chiSo, viewNumber: primary.viewNumber, commitNumber: primary.commitNumber });
    }
  }
  return daCommit;
}
function xacNhanHoiTu(a: HeThongSoCai, b: HeThongSoCai): boolean {
  return JSON.stringify([...a.cacTaiKhoan.entries()].sort()) === JSON.stringify([...b.cacTaiKhoan.entries()].sort());
}

function chayKichBanByteLedgerNhoGon(seed: bigint): { canBang: boolean; hoiTu: boolean } {
  const cs = taoChinhSachLoi(seed);
  const dh = taoDongHoAo();
  const hd = taoHangDoiMang();
  const N = 3;
  const TY_LE_MAT = 30;
  const NGUON_NGOAI = 0, TK1 = 1, TK2 = 2;
  const r0 = taoReplica(0, N, [NGUON_NGOAI, TK1, TK2]);
  const r1 = taoReplica(1, N, [NGUON_NGOAI, TK1, TK2]);
  const r2 = taoReplica(2, N, [NGUON_NGOAI, TK1, TK2]);
  const diskTheoChiSo = new Map<number, SimDisk>([[1, new SimDisk(4)], [2, new SimDisk(4)]]);
  xuLyMotButToanQuaPrimary(hd, cs.tt, dh, r0, [r1, r2], { id: 1, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 1000 }, TY_LE_MAT);
  for (const b of [r1, r2]) {
    const disk = diskTheoChiSo.get(b.chiSo)!;
    apDungLoiDiaTheoChinhSach(cs, disk, 0);
    disk.write(0, new Uint8Array([9, 9, 9, 9]));
    disk.fsync();
  }
  xuLyMotButToanQuaPrimary(hd, cs.tt, dh, r0, [r1, r2], { id: 2, debitAccountId: TK1, creditAccountId: TK2, amount: 400 }, TY_LE_MAT);
  for (const b of [r1, r2]) {
    const disk = diskTheoChiSo.get(b.chiSo)!;
    apDungLoiDiaTheoChinhSach(cs, disk, 0);
    disk.write(0, new Uint8Array([9, 9, 9, 9]));
    disk.fsync();
  }
  return {
    canBang: heThongCanBang(r0.soCai) && heThongCanBang(r1.soCai) && heThongCanBang(r2.soCai),
    hoiTu: xacNhanHoiTu(r0.soCai, r1.soCai) && xacNhanHoiTu(r0.soCai, r2.soCai),
  };
}

function phanLoaiViPham(kq: { canBang: boolean; hoiTu: boolean }): 'canBang' | 'hoiTu' | null {
  if (!kq.canBang) return 'canBang';
  if (!kq.hoiTu) return 'hoiTu';
  return null;
}
function timSeedLamVoMotTrongHaiBatBien(soLuongSeedThu: number): { seed: bigint; loaiVo: 'canBang' | 'hoiTu' } | null {
  for (let i = 0; i < soLuongSeedThu; i++) {
    const seed = BigInt(i);
    const loaiVo = phanLoaiViPham(chayKichBanByteLedgerNhoGon(seed));
    if (loaiVo !== null) return { seed, loaiVo };
  }
  return null;
}

console.log(JSON.stringify(timSeedLamVoMotTrongHaiBatBien(30)));
```

```typescript title=test
for (let i = 0; i < 30; i++) {
  const kq = chayKichBanByteLedgerNhoGon(BigInt(i));
  if (!kq.canBang) throw new Error(`seed=${i}n: VI PHAM heThongCanBang`);
  if (!kq.hoiTu) throw new Error(`seed=${i}n: KHONG hoi tu`);
}
const ketQuaQuet = timSeedLamVoMotTrongHaiBatBien(30);
if (ketQuaQuet !== null) throw new Error("quet 30 seed dau tien KHONG duoc tim thay vi pham nao -- phai tra ve null");
if (timSeedLamVoMotTrongHaiBatBien(0) !== null) throw new Error("quet 0 seed (pham vi rong) phai tra ve null");

const lan1 = chayKichBanByteLedgerNhoGon(7n);
const lan2 = chayKichBanByteLedgerNhoGon(7n);
if (JSON.stringify(lan1) !== JSON.stringify(lan2)) throw new Error("goi lai CUNG seed=7n phai cho DUNG cung ket qua (tai hien tuyet doi)");

if (phanLoaiViPham({ canBang: true, hoiTu: true }) !== null) throw new Error("ca hai bat bien giu vung thi phai tra ve null");
if (phanLoaiViPham({ canBang: false, hoiTu: true }) !== 'canBang') throw new Error("vo heThongCanBang (canBang=false) phai tra ve 'canBang', BAT KE hoiTu la gi");
if (phanLoaiViPham({ canBang: true, hoiTu: false }) !== 'hoiTu') throw new Error("vo hoi tu (hoiTu=false, nhung canBang van true) phai tra ve 'hoiTu'");
if (phanLoaiViPham({ canBang: false, hoiTu: false }) !== 'canBang') throw new Error("vo CA HAI thi phai bao canBang TRUOC (kiem tra canBang truoc hoiTu)");
```

:::hints
- kind: attention
  body: "Neu !kq.canBang tra ve 'canBang'; neu !kq.hoiTu tra ve 'hoiTu'; cuoi cung tra ve null -- ba dong."
- kind: strategy
  body: "if (!kq.canBang) return 'canBang'; if (!kq.hoiTu) return 'hoiTu'; return null;"
- kind: one-line
  body: "if (!kq.canBang) return 'canBang'; if (!kq.hoiTu) return 'hoiTu'; return null;"
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: tests
  timeoutMs: 8000
- tier: output
  match: contains
  expect: "null"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không seed nào (trong 30 seed đầu) làm vỡ được ByteLedger. Nhưng
trước khi tự tin tuyên bố "đáng tin cậy" — hãy thành thật: mô phỏng
NÀY nói dối ở đâu, so với TigerBeetle thật?
::::

::::reflect{#nghi-lai}
`phanLoaiViPham` VÀ `timSeedLamVoMotTrongHaiBatBien` không giới thiệu
khái NIỆM mới — chúng LÀ `timSeedLamVoBatBien` (BOSS q18) mở rộng
THÊM một chiều: KHÔNG chỉ MỘT bất biến, mà HAI, và KHÔNG chỉ đĩa,
mà đĩa+mạng+VSR CÙNG dưới một seed. Bài học lớn NHẤT: "quét seed
không tìm thấy gì" LÀ một PHÁT hiện thật, có GIÁ trị — nhưng nó chỉ
đúng TRONG phạm vi ĐÃ quét, VÀ chỉ đo đúng những GÌ mô hình THẬT sự
mô phỏng. Ranh giới đó chính LÀ chủ đề bài tiếp theo.
::::

::::checkpoint{mastery=0.9}
::::
