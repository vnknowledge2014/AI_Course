---
id: co-so-du-lieu.byteledger-capstone.boss-byteledger-capstone
title: "BOSS — ByteLedger Capstone"
summary: "chayByteLedgerDayDu(seed) mô phỏng ĐẦY ĐỦ NHẤT: N=3, năm bút toán (nạp 1000, chuyển 400, chuyển 100, RETRY của bút toán 100 đó, chuyển 50 mới) tiêm CẢ lỗi đĩa (apDungLoiDiaTheoChinhSach) lẫn mạng (matGoiTheoSeed/độ trễ) dưới MỘT ChinhSachLoi.tt, primary chết CỐ ĐỊNH sau đúng 2 bút toán (view change lên view=1), MỘT backup lạc hậu tự động cần State Transfer giữa chừng. seed=3n: hoiTu=true, canBang=true, soDuTK1=650 (1000-400+100-50), soDuTK2=350 (400-100+50) -- retry (bút toán id=3, gửi HAI lần) không cộng thêm nhờ dsIdDaXuLy (kích thước 4, không phải 5). Lặp lại Y HỆT qua 5 lần chạy độc lập VÀ đúng với cả 30 seed đầu tiên (0..29) -- không seed nào làm vỡ hội tụ hay cân bằng."
locale: vi
track: co-so-du-lieu
module: byteledger-capstone
order: 10
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: []
requires: [db.audit-byteledger-noi-doi-cho-nao]
concepts: [db.boss-q21]
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
Chín bài — struct sổ cái đầy đủ, VSR idempotent, đĩa riêng cho từng
replica, chính sách lỗi cho cả cụm, phục hồi một replica, view change
dưới tải, retry xuyên view change, mini-VOPR, audit thành thật. Ráp
TẤT CẢ vào MỘT kịch bản: có gì chưa từng ráp chung một lúc không?
::::

::::explain{#boss-that}
`chayByteLedgerDayDu(seed)` mô phỏng ĐẦY đủ NHẤT của cả q21: `N=3`,
NĂM bút toán (nạp `1000`, chuyển `400`, chuyển `100`, RETRY của bút
toán `100` đó, chuyển `50` MỚI) tiêm CẢ lỗi đĩa (`apDungLoiDiaTheoChinhSach`,
bài 4) LẪN mạng (`matGoiTheoSeed`/độ trễ, q19 BOSS) dưới đúng MỘT
`ChinhSachLoi.tt`, primary chết Ở thời ĐIỂM CỐ định (SAU đúng `2` bút
toán — bài 9 điểm 4) rồi VIEW change lên `view=1`, MỘT backup lạc hậu
TỰ động cần State Transfer GIỮA chừng (bài 5/q19 bài 13, tích hợp
SẴN trong `xuLyMotButToanQuaPrimary`):

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
function soDuSoSach(tk: TaiKhoan): number { return tk.creditsPosted - tk.debitsPosted; }
interface HeThongSoCai { cacTaiKhoan: Map<number, TaiKhoan>; cacPendingDangCho: Map<number, ButToan>; dsIdDaXuLy: Set<number>; }
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
  loai: 'Prepare' | 'PrepareOk' | 'Commit' | 'StartViewChange' | 'StartView' | 'PhanHoiDongBo';
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
function phatHienVaNghiNgoPrimary(replica: Replica, dh: DongHoAo, thoiDiemNgheCuoi: number, nguongTimeoutMs: number): boolean {
  if (dh.hienTai - thoiDiemNgheCuoi <= nguongTimeoutMs) return false;
  replica.status = 'view-change';
  replica.viewNumber += 1;
  return true;
}
function nhanStartViewChange(replica: Replica, phieuTheoView: Map<number, Set<number>>, td: ThongDiep): boolean {
  let phieu = phieuTheoView.get(td.viewNumber);
  if (!phieu) { phieu = new Set<number>([replica.chiSo]); phieuTheoView.set(td.viewNumber, phieu); }
  phieu.add(td.tu);
  return phieu.size >= nguongQuorum(replica.tongSo);
}
function opCuaLog(log: LogEntry[]): number { return log.length === 0 ? 0 : log[log.length - 1]!.opNumber; }
function chonLogDayDuNhat(cacLog: LogEntry[][]): LogEntry[] {
  let logTotNhat = cacLog[0]!;
  for (let i = 1; i < cacLog.length; i++) {
    if (opCuaLog(cacLog[i]!) > opCuaLog(logTotNhat)) logTotNhat = cacLog[i]!;
  }
  return logTotNhat;
}
function xuLyStartView(replica: Replica, td: ThongDiep): void {
  replica.log = [...td.log!];
  replica.opNumber = opCuaLog(replica.log);
  replica.viewNumber = td.viewNumber;
  replica.commitNumber = td.commitNumber ?? replica.commitNumber;
  replica.status = 'normal';
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

function chayByteLedgerDayDu(seed: bigint): {
  hoiTu: boolean; canBang: boolean; soDuTK1: number; soDuTK2: number;
} {
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
  function ghiDiaChoBackup(b: Replica): void {
    const disk = diskTheoChiSo.get(b.chiSo)!;
    apDungLoiDiaTheoChinhSach(cs, disk, 0);
    disk.write(0, new Uint8Array([9, 9, 9, 9]));
    disk.fsync();
  }

  // op1, op2: qua primary r0 (view 0) -- ca r1 va r2 con song; MOI backup tu ghi dia rieng
  xuLyMotButToanQuaPrimary(hd, cs.tt, dh, r0, [r1, r2], { id: 1, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 1000 }, TY_LE_MAT);
  for (const b of [r1, r2]) ghiDiaChoBackup(b);
  xuLyMotButToanQuaPrimary(hd, cs.tt, dh, r0, [r1, r2], { id: 2, debitAccountId: TK1, creditAccountId: TK2, amount: 400 }, TY_LE_MAT);
  for (const b of [r1, r2]) ghiDiaChoBackup(b);

  // r0 "chet" -- thoi diem CO DINH (sau dung 2 but toan, bai 9 diem 4); CHI mang/dia moi do seed quyet dinh
  tienToi(dh, 999);
  phatHienVaNghiNgoPrimary(r1, dh, 0, 200);
  phatHienVaNghiNgoPrimary(r2, dh, 0, 200);

  const conSong = [r1, r2];
  const viewMoi = conSong[0]!.viewNumber;
  for (const r of conSong) r.viewNumber = viewMoi;
  const phieuTheoViewTheoChiSo = new Map<number, Map<number, Set<number>>>();
  for (const r of conSong) phieuTheoViewTheoChiSo.set(r.chiSo, new Map());
  for (const r of conSong) {
    const phieuMap = phieuTheoViewTheoChiSo.get(r.chiSo)!;
    for (const nguoiKhac of conSong) {
      if (nguoiKhac.chiSo === r.chiSo) continue;
      nhanStartViewChange(r, phieuMap, { loai: 'StartViewChange', tu: nguoiKhac.chiSo, den: r.chiSo, viewNumber: viewMoi });
    }
  }
  const chiSoPrimaryMoi = viewMoi % N;
  const primaryMoi = conSong.find((r) => r.chiSo === chiSoPrimaryMoi)!;
  const logDayDuNhat = chonLogDayDuNhat(conSong.map((r) => r.log));
  const commitNumberMoi = Math.max(...conSong.map((r) => r.commitNumber));
  for (const r of conSong) {
    const commitCu = r.commitNumber;
    xuLyStartView(r, { loai: 'StartView', tu: primaryMoi.chiSo, den: r.chiSo, viewNumber: viewMoi, log: logDayDuNhat, commitNumber: commitNumberMoi });
    for (let op = commitCu + 1; op <= commitNumberMoi; op++) {
      const e = r.log.find((x) => x.opNumber === op)!;
      apDungQuaSoCai(r.soCai, e.bt);
    }
  }

  // op3: qua primary moi, backup con lai duy nhat
  const backupConLai = conSong.filter((r) => r.chiSo !== primaryMoi.chiSo);
  const btOp3: ButToan = { id: 3, debitAccountId: TK2, creditAccountId: TK1, amount: 100 };
  xuLyMotButToanQuaPrimary(hd, cs.tt, dh, primaryMoi, backupConLai, btOp3, TY_LE_MAT);
  ghiDiaChoBackup(backupConLai[0]!);

  // op4: client GUI LAI dung btOp3 (id=3) -- idempotent retry xuyen view change
  xuLyMotButToanQuaPrimary(hd, cs.tt, dh, primaryMoi, backupConLai, btOp3, TY_LE_MAT);
  ghiDiaChoBackup(backupConLai[0]!);

  // op5: but toan MOI, sau view change
  xuLyMotButToanQuaPrimary(hd, cs.tt, dh, primaryMoi, backupConLai, { id: 5, debitAccountId: TK1, creditAccountId: TK2, amount: 50 }, TY_LE_MAT);
  ghiDiaChoBackup(backupConLai[0]!);

  const soCaiPrimaryMoi = primaryMoi.soCai;
  const soCaiBackup = backupConLai[0]!.soCai;
  return {
    hoiTu: xacNhanHoiTu(soCaiPrimaryMoi, soCaiBackup),
    canBang: heThongCanBang(soCaiPrimaryMoi) && heThongCanBang(soCaiBackup),
    soDuTK1: soDuSoSach(soCaiPrimaryMoi.cacTaiKhoan.get(TK1)!),
    soDuTK2: soDuSoSach(soCaiPrimaryMoi.cacTaiKhoan.get(TK2)!),
  };
}

console.log("seed=3n:", JSON.stringify(chayByteLedgerDayDu(3n)));
const ketQua5Lan: string[] = [];
for (let i = 0; i < 5; i++) ketQua5Lan.push(JSON.stringify(chayByteLedgerDayDu(3n)));
console.log("5 lan doc lap:", ketQua5Lan.every((x) => x === ketQua5Lan[0]) ? "GIONG HET" : "LECH");
```

```text title=readonly
seed=3n: {"hoiTu":true,"canBang":true,"soDuTK1":650,"soDuTK2":350}
5 lan doc lap: GIONG HET
```

`seed=3n` kể MỘT câu chuyện ĐẦY đủ: op1 (nạp `1000`) VÀ op2 (chuyển
`400`) qua trót LỌT dưới primary `r0` (VIEW `0`), mỗi backup TỰ ghi
log của MÌNH xuống đĩa RIÊNG (có thể dính lỗi ĐĨA, không ảnh hưởng
consensus — bài 8 đã CHỨNG minh). `r0` "chết" ĐÚNG sau `2` bút toán
(cố ĐỊNH — bài 9 điểm `4`); `r1` VÀ `r2` phát hiện, View Change đưa
`r1` LÊN làm primary `view=1`. op3 (chuyển `100`) qua trót lọt DƯỚI
primary MỚI — VÀ trong quá trình ĐÓ, `xuLyMotButToanQuaPrimary` tự
động phát hiện backup CÒN lại lạc hậu (do lỡ MỘT Prepare TRƯỚC đó) VÀ
tự State Transfer bù NGAY (không CẦN can thiệp thêm). op4 LÀ client
gửi LẠI đúng `btOp3` (`id=3`) — MỘT retry idempotent (bài 7): commit
THÊM một `opNumber` MỚI nhưng KHÔNG cộng thêm tiền. op5 (chuyển `50`)
LÀ bút toán THẬT sự mới. KẾT quả cuối: `soDuTK1 = 1000 - 400 + 100 -
50 = 650`, `soDuTK2 = 400 - 100 + 50 = 350` — VÀ `hoiTu: true`
(primary mới VÀ backup CÒN lại hội tụ đúng CÙNG sổ cái), lặp LẠI y
hệt qua `5` lần chạy ĐỘC lập.
::::

::::example{#don-gian-hoa-co-chu-dich}
Giống HỆT q19 BOSS: thời ĐIỂM `r0` "chết" LÀ CỐ định theo kịch bản
(sau ĐÚNG op1, op2) — KHÔNG do seed quyết định (bài 9 điểm `4` đã
chứng minh RIÊNG điều này). CHỈ các quyết định MẠNG (mất gói, độ
trễ) VÀ ĐĨA (lost fsync, torn write) mới TẤT định theo `seed`. Quét
`30` seed đầu tiên (`0n` đến `29n`): KHÔNG seed nào làm `hoiTu` hay
`canBang` sai — quorum(`N=3`)`=2` đủ CHỊU lỗi mất gói lẻ TẺ, một
primary chết, VÀ lỗi đĩa (không ảnh hưởng BỘ nhớ, bài 8/9) CỘNG dồn
CÙNG lúc.
::::

::::predict{#doan-neu-so-du-am commitOnce}
Giả sử (giả ĐỊNH, không liên quan seed) op5 LÀ chuyển `700` thay VÌ
`50` — VƯỢT quá số dư `TK1` CÓ lúc đó (`700` SAU op3, `1000-400+100`).
Vì `apDungQuaSoCai` (bài 2) KHÔNG hề kiểm tra số DƯ trước khi ghi
(khác `guiButToanAnToan` Ở q16 BOSS), CHUYỆN gì xảy RA với `canBang`
Ở kết quả CUỐI?
:::opt{correct}
VẪN `true` — `heThongCanBang` (bài 1) CHỈ kiểm tra tính đối XỨNG
nợ/có (mỗi bút toán LUÔN cộng CÙNG `amount` VÀO một `debitsPosted`
VÀ một `creditsPosted`), HOÀN toàn KHÔNG liên quan tới việc MỘT tài
khoản CÓ số dư âm hay không — giống HỆT bài học q16 BOSS đã dạy
:::
:::opt
`false` — số DƯ âm chắc chắn phá vỡ tính CÂN bằng nợ/có của CẢ hệ
thống
::why
Trực giác NÀY lặp LẠI đúng nhầm lẫn q16 BOSS đã CHỈ ra: "MỘT tài
khoản âm" VÀ "hệ thống mất cân bằng" LÀ hai khái NIỆM hoàn toàn khác
nhau.

Chỗ lệch: `apDungQuaSoCai` (bài 2, KHÔNG kế thừa `guiButToanAnToan`'s
kiểm tra số dư từ q16 bài 10) VẪN cộng ĐÚNG cùng `amount` vào MỘT
`debitsPosted` VÀ một `creditsPosted`, DÙ kết quả LÀM `TK1` âm. Tổng
nợ VẪN bằng tổng có — CHỈ MỘT tài khoản riêng lẻ "nợ NẦN". Đây chính
LÀ lý do ByteLedger (q21) — GIỐNG q16 BOSS — KHÔNG coi `heThongCanBang`
LÀ "an toàn tuyệt đối", CHỈ LÀ một trong NHIỀU bất biến cần kiểm.
::
:::
::::

::::code{#viet_xac_nhan_hoi_tu}
Hoàn thiện `xacNhanHoiTu` — so sánh nội dung HAI sổ cái (không phụ
thuộc thứ tự tạo/chèn CỦA `Map`), trả `true` NẾU giống hệt nhau.

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
function soDuSoSach(tk: TaiKhoan): number { return tk.creditsPosted - tk.debitsPosted; }
interface HeThongSoCai { cacTaiKhoan: Map<number, TaiKhoan>; cacPendingDangCho: Map<number, ButToan>; dsIdDaXuLy: Set<number>; }
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
  loai: 'Prepare' | 'PrepareOk' | 'Commit' | 'StartViewChange' | 'StartView' | 'PhanHoiDongBo';
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
function phatHienVaNghiNgoPrimary(replica: Replica, dh: DongHoAo, thoiDiemNgheCuoi: number, nguongTimeoutMs: number): boolean {
  if (dh.hienTai - thoiDiemNgheCuoi <= nguongTimeoutMs) return false;
  replica.status = 'view-change';
  replica.viewNumber += 1;
  return true;
}
function nhanStartViewChange(replica: Replica, phieuTheoView: Map<number, Set<number>>, td: ThongDiep): boolean {
  let phieu = phieuTheoView.get(td.viewNumber);
  if (!phieu) { phieu = new Set<number>([replica.chiSo]); phieuTheoView.set(td.viewNumber, phieu); }
  phieu.add(td.tu);
  return phieu.size >= nguongQuorum(replica.tongSo);
}
function opCuaLog(log: LogEntry[]): number { return log.length === 0 ? 0 : log[log.length - 1]!.opNumber; }
function chonLogDayDuNhat(cacLog: LogEntry[][]): LogEntry[] {
  let logTotNhat = cacLog[0]!;
  for (let i = 1; i < cacLog.length; i++) {
    if (opCuaLog(cacLog[i]!) > opCuaLog(logTotNhat)) logTotNhat = cacLog[i]!;
  }
  return logTotNhat;
}
function xuLyStartView(replica: Replica, td: ThongDiep): void {
  replica.log = [...td.log!];
  replica.opNumber = opCuaLog(replica.log);
  replica.viewNumber = td.viewNumber;
  replica.commitNumber = td.commitNumber ?? replica.commitNumber;
  replica.status = 'normal';
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
  ___
}

function chayByteLedgerDayDu(seed: bigint): {
  hoiTu: boolean; canBang: boolean; soDuTK1: number; soDuTK2: number;
} {
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
  function ghiDiaChoBackup(b: Replica): void {
    const disk = diskTheoChiSo.get(b.chiSo)!;
    apDungLoiDiaTheoChinhSach(cs, disk, 0);
    disk.write(0, new Uint8Array([9, 9, 9, 9]));
    disk.fsync();
  }

  xuLyMotButToanQuaPrimary(hd, cs.tt, dh, r0, [r1, r2], { id: 1, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 1000 }, TY_LE_MAT);
  for (const b of [r1, r2]) ghiDiaChoBackup(b);
  xuLyMotButToanQuaPrimary(hd, cs.tt, dh, r0, [r1, r2], { id: 2, debitAccountId: TK1, creditAccountId: TK2, amount: 400 }, TY_LE_MAT);
  for (const b of [r1, r2]) ghiDiaChoBackup(b);

  tienToi(dh, 999);
  phatHienVaNghiNgoPrimary(r1, dh, 0, 200);
  phatHienVaNghiNgoPrimary(r2, dh, 0, 200);

  const conSong = [r1, r2];
  const viewMoi = conSong[0]!.viewNumber;
  for (const r of conSong) r.viewNumber = viewMoi;
  const phieuTheoViewTheoChiSo = new Map<number, Map<number, Set<number>>>();
  for (const r of conSong) phieuTheoViewTheoChiSo.set(r.chiSo, new Map());
  for (const r of conSong) {
    const phieuMap = phieuTheoViewTheoChiSo.get(r.chiSo)!;
    for (const nguoiKhac of conSong) {
      if (nguoiKhac.chiSo === r.chiSo) continue;
      nhanStartViewChange(r, phieuMap, { loai: 'StartViewChange', tu: nguoiKhac.chiSo, den: r.chiSo, viewNumber: viewMoi });
    }
  }
  const chiSoPrimaryMoi = viewMoi % N;
  const primaryMoi = conSong.find((r) => r.chiSo === chiSoPrimaryMoi)!;
  const logDayDuNhat = chonLogDayDuNhat(conSong.map((r) => r.log));
  const commitNumberMoi = Math.max(...conSong.map((r) => r.commitNumber));
  for (const r of conSong) {
    const commitCu = r.commitNumber;
    xuLyStartView(r, { loai: 'StartView', tu: primaryMoi.chiSo, den: r.chiSo, viewNumber: viewMoi, log: logDayDuNhat, commitNumber: commitNumberMoi });
    for (let op = commitCu + 1; op <= commitNumberMoi; op++) {
      const e = r.log.find((x) => x.opNumber === op)!;
      apDungQuaSoCai(r.soCai, e.bt);
    }
  }

  const backupConLai = conSong.filter((r) => r.chiSo !== primaryMoi.chiSo);
  const btOp3: ButToan = { id: 3, debitAccountId: TK2, creditAccountId: TK1, amount: 100 };
  xuLyMotButToanQuaPrimary(hd, cs.tt, dh, primaryMoi, backupConLai, btOp3, TY_LE_MAT);
  ghiDiaChoBackup(backupConLai[0]!);

  xuLyMotButToanQuaPrimary(hd, cs.tt, dh, primaryMoi, backupConLai, btOp3, TY_LE_MAT);
  ghiDiaChoBackup(backupConLai[0]!);

  xuLyMotButToanQuaPrimary(hd, cs.tt, dh, primaryMoi, backupConLai, { id: 5, debitAccountId: TK1, creditAccountId: TK2, amount: 50 }, TY_LE_MAT);
  ghiDiaChoBackup(backupConLai[0]!);

  const soCaiPrimaryMoi = primaryMoi.soCai;
  const soCaiBackup = backupConLai[0]!.soCai;
  return {
    hoiTu: xacNhanHoiTu(soCaiPrimaryMoi, soCaiBackup),
    canBang: heThongCanBang(soCaiPrimaryMoi) && heThongCanBang(soCaiBackup),
    soDuTK1: soDuSoSach(soCaiPrimaryMoi.cacTaiKhoan.get(TK1)!),
    soDuTK2: soDuSoSach(soCaiPrimaryMoi.cacTaiKhoan.get(TK2)!),
  };
}

console.log(JSON.stringify(chayByteLedgerDayDu(3n)));
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
function soDuSoSach(tk: TaiKhoan): number { return tk.creditsPosted - tk.debitsPosted; }
interface HeThongSoCai { cacTaiKhoan: Map<number, TaiKhoan>; cacPendingDangCho: Map<number, ButToan>; dsIdDaXuLy: Set<number>; }
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
  loai: 'Prepare' | 'PrepareOk' | 'Commit' | 'StartViewChange' | 'StartView' | 'PhanHoiDongBo';
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
function phatHienVaNghiNgoPrimary(replica: Replica, dh: DongHoAo, thoiDiemNgheCuoi: number, nguongTimeoutMs: number): boolean {
  if (dh.hienTai - thoiDiemNgheCuoi <= nguongTimeoutMs) return false;
  replica.status = 'view-change';
  replica.viewNumber += 1;
  return true;
}
function nhanStartViewChange(replica: Replica, phieuTheoView: Map<number, Set<number>>, td: ThongDiep): boolean {
  let phieu = phieuTheoView.get(td.viewNumber);
  if (!phieu) { phieu = new Set<number>([replica.chiSo]); phieuTheoView.set(td.viewNumber, phieu); }
  phieu.add(td.tu);
  return phieu.size >= nguongQuorum(replica.tongSo);
}
function opCuaLog(log: LogEntry[]): number { return log.length === 0 ? 0 : log[log.length - 1]!.opNumber; }
function chonLogDayDuNhat(cacLog: LogEntry[][]): LogEntry[] {
  let logTotNhat = cacLog[0]!;
  for (let i = 1; i < cacLog.length; i++) {
    if (opCuaLog(cacLog[i]!) > opCuaLog(logTotNhat)) logTotNhat = cacLog[i]!;
  }
  return logTotNhat;
}
function xuLyStartView(replica: Replica, td: ThongDiep): void {
  replica.log = [...td.log!];
  replica.opNumber = opCuaLog(replica.log);
  replica.viewNumber = td.viewNumber;
  replica.commitNumber = td.commitNumber ?? replica.commitNumber;
  replica.status = 'normal';
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

function chayByteLedgerDayDu(seed: bigint): {
  hoiTu: boolean; canBang: boolean; soDuTK1: number; soDuTK2: number;
} {
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
  function ghiDiaChoBackup(b: Replica): void {
    const disk = diskTheoChiSo.get(b.chiSo)!;
    apDungLoiDiaTheoChinhSach(cs, disk, 0);
    disk.write(0, new Uint8Array([9, 9, 9, 9]));
    disk.fsync();
  }

  xuLyMotButToanQuaPrimary(hd, cs.tt, dh, r0, [r1, r2], { id: 1, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 1000 }, TY_LE_MAT);
  for (const b of [r1, r2]) ghiDiaChoBackup(b);
  xuLyMotButToanQuaPrimary(hd, cs.tt, dh, r0, [r1, r2], { id: 2, debitAccountId: TK1, creditAccountId: TK2, amount: 400 }, TY_LE_MAT);
  for (const b of [r1, r2]) ghiDiaChoBackup(b);

  tienToi(dh, 999);
  phatHienVaNghiNgoPrimary(r1, dh, 0, 200);
  phatHienVaNghiNgoPrimary(r2, dh, 0, 200);

  const conSong = [r1, r2];
  const viewMoi = conSong[0]!.viewNumber;
  for (const r of conSong) r.viewNumber = viewMoi;
  const phieuTheoViewTheoChiSo = new Map<number, Map<number, Set<number>>>();
  for (const r of conSong) phieuTheoViewTheoChiSo.set(r.chiSo, new Map());
  for (const r of conSong) {
    const phieuMap = phieuTheoViewTheoChiSo.get(r.chiSo)!;
    for (const nguoiKhac of conSong) {
      if (nguoiKhac.chiSo === r.chiSo) continue;
      nhanStartViewChange(r, phieuMap, { loai: 'StartViewChange', tu: nguoiKhac.chiSo, den: r.chiSo, viewNumber: viewMoi });
    }
  }
  const chiSoPrimaryMoi = viewMoi % N;
  const primaryMoi = conSong.find((r) => r.chiSo === chiSoPrimaryMoi)!;
  const logDayDuNhat = chonLogDayDuNhat(conSong.map((r) => r.log));
  const commitNumberMoi = Math.max(...conSong.map((r) => r.commitNumber));
  for (const r of conSong) {
    const commitCu = r.commitNumber;
    xuLyStartView(r, { loai: 'StartView', tu: primaryMoi.chiSo, den: r.chiSo, viewNumber: viewMoi, log: logDayDuNhat, commitNumber: commitNumberMoi });
    for (let op = commitCu + 1; op <= commitNumberMoi; op++) {
      const e = r.log.find((x) => x.opNumber === op)!;
      apDungQuaSoCai(r.soCai, e.bt);
    }
  }

  const backupConLai = conSong.filter((r) => r.chiSo !== primaryMoi.chiSo);
  const btOp3: ButToan = { id: 3, debitAccountId: TK2, creditAccountId: TK1, amount: 100 };
  xuLyMotButToanQuaPrimary(hd, cs.tt, dh, primaryMoi, backupConLai, btOp3, TY_LE_MAT);
  ghiDiaChoBackup(backupConLai[0]!);

  xuLyMotButToanQuaPrimary(hd, cs.tt, dh, primaryMoi, backupConLai, btOp3, TY_LE_MAT);
  ghiDiaChoBackup(backupConLai[0]!);

  xuLyMotButToanQuaPrimary(hd, cs.tt, dh, primaryMoi, backupConLai, { id: 5, debitAccountId: TK1, creditAccountId: TK2, amount: 50 }, TY_LE_MAT);
  ghiDiaChoBackup(backupConLai[0]!);

  const soCaiPrimaryMoi = primaryMoi.soCai;
  const soCaiBackup = backupConLai[0]!.soCai;
  return {
    hoiTu: xacNhanHoiTu(soCaiPrimaryMoi, soCaiBackup),
    canBang: heThongCanBang(soCaiPrimaryMoi) && heThongCanBang(soCaiBackup),
    soDuTK1: soDuSoSach(soCaiPrimaryMoi.cacTaiKhoan.get(TK1)!),
    soDuTK2: soDuSoSach(soCaiPrimaryMoi.cacTaiKhoan.get(TK2)!),
  };
}

console.log(JSON.stringify(chayByteLedgerDayDu(3n)));
```

```typescript title=test
const kqChinh = chayByteLedgerDayDu(3n);
if (JSON.stringify(kqChinh) !== '{"hoiTu":true,"canBang":true,"soDuTK1":650,"soDuTK2":350}') {
  throw new Error("seed=3n phai cho DUNG {hoiTu:true,canBang:true,soDuTK1:650,soDuTK2:350}");
}
const ketQua5LanT: string[] = [];
for (let i = 0; i < 5; i++) ketQua5LanT.push(JSON.stringify(chayByteLedgerDayDu(3n)));
if (!ketQua5LanT.every((x) => x === ketQua5LanT[0])) throw new Error("goi lai CUNG seed=3n nhieu lan phai cho KET QUA giong het nhau (tai hien tuyet doi)");

for (let i = 0; i < 30; i++) {
  const kq = chayByteLedgerDayDu(BigInt(i));
  if (!kq.hoiTu) throw new Error(`seed=${i}n: primary moi VA backup KHONG hoi tu ve cung so cai`);
  if (!kq.canBang) throw new Error(`seed=${i}n: he thong VI PHAM bat bien tong no = tong co`);
}

const mA = taoHeThongSoCai([1, 2]);
mA.cacTaiKhoan.get(1)!.creditsPosted = 100;
mA.cacTaiKhoan.get(2)!.debitsPosted = 50;
const mB = taoHeThongSoCai([2, 1]);
mB.cacTaiKhoan.get(2)!.debitsPosted = 50;
mB.cacTaiKhoan.get(1)!.creditsPosted = 100;
if (xacNhanHoiTu(mA, mB) !== true) throw new Error("hai so cai CUNG noi dung, khac thu tu tao/chen, phai la HOI TU (true)");

const mC = taoHeThongSoCai([1, 2]);
mC.cacTaiKhoan.get(1)!.creditsPosted = 100;
mC.cacTaiKhoan.get(2)!.debitsPosted = 51;
if (xacNhanHoiTu(mA, mC) !== false) throw new Error("hai so cai LECH nhau (du chi 1 don vi) phai la KHONG hoi tu (false)");
```

:::hints
- kind: attention
  body: "So sanh JSON.stringify cua mang entries() DA SAP XEP (.sort()) cua ca hai HeThongSoCai -- mot dong."
- kind: strategy
  body: "return JSON.stringify([...a.cacTaiKhoan.entries()].sort()) === JSON.stringify([...b.cacTaiKhoan.entries()].sort());"
- kind: one-line
  body: "return JSON.stringify([...a.cacTaiKhoan.entries()].sort()) === JSON.stringify([...b.cacTaiKhoan.entries()].sort());"
:::

:::validate
- tier: run
  timeoutMs: 10000
- tier: tests
  timeoutMs: 10000
- tier: output
  match: contains
  expect: "true"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
R6-3 khép lại — VÀ CẢ Realm 6 (Database, 22 quest) đóng lại theo. Sổ
cái double-entry (q16), vũ trụ tất định (q17), kẻ phá hoại có chủ
đích (q18), đồng thuận VSR (q19) — ByteLedger (q21) ghép TRỌN cả bốn
thành MỘT hệ thống sống sót qua đĩa hỏng, mạng rớt, VÀ một primary
chết, hội tụ đúng, tái hiện tuyệt đối.
::::

::::reflect{#nghi-lai}
`chayByteLedgerDayDu` không giới thiệu khái niệm MỚI nào — nó LÀ
CHÍN bài trước (VÀ cả q16-q19) ráp LẠI đúng thứ tự: struct sổ cái đầy
đủ (bài 1), commit idempotent (bài 2), ghi đĩa riêng (bài 3), MỘT
chính sách lỗi CHO cả cụm (bài 4), phục hồi (bài 5, tinh THẦN tích
hợp trong State Transfer tự động), view change dưới TẢI (bài 6),
retry xuyên suốt (bài 7), mini-VOPR (bài 8), VÀ audit thành thật (bài
9) — về những GÌ mô hình này KHÔNG mô phỏng. Bài học lớn NHẤT của cả
q21: một hệ thống "đáng tin" không phải VÌ nó không BAO giờ gặp sự
cố — mà VÌ MỖI sự cố (đĩa hỏng, gói mất, primary chết, client gửi
lại) đều CÓ đúng một cơ CHẾ xử lý, VÀ những cơ chế đó GHÉP được với
NHAU mà không cái NÀO phá vỡ cái kia.
::::

::::checkpoint{mastery=0.95}
::::
