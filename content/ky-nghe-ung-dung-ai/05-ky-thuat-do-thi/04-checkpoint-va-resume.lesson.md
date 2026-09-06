---
id: ky-nghe-ung-dung-ai.ky-thuat-do-thi.checkpoint-va-resume
title: "Checkpoint và resume — không chạy lại từ đầu"
summary: "Checkpoint = { tenNodeTiepTheo: string; dsNodeDaDiTruocDo: string[] } lưu ĐÚNG hai thứ cần để resume: tên node CẦN chạy tiếp, VÀ dữ liệu (danh sách node) ĐÃ tích luỹ trước khi checkpoint được lưu. chayDoThiCoDemGoi mở rộng chayDoThi (bài 1) để đếm demGoiMoiNode[tenNode] MỖI lần một node THẬT SỰ được gọi. Trên đồ thị tuyến tính 5 node (buoc1→buoc2→buoc3→buoc4→buoc5): chạy TỪ ĐẦU (chayDoThiCoDemGoi(doThi5,'buoc1',...)) tốn ĐÚNG 5 lượt gọi (một lần mỗi node) VÀ cho dsNodeDaDi=['buoc1'..'buoc5']. Giả lập một checkpoint được lưu SAU khi buoc1,buoc2 đã chạy (checkpoint={tenNodeTiepTheo:'buoc3', dsNodeDaDiTruocDo:['buoc1','buoc2']}): chayDoThiTuCheckpoint resume ĐÚNG từ buoc3, chỉ tốn 3 lượt gọi MỚI (buoc3,buoc4,buoc5 — KHÔNG gọi lại buoc1/buoc2), NHƯNG dsNodeDaDi cuối cùng VẪN đầy đủ 5 node giống hệt chạy từ đầu — resume không đổi KẾT QUẢ, chỉ đổi SỐ LỆNH GỌI THẬT SỰ cần dùng (5 so với 3, tức N so với N-vị-trí-checkpoint với N=5, vị trí checkpoint=2)."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-do-thi
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [kna.checkpoint-va-resume]
requires: [kna.edge-coverage-do-nhanh-da-chay]
concepts: [kna.checkpoint-va-resume]
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
Ba bài trước xem đồ thị LÀ MỘT lần chạy trọn vẹn: bắt đầu Ở node gốc,
đi tới khi hết. Nhưng một hệ thống THẬT có thể gặp sự cố GIỮA CHỪNG —
mất điện, crash, hết ngân sách — VÀ câu hỏi quan trọng nhất LÚC ĐÓ LÀ:
có phải chạy LẠI TỪ ĐẦU không? Bài này dạy LÝ DO đồ thị có thể trả lời
"không": một `checkpoint` ghi lại ĐÚNG đủ thông tin để resume.
::::

::::explain{#checkpoint_la_gi}
Một `Checkpoint` LÀ một object TỐI GIẢN — CHỈ cần lưu HAI thứ: tên node
CẦN chạy TIẾP THEO (`tenNodeTiepTheo`), VÀ dữ liệu ĐÃ tích luỹ TRƯỚC khi
checkpoint được lưu (`dsNodeDaDiTruocDo` — danh sách các node ĐÃ đi qua).
`chayDoThiCoDemGoi` mở rộng `chayDoThi` (bài `1`) để đếm THÊM
`demGoiMoiNode` — MỘT bộ đếm CHUNG, tăng `1` MỖI lần một node THẬT SỰ
được gọi (`chay()` thực thi):

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

function taoDoThiTuyenTinh5(): DoThi {
  return {
    buoc1: taoNodeThanhCong("buoc2"),
    buoc2: taoNodeThanhCong("buoc3"),
    buoc3: taoNodeThanhCong("buoc4"),
    buoc4: taoNodeThanhCong("buoc5"),
    buoc5: taoNodeThanhCong(null),
  };
}

const demGoi: Record<string, number> = {};
const ketQua = chayDoThiCoDemGoi(taoDoThiTuyenTinh5(), "buoc1", demGoi);
console.log(JSON.stringify(ketQua.dsNodeDaDi), JSON.stringify(demGoi));
```

```text title=readonly
["buoc1","buoc2","buoc3","buoc4","buoc5"] {"buoc1":1,"buoc2":1,"buoc3":1,"buoc4":1,"buoc5":1}
```

Chạy TỪ ĐẦU trên đồ thị `5` node tuyến tính tốn ĐÚNG `5` lượt gọi — MỖI
node được gọi ĐÚNG `1` lần. `demGoiMoiNode` LÀ bằng chứng bằng SỐ cho
việc "node NÀO thật sự được thực thi" — không phải suy đoán từ
`dsNodeDaDi`.
::::

::::example{#resume_tu_checkpoint_giua_chung}
Giả lập một checkpoint được LƯU LẠI ngay SAU KHI `buoc1` VÀ `buoc2` đã
chạy xong (như thể hệ thống gặp sự cố NGAY SAU ĐÓ): checkpoint ghi
`tenNodeTiepTheo: "buoc3"` VÀ `dsNodeDaDiTruocDo: ["buoc1", "buoc2"]`.
`chayDoThiTuCheckpoint` resume TỪ ĐÓ — gọi `chayDoThiCoDemGoi` bắt đầu
Ở `checkpoint.tenNodeTiepTheo`, rồi GHÉP dữ liệu CŨ VÀO PHÍA TRƯỚC kết
quả MỚI:

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

function taoDoThiTuyenTinh5(): DoThi {
  return {
    buoc1: taoNodeThanhCong("buoc2"),
    buoc2: taoNodeThanhCong("buoc3"),
    buoc3: taoNodeThanhCong("buoc4"),
    buoc4: taoNodeThanhCong("buoc5"),
    buoc5: taoNodeThanhCong(null),
  };
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

const doThi5 = taoDoThiTuyenTinh5();

const demGoiTuDau: Record<string, number> = {};
const ketQuaTuDau = chayDoThiCoDemGoi(doThi5, "buoc1", demGoiTuDau);

const checkpointGiuaChung: Checkpoint = { tenNodeTiepTheo: "buoc3", dsNodeDaDiTruocDo: ["buoc1", "buoc2"] };
const demGoiTuCheckpoint: Record<string, number> = {};
const ketQuaTuCheckpoint = chayDoThiTuCheckpoint(doThi5, checkpointGiuaChung, demGoiTuCheckpoint);

console.log("tu dau:      ", JSON.stringify(ketQuaTuDau.dsNodeDaDi), "tong luot goi:", tinhTongSoLuotGoi(demGoiTuDau));
console.log("tu checkpoint:", JSON.stringify(ketQuaTuCheckpoint.dsNodeDaDi), "tong luot goi:", tinhTongSoLuotGoi(demGoiTuCheckpoint));
```

```text title=readonly
tu dau:       ["buoc1","buoc2","buoc3","buoc4","buoc5"] tong luot goi: 5
tu checkpoint: ["buoc1","buoc2","buoc3","buoc4","buoc5"] tong luot goi: 3
```

`dsNodeDaDi` CUỐI CÙNG giống HỆT nhau — cả hai cách đều cho ra ĐÚNG `5`
node theo ĐÚNG thứ tự. NHƯNG số lượt gọi THẬT SỰ khác XA nhau: chạy TỪ
ĐẦU tốn `5` lượt (`N` cho `N` node); resume TỪ checkpoint chỉ tốn `3`
lượt MỚI (`N` trừ vị trí checkpoint, `5 - 2 = 3`) — `buoc1` VÀ `buoc2`
KHÔNG hề được gọi lại, dù chúng VẪN xuất hiện trong kết quả cuối (nhờ
`dsNodeDaDiTruocDo`, không phải nhờ chạy lại).
::::

::::predict{#doan-checkpoint-ngay-truoc-cuoi commitOnce}
Nếu checkpoint được lưu NGAY TRƯỚC node CUỐI (`tenNodeTiepTheo: "buoc5"`,
`dsNodeDaDiTruocDo: ["buoc1", "buoc2", "buoc3", "buoc4"]`) — resume từ
đó tốn BAO NHIÊU lượt gọi MỚI, VÀ `dsNodeDaDi` cuối cùng CÓ đủ `5` node
không?

:::opt{correct}
Tốn ĐÚNG `1` lượt gọi MỚI (chỉ `buoc5`) — VÀ `dsNodeDaDi` cuối cùng VẪN
đủ `5` node (`4` node TRƯỚC checkpoint được GHÉP vào, CỘNG `1` node MỚI
vừa chạy), vì `chayDoThiTuCheckpoint` LUÔN nối `dsNodeDaDiTruocDo` VÀO
TRƯỚC kết quả mới, bất kể checkpoint được lưu Ở vị trí NÀO
:::
:::opt
Tốn `5` lượt gọi — resume LUÔN chạy lại TOÀN BỘ đồ thị để đảm bảo dữ
liệu nhất quán, bất kể checkpoint LÀ gì
::why
Nhầm rằng resume "chạy lại để chắc chắn" — nhưng `chayDoThiTuCheckpoint`
gọi `chayDoThiCoDemGoi` bắt đầu NGAY TỪ `checkpoint.tenNodeTiepTheo`, VÀ
`while` trong đó CHỈ lặp cho tới khi `tenNodeTiepTheo` LÀ `null` — không
hề có bước nào "quay lại kiểm tra từ đầu".

Chỗ lệch: `chayDoThiCoDemGoi(doThi, checkpoint.tenNodeTiepTheo, ...)`
nhận THẲNG tên node bắt đầu MỚI (`"buoc5"`) LÀM `tenNodeGoc` — nó không
hề biết (VÀ không cần biết) rằng CÒN có `buoc1`-`buoc4` phía trước.
::
:::
:::opt
`dsNodeDaDi` cuối cùng chỉ có `1` node (`"buoc5"`) — dữ liệu TRƯỚC
checkpoint bị MẤT vì không được `chayDoThiCoDemGoi` tính lại
::why
Nhầm rằng CHỈ dữ liệu do `chayDoThiCoDemGoi` TỰ TÍNH mới được giữ lại —
nhưng `chayDoThiTuCheckpoint` GHÉP tường minh `checkpoint.dsNodeDaDiTruocDo`
VÀO TRƯỚC kết quả mới (`[...checkpoint.dsNodeDaDiTruocDo,
...ketQuaMoi.dsNodeDaDi]`) — dữ liệu CŨ không hề bị tính LẠI, nó được
GIỮ NGUYÊN từ checkpoint.

Chỗ lệch: checkpoint chính LÀ NƠI dữ liệu "đã qua" được LƯU LẠI — mục
đích của nó LÀ để KHÔNG PHẢI tính lại phần đó, không phải để nó biến
mất.
::
:::
::::

::::code{#viet_checkpoint_resume}
Hoàn thiện `chayDoThiCoDemGoi` — ĐÚNG hình dạng `chayDoThi` (bài `1`)
NHƯNG THÊM: MỖI lần một node ĐƯỢC GỌI (trước khi gọi `chay()`), tăng
`demGoiMoiNode[tenHienTai]` lên `1` (dùng `?? 0` cho lần đầu). Hoàn
thiện `chayDoThiTuCheckpoint` — gọi `chayDoThiCoDemGoi(doThi,
checkpoint.tenNodeTiepTheo, demGoiMoiNode)`, rồi trả về kết quả đó
NHƯNG với `dsNodeDaDi` LÀ `checkpoint.dsNodeDaDiTruocDo` GHÉP TRƯỚC
`dsNodeDaDi` mới tính được.

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

function taoDoThiTuyenTinh5(): DoThi {
  return {
    buoc1: taoNodeThanhCong("buoc2"),
    buoc2: taoNodeThanhCong("buoc3"),
    buoc3: taoNodeThanhCong("buoc4"),
    buoc4: taoNodeThanhCong("buoc5"),
    buoc5: taoNodeThanhCong(null),
  };
}

type Checkpoint = { tenNodeTiepTheo: string; dsNodeDaDiTruocDo: string[] };

function tinhTongSoLuotGoi(demGoiMoiNode: Record<string, number>): number {
  return Object.values(demGoiMoiNode).reduce((acc, n) => acc + n, 0);
}

function chayDoThiCoDemGoi(
  doThi: DoThi,
  tenNodeGoc: string,
  demGoiMoiNode: Record<string, number>,
): KetQuaChayDoThi {
  ___
}

function chayDoThiTuCheckpoint(
  doThi: DoThi,
  checkpoint: Checkpoint,
  demGoiMoiNode: Record<string, number>,
): KetQuaChayDoThi {
  ___
}

const doThi5 = taoDoThiTuyenTinh5();

const demGoiTuDau: Record<string, number> = {};
const ketQuaTuDau = chayDoThiCoDemGoi(doThi5, "buoc1", demGoiTuDau);

const checkpointGiuaChung: Checkpoint = { tenNodeTiepTheo: "buoc3", dsNodeDaDiTruocDo: ["buoc1", "buoc2"] };
const demGoiTuCheckpoint: Record<string, number> = {};
const ketQuaTuCheckpoint = chayDoThiTuCheckpoint(doThi5, checkpointGiuaChung, demGoiTuCheckpoint);

console.log(
  JSON.stringify(ketQuaTuDau.dsNodeDaDi),
  tinhTongSoLuotGoi(demGoiTuDau),
  JSON.stringify(ketQuaTuCheckpoint.dsNodeDaDi),
  tinhTongSoLuotGoi(demGoiTuCheckpoint),
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

function taoDoThiTuyenTinh5(): DoThi {
  return {
    buoc1: taoNodeThanhCong("buoc2"),
    buoc2: taoNodeThanhCong("buoc3"),
    buoc3: taoNodeThanhCong("buoc4"),
    buoc4: taoNodeThanhCong("buoc5"),
    buoc5: taoNodeThanhCong(null),
  };
}

type Checkpoint = { tenNodeTiepTheo: string; dsNodeDaDiTruocDo: string[] };

function tinhTongSoLuotGoi(demGoiMoiNode: Record<string, number>): number {
  return Object.values(demGoiMoiNode).reduce((acc, n) => acc + n, 0);
}

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

function chayDoThiTuCheckpoint(
  doThi: DoThi,
  checkpoint: Checkpoint,
  demGoiMoiNode: Record<string, number>,
): KetQuaChayDoThi {
  const ketQuaMoi = chayDoThiCoDemGoi(doThi, checkpoint.tenNodeTiepTheo, demGoiMoiNode);
  return { ...ketQuaMoi, dsNodeDaDi: [...checkpoint.dsNodeDaDiTruocDo, ...ketQuaMoi.dsNodeDaDi] };
}

const doThi5 = taoDoThiTuyenTinh5();

const demGoiTuDau: Record<string, number> = {};
const ketQuaTuDau = chayDoThiCoDemGoi(doThi5, "buoc1", demGoiTuDau);

const checkpointGiuaChung: Checkpoint = { tenNodeTiepTheo: "buoc3", dsNodeDaDiTruocDo: ["buoc1", "buoc2"] };
const demGoiTuCheckpoint: Record<string, number> = {};
const ketQuaTuCheckpoint = chayDoThiTuCheckpoint(doThi5, checkpointGiuaChung, demGoiTuCheckpoint);

console.log(
  JSON.stringify(ketQuaTuDau.dsNodeDaDi),
  tinhTongSoLuotGoi(demGoiTuDau),
  JSON.stringify(ketQuaTuCheckpoint.dsNodeDaDi),
  tinhTongSoLuotGoi(demGoiTuCheckpoint),
);
```

```typescript title=test
if (JSON.stringify(ketQuaTuDau.dsNodeDaDi) !== JSON.stringify(["buoc1", "buoc2", "buoc3", "buoc4", "buoc5"])) {
  throw new Error("chay tu dau phai di qua dung 5 node theo dung thu tu");
}
if (tinhTongSoLuotGoi(demGoiTuDau) !== 5) throw new Error("chay tu dau phai ton DUNG 5 luot goi node (mot lan moi node)");

if (JSON.stringify(ketQuaTuCheckpoint.dsNodeDaDi) !== JSON.stringify(["buoc1", "buoc2", "buoc3", "buoc4", "buoc5"])) {
  throw new Error("chay tu checkpoint phai cho ra dsNodeDaDi CUOI CUNG giong het chay tu dau");
}
if (tinhTongSoLuotGoi(demGoiTuCheckpoint) !== 3) {
  throw new Error("chay tu checkpoint (bo qua buoc1, buoc2 DA chay truoc do) chi duoc ton DUNG 3 luot goi MOI");
}
if (demGoiTuCheckpoint["buoc1"] !== undefined || demGoiTuCheckpoint["buoc2"] !== undefined) {
  throw new Error("buoc1 va buoc2 KHONG duoc goi lai sau khi resume tu checkpoint");
}

const demGoiRieng: Record<string, number> = {};
const checkpointCuoi: Checkpoint = { tenNodeTiepTheo: "buoc5", dsNodeDaDiTruocDo: ["buoc1", "buoc2", "buoc3", "buoc4"] };
const ketQuaRieng = chayDoThiTuCheckpoint(doThi5, checkpointCuoi, demGoiRieng);
if (tinhTongSoLuotGoi(demGoiRieng) !== 1) throw new Error("resume tu checkpoint NGAY TRUOC node cuoi chi ton DUNG 1 luot goi");
if (JSON.stringify(ketQuaRieng.dsNodeDaDi) !== JSON.stringify(["buoc1", "buoc2", "buoc3", "buoc4", "buoc5"])) {
  throw new Error("dsNodeDaDi cuoi cung van phai la day du 5 node, ke ca phan TRUOC checkpoint");
}

const demGoiGoc: Record<string, number> = {};
chayDoThiCoDemGoi(doThi5, "buoc1", demGoiGoc);
if (demGoiGoc["buoc1"] !== 1) throw new Error("demGoiMoiNode phai tang DUNG 1 cho moi node duoc goi mot lan");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (chayDoThiCoDemGoi): DUNG HET hinh dang chayDoThi bai 1, nhung THEM dong demGoiMoiNode[tenHienTai] = (demGoiMoiNode[tenHienTai] ?? 0) + 1; NGAY SAU khi xac nhan node ton tai, TRUOC khi goi chay(). Cho hai (chayDoThiTuCheckpoint): goi chayDoThiCoDemGoi(doThi, checkpoint.tenNodeTiepTheo, demGoiMoiNode) luu vao mot bien, roi return spread object do nhung GHI DE truong dsNodeDaDi bang [...checkpoint.dsNodeDaDiTruocDo, ...ketQuaMoi.dsNodeDaDi]."
- kind: strategy
  body: "Cho dau: const dsNodeDaDi: string[] = []; let tenHienTai: string | null = tenNodeGoc; let soBuoc = 0; while (tenHienTai !== null && soBuoc < SO_BUOC_TOI_DA_AN_TOAN) { soBuoc++; const node: NodeXuLy | undefined = doThi[tenHienTai]; if (!node) return { thanhCong: false, loi: \"node_khong_ton_tai\", dsNodeDaDi }; demGoiMoiNode[tenHienTai] = (demGoiMoiNode[tenHienTai] ?? 0) + 1; dsNodeDaDi.push(tenHienTai); const kq: KetQuaNode = node.chay(); if (kq.trangThai === \"loi\") return { thanhCong: false, loi: \"loi_tai_node\", dsNodeDaDi }; tenHienTai = kq.tenNodeTiepTheo; } return { thanhCong: true, dsNodeDaDi }; Cho hai: const ketQuaMoi = chayDoThiCoDemGoi(doThi, checkpoint.tenNodeTiepTheo, demGoiMoiNode); return { ...ketQuaMoi, dsNodeDaDi: [...checkpoint.dsNodeDaDiTruocDo, ...ketQuaMoi.dsNodeDaDi] };"
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
  expect: "[\"buoc1\",\"buoc2\",\"buoc3\",\"buoc4\",\"buoc5\"] 5 [\"buoc1\",\"buoc2\",\"buoc3\",\"buoc4\",\"buoc5\"] 3"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`5` lượt gọi từ đầu, `3` lượt gọi từ checkpoint — cùng MỘT kết quả cuối
cùng. Đây LÀ khả năng KHÔNG hề có Ở LOOP (T9.4): một vòng lặp tuyến
tính KHÔNG có khái niệm "vị trí Ở giữa" để quay lại — nó CHỈ có "số bước
đã chạy". GRAPH tách RIÊNG được "vị trí" (tên node) khỏi "số lần thử",
nên có thể resume ĐÚNG. Bài sau lùi lại, nhìn CÙNG cấu trúc này qua lăng
kính lập trình hàm: fold trên một cây đệ quy.
::::

::::reflect{#nghi-lai}
Checkpoint hoạt động được vì HAI thứ nó lưu — tên node tiếp theo VÀ dữ
liệu tích luỹ — LÀ TẤT CẢ những gì `chayDoThiCoDemGoi` cần để tiếp tục:
hàm này KHÔNG hề quan tâm "làm sao tới được `tenNodeGoc`", nó chỉ cần
BIẾT `tenNodeGoc` LÀ gì. Đây LÀ một dạng của tính chất "không trạng thái
ẩn" (statelessness): mọi thông tin cần để resume nằm TRỌN VẸN trong
checkpoint, không phụ thuộc vào LỊCH SỬ đã tạo ra checkpoint đó. Khác
biệt VỚI LOOP (T9.4): một vòng lặp CHỈ có MỘT con số ("đã chạy bao nhiêu
bước") LÀM trạng thái — con số đó không đủ để "resume đúng CHỖ", vì nó
không phân biệt được VỊ TRÍ nào trong không gian bước; GRAPH lưu TÊN
NODE, một định danh RÕ RÀNG cho "đang Ở đâu", nên resume được CHÍNH XÁC.
::::

::::checkpoint{mastery=0.88}
::::
