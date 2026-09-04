---
id: co-so-du-lieu.bia-mo-va-nguoi-don-rac.nen-theo-cua-so-thoi-gian-twcs
title: "Nén theo cửa sổ thời gian — TWCS"
summary: "nhomTheoCuaSo nhóm SSTable theo cửa sổ thời gian flush (Math.floor(thoiGianFlush/windowMs)) -- CHỈ gộp các bảng CÙNG cửa sổ, không bao giờ trộn dữ liệu cũ với mới. 6 SSTable rơi vào 3 cửa sổ [a,b]/[c,d,e]/[f]; ở t=15 triệu với TTL=7.2 triệu, 2/3 cửa sổ đã hết hạn HOÀN TOÀN, có thể XOÁ NGUYÊN cả cửa sổ mà không cần đọc lại bất kỳ khoá nào bên trong."
locale: vi
track: co-so-du-lieu
module: bia-mo-va-nguoi-don-rac
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [db.nen-theo-cua-so-thoi-gian-twcs]
requires: [db.nen-theo-tang-lcs]
concepts: [db.nen-theo-cua-so-thoi-gian-twcs]
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
STCS gộp theo kích CỠ (bài 5), LCS gộp theo tầng KHÔNG chồng lấn
(bài 6) — cả hai đều TRỘN dữ liệu CŨ với MỚI khi gộp. Dữ liệu time-
series (cảm biến, log, sự kiện) có TÍNH chất riêng: KHÔNG BAO giờ
sửa dữ liệu CŨ, chỉ THÊM mới. Tận dụng được KHÔNG?
::::

::::explain{#nhom-theo-cua-so}
`nhomTheoCuaSo` nhóm SSTable theo "cửa sổ thời gian" chúng được flush
— `Math.floor(thoiGianFlush / windowMs)` — VÀ "Time-Window Compaction
Strategy" (TWCS) CHỈ gộp các bảng CÙNG một cửa sổ VỚI nhau, KHÔNG BAO
GIỜ trộn hai cửa sổ khác nhau:

```typescript title=readonly
interface SSTableThoiGian { id: string; thoiGianFlush: number; }

function nhomTheoCuaSo(cacBang: SSTableThoiGian[], windowMs: number): Map<number, SSTableThoiGian[]> {
  const nhom = new Map<number, SSTableThoiGian[]>();
  for (const bang of cacBang) {
    const cuaSo = Math.floor(bang.thoiGianFlush / windowMs);
    const ds = nhom.get(cuaSo) ?? [];
    ds.push(bang);
    nhom.set(cuaSo, ds);
  }
  return nhom;
}

const windowMs = 3600_000; // 1 gio
const cacBang: SSTableThoiGian[] = [
  { id: "a", thoiGianFlush: 0 },
  { id: "b", thoiGianFlush: 1_000_000 },
  { id: "c", thoiGianFlush: 3_600_000 },
  { id: "d", thoiGianFlush: 4_000_000 },
  { id: "e", thoiGianFlush: 5_000_000 },
  { id: "f", thoiGianFlush: 7_300_000 },
];
const nhom = nhomTheoCuaSo(cacBang, windowMs);
console.log("so cua so:", nhom.size);
for (const [cuaSo, ds] of [...nhom.entries()].sort((a, b) => a[0] - b[0])) {
  console.log(`cua so ${cuaSo}:`, ds.map((b) => b.id).join(","));
}
```

```text title=readonly
so cua so: 3
cua so 0: a,b
cua so 1: c,d,e
cua so 2: f
```

`a` (`t=0`) VÀ `b` (`t=1_000_000`) cùng rơi vào cửa SỔ `0` (`0` tới
`3_600_000`). `c, d, e` rơi VÀO cửa sổ `1`. `f` (`t=7_300_000`) MỘT
mình Ở cửa sổ `2`. `6` SSTable, `3` cửa SỔ — TWCS sẽ CHỈ từng gộp
CÁC bảng bên TRONG cùng một cửa sổ, KHÔNG BAO giờ gộp `a` với `c`.
::::

::::example{#drop-nguyen-cua-so}
Lợi ích LỚN nhất của TWCS: một khi CẢ cửa sổ đã "hết hạn" (mọi khoá
BÊN trong đều đã quá TTL), toàn bộ SSTable của cửa sổ ĐÓ có thể bị
XOÁ NGUYÊN VẸN — không cần ĐỌC lại từng khoá, không cần TẠO bia mộ
CHO từng khoá (bài 1-4). STCS/LCS KHÔNG làm được điều NÀY vì dữ liệu
CŨ và MỚI đã bị TRỘN lẫn qua nhiều lần gộp.
::::

::::predict{#doan-cua-so-het-han commitOnce}
`TTL=7_200_000` (2 giờ). Tại `thoiGianHienTai=15_000_000`, cửa sổ `0`
kết thúc Ở `3_600_000`, cửa sổ `1` kết thúc Ở `7_200_000`. Cửa sổ NÀO
đã hết hạn HOÀN toàn (`thoiGianHienTai - gioKetThuc >= TTL`)?

:::opt{correct}
Cửa sổ `0` VÀ `1` — `15_000_000 - 3_600_000 = 11_400_000 >= 7_200_000`
VÀ `15_000_000 - 7_200_000 = 7_800_000 >= 7_200_000`, cả hai ĐỀU đủ
:::

:::opt
CHỈ cửa sổ `0` — cửa sổ CÀNG cũ càng CHẮC chắn hết hạn, cửa sổ `1`
"mới hơn" NÊN chưa tới lượt
::why
Trực giác "cũ HƠN thì hết hạn TRƯỚC" đúng Ở HƯỚNG so sánh, nhưng
KHÔNG đúng LÀ chỉ MỘT cửa sổ hết hạn Ở MỘT thời điểm — nhiều cửa sổ
CÓ thể hết hạn ĐỒNG thời nếu chúng đều đủ CŨ.

Chỗ lệch: `gioKetThuc` của cửa sổ `1` LÀ `7_200_000` (KẾT thúc window
1 tại (1+1)*3_600_000). `15_000_000 - 7_200_000 = 7_800_000`, VẪN
`>= TTL(7_200_000)` — cửa sổ `1` CŨNG đã hết hạn, không CHỈ cửa sổ
`0`. Chỉ cửa sổ `2` (chứa `f`, `t=7_300_000`) CHƯA hết hạn.
::
:::
::::

::::code{#viet_nhom_theo_cua_so}
Hoàn thiện `nhomTheoCuaSo` — tính cửa sổ CỦA mỗi bảng bằng phép chia
lấy PHẦN nguyên cho `windowMs`.

```typescript title=starter
interface SSTableThoiGian { id: string; thoiGianFlush: number; }

function nhomTheoCuaSo(cacBang: SSTableThoiGian[], windowMs: number): Map<number, SSTableThoiGian[]> {
  const nhom = new Map<number, SSTableThoiGian[]>();
  for (const bang of cacBang) {
    const cuaSo = ___;
    const ds = nhom.get(cuaSo) ?? [];
    ds.push(bang);
    nhom.set(cuaSo, ds);
  }
  return nhom;
}

const cacBang: SSTableThoiGian[] = [
  { id: "a", thoiGianFlush: 0 },
  { id: "b", thoiGianFlush: 1_000_000 },
  { id: "c", thoiGianFlush: 3_600_000 },
];
console.log(nhomTheoCuaSo(cacBang, 3_600_000).size);
```

```typescript title=solution
interface SSTableThoiGian { id: string; thoiGianFlush: number; }

function nhomTheoCuaSo(cacBang: SSTableThoiGian[], windowMs: number): Map<number, SSTableThoiGian[]> {
  const nhom = new Map<number, SSTableThoiGian[]>();
  for (const bang of cacBang) {
    const cuaSo = Math.floor(bang.thoiGianFlush / windowMs);
    const ds = nhom.get(cuaSo) ?? [];
    ds.push(bang);
    nhom.set(cuaSo, ds);
  }
  return nhom;
}

const cacBang: SSTableThoiGian[] = [
  { id: "a", thoiGianFlush: 0 },
  { id: "b", thoiGianFlush: 1_000_000 },
  { id: "c", thoiGianFlush: 3_600_000 },
];
console.log(nhomTheoCuaSo(cacBang, 3_600_000).size);
```

```typescript title=test
const windowMs = 3_600_000;
const cacBang2: SSTableThoiGian[] = [
  { id: "a", thoiGianFlush: 0 },
  { id: "b", thoiGianFlush: 1_000_000 },
  { id: "c", thoiGianFlush: 3_600_000 },
  { id: "d", thoiGianFlush: 4_000_000 },
  { id: "e", thoiGianFlush: 5_000_000 },
  { id: "f", thoiGianFlush: 7_300_000 },
];
const nhom = nhomTheoCuaSo(cacBang2, windowMs);
if (nhom.size !== 3) throw new Error("6 bang phai roi vao dung 3 cua so");
if ((nhom.get(0) ?? []).map((b) => b.id).join(",") !== "a,b") throw new Error("cua so 0 phai chua dung a,b");
if ((nhom.get(1) ?? []).map((b) => b.id).join(",") !== "c,d,e") throw new Error("cua so 1 phai chua dung c,d,e");
if ((nhom.get(2) ?? []).map((b) => b.id).join(",") !== "f") throw new Error("cua so 2 phai chi chua f");

const motBang: SSTableThoiGian[] = [{ id: "x", thoiGianFlush: 0 }];
if (nhomTheoCuaSo(motBang, windowMs).size !== 1) throw new Error("1 bang phai cho dung 1 cua so");

// t=1_900_000 > nua windowMs (1_800_000) -- floor phai la 0, KHONG duoc lam tron len 1
const bangGanGiua: SSTableThoiGian[] = [{ id: "y", thoiGianFlush: 1_900_000 }];
const nhomGanGiua = nhomTheoCuaSo(bangGanGiua, windowMs);
if (!nhomGanGiua.has(0)) throw new Error("t=1_900_000 phai roi vao cua so 0 (floor), khong phai cua so 1 (neu lam tron)");
```

:::hints
- kind: attention
  body: "Cua so la phep chia lay phan nguyen thoiGianFlush cho windowMs -- mot bieu thuc."
- kind: strategy
  body: "const cuaSo = Math.floor(bang.thoiGianFlush / windowMs);"
- kind: one-line
  body: "const cuaSo = Math.floor(bang.thoiGianFlush / windowMs);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "2"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba chiến lược: STCS (theo kích cỡ), LCS (theo tầng), TWCS (theo cửa
sổ). MỖI cái thắng Ở một loại workload KHÁC nhau — đo trực tiếp CẢ
ba trên CÙNG một khối lượng dữ liệu để thấy RÕ đánh đổi.
::::

::::reflect{#nghi-lai}
TWCS KHÔNG cố gắng tối ưu số bảng (STCS) HAY read amplification (LCS)
— nó tối ưu cho MỘT thao tác đặc BIỆT: xoá HÀNG loạt theo tuổi, LÀ
thao tác CỐT lõi của dữ liệu time-series CÓ TTL. Trộn cửa sổ CŨ với
MỚI (như STCS/LCS VẪN làm) phá VỠ chính LỢI thế đó — một khi dữ liệu
CŨ đã dính VÀO một bảng có dữ liệu MỚI, bảng đó KHÔNG thể bị drop
nguyên vẹn được NỮA, dù phần LỚN nội dung của nó đã hết hạn.
::::

::::checkpoint{mastery=0.85}
::::
