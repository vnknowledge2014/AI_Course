---
id: ky-nghe-ung-dung-ai.ky-thuat-do-thi.audit-trail-ghi-lai-vi-sao-re-nhanh
title: "Audit trail — ghi lại VÌ SAO mỗi cạnh được chọn"
summary: "chayDoThiCoAudit mở rộng chayDoThi (bài 1): thay vì dsNodeDaDi (chỉ TÊN node), nó tích luỹ dsAudit: BanGhiAudit[] với BanGhiAudit={tenNode, lyDo} — lyDo lấy từ trường tuỳ chọn lyDo mà MỘT node rẽ nhánh có thể đính kèm vào KetQuaNode khi trả về (ví dụ chuỗi mô tả điều kiện dẫn tới lựa chọn đó); node KHÔNG rẽ nhánh mặc định lyDo='khong_re_nhanh'. Trên taoDoThiDayDuCoAudit(giaTri, coTheThuLai) (hai tầng rẽ nhánh, giống bài 3): chayDoThiCoAudit(...(-3, true), 'kiemTra') cho dsAudit=[{tenNode:'kiemTra',lyDo:'giaTri=-3<0'},{tenNode:'xuLyLoi',lyDo:'coTheThuLai=true'},{tenNode:'thuLai',lyDo:'khong_re_nhanh'}] — VÌ SAO mỗi cạnh được chọn, không chỉ đi qua đâu. layLyDoTaiNode(dsAudit, tenNode) tra cứu SAU KHI chạy xong: layLyDoTaiNode(dsAudit,'kiemTra')='giaTri=-3<0', layLyDoTaiNode(dsAudit,'khongTonTai')=null. Node gây lỗi VẪN được ghi vào dsAudit (với lyDo mặc định nếu bản thân nó không rẽ nhánh), y hệt dsNodeDaDi ở bài 1 vẫn ghi node gây lỗi."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-do-thi
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [kna.audit-trail-ghi-lai-vi-sao-re-nhanh]
requires: [kna.fan-out-fan-in-hai-nhanh-mot-diem-hoi-tu]
concepts: [kna.audit-trail-ghi-lai-vi-sao-re-nhanh]
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
`dsNodeDaDi` (bài `1`-`4`) trả lời ĐÚNG một câu: "đồ thị đã đi qua NHỮNG
node nào?". Nhưng khi một lần chạy đi qua nhánh HIẾM (bài `3`), câu hỏi
THẬT SỰ cần trả lời LÀ khác: "VÌ SAO nó đi nhánh NÀY mà không phải nhánh
KIA?". `dsNodeDaDi` không hề ghi lại điều đó — VÀ đây chính LÀ lỗ hổng
"không audit" MÀ MASTERPLAN liệt kê cho GRAPH.
::::

::::explain{#ban_ghi_audit_va_ly_do_mac_dinh}
`chayDoThiCoAudit` mở rộng `chayDoThi` (bài `1`): thay vì tích luỹ MỘT
mảng tên (`dsNodeDaDi`), nó tích luỹ `dsAudit: BanGhiAudit[]` — MỖI phần
tử LÀ `{ tenNode, lyDo }`. `lyDo` lấy từ một trường TUỲ CHỌN `lyDo?:
string` MÀ một node rẽ nhánh CÓ THỂ đính kèm vào `KetQuaNode` khi nó trả
về (mô tả điều kiện VỪA dẫn tới lựa chọn đó); node KHÔNG rẽ nhánh (như
`taoNodeThanhCong`) không hề đính `lyDo` — VÀ khi đó `chayDoThiCoAudit`
tự điền `LY_DO_MAC_DINH = "khong_re_nhanh"`:

```typescript title=readonly
type KetQuaNode = { trangThai: "xong" | "loi"; tenNodeTiepTheo: string | null; lyDo?: string };

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

type BanGhiAudit = { tenNode: string; lyDo: string };

type KetQuaChayDoThiCoAudit =
  | { thanhCong: true; dsAudit: BanGhiAudit[] }
  | { thanhCong: false; loi: string; dsAudit: BanGhiAudit[] };

const LY_DO_MAC_DINH = "khong_re_nhanh";

function chayDoThiCoAudit(doThi: DoThi, tenNodeGoc: string): KetQuaChayDoThiCoAudit {
  const dsAudit: BanGhiAudit[] = [];
  let tenHienTai: string | null = tenNodeGoc;
  let soBuoc = 0;
  while (tenHienTai !== null && soBuoc < SO_BUOC_TOI_DA_AN_TOAN) {
    soBuoc++;
    const node: NodeXuLy | undefined = doThi[tenHienTai];
    if (!node) return { thanhCong: false, loi: "node_khong_ton_tai", dsAudit };
    const kq: KetQuaNode = node.chay();
    dsAudit.push({ tenNode: tenHienTai, lyDo: kq.lyDo ?? LY_DO_MAC_DINH });
    if (kq.trangThai === "loi") return { thanhCong: false, loi: "loi_tai_node", dsAudit };
    tenHienTai = kq.tenNodeTiepTheo;
  }
  return { thanhCong: true, dsAudit };
}

function taoDoThiDayDuCoAudit(giaTri: number, coTheThuLai: boolean): DoThi {
  return {
    kiemTra: {
      chay(): KetQuaNode {
        if (giaTri >= 0) return { trangThai: "xong", tenNodeTiepTheo: "xuLyHopLe", lyDo: `giaTri=${giaTri}>=0` };
        return { trangThai: "xong", tenNodeTiepTheo: "xuLyLoi", lyDo: `giaTri=${giaTri}<0` };
      },
    },
    xuLyHopLe: taoNodeThanhCong(null),
    xuLyLoi: {
      chay(): KetQuaNode {
        if (coTheThuLai) return { trangThai: "xong", tenNodeTiepTheo: "thuLai", lyDo: `coTheThuLai=${coTheThuLai}` };
        return { trangThai: "xong", tenNodeTiepTheo: "boCuoc", lyDo: `coTheThuLai=${coTheThuLai}` };
      },
    },
    thuLai: taoNodeThanhCong(null),
    boCuoc: taoNodeThanhCong(null),
  };
}

console.log(JSON.stringify(chayDoThiCoAudit(taoDoThiDayDuCoAudit(-3, true), "kiemTra")));
```

```text title=readonly
{"thanhCong":true,"dsAudit":[{"tenNode":"kiemTra","lyDo":"giaTri=-3<0"},{"tenNode":"xuLyLoi","lyDo":"coTheThuLai=true"},{"tenNode":"thuLai","lyDo":"khong_re_nhanh"}]}
```

`kiemTra` VÀ `xuLyLoi` (hai node RẼ NHÁNH) đều để lại `lyDo` mô tả ĐÚNG
điều kiện vừa xét (`"giaTri=-3<0"`, `"coTheThuLai=true"`); `thuLai` (node
CUỐI, không rẽ nhánh) nhận `lyDo` MẶC ĐỊNH `"khong_re_nhanh"`. `field`
`lyDo` LÀ optional (`lyDo?: string`) — `taoNodeThanhCong` KHÔNG hề gán
`lyDo: undefined` tường minh, nó chỉ ĐƠN GIẢN không đính kèm khoá đó VÀO
object trả về (đúng gotcha `exactOptionalPropertyTypes`).
::::

::::example{#audit_tra_loi_vi_sao_khong_chi_di_qua_dau}
So sánh HAI lần chạy trên CÙNG `taoDoThiDayDuCoAudit` — CHỈ đổi `giaTri`
từ `-3` sang `5` (giữ `coTheThuLai: true`):

```typescript title=readonly
type KetQuaNode = { trangThai: "xong" | "loi"; tenNodeTiepTheo: string | null; lyDo?: string };

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

type BanGhiAudit = { tenNode: string; lyDo: string };

type KetQuaChayDoThiCoAudit =
  | { thanhCong: true; dsAudit: BanGhiAudit[] }
  | { thanhCong: false; loi: string; dsAudit: BanGhiAudit[] };

const LY_DO_MAC_DINH = "khong_re_nhanh";

function chayDoThiCoAudit(doThi: DoThi, tenNodeGoc: string): KetQuaChayDoThiCoAudit {
  const dsAudit: BanGhiAudit[] = [];
  let tenHienTai: string | null = tenNodeGoc;
  let soBuoc = 0;
  while (tenHienTai !== null && soBuoc < SO_BUOC_TOI_DA_AN_TOAN) {
    soBuoc++;
    const node: NodeXuLy | undefined = doThi[tenHienTai];
    if (!node) return { thanhCong: false, loi: "node_khong_ton_tai", dsAudit };
    const kq: KetQuaNode = node.chay();
    dsAudit.push({ tenNode: tenHienTai, lyDo: kq.lyDo ?? LY_DO_MAC_DINH });
    if (kq.trangThai === "loi") return { thanhCong: false, loi: "loi_tai_node", dsAudit };
    tenHienTai = kq.tenNodeTiepTheo;
  }
  return { thanhCong: true, dsAudit };
}

function layLyDoTaiNode(dsAudit: BanGhiAudit[], tenNode: string): string | null {
  const banGhi = dsAudit.find((b) => b.tenNode === tenNode);
  return banGhi ? banGhi.lyDo : null;
}

function taoDoThiDayDuCoAudit(giaTri: number, coTheThuLai: boolean): DoThi {
  return {
    kiemTra: {
      chay(): KetQuaNode {
        if (giaTri >= 0) return { trangThai: "xong", tenNodeTiepTheo: "xuLyHopLe", lyDo: `giaTri=${giaTri}>=0` };
        return { trangThai: "xong", tenNodeTiepTheo: "xuLyLoi", lyDo: `giaTri=${giaTri}<0` };
      },
    },
    xuLyHopLe: taoNodeThanhCong(null),
    xuLyLoi: {
      chay(): KetQuaNode {
        if (coTheThuLai) return { trangThai: "xong", tenNodeTiepTheo: "thuLai", lyDo: `coTheThuLai=${coTheThuLai}` };
        return { trangThai: "xong", tenNodeTiepTheo: "boCuoc", lyDo: `coTheThuLai=${coTheThuLai}` };
      },
    },
    thuLai: taoNodeThanhCong(null),
    boCuoc: taoNodeThanhCong(null),
  };
}

const ketQuaAm = chayDoThiCoAudit(taoDoThiDayDuCoAudit(-3, true), "kiemTra");
const ketQuaDuong = chayDoThiCoAudit(taoDoThiDayDuCoAudit(5, true), "kiemTra");
console.log(layLyDoTaiNode(ketQuaAm.dsAudit, "kiemTra"));
console.log(layLyDoTaiNode(ketQuaDuong.dsAudit, "kiemTra"));
console.log(JSON.stringify(ketQuaDuong.dsAudit.map((b) => b.tenNode)));
```

```text title=readonly
giaTri=-3<0
giaTri=5>=0
["kiemTra","xuLyHopLe"]
```

`layLyDoTaiNode` truy vết SAU KHI đã chạy xong — trả lời "VÌ SAO đồ thị
đi qua nhánh NÀY": khi `giaTri=-3`, lý do tại `"kiemTra"` LÀ
`"giaTri=-3<0"`; khi `giaTri=5`, lý do LÀ `"giaTri=5>=0"`. `dsAudit.map(b
=> b.tenNode)` cho ra ĐÚNG danh sách tên node MÀ `dsNodeDaDi` (bài `1`-`4`)
cũng sẽ cho — audit trail KHÔNG thay THẾ `dsNodeDaDi`, nó bổ sung THÊM
MỘT trường (`lyDo`) vào CÙNG một đường đi.
::::

::::predict{#doan-ten-node-audit-co-giong-dsnodedadi commitOnce}
Nếu lấy `dsAudit.map(b => b.tenNode)` VÀ so với `dsNodeDaDi` (bài `1`)
từ MỘT lần chạy `chayDoThi` THÔNG THƯỜNG trên CÙNG một đồ thị, CÙNG một
điểm bắt đầu — hai danh sách tên node ĐÓ có giống hệt nhau không?

:::opt{correct}
Giống HỆT nhau (cùng tên, cùng THỨ TỰ) — `chayDoThiCoAudit` đi qua ĐÚNG
những node MÀ `chayDoThi` (bài `1`) cũng đi qua, theo ĐÚNG một vòng
`while` tương tự; audit CHỈ thêm một trường `lyDo` vào MỖI bản ghi, KHÔNG
thêm HAY bớt node NÀO khỏi đường đi
:::
:::opt
Khác nhau — audit BỎ QUA những node KHÔNG rẽ nhánh, vì chúng không có
`lyDo` thật (chỉ có `lyDo` mặc định)
::why
Nhầm rằng `lyDo` mặc định (`"khong_re_nhanh"`) nghĩa LÀ node đó "không
được audit" — nhưng `dsAudit.push({ tenNode: tenHienTai, lyDo: ... })`
chạy CHO MỌI node, kể cả node không rẽ nhánh; `lyDo` mặc định VẪN LÀ một
bản ghi HỢP LỆ, chỉ khác Ở nội dung của `lyDo`, không phải THIẾU bản ghi.

Chỗ lệch: `dsAudit.length` LUÔN bằng `dsNodeDaDi.length` trên CÙNG một
lần chạy — không node nào bị bỏ SÓT chỉ vì nó không rẽ nhánh.
::
:::
:::opt
Khác nhau — audit thêm MỘT bản ghi PHỤ Ở ĐẦU mảng để ghi `tenNodeGoc`
trước khi vòng lặp bắt đầu
::why
Nhầm rằng có một bước "khởi tạo" ghi audit TRƯỚC vòng `while` — nhưng
`chayDoThiCoAudit` chỉ có ĐÚNG một chỗ gọi `dsAudit.push`, nằm BÊN TRONG
thân `while`, chạy MỖI lần một node được tra VÀ gọi `chay()` — không có
bước ghi nào xảy ra TRƯỚC vòng lặp.

Chỗ lệch: bản ghi ĐẦU TIÊN trong `dsAudit` chính LÀ `tenNodeGoc` (node
gốc) — nhưng nó được ghi Ở VÒNG LẶP ĐẦU TIÊN, không phải một bước riêng
biệt trước đó.
::
:::
::::

::::code{#viet_audit_trail}
Hoàn thiện `chayDoThiCoAudit` — ĐÚNG hình dạng `chayDoThi` (bài `1`)
NHƯNG: TRA node theo tên, NẾU không tồn tại trả lỗi `"node_khong_ton_tai"`
NGAY (dùng `dsAudit` rỗng/đã tích luỹ); nếu có, gọi `chay()` TRƯỚC, rồi
ĐẨY `{ tenNode: tenHienTai, lyDo: kq.lyDo ?? LY_DO_MAC_DINH }` VÀO
`dsAudit`; NẾU `trangThai` LÀ `"loi"`, trả lỗi `"loi_tai_node"` NGAY
(audit của node gây lỗi VẪN đã được đẩy); NGƯỢC LẠI đi tới
`tenNodeTiepTheo`. Hoàn thiện `layLyDoTaiNode` — tìm phần tử ĐẦU TIÊN
trong `dsAudit` có `tenNode` khớp, trả `lyDo` của nó, HOẶC `null` nếu
không tìm thấy.

```typescript title=starter
type KetQuaNode = { trangThai: "xong" | "loi"; tenNodeTiepTheo: string | null; lyDo?: string };

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

type BanGhiAudit = { tenNode: string; lyDo: string };

type KetQuaChayDoThiCoAudit =
  | { thanhCong: true; dsAudit: BanGhiAudit[] }
  | { thanhCong: false; loi: string; dsAudit: BanGhiAudit[] };

const LY_DO_MAC_DINH = "khong_re_nhanh";

function chayDoThiCoAudit(doThi: DoThi, tenNodeGoc: string): KetQuaChayDoThiCoAudit {
  ___
}

function layLyDoTaiNode(dsAudit: BanGhiAudit[], tenNode: string): string | null {
  ___
}

function taoDoThiDayDuCoAudit(giaTri: number, coTheThuLai: boolean): DoThi {
  return {
    kiemTra: {
      chay(): KetQuaNode {
        if (giaTri >= 0) return { trangThai: "xong", tenNodeTiepTheo: "xuLyHopLe", lyDo: `giaTri=${giaTri}>=0` };
        return { trangThai: "xong", tenNodeTiepTheo: "xuLyLoi", lyDo: `giaTri=${giaTri}<0` };
      },
    },
    xuLyHopLe: taoNodeThanhCong(null),
    xuLyLoi: {
      chay(): KetQuaNode {
        if (coTheThuLai) return { trangThai: "xong", tenNodeTiepTheo: "thuLai", lyDo: `coTheThuLai=${coTheThuLai}` };
        return { trangThai: "xong", tenNodeTiepTheo: "boCuoc", lyDo: `coTheThuLai=${coTheThuLai}` };
      },
    },
    thuLai: taoNodeThanhCong(null),
    boCuoc: taoNodeThanhCong(null),
  };
}

const ketQuaAmAudit = chayDoThiCoAudit(taoDoThiDayDuCoAudit(-3, true), "kiemTra");
console.log(
  layLyDoTaiNode(ketQuaAmAudit.dsAudit, "kiemTra"),
  layLyDoTaiNode(ketQuaAmAudit.dsAudit, "xuLyLoi"),
  layLyDoTaiNode(ketQuaAmAudit.dsAudit, "khongTonTai"),
);
```

```typescript title=solution
type KetQuaNode = { trangThai: "xong" | "loi"; tenNodeTiepTheo: string | null; lyDo?: string };

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

type BanGhiAudit = { tenNode: string; lyDo: string };

type KetQuaChayDoThiCoAudit =
  | { thanhCong: true; dsAudit: BanGhiAudit[] }
  | { thanhCong: false; loi: string; dsAudit: BanGhiAudit[] };

const LY_DO_MAC_DINH = "khong_re_nhanh";

function chayDoThiCoAudit(doThi: DoThi, tenNodeGoc: string): KetQuaChayDoThiCoAudit {
  const dsAudit: BanGhiAudit[] = [];
  let tenHienTai: string | null = tenNodeGoc;
  let soBuoc = 0;
  while (tenHienTai !== null && soBuoc < SO_BUOC_TOI_DA_AN_TOAN) {
    soBuoc++;
    const node: NodeXuLy | undefined = doThi[tenHienTai];
    if (!node) return { thanhCong: false, loi: "node_khong_ton_tai", dsAudit };
    const kq: KetQuaNode = node.chay();
    dsAudit.push({ tenNode: tenHienTai, lyDo: kq.lyDo ?? LY_DO_MAC_DINH });
    if (kq.trangThai === "loi") return { thanhCong: false, loi: "loi_tai_node", dsAudit };
    tenHienTai = kq.tenNodeTiepTheo;
  }
  return { thanhCong: true, dsAudit };
}

function layLyDoTaiNode(dsAudit: BanGhiAudit[], tenNode: string): string | null {
  const banGhi = dsAudit.find((b) => b.tenNode === tenNode);
  return banGhi ? banGhi.lyDo : null;
}

function taoDoThiDayDuCoAudit(giaTri: number, coTheThuLai: boolean): DoThi {
  return {
    kiemTra: {
      chay(): KetQuaNode {
        if (giaTri >= 0) return { trangThai: "xong", tenNodeTiepTheo: "xuLyHopLe", lyDo: `giaTri=${giaTri}>=0` };
        return { trangThai: "xong", tenNodeTiepTheo: "xuLyLoi", lyDo: `giaTri=${giaTri}<0` };
      },
    },
    xuLyHopLe: taoNodeThanhCong(null),
    xuLyLoi: {
      chay(): KetQuaNode {
        if (coTheThuLai) return { trangThai: "xong", tenNodeTiepTheo: "thuLai", lyDo: `coTheThuLai=${coTheThuLai}` };
        return { trangThai: "xong", tenNodeTiepTheo: "boCuoc", lyDo: `coTheThuLai=${coTheThuLai}` };
      },
    },
    thuLai: taoNodeThanhCong(null),
    boCuoc: taoNodeThanhCong(null),
  };
}

const ketQuaAmAudit = chayDoThiCoAudit(taoDoThiDayDuCoAudit(-3, true), "kiemTra");
console.log(
  layLyDoTaiNode(ketQuaAmAudit.dsAudit, "kiemTra"),
  layLyDoTaiNode(ketQuaAmAudit.dsAudit, "xuLyLoi"),
  layLyDoTaiNode(ketQuaAmAudit.dsAudit, "khongTonTai"),
);
```

```typescript title=test
if (layLyDoTaiNode(ketQuaAmAudit.dsAudit, "kiemTra") !== "giaTri=-3<0") {
  throw new Error("ly do tai kiemTra (giaTri=-3) phai la 'giaTri=-3<0'");
}
if (layLyDoTaiNode(ketQuaAmAudit.dsAudit, "xuLyLoi") !== "coTheThuLai=true") {
  throw new Error("ly do tai xuLyLoi (coTheThuLai=true) phai la 'coTheThuLai=true'");
}
if (layLyDoTaiNode(ketQuaAmAudit.dsAudit, "khongTonTai") !== null) {
  throw new Error("truy van mot ten node KHONG co trong dsAudit phai tra ve null");
}
if (layLyDoTaiNode(ketQuaAmAudit.dsAudit, "thuLai") !== "khong_re_nhanh") {
  throw new Error("node cuoi (thuLai) khong re nhanh phai co ly do mac dinh 'khong_re_nhanh'");
}
if (JSON.stringify(ketQuaAmAudit.dsAudit.map((b) => b.tenNode)) !== JSON.stringify(["kiemTra", "xuLyLoi", "thuLai"])) {
  throw new Error("danh sach ten node trong dsAudit phai dung 3 phan tu theo dung thu tu di qua");
}

const ketQuaDuongAudit = chayDoThiCoAudit(taoDoThiDayDuCoAudit(5, true), "kiemTra");
if (layLyDoTaiNode(ketQuaDuongAudit.dsAudit, "kiemTra") !== "giaTri=5>=0") {
  throw new Error("ly do tai kiemTra (giaTri=5) phai la 'giaTri=5>=0'");
}
if (JSON.stringify(ketQuaDuongAudit.dsAudit.map((b) => b.tenNode)) !== JSON.stringify(["kiemTra", "xuLyHopLe"])) {
  throw new Error("giaTri=5 phai di qua dung kiemTra roi xuLyHopLe");
}

const doThiLoiAudit: DoThi = {
  start: taoNodeThanhCong("loiNode"),
  loiNode: taoNodeLoi(),
};
const ketQuaLoiAudit = chayDoThiCoAudit(doThiLoiAudit, "start");
if (ketQuaLoiAudit.thanhCong !== false) throw new Error("do thi co node bao loi phai that bai");
if (JSON.stringify(ketQuaLoiAudit.dsAudit.map((b) => b.tenNode)) !== JSON.stringify(["start", "loiNode"])) {
  throw new Error("node gay loi (loiNode) VAN phai duoc ghi vao dsAudit, giong het dsNodeDaDi o bai 1");
}
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (chayDoThiCoAudit): DUNG HET vong while cua chayDoThi bai 1, nhung THU TU trong than vong lap la: tra node, neu khong ton tai tra loi NGAY (chua push gi), neu co thi GOI chay() TRUOC (luu vao kq), roi moi day { tenNode: tenHienTai, lyDo: kq.lyDo ?? LY_DO_MAC_DINH } vao dsAudit, roi moi kiem tra kq.trangThai. Cho hai (layLyDoTaiNode): dung .find tren dsAudit de tim phan tu co tenNode khop, tra ve b.lyDo neu tim thay, nguoc lai tra null."
- kind: strategy
  body: "Cho dau: const dsAudit: BanGhiAudit[] = []; let tenHienTai: string | null = tenNodeGoc; let soBuoc = 0; while (tenHienTai !== null && soBuoc < SO_BUOC_TOI_DA_AN_TOAN) { soBuoc++; const node: NodeXuLy | undefined = doThi[tenHienTai]; if (!node) return { thanhCong: false, loi: 'node_khong_ton_tai', dsAudit }; const kq: KetQuaNode = node.chay(); dsAudit.push({ tenNode: tenHienTai, lyDo: kq.lyDo ?? LY_DO_MAC_DINH }); if (kq.trangThai === 'loi') return { thanhCong: false, loi: 'loi_tai_node', dsAudit }; tenHienTai = kq.tenNodeTiepTheo; } return { thanhCong: true, dsAudit }; Cho hai: const banGhi = dsAudit.find((b) => b.tenNode === tenNode); return banGhi ? banGhi.lyDo : null;"
- kind: one-line
  body: "Sao chep dung hai khoi o phan Strategy vao dung vi tri tuong ung -- CHU Y thu tu goi chay() TRUOC roi moi push trong cho dau."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "giaTri=-3<0 coTheThuLai=true null"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`dsNodeDaDi` nói ĐÃ đi qua đâu; `dsAudit` nói VÌ SAO. Cùng một đường đi,
cùng thứ tự tên node — CHỈ thêm một trường `lyDo` VÀO MỖI bước rẽ nhánh.
Bài sau khai thác một hệ quả trực tiếp: nếu biết ĐƯỢC vì sao, ta cũng có
thể TEST một nhánh cụ thể mà KHÔNG cần dàn dựng lại từ node gốc.
::::

::::reflect{#nghi-lai}
Audit trail KHÔNG phải một cơ chế CHẠY mới — `chayDoThiCoAudit` VẪN LÀ
đúng một vòng `while` đi theo `tenNodeTiepTheo`, giống HỆT `chayDoThi`
(bài `1`). Điều nó THÊM VÀO LÀ một KÊNH THÔNG TIN phụ, chảy // SONG SONG
với luồng điều khiển chính: mỗi node rẽ nhánh có thể "để lại dấu vết" MÀ
không ảnh hưởng gì tới VIỆC nó chạy tiếp Ở đâu. Đây LÀ lý do MASTERPLAN
xếp "không audit" LÀ một chế độ hỏng RIÊNG so VỚI "không test từng nhánh"
(bài sau) — một hệ thống có audit trail vẫn có thể CHƯA từng test một
nhánh hiếm; VÀ một hệ thống test được từng nhánh vẫn có thể KHÔNG audit
được vì sao một lần chạy THẬT đã chọn nhánh đó. Hai khả năng nầy ĐỘC LẬP,
đo bằng những câu hỏi khác nhau: "test được không" so VỚI "biết được vì
sao không".
::::

::::checkpoint{mastery=0.86}
::::
