---
id: co-so-du-lieu.vong-tron-quyen-luc.boss-vong-tron-quyen-luc
title: "BOSS — Vòng tròn quyền lực"
summary: "Ráp TRỌN q11: heThongDinhTuyen xây vòng vnode (bài 8) cho cụm bốn node, thêm một node thứ năm, rồi đếm số khoá đổi node (bài 5) — trên 100 khoá mẫu: 38 khoá đổi (38%), so với 85 khoá (85%) nếu dùng cách ngây thơ bam(khoa) % soLuongNode (bài 1-2) trên CÙNG dữ liệu. Toàn bộ hệ thống định tuyến — băm (bài 6), vòng tròn (bài 3-4), vnode cân bằng tải (bài 7-8) — gói gọn trong một hàm."
locale: vi
track: co-so-du-lieu
module: vong-tron-quyen-luc
order: 9
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: []
requires: [db.vnode-nhieu-diem-ao]
concepts: [db.boss-q11]
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
Băm rồi chia (bài 1-2), vòng tròn (bài 3-5), hàm băm tốt (bài 6),
vnode cân bằng tải (bài 7-8). Ráp TẤT cả thành một hệ định tuyến
hoàn chỉnh — trông ra sao?
::::

::::explain{#boss-that}
`heThongDinhTuyen` xây vòng vnode CHO cụm node TRƯỚC, xây LẠI vòng
vnode CHO cụm node SAU (đã thêm một node MỚI), rồi đếm SỐ khoá CÓ
node chịu trách nhiệm thay ĐỔI:

```typescript title=readonly
function bam(s: string): number {
  let h = 2166136261;
  for (let i = 0; i < s.length; i++) {
    h ^= s.charCodeAt(i);
    h = Math.imul(h, 16777619);
  }
  h ^= h >>> 16;
  h = Math.imul(h, 0x85ebca6b);
  h ^= h >>> 13;
  h = Math.imul(h, 0xc2b2ae35);
  h ^= h >>> 16;
  return Math.abs(h) % 1000;
}

interface DiemNode { viTri: number; ten: string; }

function xayVongVnode(tenCacNode: string[], soVnodeMoiNode: number): DiemNode[] {
  const vong: DiemNode[] = [];
  for (const ten of tenCacNode) {
    for (let i = 0; i < soVnodeMoiNode; i++) {
      vong.push({ viTri: bam(`${ten}#${i}`), ten });
    }
  }
  vong.sort((a, b) => a.viTri - b.viTri);
  return vong;
}

function timNodeChiuTrachNhiem(vong: DiemNode[], khoa: string): string {
  const viTriKhoa = bam(khoa);
  for (const diem of vong) {
    if (diem.viTri >= viTriKhoa) return diem.ten;
  }
  return vong[0]!.ten;
}

function demSoKhoaDoiVong(cacKhoa: string[], vongTruoc: DiemNode[], vongSau: DiemNode[]): number {
  let dem = 0;
  for (const khoa of cacKhoa) {
    if (timNodeChiuTrachNhiem(vongTruoc, khoa) !== timNodeChiuTrachNhiem(vongSau, khoa)) {
      dem = dem + 1;
    }
  }
  return dem;
}

function heThongDinhTuyen(
  tenCacNodeTruoc: string[],
  tenNodeMoi: string,
  soVnodeMoiNode: number,
  cacKhoa: string[]
): number {
  const vongTruoc = xayVongVnode(tenCacNodeTruoc, soVnodeMoiNode);
  const vongSau = xayVongVnode([...tenCacNodeTruoc, tenNodeMoi], soVnodeMoiNode);
  return demSoKhoaDoiVong(cacKhoa, vongTruoc, vongSau);
}

const cacKhoa: string[] = [];
for (let i = 0; i < 100; i++) cacKhoa.push("nguoidung" + i);
console.log(heThongDinhTuyen(["alpha", "beta", "gamma", "delta"], "epsilon", 20, cacKhoa));
```

```text title=readonly
38
```

`38` TRÊN `100` khoá — `38%` — đổi node khi cụm bốn node (`20`
vnode MỖI node) nhận THÊM `epsilon`. So VỚI cách ngây thơ (bài 1-2):
CÙNG kịch bản "cụm bốn node THÀNH năm node", `bam(khoa) %
soLuongNode` khiến `85` trên `100` khoá (`85%`) đổi NODE — hơn gấp
ĐÔI.
::::

::::example{#ba-lop-ghep-lai}
`heThongDinhTuyen` KHÔNG viết logic MỚI nào — nó ghép ĐÚNG ba lớp
đã xây TỪNG bài một: (1) `bam` (bài 6) — hàm băm ĐỦ trộn để tránh
dồn CỤM; (2) `xayVongVnode` + `timNodeChiuTrachNhiem` (bài 4, 8) —
đặt NHIỀU điểm ảo cho mỗi node LÊN vòng, cân bằng tải TỐT hơn một
điểm DUY nhất; (3) `demSoKhoaDoiVong` (bài 5) — đo LẠI đúng chỉ số
đã dùng xuyên SUỐT quest: bao nhiêu khoá THỰC sự phải di CHUYỂN.
KHÔNG lớp nào biết VỀ hai lớp kia LÀM gì bên trong — CHỈ ghép đúng
THỨ tự gọi hàm.
::::

::::predict{#doan-them-hai-node commitOnce}
Thay VÌ thêm ĐÚNG một node (`epsilon`), tưởng tượng THÊM liên tiếp
HAI node (`epsilon` RỒI `zeta`) VÀO cụm bốn node BAN đầu. Tổng số
khoá bị ảnh HƯỞNG (tính GỘP cả hai lần thêm) SO với thêm MỘT node
DUY nhất — nhiều HƠN, ÍT hơn, hay BẰNG?

:::opt{correct}
NHIỀU hơn — MỖI lần thêm một node LÀ một lần "chia nhỏ" thêm một
đoạn TRÊN vòng, VÀ mỗi lần chia đều kéo THEO một SỐ khoá di chuyển
RIÊNG — hai lần thêm CỘNG dồn số khoá bị ảnh hưởng CỦA từng lần
:::

:::opt
BẰNG nhau — tổng SỐ node cuối cùng LÀ như nhau (`sáu` node) dù thêm
MỘT lần hai node HAY hai lần một node, nên tổng ẢNH hưởng CŨNG phải
bằng nhau
::why
Gần đúng ở việc bạn nghĩ TỚI "trạng thái CUỐI cùng giống nhau nên
kết quả GIỐNG nhau" — MỘT trực giác hợp lý nếu hệ THỐNG chỉ quan
tâm điểm ĐẾN, không quan tâm ĐƯỜNG đi.

Chỗ lệch: `demSoKhoaDoiVong` đo THEO từng CẶP trạng thái (TRƯỚC/sau
MỘT lần thay đổi), không phải chỉ so sánh điểm ĐẦU với điểm CUỐI.
Thêm `epsilon` rồi thêm TIẾP `zeta` LÀ HAI lần thay đổi riêng biệt —
mỗi lần CÓ một tập khoá bị ảnh hưởng RIÊNG (đoạn quanh `epsilon` LẦN
đầu, đoạn quanh `zeta` lần SAU) — CỘNG dồn hai tập ĐÓ (dù CÓ trùng
lặp một PHẦN) vẫn cho tổng ảnh hưởng LỚN hơn một lần thêm DUY nhất.
::
:::
::::

::::code{#viet_boss_q11}
Hoàn thiện `heThongDinhTuyen` — xây vòng TRƯỚC VÀ sau khi thêm node
mới, rồi đếm SỐ khoá đổi node.

```typescript title=starter
function bam(s: string): number {
  let h = 2166136261;
  for (let i = 0; i < s.length; i++) {
    h ^= s.charCodeAt(i);
    h = Math.imul(h, 16777619);
  }
  h ^= h >>> 16;
  h = Math.imul(h, 0x85ebca6b);
  h ^= h >>> 13;
  h = Math.imul(h, 0xc2b2ae35);
  h ^= h >>> 16;
  return Math.abs(h) % 1000;
}

interface DiemNode { viTri: number; ten: string; }

function xayVongVnode(tenCacNode: string[], soVnodeMoiNode: number): DiemNode[] {
  const vong: DiemNode[] = [];
  for (const ten of tenCacNode) {
    for (let i = 0; i < soVnodeMoiNode; i++) {
      vong.push({ viTri: bam(`${ten}#${i}`), ten });
    }
  }
  vong.sort((a, b) => a.viTri - b.viTri);
  return vong;
}

function timNodeChiuTrachNhiem(vong: DiemNode[], khoa: string): string {
  const viTriKhoa = bam(khoa);
  for (const diem of vong) {
    if (diem.viTri >= viTriKhoa) return diem.ten;
  }
  return vong[0]!.ten;
}

function demSoKhoaDoiVong(cacKhoa: string[], vongTruoc: DiemNode[], vongSau: DiemNode[]): number {
  let dem = 0;
  for (const khoa of cacKhoa) {
    if (timNodeChiuTrachNhiem(vongTruoc, khoa) !== timNodeChiuTrachNhiem(vongSau, khoa)) {
      dem = dem + 1;
    }
  }
  return dem;
}

function heThongDinhTuyen(
  tenCacNodeTruoc: string[],
  tenNodeMoi: string,
  soVnodeMoiNode: number,
  cacKhoa: string[]
): number {
  const vongTruoc = xayVongVnode(tenCacNodeTruoc, soVnodeMoiNode);
  const vongSau = ___;
  return demSoKhoaDoiVong(cacKhoa, vongTruoc, vongSau);
}

const cacKhoa: string[] = [];
for (let i = 0; i < 100; i++) cacKhoa.push("nguoidung" + i);
console.log(heThongDinhTuyen(["alpha", "beta", "gamma", "delta"], "epsilon", 20, cacKhoa));
```

```typescript title=solution
function bam(s: string): number {
  let h = 2166136261;
  for (let i = 0; i < s.length; i++) {
    h ^= s.charCodeAt(i);
    h = Math.imul(h, 16777619);
  }
  h ^= h >>> 16;
  h = Math.imul(h, 0x85ebca6b);
  h ^= h >>> 13;
  h = Math.imul(h, 0xc2b2ae35);
  h ^= h >>> 16;
  return Math.abs(h) % 1000;
}

interface DiemNode { viTri: number; ten: string; }

function xayVongVnode(tenCacNode: string[], soVnodeMoiNode: number): DiemNode[] {
  const vong: DiemNode[] = [];
  for (const ten of tenCacNode) {
    for (let i = 0; i < soVnodeMoiNode; i++) {
      vong.push({ viTri: bam(`${ten}#${i}`), ten });
    }
  }
  vong.sort((a, b) => a.viTri - b.viTri);
  return vong;
}

function timNodeChiuTrachNhiem(vong: DiemNode[], khoa: string): string {
  const viTriKhoa = bam(khoa);
  for (const diem of vong) {
    if (diem.viTri >= viTriKhoa) return diem.ten;
  }
  return vong[0]!.ten;
}

function demSoKhoaDoiVong(cacKhoa: string[], vongTruoc: DiemNode[], vongSau: DiemNode[]): number {
  let dem = 0;
  for (const khoa of cacKhoa) {
    if (timNodeChiuTrachNhiem(vongTruoc, khoa) !== timNodeChiuTrachNhiem(vongSau, khoa)) {
      dem = dem + 1;
    }
  }
  return dem;
}

function heThongDinhTuyen(
  tenCacNodeTruoc: string[],
  tenNodeMoi: string,
  soVnodeMoiNode: number,
  cacKhoa: string[]
): number {
  const vongTruoc = xayVongVnode(tenCacNodeTruoc, soVnodeMoiNode);
  const vongSau = xayVongVnode([...tenCacNodeTruoc, tenNodeMoi], soVnodeMoiNode);
  return demSoKhoaDoiVong(cacKhoa, vongTruoc, vongSau);
}

const cacKhoa: string[] = [];
for (let i = 0; i < 100; i++) cacKhoa.push("nguoidung" + i);
console.log(heThongDinhTuyen(["alpha", "beta", "gamma", "delta"], "epsilon", 20, cacKhoa));
```

```typescript title=test
const cacKhoa2: string[] = [];
for (let i = 0; i < 100; i++) cacKhoa2.push("nguoidung" + i);
console.log(heThongDinhTuyen(["alpha", "beta", "gamma", "delta"], "epsilon", 20, cacKhoa2));
if (heThongDinhTuyen(["alpha", "beta", "gamma", "delta"], "epsilon", 20, cacKhoa2) !== 38) throw new Error("them epsilon (20 vnode) tren 100 khoa phai co dung 38 khoa doi node");

const cacKhoa12: string[] = [];
for (let i = 0; i < 12; i++) cacKhoa12.push("nguoidung" + i);
if (heThongDinhTuyen(["alpha", "beta", "gamma", "delta"], "epsilon", 20, cacKhoa12) !== 4) throw new Error("voi 12 khoa mau phai co dung 4 khoa doi node");

if (heThongDinhTuyen(["alpha", "beta", "gamma", "delta"], "epsilon", 20, []) !== 0) throw new Error("danh sach khoa rong thi khong khoa nao doi");
```

:::hints
- kind: attention
  body: "Xay vong SAU khi them node moi: dung xayVongVnode tren mang node cu CONG them tenNodeMoi -- mot dong."
- kind: strategy
  body: "xayVongVnode([...tenCacNodeTruoc, tenNodeMoi], soVnodeMoiNode)"
- kind: one-line
  body: "const vongSau = xayVongVnode([...tenCacNodeTruoc, tenNodeMoi], soVnodeMoiNode);"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "38"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Băm, vòng tròn, vnode — ráp thành một hệ định tuyến khớp đúng những
gì hệ phân tán thật (ScyllaDB, Cassandra) làm. q11 "Vòng tròn quyền
lực" khép lại.
::::

::::reflect{#nghi-lai}
`heThongDinhTuyen` LÀ toàn bộ q11 gói TRONG một hàm: hàm băm ĐỦ
trộn (bài 6) tránh dồn CỤM, vòng tròn (bài 3-4) thay THẾ `%
soLuongNode` bằng một quan HỆ hình học không phụ thuộc TỔNG số
node, VÀ vnode (bài 7-8) sửa vấn đề "một điểm MỖI node phân bố
không đều". Con SỐ cuối cùng — `38%` khoá di chuyển thay vì `85%` —
KHÔNG phải một PHÉP màu, mà LÀ hệ quả trực tiếp CỦA từng quyết định
thiết kế NHỎ, xây từng bài MỘT. Đây chính LÀ cách một hệ THẬT (như
ScyllaDB) chia dữ liệu RA hàng trăm máy MÀ vẫn chịu được việc MÁY
móc tới VÀ đi liên tục — không CÓ máy chủ trung tâm NÀO cần biết
"khoá NÀO ở đâu", mọi node tự TÍNH được câu trả LỜI từ đúng CÔNG
thức đã xây Ở đây.
::::

::::checkpoint{mastery=0.9}
::::
