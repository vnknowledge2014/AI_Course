---
id: ky-nghe-ung-dung-ai.ky-thuat-do-thi.so-sanh-cau-hinh-do-thi-qua-bang-danh-gia
title: "Bảng so sánh nhiều cấu hình đồ thị — audit, chu trình, fan-out"
summary: "CauHinhDoThi {ten, coPhatHienChuTrinh, coFanOut, coAudit} mo ta MOT lua chon ket hop ba co che q9.5b. chayMotCauHinhDoThi chay CUNG mot bo 5 kich ban (3 kich ban re-nhanh tren mot do thi CO CANH QUAY LAI khi coTheThuLai=true, 2 kich ban fan-out) qua DUNG MOT cau hinh, tra ve KetQuaDanhGia {ten, tyLeHoanThanh, tyLeEdgeCoverage, tongLuotGoi} -- BA truc do TACH BIET tren tong 7 canh co the. Bon cau hinh xac thuc qua engine that: 'toi-gian' (khong cycle, khong fan-out) cho tyLeHoanThanh=4/5 (0.8), tyLeEdgeCoverage=6/7 (JS in ra 0.8571428571428571), tongLuotGoi=29. 'chi-cycle' (bat phat hien chu trinh) GIU NGUYEN tyLeHoanThanh=0.8 nhung tongLuotGoi giam con 12 (tiet kiem 17 tu kich ban chu trinh) va tyLeEdgeCoverage GIAM con 5/7 (in ra 0.7142857142857143 -- mat canh quay lai vi dung som). 'chi-fanout' (bat fan-out, khong cycle) cho tyLeHoanThanh GIAM con 3/5 (0.6 -- fan-out bat dung loi nhanh B ma cau hinh khong-fan-out bo lo) nhung tyLeEdgeCoverage TANG len 7/7 (1, 100%) va tongLuotGoi=32. 'day-du' (ca ba co che, coAudit khong anh huong tyLeHoanThanh/tyLeEdgeCoverage/tongLuotGoi) cho tyLeHoanThanh=0.6, tyLeEdgeCoverage=6/7, tongLuotGoi=15 -- audit la co che 'mien phi' tren ca ba truc do."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-do-thi
order: 11
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [kna.so-sanh-cau-hinh-do-thi-qua-bang-danh-gia]
requires: [kna.phat-hien-chu-trinh]
concepts: [kna.so-sanh-cau-hinh-do-thi-qua-bang-danh-gia]
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
Bốn bài Ở q9.5b: fan-out/fan-in, audit trail, test một nhánh, phát hiện
chu trình — mỗi bài giải quyết ĐÚNG một chế độ hỏng, tách riêng khỏi mọi
cơ chế khác. Nhưng một đội kỹ sư THẬT không bật MỘT cơ chế một lần — họ
so sánh CHỤC tổ hợp cấu hình. Bài này đóng gói phép so sánh Ở q9.3a/q9.4a
(HARNESS/LOOP đã dạy) VÀO đúng ba cơ chế GRAPH vừa học.
::::

::::explain{#cau_hinh_do_thi_va_bo_kich_ban}
`CauHinhDoThi` gói TÊN một lựa chọn VÀ BA cờ boolean: `coPhatHienChuTrinh`
(bài `10`), `coFanOut` (bài `7`), `coAudit` (bài `8`). Bộ kịch bản dùng
CHUNG cho MỌI cấu hình gồm HAI loại: `"re_nhanh"` (chạy trên MỘT đồ thị
CÓ cạnh quay lại — `thuLai` trỏ VỀ `"kiemTra"`, giống bài `10`) VÀ
`"fan_out"` (chạy qua HAI nhánh A/B, giống bài `7`):

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

function taoNodeLoi(): NodeXuLy {
  return {
    chay(): KetQuaNode {
      return { trangThai: "loi", tenNodeTiepTheo: null };
    },
  };
}

const SO_BUOC_TOI_DA_AN_TOAN = 20;

type KetQuaChayDoThi =
  | { thanhCong: true; dsNodeDaDi: string[] }
  | { thanhCong: false; loi: string; dsNodeDaDi: string[] };

function chayDoThi(doThi: DoThi, tenNodeGoc: string): KetQuaChayDoThi {
  const dsNodeDaDi: string[] = [];
  let tenHienTai: string | null = tenNodeGoc;
  let soBuoc = 0;
  while (tenHienTai !== null && soBuoc < SO_BUOC_TOI_DA_AN_TOAN) {
    soBuoc++;
    const node: NodeXuLy | undefined = doThi[tenHienTai];
    if (!node) return { thanhCong: false, loi: "node_khong_ton_tai", dsNodeDaDi };
    dsNodeDaDi.push(tenHienTai);
    const kq: KetQuaNode = node.chay();
    if (kq.trangThai === "loi") return { thanhCong: false, loi: "loi_tai_node", dsNodeDaDi };
    tenHienTai = kq.tenNodeTiepTheo;
  }
  return { thanhCong: true, dsNodeDaDi };
}

function layCacCanhDaDi(dsNodeDaDi: string[]): string[] {
  const canh: string[] = [];
  for (let i = 0; i < dsNodeDaDi.length - 1; i++) {
    canh.push(`${dsNodeDaDi[i]}->${dsNodeDaDi[i + 1]}`);
  }
  return canh;
}

type KetQuaChayMoRong = { thanhCong: boolean; dsNodeDaDi: string[] };

function chayDoThiTuyChonChuTrinh(doThi: DoThi, tenNodeGoc: string, coPhatHienChuTrinh: boolean): KetQuaChayMoRong {
  const dsNodeDaDi: string[] = [];
  const daGhePhamVi = new Set<string>();
  let tenHienTai: string | null = tenNodeGoc;
  let soBuoc = 0;
  while (tenHienTai !== null) {
    if (soBuoc >= SO_BUOC_TOI_DA_AN_TOAN) return { thanhCong: false, dsNodeDaDi };
    if (coPhatHienChuTrinh && daGhePhamVi.has(tenHienTai)) return { thanhCong: false, dsNodeDaDi };
    soBuoc++;
    const node: NodeXuLy | undefined = doThi[tenHienTai];
    if (!node) return { thanhCong: false, dsNodeDaDi };
    daGhePhamVi.add(tenHienTai);
    dsNodeDaDi.push(tenHienTai);
    const kq: KetQuaNode = node.chay();
    if (kq.trangThai === "loi") return { thanhCong: false, dsNodeDaDi };
    tenHienTai = kq.tenNodeTiepTheo;
  }
  return { thanhCong: true, dsNodeDaDi };
}

type KetQuaFanOutTuyChon = { thanhCong: boolean; dsNodeDaDiNhanhA: string[]; dsNodeDaDiNhanhB: string[] };

function chayFanOutTuyChon(
  doThiNhanhA: DoThi, tenGocNhanhA: string,
  doThiNhanhB: DoThi, tenGocNhanhB: string,
  coFanOut: boolean,
): KetQuaFanOutTuyChon {
  const ketQuaA = chayDoThi(doThiNhanhA, tenGocNhanhA);
  if (!coFanOut) {
    return { thanhCong: ketQuaA.thanhCong, dsNodeDaDiNhanhA: ketQuaA.dsNodeDaDi, dsNodeDaDiNhanhB: [] };
  }
  const ketQuaB = chayDoThi(doThiNhanhB, tenGocNhanhB);
  return {
    thanhCong: ketQuaA.thanhCong && ketQuaB.thanhCong,
    dsNodeDaDiNhanhA: ketQuaA.dsNodeDaDi,
    dsNodeDaDiNhanhB: ketQuaB.dsNodeDaDi,
  };
}

function taoDoThiSoSanh(giaTri: number, coTheThuLai: boolean): DoThi {
  return {
    kiemTra: {
      chay(): KetQuaNode {
        if (giaTri >= 0) return { trangThai: "xong", tenNodeTiepTheo: "opLe" };
        return { trangThai: "xong", tenNodeTiepTheo: "xuLyLoi" };
      },
    },
    opLe: taoNodeThanhCong(null),
    xuLyLoi: {
      chay(): KetQuaNode {
        if (coTheThuLai) return { trangThai: "xong", tenNodeTiepTheo: "thuLai" };
        return { trangThai: "xong", tenNodeTiepTheo: "boCuoc" };
      },
    },
    thuLai: taoNodeThanhCong("kiemTra"),
    boCuoc: taoNodeThanhCong(null),
  };
}

console.log(taoDoThiSoSanh(-3, true).thuLai!.chay().tenNodeTiepTheo);
```

```text title=readonly
kiemTra
```

`thuLai` trỏ VỀ `"kiemTra"` — cạnh quay lại NÀY chỉ trở thành chu trình
THẬT khi `giaTri` VẪN âm mỗi lần `"kiemTra"` được đánh giá lại (đúng khi
`giaTri` cố định LÀ `-3`, một node KHÔNG BAO GIỜ tự thoát vòng lặp thử
lại).
::::

::::example{#bon_cau_hinh_ba_truc_do}
`chayMotCauHinhDoThi` chạy `5` kịch bản (`3` "re_nhanh" — `giaTri=5`,
`giaTri=-3,coTheThuLai=false`, `giaTri=-3,coTheThuLai=true` [chu trình];
CỘNG `2` "fan_out" — nhánh B hợp lệ, nhánh B lỗi) qua MỘT cấu hình, gộp
CẢ `dsCanhDaDi` (edge coverage) LẪN `soLuotGoi` (chi phí):

```typescript title=readonly
type KichBanSoSanh =
  | { loai: "re_nhanh"; giaTri: number; coTheThuLai: boolean }
  | { loai: "fan_out"; nhanhBLoi: boolean };

type CauHinhDoThi = { ten: string; coPhatHienChuTrinh: boolean; coFanOut: boolean; coAudit: boolean };

type KetQuaMotKichBan = { thanhCong: boolean; dsCanhDaDi: string[]; soLuotGoi: number };

type KetQuaDanhGiaDoThi = { ten: string; tyLeHoanThanh: number; tyLeEdgeCoverage: number; tongLuotGoi: number };

// (Xem khoi `solution` o buoc ::::code de doc TOAN VAN chayMotKichBan,
// doThiNhanhA/B, taoDoThiSoSanh, chayDoThiTuyChonChuTrinh, chayFanOutTuyChon.)

const TOAN_BO_CANH_CO_THE = [
  "kiemTra->opLe", "kiemTra->xuLyLoi", "xuLyLoi->thuLai", "xuLyLoi->boCuoc", "thuLai->kiemTra",
  "nhanhA1->nhanhA2", "nhanhB1->nhanhB2",
];

const boKichBan: KichBanSoSanh[] = [
  { loai: "re_nhanh", giaTri: 5, coTheThuLai: true },
  { loai: "re_nhanh", giaTri: -3, coTheThuLai: false },
  { loai: "re_nhanh", giaTri: -3, coTheThuLai: true },
  { loai: "fan_out", nhanhBLoi: false },
  { loai: "fan_out", nhanhBLoi: true },
];

const cacCauHinh: CauHinhDoThi[] = [
  { ten: "toi-gian", coPhatHienChuTrinh: false, coFanOut: false, coAudit: false },
  { ten: "chi-cycle", coPhatHienChuTrinh: true, coFanOut: false, coAudit: false },
  { ten: "chi-fanout", coPhatHienChuTrinh: false, coFanOut: true, coAudit: false },
  { ten: "day-du", coPhatHienChuTrinh: true, coFanOut: true, coAudit: true },
];

// bangSoSanh = soSanhNhieuCauHinhDoThi(cacCauHinh, boKichBan, TOAN_BO_CANH_CO_THE);
console.log(TOAN_BO_CANH_CO_THE.length, boKichBan.length, cacCauHinh.length);
```

```text title=readonly
7 5 4
```

`7` cạnh CÓ THỂ (`5` từ đồ thị rẽ nhánh, `2` từ hai nhánh fan-out), `5`
kịch bản, `4` cấu hình. Chạy đủ (Ở bước `::::code`) cho ra bảng:
`"toi-gian"` → `tyLeHoanThanh` LÀ `4/5` (`0.8`), edge coverage LÀ `6/7`
(JS in ra `0.8571428571428571`), `tongLuotGoi` LÀ `29`. `"chi-cycle"`
GIỮ NGUYÊN `tyLeHoanThanh=0.8` NHƯNG `tongLuotGoi` giảm còn `12` VÀ edge
coverage GIẢM còn `5/7` (in ra `0.7142857142857143` — MẤT cạnh quay lại,
vì dừng SỚM trước khi đi lại cạnh đó). `"chi-fanout"` làm `tyLeHoanThanh`
GIẢM còn `3/5` (`0.6` — fan-out bắt ĐÚNG lỗi nhánh B MÀ cấu hình
"toi-gian" đã BỎ LỠ) NHƯNG edge coverage TĂNG lên `7/7` (`1`, `100%`).
`"day-du"` (bật CẢ ba cờ) cho CÙNG số VỚI `"chi-fanout"` Ở `tyLeHoanThanh`
VÀ `tongLuotGoi` giảm (nhờ cycle detection), edge coverage LÀ `6/7` —
CHỨNG MINH `coAudit` KHÔNG hề đổi bất kỳ con số NÀO trong BA cột.
::::

::::predict{#doan-bat-fanout-doi-hoan-thanh commitOnce}
So `"toi-gian"` (`coFanOut: false`) VỚI `"chi-fanout"` (`coFanOut: true`,
mọi cờ khác giữ nguyên) — VÌ SAO `"chi-fanout"` có `tyLeHoanThanh` THẤP
HƠN, dù nó LÀ cấu hình "đầy đủ tính năng" hơn?

:::opt{correct}
Vì `"toi-gian"` KHÔNG hề kiểm nhánh B của kịch bản fan-out (chỉ chạy
nhánh A rồi BÁO thành công LUÔN) — kịch bản `nhanhBLoi: true` bị BỎ LỠ,
tính LÀ thành công GIẢ; `"chi-fanout"` THẬT SỰ chạy nhánh B, phát hiện
lỗi, VÀ báo thất bại ĐÚNG — `tyLeHoanThanh` thấp hơn Ở ĐÂY nghĩa LÀ
CHÍNH XÁC hơn, không phải TỆ hơn
:::
:::opt
Vì bật fan-out khiến `SO_BUOC_TOI_DA_AN_TOAN` bị chạm SỚM hơn Ở NHIỀU
kịch bản hơn, làm tăng số lần `"hetBuoc"`
::why
Nhầm rằng fan-out ảnh hưởng tới LƯỚI AN TOÀN hay chu trình — nhưng
`coFanOut` CHỈ quyết định `chayFanOutTuyChon` có chạy nhánh B hay không;
nó KHÔNG chạm tới `chayDoThiTuyChonChuTrinh` (phần rẽ nhánh/chu trình)
CHÚT NÀO — hai cờ hoàn toàn ĐỘC LẬP.

Chỗ lệch: chênh lệch `tyLeHoanThanh` giữa hai cấu hình CHỈ tới từ kịch
bản `"fan_out"` (`nhanhBLoi: true`) — ba kịch bản `"re_nhanh"` cho ra
CÙNG kết quả Ở cả hai cấu hình, vì `coPhatHienChuTrinh` giống nhau Ở CẢ
HAI.
::
:::
:::opt
Vì `"chi-fanout"` chạy NHIỀU kịch bản hơn `"toi-gian"` (dùng thêm dữ liệu
Ở nhánh B), nên MẪU SỐ của `tyLeHoanThanh` lớn hơn
::why
Nhầm rằng số kịch bản (MẪU SỐ) khác nhau giữa hai cấu hình — nhưng CẢ
HAI đều chạy TRÊN CÙNG `boKichBan` gồm ĐÚNG `5` phần tử; `tyLeHoanThanh`
của CẢ HAI đều chia cho `5`. Khác biệt CHỈ nằm Ở TỬ SỐ (bao nhiêu kịch
bản được TÍNH LÀ thành công), không phải mẫu số.

Chỗ lệch: `chayMotCauHinhDoThi` gọi `dsKichBan.map(...)` — LUÔN ánh xạ
qua ĐÚNG `dsKichBan.length` phần tử, bất kể cấu hình.
::
:::
::::

::::code{#viet_so_sanh_cau_hinh_do_thi}
Hoàn thiện `chayMotCauHinhDoThi` — chạy `chayMotKichBan` cho MỖI kịch
bản trong `dsKichBan` (VỚI CÙNG `cauHinh`), tính `tyLeHoanThanh` (tỉ lệ
`thanhCong`), gộp `dsCanhDaDi` của MỌI kịch bản VÀO một `Set`, tính
`tyLeEdgeCoverage` (đếm phần tử `toanBoCanh` CÓ trong `Set` đó, chia
`toanBoCanh.length`), VÀ `tongLuotGoi` (tổng `soLuotGoi` của MỌI kịch
bản). Hoàn thiện `soSanhNhieuCauHinhDoThi` — chạy `chayMotCauHinhDoThi`
cho TỪNG cấu hình trong `cacCauHinh`, trả về mảng kết quả.

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

function taoNodeLoi(): NodeXuLy {
  return {
    chay(): KetQuaNode {
      return { trangThai: "loi", tenNodeTiepTheo: null };
    },
  };
}

const SO_BUOC_TOI_DA_AN_TOAN = 20;

type KetQuaChayDoThi =
  | { thanhCong: true; dsNodeDaDi: string[] }
  | { thanhCong: false; loi: string; dsNodeDaDi: string[] };

function chayDoThi(doThi: DoThi, tenNodeGoc: string): KetQuaChayDoThi {
  const dsNodeDaDi: string[] = [];
  let tenHienTai: string | null = tenNodeGoc;
  let soBuoc = 0;
  while (tenHienTai !== null && soBuoc < SO_BUOC_TOI_DA_AN_TOAN) {
    soBuoc++;
    const node: NodeXuLy | undefined = doThi[tenHienTai];
    if (!node) return { thanhCong: false, loi: "node_khong_ton_tai", dsNodeDaDi };
    dsNodeDaDi.push(tenHienTai);
    const kq: KetQuaNode = node.chay();
    if (kq.trangThai === "loi") return { thanhCong: false, loi: "loi_tai_node", dsNodeDaDi };
    tenHienTai = kq.tenNodeTiepTheo;
  }
  return { thanhCong: true, dsNodeDaDi };
}

function layCacCanhDaDi(dsNodeDaDi: string[]): string[] {
  const canh: string[] = [];
  for (let i = 0; i < dsNodeDaDi.length - 1; i++) {
    canh.push(`${dsNodeDaDi[i]}->${dsNodeDaDi[i + 1]}`);
  }
  return canh;
}

type KetQuaChayMoRong = { thanhCong: boolean; dsNodeDaDi: string[] };

function chayDoThiTuyChonChuTrinh(doThi: DoThi, tenNodeGoc: string, coPhatHienChuTrinh: boolean): KetQuaChayMoRong {
  const dsNodeDaDi: string[] = [];
  const daGhePhamVi = new Set<string>();
  let tenHienTai: string | null = tenNodeGoc;
  let soBuoc = 0;
  while (tenHienTai !== null) {
    if (soBuoc >= SO_BUOC_TOI_DA_AN_TOAN) return { thanhCong: false, dsNodeDaDi };
    if (coPhatHienChuTrinh && daGhePhamVi.has(tenHienTai)) return { thanhCong: false, dsNodeDaDi };
    soBuoc++;
    const node: NodeXuLy | undefined = doThi[tenHienTai];
    if (!node) return { thanhCong: false, dsNodeDaDi };
    daGhePhamVi.add(tenHienTai);
    dsNodeDaDi.push(tenHienTai);
    const kq: KetQuaNode = node.chay();
    if (kq.trangThai === "loi") return { thanhCong: false, dsNodeDaDi };
    tenHienTai = kq.tenNodeTiepTheo;
  }
  return { thanhCong: true, dsNodeDaDi };
}

type KetQuaFanOutTuyChon = { thanhCong: boolean; dsNodeDaDiNhanhA: string[]; dsNodeDaDiNhanhB: string[] };

function chayFanOutTuyChon(
  doThiNhanhA: DoThi, tenGocNhanhA: string,
  doThiNhanhB: DoThi, tenGocNhanhB: string,
  coFanOut: boolean,
): KetQuaFanOutTuyChon {
  const ketQuaA = chayDoThi(doThiNhanhA, tenGocNhanhA);
  if (!coFanOut) {
    return { thanhCong: ketQuaA.thanhCong, dsNodeDaDiNhanhA: ketQuaA.dsNodeDaDi, dsNodeDaDiNhanhB: [] };
  }
  const ketQuaB = chayDoThi(doThiNhanhB, tenGocNhanhB);
  return {
    thanhCong: ketQuaA.thanhCong && ketQuaB.thanhCong,
    dsNodeDaDiNhanhA: ketQuaA.dsNodeDaDi,
    dsNodeDaDiNhanhB: ketQuaB.dsNodeDaDi,
  };
}

function taoDoThiSoSanh(giaTri: number, coTheThuLai: boolean): DoThi {
  return {
    kiemTra: {
      chay(): KetQuaNode {
        if (giaTri >= 0) return { trangThai: "xong", tenNodeTiepTheo: "opLe" };
        return { trangThai: "xong", tenNodeTiepTheo: "xuLyLoi" };
      },
    },
    opLe: taoNodeThanhCong(null),
    xuLyLoi: {
      chay(): KetQuaNode {
        if (coTheThuLai) return { trangThai: "xong", tenNodeTiepTheo: "thuLai" };
        return { trangThai: "xong", tenNodeTiepTheo: "boCuoc" };
      },
    },
    thuLai: taoNodeThanhCong("kiemTra"),
    boCuoc: taoNodeThanhCong(null),
  };
}

const doThiNhanhA: DoThi = { nhanhA1: taoNodeThanhCong("nhanhA2"), nhanhA2: taoNodeThanhCong(null) };
const doThiNhanhBHopLe: DoThi = { nhanhB1: taoNodeThanhCong("nhanhB2"), nhanhB2: taoNodeThanhCong(null) };
const doThiNhanhBLoi: DoThi = { nhanhB1: taoNodeLoi() };

type KichBanSoSanh =
  | { loai: "re_nhanh"; giaTri: number; coTheThuLai: boolean }
  | { loai: "fan_out"; nhanhBLoi: boolean };

type KetQuaMotKichBan = { thanhCong: boolean; dsCanhDaDi: string[]; soLuotGoi: number };

type CauHinhDoThi = { ten: string; coPhatHienChuTrinh: boolean; coFanOut: boolean; coAudit: boolean };

function chayMotKichBan(kichBan: KichBanSoSanh, cauHinh: CauHinhDoThi): KetQuaMotKichBan {
  if (kichBan.loai === "re_nhanh") {
    const doThi = taoDoThiSoSanh(kichBan.giaTri, kichBan.coTheThuLai);
    const kq = chayDoThiTuyChonChuTrinh(doThi, "kiemTra", cauHinh.coPhatHienChuTrinh);
    return { thanhCong: kq.thanhCong, dsCanhDaDi: layCacCanhDaDi(kq.dsNodeDaDi), soLuotGoi: kq.dsNodeDaDi.length };
  }
  const doThiB = kichBan.nhanhBLoi ? doThiNhanhBLoi : doThiNhanhBHopLe;
  const kq = chayFanOutTuyChon(doThiNhanhA, "nhanhA1", doThiB, "nhanhB1", cauHinh.coFanOut);
  const canh = [...layCacCanhDaDi(kq.dsNodeDaDiNhanhA), ...layCacCanhDaDi(kq.dsNodeDaDiNhanhB)];
  const soLuotGoi = kq.dsNodeDaDiNhanhA.length + kq.dsNodeDaDiNhanhB.length;
  return { thanhCong: kq.thanhCong, dsCanhDaDi: canh, soLuotGoi };
}

type KetQuaDanhGiaDoThi = { ten: string; tyLeHoanThanh: number; tyLeEdgeCoverage: number; tongLuotGoi: number };

function chayMotCauHinhDoThi(cauHinh: CauHinhDoThi, dsKichBan: KichBanSoSanh[], toanBoCanh: string[]): KetQuaDanhGiaDoThi {
  ___
}

function soSanhNhieuCauHinhDoThi(cacCauHinh: CauHinhDoThi[], dsKichBan: KichBanSoSanh[], toanBoCanh: string[]): KetQuaDanhGiaDoThi[] {
  ___
}

const TOAN_BO_CANH_CO_THE = [
  "kiemTra->opLe", "kiemTra->xuLyLoi", "xuLyLoi->thuLai", "xuLyLoi->boCuoc", "thuLai->kiemTra",
  "nhanhA1->nhanhA2", "nhanhB1->nhanhB2",
];

const boKichBan: KichBanSoSanh[] = [
  { loai: "re_nhanh", giaTri: 5, coTheThuLai: true },
  { loai: "re_nhanh", giaTri: -3, coTheThuLai: false },
  { loai: "re_nhanh", giaTri: -3, coTheThuLai: true },
  { loai: "fan_out", nhanhBLoi: false },
  { loai: "fan_out", nhanhBLoi: true },
];

const cacCauHinh: CauHinhDoThi[] = [
  { ten: "toi-gian", coPhatHienChuTrinh: false, coFanOut: false, coAudit: false },
  { ten: "chi-cycle", coPhatHienChuTrinh: true, coFanOut: false, coAudit: false },
  { ten: "chi-fanout", coPhatHienChuTrinh: false, coFanOut: true, coAudit: false },
  { ten: "day-du", coPhatHienChuTrinh: true, coFanOut: true, coAudit: true },
];

const bangSoSanh = soSanhNhieuCauHinhDoThi(cacCauHinh, boKichBan, TOAN_BO_CANH_CO_THE);
console.log(JSON.stringify(bangSoSanh));
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

function taoNodeLoi(): NodeXuLy {
  return {
    chay(): KetQuaNode {
      return { trangThai: "loi", tenNodeTiepTheo: null };
    },
  };
}

const SO_BUOC_TOI_DA_AN_TOAN = 20;

type KetQuaChayDoThi =
  | { thanhCong: true; dsNodeDaDi: string[] }
  | { thanhCong: false; loi: string; dsNodeDaDi: string[] };

function chayDoThi(doThi: DoThi, tenNodeGoc: string): KetQuaChayDoThi {
  const dsNodeDaDi: string[] = [];
  let tenHienTai: string | null = tenNodeGoc;
  let soBuoc = 0;
  while (tenHienTai !== null && soBuoc < SO_BUOC_TOI_DA_AN_TOAN) {
    soBuoc++;
    const node: NodeXuLy | undefined = doThi[tenHienTai];
    if (!node) return { thanhCong: false, loi: "node_khong_ton_tai", dsNodeDaDi };
    dsNodeDaDi.push(tenHienTai);
    const kq: KetQuaNode = node.chay();
    if (kq.trangThai === "loi") return { thanhCong: false, loi: "loi_tai_node", dsNodeDaDi };
    tenHienTai = kq.tenNodeTiepTheo;
  }
  return { thanhCong: true, dsNodeDaDi };
}

function layCacCanhDaDi(dsNodeDaDi: string[]): string[] {
  const canh: string[] = [];
  for (let i = 0; i < dsNodeDaDi.length - 1; i++) {
    canh.push(`${dsNodeDaDi[i]}->${dsNodeDaDi[i + 1]}`);
  }
  return canh;
}

type KetQuaChayMoRong = { thanhCong: boolean; dsNodeDaDi: string[] };

function chayDoThiTuyChonChuTrinh(doThi: DoThi, tenNodeGoc: string, coPhatHienChuTrinh: boolean): KetQuaChayMoRong {
  const dsNodeDaDi: string[] = [];
  const daGhePhamVi = new Set<string>();
  let tenHienTai: string | null = tenNodeGoc;
  let soBuoc = 0;
  while (tenHienTai !== null) {
    if (soBuoc >= SO_BUOC_TOI_DA_AN_TOAN) return { thanhCong: false, dsNodeDaDi };
    if (coPhatHienChuTrinh && daGhePhamVi.has(tenHienTai)) return { thanhCong: false, dsNodeDaDi };
    soBuoc++;
    const node: NodeXuLy | undefined = doThi[tenHienTai];
    if (!node) return { thanhCong: false, dsNodeDaDi };
    daGhePhamVi.add(tenHienTai);
    dsNodeDaDi.push(tenHienTai);
    const kq: KetQuaNode = node.chay();
    if (kq.trangThai === "loi") return { thanhCong: false, dsNodeDaDi };
    tenHienTai = kq.tenNodeTiepTheo;
  }
  return { thanhCong: true, dsNodeDaDi };
}

type KetQuaFanOutTuyChon = { thanhCong: boolean; dsNodeDaDiNhanhA: string[]; dsNodeDaDiNhanhB: string[] };

function chayFanOutTuyChon(
  doThiNhanhA: DoThi, tenGocNhanhA: string,
  doThiNhanhB: DoThi, tenGocNhanhB: string,
  coFanOut: boolean,
): KetQuaFanOutTuyChon {
  const ketQuaA = chayDoThi(doThiNhanhA, tenGocNhanhA);
  if (!coFanOut) {
    return { thanhCong: ketQuaA.thanhCong, dsNodeDaDiNhanhA: ketQuaA.dsNodeDaDi, dsNodeDaDiNhanhB: [] };
  }
  const ketQuaB = chayDoThi(doThiNhanhB, tenGocNhanhB);
  return {
    thanhCong: ketQuaA.thanhCong && ketQuaB.thanhCong,
    dsNodeDaDiNhanhA: ketQuaA.dsNodeDaDi,
    dsNodeDaDiNhanhB: ketQuaB.dsNodeDaDi,
  };
}

function taoDoThiSoSanh(giaTri: number, coTheThuLai: boolean): DoThi {
  return {
    kiemTra: {
      chay(): KetQuaNode {
        if (giaTri >= 0) return { trangThai: "xong", tenNodeTiepTheo: "opLe" };
        return { trangThai: "xong", tenNodeTiepTheo: "xuLyLoi" };
      },
    },
    opLe: taoNodeThanhCong(null),
    xuLyLoi: {
      chay(): KetQuaNode {
        if (coTheThuLai) return { trangThai: "xong", tenNodeTiepTheo: "thuLai" };
        return { trangThai: "xong", tenNodeTiepTheo: "boCuoc" };
      },
    },
    thuLai: taoNodeThanhCong("kiemTra"),
    boCuoc: taoNodeThanhCong(null),
  };
}

const doThiNhanhA: DoThi = { nhanhA1: taoNodeThanhCong("nhanhA2"), nhanhA2: taoNodeThanhCong(null) };
const doThiNhanhBHopLe: DoThi = { nhanhB1: taoNodeThanhCong("nhanhB2"), nhanhB2: taoNodeThanhCong(null) };
const doThiNhanhBLoi: DoThi = { nhanhB1: taoNodeLoi() };

type KichBanSoSanh =
  | { loai: "re_nhanh"; giaTri: number; coTheThuLai: boolean }
  | { loai: "fan_out"; nhanhBLoi: boolean };

type KetQuaMotKichBan = { thanhCong: boolean; dsCanhDaDi: string[]; soLuotGoi: number };

type CauHinhDoThi = { ten: string; coPhatHienChuTrinh: boolean; coFanOut: boolean; coAudit: boolean };

function chayMotKichBan(kichBan: KichBanSoSanh, cauHinh: CauHinhDoThi): KetQuaMotKichBan {
  if (kichBan.loai === "re_nhanh") {
    const doThi = taoDoThiSoSanh(kichBan.giaTri, kichBan.coTheThuLai);
    const kq = chayDoThiTuyChonChuTrinh(doThi, "kiemTra", cauHinh.coPhatHienChuTrinh);
    return { thanhCong: kq.thanhCong, dsCanhDaDi: layCacCanhDaDi(kq.dsNodeDaDi), soLuotGoi: kq.dsNodeDaDi.length };
  }
  const doThiB = kichBan.nhanhBLoi ? doThiNhanhBLoi : doThiNhanhBHopLe;
  const kq = chayFanOutTuyChon(doThiNhanhA, "nhanhA1", doThiB, "nhanhB1", cauHinh.coFanOut);
  const canh = [...layCacCanhDaDi(kq.dsNodeDaDiNhanhA), ...layCacCanhDaDi(kq.dsNodeDaDiNhanhB)];
  const soLuotGoi = kq.dsNodeDaDiNhanhA.length + kq.dsNodeDaDiNhanhB.length;
  return { thanhCong: kq.thanhCong, dsCanhDaDi: canh, soLuotGoi };
}

type KetQuaDanhGiaDoThi = { ten: string; tyLeHoanThanh: number; tyLeEdgeCoverage: number; tongLuotGoi: number };

function chayMotCauHinhDoThi(cauHinh: CauHinhDoThi, dsKichBan: KichBanSoSanh[], toanBoCanh: string[]): KetQuaDanhGiaDoThi {
  const ketQuaTungKichBan = dsKichBan.map((kb) => chayMotKichBan(kb, cauHinh));
  const tyLeHoanThanh = ketQuaTungKichBan.filter((k) => k.thanhCong).length / ketQuaTungKichBan.length;
  const tapCanh = new Set<string>();
  for (const k of ketQuaTungKichBan) for (const c of k.dsCanhDaDi) tapCanh.add(c);
  const tyLeEdgeCoverage = toanBoCanh.filter((c) => tapCanh.has(c)).length / toanBoCanh.length;
  const tongLuotGoi = ketQuaTungKichBan.reduce((tong, k) => tong + k.soLuotGoi, 0);
  return { ten: cauHinh.ten, tyLeHoanThanh, tyLeEdgeCoverage, tongLuotGoi };
}

function soSanhNhieuCauHinhDoThi(cacCauHinh: CauHinhDoThi[], dsKichBan: KichBanSoSanh[], toanBoCanh: string[]): KetQuaDanhGiaDoThi[] {
  return cacCauHinh.map((ch) => chayMotCauHinhDoThi(ch, dsKichBan, toanBoCanh));
}

const TOAN_BO_CANH_CO_THE = [
  "kiemTra->opLe", "kiemTra->xuLyLoi", "xuLyLoi->thuLai", "xuLyLoi->boCuoc", "thuLai->kiemTra",
  "nhanhA1->nhanhA2", "nhanhB1->nhanhB2",
];

const boKichBan: KichBanSoSanh[] = [
  { loai: "re_nhanh", giaTri: 5, coTheThuLai: true },
  { loai: "re_nhanh", giaTri: -3, coTheThuLai: false },
  { loai: "re_nhanh", giaTri: -3, coTheThuLai: true },
  { loai: "fan_out", nhanhBLoi: false },
  { loai: "fan_out", nhanhBLoi: true },
];

const cacCauHinh: CauHinhDoThi[] = [
  { ten: "toi-gian", coPhatHienChuTrinh: false, coFanOut: false, coAudit: false },
  { ten: "chi-cycle", coPhatHienChuTrinh: true, coFanOut: false, coAudit: false },
  { ten: "chi-fanout", coPhatHienChuTrinh: false, coFanOut: true, coAudit: false },
  { ten: "day-du", coPhatHienChuTrinh: true, coFanOut: true, coAudit: true },
];

const bangSoSanh = soSanhNhieuCauHinhDoThi(cacCauHinh, boKichBan, TOAN_BO_CANH_CO_THE);
console.log(JSON.stringify(bangSoSanh));
```

```typescript title=test
if (bangSoSanh.length !== 4) throw new Error("soSanhNhieuCauHinhDoThi phai tra ve dung 4 phan tu");

const toiGian = bangSoSanh[0]!;
if (toiGian.ten !== "toi-gian" || Math.abs(toiGian.tyLeHoanThanh - 0.8) > 1e-9 || toiGian.tongLuotGoi !== 29) {
  throw new Error("cau hinh toi-gian phai co tyLeHoanThanh=0.8 va tongLuotGoi=29");
}
if (Math.abs(toiGian.tyLeEdgeCoverage - 6 / 7) > 1e-9) throw new Error("cau hinh toi-gian phai co tyLeEdgeCoverage la 6/7");

const chiCycle = bangSoSanh[1]!;
if (chiCycle.ten !== "chi-cycle" || Math.abs(chiCycle.tyLeHoanThanh - 0.8) > 1e-9 || chiCycle.tongLuotGoi !== 12) {
  throw new Error("cau hinh chi-cycle phai GIU tyLeHoanThanh=0.8 (khong doi) nhung tongLuotGoi giam con 12");
}
if (Math.abs(chiCycle.tyLeEdgeCoverage - 5 / 7) > 1e-9) throw new Error("cau hinh chi-cycle phai co tyLeEdgeCoverage la 5/7 (mat canh quay lai)");

const chiFanout = bangSoSanh[2]!;
if (chiFanout.ten !== "chi-fanout" || Math.abs(chiFanout.tyLeHoanThanh - 0.6) > 1e-9 || chiFanout.tongLuotGoi !== 32) {
  throw new Error("cau hinh chi-fanout phai co tyLeHoanThanh=0.6 (GIAM vi bat loi that o nhanh B) va tongLuotGoi=32");
}
if (Math.abs(chiFanout.tyLeEdgeCoverage - 1) > 1e-9) throw new Error("cau hinh chi-fanout phai dat tyLeEdgeCoverage=1 (100%)");

const dayDu = bangSoSanh[3]!;
if (dayDu.ten !== "day-du" || Math.abs(dayDu.tyLeHoanThanh - 0.6) > 1e-9 || dayDu.tongLuotGoi !== 15) {
  throw new Error("cau hinh day-du phai co tyLeHoanThanh=0.6 va tongLuotGoi=15");
}
if (Math.abs(dayDu.tyLeEdgeCoverage - 6 / 7) > 1e-9) throw new Error("cau hinh day-du phai co tyLeEdgeCoverage la 6/7");

const motCauHinh = soSanhNhieuCauHinhDoThi(
  [{ ten: "rieng", coPhatHienChuTrinh: true, coFanOut: true, coAudit: false }],
  boKichBan,
  TOAN_BO_CANH_CO_THE,
);
if (motCauHinh.length !== 1) throw new Error("doi so danh sach cau hinh phai doi do dai mang tra ve -- tham so phai duoc dung that");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (chayMotCauHinhDoThi): dung .map goi chayMotKichBan cho tung kich ban VOI CUNG cauHinh; tinh tyLeHoanThanh bang so luong thanhCong chia tong so kich ban; gop dsCanhDaDi cua MOI kich ban vao MOT Set; tinh tyLeEdgeCoverage bang so phan tu toanBoCanh CO trong Set do chia toanBoCanh.length; tinh tongLuotGoi bang tong soLuotGoi cua MOI kich ban; tra ve object KetQuaDanhGiaDoThi. Cho hai (soSanhNhieuCauHinhDoThi): dung .map tren cacCauHinh, moi phan tu goi chayMotCauHinhDoThi voi CUNG dsKichBan va toanBoCanh."
- kind: strategy
  body: "Cho dau: const ketQuaTungKichBan = dsKichBan.map((kb) => chayMotKichBan(kb, cauHinh)); const tyLeHoanThanh = ketQuaTungKichBan.filter((k) => k.thanhCong).length / ketQuaTungKichBan.length; const tapCanh = new Set<string>(); for (const k of ketQuaTungKichBan) for (const c of k.dsCanhDaDi) tapCanh.add(c); const tyLeEdgeCoverage = toanBoCanh.filter((c) => tapCanh.has(c)).length / toanBoCanh.length; const tongLuotGoi = ketQuaTungKichBan.reduce((tong, k) => tong + k.soLuotGoi, 0); return { ten: cauHinh.ten, tyLeHoanThanh, tyLeEdgeCoverage, tongLuotGoi }; Cho hai: return cacCauHinh.map((ch) => chayMotCauHinhDoThi(ch, dsKichBan, toanBoCanh));"
- kind: one-line
  body: "Sao chep dung hai khoi o phan Strategy vao dung vi tri tuong ung."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 8000
- tier: output
  match: contains
  expect: "\"ten\":\"toi-gian\""
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn cấu hình, BA trục đo — VÀ audit không đổi con số NÀO trong ba cột,
trong khi cycle-detection VÀ fan-out kéo edge coverage theo hai HƯỚNG
NGƯỢC nhau. Track còn đúng MỘT bài: ráp NGUYÊN VĂN mọi cơ chế của CẢ
q9.5a VÀ q9.5b thành một graph runner production-grade DUY NHẤT.
::::

::::reflect{#nghi-lai}
Bảng Ở bài này lộ ra một sự thật khó chịu nhưng QUAN TRỌNG: bật cycle
detection làm edge coverage GIẢM (từ `6/7` xuống `5/7`) — không phải vì
nó "tệ hơn", mà vì nó DỪNG SỚM trước khi kịp đi lại cạnh quay về. Ngược
lại, bật fan-out làm `tyLeHoanThanh` GIẢM (từ `0.8` xuống `0.6`) — không
phải vì nó "kém tin cậy hơn", mà vì nó BẮT ĐÚNG một lỗi mà cấu hình đơn
giản đã ÂM THẦM bỏ lỡ. Đây chính LÀ lý do MASTERPLAN đòi BA trục đo TÁCH
BIỆT thay vì MỘT con số duy nhất: gộp `tyLeHoanThanh`, `tyLeEdgeCoverage`
VÀ `tongLuotGoi` LÀM MỘT sẽ xoá mất đúng thông tin quan trọng nhất — chi
tiết VÌ SAO một cấu hình "tốt hơn" Ở CHỖ NÀO, VÀ phải ĐÁNH ĐỔI GÌ để đạt
được điều đó.
::::

::::checkpoint{mastery=0.9}
::::
