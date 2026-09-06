---
id: ky-nghe-ung-dung-ai.ky-thuat-do-thi.do-thi-la-node-va-edge-tuong-minh
title: "Đồ thị là node + edge tường minh — bước NÀY tự quyết định bước SAU"
summary: "Một Node là một bước xử lý mô phỏng: chay() trả về { trangThai: 'xong'|'loi'; tenNodeTiepTheo: string|null } — tên node KẾ TIẾP do CHÍNH node quyết định ở runtime, không phải một danh sách bước cố định trước như LOOP (T9.4). Một execution graph tối giản là DoThi = Record<tênNode, Node>; chayDoThi(doThi, tenNodeGoc) chạy tuần tự bắt đầu từ node gốc, đi theo tenNodeTiepTheo. Trên đồ thị tuyến tính 3 node (batDau→xuLy→ketThuc): chayDoThi(doThiTuyenTinh,'batDau') cho dsNodeDaDi=['batDau','xuLy','ketThuc'], thanhCong=true. chayTuNhieuNodeGoc chạy từ 3 node gốc khác nhau trên CÙNG đồ thị: từ 'batDau' đi qua đủ 3 node; từ 'xuLy' (bắt đầu GIỮA đồ thị) chỉ đi qua 2 node còn lại (['xuLy','ketThuc']); từ một tên node không tồn tại trả về ngay lỗi 'node_khong_ton_tai' với dsNodeDaDi rỗng ([]). Trên một đồ thị có node báo lỗi (doThiCoLoi: batDau→kiemTra, kiemTra báo lỗi): chayDoThi dừng NGAY tại kiemTra, trả về thanhCong=false, loi='loi_tai_node', dsNodeDaDi=['batDau','kiemTra'] — node gây lỗi VẪN được ghi nhận đã đi qua."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-do-thi
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [kna.do-thi-la-node-va-edge-tuong-minh]
requires: [kna.boss-vong-lap-production-grade]
concepts: [kna.do-thi-la-node-va-edge-tuong-minh]
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
T9.4 đóng lại ở 12/12: một vòng LẶP luôn có hình dạng TUYẾN TÍNH — bước
`k+1` LUÔN là "gọi lại chính bước vừa chạy". T9.5 dạy một hình dạng KHÁC
hẳn: một mạng NÚT (node) nơi bước SAU không cố định trước — CHÍNH node
vừa chạy xong quyết định node NÀO chạy tiếp, dựa trên kết quả của nó.
Đây LÀ đồ thị thực thi (execution graph) — nền tảng của cả track này.
::::

::::explain{#node_va_do_thi_toi_gian}
Một `Node` (viết tắt `NodeXuLy` cho rõ vai trò) LÀ một bước xử lý mô
phỏng — một hàm `chay()` KHÔNG nhận đối số, trả về `KetQuaNode`: MỘT
`trangThai` (`"xong"` hoặc `"loi"`) VÀ `tenNodeTiepTheo` — tên CHÍNH XÁC
của node kế tiếp cần chạy, HOẶC `null` nếu đây LÀ node CUỐI của nhánh
đang đi. Một "đồ thị" (execution graph) tối giản LÀ `DoThi = Record<tên
node, NodeXuLy>` — một bảng tra tên → node, KHÔNG có "bước kế tiếp" nào
được liệt kê SẴN Ở NGOÀI node:

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

const doThiTuyenTinh: DoThi = {
  batDau: taoNodeThanhCong("xuLy"),
  xuLy: taoNodeThanhCong("ketThuc"),
  ketThuc: taoNodeThanhCong(null),
};

console.log(JSON.stringify(doThiTuyenTinh.batDau!.chay()));
console.log(JSON.stringify(doThiTuyenTinh.xuLy!.chay()));
console.log(JSON.stringify(doThiTuyenTinh.ketThuc!.chay()));
```

```text title=readonly
{"trangThai":"xong","tenNodeTiepTheo":"xuLy"}
{"trangThai":"xong","tenNodeTiepTheo":"ketThuc"}
{"trangThai":"xong","tenNodeTiepTheo":null}
```

`batDau` tự báo node kế tiếp LÀ `"xuLy"` — không phải một chỉ số mảng,
một cạnh (edge) cố định nào được viết Ở NGOÀI node đó. `ketThuc` trả về
`tenNodeTiepTheo: null` — đây LÀ dấu hiệu DUY NHẤT báo "đã tới nút cuối
của nhánh này", khác hẳn LOOP (T9.4): LOOP dừng khi CHÍNH tác vụ báo
`"xong"`; GRAPH dừng khi node báo KHÔNG CÒN node kế tiếp nào để đi.
::::

::::example{#chay_do_thi_va_nhieu_diem_bat_dau}
`chayDoThi(doThi, tenNodeGoc)` chạy tuần tự: bắt đầu Ở `tenNodeGoc`, mỗi
vòng tra node theo tên trong `doThi`, gọi `chay()`, rồi ĐI THEO
`tenNodeTiepTheo` do CHÍNH node đó trả về — cho tới khi gặp `null`
(thành công) hoặc `"loi"` (thất bại dứt khoát). `chayTuNhieuNodeGoc`
chạy hàm đó từ NHIỀU điểm bắt đầu khác nhau trên CÙNG một đồ thị:

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

function chayTuNhieuNodeGoc(doThi: DoThi, dsTenNodeGoc: string[]): KetQuaChayDoThi[] {
  const ketQua: KetQuaChayDoThi[] = [];
  for (const tenGoc of dsTenNodeGoc) {
    ketQua.push(chayDoThi(doThi, tenGoc));
  }
  return ketQua;
}

const doThiTuyenTinh: DoThi = {
  batDau: taoNodeThanhCong("xuLy"),
  xuLy: taoNodeThanhCong("ketThuc"),
  ketThuc: taoNodeThanhCong(null),
};

const ketQuaBaGoc = chayTuNhieuNodeGoc(doThiTuyenTinh, ["batDau", "xuLy", "khongTonTai"]);
console.log(JSON.stringify(ketQuaBaGoc.map((k) => k.dsNodeDaDi)));

const doThiCoLoi: DoThi = {
  batDau: taoNodeThanhCong("kiemTra"),
  kiemTra: taoNodeLoi(),
};
console.log(JSON.stringify(chayDoThi(doThiCoLoi, "batDau")));
```

```text title=readonly
[["batDau","xuLy","ketThuc"],["xuLy","ketThuc"],[]]
{"thanhCong":false,"loi":"loi_tai_node","dsNodeDaDi":["batDau","kiemTra"]}
```

CÙNG một `doThiTuyenTinh`, BA điểm bắt đầu khác nhau cho BA kết quả khác
nhau: từ `"batDau"` đi qua ĐỦ `3` node; từ `"xuLy"` (bắt đầu GIỮA đồ thị)
chỉ đi qua `2` node CÒN LẠI; từ một tên KHÔNG tồn tại trong `doThi`, trả
về NGAY lỗi `"node_khong_ton_tai"` VỚI `dsNodeDaDi` rỗng — chưa từng bước
vào node nào. Trên `doThiCoLoi`, node `kiemTra` báo lỗi — `chayDoThi`
dừng NGAY tại đó: `dsNodeDaDi` VẪN ghi nhận `["batDau","kiemTra"]` (node
gây lỗi ĐÃ được đi qua trước khi thất bại), KHÔNG rỗng.
::::

::::predict{#doan-them-node-giua-tuyen commitOnce}
Nếu chèn THÊM một node mới `"kiemTraGiua"` VÀO GIỮA `"xuLy"` VÀ
`"ketThuc"` (đổi `xuLy: taoNodeThanhCong("kiemTraGiua")`, thêm
`kiemTraGiua: taoNodeThanhCong("ketThuc")`, giữ NGUYÊN mọi node khác) —
`chayDoThi(doThiTuyenTinh, "batDau")` có cần SỬA GÌ Ở hàm `chayDoThi`
để đi qua node mới đó không?

:::opt{correct}
Không — `chayDoThi` KHÔNG hề biết trước danh sách node nào tồn tại; nó
CHỈ đọc `tenNodeTiepTheo` do CHÍNH node vừa chạy trả về Ở MỖI bước, nên
thêm một node mới VÀO GIỮA chuỗi cạnh chỉ cần SỬA đúng ĐỊNH NGHĨA
`doThiTuyenTinh` (dữ liệu), không cần đụng vào logic chạy đồ thị
:::
:::opt
Có — phải thêm một dòng kiểm tra `tenHienTai === "kiemTraGiua"` VÀO
`chayDoThi` để hàm "biết" node mới này tồn tại
::why
Nhầm rằng `chayDoThi` cần biết TRƯỚC tên từng node cụ thể — nhưng hàm
này hoàn toàn TỔNG QUÁT: nó tra `doThi[tenHienTai]` bằng tên bất kỳ, và
CHỈ dựa vào `tenNodeTiepTheo` do node đó tự trả về để đi tiếp.

Chỗ lệch: `chayDoThi` không hề chứa một danh sách tên node CỐ ĐỊNH nào
Ở BÊN TRONG thân hàm — mọi thông tin về "node nào tồn tại, cạnh nào nối
tới đâu" nằm HOÀN TOÀN trong tham số `doThi` (dữ liệu), không nằm trong
code của hàm.
::
:::
:::opt
Có — phải tăng `SO_BUOC_TOI_DA_AN_TOAN` lên vì đồ thị giờ có nhiều node
hơn
::why
Nhầm rằng thêm MỘT node sẽ vượt giới hạn an toàn — nhưng
`SO_BUOC_TOI_DA_AN_TOAN = 20` LÀ một lưới an toàn RỘNG RÃI so với một
đồ thị chỉ có VÀI node tuyến tính; thêm một node khiến đường đi dài
thêm ĐÚNG `1` bước, còn rất xa mới chạm `20`.

Chỗ lệch: giới hạn an toàn chỉ cần LỚN HƠN số node có thể đi qua trong
MỘT lần chạy — thêm một node vào một chuỗi `3`-`4` node không hề đe doạ
một giới hạn LÀ `20`.
::
:::
::::

::::code{#viet_chay_do_thi}
Hoàn thiện `chayDoThi` — lặp `while` (tối đa `SO_BUOC_TOI_DA_AN_TOAN`
bước AN TOÀN), bắt đầu từ `tenNodeGoc`; MỖI vòng: tra node theo tên
TRONG `doThi`, NẾU không tồn tại thì trả `{ thanhCong: false, loi:
"node_khong_ton_tai", dsNodeDaDi }` NGAY; nếu có, GHI node đó VÀO
`dsNodeDaDi`, gọi `chay()`; NẾU `trangThai` LÀ `"loi"`, trả `{
thanhCong: false, loi: "loi_tai_node", dsNodeDaDi }` NGAY; NGƯỢC LẠI,
đi tới `tenNodeTiepTheo` (có thể LÀ `null`, khiến vòng lặp dừng); hết
vòng lặp mà chưa `return` thì trả `{ thanhCong: true, dsNodeDaDi }`.
Hoàn thiện `chayTuNhieuNodeGoc` — với MỖI tên trong `dsTenNodeGoc`, gọi
`chayDoThi(doThi, tenGoc)`, đẩy kết quả vào mảng trả về.

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

type KetQuaChayDoThi =
  | { thanhCong: true; dsNodeDaDi: string[] }
  | { thanhCong: false; loi: string; dsNodeDaDi: string[] };

const SO_BUOC_TOI_DA_AN_TOAN = 20;

function chayDoThi(doThi: DoThi, tenNodeGoc: string): KetQuaChayDoThi {
  ___
}

function chayTuNhieuNodeGoc(doThi: DoThi, dsTenNodeGoc: string[]): KetQuaChayDoThi[] {
  ___
}

const doThiTuyenTinh: DoThi = {
  batDau: taoNodeThanhCong("xuLy"),
  xuLy: taoNodeThanhCong("ketThuc"),
  ketThuc: taoNodeThanhCong(null),
};

const ketQuaBaGoc = chayTuNhieuNodeGoc(doThiTuyenTinh, ["batDau", "xuLy", "khongTonTai"]);
console.log(JSON.stringify(ketQuaBaGoc.map((k) => k.dsNodeDaDi)));
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

function chayTuNhieuNodeGoc(doThi: DoThi, dsTenNodeGoc: string[]): KetQuaChayDoThi[] {
  const ketQua: KetQuaChayDoThi[] = [];
  for (const tenGoc of dsTenNodeGoc) {
    ketQua.push(chayDoThi(doThi, tenGoc));
  }
  return ketQua;
}

const doThiTuyenTinh: DoThi = {
  batDau: taoNodeThanhCong("xuLy"),
  xuLy: taoNodeThanhCong("ketThuc"),
  ketThuc: taoNodeThanhCong(null),
};

const ketQuaBaGoc = chayTuNhieuNodeGoc(doThiTuyenTinh, ["batDau", "xuLy", "khongTonTai"]);
console.log(JSON.stringify(ketQuaBaGoc.map((k) => k.dsNodeDaDi)));
```

```typescript title=test
if (ketQuaBaGoc.length !== 3) throw new Error("chayTuNhieuNodeGoc phai tra ve mang dung 3 phan tu");
if (JSON.stringify(ketQuaBaGoc.map((k) => k.dsNodeDaDi)) !== JSON.stringify([["batDau", "xuLy", "ketThuc"], ["xuLy", "ketThuc"], []])) {
  throw new Error("chay tu 3 node goc khac nhau phai cho dsNodeDaDi dung nhu tren");
}
if (ketQuaBaGoc[0]!.thanhCong !== true) throw new Error("chay tu batDau phai thanh cong");
if (ketQuaBaGoc[1]!.thanhCong !== true) throw new Error("chay tu xuLy (giua do thi) phai thanh cong");
if (ketQuaBaGoc[2]!.thanhCong !== false) throw new Error("chay tu node khong ton tai phai that bai");
if (ketQuaBaGoc[2]!.thanhCong === false && ketQuaBaGoc[2]!.loi !== "node_khong_ton_tai") {
  throw new Error("loi phai la node_khong_ton_tai");
}

const doThiCoLoi: DoThi = {
  batDau: taoNodeThanhCong("kiemTra"),
  kiemTra: taoNodeLoi(),
};
const ketQuaLoi = chayDoThi(doThiCoLoi, "batDau");
if (ketQuaLoi.thanhCong !== false) throw new Error("do thi co node bao loi phai that bai");
if (ketQuaLoi.thanhCong === false && ketQuaLoi.loi !== "loi_tai_node") throw new Error("loi phai la loi_tai_node");
if (JSON.stringify(ketQuaLoi.dsNodeDaDi) !== JSON.stringify(["batDau", "kiemTra"])) {
  throw new Error("node gay loi VAN phai duoc ghi nhan da di qua trong dsNodeDaDi");
}

const ketQuaMotGoc = chayTuNhieuNodeGoc(doThiTuyenTinh, ["ketThuc"]);
if (ketQuaMotGoc.length !== 1) throw new Error("doi so danh sach node goc phai doi do dai mang tra ve -- tham so phai duoc dung that");
if (JSON.stringify(ketQuaMotGoc[0]!.dsNodeDaDi) !== JSON.stringify(["ketThuc"])) {
  throw new Error("chay tu ketThuc (node cuoi) chi di qua dung 1 node roi dung vi tenNodeTiepTheo la null");
}
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (chayDoThi): mot vong while chay toi da SO_BUOC_TOI_DA_AN_TOAN buoc, bat dau tenHienTai = tenNodeGoc; moi vong tra doThi[tenHienTai] (nho khai bao kieu NodeXuLy | undefined), neu khong ton tai thi return loi node_khong_ton_tai NGAY; neu co thi day ten node vao dsNodeDaDi, goi chay(), neu trangThai la loi thi return loi_tai_node NGAY, nguoc lai gan tenHienTai = tenNodeTiepTheo (co the la null, khien while dung); het vong lap tra thanhCong true. Cho hai (chayTuNhieuNodeGoc): mot vong for-of qua dsTenNodeGoc, goi chayDoThi(doThi, tenGoc) tren tung ten, day ket qua vao mot mang."
- kind: strategy
  body: "Cho dau: const dsNodeDaDi: string[] = []; let tenHienTai: string | null = tenNodeGoc; let soBuoc = 0; while (tenHienTai !== null && soBuoc < SO_BUOC_TOI_DA_AN_TOAN) { soBuoc++; const node: NodeXuLy | undefined = doThi[tenHienTai]; if (!node) return { thanhCong: false, loi: \"node_khong_ton_tai\", dsNodeDaDi }; dsNodeDaDi.push(tenHienTai); const kq: KetQuaNode = node.chay(); if (kq.trangThai === \"loi\") return { thanhCong: false, loi: \"loi_tai_node\", dsNodeDaDi }; tenHienTai = kq.tenNodeTiepTheo; } return { thanhCong: true, dsNodeDaDi }; Cho hai: const ketQua: KetQuaChayDoThi[] = []; for (const tenGoc of dsTenNodeGoc) { ketQua.push(chayDoThi(doThi, tenGoc)); } return ketQua;"
- kind: one-line
  body: "Sao chep dung hai khoi o phan Strategy vao dung vi tri tuong ung -- CHU Y khai bao kieu tuong minh cho node va kq (xem gotcha o phan Reflect)."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "[[\"batDau\",\"xuLy\",\"ketThuc\"],[\"xuLy\",\"ketThuc\"],[]]"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cùng một đồ thị, ba điểm bắt đầu, ba kết quả khác nhau — VÀ node gây lỗi
vẫn được ghi nhận đã đi qua. Đồ thị KHÔNG có "bước kế tiếp" cố định
trước như LOOP — CHÍNH node quyết định. Bài sau khai thác đúng điều đó:
để MỘT node trả về HAI (hoặc nhiều) tên node kế tiếp khác nhau, tuỳ dữ
liệu nó xử lý.
::::

::::reflect{#nghi-lai}
Bài này KHÔNG dạy một thuật toán mới — `while` lặp theo một con trỏ tên
node LÀ cấu trúc quen thuộc từ LOOP. Điều MỚI nằm Ở CHỖ thông tin "bước
nào tiếp theo" tới từ ĐÂU: Ở LOOP (T9.4), số bước VÀ điều kiện dừng do
HARNESS BÊN NGOÀI quyết định (`soBuocToiDa`, điều kiện `while`); Ở GRAPH,
CHÍNH node — kết quả của bước VỪA chạy — quyết định bước nào chạy tiếp.
Đây LÀ khác biệt cốt lõi giữa DÒNG THỜI GIAN (LOOP, một trạng thái, một
hướng) VÀ KHÔNG GIAN TRẠNG THÁI (GRAPH, một mạng node, đường đi do runtime
quyết định). Một gotcha TypeScript đáng chú ý xuất hiện Ở CHÍNH bài này:
khi một biến vòng lặp (`tenHienTai`) được TRA CỨU để lấy giá trị TIẾP
THEO của CHÍNH nó (qua `doThi[tenHienTai]` rồi `kq.tenNodeTiepTheo`),
trình biên dịch có thể báo `TS7022` ("implicitly has type 'any' ... is
referenced ... in its own initializer") nếu `node` VÀ `kq` không được
khai kiểu TƯỜNG MINH — đây LÀ một vòng suy luận kiểu (type-inference
cycle) THẬT, không phải lỗi cú pháp; khai rõ `NodeXuLy | undefined` VÀ
`KetQuaNode` giải quyết được NGAY.
::::

::::checkpoint{mastery=0.85}
::::
