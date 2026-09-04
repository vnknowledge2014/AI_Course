---
id: co-so-du-lieu.dong-thuan-kieu-vsr.boss-dong-thuan-kieu-vsr
title: "BOSS — Đồng thuận kiểu VSR"
summary: "chayKichBanVSR(seed) mô phỏng ĐẦY ĐỦ: N=3, ba bút toán (nạp 1000, chuyển 400, chuyển 100) qua primary r0 (view 0, có tiêm lỗi mạng + retry), r0 'chết' giữa chừng, view change bầu r1 làm primary mới (view 1), rồi bút toán cuối qua r1. seed=3n: MỘT Prepare tới backup1 bị mất (quorum vẫn đạt qua backup2) kích hoạt state transfer khi commit lan xuống, MỘT vòng cần gửi lại (retry) -- kết quả cuối: hoiTu=true, canBang=true, soDuTK1=700, soDuTK2=300, lặp lại y hệt qua 5 lần chạy độc lập VÀ đúng với cả 30 seed đầu tiên (0..29), không seed nào làm vỡ bất biến hay khiến hai replica lệch nhau."
locale: vi
track: co-so-du-lieu
module: dong-thuan-kieu-vsr
order: 14
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: []
requires: [db.state-transfer-dong-bo-lac-hau]
concepts: [db.boss-q19]
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
Mười ba mảnh — quorum, log, Prepare/PrepareOk/Commit, retry, phát
hiện primary chết, view change, state transfer. Giờ ráp TẤT CẢ vào
MỘT kịch bản: ba replica thật, một primary chết GIỮA chừng, mạng
không hợp tác SUỐT quá trình — mọi replica còn sống có hội tụ về
ĐÚNG cùng một sổ cái không?
::::

::::explain{#rap-tat-ca-13-bai}
`chayKichBanVSR(seed)` script hoá MỘT kịch bản đầy đủ: `N=3` (`r0`,
`r1`, `r2`), ba bút toán qua primary `r0` (view `0`) — MỖI bút toán
đi qua `xuLyMotButToanQuaPrimary` (gộp lại nguyên xi Prepare/xuLyPrepare/
nhanPrepareOk/xuLyCommit, CÓ tiêm `matGoiTheoSeed` VÀ retry dựa
`kiemTraCanGuiLai`, VÀ tự động State Transfer nếu phát hiện một
backup lạc hậu Ở đúng lúc commit lan xuống). SAU hai bút toán đầu,
`r0` "chết" (script CỐ định thời điểm crash để dễ kiểm chứng — CHỈ
các quyết định MẠNG, mất gói VÀ độ trễ, mới do `seed` quyết định);
`r1` VÀ `r2` phát hiện (bài 9), View Change diễn ra (bài 10-12), `r1`
trở thành primary VIEW `1`; bút toán CUỐI đi qua `r1`:

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

interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; }
function taoTaiKhoan(id: number): TaiKhoan { return { id, debitsPosted: 0, creditsPosted: 0 }; }
function heThongCanBang(cacTaiKhoan: Map<number, TaiKhoan>): boolean {
  let no = 0, co = 0;
  for (const [, tk] of cacTaiKhoan) { no += tk.debitsPosted; co += tk.creditsPosted; }
  return no === co;
}
function apDungButToanThuong(cacTaiKhoan: Map<number, TaiKhoan>, bt: ButToan): void {
  cacTaiKhoan.get(bt.debitAccountId)!.debitsPosted += bt.amount;
  cacTaiKhoan.get(bt.creditAccountId)!.creditsPosted += bt.amount;
}

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
function nguongQuorum(tongSo: number): number { return Math.floor(tongSo / 2) + 1; }

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
function nhanPrepareOk(primary: Replica, cacTaiKhoan: Map<number, TaiKhoan>, phieuTheoOp: Map<number, Set<number>>, td: ThongDiep): boolean {
  const opNumber = td.opNumber!;
  let phieu = phieuTheoOp.get(opNumber);
  if (!phieu) { phieu = new Set<number>([primary.chiSo]); phieuTheoOp.set(opNumber, phieu); }
  phieu.add(td.tu);
  if (phieu.size < nguongQuorum(primary.tongSo)) return false;
  if (opNumber !== primary.commitNumber + 1) return false;
  const entry = primary.log.find((e) => e.opNumber === opNumber)!;
  apDungButToanThuong(cacTaiKhoan, entry.bt);
  primary.commitNumber = opNumber;
  return true;
}
function xuLyCommit(cacTaiKhoan: Map<number, TaiKhoan>, backup: Replica, td: ThongDiep): void {
  const commitMoi = td.commitNumber!;
  for (let op = backup.commitNumber + 1; op <= commitMoi; op++) {
    const entry = backup.log.find((e) => e.opNumber === op)!;
    apDungButToanThuong(cacTaiKhoan, entry.bt);
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
  replica.log = td.log!;
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
  backup.opNumber = opCuaLog(backup.log);
}

function xuLyMotButToanQuaPrimary(
  hd: HangDoiMang, tt: TrangThai, dh: DongHoAo,
  primary: Replica, cacBackupConSong: Replica[],
  soCaiTheoChiSo: Map<number, Map<number, TaiKhoan>>,
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
      if (nhanPrepareOk(primary, soCaiTheoChiSo.get(primary.chiSo)!, phieuTheoOp, sk.td)) daCommit = true;
    }
  }

  if (daCommit) {
    for (const backup of cacBackupConSong) {
      if (kiemTraCanDongBo(backup, primary.opNumber)) {
        const conThieu = primary.log.filter((e) => e.opNumber > backup.opNumber);
        xuLyPhanHoiDongBo(backup, { loai: 'PhanHoiDongBo', tu: primary.chiSo, den: backup.chiSo, viewNumber: primary.viewNumber, log: conThieu });
      }
      xuLyCommit(soCaiTheoChiSo.get(backup.chiSo)!, backup, { loai: 'Commit', tu: primary.chiSo, den: backup.chiSo, viewNumber: primary.viewNumber, commitNumber: primary.commitNumber });
    }
  }
  return daCommit;
}

function xacNhanHoiTu(soCaiA: Map<number, TaiKhoan>, soCaiB: Map<number, TaiKhoan>): boolean {
  return JSON.stringify([...soCaiA.entries()].sort()) === JSON.stringify([...soCaiB.entries()].sort());
}

function chayKichBanVSR(seed: bigint): { hoiTu: boolean; canBang: boolean; soDuTK1: number; soDuTK2: number } {
  const tt = gieoHat(seed);
  const dh = taoDongHoAo();
  const hd = taoHangDoiMang();
  const N = 3;
  const TY_LE_MAT = 30;

  const r0 = taoReplica(0, N);
  const r1 = taoReplica(1, N);
  const r2 = taoReplica(2, N);

  const NGUON_NGOAI = 0, TK1 = 1, TK2 = 2;
  function taoSoCai(): Map<number, TaiKhoan> {
    const m = new Map<number, TaiKhoan>();
    for (const id of [NGUON_NGOAI, TK1, TK2]) m.set(id, taoTaiKhoan(id));
    return m;
  }
  const soCaiTheoChiSo = new Map<number, Map<number, TaiKhoan>>([[0, taoSoCai()], [1, taoSoCai()], [2, taoSoCai()]]);

  // op1, op2: qua primary r0 (view 0) -- ca r1 va r2 con song
  xuLyMotButToanQuaPrimary(hd, tt, dh, r0, [r1, r2], soCaiTheoChiSo,
    { id: 1, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 1000 }, TY_LE_MAT);
  xuLyMotButToanQuaPrimary(hd, tt, dh, r0, [r1, r2], soCaiTheoChiSo,
    { id: 2, debitAccountId: TK1, creditAccountId: TK2, amount: 400 }, TY_LE_MAT);

  // r0 "chet" -- r1, r2 nghi ngo va bat dau view change (thoi diem crash CO DINH theo kich ban;
  // chi cac quyet dinh MANG -- mat goi/tre -- moi do seed quyet dinh)
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
      apDungButToanThuong(soCaiTheoChiSo.get(r.chiSo)!, e.bt);
    }
  }

  // op3: qua primary moi, backup con lai duy nhat
  const backupConLai = conSong.filter((r) => r.chiSo !== primaryMoi.chiSo);
  xuLyMotButToanQuaPrimary(hd, tt, dh, primaryMoi, backupConLai, soCaiTheoChiSo,
    { id: 3, debitAccountId: TK2, creditAccountId: TK1, amount: 100 }, TY_LE_MAT);

  const soCaiPrimaryMoi = soCaiTheoChiSo.get(primaryMoi.chiSo)!;
  const soCaiBackup = soCaiTheoChiSo.get(backupConLai[0]!.chiSo)!;
  return {
    hoiTu: xacNhanHoiTu(soCaiPrimaryMoi, soCaiBackup),
    canBang: heThongCanBang(soCaiPrimaryMoi) && heThongCanBang(soCaiBackup),
    soDuTK1: soCaiPrimaryMoi.get(TK1)!.creditsPosted - soCaiPrimaryMoi.get(TK1)!.debitsPosted,
    soDuTK2: soCaiPrimaryMoi.get(TK2)!.creditsPosted - soCaiPrimaryMoi.get(TK2)!.debitsPosted,
  };
}

console.log("seed=3n:", JSON.stringify(chayKichBanVSR(3n)));
const ketQua5Lan: string[] = [];
for (let i = 0; i < 5; i++) ketQua5Lan.push(JSON.stringify(chayKichBanVSR(3n)));
console.log("seed=3n, 5 lan doc lap:", ketQua5Lan.every((x) => x === ketQua5Lan[0]) ? "TAT CA GIONG HET" : "LECH NHAU");
```

```text title=readonly
seed=3n: {"hoiTu":true,"canBang":true,"soDuTK1":700,"soDuTK2":300}
seed=3n, 5 lan doc lap: TAT CA GIONG HET
```

`seed=3n` kể MỘT câu chuyện đầy đủ: Prepare op1 gửi TỚI `backup1`
(`r1`) BỊ mất, nhưng `backup2` (`r2`) nhận được — quorum (`primary +
r2 = 2`) vẫn ĐẠT, `r0` commit op1. NGAY khi commit lan xuống, `r0`
phát hiện `r1` lạc HẬU (`kiemTraCanDongBo`) VÀ tự động State Transfer
bù trước KHI gửi Commit — bài 13 vừa học, dùng NGAY trong dòng chảy
bình THƯỜNG, không cần đợi view change. op2 cần MỘT vòng gửi lại
(retry, bài 8) mới đạt quorum. `r0` "chết", `r1` VÀ `r2` phát hiện
(bài 9), View Change (bài 10-12) đưa `r1` lên LÀM primary view `1`.
op3 đi qua `r1` trót lọt ngay VÒNG đầu. Kết quả CUỐI: `r1` (primary
mới) VÀ `r2` (backup còn lại) hội tụ Về đúng CÙNG một sổ cái —
`TK1=700`, `TK2=300` — VÀ lặp LẠI y hệt qua `5` lần chạy độc lập.
::::

::::example{#don-gian-hoa-co-chu-dich}
Thời điểm `r0` "chết" Ở bài NÀY LÀ CỐ ĐỊNH theo kịch bản (ngay sau
op1, op2), không do `seed` quyết định — CHỈ các quyết định MẠNG (mất
gói, độ trễ) mới tất định THEO seed. Đây LÀ một đơn giản hoá CÓ chủ
đích so VỚI VOPR thật (q18 bài 11 giới thiệu tinh thần NÀY: TigerBeetle's
VOPR THẬT còn ngẫu-nhiên-hoá cả THỜI điểm crash) — giữ scope Ở mức
"đủ để chứng minh cơ chế đúng", không PHẢI một bộ kiểm thử phát hành
thật. Quét `30` seed đầu tiên (`0n` đến `29n`): KHÔNG seed nào làm
`hoiTu` hay `canBang` sai — quorum(`N=3`)`=2` đủ CHỊU lỗi mất gói
lẻ tẻ VÀ một primary chết, dù mạng cư XỬ khác nhau Ở mỗi seed.
::::

::::predict{#doan-doi-ty-le-mat-100 commitOnce}
Nếu ĐỔI `TY_LE_MAT` từ `30` thành `100` (MỌI thông điệp ĐỀU mất, cả
hai chiều) — `chayKichBanVSR(3n)` CÓ còn cho `hoiTu:true` không?
:::opt{correct}
KHÔNG chắc — VỚI `100%` mất gói, `xuLyMotButToanQuaPrimary` KHÔNG
bao giờ nhận được `PrepareOk` NÀO (mọi lượt gửi ĐỀU bị chặn ngay từ
`matGoiTheoSeed`), `daCommit` mãi mãi `false` sau `6` vòng retry — op1
KHÔNG BAO GIỜ commit, VÀ toàn bộ kịch bản dừng LẠI Ở trạng thái không
tiến triển
:::
:::opt
CÓ, vẫn `true` — quorum chỉ cần MỘT backup, dù mất gói bao nhiêu
phần trăm hệ thống VẪN tự tìm được đường
::why
Trực giác NÀY đúng KHI còn ÍT nhất một tia HY vọng thống kê (VÍ dụ
`30%`, Ở đó THỈNH thoảng một gói VẪN lọt qua) — nhưng SAI ở giới hạn
CỰC đoan `100%`, nơi XÁC suất "lọt qua" LÀ đúng `0` tuyệt đối.

Chỗ lệch: `matGoiTheoSeed(tt, 100)` LUÔN trả `true` (bài 8 q18: điều
kiện `soNguyenTrongKhoang(tt,0,100) < 100` LUÔN đúng VÌ miền giá trị
`[0,100)` không BAO giờ chạm `100`) — không CÓ retry NÀO, dù `6`
vòng hay `600` vòng, cứu ĐƯỢC một hệ thống nơi MỌI gói tin đều mất
tuyệt đối. Đây chính LÀ giới hạn CỦA quorum: nó chịu được THIỂU số
lỗi, không phải MỌI mức độ lỗi.
::
:::
::::

::::code{#viet_xac_nhan_hoi_tu}
Hoàn thiện `xacNhanHoiTu` — so sánh nội dung HAI sổ cái (không phụ
thuộc thứ tự chèn CỦA `Map`), trả `true` nếu GIỐNG hệt nhau.

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

interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; }
function taoTaiKhoan(id: number): TaiKhoan { return { id, debitsPosted: 0, creditsPosted: 0 }; }
function heThongCanBang(cacTaiKhoan: Map<number, TaiKhoan>): boolean {
  let no = 0, co = 0;
  for (const [, tk] of cacTaiKhoan) { no += tk.debitsPosted; co += tk.creditsPosted; }
  return no === co;
}
function apDungButToanThuong(cacTaiKhoan: Map<number, TaiKhoan>, bt: ButToan): void {
  cacTaiKhoan.get(bt.debitAccountId)!.debitsPosted += bt.amount;
  cacTaiKhoan.get(bt.creditAccountId)!.creditsPosted += bt.amount;
}

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
function nguongQuorum(tongSo: number): number { return Math.floor(tongSo / 2) + 1; }

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
function nhanPrepareOk(primary: Replica, cacTaiKhoan: Map<number, TaiKhoan>, phieuTheoOp: Map<number, Set<number>>, td: ThongDiep): boolean {
  const opNumber = td.opNumber!;
  let phieu = phieuTheoOp.get(opNumber);
  if (!phieu) { phieu = new Set<number>([primary.chiSo]); phieuTheoOp.set(opNumber, phieu); }
  phieu.add(td.tu);
  if (phieu.size < nguongQuorum(primary.tongSo)) return false;
  if (opNumber !== primary.commitNumber + 1) return false;
  const entry = primary.log.find((e) => e.opNumber === opNumber)!;
  apDungButToanThuong(cacTaiKhoan, entry.bt);
  primary.commitNumber = opNumber;
  return true;
}
function xuLyCommit(cacTaiKhoan: Map<number, TaiKhoan>, backup: Replica, td: ThongDiep): void {
  const commitMoi = td.commitNumber!;
  for (let op = backup.commitNumber + 1; op <= commitMoi; op++) {
    const entry = backup.log.find((e) => e.opNumber === op)!;
    apDungButToanThuong(cacTaiKhoan, entry.bt);
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
  replica.log = td.log!;
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
  backup.opNumber = opCuaLog(backup.log);
}

function xuLyMotButToanQuaPrimary(
  hd: HangDoiMang, tt: TrangThai, dh: DongHoAo,
  primary: Replica, cacBackupConSong: Replica[],
  soCaiTheoChiSo: Map<number, Map<number, TaiKhoan>>,
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
      if (nhanPrepareOk(primary, soCaiTheoChiSo.get(primary.chiSo)!, phieuTheoOp, sk.td)) daCommit = true;
    }
  }

  if (daCommit) {
    for (const backup of cacBackupConSong) {
      if (kiemTraCanDongBo(backup, primary.opNumber)) {
        const conThieu = primary.log.filter((e) => e.opNumber > backup.opNumber);
        xuLyPhanHoiDongBo(backup, { loai: 'PhanHoiDongBo', tu: primary.chiSo, den: backup.chiSo, viewNumber: primary.viewNumber, log: conThieu });
      }
      xuLyCommit(soCaiTheoChiSo.get(backup.chiSo)!, backup, { loai: 'Commit', tu: primary.chiSo, den: backup.chiSo, viewNumber: primary.viewNumber, commitNumber: primary.commitNumber });
    }
  }
  return daCommit;
}

function xacNhanHoiTu(soCaiA: Map<number, TaiKhoan>, soCaiB: Map<number, TaiKhoan>): boolean {
  ___
}

function chayKichBanVSR(seed: bigint): { hoiTu: boolean; canBang: boolean; soDuTK1: number; soDuTK2: number } {
  const tt = gieoHat(seed);
  const dh = taoDongHoAo();
  const hd = taoHangDoiMang();
  const N = 3;
  const TY_LE_MAT = 30;

  const r0 = taoReplica(0, N);
  const r1 = taoReplica(1, N);
  const r2 = taoReplica(2, N);

  const NGUON_NGOAI = 0, TK1 = 1, TK2 = 2;
  function taoSoCai(): Map<number, TaiKhoan> {
    const m = new Map<number, TaiKhoan>();
    for (const id of [NGUON_NGOAI, TK1, TK2]) m.set(id, taoTaiKhoan(id));
    return m;
  }
  const soCaiTheoChiSo = new Map<number, Map<number, TaiKhoan>>([[0, taoSoCai()], [1, taoSoCai()], [2, taoSoCai()]]);

  // op1, op2: qua primary r0 (view 0) -- ca r1 va r2 con song
  xuLyMotButToanQuaPrimary(hd, tt, dh, r0, [r1, r2], soCaiTheoChiSo,
    { id: 1, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 1000 }, TY_LE_MAT);
  xuLyMotButToanQuaPrimary(hd, tt, dh, r0, [r1, r2], soCaiTheoChiSo,
    { id: 2, debitAccountId: TK1, creditAccountId: TK2, amount: 400 }, TY_LE_MAT);

  // r0 "chet" -- r1, r2 nghi ngo va bat dau view change (thoi diem crash CO DINH theo kich ban;
  // chi cac quyet dinh MANG -- mat goi/tre -- moi do seed quyet dinh)
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
      apDungButToanThuong(soCaiTheoChiSo.get(r.chiSo)!, e.bt);
    }
  }

  // op3: qua primary moi, backup con lai duy nhat
  const backupConLai = conSong.filter((r) => r.chiSo !== primaryMoi.chiSo);
  xuLyMotButToanQuaPrimary(hd, tt, dh, primaryMoi, backupConLai, soCaiTheoChiSo,
    { id: 3, debitAccountId: TK2, creditAccountId: TK1, amount: 100 }, TY_LE_MAT);

  const soCaiPrimaryMoi = soCaiTheoChiSo.get(primaryMoi.chiSo)!;
  const soCaiBackup = soCaiTheoChiSo.get(backupConLai[0]!.chiSo)!;
  return {
    hoiTu: xacNhanHoiTu(soCaiPrimaryMoi, soCaiBackup),
    canBang: heThongCanBang(soCaiPrimaryMoi) && heThongCanBang(soCaiBackup),
    soDuTK1: soCaiPrimaryMoi.get(TK1)!.creditsPosted - soCaiPrimaryMoi.get(TK1)!.debitsPosted,
    soDuTK2: soCaiPrimaryMoi.get(TK2)!.creditsPosted - soCaiPrimaryMoi.get(TK2)!.debitsPosted,
  };
}

console.log(JSON.stringify(chayKichBanVSR(3n)));
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

interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; }
function taoTaiKhoan(id: number): TaiKhoan { return { id, debitsPosted: 0, creditsPosted: 0 }; }
function heThongCanBang(cacTaiKhoan: Map<number, TaiKhoan>): boolean {
  let no = 0, co = 0;
  for (const [, tk] of cacTaiKhoan) { no += tk.debitsPosted; co += tk.creditsPosted; }
  return no === co;
}
function apDungButToanThuong(cacTaiKhoan: Map<number, TaiKhoan>, bt: ButToan): void {
  cacTaiKhoan.get(bt.debitAccountId)!.debitsPosted += bt.amount;
  cacTaiKhoan.get(bt.creditAccountId)!.creditsPosted += bt.amount;
}

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
function nguongQuorum(tongSo: number): number { return Math.floor(tongSo / 2) + 1; }

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
function nhanPrepareOk(primary: Replica, cacTaiKhoan: Map<number, TaiKhoan>, phieuTheoOp: Map<number, Set<number>>, td: ThongDiep): boolean {
  const opNumber = td.opNumber!;
  let phieu = phieuTheoOp.get(opNumber);
  if (!phieu) { phieu = new Set<number>([primary.chiSo]); phieuTheoOp.set(opNumber, phieu); }
  phieu.add(td.tu);
  if (phieu.size < nguongQuorum(primary.tongSo)) return false;
  if (opNumber !== primary.commitNumber + 1) return false;
  const entry = primary.log.find((e) => e.opNumber === opNumber)!;
  apDungButToanThuong(cacTaiKhoan, entry.bt);
  primary.commitNumber = opNumber;
  return true;
}
function xuLyCommit(cacTaiKhoan: Map<number, TaiKhoan>, backup: Replica, td: ThongDiep): void {
  const commitMoi = td.commitNumber!;
  for (let op = backup.commitNumber + 1; op <= commitMoi; op++) {
    const entry = backup.log.find((e) => e.opNumber === op)!;
    apDungButToanThuong(cacTaiKhoan, entry.bt);
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
  replica.log = td.log!;
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
  backup.opNumber = opCuaLog(backup.log);
}

function xuLyMotButToanQuaPrimary(
  hd: HangDoiMang, tt: TrangThai, dh: DongHoAo,
  primary: Replica, cacBackupConSong: Replica[],
  soCaiTheoChiSo: Map<number, Map<number, TaiKhoan>>,
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
      if (nhanPrepareOk(primary, soCaiTheoChiSo.get(primary.chiSo)!, phieuTheoOp, sk.td)) daCommit = true;
    }
  }

  if (daCommit) {
    for (const backup of cacBackupConSong) {
      if (kiemTraCanDongBo(backup, primary.opNumber)) {
        const conThieu = primary.log.filter((e) => e.opNumber > backup.opNumber);
        xuLyPhanHoiDongBo(backup, { loai: 'PhanHoiDongBo', tu: primary.chiSo, den: backup.chiSo, viewNumber: primary.viewNumber, log: conThieu });
      }
      xuLyCommit(soCaiTheoChiSo.get(backup.chiSo)!, backup, { loai: 'Commit', tu: primary.chiSo, den: backup.chiSo, viewNumber: primary.viewNumber, commitNumber: primary.commitNumber });
    }
  }
  return daCommit;
}

function xacNhanHoiTu(soCaiA: Map<number, TaiKhoan>, soCaiB: Map<number, TaiKhoan>): boolean {
  return JSON.stringify([...soCaiA.entries()].sort()) === JSON.stringify([...soCaiB.entries()].sort());
}

function chayKichBanVSR(seed: bigint): { hoiTu: boolean; canBang: boolean; soDuTK1: number; soDuTK2: number } {
  const tt = gieoHat(seed);
  const dh = taoDongHoAo();
  const hd = taoHangDoiMang();
  const N = 3;
  const TY_LE_MAT = 30;

  const r0 = taoReplica(0, N);
  const r1 = taoReplica(1, N);
  const r2 = taoReplica(2, N);

  const NGUON_NGOAI = 0, TK1 = 1, TK2 = 2;
  function taoSoCai(): Map<number, TaiKhoan> {
    const m = new Map<number, TaiKhoan>();
    for (const id of [NGUON_NGOAI, TK1, TK2]) m.set(id, taoTaiKhoan(id));
    return m;
  }
  const soCaiTheoChiSo = new Map<number, Map<number, TaiKhoan>>([[0, taoSoCai()], [1, taoSoCai()], [2, taoSoCai()]]);

  // op1, op2: qua primary r0 (view 0) -- ca r1 va r2 con song
  xuLyMotButToanQuaPrimary(hd, tt, dh, r0, [r1, r2], soCaiTheoChiSo,
    { id: 1, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 1000 }, TY_LE_MAT);
  xuLyMotButToanQuaPrimary(hd, tt, dh, r0, [r1, r2], soCaiTheoChiSo,
    { id: 2, debitAccountId: TK1, creditAccountId: TK2, amount: 400 }, TY_LE_MAT);

  // r0 "chet" -- r1, r2 nghi ngo va bat dau view change (thoi diem crash CO DINH theo kich ban;
  // chi cac quyet dinh MANG -- mat goi/tre -- moi do seed quyet dinh)
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
      apDungButToanThuong(soCaiTheoChiSo.get(r.chiSo)!, e.bt);
    }
  }

  // op3: qua primary moi, backup con lai duy nhat
  const backupConLai = conSong.filter((r) => r.chiSo !== primaryMoi.chiSo);
  xuLyMotButToanQuaPrimary(hd, tt, dh, primaryMoi, backupConLai, soCaiTheoChiSo,
    { id: 3, debitAccountId: TK2, creditAccountId: TK1, amount: 100 }, TY_LE_MAT);

  const soCaiPrimaryMoi = soCaiTheoChiSo.get(primaryMoi.chiSo)!;
  const soCaiBackup = soCaiTheoChiSo.get(backupConLai[0]!.chiSo)!;
  return {
    hoiTu: xacNhanHoiTu(soCaiPrimaryMoi, soCaiBackup),
    canBang: heThongCanBang(soCaiPrimaryMoi) && heThongCanBang(soCaiBackup),
    soDuTK1: soCaiPrimaryMoi.get(TK1)!.creditsPosted - soCaiPrimaryMoi.get(TK1)!.debitsPosted,
    soDuTK2: soCaiPrimaryMoi.get(TK2)!.creditsPosted - soCaiPrimaryMoi.get(TK2)!.debitsPosted,
  };
}

console.log(JSON.stringify(chayKichBanVSR(3n)));
```

```typescript title=test
const kqChinh = chayKichBanVSR(3n);
if (JSON.stringify(kqChinh) !== '{"hoiTu":true,"canBang":true,"soDuTK1":700,"soDuTK2":300}') {
  throw new Error("seed=3n phai cho DUNG {hoiTu:true,canBang:true,soDuTK1:700,soDuTK2:300}");
}
const ketQua5LanT: string[] = [];
for (let i = 0; i < 5; i++) ketQua5LanT.push(JSON.stringify(chayKichBanVSR(3n)));
if (!ketQua5LanT.every((x) => x === ketQua5LanT[0])) throw new Error("goi lai CUNG seed=3n nhieu lan phai cho KET QUA giong het nhau (tai hien tuyet doi)");
for (let i = 0; i < 30; i++) {
  const kq = chayKichBanVSR(BigInt(i));
  if (!kq.hoiTu) throw new Error(`seed=${i}n: primary moi VA backup KHONG hoi tu ve cung so cai`);
  if (!kq.canBang) throw new Error(`seed=${i}n: he thong VI PHAM bat bien tong no = tong co`);
}
const mA = new Map<number, TaiKhoan>([[1, { id: 1, debitsPosted: 0, creditsPosted: 100 }], [2, { id: 2, debitsPosted: 50, creditsPosted: 0 }]]);
const mB = new Map<number, TaiKhoan>([[2, { id: 2, debitsPosted: 50, creditsPosted: 0 }], [1, { id: 1, debitsPosted: 0, creditsPosted: 100 }]]);
if (xacNhanHoiTu(mA, mB) !== true) throw new Error("hai so cai CUNG noi dung, khac thu tu chen, phai la HOI TU (true)");
const mC = new Map<number, TaiKhoan>([[1, { id: 1, debitsPosted: 0, creditsPosted: 100 }], [2, { id: 2, debitsPosted: 50, creditsPosted: 1 }]]);
if (xacNhanHoiTu(mA, mC) !== false) throw new Error("hai so cai LECH nhau phai la KHONG hoi tu (false)");
```

:::hints
- kind: attention
  body: "So sanh JSON.stringify cua mang entries() DA SAP XEP (.sort()) cua ca hai Map -- mot dong."
- kind: strategy
  body: "return JSON.stringify([...soCaiA.entries()].sort()) === JSON.stringify([...soCaiB.entries()].sort());"
- kind: one-line
  body: "return JSON.stringify([...soCaiA.entries()].sort()) === JSON.stringify([...soCaiB.entries()].sort());"
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: tests
  timeoutMs: 8000
- tier: output
  match: contains
  expect: "true"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
q19 khép lại: prepare/prepare_ok/commit, phát hiện lỗi, view change,
state transfer — tất cả sống sót qua mạng bất định VÀ một primary
chết, hội tụ đúng, tái hiện tuyệt đối. Đây chính LÀ điều làm
TigerBeetle (VÀ VSR) đáng tin: KHÔNG phải "không bao giờ lỗi", mà LÀ
"lỗi không làm nó SAI".
::::

::::reflect{#nghi-lai}
`chayKichBanVSR` không giới thiệu khái niệm MỚI nào — nó LÀ mười ba
hàm ĐÃ học (bài 1-13) ráp lại đúng THỨ tự, cộng thêm MỘT phép so
sánh (`xacNhanHoiTu`) để biến "hội tụ" từ một LỜI hứa thành một điều
KIỂM được bằng code — đúng tinh thần `timSeedLamVoBatBien` (BOSS
q18). Khác biệt LỚN nhất so VỚI q18: Ở đó ta TÌM một seed làm VỠ bất
biến; Ở đây, ta XÁC nhận KHÔNG seed nào làm vỡ được — quorum, retry,
view change, VÀ state transfer CÙNG nhau tạo thành đúng lớp phòng THỦ
mà một bản sao đơn LẺ (bài 1) không bao giờ có được.
::::

::::checkpoint{mastery=0.95}
::::
