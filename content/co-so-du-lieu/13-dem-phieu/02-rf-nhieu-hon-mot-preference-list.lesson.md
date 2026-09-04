---
id: co-so-du-lieu.dem-phieu.rf-nhieu-hon-mot-preference-list
title: "RF nhiều hơn 1 — preference list"
summary: "laySachSoHuu đi tiếp trên vòng vnode TỪ vị trí khoá, gom RF node vật lý PHÂN BIỆT (bỏ qua vnode trùng node vật lý đã có) — đây LÀ 'preference list' của một khoá. Với RF=3, 'khoa1' thuộc về [alpha, gamma, delta] theo đúng thứ tự gặp trên vòng; RF=1 cho kết quả giống HỆT timNodeChiuTrachNhiem (bài 1); RF=5 (lớn hơn số node vật lý) dừng lại ở đúng 4 — không thể có nhiều bản sao hơn số node đang có."
locale: vi
track: co-so-du-lieu
module: dem-phieu
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [db.rf-nhieu-hon-mot-preference-list]
requires: [db.mot-ban-sao-la-chua-du]
concepts: [db.rf-nhieu-hon-mot-preference-list]
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
RF=1 (bài trước) LÀ trường hợp "chưa đủ". Vậy RF=3 nghĩa LÀ gì — một
khoá cần lưu Ở BA node NÀO, chọn RA sao?
::::

::::explain{#lay-sach-so-huu}
`laySachSoHuu` đi TIẾP trên vòng vnode (ĐÚNG cách `timNodeChiuTrachNhiem`
tìm node ĐẦU tiên, chỉ khác LÀ đi TIẾP thay VÌ dừng), gom đủ `rf` TÊN
node vật lý PHÂN biệt — bỏ qua vnode trùng TÊN node đã có trong danh
sách. Danh sách nhận VỀ gọi LÀ "preference list": thứ tự ưu tiên các
node chịu trách nhiệm cho một khoá:

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

function laySachSoHuu(vong: DiemNode[], khoa: string, rf: number): string[] {
  const viTriKhoa = bam(khoa);
  let batDau = vong.findIndex((d) => d.viTri >= viTriKhoa);
  if (batDau === -1) batDau = 0;
  const ketQua: string[] = [];
  for (let i = 0; i < vong.length && ketQua.length < rf; i++) {
    const diem = vong[(batDau + i) % vong.length]!;
    if (!ketQua.includes(diem.ten)) ketQua.push(diem.ten);
  }
  return ketQua;
}

const vong = xayVongVnode(["alpha", "beta", "gamma", "delta"], 20);
console.log(laySachSoHuu(vong, "khoa1", 3));
console.log(laySachSoHuu(vong, "khoa2", 3));
```

```text title=readonly
[ 'alpha', 'gamma', 'delta' ]
[ 'beta', 'gamma', 'delta' ]
```

`vong.findIndex` tìm vị trí ĐẦU tiên (`batDau`) — ĐÚNG chỗ
`timNodeChiuTrachNhiem` bắt đầu. Vòng lặp `for` sau đó đi TIẾP từ ĐÓ,
quấn quanh vòng BẰNG phép chia lấy DƯ `(batDau + i) % vong.length`,
GHI nhận TÊN node MỖI khi gặp một TÊN chưa CÓ trong `ketQua` — dừng
NGAY khi đủ `rf` tên phân BIỆT.
::::

::::example{#rf1-trung-voi-tim-node}
`laySachSoHuu(vong, khoa, 1)` LUÔN cho kết quả giống HỆT
`[timNodeChiuTrachNhiem(vong, khoa)]` — MẢNG một phần TỬ chứa đúng
node ĐẦU tiên gặp được. RF=1 KHÔNG phải một khái niệm mới, mà chính
LÀ preference list rút GỌN xuống còn MỘT phần tử — cầu nối trực tiếp
TỚI bài 1.
::::

::::predict{#doan-rf-vuot-so-node commitOnce}
Cụm CHỈ có `4` node vật lý (`alpha, beta, gamma, delta`). Gọi
`laySachSoHuu(vong, "khoa1", 5)` (RF=5, LỚN hơn số node đang có) —
kết quả LÀ gì?

:::opt{correct}
Một mảng `4` phần tử — CHỨA đủ cả bốn node vật lý (không trùng LẶP
tên nào) — vì KHÔNG THỂ có nhiều bản sao hơn số node đang tồn tại
:::

:::opt
Báo lỗi HOẶC mảng CHỨA node trùng lặp (`5` phần tử, một TÊN xuất
hiện hai lần) — vì vòng lặp CỐ đi cho đủ `rf=5` phần tử BẰNG mọi giá
::why
Gần đúng ở việc bạn để Ý điều kiện dừng `ketQua.length < rf` VẪN
đúng ngay cả SAU khi đã gom đủ 4 tên — vòng `for` (theo chỉ SỐ `i`)
thật SỰ tiếp tục chạy.

Chỗ lệch: vòng `for` tiếp tục chạy, NHƯNG mỗi lần gặp một `diem.ten`
ĐÃ có sẵn trong `ketQua`, điều kiện `!ketQua.includes(diem.ten)` NGĂN
nó được thêm LẦN nữa — nên `ketQua` không hề vượt QUÁ 4 phần tử phân
biệt. Vòng `for` chỉ THỰC sự dừng khi `i` chạy hết `vong.length` (80)
MÀ `ketQua.length` vẫn dừng Ở 4 — không lỗi, không trùng lặp, chỉ
đơn giản LÀ "hết node để cho".
::
:::
::::

::::code{#viet_lay_sach_so_huu}
Hoàn thiện `laySachSoHuu` — VỚI mỗi điểm gặp được trên vòng (đi TỪ
`batDau`, quấn VÒNG), nếu TÊN node của nó CHƯA có trong `ketQua` thì
thêm VÀO.

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

function laySachSoHuu(vong: DiemNode[], khoa: string, rf: number): string[] {
  const viTriKhoa = bam(khoa);
  let batDau = vong.findIndex((d) => d.viTri >= viTriKhoa);
  if (batDau === -1) batDau = 0;
  const ketQua: string[] = [];
  for (let i = 0; i < vong.length && ketQua.length < rf; i++) {
    const diem = vong[(batDau + i) % vong.length]!;
    ___
  }
  return ketQua;
}

const vong = xayVongVnode(["alpha", "beta", "gamma", "delta"], 20);
console.log(laySachSoHuu(vong, "khoa1", 3));
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

function laySachSoHuu(vong: DiemNode[], khoa: string, rf: number): string[] {
  const viTriKhoa = bam(khoa);
  let batDau = vong.findIndex((d) => d.viTri >= viTriKhoa);
  if (batDau === -1) batDau = 0;
  const ketQua: string[] = [];
  for (let i = 0; i < vong.length && ketQua.length < rf; i++) {
    const diem = vong[(batDau + i) % vong.length]!;
    if (!ketQua.includes(diem.ten)) ketQua.push(diem.ten);
  }
  return ketQua;
}

const vong = xayVongVnode(["alpha", "beta", "gamma", "delta"], 20);
console.log(laySachSoHuu(vong, "khoa1", 3));
```

```typescript title=test
const dsKhoa1 = laySachSoHuu(vong, "khoa1", 3);
console.log(dsKhoa1);
if (JSON.stringify(dsKhoa1) !== JSON.stringify(["alpha", "gamma", "delta"])) throw new Error("khoa1 RF=3 phai la [alpha, gamma, delta] theo dung thu tu");
if (new Set(dsKhoa1).size !== dsKhoa1.length) throw new Error("danh sach so huu khong duoc co ten node trung lap");

const dsKhoa2 = laySachSoHuu(vong, "khoa2", 3);
if (JSON.stringify(dsKhoa2) !== JSON.stringify(["beta", "gamma", "delta"])) throw new Error("khoa2 RF=3 phai la [beta, gamma, delta]");

if (JSON.stringify(laySachSoHuu(vong, "khoa1", 1)) !== JSON.stringify(["alpha"])) throw new Error("RF=1 phai tra ve dung 1 phan tu, giong timNodeChiuTrachNhiem");

const dsRf5 = laySachSoHuu(vong, "khoa1", 5);
if (dsRf5.length !== 4) throw new Error("chi co 4 node vat ly, RF=5 phai dung lai o 4 phan tu");
if (new Set(dsRf5).size !== 4) throw new Error("RF=5 phai chua du 4 ten node phan biet, khong trung");
```

:::hints
- kind: attention
  body: "Neu ten cua diem CHUA co trong ketQua thi them vao -- mot dong."
- kind: strategy
  body: "if (!ketQua.includes(diem.ten)) ketQua.push(diem.ten);"
- kind: one-line
  body: "if (!ketQua.includes(diem.ten)) ketQua.push(diem.ten);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "alpha"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Biết BA node nào giữ một khoá — bước tiếp theo: GHI có cần cả BA
node xác nhận không, hay ÍT hơn cũng đủ?
::::

::::reflect{#nghi-lai}
`laySachSoHuu` không cần một cấu trúc dữ liệu MỚI nào — nó dùng LẠI
NGUYÊN vòng vnode Đà xây Ở q11, chỉ đổi cách DUYỆT: thay VÌ dừng NGAY
ở node đầu tiên, nó đi TIẾP và gom nhiều tên phân biệt. Danh sách
NÀY (preference list) sẽ LÀ nền tảng cho MỌI khái niệm còn lại của
quest NÀY — ghi, đọc, hint, sửa lỗi — tất cả đều thao TÁC trên đúng
DANH sách này, không phải MỘT node đơn lẻ nữa.
::::

::::checkpoint{mastery=0.8}
::::
