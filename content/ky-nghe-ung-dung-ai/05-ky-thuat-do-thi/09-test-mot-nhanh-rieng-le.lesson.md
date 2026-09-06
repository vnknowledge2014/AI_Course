---
id: ky-nghe-ung-dung-ai.ky-thuat-do-thi.test-mot-nhanh-rieng-le
title: "Test một nhánh riêng lẻ — không cần chạy cả đồ thị"
summary: "chayNhanhRieng(doThi, tenNodeBatDau, tenNodeDich) chạy TỪ MỘT node GIỮA đồ thị, DỪNG NGAY khi đến MỘT node đích cụ thể (kiểm tenHienTai===tenNodeDich TRƯỚC KHI tra cứu/thực thi node đó) thay vì chạy tới hết như chayDoThi (bài 1) — cho phép kiểm MỘT nhánh (ví dụ nhánh xử lý lỗi) MÀ KHÔNG cần dàn dựng lại điều kiện từ node gốc. Trên doThiTest=taoDoThiDayDu(999, false) (giaTri=999 sẽ đi 'xuLyHopLe' NẾU chạy từ gốc 'kiemTra' — nhưng KHÔNG liên quan khi test bắt đầu Ở giữa): chayNhanhRieng(doThiTest,'xuLyLoi','boCuoc') cho {daToiDich:true, dsNodeDaDi:['xuLyLoi','boCuoc']} — kiểm được nhánh boCuoc dù giaTri=999 sẽ KHÔNG BAO GIỜ tự nhiên đi tới đó nếu chạy từ gốc. chayNhanhRieng(doThiTest,'xuLyLoi','thuLai') cho {daToiDich:false, lyDoDung:'khong_toi_dich', dsNodeDaDi:['xuLyLoi','boCuoc']} — đường đi thật không chạm đích mong muốn. chayNhieuKichBanNhanhRieng chạy 3 kịch bản (đích boCuoc/thuLai/xuLyHopLe) cho [true,false,true]. Nếu tenNodeBatDau TRÙNG tenNodeDich, kết quả LÀ daToiDich=true NGAY LẬP TỨC với dsNodeDaDi CHỈ 1 phần tử, không thực thi chay() của node đó."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-do-thi
order: 9
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [kna.test-mot-nhanh-rieng-le]
requires: [kna.audit-trail-ghi-lai-vi-sao-re-nhanh]
concepts: [kna.test-mot-nhanh-rieng-le]
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
Bài `3` đo edge coverage: một nhánh HIẾM có thể mãi mãi Ở `0%` nếu bộ
kịch bản KHÔNG BAO GIỜ đưa ra dữ liệu dẫn tới nó. Nhưng biết "nhánh này
CHƯA từng được thử" chỉ LÀ nửa vấn đề — nửa CÒN LẠI LÀ: làm sao TEST
đúng nhánh đó, MÀ KHÔNG phải dàn dựng lại toàn bộ điều kiện đầu vào từ
node gốc? Đây LÀ lỗ hổng "không test từng nhánh" MÀ MASTERPLAN liệt kê.
::::

::::explain{#chay_tu_giua_dung_o_dich}
`chayNhanhRieng(doThi, tenNodeBatDau, tenNodeDich)` chạy y hệt `chayDoThi`
(bài `1`) NHƯNG với MỘT khác biệt cốt lõi: TRƯỚC KHI tra cứu VÀ thực thi
node hiện tại, nó kiểm `tenHienTai === tenNodeDich` — NẾU khớp, DỪNG
NGAY, ghi nhận ĐÃ đến đích, KHÔNG chạy `chay()` của node đó. Đây LÀ cách
"test một nhánh": bắt đầu Ở BẤT KỲ node nào GIỮA đồ thị (không cần chạy
từ gốc), VÀ chỉ cần biết nó có ĐI TỚI một node đích cụ thể hay không:

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

type KetQuaChayNhanhRieng =
  | { daToiDich: true; dsNodeDaDi: string[] }
  | { daToiDich: false; lyDoDung: "khong_toi_dich" | "loi_tai_node" | "node_khong_ton_tai"; dsNodeDaDi: string[] };

function chayNhanhRieng(doThi: DoThi, tenNodeBatDau: string, tenNodeDich: string): KetQuaChayNhanhRieng {
  const dsNodeDaDi: string[] = [];
  let tenHienTai: string | null = tenNodeBatDau;
  let soBuoc = 0;
  while (tenHienTai !== null && soBuoc < SO_BUOC_TOI_DA_AN_TOAN) {
    soBuoc++;
    if (tenHienTai === tenNodeDich) {
      dsNodeDaDi.push(tenHienTai);
      return { daToiDich: true, dsNodeDaDi };
    }
    const node: NodeXuLy | undefined = doThi[tenHienTai];
    if (!node) return { daToiDich: false, lyDoDung: "node_khong_ton_tai", dsNodeDaDi };
    dsNodeDaDi.push(tenHienTai);
    const kq: KetQuaNode = node.chay();
    if (kq.trangThai === "loi") return { daToiDich: false, lyDoDung: "loi_tai_node", dsNodeDaDi };
    tenHienTai = kq.tenNodeTiepTheo;
  }
  return { daToiDich: false, lyDoDung: "khong_toi_dich", dsNodeDaDi };
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

const doThiTest = taoDoThiDayDu(999, false);
console.log(JSON.stringify(chayNhanhRieng(doThiTest, "xuLyLoi", "boCuoc")));
```

```text title=readonly
{"daToiDich":true,"dsNodeDaDi":["xuLyLoi","boCuoc"]}
```

Chú ý `giaTri=999` — NẾU chạy `chayDoThi` từ node GỐC `"kiemTra"`, đường
đi thật sẽ đi `"xuLyHopLe"` (vì `999 >= 0`), KHÔNG BAO GIỜ chạm
`"xuLyLoi"`/`"boCuoc"`. Nhưng `chayNhanhRieng` bắt đầu THẲNG Ở
`"xuLyLoi"` — `giaTri` KHÔNG còn liên quan (node `"kiemTra"` không hề
được chạy), CHỈ `coTheThuLai=false` (đã đóng gói trong `doThiTest` qua
closure) quyết định đường đi TỪ đó. Đây chính LÀ điều cho phép test một
nhánh HIẾM (ví dụ nhánh lỗi) MÀ KHÔNG cần dàn dựng lại `giaTri` phù hợp
Ở node gốc.
::::

::::example{#duong_di_that_khong_cham_dich_mong_muon}
Cùng `doThiTest`, đổi `tenNodeDich` thành `"thuLai"` (một node KHÁC với
đường đi thật sự Ở `coTheThuLai=false`):

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

type KetQuaChayNhanhRieng =
  | { daToiDich: true; dsNodeDaDi: string[] }
  | { daToiDich: false; lyDoDung: "khong_toi_dich" | "loi_tai_node" | "node_khong_ton_tai"; dsNodeDaDi: string[] };

function chayNhanhRieng(doThi: DoThi, tenNodeBatDau: string, tenNodeDich: string): KetQuaChayNhanhRieng {
  const dsNodeDaDi: string[] = [];
  let tenHienTai: string | null = tenNodeBatDau;
  let soBuoc = 0;
  while (tenHienTai !== null && soBuoc < SO_BUOC_TOI_DA_AN_TOAN) {
    soBuoc++;
    if (tenHienTai === tenNodeDich) {
      dsNodeDaDi.push(tenHienTai);
      return { daToiDich: true, dsNodeDaDi };
    }
    const node: NodeXuLy | undefined = doThi[tenHienTai];
    if (!node) return { daToiDich: false, lyDoDung: "node_khong_ton_tai", dsNodeDaDi };
    dsNodeDaDi.push(tenHienTai);
    const kq: KetQuaNode = node.chay();
    if (kq.trangThai === "loi") return { daToiDich: false, lyDoDung: "loi_tai_node", dsNodeDaDi };
    tenHienTai = kq.tenNodeTiepTheo;
  }
  return { daToiDich: false, lyDoDung: "khong_toi_dich", dsNodeDaDi };
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

const doThiTest = taoDoThiDayDu(999, false);
console.log(JSON.stringify(chayNhanhRieng(doThiTest, "xuLyLoi", "thuLai")));
```

```text title=readonly
{"daToiDich":false,"lyDoDung":"khong_toi_dich","dsNodeDaDi":["xuLyLoi","boCuoc"]}
```

`daToiDich` LÀ `false` — đường đi THẬT (`"xuLyLoi"` → `"boCuoc"` → `null`,
vì `coTheThuLai=false`) KHÔNG hề chạm `"thuLai"`. `lyDoDung` ghi
`"khong_toi_dich"` — khác HẲN `"loi_tai_node"` (node báo lỗi) VÀ
`"node_khong_ton_tai"` (tên sai): đây LÀ một bài test đã CHẠY XONG bình
thường, chỉ LÀ kết quả không phải đích ta ĐANG kiểm.
::::

::::predict{#doan-bat-dau-trung-dich commitOnce}
Nếu `tenNodeBatDau` VÀ `tenNodeDich` LÀ CÙNG một tên (ví dụ CẢ HAI đều
LÀ `"xuLyLoi"`) — `chayNhanhRieng` trả về gì?

:::opt{correct}
`daToiDich: true` NGAY LẬP TỨC, VỚI `dsNodeDaDi` CHỈ đúng `1` phần tử
(`["xuLyLoi"]`) — vòng lặp kiểm `tenHienTai === tenNodeDich` Ở VÒNG ĐẦU
TIÊN, TRƯỚC KHI tra cứu HAY thực thi `chay()` của node đó, nên node
`"xuLyLoi"` KHÔNG hề được chạy
:::
:::opt
`daToiDich: false` VỚI `lyDoDung: "khong_toi_dich"` — bắt đầu VÀ đích
trùng nhau LÀ một trường hợp KHÔNG hợp lệ
::why
Nhầm rằng bắt đầu VÀ đích trùng nhau LÀ lỗi đầu VÀO — nhưng thân hàm
KHÔNG có kiểm tra NÀO loại trừ trường hợp này; nó chỉ chạy vòng `while`
BÌNH THƯỜNG, VÀ Ở vòng ĐẦU TIÊN, điều kiện `tenHienTai === tenNodeDich`
LUÔN đúng (vì `tenHienTai` khởi tạo bằng `tenNodeBatDau`).

Chỗ lệch: không có nhánh code NÀO xử lý "bắt đầu = đích" LÀ một trường
hợp đặc biệt cần TỪ CHỐI — nó CHỈ đơn thuần LÀ trường hợp "đã Ở đích
NGAY từ bước đầu".
::
:::
:::opt
`daToiDich: true`, NHƯNG `dsNodeDaDi` VẪN chứa TOÀN BỘ đường đi (chạy hết
`chay()` của `"xuLyLoi"` rồi mới dừng, vì tên đã "đi qua" node đó)
::why
Nhầm rằng "đã đi qua" nghĩa LÀ PHẢI thực thi `chay()` — nhưng thân hàm
kiểm `tenHienTai === tenNodeDich` TRƯỚC dòng gọi `doThi[tenHienTai]`
VÀ TRƯỚC dòng gọi `node.chay()`; khi điều kiện đó đúng, hàm `return` NGAY,
không hề chạm tới hai dòng đó.

Chỗ lệch: thứ tự CÁC DÒNG lệnh bên trong `while` LÀ: (1) kiểm đích, (2)
tra node, (3) gọi `chay()` — dừng Ở bước `(1)` nghĩa LÀ bước `(2)` VÀ
`(3)` không bao giờ chạy tới.
::
:::
::::

::::code{#viet_chay_nhanh_rieng}
Hoàn thiện `chayNhanhRieng` — vòng `while` (tối đa `SO_BUOC_TOI_DA_AN_TOAN`
bước), bắt đầu từ `tenNodeBatDau`; MỖI vòng: NẾU `tenHienTai ===
tenNodeDich`, đẩy tên đó vào `dsNodeDaDi` rồi trả `{ daToiDich: true,
dsNodeDaDi }` NGAY (KHÔNG tra cứu HAY gọi `chay()`); NGƯỢC LẠI tra node
theo tên, NẾU không tồn tại trả `{ daToiDich: false, lyDoDung:
"node_khong_ton_tai", dsNodeDaDi }`; nếu có, đẩy tên VÀO `dsNodeDaDi`,
gọi `chay()`, NẾU `"loi"` trả `{ daToiDich: false, lyDoDung:
"loi_tai_node", dsNodeDaDi }`; NGƯỢC LẠI đi tới `tenNodeTiepTheo`; hết
vòng lặp (hết bước an toàn mà CHƯA `return`) trả `{ daToiDich: false,
lyDoDung: "khong_toi_dich", dsNodeDaDi }`. Hoàn thiện
`chayNhieuKichBanNhanhRieng` — với MỖI phần tử trong `dsKichBan`, gọi
`chayNhanhRieng` VỚI đúng ba trường của nó, đẩy kết quả vào mảng trả về.

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

type KetQuaChayNhanhRieng =
  | { daToiDich: true; dsNodeDaDi: string[] }
  | { daToiDich: false; lyDoDung: "khong_toi_dich" | "loi_tai_node" | "node_khong_ton_tai"; dsNodeDaDi: string[] };

function chayNhanhRieng(doThi: DoThi, tenNodeBatDau: string, tenNodeDich: string): KetQuaChayNhanhRieng {
  ___
}

type KichBanNhanhRieng = { doThi: DoThi; tenNodeBatDau: string; tenNodeDich: string };

function chayNhieuKichBanNhanhRieng(dsKichBan: KichBanNhanhRieng[]): KetQuaChayNhanhRieng[] {
  ___
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

const doThiTest = taoDoThiDayDu(999, false);
const ketQuaBaKichBan = chayNhieuKichBanNhanhRieng([
  { doThi: doThiTest, tenNodeBatDau: "xuLyLoi", tenNodeDich: "boCuoc" },
  { doThi: doThiTest, tenNodeBatDau: "xuLyLoi", tenNodeDich: "thuLai" },
  { doThi: doThiTest, tenNodeBatDau: "kiemTra", tenNodeDich: "xuLyHopLe" },
]);
console.log(JSON.stringify(ketQuaBaKichBan.map((k) => k.daToiDich)));
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

type KetQuaChayNhanhRieng =
  | { daToiDich: true; dsNodeDaDi: string[] }
  | { daToiDich: false; lyDoDung: "khong_toi_dich" | "loi_tai_node" | "node_khong_ton_tai"; dsNodeDaDi: string[] };

function chayNhanhRieng(doThi: DoThi, tenNodeBatDau: string, tenNodeDich: string): KetQuaChayNhanhRieng {
  const dsNodeDaDi: string[] = [];
  let tenHienTai: string | null = tenNodeBatDau;
  let soBuoc = 0;
  while (tenHienTai !== null && soBuoc < SO_BUOC_TOI_DA_AN_TOAN) {
    soBuoc++;
    if (tenHienTai === tenNodeDich) {
      dsNodeDaDi.push(tenHienTai);
      return { daToiDich: true, dsNodeDaDi };
    }
    const node: NodeXuLy | undefined = doThi[tenHienTai];
    if (!node) return { daToiDich: false, lyDoDung: "node_khong_ton_tai", dsNodeDaDi };
    dsNodeDaDi.push(tenHienTai);
    const kq: KetQuaNode = node.chay();
    if (kq.trangThai === "loi") return { daToiDich: false, lyDoDung: "loi_tai_node", dsNodeDaDi };
    tenHienTai = kq.tenNodeTiepTheo;
  }
  return { daToiDich: false, lyDoDung: "khong_toi_dich", dsNodeDaDi };
}

type KichBanNhanhRieng = { doThi: DoThi; tenNodeBatDau: string; tenNodeDich: string };

function chayNhieuKichBanNhanhRieng(dsKichBan: KichBanNhanhRieng[]): KetQuaChayNhanhRieng[] {
  const ketQua: KetQuaChayNhanhRieng[] = [];
  for (const kb of dsKichBan) {
    ketQua.push(chayNhanhRieng(kb.doThi, kb.tenNodeBatDau, kb.tenNodeDich));
  }
  return ketQua;
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

const doThiTest = taoDoThiDayDu(999, false);
const ketQuaBaKichBan = chayNhieuKichBanNhanhRieng([
  { doThi: doThiTest, tenNodeBatDau: "xuLyLoi", tenNodeDich: "boCuoc" },
  { doThi: doThiTest, tenNodeBatDau: "xuLyLoi", tenNodeDich: "thuLai" },
  { doThi: doThiTest, tenNodeBatDau: "kiemTra", tenNodeDich: "xuLyHopLe" },
]);
console.log(JSON.stringify(ketQuaBaKichBan.map((k) => k.daToiDich)));
```

```typescript title=test
if (ketQuaBaKichBan.length !== 3) throw new Error("chayNhieuKichBanNhanhRieng phai tra ve mang dung 3 phan tu");
if (JSON.stringify(ketQuaBaKichBan.map((k) => k.daToiDich)) !== JSON.stringify([true, false, true])) {
  throw new Error("ba kich ban phai cho dung [true,false,true]");
}

const kbBoCuoc = ketQuaBaKichBan[0]!;
if (kbBoCuoc.daToiDich === true && JSON.stringify(kbBoCuoc.dsNodeDaDi) !== JSON.stringify(["xuLyLoi", "boCuoc"])) {
  throw new Error("kich ban 1 (dich boCuoc) phai co dsNodeDaDi la ['xuLyLoi','boCuoc']");
}

const kbThuLai = ketQuaBaKichBan[1]!;
if (kbThuLai.daToiDich !== false) throw new Error("kich ban 2 (dich thuLai, duong that di boCuoc) phai KHONG toi dich");
if (kbThuLai.daToiDich === false && kbThuLai.lyDoDung !== "khong_toi_dich") {
  throw new Error("kich ban 2 phai co lyDoDung la 'khong_toi_dich'");
}

const ktTrungTen = chayNhanhRieng(doThiTest, "xuLyLoi", "xuLyLoi");
if (ktTrungTen.daToiDich !== true) throw new Error("bat dau trung dich phai daToiDich=true NGAY LAP TUC");
if (JSON.stringify(ktTrungTen.dsNodeDaDi) !== JSON.stringify(["xuLyLoi"])) {
  throw new Error("bat dau trung dich chi duoc ghi DUNG 1 phan tu, khong chay tiep");
}

const ktKhongTonTai = chayNhanhRieng(doThiTest, "tenKhongCoThat", "boCuoc");
if (ktKhongTonTai.daToiDich !== false) throw new Error("bat dau tu ten khong ton tai phai that bai");
if (ktKhongTonTai.daToiDich === false && ktKhongTonTai.lyDoDung !== "node_khong_ton_tai") {
  throw new Error("bat dau tu ten khong ton tai phai co lyDoDung la 'node_khong_ton_tai'");
}

const motKichBan = chayNhieuKichBanNhanhRieng([
  { doThi: doThiTest, tenNodeBatDau: "kiemTra", tenNodeDich: "boCuoc" },
]);
if (motKichBan.length !== 1) throw new Error("doi so danh sach kich ban phai doi do dai mang tra ve -- tham so phai duoc dung that");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (chayNhanhRieng): trong than while, KIEM tenHienTai === tenNodeDich TRUOC TIEN -- neu dung thi day vao dsNodeDaDi roi return daToiDich:true NGAY; chi khi KHONG khop moi tra doThi[tenHienTai], xu ly node_khong_ton_tai / loi_tai_node / di toi tenNodeTiepTheo giong het chayDoThi bai 1; het vong lap tra khong_toi_dich. Cho hai (chayNhieuKichBanNhanhRieng): mot vong for-of qua dsKichBan, moi phan tu goi chayNhanhRieng voi ba truong cua no, day ket qua vao mot mang."
- kind: strategy
  body: "Cho dau: const dsNodeDaDi: string[] = []; let tenHienTai: string | null = tenNodeBatDau; let soBuoc = 0; while (tenHienTai !== null && soBuoc < SO_BUOC_TOI_DA_AN_TOAN) { soBuoc++; if (tenHienTai === tenNodeDich) { dsNodeDaDi.push(tenHienTai); return { daToiDich: true, dsNodeDaDi }; } const node: NodeXuLy | undefined = doThi[tenHienTai]; if (!node) return { daToiDich: false, lyDoDung: 'node_khong_ton_tai', dsNodeDaDi }; dsNodeDaDi.push(tenHienTai); const kq: KetQuaNode = node.chay(); if (kq.trangThai === 'loi') return { daToiDich: false, lyDoDung: 'loi_tai_node', dsNodeDaDi }; tenHienTai = kq.tenNodeTiepTheo; } return { daToiDich: false, lyDoDung: 'khong_toi_dich', dsNodeDaDi }; Cho hai: const ketQua: KetQuaChayNhanhRieng[] = []; for (const kb of dsKichBan) { ketQua.push(chayNhanhRieng(kb.doThi, kb.tenNodeBatDau, kb.tenNodeDich)); } return ketQua;"
- kind: one-line
  body: "Sao chep dung hai khoi o phan Strategy vao dung vi tri tuong ung -- CHU Y kiem dich TRUOC khi tra node."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "[true,false,true]"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`giaTri=999` không còn liên quan khi test bắt đầu Ở GIỮA đồ thị — chỉ cần
neo đúng node VÀ dữ liệu closure Ở CHỖ cần kiểm. Nhánh hiếm giờ TEST được
TRỰC TIẾP, không cần dàn dựng input phức tạp từ gốc. Bài sau xử lý một
hình dạng đồ thị hoàn toàn khác: khi một cạnh quay LẠI một node ĐàI QUA
rồi — đồ thị không phải LUÔN LÀ một cây.
::::

::::reflect{#nghi-lai}
`chayNhanhRieng` không thêm khả năng CHẠY mới — nó VẪN LÀ một vòng `while`
đi theo `tenNodeTiepTheo`, giống HỆT `chayDoThi` (bài `1`). Điều khác biệt
DUY NHẤT LÀ điều kiện DỪNG: `chayDoThi` dừng khi `tenNodeTiepTheo` LÀ
`null`; `chayNhanhRieng` dừng SỚM HƠN, ngay khi CHẠM một node đích do
NGƯỜI TEST chỉ định. Đây chính LÀ điều khiến một nhánh "hiếm khi được
chạm tới" (bài `3`, edge coverage thấp) TRỞ NÊN test được: thay vì phải
tìm MỘT bộ input Ở node GỐC dẫn đúng qua toàn bộ chuỗi rẽ nhánh để tới
được nhánh đó, ta CHỈ cần neo `doThi` (đã đóng gói dữ liệu closure đúng
Ở nhánh cần) VÀ bắt đầu THẲNG Ở node ngay TRƯỚC nhánh đó. MASTERPLAN gọi
đây LÀ chữa lỗi "không test từng nhánh" — VÀ nó hoạt động được chính VÌ
`DoThi` LÀ một object DỮ LIỆU (`Record<string, NodeXuLy>`), không phải
một chuỗi lệnh gọi hàm lồng nhau chỉ chạy được từ ĐẦU.
::::

::::checkpoint{mastery=0.87}
::::
