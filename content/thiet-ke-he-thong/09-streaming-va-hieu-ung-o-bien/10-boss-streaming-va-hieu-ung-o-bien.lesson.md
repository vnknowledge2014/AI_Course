---
id: thiet-ke-he-thong.streaming-va-hieu-ung-o-bien.boss-streaming-va-hieu-ung-o-bien
title: "BOSS — luồng ingest nhỏ hoàn chỉnh: nhật ký + CRDT + streaming pipeline"
summary: "tinhGCounterTuNhatKy(nhatKy) fold nhat ky local (tai dung SuKien/NhatKy/ghiSuKien/Kho tu quest 'Nhat ky bat bien va fold') thanh MOT GCounter (tai dung GCounter/tangGCounter/hopNhatGCounter/hopNhatNhieuBanSao tu quest 'CRDT va hop nhat'); ba edge server tu ghi nhat ky rieng, fold thanh GCounter, hop nhat theo BA thu tu khac nhau deu ra 9; song song, bo dem TRUNG TAM gom RAW event qua quyetDinhGomLo (bai 2) VA ghiRaNgoai (bai 5) dung so lo -- hai duong tinh DOC LAP (merge CRDT vs batching+ship) hoi tu ve CUNG mot tong 9, bat ke thu tu."
locale: vi
track: thiet-ke-he-thong
module: streaming-va-hieu-ung-o-bien
order: 10
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [sd.fp.boss-streaming-va-hieu-ung-o-bien]
requires: [sd.fp.rap-effect-that-vao-pipeline]
concepts: [sd.fp.boss-streaming-va-hieu-ung-o-bien]
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
Ba mươi bài, ba quest — nhật ký bất biến VÀ fold, CRDT VÀ hợp nhất,
streaming VÀ hiệu ứng Ở biên. Mỗi quest đứng vững RIÊNG LẺ, kiểm chứng
kỹ Ở chính domain của nó. BOSS cuối cùng của track "System Design for
Functional" không dạy khái niệm MỚI nào — nó ráp ĐÚNG ba mảnh đó thành
một luồng ingest nhỏ, hoàn chỉnh: nhiều edge server nhận sự kiện
"lượt xem", ghi log, fold thành bộ đếm, hợp nhất, gom lô, gửi đi.
::::

::::explain{#rap_ba_quest_thanh_mot_luong_ingest}
`tinhGCounterTuNhatKy` LÀ mảnh ghép MỚI DUY nhất của bài này — nó fold
một `NhatKy` (tái dùng ĐÚNG `SuKien`, `NhatKy`, `ghiSuKien`, `Kho` từ
quest "Nhật ký bất biến và fold") thành một `GCounter` (tái dùng ĐÚNG
`GCounter`, `tangGCounter`, `hopNhatGCounter`, `hopNhatNhieuBanSao` từ
quest "CRDT và hợp nhất") — đúng vai trò của `tinhTrangThai` /
`taiTaoTuNhatKy` Ở quest trước, chỉ khác kiểu trả về LÀ `GCounter`
thay vì `TrangThai{soDu}`. Ba edge server tự ghi log riêng, fold riêng,
rồi hợp nhất:

```typescript title=readonly
// --- tu quest "Nhat ky bat bien va fold" (dung LAI dung ten) ---
interface SuKien { id: string; nguon: string; thoiDiem: number; soLuong: number; }
type NhatKy = SuKien[];
function ghiSuKien(nk: NhatKy, sk: SuKien): NhatKy { return [...nk, sk]; }

interface Kho { nhatKy: NhatKy; boDemId: number; }
function taoKho(): Kho { return { nhatKy: [], boDemId: 0 }; }

// vo: moi edge server ghi lai luot xem cua CHINH NO vao Kho rieng
function ghiNhanLuotXem(kho: Kho, nguon: string, thoiDiem: number, soLuong: number): Kho {
  kho.boDemId += 1;
  const idMoi = nguon + "-ev-" + kho.boDemId;
  kho.nhatKy = ghiSuKien(kho.nhatKy, { id: idMoi, nguon, thoiDiem, soLuong });
  return kho;
}

// --- tu quest "CRDT va hop nhat" (dung LAI dung ten) ---
interface GCounter { theoReplica: Record<string, number>; }
function counterRong(): GCounter { return { theoReplica: {} }; }
function tangGCounter(gc: GCounter, replicaId: string, buoc: number): GCounter {
  const hienTai = gc.theoReplica[replicaId] ?? 0;
  return { theoReplica: { ...gc.theoReplica, [replicaId]: hienTai + buoc } };
}
function giaTriGCounter(gc: GCounter): number {
  return Object.values(gc.theoReplica).reduce((tong, n) => tong + n, 0);
}
function hopNhatGCounter(a: GCounter, b: GCounter): GCounter {
  const ketQua: Record<string, number> = { ...a.theoReplica };
  for (const [replicaId, soDem] of Object.entries(b.theoReplica)) {
    const hienTai = ketQua[replicaId] ?? 0;
    ketQua[replicaId] = Math.max(hienTai, soDem);
  }
  return { theoReplica: ketQua };
}
function hopNhatNhieuBanSao(cacBoDem: GCounter[]): GCounter {
  return cacBoDem.reduce(hopNhatGCounter, counterRong());
}

// fold nhat ky local thanh MOT GCounter cho khung hien tai -- cung vai tro voi
// tinhTrangThai / taiTaoTuNhatKy cua quest truoc, chi khac kieu tra ve
function tinhGCounterTuNhatKy(nhatKy: NhatKy): GCounter {
  return nhatKy.reduce((gc, sk) => tangGCounter(gc, sk.nguon, sk.soLuong), counterRong());
}

// --- tu quest NAY (bai 2 va bai 5, dung LAI dung ten) ---
interface KetQuaGomLo { loMoi: SuKien[] | undefined; bufferMoi: SuKien[]; }
function quyetDinhGomLo(buffer: SuKien[], suKienMoi: SuKien, kichThuocLo: number, thoiGianChoMs: number): KetQuaGomLo {
  const bufferSau = [...buffer, suKienMoi];
  const dauBuffer = bufferSau[0];
  const daDuKichThuoc = bufferSau.length >= kichThuocLo;
  const daHetThoiGianCho = dauBuffer !== undefined && suKienMoi.thoiDiem - dauBuffer.thoiDiem >= thoiGianChoMs;
  if (daDuKichThuoc || daHetThoiGianCho) {
    return { loMoi: bufferSau, bufferMoi: [] };
  }
  return { loMoi: undefined, bufferMoi: bufferSau };
}

interface NhatKyGhiRaNgoai { cacLoDaGhi: SuKien[][]; }
function taoNhatKyGhiRaNgoai(): NhatKyGhiRaNgoai { return { cacLoDaGhi: [] }; }
function ghiRaNgoai(nhatKy: NhatKyGhiRaNgoai, lo: SuKien[]): void {
  nhatKy.cacLoDaGhi.push(lo);
}

// ===== (a) moi edge server ghi su kien vao NHAT KY bat bien rieng =====
let edge1 = taoKho();
edge1 = ghiNhanLuotXem(edge1, "edge-1", 0, 1);
edge1 = ghiNhanLuotXem(edge1, "edge-1", 100, 1);
edge1 = ghiNhanLuotXem(edge1, "edge-1", 200, 1);

let edge2 = taoKho();
edge2 = ghiNhanLuotXem(edge2, "edge-2", 50, 1);
edge2 = ghiNhanLuotXem(edge2, "edge-2", 150, 1);
edge2 = ghiNhanLuotXem(edge2, "edge-2", 250, 1);
edge2 = ghiNhanLuotXem(edge2, "edge-2", 350, 1);

let edge3 = taoKho();
edge3 = ghiNhanLuotXem(edge3, "edge-3", 80, 1);
edge3 = ghiNhanLuotXem(edge3, "edge-3", 180, 1);

console.log("edge-1 co", edge1.nhatKy.length, "su kien trong nhat ky rieng");
console.log("edge-2 co", edge2.nhatKy.length, "su kien trong nhat ky rieng");
console.log("edge-3 co", edge3.nhatKy.length, "su kien trong nhat ky rieng");

// ===== (b) fold nhat ky local cua TUNG server thanh GCounter =====
const gc1 = tinhGCounterTuNhatKy(edge1.nhatKy);
const gc2 = tinhGCounterTuNhatKy(edge2.nhatKy);
const gc3 = tinhGCounterTuNhatKy(edge3.nhatKy);
console.log("GCounter cua edge-1:", JSON.stringify(gc1.theoReplica));
console.log("GCounter cua edge-2:", JSON.stringify(gc2.theoReplica));
console.log("GCounter cua edge-3:", JSON.stringify(gc3.theoReplica));

// ===== (c) hop nhat GCounter giua cac server, theo BA thu tu khac nhau =====
const hopThuTu1 = hopNhatNhieuBanSao([gc1, gc2, gc3]);
const hopThuTu2 = hopNhatNhieuBanSao([gc3, gc1, gc2]);
const hopCoLap = hopNhatNhieuBanSao([gc1, gc2, gc3, gc1, gc2]); // mo phong hop nhat LAP LAI
console.log("hop nhat thu tu 1 [1,2,3]:", giaTriGCounter(hopThuTu1));
console.log("hop nhat thu tu 2 [3,1,2]:", giaTriGCounter(hopThuTu2));
console.log("hop nhat CO LAP [1,2,3,1,2]:", giaTriGCounter(hopCoLap));
console.log("ca ba deu GIONG NHAU?", giaTriGCounter(hopThuTu1) === giaTriGCounter(hopThuTu2) && giaTriGCounter(hopThuTu2) === giaTriGCounter(hopCoLap));

// ===== (d) mot bo dem TRUNG TAM nhan RAW event tu ca ba server, thu tu XEN KE =====
const luongTrungTam: SuKien[] = [
  edge1.nhatKy[0]!, edge2.nhatKy[0]!, edge3.nhatKy[0]!,
  edge1.nhatKy[1]!, edge2.nhatKy[1]!, edge3.nhatKy[1]!,
  edge1.nhatKy[2]!, edge2.nhatKy[2]!, edge2.nhatKy[3]!,
];
const { cacLoCanGui } = luongTrungTam.reduce(
  (tich, sk) => {
    const kq = quyetDinhGomLo(tich.buffer, sk, 3, 1000000);
    if (kq.loMoi !== undefined) return { buffer: kq.bufferMoi, cacLoCanGui: [...tich.cacLoCanGui, kq.loMoi] };
    return { buffer: kq.bufferMoi, cacLoCanGui: tich.cacLoCanGui };
  },
  { buffer: [] as SuKien[], cacLoCanGui: [] as SuKien[][] }
);
console.log("so lo trung tam da gom (kichThuocLo=3):", cacLoCanGui.length);

// ===== (e) ghiRaNgoai dung lo do =====
const nhatKyGhi = taoNhatKyGhiRaNgoai();
for (const lo of cacLoCanGui) ghiRaNgoai(nhatKyGhi, lo);
console.log("so lo THAT SU da ghi ra ngoai:", nhatKyGhi.cacLoDaGhi.length);

const tongDaGuiRa = nhatKyGhi.cacLoDaGhi.reduce((tong, lo) => tong + lo.reduce((t, sk) => t + sk.soLuong, 0), 0);
console.log("tong luot xem DA GUI RA NGOAI (qua duong batching):", tongDaGuiRa);
console.log("tong luot xem QUA hop nhat CRDT (qua duong merge):", giaTriGCounter(hopThuTu1));
console.log("hai duong TINH DOC LAP co KHOP nhau khong?", tongDaGuiRa === giaTriGCounter(hopThuTu1));
```

```text title=readonly
edge-1 co 3 su kien trong nhat ky rieng
edge-2 co 4 su kien trong nhat ky rieng
edge-3 co 2 su kien trong nhat ky rieng
GCounter cua edge-1: {"edge-1":3}
GCounter cua edge-2: {"edge-2":4}
GCounter cua edge-3: {"edge-3":2}
hop nhat thu tu 1 [1,2,3]: 9
hop nhat thu tu 2 [3,1,2]: 9
hop nhat CO LAP [1,2,3,1,2]: 9
ca ba deu GIONG NHAU? true
so lo trung tam da gom (kichThuocLo=3): 3
so lo THAT SU da ghi ra ngoai: 3
tong luot xem DA GUI RA NGOAI (qua duong batching): 9
tong luot xem QUA hop nhat CRDT (qua duong merge): 9
hai duong TINH DOC LAP co KHOP nhau khong? true
```

Hai đường tính TOÁN hoàn toàn độc lập — một đi qua fold-nhật-ký rồi
hợp nhất CRDT (`gc1`, `gc2`, `gc3` → `hopThuTu1`), một đi qua batching
RAW event Ở một bộ đệm trung tâm rồi gửi đi (`luongTrungTam` →
`cacLoCanGui` → `nhatKyGhi`) — đều hội tụ về ĐÚNG cùng con số `9`
(`3 + 4 + 2`, đúng bằng tổng số sự kiện Ở CẢ ba edge server). Không có
đường nào "biết" về đường kia; sự khớp nhau này chính LÀ bằng chứng
CẢ hai đường đều đúng.
::::

::::example{#hoi_tu_bat_ke_thu_tu_hay_so_lan_hop_nhat}
Đổi thứ tự sự kiện đến bộ đệm trung tâm HOÀN TOÀN (gộp nguyên khối
từng server thay vì xen kẽ), VÀ hợp nhất GCounter LẶP ĐI LẶP LẠI rất
nhiều lần — tổng cuối vẫn LUÔN LÀ `9`:

```typescript title=readonly
interface SuKien { id: string; nguon: string; thoiDiem: number; soLuong: number; }
type NhatKy = SuKien[];
function ghiSuKien(nk: NhatKy, sk: SuKien): NhatKy { return [...nk, sk]; }
interface Kho { nhatKy: NhatKy; boDemId: number; }
function taoKho(): Kho { return { nhatKy: [], boDemId: 0 }; }
function ghiNhanLuotXem(kho: Kho, nguon: string, thoiDiem: number, soLuong: number): Kho {
  kho.boDemId += 1;
  const idMoi = nguon + "-ev-" + kho.boDemId;
  kho.nhatKy = ghiSuKien(kho.nhatKy, { id: idMoi, nguon, thoiDiem, soLuong });
  return kho;
}
interface GCounter { theoReplica: Record<string, number>; }
function counterRong(): GCounter { return { theoReplica: {} }; }
function tangGCounter(gc: GCounter, replicaId: string, buoc: number): GCounter {
  const hienTai = gc.theoReplica[replicaId] ?? 0;
  return { theoReplica: { ...gc.theoReplica, [replicaId]: hienTai + buoc } };
}
function giaTriGCounter(gc: GCounter): number {
  return Object.values(gc.theoReplica).reduce((tong, n) => tong + n, 0);
}
function hopNhatGCounter(a: GCounter, b: GCounter): GCounter {
  const ketQua: Record<string, number> = { ...a.theoReplica };
  for (const [replicaId, soDem] of Object.entries(b.theoReplica)) {
    const hienTai = ketQua[replicaId] ?? 0;
    ketQua[replicaId] = Math.max(hienTai, soDem);
  }
  return { theoReplica: ketQua };
}
function hopNhatNhieuBanSao(cacBoDem: GCounter[]): GCounter {
  return cacBoDem.reduce(hopNhatGCounter, counterRong());
}
function tinhGCounterTuNhatKy(nhatKy: NhatKy): GCounter {
  return nhatKy.reduce((gc, sk) => tangGCounter(gc, sk.nguon, sk.soLuong), counterRong());
}
interface KetQuaGomLo { loMoi: SuKien[] | undefined; bufferMoi: SuKien[]; }
function quyetDinhGomLo(buffer: SuKien[], suKienMoi: SuKien, kichThuocLo: number, thoiGianChoMs: number): KetQuaGomLo {
  const bufferSau = [...buffer, suKienMoi];
  const dauBuffer = bufferSau[0];
  const daDuKichThuoc = bufferSau.length >= kichThuocLo;
  const daHetThoiGianCho = dauBuffer !== undefined && suKienMoi.thoiDiem - dauBuffer.thoiDiem >= thoiGianChoMs;
  if (daDuKichThuoc || daHetThoiGianCho) {
    return { loMoi: bufferSau, bufferMoi: [] };
  }
  return { loMoi: undefined, bufferMoi: bufferSau };
}
interface NhatKyGhiRaNgoai { cacLoDaGhi: SuKien[][]; }
function taoNhatKyGhiRaNgoai(): NhatKyGhiRaNgoai { return { cacLoDaGhi: [] }; }
function ghiRaNgoai(nhatKy: NhatKyGhiRaNgoai, lo: SuKien[]): void {
  nhatKy.cacLoDaGhi.push(lo);
}
function guiRaTuLuong(luong: SuKien[], kichThuocLo: number): number {
  const { cacLoCanGui } = luong.reduce(
    (tich, sk) => {
      const kq = quyetDinhGomLo(tich.buffer, sk, kichThuocLo, 1000000);
      if (kq.loMoi !== undefined) return { buffer: kq.bufferMoi, cacLoCanGui: [...tich.cacLoCanGui, kq.loMoi] };
      return { buffer: kq.bufferMoi, cacLoCanGui: tich.cacLoCanGui };
    },
    { buffer: [] as SuKien[], cacLoCanGui: [] as SuKien[][] }
  );
  const nhatKyGhi = taoNhatKyGhiRaNgoai();
  for (const lo of cacLoCanGui) ghiRaNgoai(nhatKyGhi, lo);
  return nhatKyGhi.cacLoDaGhi.reduce((tong, lo) => tong + lo.reduce((t, sk) => t + sk.soLuong, 0), 0);
}

let edge1 = taoKho();
edge1 = ghiNhanLuotXem(edge1, "edge-1", 0, 1);
edge1 = ghiNhanLuotXem(edge1, "edge-1", 100, 1);
edge1 = ghiNhanLuotXem(edge1, "edge-1", 200, 1);
let edge2 = taoKho();
edge2 = ghiNhanLuotXem(edge2, "edge-2", 50, 1);
edge2 = ghiNhanLuotXem(edge2, "edge-2", 150, 1);
edge2 = ghiNhanLuotXem(edge2, "edge-2", 250, 1);
edge2 = ghiNhanLuotXem(edge2, "edge-2", 350, 1);
let edge3 = taoKho();
edge3 = ghiNhanLuotXem(edge3, "edge-3", 80, 1);
edge3 = ghiNhanLuotXem(edge3, "edge-3", 180, 1);

// thu tu XEN KE nhu o explain
const thuTuXenKe: SuKien[] = [
  edge1.nhatKy[0]!, edge2.nhatKy[0]!, edge3.nhatKy[0]!,
  edge1.nhatKy[1]!, edge2.nhatKy[1]!, edge3.nhatKy[1]!,
  edge1.nhatKy[2]!, edge2.nhatKy[2]!, edge2.nhatKy[3]!,
];
// thu tu HOAN TOAN KHAC: gop tung server LIEN TIEP thay vi xen ke
const thuTuGopKhoi: SuKien[] = [...edge3.nhatKy, ...edge1.nhatKy, ...edge2.nhatKy];

console.log("tong da gui ra (thu tu xen ke):", guiRaTuLuong(thuTuXenKe, 3));
console.log("tong da gui ra (thu tu gop khoi):", guiRaTuLuong(thuTuGopKhoi, 3));

// hop nhat GCounter LAP LAI rat nhieu lan, xen giua cac server theo thu tu bat ky
const gc1 = tinhGCounterTuNhatKy(edge1.nhatKy);
const gc2 = tinhGCounterTuNhatKy(edge2.nhatKy);
const gc3 = tinhGCounterTuNhatKy(edge3.nhatKy);
const hopRatNhieuLan = hopNhatNhieuBanSao([gc2, gc2, gc1, gc3, gc1, gc2, gc3, gc3, gc1, gc2]);
console.log("hop nhat 10 lan (nhieu ban sao trung lap):", giaTriGCounter(hopRatNhieuLan));

console.log("moi duong hoi tu ve DUNG 9 -- bat ke thu tu server hop nhat hay so lan mot khung duoc hop nhat lai");
```

```text title=readonly
tong da gui ra (thu tu xen ke): 9
tong da gui ra (thu tu gop khoi): 9
hop nhat 10 lan (nhieu ban sao trung lap): 9
moi duong hoi tu ve DUNG 9 -- bat ke thu tu server hop nhat hay so lan mot khung duoc hop nhat lai
```

Dù bộ đệm trung tâm nhận sự kiện theo thứ tự XEN KẼ (mô phỏng mạng
thật, các server gửi tới gần như đồng thời) hay theo từng KHỐI liên
tiếp của một server, tổng số lượt xem đã gửi ra vẫn LÀ `9` — vì
`quyetDinhGomLo` chỉ đếm SỐ LƯỢNG sự kiện trong buffer, không quan tâm
chúng đến từ nguồn nào theo thứ tự nào. VÀ hợp nhất GCounter `10` lần
với nhiều bản sao TRÙNG lặp xen kẽ vẫn hội tụ về ĐÚNG `9` — nhờ
`Math.max` bên trong `hopNhatGCounter`, không phép hợp nhất LẶP lại
nào (dù bao nhiêu lần, theo thứ tự nào) làm tổng SAI lệch.
::::

::::predict{#doan-mat-mot-su-kien-o-duong-batching commitOnce}
Giả sử một sự kiện của `edge-3` (`edge3.nhatKy[1]`, `thoiDiem: 180`)
KHÔNG bao giờ tới được bộ đệm trung tâm (gói tin bị rớt trên đường
truyền) — nhưng nó VẪN còn nguyên trong nhật ký CỤC BỘ của `edge-3`
(dùng để fold `GCounter`). Lúc này, `tongDaGuiRa` (đường batching,
chỉ còn `8` sự kiện tới trung tâm) VÀ `giaTriGCounter(hopThuTu1)`
(đường CRDT merge, vẫn fold đủ `9` sự kiện từ log cục bộ) — còn KHỚP
nhau không?

:::opt{correct}
KHÔNG — `tongDaGuiRa` sẽ TỤT xuống `6` (chỉ còn `2` lô đủ `3` sự kiện
từ `8` sự kiện còn lại, `2` sự kiện lẻ Ở buffer chưa gửi), trong khi
`giaTriGCounter(hopThuTu1)` VẪN LÀ `9` — hai đường tính đã LỆCH nhau,
đúng dấu hiệu một sự kiện đã "biến mất" giữa nhật ký cục bộ VÀ luồng
tới trung tâm
:::
:::opt
Vẫn khớp — vì cả hai con số đều tính từ CÙNG một tập sự kiện gốc, nên
mất một sự kiện Ở MỘT đường sẽ tự động được đường CÒN LẠI "bù" lại để
giữ tổng bằng nhau
::why
Nhầm "hai đường tính đều dựa trên cùng dữ liệu GỐC" với "hai đường tự
động đồng bộ VỚI NHAU" — nhưng `tongDaGuiRa` VÀ `giaTriGCounter(hopThuTu1)`
là hai PHÉP TÍNH hoàn toàn tách biệt: một tính từ những gì THẬT SỰ tới
được bộ đệm trung tâm, một tính từ nhật ký CỤC BỘ Ở từng server —
không có cơ chế nào kết nối ngược lại giữa chúng.

Chỗ lệch: chính vì hai đường ĐỘC LẬP hoàn toàn (không đường nào gọi
hay tham chiếu đường kia) mà việc chúng KHỚP nhau Ở đoạn `explain`
mới có Ý nghĩa — đó LÀ một phép kiểm chứng chéo THẬT SỰ, có khả năng
phát hiện lỗi (một sự kiện bị mất trên đường truyền tới bộ đệm trung
tâm, dù vẫn còn Ở log cục bộ), không phải một đẳng thức luôn đúng theo
định nghĩa. Nếu dữ liệu tới hai nơi bị lệch nhau, hai tổng SẼ lệch
nhau theo — đó chính LÀ giá trị của việc có HAI con đường tính toán
độc lập cho CÙNG một sự thật.
::
:::
::::

::::code{#viet_tinh_gcounter_tu_nhat_ky}
Hoàn thiện `tinhGCounterTuNhatKy` — dùng `reduce` trên `nhatKy`, bắt
đầu từ `counterRong()`, gọi `tangGCounter(gc, sk.nguon, sk.soLuong)`
cho mỗi sự kiện.

```typescript title=starter
interface SuKien { id: string; nguon: string; thoiDiem: number; soLuong: number; }
type NhatKy = SuKien[];
function ghiSuKien(nk: NhatKy, sk: SuKien): NhatKy { return [...nk, sk]; }
interface Kho { nhatKy: NhatKy; boDemId: number; }
function taoKho(): Kho { return { nhatKy: [], boDemId: 0 }; }
function ghiNhanLuotXem(kho: Kho, nguon: string, thoiDiem: number, soLuong: number): Kho {
  kho.boDemId += 1;
  const idMoi = nguon + "-ev-" + kho.boDemId;
  kho.nhatKy = ghiSuKien(kho.nhatKy, { id: idMoi, nguon, thoiDiem, soLuong });
  return kho;
}

interface GCounter { theoReplica: Record<string, number>; }
function counterRong(): GCounter { return { theoReplica: {} }; }
function tangGCounter(gc: GCounter, replicaId: string, buoc: number): GCounter {
  const hienTai = gc.theoReplica[replicaId] ?? 0;
  return { theoReplica: { ...gc.theoReplica, [replicaId]: hienTai + buoc } };
}
function giaTriGCounter(gc: GCounter): number {
  return Object.values(gc.theoReplica).reduce((tong, n) => tong + n, 0);
}
function hopNhatGCounter(a: GCounter, b: GCounter): GCounter {
  const ketQua: Record<string, number> = { ...a.theoReplica };
  for (const [replicaId, soDem] of Object.entries(b.theoReplica)) {
    const hienTai = ketQua[replicaId] ?? 0;
    ketQua[replicaId] = Math.max(hienTai, soDem);
  }
  return { theoReplica: ketQua };
}
function hopNhatNhieuBanSao(cacBoDem: GCounter[]): GCounter {
  return cacBoDem.reduce(hopNhatGCounter, counterRong());
}

function tinhGCounterTuNhatKy(nhatKy: NhatKy): GCounter {
  ___
}

let khoX = taoKho();
khoX = ghiNhanLuotXem(khoX, "edge-x", 0, 1);
khoX = ghiNhanLuotXem(khoX, "edge-x", 100, 1);
const gcX = tinhGCounterTuNhatKy(khoX.nhatKy);
console.log(giaTriGCounter(gcX), JSON.stringify(gcX.theoReplica));
```

```typescript title=solution
interface SuKien { id: string; nguon: string; thoiDiem: number; soLuong: number; }
type NhatKy = SuKien[];
function ghiSuKien(nk: NhatKy, sk: SuKien): NhatKy { return [...nk, sk]; }
interface Kho { nhatKy: NhatKy; boDemId: number; }
function taoKho(): Kho { return { nhatKy: [], boDemId: 0 }; }
function ghiNhanLuotXem(kho: Kho, nguon: string, thoiDiem: number, soLuong: number): Kho {
  kho.boDemId += 1;
  const idMoi = nguon + "-ev-" + kho.boDemId;
  kho.nhatKy = ghiSuKien(kho.nhatKy, { id: idMoi, nguon, thoiDiem, soLuong });
  return kho;
}

interface GCounter { theoReplica: Record<string, number>; }
function counterRong(): GCounter { return { theoReplica: {} }; }
function tangGCounter(gc: GCounter, replicaId: string, buoc: number): GCounter {
  const hienTai = gc.theoReplica[replicaId] ?? 0;
  return { theoReplica: { ...gc.theoReplica, [replicaId]: hienTai + buoc } };
}
function giaTriGCounter(gc: GCounter): number {
  return Object.values(gc.theoReplica).reduce((tong, n) => tong + n, 0);
}
function hopNhatGCounter(a: GCounter, b: GCounter): GCounter {
  const ketQua: Record<string, number> = { ...a.theoReplica };
  for (const [replicaId, soDem] of Object.entries(b.theoReplica)) {
    const hienTai = ketQua[replicaId] ?? 0;
    ketQua[replicaId] = Math.max(hienTai, soDem);
  }
  return { theoReplica: ketQua };
}
function hopNhatNhieuBanSao(cacBoDem: GCounter[]): GCounter {
  return cacBoDem.reduce(hopNhatGCounter, counterRong());
}

function tinhGCounterTuNhatKy(nhatKy: NhatKy): GCounter {
  return nhatKy.reduce((gc, sk) => tangGCounter(gc, sk.nguon, sk.soLuong), counterRong());
}

let khoX = taoKho();
khoX = ghiNhanLuotXem(khoX, "edge-x", 0, 1);
khoX = ghiNhanLuotXem(khoX, "edge-x", 100, 1);
const gcX = tinhGCounterTuNhatKy(khoX.nhatKy);
console.log(giaTriGCounter(gcX), JSON.stringify(gcX.theoReplica));
```

```typescript title=test
const tRong = tinhGCounterTuNhatKy([]);
if (giaTriGCounter(tRong) !== 0) throw new Error("nhat ky rong phai fold ra GCounter gia tri 0");

let tKhoA = taoKho();
tKhoA = ghiNhanLuotXem(tKhoA, "server-a", 0, 5);
tKhoA = ghiNhanLuotXem(tKhoA, "server-a", 10, 3);
const tGcA = tinhGCounterTuNhatKy(tKhoA.nhatKy);
if (giaTriGCounter(tGcA) !== 8) throw new Error("hai su kien cung nguon (5+3) phai fold ra gia tri 8");
if (tGcA.theoReplica["server-a"] !== 8) throw new Error("o theoReplica cua dung nguon phai la 8");

let tKhoB = taoKho();
tKhoB = ghiNhanLuotXem(tKhoB, "server-b", 0, 2);
const tGcB = tinhGCounterTuNhatKy(tKhoB.nhatKy);

const tHopNhat = hopNhatNhieuBanSao([tGcA, tGcB]);
if (giaTriGCounter(tHopNhat) !== 10) throw new Error("hop nhat GCounter cua hai nguon KHAC nhau phai la 8 + 2 = 10");

const tHopNhatLap = hopNhatNhieuBanSao([tGcA, tGcB, tGcA, tGcA]);
if (giaTriGCounter(tHopNhatLap) !== 10) throw new Error("hop nhat LAP LAI GCounter cua CUNG mot nguon KHONG duoc lam tang gia tri (idempotent qua MAX)");

const tChuoiTruoc = JSON.stringify(tKhoA.nhatKy);
tinhGCounterTuNhatKy(tKhoA.nhatKy);
const tChuoiSau = JSON.stringify(tKhoA.nhatKy);
if (tChuoiTruoc !== tChuoiSau) throw new Error("tinhGCounterTuNhatKy KHONG duoc mutate nhatKy truyen vao");
```

:::hints
- kind: attention
  body: "Dung nhatKy.reduce voi gia tri khoi tao counterRong(). Ham gop: (gc, sk) => tangGCounter(gc, sk.nguon, sk.soLuong) -- moi su kien tang dung O cua replica la sk.nguon, theo dung sk.soLuong."
- kind: strategy
  body: "return nhatKy.reduce((gc, sk) => tangGCounter(gc, sk.nguon, sk.soLuong), counterRong());"
- kind: one-line
  body: "return nhatKy.reduce((gc, sk) => tangGCounter(gc, sk.nguon, sk.soLuong), counterRong());"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 10000
- tier: output
  match: contains
  expect: '2 {"edge-x":2}'
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba mươi bài, ba quest, một luồng ingest hoàn chỉnh: log cục bộ bất
biến, fold thành CRDT, hợp nhất bất kể thứ tự, gom lô, gửi ra ngoài —
tất cả hội tụ về đúng CÙNG một con số, đo bằng hai con đường độc lập.
Track "System Design for Functional" khép lại Ở đây.
::::

::::reflect{#nghi-lai}
Bài BOSS này không phát minh một khái niệm nào MỚI — mảnh duy nhất nó
thêm vào (`tinhGCounterTuNhatKy`) chỉ LÀ một phép fold, giống HỆT cấu
trúc `tinhTrangThai`/`taiTaoTuNhatKy` đã học, đặt CẠNH một `GCounter`
đã kiểm chứng đầy đủ. Giá trị thật của bài học không nằm Ở dòng code
mới, mà Ở việc BA quest — viết Ở BA thời điểm khác nhau, kiểm chứng
ĐỘC LẬP từng bài một — ráp lại được với NHAU mà không cần sửa một dòng
nào của bất kỳ quest nào trong số đó. Đây chính LÀ bài kiểm tra thật
sự cho "lõi thuần, vỏ mệnh lệnh" Ở QUY MÔ toàn hệ thống: khi từng mảnh
đã đúng VÀ độc lập, ráp chúng lại không tạo ra lỗi MỚI — nó chỉ tạo ra
một hệ thống LỚN hơn, vẫn đúng, vẫn kiểm chứng được, theo đúng cách
từng mảnh nhỏ đã từng được kiểm chứng.
::::

::::checkpoint{mastery=0.9}
::::
