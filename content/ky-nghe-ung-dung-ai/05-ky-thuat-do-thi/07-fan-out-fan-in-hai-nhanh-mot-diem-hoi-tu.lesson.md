---
id: ky-nghe-ung-dung-ai.ky-thuat-do-thi.fan-out-fan-in-hai-nhanh-mot-diem-hoi-tu
title: "Fan-out / fan-in — hai nhánh độc lập, một điểm hội tụ"
summary: "chayFanOutFanIn(doThiA, tenGocA, doThiB, tenGocB) mô phỏng 'chờ song song' bằng cách gọi TUẦN TỰ: chạy trọn nhánh A qua chayDoThi (bài 1), rồi chạy TRỌN nhánh B — KHÔNG có async/await/Promise thật, VÀ CẢ HAI nhánh LUÔN được chạy tới hết BẤT KỂ nhánh kia thành công hay lỗi (không có 'dừng sớm' khi phát hiện một nhánh lỗi). Trên doThiNhanhA (2 node: nhanhA_buoc1->nhanhA_buoc2->null) và doThiNhanhB (1 node: nhanhB_buoc1->null): khi cả hai thành công, thanhCong=true, dsNodeDaDiNhanhA=['nhanhA_buoc1','nhanhA_buoc2'], dsNodeDaDiNhanhB=['nhanhB_buoc1']. Khi nhánh B lỗi (đổi thành taoNodeLoi()), thanhCong=false, nhanhLoi=['B'] — NHƯNG dsNodeDaDiNhanhA VẪN đầy đủ 2 phần tử (nhánh A đã chạy TRỌN VẸN dù B lỗi, không bị huỷ giữa chừng). chayNhieuCapNhanh chạy 3 cặp nhánh liên tiếp cho ra [true,false,false] — cặp 2 (B lỗi) có nhanhLoi=['B'], cặp 3 (A lỗi) có nhanhLoi=['A']; khi CẢ HAI nhánh cùng lỗi, nhanhLoi=['A','B']."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-do-thi
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [kna.fan-out-fan-in-hai-nhanh-mot-diem-hoi-tu]
requires: [kna.boss-do-thi-tu-dau-vs-checkpoint-resume]
concepts: [kna.fan-out-fan-in-hai-nhanh-mot-diem-hoi-tu]
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
q9.5a khép lại ở 6/6: node+edge, rẽ nhánh, edge coverage, checkpoint/resume,
catamorphism. Nhưng MASTERPLAN liệt kê GRAPH phải chữa BỐN chế độ hỏng —
"không resume/không song song/không audit/không test từng nhánh" — VÀ
q9.5a mới chạm ĐÚNG một, rưỡi. q9.5b bắt đầu Ở "không song song": một
node kích hoạt HAI nhánh ĐỘC LẬP, chờ CẢ HAI xong rồi mới đi tiếp.
::::

::::explain{#fan_out_fan_in_mo_phong_tuan_tu}
"Fan-out" LÀ một điểm rẽ ra NHIỀU nhánh con chạy ĐỘC LẬP (không chia sẻ
trạng thái); "fan-in" (join) LÀ điểm CHỜ tất cả nhánh đó xong rồi mới hội
tụ VỀ một kết quả DUY NHẤT. Dự án này CẤM `async`/`await`/`Promise` thật
(xem gotcha #6), nên "chờ song song" được mô phỏng bằng cách gọi TUẦN TỰ:
chạy TRỌN nhánh A (qua `chayDoThi`, bài `1`), rồi chạy TRỌN nhánh B —
VÀ ĐIỂM MẤU CHỐT: CẢ HAI nhánh LUÔN được chạy tới hết, BẤT KỂ nhánh kia
thành công hay lỗi. Đây khác VỚI một `if` dừng sớm khi gặp lỗi — fan-out
THẬT không "huỷ" nhánh còn lại giữa chừng chỉ vì nhánh kia đã hỏng, vì
Ở đời thực hai nhánh chạy CÙNG lúc, không nhánh nào "biết" nhánh kia lỗi
cho tới khi CẢ HAI đã xong:

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

type KetQuaFanOutFanIn =
  | { thanhCong: true; dsNodeDaDiNhanhA: string[]; dsNodeDaDiNhanhB: string[] }
  | { thanhCong: false; nhanhLoi: ("A" | "B")[]; dsNodeDaDiNhanhA: string[]; dsNodeDaDiNhanhB: string[] };

function chayFanOutFanIn(
  doThiNhanhA: DoThi, tenGocNhanhA: string,
  doThiNhanhB: DoThi, tenGocNhanhB: string,
): KetQuaFanOutFanIn {
  const ketQuaA = chayDoThi(doThiNhanhA, tenGocNhanhA);
  const ketQuaB = chayDoThi(doThiNhanhB, tenGocNhanhB);
  if (ketQuaA.thanhCong && ketQuaB.thanhCong) {
    return { thanhCong: true, dsNodeDaDiNhanhA: ketQuaA.dsNodeDaDi, dsNodeDaDiNhanhB: ketQuaB.dsNodeDaDi };
  }
  const nhanhLoi: ("A" | "B")[] = [];
  if (!ketQuaA.thanhCong) nhanhLoi.push("A");
  if (!ketQuaB.thanhCong) nhanhLoi.push("B");
  return { thanhCong: false, nhanhLoi, dsNodeDaDiNhanhA: ketQuaA.dsNodeDaDi, dsNodeDaDiNhanhB: ketQuaB.dsNodeDaDi };
}

const doThiNhanhA: DoThi = {
  nhanhA_buoc1: taoNodeThanhCong("nhanhA_buoc2"),
  nhanhA_buoc2: taoNodeThanhCong(null),
};
const doThiNhanhB: DoThi = {
  nhanhB_buoc1: taoNodeThanhCong(null),
};

console.log(JSON.stringify(chayFanOutFanIn(doThiNhanhA, "nhanhA_buoc1", doThiNhanhB, "nhanhB_buoc1")));
```

```text title=readonly
{"thanhCong":true,"dsNodeDaDiNhanhA":["nhanhA_buoc1","nhanhA_buoc2"],"dsNodeDaDiNhanhB":["nhanhB_buoc1"]}
```

`chayFanOutFanIn` gọi ĐÚNG `chayDoThi` HAI lần — một cho `doThiNhanhA`,
một cho `doThiNhanhB` — KHÔNG có cơ chế bất đồng bộ nào; "song song" Ở đây
CHỈ LÀ ẩn dụ cho "hai nhánh KHÔNG chia sẻ trạng thái, kết quả của nhánh
này KHÔNG ảnh hưởng cách nhánh kia chạy". Điểm "gộp" (join) nằm NGAY tại
dòng `if (ketQuaA.thanhCong && ketQuaB.thanhCong)` — CẢ HAI phải thành
công thì tổng thể mới thành công.
::::

::::example{#nhanh_b_loi_nhung_nhanh_a_van_chay_het}
Đổi `doThiNhanhB` thành một đồ thị BÁO LỖI (`nhanhB_buoc1` dùng
`taoNodeLoi()` thay vì `taoNodeThanhCong`) — giữ NGUYÊN `doThiNhanhA`:

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

type KetQuaFanOutFanIn =
  | { thanhCong: true; dsNodeDaDiNhanhA: string[]; dsNodeDaDiNhanhB: string[] }
  | { thanhCong: false; nhanhLoi: ("A" | "B")[]; dsNodeDaDiNhanhA: string[]; dsNodeDaDiNhanhB: string[] };

function chayFanOutFanIn(
  doThiNhanhA: DoThi, tenGocNhanhA: string,
  doThiNhanhB: DoThi, tenGocNhanhB: string,
): KetQuaFanOutFanIn {
  const ketQuaA = chayDoThi(doThiNhanhA, tenGocNhanhA);
  const ketQuaB = chayDoThi(doThiNhanhB, tenGocNhanhB);
  if (ketQuaA.thanhCong && ketQuaB.thanhCong) {
    return { thanhCong: true, dsNodeDaDiNhanhA: ketQuaA.dsNodeDaDi, dsNodeDaDiNhanhB: ketQuaB.dsNodeDaDi };
  }
  const nhanhLoi: ("A" | "B")[] = [];
  if (!ketQuaA.thanhCong) nhanhLoi.push("A");
  if (!ketQuaB.thanhCong) nhanhLoi.push("B");
  return { thanhCong: false, nhanhLoi, dsNodeDaDiNhanhA: ketQuaA.dsNodeDaDi, dsNodeDaDiNhanhB: ketQuaB.dsNodeDaDi };
}

const doThiNhanhA: DoThi = {
  nhanhA_buoc1: taoNodeThanhCong("nhanhA_buoc2"),
  nhanhA_buoc2: taoNodeThanhCong(null),
};
const doThiNhanhBLoi: DoThi = {
  nhanhB_buoc1: taoNodeLoi(),
};

console.log(JSON.stringify(chayFanOutFanIn(doThiNhanhA, "nhanhA_buoc1", doThiNhanhBLoi, "nhanhB_buoc1")));
```

```text title=readonly
{"thanhCong":false,"nhanhLoi":["B"],"dsNodeDaDiNhanhA":["nhanhA_buoc1","nhanhA_buoc2"],"dsNodeDaDiNhanhB":["nhanhB_buoc1"]}
```

`thanhCong` LÀ `false`, `nhanhLoi` chỉ ghi `["B"]` — NHƯNG `dsNodeDaDiNhanhA`
VẪN đủ HAI phần tử `["nhanhA_buoc1","nhanhA_buoc2"]`, y hệt lần chạy
thành công Ở trên. Nhánh A KHÔNG hề bị "huỷ giữa chừng" chỉ vì B lỗi —
nó ĐÃ chạy TRỌN VẸN TRƯỚC KHI `chayFanOutFanIn` biết B lỗi (vì lệnh gọi
`chayDoThi(doThiNhanhA, ...)` đứng TRƯỚC, VÀ không có nhánh nào kiểm tra
kết quả của nhánh kia GIỮA hai lệnh gọi đó).
::::

::::predict{#doan-nhanh-a-loi-b-co-chay-khong commitOnce}
Đổi ngược lại: `doThiNhanhA` báo lỗi (`nhanhA_buoc1` dùng `taoNodeLoi()`),
`doThiNhanhB` VẪN thành công bình thường. Nhánh B có được chạy ĐẦY ĐỦ
không, VÀ `nhanhLoi` ghi gì?

:::opt{correct}
Nhánh B VẪN được chạy ĐẦY ĐỦ (`dsNodeDaDiNhanhB` đủ mọi node của nó) —
`chayFanOutFanIn` gọi `chayDoThi(doThiNhanhB, ...)` KHÔNG điều kiện, bất
kể `ketQuaA` LÀ gì; `nhanhLoi` chỉ ghi `["A"]`, vì CHỈ nhánh A thất bại
:::
:::opt
Nhánh B sẽ KHÔNG được chạy — `chayFanOutFanIn` dừng NGAY khi thấy nhánh A
lỗi, giống một `if/else` rẽ nhánh sớm
::why
Nhầm rằng fan-out "dừng sớm" giống một `if` rẽ nhánh — nhưng trong thân
`chayFanOutFanIn`, dòng `const ketQuaB = chayDoThi(doThiNhanhB, ...)` đứng
NGAY SAU dòng gọi nhánh A, KHÔNG có `if` nào đứng GIỮA hai dòng đó để
"bỏ qua" nhánh B dựa trên kết quả của A.

Chỗ lệch: hai lệnh gọi `chayDoThi` cho nhánh A VÀ nhánh B LÀ hai câu lệnh
TUẦN TỰ, KHÔNG PHỤ THUỘC VÀO NHAU — mô phỏng đúng ý "hai nhánh chạy CÙNG
lúc trong đời thực, không nhánh nào biết nhánh kia đã xong hay chưa".
::
:::
:::opt
`nhanhLoi` sẽ ghi CẢ `["A","B"]`, vì lỗi Ở nhánh A "lan" sang nhánh B
::why
Nhầm rằng lỗi của MỘT nhánh ảnh hưởng tới KẾT QUẢ của nhánh KIA — nhưng
hai nhánh (`doThiNhanhA`, `doThiNhanhB`) LÀ hai đối tượng `DoThi` hoàn
toàn TÁCH BIỆT, không chia sẻ biến hay trạng thái nào; `chayDoThi` chạy
trên nhánh B không hề đọc `ketQuaA`.

Chỗ lệch: `nhanhLoi` CHỈ ghi tên nhánh MÀ `ketQuaX.thanhCong` của CHÍNH
nhánh đó LÀ `false` — nếu nhánh B thật sự thành công, `!ketQuaB.thanhCong`
LÀ `false`, nên `"B"` không được đẩy vào `nhanhLoi`.
::
:::
::::

::::code{#viet_fan_out_fan_in}
Hoàn thiện `chayFanOutFanIn` — gọi `chayDoThi(doThiNhanhA, tenGocNhanhA)`
VÀ `chayDoThi(doThiNhanhB, tenGocNhanhB)` KHÔNG điều kiện (LUÔN chạy CẢ
HAI); NẾU cả hai `thanhCong`, trả `{ thanhCong: true, dsNodeDaDiNhanhA,
dsNodeDaDiNhanhB }`; NGƯỢC LẠI, dựng mảng `nhanhLoi` (đẩy `"A"` NẾU
`ketQuaA` thất bại, đẩy `"B"` NẾU `ketQuaB` thất bại — CÓ THỂ cả hai),
trả `{ thanhCong: false, nhanhLoi, dsNodeDaDiNhanhA, dsNodeDaDiNhanhB }`.
Hoàn thiện `chayNhieuCapNhanh` — với MỖI phần tử trong `dsCap`, gọi
`chayFanOutFanIn` VỚI đúng bốn tham số của cặp đó, đẩy kết quả vào mảng
trả về.

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

type KetQuaFanOutFanIn =
  | { thanhCong: true; dsNodeDaDiNhanhA: string[]; dsNodeDaDiNhanhB: string[] }
  | { thanhCong: false; nhanhLoi: ("A" | "B")[]; dsNodeDaDiNhanhA: string[]; dsNodeDaDiNhanhB: string[] };

function chayFanOutFanIn(
  doThiNhanhA: DoThi, tenGocNhanhA: string,
  doThiNhanhB: DoThi, tenGocNhanhB: string,
): KetQuaFanOutFanIn {
  ___
}

type CapNhanh = { doThiA: DoThi; tenGocA: string; doThiB: DoThi; tenGocB: string };

function chayNhieuCapNhanh(dsCap: CapNhanh[]): KetQuaFanOutFanIn[] {
  ___
}

const doThiNhanhA: DoThi = {
  nhanhA_buoc1: taoNodeThanhCong("nhanhA_buoc2"),
  nhanhA_buoc2: taoNodeThanhCong(null),
};
const doThiNhanhALoi: DoThi = {
  nhanhA_buoc1: taoNodeLoi(),
};
const doThiNhanhB: DoThi = {
  nhanhB_buoc1: taoNodeThanhCong(null),
};
const doThiNhanhBLoi: DoThi = {
  nhanhB_buoc1: taoNodeLoi(),
};

const ketQuaBaCap = chayNhieuCapNhanh([
  { doThiA: doThiNhanhA, tenGocA: "nhanhA_buoc1", doThiB: doThiNhanhB, tenGocB: "nhanhB_buoc1" },
  { doThiA: doThiNhanhA, tenGocA: "nhanhA_buoc1", doThiB: doThiNhanhBLoi, tenGocB: "nhanhB_buoc1" },
  { doThiA: doThiNhanhALoi, tenGocA: "nhanhA_buoc1", doThiB: doThiNhanhB, tenGocB: "nhanhB_buoc1" },
]);
console.log(JSON.stringify(ketQuaBaCap.map((k) => k.thanhCong)));
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

type KetQuaFanOutFanIn =
  | { thanhCong: true; dsNodeDaDiNhanhA: string[]; dsNodeDaDiNhanhB: string[] }
  | { thanhCong: false; nhanhLoi: ("A" | "B")[]; dsNodeDaDiNhanhA: string[]; dsNodeDaDiNhanhB: string[] };

function chayFanOutFanIn(
  doThiNhanhA: DoThi, tenGocNhanhA: string,
  doThiNhanhB: DoThi, tenGocNhanhB: string,
): KetQuaFanOutFanIn {
  const ketQuaA = chayDoThi(doThiNhanhA, tenGocNhanhA);
  const ketQuaB = chayDoThi(doThiNhanhB, tenGocNhanhB);
  if (ketQuaA.thanhCong && ketQuaB.thanhCong) {
    return { thanhCong: true, dsNodeDaDiNhanhA: ketQuaA.dsNodeDaDi, dsNodeDaDiNhanhB: ketQuaB.dsNodeDaDi };
  }
  const nhanhLoi: ("A" | "B")[] = [];
  if (!ketQuaA.thanhCong) nhanhLoi.push("A");
  if (!ketQuaB.thanhCong) nhanhLoi.push("B");
  return { thanhCong: false, nhanhLoi, dsNodeDaDiNhanhA: ketQuaA.dsNodeDaDi, dsNodeDaDiNhanhB: ketQuaB.dsNodeDaDi };
}

type CapNhanh = { doThiA: DoThi; tenGocA: string; doThiB: DoThi; tenGocB: string };

function chayNhieuCapNhanh(dsCap: CapNhanh[]): KetQuaFanOutFanIn[] {
  const ketQua: KetQuaFanOutFanIn[] = [];
  for (const cap of dsCap) {
    ketQua.push(chayFanOutFanIn(cap.doThiA, cap.tenGocA, cap.doThiB, cap.tenGocB));
  }
  return ketQua;
}

const doThiNhanhA: DoThi = {
  nhanhA_buoc1: taoNodeThanhCong("nhanhA_buoc2"),
  nhanhA_buoc2: taoNodeThanhCong(null),
};
const doThiNhanhALoi: DoThi = {
  nhanhA_buoc1: taoNodeLoi(),
};
const doThiNhanhB: DoThi = {
  nhanhB_buoc1: taoNodeThanhCong(null),
};
const doThiNhanhBLoi: DoThi = {
  nhanhB_buoc1: taoNodeLoi(),
};

const ketQuaBaCap = chayNhieuCapNhanh([
  { doThiA: doThiNhanhA, tenGocA: "nhanhA_buoc1", doThiB: doThiNhanhB, tenGocB: "nhanhB_buoc1" },
  { doThiA: doThiNhanhA, tenGocA: "nhanhA_buoc1", doThiB: doThiNhanhBLoi, tenGocB: "nhanhB_buoc1" },
  { doThiA: doThiNhanhALoi, tenGocA: "nhanhA_buoc1", doThiB: doThiNhanhB, tenGocB: "nhanhB_buoc1" },
]);
console.log(JSON.stringify(ketQuaBaCap.map((k) => k.thanhCong)));
```

```typescript title=test
if (ketQuaBaCap.length !== 3) throw new Error("chayNhieuCapNhanh phai tra ve mang dung 3 phan tu");
if (JSON.stringify(ketQuaBaCap.map((k) => k.thanhCong)) !== JSON.stringify([true, false, false])) {
  throw new Error("ba cap nhanh phai cho ra dung [true,false,false] -- cap 1 ca hai thanh cong, cap 2 va 3 co dung mot nhanh loi");
}

const capBLoi = ketQuaBaCap[1]!;
if (capBLoi.thanhCong !== false) throw new Error("cap 2 (nhanh B loi) phai that bai");
if (capBLoi.thanhCong === false && JSON.stringify(capBLoi.nhanhLoi) !== JSON.stringify(["B"])) {
  throw new Error("cap 2 nhanhLoi phai la ['B'] -- chi nhanh B loi");
}
if (JSON.stringify(capBLoi.dsNodeDaDiNhanhA) !== JSON.stringify(["nhanhA_buoc1", "nhanhA_buoc2"])) {
  throw new Error("nhanh A phai VAN chay DAY DU du nhanh B loi -- khong bi huy giua chung");
}

const capALoi = ketQuaBaCap[2]!;
if (capALoi.thanhCong === false && JSON.stringify(capALoi.nhanhLoi) !== JSON.stringify(["A"])) {
  throw new Error("cap 3 nhanhLoi phai la ['A'] -- chi nhanh A loi");
}
if (JSON.stringify(capALoi.dsNodeDaDiNhanhB) !== JSON.stringify(["nhanhB_buoc1"])) {
  throw new Error("nhanh B phai VAN chay DAY DU du nhanh A loi -- khong bi bo qua");
}

const caHaiLoi = chayFanOutFanIn(doThiNhanhALoi, "nhanhA_buoc1", doThiNhanhBLoi, "nhanhB_buoc1");
if (caHaiLoi.thanhCong !== false) throw new Error("ca hai nhanh loi thi tong the phai that bai");
if (caHaiLoi.thanhCong === false && JSON.stringify(caHaiLoi.nhanhLoi) !== JSON.stringify(["A", "B"])) {
  throw new Error("khi CA HAI nhanh loi, nhanhLoi phai ghi CA HAI ['A','B']");
}

const motCap = chayNhieuCapNhanh([
  { doThiA: doThiNhanhA, tenGocA: "nhanhA_buoc1", doThiB: doThiNhanhB, tenGocB: "nhanhB_buoc1" },
]);
if (motCap.length !== 1) throw new Error("doi so danh sach cap phai doi do dai mang tra ve -- tham so phai duoc dung that");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (chayFanOutFanIn): goi chayDoThi cho CA HAI nhanh KHONG dieu kien (khong co if nao dung giua hai lenh goi); neu ca hai thanhCong thi tra ve thanhCong:true kem hai dsNodeDaDi; nguoc lai dung mot mang nhanhLoi, day 'A' neu ketQuaA that bai, day 'B' neu ketQuaB that bai, roi tra ve thanhCong:false kem nhanhLoi va hai dsNodeDaDi. Cho hai (chayNhieuCapNhanh): mot vong for-of qua dsCap, moi phan tu goi chayFanOutFanIn voi bon truong cua no, day ket qua vao mot mang."
- kind: strategy
  body: "Cho dau: const ketQuaA = chayDoThi(doThiNhanhA, tenGocNhanhA); const ketQuaB = chayDoThi(doThiNhanhB, tenGocNhanhB); if (ketQuaA.thanhCong && ketQuaB.thanhCong) { return { thanhCong: true, dsNodeDaDiNhanhA: ketQuaA.dsNodeDaDi, dsNodeDaDiNhanhB: ketQuaB.dsNodeDaDi }; } const nhanhLoi: ('A' | 'B')[] = []; if (!ketQuaA.thanhCong) nhanhLoi.push('A'); if (!ketQuaB.thanhCong) nhanhLoi.push('B'); return { thanhCong: false, nhanhLoi, dsNodeDaDiNhanhA: ketQuaA.dsNodeDaDi, dsNodeDaDiNhanhB: ketQuaB.dsNodeDaDi }; Cho hai: const ketQua: KetQuaFanOutFanIn[] = []; for (const cap of dsCap) { ketQua.push(chayFanOutFanIn(cap.doThiA, cap.tenGocA, cap.doThiB, cap.tenGocB)); } return ketQua;"
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
  expect: "[true,false,false]"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai nhánh, một điểm hội tụ — VÀ nhánh KHÔNG lỗi luôn chạy TRỌN VẸN, bất
kể nhánh kia ra sao. Đây LÀ cách "song song" hoạt động: không nhánh nào
biết nhánh kia đã xong. Bài sau chuyển hướng: khi một node rẽ nhánh, LÀM
SAO ghi lại VÌ SAO nó chọn nhánh này — không chỉ ghi ĐàI QUA đâu.
::::

::::reflect{#nghi-lai}
`chayFanOutFanIn` không thêm một CƠ CHẾ chạy mới — nó vẫn dùng NGUYÊN VĂN
`chayDoThi` (bài `1`) cho TỪNG nhánh. Điều MỚI nằm Ở CÁCH GỌI: hai lệnh
gọi TUẦN TỰ, KHÔNG PHỤ THUỘC kết quả của nhau, rồi một bước GỘP xét CẢ
HAI kết quả CÙNG lúc. Đây LÀ khác biệt với một `if/else` rẽ nhánh (bài
`2`) — rẽ nhánh CHỌN một trong hai đường, còn fan-out CHẠY cả hai đường
rồi GỘP. MASTERPLAN gọi lỗi mà cơ chế này chữa LÀ "không song song": một
hệ thống thiếu fan-out/fan-in thường xử lý nhánh B *sau khi* biết nhánh A
ra sao — làm lộ ra một phụ thuộc GIẢ giữa hai việc vốn ĐỘC LẬP. Việc luôn
chạy CẢ HAI nhánh, bất kể nhánh kia thế nào, chính LÀ điều loại bỏ phụ
thuộc giả đó.
::::

::::checkpoint{mastery=0.85}
::::
