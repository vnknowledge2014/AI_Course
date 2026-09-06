---
id: ky-nghe-ung-dung-ai.ky-thuat-do-thi.re-nhanh-co-dieu-kien
title: "Rẽ nhánh có điều kiện — MỘT node, HAI tên node kế tiếp khác nhau"
summary: "taoDoThiReNhanh(giaTri) dựng một đồ thị mà node 'kiemTra' trả về tenNodeTiepTheo='xuLyHopLe' NẾU giaTri>=0, HOẶC 'xuLyLoi' NẾU KHÔNG — cùng MỘT node, khác tên node kế tiếp tuỳ dữ liệu nó xử lý, KHÔNG phải hai node khác nhau được hard-code từ ngoài. chayTrenNhieuGiaTri([5,-3,0,-1]) (chạy chayDoThi từ 'kiemTra' trên MỘT đồ thị MỚI cho MỖI giá trị) cho 4 đường đi: [['kiemTra','xuLyHopLe'],['kiemTra','xuLyLoi'],['kiemTra','xuLyHopLe'],['kiemTra','xuLyLoi']] — giá trị 0 đi 'xuLyHopLe' (đúng ngưỡng >=0). Xác nhận bằng tập hợp: đường đi của giaTri=5 VÀ giaTri=-3 chỉ CHUNG đúng node 'kiemTra' — 'xuLyHopLe' KHÔNG BAO GIỜ xuất hiện trên đường đi của giá trị âm, và ngược lại 'xuLyLoi' không bao giờ xuất hiện trên đường đi của giá trị không âm: hai lần chạy với input khác nhau đi qua tập hợp node HOÀN TOÀN khác nhau ngoài node rẽ nhánh chung."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-do-thi
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 9
teaches: [kna.re-nhanh-co-dieu-kien]
requires: [kna.do-thi-la-node-va-edge-tuong-minh]
concepts: [kna.re-nhanh-co-dieu-kien]
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
Bài trước, MỖI node chỉ có ĐÚNG một `tenNodeTiepTheo` — cạnh cố định,
biết trước ngay khi tạo node. Nhưng một `tenNodeTiepTheo` không PHẢI
LÀ hằng số — nó LÀ kết quả của một PHÉP TÍNH bên trong `chay()`. Bài
này khai thác đúng điều đó: một node "kiểm tra" có thể trả về MỘT
trong HAI tên node khác nhau, tuỳ dữ liệu nó đang xử lý.
::::

::::explain{#mot_node_hai_ten_ke_tiep}
`taoDoThiReNhanh(giaTri)` dựng một đồ thị MỚI cho MỖI `giaTri` truyền
vào — node `"kiemTra"` đóng gói `giaTri` qua closure, VÀ trong `chay()`,
nó tự QUYẾT ĐỊNH trả về `tenNodeTiepTheo` NÀO dựa trên `giaTri >= 0`.
Đây KHÔNG phải hai node khác nhau được nối cứng từ NGOÀI — đây LÀ MỘT
node DUY NHẤT, với logic rẽ nhánh nằm NGAY BÊN TRONG `chay()` của nó:

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

function taoDoThiReNhanh(giaTri: number): DoThi {
  return {
    kiemTra: {
      chay(): KetQuaNode {
        if (giaTri >= 0) return { trangThai: "xong", tenNodeTiepTheo: "xuLyHopLe" };
        return { trangThai: "xong", tenNodeTiepTheo: "xuLyLoi" };
      },
    },
    xuLyHopLe: taoNodeThanhCong(null),
    xuLyLoi: taoNodeThanhCong(null),
  };
}

console.log(JSON.stringify(taoDoThiReNhanh(5).kiemTra!.chay()));
console.log(JSON.stringify(taoDoThiReNhanh(-3).kiemTra!.chay()));
```

```text title=readonly
{"trangThai":"xong","tenNodeTiepTheo":"xuLyHopLe"}
{"trangThai":"xong","tenNodeTiepTheo":"xuLyLoi"}
```

CÙNG một hàm `chay()` (đúng dòng code, đúng cấu trúc `if/return`) — nhưng
CHẠY với `giaTri` khác nhau cho ra `tenNodeTiepTheo` khác nhau. Rẽ nhánh
KHÔNG nằm Ở CẤU TRÚC đồ thị (`doThi` vẫn LÀ MỘT `Record` như bài trước),
mà nằm Ở KẾT QUẢ của phép tính bên trong node.
::::

::::example{#bon_gia_tri_bon_duong_di}
`chayTrenNhieuGiaTri` dựng MỘT đồ thị MỚI cho MỖI giá trị (qua
`taoDoThiReNhanh`), rồi chạy `chayDoThi` từ `"kiemTra"` trên đồ thị đó —
trên bộ `4` giá trị hỗn hợp (`5`, `-3`, `0`, `-1`):

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

function taoDoThiReNhanh(giaTri: number): DoThi {
  return {
    kiemTra: {
      chay(): KetQuaNode {
        if (giaTri >= 0) return { trangThai: "xong", tenNodeTiepTheo: "xuLyHopLe" };
        return { trangThai: "xong", tenNodeTiepTheo: "xuLyLoi" };
      },
    },
    xuLyHopLe: taoNodeThanhCong(null),
    xuLyLoi: taoNodeThanhCong(null),
  };
}

function chayTrenNhieuGiaTri(dsGiaTri: number[]): KetQuaChayDoThi[] {
  const ketQua: KetQuaChayDoThi[] = [];
  for (const giaTri of dsGiaTri) {
    ketQua.push(chayDoThi(taoDoThiReNhanh(giaTri), "kiemTra"));
  }
  return ketQua;
}

const ketQuaBonGiaTri = chayTrenNhieuGiaTri([5, -3, 0, -1]);
console.log(JSON.stringify(ketQuaBonGiaTri.map((k) => k.dsNodeDaDi)));
```

```text title=readonly
[["kiemTra","xuLyHopLe"],["kiemTra","xuLyLoi"],["kiemTra","xuLyHopLe"],["kiemTra","xuLyLoi"]]
```

`5` VÀ `0` (cả hai `>= 0`) đi `"xuLyHopLe"`; `-3` VÀ `-1` đi `"xuLyLoi"`.
Chú ý giá trị `0`: đúng NGƯỠNG `giaTri >= 0`, nó đi `"xuLyHopLe"` — MỘT
đơn vị khác biệt Ở điều kiện (`>=` so VỚI `>`) sẽ đổi hẳn kết quả của
riêng giá trị biên này.
::::

::::predict{#doan-doi-nguong-kiem-tra commitOnce}
Nếu đổi điều kiện trong `kiemTra` từ `giaTri >= 0` thành `giaTri > 0`
(giữ NGUYÊN mọi giá trị đầu vào `[5, -3, 0, -1]`) — kết quả CỦA RIÊNG
`giaTri = 0` đổi ra sao, VÀ ba giá trị còn lại có đổi theo không?

:::opt{correct}
CHỈ `giaTri = 0` đổi — nó chuyển từ `"xuLyHopLe"` sang `"xuLyLoi"` (vì
`0 > 0` LÀ `false`, trong khi `0 >= 0` LÀ `true`); BA giá trị còn lại
(`5`, `-3`, `-1`) giữ NGUYÊN kết quả cũ, vì `>` VÀ `>=` chỉ khác nhau
tại ĐÚNG điểm `0`
:::
:::opt
CẢ BỐN giá trị đổi kết quả — thay đổi điều kiện rẽ nhánh ảnh hưởng tới
TOÀN BỘ đồ thị, không riêng một giá trị nào
::why
Nhầm rằng sửa MỘT điều kiện so sánh làm lệch TOÀN BỘ tập giá trị — nhưng
`>=` VÀ `>` chỉ khác nhau Ở ĐÚNG một điểm: khi hai vế BẰNG nhau
(`giaTri === 0`). Với MỌI giá trị KHÁC `0`, hai điều kiện luôn cho CÙNG
kết quả boolean.

Chỗ lệch: `5 >= 0` VÀ `5 > 0` đều LÀ `true`; `-3 >= 0` VÀ `-3 > 0` đều
LÀ `false`; `-1` tương tự — chỉ `0 >= 0` (`true`) VÀ `0 > 0` (`false`)
LỆCH nhau.
::
:::
:::opt
Không giá trị nào đổi — `taoDoThiReNhanh` xây MỘT đồ thị MỚI cho MỖI lần
gọi, nên đổi điều kiện chỉ ảnh hưởng LẦN GỌI tiếp theo, không ảnh hưởng
các giá trị ĐÃ chạy trước đó
::why
Nhầm với việc "code đã chạy rồi thì không đổi được nữa" — nhưng ĐÂY LÀ
việc SỬA mã nguồn của `taoDoThiReNhanh` rồi CHẠY LẠI toàn bộ `chayTrenNhieuGiaTri`
từ đầu VỚI điều kiện MỚI — không phải chạy tiếp trên kết quả CŨ.

Chỗ lệch: câu hỏi giả định TOÀN BỘ `[5, -3, 0, -1]` được chạy LẠI, từ
đầu, VỚI đúng MỘT thay đổi (điều kiện so sánh) — không có "lần chạy
trước" nào còn tồn tại để so sánh.
::
:::
::::

::::code{#viet_do_thi_re_nhanh}
Hoàn thiện `taoDoThiReNhanh(giaTri)` — trả về một `DoThi` VỚI node
`"kiemTra"`: NẾU `giaTri >= 0`, `chay()` trả `{ trangThai: "xong",
tenNodeTiepTheo: "xuLyHopLe" }`; NGƯỢC LẠI trả `{ trangThai: "xong",
tenNodeTiepTheo: "xuLyLoi" }`; CỘNG hai node lá `"xuLyHopLe"` VÀ
`"xuLyLoi"` (dùng `taoNodeThanhCong(null)`, đều LÀ node CUỐI). Hoàn
thiện `chayTrenNhieuGiaTri` — với MỖI giá trị trong `dsGiaTri`, dựng một
đồ thị MỚI qua `taoDoThiReNhanh`, chạy `chayDoThi` từ `"kiemTra"`, đẩy
kết quả vào mảng trả về.

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

function taoDoThiReNhanh(giaTri: number): DoThi {
  ___
}

function chayTrenNhieuGiaTri(dsGiaTri: number[]): KetQuaChayDoThi[] {
  ___
}

const ketQuaBonGiaTri = chayTrenNhieuGiaTri([5, -3, 0, -1]);
console.log(JSON.stringify(ketQuaBonGiaTri.map((k) => k.dsNodeDaDi)));
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

function taoDoThiReNhanh(giaTri: number): DoThi {
  return {
    kiemTra: {
      chay(): KetQuaNode {
        if (giaTri >= 0) return { trangThai: "xong", tenNodeTiepTheo: "xuLyHopLe" };
        return { trangThai: "xong", tenNodeTiepTheo: "xuLyLoi" };
      },
    },
    xuLyHopLe: taoNodeThanhCong(null),
    xuLyLoi: taoNodeThanhCong(null),
  };
}

function chayTrenNhieuGiaTri(dsGiaTri: number[]): KetQuaChayDoThi[] {
  const ketQua: KetQuaChayDoThi[] = [];
  for (const giaTri of dsGiaTri) {
    ketQua.push(chayDoThi(taoDoThiReNhanh(giaTri), "kiemTra"));
  }
  return ketQua;
}

const ketQuaBonGiaTri = chayTrenNhieuGiaTri([5, -3, 0, -1]);
console.log(JSON.stringify(ketQuaBonGiaTri.map((k) => k.dsNodeDaDi)));
```

```typescript title=test
if (ketQuaBonGiaTri.length !== 4) throw new Error("chayTrenNhieuGiaTri phai tra ve mang dung 4 phan tu");
if (JSON.stringify(ketQuaBonGiaTri.map((k) => k.dsNodeDaDi)) !== JSON.stringify([
  ["kiemTra", "xuLyHopLe"],
  ["kiemTra", "xuLyLoi"],
  ["kiemTra", "xuLyHopLe"],
  ["kiemTra", "xuLyLoi"],
])) {
  throw new Error("re nhanh tren [5,-3,0,-1] phai cho dung 4 duong di nhu tren -- gia tri 0 phai di xuLyHopLe (>=0)");
}
for (const k of ketQuaBonGiaTri) {
  if (k.thanhCong !== true) throw new Error("moi gia tri phai cho ra mot duong di THANH CONG, khong co gia tri nao gay loi");
}

const duongHopLe = chayDoThi(taoDoThiReNhanh(5), "kiemTra");
const duongLoi = chayDoThi(taoDoThiReNhanh(-3), "kiemTra");
const tapHopLe = new Set(duongHopLe.dsNodeDaDi);
const tapLoi = new Set(duongLoi.dsNodeDaDi);
if (tapHopLe.has("xuLyLoi")) throw new Error("duong gia tri hop le KHONG duoc di qua xuLyLoi");
if (tapLoi.has("xuLyHopLe")) throw new Error("duong gia tri am KHONG duoc di qua xuLyHopLe");
if (!tapHopLe.has("kiemTra") || !tapLoi.has("kiemTra")) throw new Error("CA HAI duong deu phai di qua kiemTra (node chung)");

const ketQuaMotGiaTri = chayTrenNhieuGiaTri([100]);
if (ketQuaMotGiaTri.length !== 1) throw new Error("doi so danh sach gia tri phai doi do dai mang tra ve -- tham so phai duoc dung that");
if (JSON.stringify(ketQuaMotGiaTri[0]!.dsNodeDaDi) !== JSON.stringify(["kiemTra", "xuLyHopLe"])) {
  throw new Error("gia tri 100 phai di qua xuLyHopLe");
}
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (taoDoThiReNhanh): tra ve mot object voi ba khoa -- kiemTra (mot node CHUA closure giu giaTri, chay() kiem giaTri >= 0 de chon tenNodeTiepTheo la xuLyHopLe hay xuLyLoi), xuLyHopLe VA xuLyLoi (deu dung taoNodeThanhCong(null), la node CUOI). Cho hai (chayTrenNhieuGiaTri): mot vong for-of qua dsGiaTri, MOI gia tri goi taoDoThiReNhanh(giaTri) de tao MOT do thi MOI, roi chayDoThi(..., \"kiemTra\"), day ket qua vao mot mang."
- kind: strategy
  body: "Cho dau: return { kiemTra: { chay(): KetQuaNode { if (giaTri >= 0) return { trangThai: \"xong\", tenNodeTiepTheo: \"xuLyHopLe\" }; return { trangThai: \"xong\", tenNodeTiepTheo: \"xuLyLoi\" }; } }, xuLyHopLe: taoNodeThanhCong(null), xuLyLoi: taoNodeThanhCong(null) }; Cho hai: const ketQua: KetQuaChayDoThi[] = []; for (const giaTri of dsGiaTri) { ketQua.push(chayDoThi(taoDoThiReNhanh(giaTri), \"kiemTra\")); } return ketQua;"
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
  expect: "[[\"kiemTra\",\"xuLyHopLe\"],[\"kiemTra\",\"xuLyLoi\"],[\"kiemTra\",\"xuLyHopLe\"],[\"kiemTra\",\"xuLyLoi\"]]"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn giá trị, hai nhánh — VÀ ngoài node `"kiemTra"` chung, hai đường đi
KHÔNG hề chạm nhau. Đây LÀ lý do một đồ thị có thể có node "hiếm khi được
chạm tới": nếu bộ input KHÔNG BAO GIỜ đưa ra một giá trị THUỘC nhánh đó,
cạnh dẫn tới nó KHÔNG BAO GIỜ được thực thi. Bài sau đo đúng điều đó —
edge coverage.
::::

::::reflect{#nghi-lai}
Rẽ nhánh KHÔNG thêm một CƠ CHẾ mới vào `chayDoThi` — hàm đó Ở bài trước
đã đủ tổng quát để chạy đúng bất kỳ đồ thị nào, kể cả đồ thị rẽ nhánh.
Điều thật sự mới nằm Ở CHỖ node "kiemTra" — nó không còn trả về MỘT
`tenNodeTiepTheo` cố định, mà tính toán ra tên đó dựa trên dữ liệu ĐANG
xử lý. Đây LÀ khác biệt so VỚI LOOP T9.4: một vòng lặp CÓ THỂ rẽ nhánh
BÊN TRONG một bước (`if/else` trong thân xử lý), nhưng KHÔNG BAO GIỜ đổi
bước NÀO chạy TIẾP THEO — bước tiếp theo LUÔN LÀ "gọi lại chính hàm vừa
chạy". Ở GRAPH, rẽ nhánh đổi hẳn ĐƯỜNG ĐI — hai lần chạy VỚI input khác
nhau có thể không hề dùng chung một tập hợp node nào ngoài node rẽ nhánh.
::::

::::checkpoint{mastery=0.85}
::::
