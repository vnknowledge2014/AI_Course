---
id: ky-nghe-ung-dung-ai.ky-thuat-do-thi.boss-do-thi-tu-dau-vs-checkpoint-resume
title: "BOSS q9.5a — đồ thị rẽ nhánh: edge coverage VÀ tiết kiệm lệnh gọi nhờ checkpoint-resume"
summary: "taoDoThiBoss(giaTri, coTheThuLai) ráp NGUYÊN VĂN bốn cơ chế đã học: node+edge tường minh (bài 1: buoc1→buoc2→buoc3→buoc4→kiemTra, 4 cạnh chuẩn bị KHÔNG rẽ nhánh), rẽ nhánh có điều kiện (bài 2: kiemTra→xuLyHopLe|xuLyLoi, VÀ xuLyLoi→thuLai|boCuoc, 4 cạnh rẽ nhánh -- tổng 8 cạnh CÓ THỂ). Phần A -- edge coverage: một bộ 4 kịch bản input (giaTri 5/-3/-1/10, coTheThuLai=true CẢ 4) đạt tyLeHoanThanh=1 (4/4, 100% completion) NHƯNG tyLeEdgeCoverage chỉ 7/8 (in ra 0.875) -- cạnh 'xuLyLoi->boCuoc' chưa từng được đi qua. Phần B -- checkpoint/resume: trên MỘT kịch bản (giaTri=-3, coTheThuLai=true, đường đi thật cần 7 node/7 lệnh gọi), MỘT lần chạy TỪ ĐẦU tốn tongLuotGoiTuDau=7; giả lập một checkpoint được lưu SAU 4 bước chuẩn bị (như một 'lỗi giữa chừng' buộc phải resume thay vì chạy lại), MỘT lần chạy CÓ checkpoint-resume chỉ tốn tongLuotGoiTuCheckpoint=3 (kiemTra, xuLyLoi, thuLai) -- soLuotGoiTietKiem=4, VÀ dsNodeDaDi cuối cùng của cả hai cách GIỐNG HỆT nhau. Đóng q9.5a tại 6/6."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-do-thi
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [kna.boss-do-thi-tu-dau-vs-checkpoint-resume]
requires: [kna.do-thi-thuc-thi-la-catamorphism]
concepts: [kna.boss-do-thi-tu-dau-vs-checkpoint-resume]
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

::::byte{trigger=enter mood=happy pose=jump}
Năm bài: node + edge tường minh, rẽ nhánh có điều kiện, edge coverage,
checkpoint/resume, VÀ cầu nối FP — cấu trúc rẽ nhánh LÀ catamorphism.
BOSS ráp NGUYÊN VĂN BỐN cơ chế đầu (bài `1`-`4`) thành MỘT đồ thị DUY
NHẤT — có PHẦN chuẩn bị tuyến tính, có PHẦN rẽ nhánh HAI TẦNG — rồi đo
HAI thứ tách biệt: edge coverage của một bộ kịch bản, VÀ số lệnh gọi
node THẬT SỰ tiết kiệm được khi resume TỪ một checkpoint, thay vì chạy
LẠI từ đầu sau một "lỗi giữa chừng".
::::

::::explain{#mot_do_thi_rap_du_bon_co_che}
`taoDoThiBoss(giaTri, coTheThuLai)` nối HAI phần: BỐN node CHUẨN BỊ
tuyến tính KHÔNG rẽ nhánh (`buoc1→buoc2→buoc3→buoc4→kiemTra`, giống hệt
hình dạng bài `1`/`4`), rồi TỚI `"kiemTra"` — rẽ nhánh HAI TẦNG giống hệt
bài `2`/`3` (`kiemTra` rẽ `xuLyHopLe`/`xuLyLoi`; `xuLyLoi` rẽ TIẾP
`thuLai`/`boCuoc`). Tổng cộng `9` node, `8` cạnh CÓ THỂ — liệt kê tường
minh Ở `TOAN_BO_CANH_CO_THE`:

```typescript title=readonly
type KetQuaNode = { trangThai: "xong" | "loi"; tenNodeTiepTheo: string | null };

interface NodeXuLy {
  chay(): KetQuaNode;
}

type DoThi = Record<string, NodeXuLy>;

function taoNodeThanhCong(tenNodeTiepTheo: string | null): NodeXuLy {
  return {
    chay(): KetQuaNode {
      return { trangThai: "xong", tenNodeTiepTheo };
    },
  };
}

function taoDoThiBoss(giaTri: number, coTheThuLai: boolean): DoThi {
  return {
    buoc1: taoNodeThanhCong("buoc2"),
    buoc2: taoNodeThanhCong("buoc3"),
    buoc3: taoNodeThanhCong("buoc4"),
    buoc4: taoNodeThanhCong("kiemTra"),
    kiemTra: {
      chay(): KetQuaNode {
        if (giaTri >= 0) return { trangThai: "xong", tenNodeTiepTheo: "xuLyHopLe" };
        return { trangThai: "xong", tenNodeTiepTheo: "xuLyLoi" };
      },
    },
    xuLyHopLe: taoNodeThanhCong(null),
    xuLyLoi: {
      chay(): KetQuaNode {
        if (coTheThuLai) return { trangThai: "xong", tenNodeTiepTheo: "thuLai" };
        return { trangThai: "xong", tenNodeTiepTheo: "boCuoc" };
      },
    },
    thuLai: taoNodeThanhCong(null),
    boCuoc: taoNodeThanhCong(null),
  };
}

const TOAN_BO_CANH_CO_THE = [
  "buoc1->buoc2", "buoc2->buoc3", "buoc3->buoc4", "buoc4->kiemTra",
  "kiemTra->xuLyHopLe", "kiemTra->xuLyLoi", "xuLyLoi->thuLai", "xuLyLoi->boCuoc",
];
console.log(TOAN_BO_CANH_CO_THE.length);
```

```text title=readonly
8
```

BỐN cạnh ĐẦU (`buoc1→buoc2→buoc3→buoc4→kiemTra`) LUÔN được đi qua Ở MỌI
kịch bản (không rẽ nhánh) — chỉ BỐN cạnh SAU (từ `"kiemTra"` trở đi) mới
phụ thuộc `giaTri`/`coTheThuLai`.
::::

::::example{#phan_a_edge_coverage_va_phan_b_checkpoint}
**Phần A** — chạy một bộ `4` kịch bản qua `chayDoThi` (bài `1`), TẤT CẢ
đều `coTheThuLai: true`; đo `tyLeHoanThanh` (completion) VÀ
`tyLeEdgeCoverage` (bài `3`). **Phần B** — trên MỘT kịch bản ĐƠN
(`giaTri: -3, coTheThuLai: true`), so sánh chạy TỪ ĐẦU VỚI chạy CÓ
checkpoint-resume (bài `4`), giả lập checkpoint được lưu NGAY SAU `4`
bước chuẩn bị (như một lỗi giữa chừng buộc phải resume):

```typescript title=readonly
// (day du cac ham chayDoThiCoDemGoi, chayDoThiTuCheckpoint, taoDoThiBoss,
// tinhTapCanhDaDi, tinhTyLeEdgeCoverage -- xem khoi `solution` o buoc
// ::::code ben duoi de doc TOAN VAN)

type KichBan = { giaTri: number; coTheThuLai: boolean };

function chayBoKichBan(dsKichBan: KichBan[]): KetQuaChayDoThi[] {
  const ketQua: KetQuaChayDoThi[] = [];
  for (const kb of dsKichBan) {
    ketQua.push(chayDoThi(taoDoThiBoss(kb.giaTri, kb.coTheThuLai), "buoc1"));
  }
  return ketQua;
}

// Phan A: edge coverage cua mot bo kich ban.
const boKichBan: KichBan[] = [
  { giaTri: 5, coTheThuLai: true },
  { giaTri: -3, coTheThuLai: true },
  { giaTri: -1, coTheThuLai: true },
  { giaTri: 10, coTheThuLai: true },
];
const ketQuaBoKichBan = chayBoKichBan(boKichBan);
const tapCanh = tinhTapCanhDaDi(ketQuaBoKichBan);
console.log("hoan thanh:", tinhTyLeHoanThanh(ketQuaBoKichBan));
console.log("edge coverage:", tinhTyLeEdgeCoverage(tapCanh, TOAN_BO_CANH_CO_THE));

// Phan B: tu dau vs checkpoint-resume tren MOT kich ban.
const doThiKichBanDon = taoDoThiBoss(-3, true);

const demGoiTuDau: Record<string, number> = {};
const ketQuaTuDau = chayDoThiCoDemGoi(doThiKichBanDon, "buoc1", demGoiTuDau);

const checkpointGiuaChung: Checkpoint = {
  tenNodeTiepTheo: "kiemTra",
  dsNodeDaDiTruocDo: ["buoc1", "buoc2", "buoc3", "buoc4"],
};
const demGoiTuCheckpoint: Record<string, number> = {};
const ketQuaTuCheckpoint = chayDoThiTuCheckpoint(doThiKichBanDon, checkpointGiuaChung, demGoiTuCheckpoint);

console.log("tu dau:      ", JSON.stringify(ketQuaTuDau.dsNodeDaDi), tinhTongSoLuotGoi(demGoiTuDau));
console.log("tu checkpoint:", JSON.stringify(ketQuaTuCheckpoint.dsNodeDaDi), tinhTongSoLuotGoi(demGoiTuCheckpoint));
```

```text title=readonly
hoan thanh: 1
edge coverage: 0.875
tu dau:       ["buoc1","buoc2","buoc3","buoc4","kiemTra","xuLyLoi","thuLai"] 7
tu checkpoint: ["buoc1","buoc2","buoc3","buoc4","kiemTra","xuLyLoi","thuLai"] 3
```

Completion `100%` (`4`/`4`), edge coverage CHỈ `0.875` (`7`/`8` — cạnh
`"xuLyLoi->boCuoc"` chưa từng chạm tới, ĐÚNG như bài `3`, giờ trên MỘT
đồ thị LỚN HƠN). Chạy TỪ ĐẦU tốn `7` lệnh gọi; chạy CÓ checkpoint-resume
CHO RA CÙNG một `dsNodeDaDi` cuối cùng NHƯNG chỉ tốn `3` lệnh gọi MỚI —
tiết kiệm ĐÚNG `4` lệnh gọi (`buoc1`-`buoc4` không hề bị gọi lại).
::::

::::predict{#doan-hai-truc-do-doc-lap commitOnce}
Nếu THÊM MỘT kịch bản VÀO bộ `4` Ở Phần A (`giaTri: -1, coTheThuLai:
false` — đi qua ĐÚNG cạnh `"xuLyLoi->boCuoc"` còn thiếu) — `tyLeEdgeCoverage`
tăng lên bao nhiêu, VÀ số liệu Ở Phần B (checkpoint-resume trên kịch bản
`giaTri=-3`) có đổi theo KHÔNG?

:::opt{correct}
`tyLeEdgeCoverage` tăng lên ĐÚNG `1` (100%, `8`/`8`) — NHƯNG số liệu Ở
Phần B (`7` lệnh gọi tu đầu, `3` lệnh gọi từ checkpoint, tiết kiệm `4`)
KHÔNG đổi GÌ CẢ: hai phần đo HAI thứ HOÀN TOÀN độc lập — Phần A đo TẬP
HỢP kịch bản, Phần B đo MỘT kịch bản riêng lẻ chạy theo HAI cách khác
nhau
:::
:::opt
Cả `tyLeEdgeCoverage` LẪN số liệu Phần B đều tăng, vì thêm một kịch bản
làm đồ thị "chạy nhiều hơn" Ở MỌI phép đo
::why
Nhầm rằng MỌI phép đo trong BOSS đều CHIA SẺ cùng một nguồn dữ liệu —
nhưng Phần A (`boKichBan`, `4` rồi `5` kịch bản) VÀ Phần B
(`doThiKichBanDon`, LUÔN CHỈ MỘT kịch bản `giaTri=-3`) LÀ HAI biến hoàn
toàn tách biệt, không hề đọc HAY ghi vào nhau.

Chỗ lệch: `ketQuaBoKichBan`/`tapCanh` (Phần A) VÀ
`ketQuaTuDau`/`ketQuaTuCheckpoint` (Phần B) được tính từ HAI lời gọi
hàm ĐỘC LẬP, trên HAI cấu trúc dữ liệu KHÁC nhau — thêm một phần tử vào
`boKichBan` không hề chạm tới `doThiKichBanDon`.
::
:::
:::opt
`tyLeEdgeCoverage` không đổi (vẫn `0.875`) vì `TOAN_BO_CANH_CO_THE` đã
CỐ ĐỊNH từ đầu, không thể tăng thêm
::why
Nhầm rằng `TOAN_BO_CANH_CO_THE` (mẫu số, CỐ ĐỊNH) LÀ thứ quyết định kết
quả — nhưng `tyLeEdgeCoverage` phụ thuộc và TỬ SỐ: số cạnh trong
`TOAN_BO_CANH_CO_THE` MÀ tập `canhDaDi` (từ kịch bản THẬT SỰ chạy) CÓ
chứa. Thêm kịch bản làm `canhDaDi` LỚN hơn, dù `TOAN_BO_CANH_CO_THE`
không đổi.

Chỗ lệch: `tinhTyLeEdgeCoverage` đếm `toanBoCanh.filter((c) =>
canhDaDi.has(c))` — mẫu số (`toanBoCanh.length`) đúng LÀ cố định, nhưng
tử số (số phần tử CÓ mặt trong `canhDaDi`) tăng khi `canhDaDi` được BỔ
SUNG cạnh mới.
::
:::
::::

::::code{#viet_boss_do_thi}
Hoàn thiện `chayDoThiCoDemGoi` — ĐÚNG hình dạng bài `4`: lặp `while` từ
`tenNodeGoc`, MỖI lần tăng `demGoiMoiNode[tenHienTai]`, GHI vào
`dsNodeDaDi`, xử lý `"loi"`/`tenNodeTiepTheo` NHƯ bài `1`. Hoàn thiện
`chayDoThiTuCheckpoint` — gọi `chayDoThiCoDemGoi` bắt đầu Ở
`checkpoint.tenNodeTiepTheo`, GHÉP `checkpoint.dsNodeDaDiTruocDo` VÀO
TRƯỚC `dsNodeDaDi` mới tính được (ĐÚNG bài `4`).

```typescript title=starter
type KetQuaNode = { trangThai: "xong" | "loi"; tenNodeTiepTheo: string | null };

interface NodeXuLy {
  chay(): KetQuaNode;
}

type DoThi = Record<string, NodeXuLy>;

function taoNodeThanhCong(tenNodeTiepTheo: string | null): NodeXuLy {
  return {
    chay(): KetQuaNode {
      return { trangThai: "xong", tenNodeTiepTheo };
    },
  };
}

type KetQuaChayDoThi =
  | { thanhCong: true; dsNodeDaDi: string[] }
  | { thanhCong: false; loi: string; dsNodeDaDi: string[] };

const SO_BUOC_TOI_DA_AN_TOAN = 20;

function chayDoThiCoDemGoi(
  doThi: DoThi,
  tenNodeGoc: string,
  demGoiMoiNode: Record<string, number>,
): KetQuaChayDoThi {
  ___
}

function chayDoThi(doThi: DoThi, tenNodeGoc: string): KetQuaChayDoThi {
  return chayDoThiCoDemGoi(doThi, tenNodeGoc, {});
}

type Checkpoint = { tenNodeTiepTheo: string; dsNodeDaDiTruocDo: string[] };

function chayDoThiTuCheckpoint(
  doThi: DoThi,
  checkpoint: Checkpoint,
  demGoiMoiNode: Record<string, number>,
): KetQuaChayDoThi {
  ___
}

function tinhTongSoLuotGoi(demGoiMoiNode: Record<string, number>): number {
  return Object.values(demGoiMoiNode).reduce((acc, n) => acc + n, 0);
}

function taoDoThiBoss(giaTri: number, coTheThuLai: boolean): DoThi {
  return {
    buoc1: taoNodeThanhCong("buoc2"),
    buoc2: taoNodeThanhCong("buoc3"),
    buoc3: taoNodeThanhCong("buoc4"),
    buoc4: taoNodeThanhCong("kiemTra"),
    kiemTra: {
      chay(): KetQuaNode {
        if (giaTri >= 0) return { trangThai: "xong", tenNodeTiepTheo: "xuLyHopLe" };
        return { trangThai: "xong", tenNodeTiepTheo: "xuLyLoi" };
      },
    },
    xuLyHopLe: taoNodeThanhCong(null),
    xuLyLoi: {
      chay(): KetQuaNode {
        if (coTheThuLai) return { trangThai: "xong", tenNodeTiepTheo: "thuLai" };
        return { trangThai: "xong", tenNodeTiepTheo: "boCuoc" };
      },
    },
    thuLai: taoNodeThanhCong(null),
    boCuoc: taoNodeThanhCong(null),
  };
}

const TOAN_BO_CANH_CO_THE = [
  "buoc1->buoc2", "buoc2->buoc3", "buoc3->buoc4", "buoc4->kiemTra",
  "kiemTra->xuLyHopLe", "kiemTra->xuLyLoi", "xuLyLoi->thuLai", "xuLyLoi->boCuoc",
];

function layCacCanhDaDi(dsNodeDaDi: string[]): string[] {
  const canh: string[] = [];
  for (let i = 0; i < dsNodeDaDi.length - 1; i++) {
    canh.push(`${dsNodeDaDi[i]}->${dsNodeDaDi[i + 1]}`);
  }
  return canh;
}

function tinhTapCanhDaDi(dsKetQua: KetQuaChayDoThi[]): Set<string> {
  const tap = new Set<string>();
  for (const kq of dsKetQua) {
    for (const canh of layCacCanhDaDi(kq.dsNodeDaDi)) tap.add(canh);
  }
  return tap;
}

function tinhTyLeEdgeCoverage(canhDaDi: Set<string>, toanBoCanh: string[]): number {
  if (toanBoCanh.length === 0) return 0;
  const soCanhDaDi = toanBoCanh.filter((c) => canhDaDi.has(c)).length;
  return soCanhDaDi / toanBoCanh.length;
}

function tinhTyLeHoanThanh(dsKetQua: KetQuaChayDoThi[]): number {
  if (dsKetQua.length === 0) return 0;
  return dsKetQua.filter((k) => k.thanhCong).length / dsKetQua.length;
}

type KichBan = { giaTri: number; coTheThuLai: boolean };

function chayBoKichBan(dsKichBan: KichBan[]): KetQuaChayDoThi[] {
  const ketQua: KetQuaChayDoThi[] = [];
  for (const kb of dsKichBan) {
    ketQua.push(chayDoThi(taoDoThiBoss(kb.giaTri, kb.coTheThuLai), "buoc1"));
  }
  return ketQua;
}

const boKichBan: KichBan[] = [
  { giaTri: 5, coTheThuLai: true },
  { giaTri: -3, coTheThuLai: true },
  { giaTri: -1, coTheThuLai: true },
  { giaTri: 10, coTheThuLai: true },
];
const ketQuaBoKichBan = chayBoKichBan(boKichBan);
const tapCanh = tinhTapCanhDaDi(ketQuaBoKichBan);
const tyLeHoanThanh = tinhTyLeHoanThanh(ketQuaBoKichBan);
const tyLeEdgeCoverage = tinhTyLeEdgeCoverage(tapCanh, TOAN_BO_CANH_CO_THE);

const doThiKichBanDon = taoDoThiBoss(-3, true);

const demGoiTuDau: Record<string, number> = {};
const ketQuaTuDau = chayDoThiCoDemGoi(doThiKichBanDon, "buoc1", demGoiTuDau);
const tongLuotGoiTuDau = tinhTongSoLuotGoi(demGoiTuDau);

const checkpointGiuaChung: Checkpoint = {
  tenNodeTiepTheo: "kiemTra",
  dsNodeDaDiTruocDo: ["buoc1", "buoc2", "buoc3", "buoc4"],
};
const demGoiTuCheckpoint: Record<string, number> = {};
const ketQuaTuCheckpoint = chayDoThiTuCheckpoint(doThiKichBanDon, checkpointGiuaChung, demGoiTuCheckpoint);
const tongLuotGoiTuCheckpoint = tinhTongSoLuotGoi(demGoiTuCheckpoint);

const soLuotGoiTietKiem = tongLuotGoiTuDau - tongLuotGoiTuCheckpoint;

console.log(
  tyLeHoanThanh,
  tyLeEdgeCoverage,
  tongLuotGoiTuDau,
  tongLuotGoiTuCheckpoint,
  soLuotGoiTietKiem,
);
```

```typescript title=solution
type KetQuaNode = { trangThai: "xong" | "loi"; tenNodeTiepTheo: string | null };

interface NodeXuLy {
  chay(): KetQuaNode;
}

type DoThi = Record<string, NodeXuLy>;

function taoNodeThanhCong(tenNodeTiepTheo: string | null): NodeXuLy {
  return {
    chay(): KetQuaNode {
      return { trangThai: "xong", tenNodeTiepTheo };
    },
  };
}

type KetQuaChayDoThi =
  | { thanhCong: true; dsNodeDaDi: string[] }
  | { thanhCong: false; loi: string; dsNodeDaDi: string[] };

const SO_BUOC_TOI_DA_AN_TOAN = 20;

function chayDoThiCoDemGoi(
  doThi: DoThi,
  tenNodeGoc: string,
  demGoiMoiNode: Record<string, number>,
): KetQuaChayDoThi {
  const dsNodeDaDi: string[] = [];
  let tenHienTai: string | null = tenNodeGoc;
  let soBuoc = 0;
  while (tenHienTai !== null && soBuoc < SO_BUOC_TOI_DA_AN_TOAN) {
    soBuoc++;
    const node: NodeXuLy | undefined = doThi[tenHienTai];
    if (!node) return { thanhCong: false, loi: "node_khong_ton_tai", dsNodeDaDi };
    demGoiMoiNode[tenHienTai] = (demGoiMoiNode[tenHienTai] ?? 0) + 1;
    dsNodeDaDi.push(tenHienTai);
    const kq: KetQuaNode = node.chay();
    if (kq.trangThai === "loi") return { thanhCong: false, loi: "loi_tai_node", dsNodeDaDi };
    tenHienTai = kq.tenNodeTiepTheo;
  }
  return { thanhCong: true, dsNodeDaDi };
}

function chayDoThi(doThi: DoThi, tenNodeGoc: string): KetQuaChayDoThi {
  return chayDoThiCoDemGoi(doThi, tenNodeGoc, {});
}

type Checkpoint = { tenNodeTiepTheo: string; dsNodeDaDiTruocDo: string[] };

function chayDoThiTuCheckpoint(
  doThi: DoThi,
  checkpoint: Checkpoint,
  demGoiMoiNode: Record<string, number>,
): KetQuaChayDoThi {
  const ketQuaMoi = chayDoThiCoDemGoi(doThi, checkpoint.tenNodeTiepTheo, demGoiMoiNode);
  return { ...ketQuaMoi, dsNodeDaDi: [...checkpoint.dsNodeDaDiTruocDo, ...ketQuaMoi.dsNodeDaDi] };
}

function tinhTongSoLuotGoi(demGoiMoiNode: Record<string, number>): number {
  return Object.values(demGoiMoiNode).reduce((acc, n) => acc + n, 0);
}

function taoDoThiBoss(giaTri: number, coTheThuLai: boolean): DoThi {
  return {
    buoc1: taoNodeThanhCong("buoc2"),
    buoc2: taoNodeThanhCong("buoc3"),
    buoc3: taoNodeThanhCong("buoc4"),
    buoc4: taoNodeThanhCong("kiemTra"),
    kiemTra: {
      chay(): KetQuaNode {
        if (giaTri >= 0) return { trangThai: "xong", tenNodeTiepTheo: "xuLyHopLe" };
        return { trangThai: "xong", tenNodeTiepTheo: "xuLyLoi" };
      },
    },
    xuLyHopLe: taoNodeThanhCong(null),
    xuLyLoi: {
      chay(): KetQuaNode {
        if (coTheThuLai) return { trangThai: "xong", tenNodeTiepTheo: "thuLai" };
        return { trangThai: "xong", tenNodeTiepTheo: "boCuoc" };
      },
    },
    thuLai: taoNodeThanhCong(null),
    boCuoc: taoNodeThanhCong(null),
  };
}

const TOAN_BO_CANH_CO_THE = [
  "buoc1->buoc2", "buoc2->buoc3", "buoc3->buoc4", "buoc4->kiemTra",
  "kiemTra->xuLyHopLe", "kiemTra->xuLyLoi", "xuLyLoi->thuLai", "xuLyLoi->boCuoc",
];

function layCacCanhDaDi(dsNodeDaDi: string[]): string[] {
  const canh: string[] = [];
  for (let i = 0; i < dsNodeDaDi.length - 1; i++) {
    canh.push(`${dsNodeDaDi[i]}->${dsNodeDaDi[i + 1]}`);
  }
  return canh;
}

function tinhTapCanhDaDi(dsKetQua: KetQuaChayDoThi[]): Set<string> {
  const tap = new Set<string>();
  for (const kq of dsKetQua) {
    for (const canh of layCacCanhDaDi(kq.dsNodeDaDi)) tap.add(canh);
  }
  return tap;
}

function tinhTyLeEdgeCoverage(canhDaDi: Set<string>, toanBoCanh: string[]): number {
  if (toanBoCanh.length === 0) return 0;
  const soCanhDaDi = toanBoCanh.filter((c) => canhDaDi.has(c)).length;
  return soCanhDaDi / toanBoCanh.length;
}

function tinhTyLeHoanThanh(dsKetQua: KetQuaChayDoThi[]): number {
  if (dsKetQua.length === 0) return 0;
  return dsKetQua.filter((k) => k.thanhCong).length / dsKetQua.length;
}

type KichBan = { giaTri: number; coTheThuLai: boolean };

function chayBoKichBan(dsKichBan: KichBan[]): KetQuaChayDoThi[] {
  const ketQua: KetQuaChayDoThi[] = [];
  for (const kb of dsKichBan) {
    ketQua.push(chayDoThi(taoDoThiBoss(kb.giaTri, kb.coTheThuLai), "buoc1"));
  }
  return ketQua;
}

const boKichBan: KichBan[] = [
  { giaTri: 5, coTheThuLai: true },
  { giaTri: -3, coTheThuLai: true },
  { giaTri: -1, coTheThuLai: true },
  { giaTri: 10, coTheThuLai: true },
];
const ketQuaBoKichBan = chayBoKichBan(boKichBan);
const tapCanh = tinhTapCanhDaDi(ketQuaBoKichBan);
const tyLeHoanThanh = tinhTyLeHoanThanh(ketQuaBoKichBan);
const tyLeEdgeCoverage = tinhTyLeEdgeCoverage(tapCanh, TOAN_BO_CANH_CO_THE);

const doThiKichBanDon = taoDoThiBoss(-3, true);

const demGoiTuDau: Record<string, number> = {};
const ketQuaTuDau = chayDoThiCoDemGoi(doThiKichBanDon, "buoc1", demGoiTuDau);
const tongLuotGoiTuDau = tinhTongSoLuotGoi(demGoiTuDau);

const checkpointGiuaChung: Checkpoint = {
  tenNodeTiepTheo: "kiemTra",
  dsNodeDaDiTruocDo: ["buoc1", "buoc2", "buoc3", "buoc4"],
};
const demGoiTuCheckpoint: Record<string, number> = {};
const ketQuaTuCheckpoint = chayDoThiTuCheckpoint(doThiKichBanDon, checkpointGiuaChung, demGoiTuCheckpoint);
const tongLuotGoiTuCheckpoint = tinhTongSoLuotGoi(demGoiTuCheckpoint);

const soLuotGoiTietKiem = tongLuotGoiTuDau - tongLuotGoiTuCheckpoint;

console.log(
  tyLeHoanThanh,
  tyLeEdgeCoverage,
  tongLuotGoiTuDau,
  tongLuotGoiTuCheckpoint,
  soLuotGoiTietKiem,
);
```

```typescript title=test
if (Math.abs(tyLeHoanThanh - 1) > 1e-9) throw new Error("ty le hoan thanh tren bo kich ban phai la 1 (100%)");
if (Math.abs(tyLeEdgeCoverage - 0.875) > 1e-9) throw new Error("edge coverage phai la 0.875 (7/8) -- canh xuLyLoi->boCuoc chua tung duoc di qua");

if (JSON.stringify(ketQuaTuDau.dsNodeDaDi) !== JSON.stringify(["buoc1", "buoc2", "buoc3", "buoc4", "kiemTra", "xuLyLoi", "thuLai"])) {
  throw new Error("chay tu dau tren kich ban don (giaTri=-3, coTheThuLai=true) phai di qua dung 7 node theo dung thu tu");
}
if (tongLuotGoiTuDau !== 7) throw new Error("chay tu dau phai ton DUNG 7 luot goi node");

if (JSON.stringify(ketQuaTuCheckpoint.dsNodeDaDi) !== JSON.stringify(["buoc1", "buoc2", "buoc3", "buoc4", "kiemTra", "xuLyLoi", "thuLai"])) {
  throw new Error("chay tu checkpoint phai cho dsNodeDaDi CUOI CUNG giong het chay tu dau");
}
if (tongLuotGoiTuCheckpoint !== 3) throw new Error("chay tu checkpoint (bo qua 4 buoc chuan bi DA chay truoc do) chi duoc ton DUNG 3 luot goi MOI");
if (soLuotGoiTietKiem !== 4) throw new Error("so luot goi tiet kiem nho resume phai la 4 (7 - 3)");

if (demGoiTuCheckpoint["buoc1"] !== undefined) throw new Error("buoc1 KHONG duoc goi lai sau khi resume tu checkpoint");
if (demGoiTuCheckpoint["kiemTra"] !== 1) throw new Error("kiemTra phai duoc goi DUNG 1 lan sau checkpoint");

const demGoiRieng: Record<string, number> = {};
const ketQuaHopLe = chayDoThiCoDemGoi(taoDoThiBoss(5, true), "buoc1", demGoiRieng);
if (ketQuaHopLe.dsNodeDaDi[ketQuaHopLe.dsNodeDaDi.length - 1] !== "xuLyHopLe") {
  throw new Error("kich ban giaTri=5 phai ket thuc o xuLyHopLe");
}
if (tinhTongSoLuotGoi(demGoiRieng) !== 6) throw new Error("kich ban giaTri=5 (hop le, khong vao xuLyLoi) chi ton 6 luot goi");
```

:::hints
- kind: attention
  body: "Hai cho trong. chayDoThiCoDemGoi: DUNG HET bai 4 -- vong while tu tenNodeGoc, MOI vong tang demGoiMoiNode[tenHienTai], ghi node vao dsNodeDaDi, xu ly loi/tenNodeTiepTheo dung nhu bai 1. chayDoThiTuCheckpoint: DUNG HET bai 4 -- goi chayDoThiCoDemGoi bat dau o checkpoint.tenNodeTiepTheo, ghep checkpoint.dsNodeDaDiTruocDo VAO TRUOC dsNodeDaDi moi tinh duoc."
- kind: strategy
  body: "chayDoThiCoDemGoi: const dsNodeDaDi: string[] = []; let tenHienTai: string | null = tenNodeGoc; let soBuoc = 0; while (tenHienTai !== null && soBuoc < SO_BUOC_TOI_DA_AN_TOAN) { soBuoc++; const node: NodeXuLy | undefined = doThi[tenHienTai]; if (!node) return { thanhCong: false, loi: \"node_khong_ton_tai\", dsNodeDaDi }; demGoiMoiNode[tenHienTai] = (demGoiMoiNode[tenHienTai] ?? 0) + 1; dsNodeDaDi.push(tenHienTai); const kq: KetQuaNode = node.chay(); if (kq.trangThai === \"loi\") return { thanhCong: false, loi: \"loi_tai_node\", dsNodeDaDi }; tenHienTai = kq.tenNodeTiepTheo; } return { thanhCong: true, dsNodeDaDi }; chayDoThiTuCheckpoint: const ketQuaMoi = chayDoThiCoDemGoi(doThi, checkpoint.tenNodeTiepTheo, demGoiMoiNode); return { ...ketQuaMoi, dsNodeDaDi: [...checkpoint.dsNodeDaDiTruocDo, ...ketQuaMoi.dsNodeDaDi] };"
- kind: one-line
  body: "Sao chep dung hai ham tu bai 4 (chayDoThiCoDemGoi va chayDoThiTuCheckpoint), GIU NGUYEN toan bo logic ben trong."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 10000
- tier: output
  match: contains
  expect: "1 0.875 7 3 4"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Completion `100%`, edge coverage CHỈ `87.5%` — hai con số ĐO hai thứ
khác nhau, y hệt bài học từ bài `3`. VÀ `7` lệnh gọi TỪ ĐẦU so VỚI `3`
lệnh gọi TỪ checkpoint — CÙNG một kết quả cuối cùng, khác nhau HOÀN
TOÀN Ở chi phí đạt được nó. Quest `q9.5a` — "Kỹ thuật Đồ thị: node+edge,
rẽ nhánh, edge coverage, checkpoint/resume, VÀ catamorphism" — khép lại
tại `6/6`.
::::

::::reflect{#nghi-lai}
Bốn cơ chế BOSS ráp lại không hề "cộng dồn" thành một con số DUY NHẤT —
chúng đo BỐN chiều TÁCH BIỆT của cùng một đồ thị: node+edge tường minh
(bài `1`) cho ta MỘT cách CHẠY một đường đi; rẽ nhánh (bài `2`) cho một
node QUYẾT ĐỊNH đường đi NÀO dựa trên dữ liệu; edge coverage (bài `3`)
đo BỘ kịch bản đã chạm được BAO NHIÊU CẠNH trong TOÀN BỘ cấu trúc CÓ
THỂ; checkpoint/resume (bài `4`) đo MỘT lần chạy CỤ THỂ tiết kiệm được
BAO NHIÊU lệnh gọi khi không phải bắt đầu lại. MASTERPLAN đặt tên hai
trục đo CHÍNH của GRAPH LÀ "edge coverage" VÀ "replay từ checkpoint" —
VÀ số liệu Ở bài này (`0.875` VÀ `4`) chứng minh CẢ HAI đều đo được
TRÊN CÙNG một đồ thị, KHÔNG cần đánh đổi cái NÀY để đo được cái KIA. Đây
LÀ nền tảng track `T9.5` xây tiếp: một đồ thị THẬT (nhiều node, rẽ
nhánh, có thể resume) không LÀ một vòng lặp lớn hơn — nó LÀ một hình
dạng control-flow KHÁC hẳn, đo bằng những con số KHÁC hẳn LOOP đã dạy.
::::

::::checkpoint{mastery=0.95}
::::
