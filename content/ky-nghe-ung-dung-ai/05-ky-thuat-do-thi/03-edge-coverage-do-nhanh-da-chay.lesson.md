---
id: ky-nghe-ung-dung-ai.ky-thuat-do-thi.edge-coverage-do-nhanh-da-chay
title: "Edge coverage — đo nhánh nào ĐÃ chạy, nhánh nào CHƯA"
summary: "Trên một đồ thị có 4 cạnh CÓ THỂ (liệt kê tường minh trong TOAN_BO_CANH_CO_THE: kiemTra->xuLyHopLe, kiemTra->xuLyLoi, xuLyLoi->thuLai, xuLyLoi->boCuoc), MỘT bộ 4 kịch bản input (giaTri lần lượt 5/-3/-1/10, coTheThuLai=true cho cả 4) đạt tyLeHoanThanh=1 (4/4 thanhCong, 100% completion rate) NHƯNG tyLeEdgeCoverage chỉ 3/4 (in ra 0.75) — cạnh 'xuLyLoi->boCuoc' CHƯA TỪNG được đi qua, vì không kịch bản nào có coTheThuLai=false. layCacCanhDaDi ghép từng cặp node LIÊN TIẾP trong một đường đi thành chuỗi 'tu->den'; tinhTapCanhDaDi gộp tập hợp cạnh của MỌI kịch bản; tinhTyLeEdgeCoverage = số cạnh trong tập đã đi qua / tổng số cạnh có thể = số phần tử giao nhau chia tổng cạnh khai báo. Thêm MỘT kịch bản thứ 5 (giaTri=-1, coTheThuLai=false) nâng edge coverage lên ĐÚNG 4/4=1 (100%) — xác nhận %edge coverage đo được ĐÚNG nhánh nào đã/chưa chạm tới, một con số HOÀN TOÀN tách biệt khỏi completion rate (chỉ đo 'có xong hay không', không đo 'đã thử hết các đường chưa')."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-do-thi
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [kna.edge-coverage-do-nhanh-da-chay]
requires: [kna.re-nhanh-co-dieu-kien]
concepts: [kna.edge-coverage-do-nhanh-da-chay]
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
Bài trước cho thấy hai lần chạy với input khác nhau có thể đi qua tập
node hoàn toàn khác nhau. Câu hỏi tự nhiên tiếp theo: nếu chạy MỘT BỘ
kịch bản input, liệu TẤT CẢ cạnh CÓ THỂ có trong đồ thị đã từng được đi
qua chưa? MASTERPLAN gọi con số đó LÀ **edge coverage** — VÀ nó có thể
THẤP ngay cả khi MỌI kịch bản đều chạy THÀNH CÔNG.
::::

::::explain{#canh_la_gi_va_tap_hop_canh_da_di}
Một "cạnh" (edge) LÀ một cặp `[tenNodeTu, tenNodeDen]` — mã hoá gọn
thành chuỗi `"tu->den"` để dùng LÀM khoá trong `Set`. `layCacCanhDaDi`
ghép TỪNG cặp node LIÊN TIẾP trong MỘT đường đi (`dsNodeDaDi`, đã học
bài `1`) thành danh sách cạnh; `tinhTapCanhDaDi` gộp cạnh của NHIỀU
đường đi (từ NHIỀU kịch bản) VÀO một `Set` DUY NHẤT — tập hợp mọi cạnh
ĐÃ từng được đi qua, không phân biệt kịch bản nào tạo ra nó:

```typescript title=readonly
function layCacCanhDaDi(dsNodeDaDi: string[]): string[] {
  const canh: string[] = [];
  for (let i = 0; i < dsNodeDaDi.length - 1; i++) {
    canh.push(`${dsNodeDaDi[i]}->${dsNodeDaDi[i + 1]}`);
  }
  return canh;
}

console.log(JSON.stringify(layCacCanhDaDi(["kiemTra", "xuLyLoi", "thuLai"])));
console.log(JSON.stringify(layCacCanhDaDi(["kiemTra", "xuLyHopLe"])));
```

```text title=readonly
["kiemTra->xuLyLoi","xuLyLoi->thuLai"]
["kiemTra->xuLyHopLe"]
```

Một đường đi qua `3` node sinh ra ĐÚNG `2` cạnh (số cạnh LUÔN kém số node
`1`, vì mỗi cạnh nối HAI node liên tiếp). Đây LÀ ĐƠN VỊ đo của edge
coverage — KHÔNG phải "node nào đã ghé qua" (đã học bài `1`), mà LÀ
"cặp node LIÊN TIẾP nào đã thực sự nối tiếp nhau".
::::

::::example{#bon_kich_ban_ba_phan_tu_canh}
Một đồ thị đầy đủ hơn: `"kiemTra"` rẽ nhánh `"xuLyHopLe"` hoặc
`"xuLyLoi"` (bài `2`); `"xuLyLoi"` rẽ nhánh TIẾP `"thuLai"` hoặc
`"boCuoc"` tuỳ `coTheThuLai`. Đồ thị này có ĐÚNG `4` cạnh CÓ THỂ, liệt kê
tường minh Ở `TOAN_BO_CANH_CO_THE`. Chạy MỘT bộ `4` kịch bản — TẤT CẢ
đều có `coTheThuLai: true`:

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

type KetQuaChayDoThi =
  | { thanhCong: true; dsNodeDaDi: string[] }
  | { thanhCong: false; loi: string; dsNodeDaDi: string[] };

const SO_BUOC_TOI_DA_AN_TOAN = 20;

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

function taoDoThiDayDu(giaTri: number, coTheThuLai: boolean): DoThi {
  return {
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
  "kiemTra->xuLyHopLe",
  "kiemTra->xuLyLoi",
  "xuLyLoi->thuLai",
  "xuLyLoi->boCuoc",
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
    ketQua.push(chayDoThi(taoDoThiDayDu(kb.giaTri, kb.coTheThuLai), "kiemTra"));
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
console.log("hoan thanh:", tinhTyLeHoanThanh(ketQuaBoKichBan));
console.log("edge coverage:", tinhTyLeEdgeCoverage(tapCanh, TOAN_BO_CANH_CO_THE));
```

```text title=readonly
hoan thanh: 1
edge coverage: 0.75
```

Cả `4` kịch bản đều `thanhCong` — completion rate `1` (100%). NHƯNG edge
coverage chỉ `0.75` (`3/4`): cạnh `"xuLyLoi->boCuoc"` CHƯA TỪNG được đi
qua, vì KHÔNG kịch bản nào Ở bộ này có `coTheThuLai: false`. Completion
rate CAO không hề đảm bảo MỌI nhánh đã được thử — đây chính LÀ khoảng
cách mà edge coverage đo được VÀ completion rate thì KHÔNG.
::::

::::predict{#doan-them-kich-ban-con-thieu commitOnce}
Nếu thêm MỘT kịch bản thứ `5` VÀO bộ trên VỚI `giaTri: -1, coTheThuLai:
false` (giữ NGUYÊN `4` kịch bản cũ) — `tyLeEdgeCoverage` đổi ra sao?

:::opt{correct}
Tăng lên ĐÚNG `1` (100%) — kịch bản mới đi qua đường
`"kiemTra"→"xuLyLoi"→"boCuoc"`, CHẠM đúng cạnh `"xuLyLoi->boCuoc"` còn
thiếu; giờ CẢ `4` cạnh trong `TOAN_BO_CANH_CO_THE` đều nằm trong tập
cạnh đã đi qua
:::
:::opt
Không đổi — thêm MỘT kịch bản không ảnh hưởng tới TẬP cạnh đã tích luỹ
từ `4` kịch bản trước đó
::why
Nhầm rằng `tinhTapCanhDaDi` chỉ tính riêng TỪNG bộ kịch bản, không GỘP
dồn — nhưng hàm này nhận NGUYÊN mảng `dsKetQua` (bao gồm CẢ `5` kịch
bản MỚI), rồi hợp (union) cạnh của TẤT CẢ chúng VÀO một `Set` DUY NHẤT.

Chỗ lệch: `tinhTapCanhDaDi` không hề "nhớ" tập cạnh CŨ từ lần gọi trước
— mỗi lần gọi nhận một mảng kết quả MỚI VÀ tính tập cạnh TỪ ĐẦU trên
CHÍNH mảng đó; thêm một kịch bản vào mảng ĐẦU VÀO chắc chắn có thể thêm
cạnh MỚI vào tập kết quả.
::
:::
:::opt
Giảm xuống, vì `TOAN_BO_CANH_CO_THE` phải LIỆT KÊ THÊM cạnh mới khi có
kịch bản mới
::why
Nhầm rằng danh sách cạnh CÓ THỂ (`TOAN_BO_CANH_CO_THE`) thay đổi theo
KỊCH BẢN — nhưng đây LÀ danh sách cạnh CỐ ĐỊNH, mô tả CẤU TRÚC của đồ
thị (bao nhiêu cạnh CÓ THỂ tồn tại), hoàn toàn ĐỘC LẬP VỚI việc kịch bản
nào được chạy.

Chỗ lệch: cạnh `"xuLyLoi->boCuoc"` LUÔN nằm trong `TOAN_BO_CANH_CO_THE`
kể cả TRƯỚC khi kịch bản thứ `5` được thêm vào — vấn đề Ở bộ `4` kịch
bản cũ LÀ KHÔNG kịch bản nào CHẠM tới cạnh đó, không phải danh sách cạnh
thiếu sót.
::
:::
::::

::::code{#viet_do_edge_coverage}
Hoàn thiện `layCacCanhDaDi` — với MỖI cặp chỉ số liên tiếp `i` VÀ `i+1`
trong `dsNodeDaDi`, tạo chuỗi `` `${dsNodeDaDi[i]}->${dsNodeDaDi[i +
1]}` ``, đẩy vào mảng trả về (mảng CHỈ `1` node cho ra mảng RỖNG). Hoàn
thiện `tinhTyLeEdgeCoverage` — đếm bao nhiêu phần tử trong `toanBoCanh`
CÓ mặt trong `canhDaDi` (dùng `.has`), chia cho TỔNG số phần tử của
`toanBoCanh` (trả `0` nếu `toanBoCanh` rỗng).

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

function taoDoThiDayDu(giaTri: number, coTheThuLai: boolean): DoThi {
  return {
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
  "kiemTra->xuLyHopLe",
  "kiemTra->xuLyLoi",
  "xuLyLoi->thuLai",
  "xuLyLoi->boCuoc",
];

function layCacCanhDaDi(dsNodeDaDi: string[]): string[] {
  ___
}

function tinhTapCanhDaDi(dsKetQua: KetQuaChayDoThi[]): Set<string> {
  const tap = new Set<string>();
  for (const kq of dsKetQua) {
    for (const canh of layCacCanhDaDi(kq.dsNodeDaDi)) tap.add(canh);
  }
  return tap;
}

function tinhTyLeEdgeCoverage(canhDaDi: Set<string>, toanBoCanh: string[]): number {
  ___
}

function tinhTyLeHoanThanh(dsKetQua: KetQuaChayDoThi[]): number {
  if (dsKetQua.length === 0) return 0;
  return dsKetQua.filter((k) => k.thanhCong).length / dsKetQua.length;
}

type KichBan = { giaTri: number; coTheThuLai: boolean };

function chayBoKichBan(dsKichBan: KichBan[]): KetQuaChayDoThi[] {
  const ketQua: KetQuaChayDoThi[] = [];
  for (const kb of dsKichBan) {
    ketQua.push(chayDoThi(taoDoThiDayDu(kb.giaTri, kb.coTheThuLai), "kiemTra"));
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
console.log(tyLeHoanThanh, tyLeEdgeCoverage);
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

function taoDoThiDayDu(giaTri: number, coTheThuLai: boolean): DoThi {
  return {
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
  "kiemTra->xuLyHopLe",
  "kiemTra->xuLyLoi",
  "xuLyLoi->thuLai",
  "xuLyLoi->boCuoc",
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
    ketQua.push(chayDoThi(taoDoThiDayDu(kb.giaTri, kb.coTheThuLai), "kiemTra"));
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
console.log(tyLeHoanThanh, tyLeEdgeCoverage);
```

```typescript title=test
if (Math.abs(tyLeHoanThanh - 1) > 1e-9) throw new Error("ty le hoan thanh tren bo kich ban phai la 1 (100%)");
if (Math.abs(tyLeEdgeCoverage - 0.75) > 1e-9) throw new Error("edge coverage phai la 0.75 (3/4) -- canh xuLyLoi->boCuoc chua tung duoc di qua");

if (JSON.stringify([...tapCanh].sort()) !== JSON.stringify(["kiemTra->xuLyHopLe", "kiemTra->xuLyLoi", "xuLyLoi->thuLai"])) {
  throw new Error("tap canh da di phai dung 3 canh, KHONG bao gom xuLyLoi->boCuoc");
}

if (JSON.stringify(layCacCanhDaDi(["a", "b", "c"])) !== JSON.stringify(["a->b", "b->c"])) {
  throw new Error("layCacCanhDaDi phai ghep tung cap node LIEN TIEP thanh chuoi 'tu->den'");
}
if (JSON.stringify(layCacCanhDaDi(["a"])) !== JSON.stringify([])) {
  throw new Error("mot duong di CHI 1 node khong co canh nao (mang rong)");
}

const kichBanDayDu: KichBan[] = [
  { giaTri: 5, coTheThuLai: true },
  { giaTri: -3, coTheThuLai: true },
  { giaTri: -1, coTheThuLai: true },
  { giaTri: 10, coTheThuLai: true },
  { giaTri: -1, coTheThuLai: false },
];
const ketQuaDayDu = chayBoKichBan(kichBanDayDu);
const tapCanhDayDu = tinhTapCanhDaDi(ketQuaDayDu);
const tyLeDayDu = tinhTyLeEdgeCoverage(tapCanhDayDu, TOAN_BO_CANH_CO_THE);
if (Math.abs(tyLeDayDu - 1) > 1e-9) throw new Error("them kich ban coTheThuLai=false phai dat DUNG 100% edge coverage (4/4)");

const chiMotCanh = tinhTyLeEdgeCoverage(new Set(["kiemTra->xuLyHopLe"]), TOAN_BO_CANH_CO_THE);
if (Math.abs(chiMotCanh - 0.25) > 1e-9) throw new Error("tap chi co 1 canh trong tong so 4 canh phai la 0.25");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (layCacCanhDaDi): mot vong for voi i chay tu 0 den dsNodeDaDi.length - 2 (dieu kien i < dsNodeDaDi.length - 1), moi vong day chuoi `${dsNodeDaDi[i]}->${dsNodeDaDi[i + 1]}` vao mang canh, cuoi cung return mang do. Cho hai (tinhTyLeEdgeCoverage): loc toanBoCanh chi giu nhung phan tu canhDaDi.has(...) tra true, dem so luong, chia cho toanBoCanh.length (tra 0 neu toanBoCanh rong)."
- kind: strategy
  body: "Cho dau: const canh: string[] = []; for (let i = 0; i < dsNodeDaDi.length - 1; i++) { canh.push(`${dsNodeDaDi[i]}->${dsNodeDaDi[i + 1]}`); } return canh; Cho hai: if (toanBoCanh.length === 0) return 0; const soCanhDaDi = toanBoCanh.filter((c) => canhDaDi.has(c)).length; return soCanhDaDi / toanBoCanh.length;"
- kind: one-line
  body: "Sao chep dung hai khoi o phan Strategy vao dung vi tri tuong ung."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "1 0.75"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
100% completion rate, chỉ `75%` edge coverage — hai con số ĐO hai thứ
khác nhau, VÀ một bộ kịch bản có thể "trông ổn" (mọi kịch bản thành
công) trong khi vẫn CHƯA từng thử một nhánh hiếm gặp. Bài sau xử lý một
vấn đề khác hẳn: LÀM SAO để không phải chạy LẠI từ đầu mỗi khi một node
giữa chừng gặp sự cố.
::::

::::reflect{#nghi-lai}
Edge coverage KHÔNG thay thế completion rate — nó ĐO một CHIỀU khác của
CÙNG một hệ thống. Completion rate trả lời "input NÀY có xử lý xong
không"; edge coverage trả lời "bộ kịch bản NÀY đã từng CHẠM tới cạnh
NÀO trong TOÀN BỘ cấu trúc đồ thị". Một hệ thống có thể đạt `100%`
Ở trục ĐẦU (mọi input được xử lý) NHƯNG rất thấp Ở trục SAU (nhiều nhánh
lỗi/hiếm chưa từng được thử) — VÀ chỉ trục sau mới lộ ra rủi ro "nhánh
NÀY chưa ai kiểm tra bao giờ". Đây LÀ lý do MASTERPLAN đặt "edge
coverage" LÀM một trong hai con số đo CHÍNH của GRAPH, tách biệt hoàn
toàn khỏi completion rate mà LOOP (T9.4) đã dùng.
::::

::::checkpoint{mastery=0.88}
::::
