---
id: ky-nghe-ung-dung-ai.ky-thuat-do-thi.phat-hien-chu-trinh
title: "Phát hiện chu trình — đồ thị không phải luôn là cây"
summary: "taoDoThiCoChuTrinh() dựng đồ thị kiemTra->xuLyLoi->thuLai->kiemTra — cạnh CUỐI quay LẠI node ĐÃ đi qua, không phải một lá. chayDoThiCoTranAnToan (dùng SO_BUOC_TOI_DA_AN_TOAN=20 làm lưới an toàn thô, KHÔNG phát hiện chu trình) chạy đúng 20 lượt gọi rồi dừng với trangThai='hetBuoc' — không hề biết đây LÀ một chu trình hay chỉ một đồ thị dài. chayDoThiCoPhatHienChuTrinh thêm một Set các node ĐÃ ghé qua TRONG LẦN CHẠY NÀY: dừng NGAY khi gặp lại một node đã có trong Set, trả về trangThai='phat_hien_chu_trinh' kèm tenNodeLap='kiemTra' và dsNodeDaDi=['kiemTra','xuLyLoi','thuLai'] — CHỈ 3 lượt gọi, tiết kiệm đúng 17 lượt (20-3) so với chạy tới hết lưới an toàn. Đổi cạnh quay lại thành 'thuLai->xuLyLoi' thì tenNodeLap đổi thành 'xuLyLoi' nhưng tổng lượt gọi trước phát hiện KHÔNG đổi (vẫn 3, vì chu trình vẫn dài 3 node). Trên đồ thị KHÔNG có chu trình (batDau->xuLy->ketThuc->null), soSanhChuTrinh cho soLuotGoiTietKiem=0 — cả hai cách cho ra CÙNG kết quả, chỉ khác khi THẬT SỰ có chu trình."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-do-thi
order: 10
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [kna.phat-hien-chu-trinh]
requires: [kna.test-mot-nhanh-rieng-le]
concepts: [kna.phat-hien-chu-trinh]
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
Chín bài trước NGẦM giả định một điều: đường đi LUÔN đi TỚI — từ gốc,
qua các nhánh, rồi dừng Ở một lá. Nhưng KHÔNG GÌ trong `KetQuaNode` ép
buộc điều đó — một node HOÀN TOÀN có thể trả về `tenNodeTiepTheo` LÀ tên
một node ĐÃ đi qua RỒI. Đồ thị THẬT không phải LUÔN LÀ một cây; đôi khi
nó LÀ một chu trình — VÀ `SO_BUOC_TOI_DA_AN_TOAN` (bài `1`) chỉ LÀ một
lưới an toàn THÔ, không phải một cách PHÁT HIỆN.
::::

::::explain{#luoi_an_toan_tho_khong_biet_dang_lap}
`taoDoThiCoChuTrinh()` dựng MỘT đồ thị mà cạnh CUỐI quay LẠI node ĐẦU:
`kiemTra → xuLyLoi → thuLai → kiemTra` — KHÔNG có lá nào, KHÔNG BAO GIỜ
tự dừng. `chayDoThiCoTranAnToan` LÀ `chayDoThi` (bài `1`) được viết lại
để BÁO RÕ khi nó dừng VÌ CHẠM lưới an toàn (`trangThai: "hetBuoc"`) thay
vì lặng lẽ trả `thanhCong: true` như bản gốc:

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

const SO_BUOC_TOI_DA_AN_TOAN = 20;

type KetQuaChayDoThiMoRong =
  | { trangThai: "thanhCong"; dsNodeDaDi: string[] }
  | { trangThai: "loi"; loi: string; dsNodeDaDi: string[] }
  | { trangThai: "hetBuoc"; dsNodeDaDi: string[] };

function chayDoThiCoTranAnToan(doThi: DoThi, tenNodeGoc: string): KetQuaChayDoThiMoRong {
  const dsNodeDaDi: string[] = [];
  let tenHienTai: string | null = tenNodeGoc;
  let soBuoc = 0;
  while (tenHienTai !== null) {
    if (soBuoc >= SO_BUOC_TOI_DA_AN_TOAN) return { trangThai: "hetBuoc", dsNodeDaDi };
    soBuoc++;
    const node: NodeXuLy | undefined = doThi[tenHienTai];
    if (!node) return { trangThai: "loi", loi: "node_khong_ton_tai", dsNodeDaDi };
    dsNodeDaDi.push(tenHienTai);
    const kq: KetQuaNode = node.chay();
    if (kq.trangThai === "loi") return { trangThai: "loi", loi: "loi_tai_node", dsNodeDaDi };
    tenHienTai = kq.tenNodeTiepTheo;
  }
  return { trangThai: "thanhCong", dsNodeDaDi };
}

function taoDoThiCoChuTrinh(): DoThi {
  return {
    kiemTra: taoNodeThanhCong("xuLyLoi"),
    xuLyLoi: taoNodeThanhCong("thuLai"),
    thuLai: taoNodeThanhCong("kiemTra"),
  };
}

const ketQuaTranAnToan = chayDoThiCoTranAnToan(taoDoThiCoChuTrinh(), "kiemTra");
console.log(ketQuaTranAnToan.trangThai, ketQuaTranAnToan.dsNodeDaDi.length);
```

```text title=readonly
hetBuoc 20
```

`chayDoThiCoTranAnToan` chạy ĐÚNG `20` lượt gọi (đúng
`SO_BUOC_TOI_DA_AN_TOAN`) rồi dừng VỚI `trangThai: "hetBuoc"` — nó KHÔNG
hề "biết" đây LÀ một chu trình `3` node lặp `6`-`7` lần; nó chỉ biết "đã
chạm giới hạn AN TOÀN". Một đồ thị THẬT SỰ dài `20` node (không chu
trình) sẽ cho ra CÙNG `trangThai: "hetBuoc"` NHƯ VẬY — lưới an toàn
KHÔNG phân biệt được hai tình huống đó.
::::

::::example{#tap_da_ghe_qua_dung_som_va_tiet_kiem}
`chayDoThiCoPhatHienChuTrinh` thêm ĐÚNG một cấu trúc: `daGhePhamVi = new
Set<string>()`, ghi lại node NÀO đã ghé qua TRONG LẦN CHẠY NÀY — VÀ dừng
NGAY khi gặp LẠI một node ĐÃ có trong `Set` đó, TRƯỚC KHI gọi `chay()`
của nó lần THỨ HAI:

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

const SO_BUOC_TOI_DA_AN_TOAN = 20;

type KetQuaChayDoThiCoPhatHienChuTrinh =
  | { trangThai: "thanhCong"; dsNodeDaDi: string[] }
  | { trangThai: "loi"; loi: string; dsNodeDaDi: string[] }
  | { trangThai: "hetBuoc"; dsNodeDaDi: string[] }
  | { trangThai: "phat_hien_chu_trinh"; tenNodeLap: string; dsNodeDaDi: string[] };

function chayDoThiCoPhatHienChuTrinh(doThi: DoThi, tenNodeGoc: string): KetQuaChayDoThiCoPhatHienChuTrinh {
  const dsNodeDaDi: string[] = [];
  const daGhePhamVi = new Set<string>();
  let tenHienTai: string | null = tenNodeGoc;
  let soBuoc = 0;
  while (tenHienTai !== null) {
    if (soBuoc >= SO_BUOC_TOI_DA_AN_TOAN) return { trangThai: "hetBuoc", dsNodeDaDi };
    if (daGhePhamVi.has(tenHienTai)) return { trangThai: "phat_hien_chu_trinh", tenNodeLap: tenHienTai, dsNodeDaDi };
    soBuoc++;
    const node: NodeXuLy | undefined = doThi[tenHienTai];
    if (!node) return { trangThai: "loi", loi: "node_khong_ton_tai", dsNodeDaDi };
    daGhePhamVi.add(tenHienTai);
    dsNodeDaDi.push(tenHienTai);
    const kq: KetQuaNode = node.chay();
    if (kq.trangThai === "loi") return { trangThai: "loi", loi: "loi_tai_node", dsNodeDaDi };
    tenHienTai = kq.tenNodeTiepTheo;
  }
  return { trangThai: "thanhCong", dsNodeDaDi };
}

function taoDoThiCoChuTrinh(): DoThi {
  return {
    kiemTra: taoNodeThanhCong("xuLyLoi"),
    xuLyLoi: taoNodeThanhCong("thuLai"),
    thuLai: taoNodeThanhCong("kiemTra"),
  };
}

console.log(JSON.stringify(chayDoThiCoPhatHienChuTrinh(taoDoThiCoChuTrinh(), "kiemTra")));
```

```text title=readonly
{"trangThai":"phat_hien_chu_trinh","tenNodeLap":"kiemTra","dsNodeDaDi":["kiemTra","xuLyLoi","thuLai"]}
```

`dsNodeDaDi` chỉ có ĐÚNG `3` phần tử — dừng khi node kế tiếp SẼ LÀ
`"kiemTra"` (đã có trong `daGhePhamVi`), `tenNodeLap` ghi ĐÚNG tên node
bị lặp. So VỚI `chayDoThiCoTranAnToan` (chạy đủ `20` lượt Ở TRÊN), cách
này tiết kiệm ĐÚNG `17` lượt gọi (`20 - 3 = 17`) — VÀ quan trọng hơn số
lượt: nó biết CHÍNH XÁC đây LÀ một chu trình, không phải một đồ thị dài
tình cờ chạm giới hạn.
::::

::::predict{#doan-doi-canh-quay-lai commitOnce}
Nếu đổi cạnh CUỐI của `taoDoThiCoChuTrinh` từ `thuLai → kiemTra` thành
`thuLai → xuLyLoi` (chu trình giờ quay LẠI `"xuLyLoi"` thay vì
`"kiemTra"`, giữ NGUYÊN `kiemTra → xuLyLoi → thuLai`) — `tenNodeLap` đổi
ra sao, VÀ tổng lượt gọi TRƯỚC KHI phát hiện (`dsNodeDaDi.length`) có đổi
theo KHÔNG?

:::opt{correct}
`tenNodeLap` đổi thành `"xuLyLoi"` (node BỊ LẶP giờ khác) — NHƯNG tổng
lượt gọi TRƯỚC phát hiện KHÔNG đổi, VẪN LÀ `3` (`["kiemTra","xuLyLoi",
"thuLai"]`), vì chu trình VẪN dài đúng `3` node, chỉ khác Ở CHỖ nó "khép
lại" tại node NÀO
:::
:::opt
Cả `tenNodeLap` LẪN tổng lượt gọi đều KHÔNG đổi — đổi cạnh quay lại chỉ
ảnh hưởng TỚI tên hiển thị, không ảnh hưởng CÁCH `Set` hoạt động
::why
Nhầm rằng `tenNodeLap` LÀ một hằng số cố định của thuật toán — nhưng nó
LÀ CHÍNH tên node MÀ `daGhePhamVi.has(tenHienTai)` trả `true`; đổi cạnh
quay lại nghĩa LÀ node NÀO "bị lặp lại" cũng đổi THEO, nên `tenNodeLap`
BẮT BUỘC phải đổi.

Chỗ lệch: `daGhePhamVi.has(tenHienTai)` kiểm ĐÚNG giá trị `tenHienTai`
TẠI thời điểm đó — Ở đồ thị MỚI, giá trị Đó LÀ `"xuLyLoi"` (không phải
`"kiemTra"`), nên `tenNodeLap` phải LÀ `"xuLyLoi"`.
::
:::
:::opt
Tổng lượt gọi TĂNG lên `4`, vì chu trình MỚI "dài hơn" một bước (phải đi
qua `"kiemTra"` lần NỮA trước khi phát hiện lặp Ở `"xuLyLoi"`)
::why
Nhầm rằng chu trình MỚI cần thêm MỘT bước để "khép kín" — nhưng đường
đi VẪN LÀ `kiemTra → xuLyLoi → thuLai`, VÀ bước KẾ TIẾP (từ `"thuLai"`)
LÀ `"xuLyLoi"` — ĐÃ có trong `daGhePhamVi` NGAY LẬP TỨC (được thêm Ở vòng
lặp THỨ HAI); không có bước "đi qua kiemTra lần nữa" nào chen giữa.

Chỗ lệch: `daGhePhamVi` sau `3` vòng lặp chứa ĐÚNG `{"kiemTra",
"xuLyLoi", "thuLai"}` — bất kể cạnh cuối trỏ VỀ node NÀO trong BA node
đó, sự lặp lại được phát hiện Ở NGAY vòng lặp thứ `4`, chưa hề thêm một
node MỚI vào `dsNodeDaDi`.
::
:::
::::

::::code{#viet_phat_hien_chu_trinh}
Hoàn thiện `chayDoThiCoPhatHienChuTrinh` — vòng `while` KHÔNG giới hạn
bằng điều kiện (chỉ `tenHienTai !== null`); MỖI vòng: NẾU `soBuoc >=
SO_BUOC_TOI_DA_AN_TOAN`, trả `hetBuoc` NGAY (lưới an toàn VẪN giữ, phòng
đồ thị dài THẬT không phải chu trình); NẾU `daGhePhamVi.has(tenHienTai)`,
trả `{ trangThai: "phat_hien_chu_trinh", tenNodeLap: tenHienTai,
dsNodeDaDi }` NGAY; NGƯỢC LẠI tăng `soBuoc`, tra node (lỗi nếu không tồn
tại), THÊM `tenHienTai` VÀO `daGhePhamVi` VÀ `dsNodeDaDi`, gọi `chay()`,
xử lý `"loi"`/đi tới `tenNodeTiepTheo` NHƯ thường. Hoàn thiện
`soSanhChuTrinh` — chạy CẢ `chayDoThiCoTranAnToan` VÀ
`chayDoThiCoPhatHienChuTrinh` trên CÙNG `doThi`/`tenNodeGoc`, trả về số
lượt gọi (`dsNodeDaDi.length`) của MỖI cách VÀ hiệu số tiết kiệm được.

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

const SO_BUOC_TOI_DA_AN_TOAN = 20;

type KetQuaChayDoThiMoRong =
  | { trangThai: "thanhCong"; dsNodeDaDi: string[] }
  | { trangThai: "loi"; loi: string; dsNodeDaDi: string[] }
  | { trangThai: "hetBuoc"; dsNodeDaDi: string[] };

function chayDoThiCoTranAnToan(doThi: DoThi, tenNodeGoc: string): KetQuaChayDoThiMoRong {
  const dsNodeDaDi: string[] = [];
  let tenHienTai: string | null = tenNodeGoc;
  let soBuoc = 0;
  while (tenHienTai !== null) {
    if (soBuoc >= SO_BUOC_TOI_DA_AN_TOAN) return { trangThai: "hetBuoc", dsNodeDaDi };
    soBuoc++;
    const node: NodeXuLy | undefined = doThi[tenHienTai];
    if (!node) return { trangThai: "loi", loi: "node_khong_ton_tai", dsNodeDaDi };
    dsNodeDaDi.push(tenHienTai);
    const kq: KetQuaNode = node.chay();
    if (kq.trangThai === "loi") return { trangThai: "loi", loi: "loi_tai_node", dsNodeDaDi };
    tenHienTai = kq.tenNodeTiepTheo;
  }
  return { trangThai: "thanhCong", dsNodeDaDi };
}

type KetQuaChayDoThiCoPhatHienChuTrinh =
  | { trangThai: "thanhCong"; dsNodeDaDi: string[] }
  | { trangThai: "loi"; loi: string; dsNodeDaDi: string[] }
  | { trangThai: "hetBuoc"; dsNodeDaDi: string[] }
  | { trangThai: "phat_hien_chu_trinh"; tenNodeLap: string; dsNodeDaDi: string[] };

function chayDoThiCoPhatHienChuTrinh(doThi: DoThi, tenNodeGoc: string): KetQuaChayDoThiCoPhatHienChuTrinh {
  ___
}

type KetQuaSoSanhChuTrinh = {
  soLuotGoiTranAnToan: number;
  soLuotGoiPhatHien: number;
  soLuotGoiTietKiem: number;
};

function soSanhChuTrinh(doThi: DoThi, tenNodeGoc: string): KetQuaSoSanhChuTrinh {
  ___
}

function taoDoThiCoChuTrinh(): DoThi {
  return {
    kiemTra: taoNodeThanhCong("xuLyLoi"),
    xuLyLoi: taoNodeThanhCong("thuLai"),
    thuLai: taoNodeThanhCong("kiemTra"),
  };
}

const soSanh1 = soSanhChuTrinh(taoDoThiCoChuTrinh(), "kiemTra");
console.log(JSON.stringify(soSanh1));
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

const SO_BUOC_TOI_DA_AN_TOAN = 20;

type KetQuaChayDoThiMoRong =
  | { trangThai: "thanhCong"; dsNodeDaDi: string[] }
  | { trangThai: "loi"; loi: string; dsNodeDaDi: string[] }
  | { trangThai: "hetBuoc"; dsNodeDaDi: string[] };

function chayDoThiCoTranAnToan(doThi: DoThi, tenNodeGoc: string): KetQuaChayDoThiMoRong {
  const dsNodeDaDi: string[] = [];
  let tenHienTai: string | null = tenNodeGoc;
  let soBuoc = 0;
  while (tenHienTai !== null) {
    if (soBuoc >= SO_BUOC_TOI_DA_AN_TOAN) return { trangThai: "hetBuoc", dsNodeDaDi };
    soBuoc++;
    const node: NodeXuLy | undefined = doThi[tenHienTai];
    if (!node) return { trangThai: "loi", loi: "node_khong_ton_tai", dsNodeDaDi };
    dsNodeDaDi.push(tenHienTai);
    const kq: KetQuaNode = node.chay();
    if (kq.trangThai === "loi") return { trangThai: "loi", loi: "loi_tai_node", dsNodeDaDi };
    tenHienTai = kq.tenNodeTiepTheo;
  }
  return { trangThai: "thanhCong", dsNodeDaDi };
}

type KetQuaChayDoThiCoPhatHienChuTrinh =
  | { trangThai: "thanhCong"; dsNodeDaDi: string[] }
  | { trangThai: "loi"; loi: string; dsNodeDaDi: string[] }
  | { trangThai: "hetBuoc"; dsNodeDaDi: string[] }
  | { trangThai: "phat_hien_chu_trinh"; tenNodeLap: string; dsNodeDaDi: string[] };

function chayDoThiCoPhatHienChuTrinh(doThi: DoThi, tenNodeGoc: string): KetQuaChayDoThiCoPhatHienChuTrinh {
  const dsNodeDaDi: string[] = [];
  const daGhePhamVi = new Set<string>();
  let tenHienTai: string | null = tenNodeGoc;
  let soBuoc = 0;
  while (tenHienTai !== null) {
    if (soBuoc >= SO_BUOC_TOI_DA_AN_TOAN) return { trangThai: "hetBuoc", dsNodeDaDi };
    if (daGhePhamVi.has(tenHienTai)) return { trangThai: "phat_hien_chu_trinh", tenNodeLap: tenHienTai, dsNodeDaDi };
    soBuoc++;
    const node: NodeXuLy | undefined = doThi[tenHienTai];
    if (!node) return { trangThai: "loi", loi: "node_khong_ton_tai", dsNodeDaDi };
    daGhePhamVi.add(tenHienTai);
    dsNodeDaDi.push(tenHienTai);
    const kq: KetQuaNode = node.chay();
    if (kq.trangThai === "loi") return { trangThai: "loi", loi: "loi_tai_node", dsNodeDaDi };
    tenHienTai = kq.tenNodeTiepTheo;
  }
  return { trangThai: "thanhCong", dsNodeDaDi };
}

type KetQuaSoSanhChuTrinh = {
  soLuotGoiTranAnToan: number;
  soLuotGoiPhatHien: number;
  soLuotGoiTietKiem: number;
};

function soSanhChuTrinh(doThi: DoThi, tenNodeGoc: string): KetQuaSoSanhChuTrinh {
  const ktTranAnToan = chayDoThiCoTranAnToan(doThi, tenNodeGoc);
  const ktPhatHien = chayDoThiCoPhatHienChuTrinh(doThi, tenNodeGoc);
  const soLuotGoiTranAnToan = ktTranAnToan.dsNodeDaDi.length;
  const soLuotGoiPhatHien = ktPhatHien.dsNodeDaDi.length;
  return {
    soLuotGoiTranAnToan,
    soLuotGoiPhatHien,
    soLuotGoiTietKiem: soLuotGoiTranAnToan - soLuotGoiPhatHien,
  };
}

function taoDoThiCoChuTrinh(): DoThi {
  return {
    kiemTra: taoNodeThanhCong("xuLyLoi"),
    xuLyLoi: taoNodeThanhCong("thuLai"),
    thuLai: taoNodeThanhCong("kiemTra"),
  };
}

const soSanh1 = soSanhChuTrinh(taoDoThiCoChuTrinh(), "kiemTra");
console.log(JSON.stringify(soSanh1));
```

```typescript title=test
if (JSON.stringify(soSanh1) !== JSON.stringify({ soLuotGoiTranAnToan: 20, soLuotGoiPhatHien: 3, soLuotGoiTietKiem: 17 })) {
  throw new Error("tren do thi co chu trinh, soSanhChuTrinh phai cho { soLuotGoiTranAnToan: 20, soLuotGoiPhatHien: 3, soLuotGoiTietKiem: 17 }");
}

const ktPhatHienTrucTiep = chayDoThiCoPhatHienChuTrinh(taoDoThiCoChuTrinh(), "kiemTra");
if (ktPhatHienTrucTiep.trangThai !== "phat_hien_chu_trinh") throw new Error("do thi co chu trinh phai cho trangThai la phat_hien_chu_trinh");
if (ktPhatHienTrucTiep.trangThai === "phat_hien_chu_trinh" && ktPhatHienTrucTiep.tenNodeLap !== "kiemTra") {
  throw new Error("tenNodeLap phai la kiemTra tren taoDoThiCoChuTrinh");
}
if (JSON.stringify(ktPhatHienTrucTiep.dsNodeDaDi) !== JSON.stringify(["kiemTra", "xuLyLoi", "thuLai"])) {
  throw new Error("dsNodeDaDi truoc khi phat hien phai dung 3 phan tu theo dung thu tu");
}

function taoDoThiTuyenTinh(): DoThi {
  return {
    batDau: taoNodeThanhCong("xuLy"),
    xuLy: taoNodeThanhCong("ketThuc"),
    ketThuc: taoNodeThanhCong(null),
  };
}
const soSanh2 = soSanhChuTrinh(taoDoThiTuyenTinh(), "batDau");
if (JSON.stringify(soSanh2) !== JSON.stringify({ soLuotGoiTranAnToan: 3, soLuotGoiPhatHien: 3, soLuotGoiTietKiem: 0 })) {
  throw new Error("tren do thi KHONG co chu trinh, ca hai cach phai cho CUNG so luot goi -- soLuotGoiTietKiem phai la 0");
}

const ktBinhThuong = chayDoThiCoPhatHienChuTrinh(taoDoThiTuyenTinh(), "batDau");
if (ktBinhThuong.trangThai !== "thanhCong") throw new Error("do thi tuyen tinh khong chu trinh phai cho trangThai la thanhCong");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (chayDoThiCoPhatHienChuTrinh): DUNG HET chayDoThiCoTranAnToan, nhung THEM mot dong kiem daGhePhamVi.has(tenHienTai) NGAY SAU kiem het buoc (TRUOC khi tra node), va THEM mot dong daGhePhamVi.add(tenHienTai) NGAY SAU khi xac nhan node ton tai (TRUOC khi push vao dsNodeDaDi). Cho hai (soSanhChuTrinh): goi CA HAI ham tren CUNG doThi/tenNodeGoc, lay .dsNodeDaDi.length cua moi ket qua, tra ve object voi hieu so."
- kind: strategy
  body: "Cho dau: const dsNodeDaDi: string[] = []; const daGhePhamVi = new Set<string>(); let tenHienTai: string | null = tenNodeGoc; let soBuoc = 0; while (tenHienTai !== null) { if (soBuoc >= SO_BUOC_TOI_DA_AN_TOAN) return { trangThai: 'hetBuoc', dsNodeDaDi }; if (daGhePhamVi.has(tenHienTai)) return { trangThai: 'phat_hien_chu_trinh', tenNodeLap: tenHienTai, dsNodeDaDi }; soBuoc++; const node: NodeXuLy | undefined = doThi[tenHienTai]; if (!node) return { trangThai: 'loi', loi: 'node_khong_ton_tai', dsNodeDaDi }; daGhePhamVi.add(tenHienTai); dsNodeDaDi.push(tenHienTai); const kq: KetQuaNode = node.chay(); if (kq.trangThai === 'loi') return { trangThai: 'loi', loi: 'loi_tai_node', dsNodeDaDi }; tenHienTai = kq.tenNodeTiepTheo; } return { trangThai: 'thanhCong', dsNodeDaDi }; Cho hai: const ktTranAnToan = chayDoThiCoTranAnToan(doThi, tenNodeGoc); const ktPhatHien = chayDoThiCoPhatHienChuTrinh(doThi, tenNodeGoc); const soLuotGoiTranAnToan = ktTranAnToan.dsNodeDaDi.length; const soLuotGoiPhatHien = ktPhatHien.dsNodeDaDi.length; return { soLuotGoiTranAnToan, soLuotGoiPhatHien, soLuotGoiTietKiem: soLuotGoiTranAnToan - soLuotGoiPhatHien };"
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
  expect: "{\"soLuotGoiTranAnToan\":20,\"soLuotGoiPhatHien\":3,\"soLuotGoiTietKiem\":17}"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`20` lượt gọi trước khi lưới an toàn chịu thua, so VỚI `3` lượt gọi khi
BIẾT chính xác đây LÀ một chu trình — VÀ trên đồ thị KHÔNG có chu trình,
cả hai cách cho CÙNG kết quả (tiết kiệm `0`). Bài sau lùi lại, ráp NHIỀU
cấu hình (audit, phát hiện chu trình, fan-out) thành MỘT bảng so sánh.
::::

::::reflect{#nghi-lai}
`SO_BUOC_TOI_DA_AN_TOAN` VÀ `daGhePhamVi` giải quyết HAI vấn đề khác
nhau, dù trông giống nhau ("dừng một vòng lặp"). Lưới an toàn trả lời
"đã chạy QUÁ NHIỀU bước chưa" — một câu hỏi về SỐ LƯỢNG, không quan tâm
NỘI DUNG đường đi. Tập `daGhePhamVi` trả lời "đã ĐI QUA node NÀY trước
đó trong LẦN CHẠY NÀY chưa" — một câu hỏi về CẤU TRÚC. Một đồ thị dài
`20` node THẬT (không chu trình) sẽ khiến `chayDoThiCoPhatHienChuTrinh`
chạy ĐỦ `20` bước rồi dừng Ở `hetBuoc`, giống HỆT `chayDoThiCoTranAnToan`
— phát hiện chu trình KHÔNG thay THẾ lưới an toàn, nó chỉ bắt được SỚM
HƠN đúng những trường hợp THẬT SỰ LÀ chu trình. Đây LÀ lý do CẢ hai lớp
kiểm tra (`soBuoc >= ...` VÀ `daGhePhamVi.has(...)`) đều cần Ở TRONG
CÙNG một hàm — bỏ một trong hai đều để lọt một chế độ hỏng KHÁC nhau.
::::

::::checkpoint{mastery=0.87}
::::
