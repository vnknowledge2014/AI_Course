---
id: co-so-du-lieu.moi-loi-mot-vuong-quoc.them-loi-tham-hoa-rehash
title: "Thêm một lõi — thảm hoạ rehash (lại)"
summary: "demSoKhoaDoiLoi đếm khoá đổi lõi khi soLoi thay đổi — CÙNG công thức demSoKhoaDoiNode của q11 bài 2, chỉ đổi tên. Đi từ 3 lên 4 lõi (ví dụ tăng số lõi CPU của máy), 9/12 khoá (75%) đổi lõi — ĐÚNG tỉ lệ q11 bài 2 đã thấy ở quy mô máy, vì chonLoiChoKhoa dùng đúng công thức bam(khoa) % soLoi bị rehash bởi cùng một lý do: đổi soLoi đổi TOÀN BỘ phép chia lấy dư."
locale: vi
track: co-so-du-lieu
module: moi-loi-mot-vuong-quoc
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [db.them-loi-tham-hoa-rehash]
requires: [db.gui-tin-nhan-toi-loi-khac]
concepts: [db.them-loi-tham-hoa-rehash]
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
Kho RIÊNG, routing tất định, tin NHẮN — mọi thứ đã HOẠT động. NHƯNG
nếu máy được NÂNG cấp lên NHIỀU lõi hơn (`3` lên `4`), CHUYỆN gì xảy
ra VỚI dữ liệu đã lưu?
::::

::::explain{#dem-so-khoa-doi-loi}
`demSoKhoaDoiLoi` so sánh lõi CHỊU trách nhiệm CHO mỗi khoá GIỮA hai
giá trị `soLoi` — ĐÚNG công thức `demSoKhoaDoiNode` (q11 bài 2), CHỈ
đổi TÊN:

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
function chonLoiChoKhoa(khoa: string, soLoi: number): number {
  return bam(khoa) % soLoi;
}

function demSoKhoaDoiLoi(cacKhoa: string[], soLoiTruoc: number, soLoiSau: number): number {
  let dem = 0;
  for (const khoa of cacKhoa) {
    if (chonLoiChoKhoa(khoa, soLoiTruoc) !== chonLoiChoKhoa(khoa, soLoiSau)) {
      dem = dem + 1;
    }
  }
  return dem;
}

const cacKhoa: string[] = [];
for (let i = 0; i < 12; i++) cacKhoa.push("nguoidung" + i);
console.log(demSoKhoaDoiLoi(cacKhoa, 3, 4));
```

```text title=readonly
9
```

`9` TRÊN `12` khoá — `75%` — đổi LÕI khi `soLoi` đi TỪ `3` lên `4`.
ĐÚNG tỉ lệ q11 bài 2 ĐÃ thấy Ở quy mô MÁY — VÌ đây LÀ đúng cùng một
phép TOÁN (`% soLoi`), chỉ khác Ở TÊN biến. `75%` dữ liệu (nằm TRONG
`Map` của lõi CŨ) giờ "thuộc VỀ" một lõi KHÁC — muốn ĐỌC lại đúng,
phải DI chuyển dữ liệu SANG kho mới, HOẶC route sai chỗ.
::::

::::example{#khong-lien-quan-toi-share-nothing}
Vấn đề NÀY KHÔNG hề mâu thuẫn VỚI "share-nothing" (bài 3-6) — MỖI
kho VẪN hoàn toàn ĐỘC lập, MỖI yêu cầu VẪN routing đúng theo `soLoi`
HIỆN tại. Vấn đề LÀ: `soLoi` VỪA thay đổi (`3`→`4`), NÊN "routing
ĐÚNG theo `soLoi` hiện tại" GIỜ trỏ tới một lõi KHÁC hẳn so VỚI lúc
dữ liệu được GHI. Dữ liệu VẪN nằm nguyên Ở `Map` CŨ — chỉ LÀ không
còn AI route TỚI đúng chỗ NỮA.
::::

::::predict{#doan-bot-loi commitOnce}
THAY vì THÊM lõi (`3→4`), giả sử MÁY bị RÚT bớt lõi (`4→3`, VÍ dụ
giới hạn tài nguyên). SO với việc THÊM lõi, việc BỚT lõi (`4→3`) CÓ
khiến ÍT khoá đổi LÕI hơn không?

:::opt{correct}
KHÔNG — `demSoKhoaDoiLoi(cacKhoa, 4, 3)` VẪN đổi phần LỚN khoá, cùng
LÝ do đã thấy Ở q11 bài 2: `% 4` VÀ `% 3` LÀ hai phép chia gần như
KHÔNG liên quan
:::

:::opt
CÓ, ÍT hơn — bớt lõi LÀ thao tác "NGƯỢC" của thêm lõi
::why
Gần đúng ở việc bạn nghĩ "NGƯỢC thao tác" nên "NGƯỢC mức độ ảnh
hưởng" — CÙNG trực giác ĐÃ xuất hiện (VÀ đã SAI) Ở q11 bài 2.

Chỗ lệch: mức độ XÁO trộn của `% soLoi` phụ thuộc CHÊNH lệch giữa
HAI phép chia lấy dư, KHÔNG phụ thuộc "thêm HAY bớt". `% 4` VÀ `% 3`
xáo trộn NGANG nhau bất kể ĐI theo chiều NÀO — bớt lõi gây xáo trộn
TƯƠNG đương thêm lõi, KHÔNG hề nhẹ hơn.
::
:::
::::

::::code{#viet_dem_so_khoa_doi_loi}
Hoàn thiện `demSoKhoaDoiLoi` — đếm số khoá CÓ lõi TRƯỚC khác lõi SAU.

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
function chonLoiChoKhoa(khoa: string, soLoi: number): number {
  return bam(khoa) % soLoi;
}

function demSoKhoaDoiLoi(cacKhoa: string[], soLoiTruoc: number, soLoiSau: number): number {
  let dem = 0;
  for (const khoa of cacKhoa) {
    if (___) {
      dem = dem + 1;
    }
  }
  return dem;
}

const cacKhoa: string[] = [];
for (let i = 0; i < 12; i++) cacKhoa.push("nguoidung" + i);
console.log(demSoKhoaDoiLoi(cacKhoa, 3, 4));
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
function chonLoiChoKhoa(khoa: string, soLoi: number): number {
  return bam(khoa) % soLoi;
}

function demSoKhoaDoiLoi(cacKhoa: string[], soLoiTruoc: number, soLoiSau: number): number {
  let dem = 0;
  for (const khoa of cacKhoa) {
    if (chonLoiChoKhoa(khoa, soLoiTruoc) !== chonLoiChoKhoa(khoa, soLoiSau)) {
      dem = dem + 1;
    }
  }
  return dem;
}

const cacKhoa: string[] = [];
for (let i = 0; i < 12; i++) cacKhoa.push("nguoidung" + i);
console.log(demSoKhoaDoiLoi(cacKhoa, 3, 4));
```

```typescript title=test
const cacKhoa2: string[] = [];
for (let i = 0; i < 12; i++) cacKhoa2.push("nguoidung" + i);
console.log(demSoKhoaDoiLoi(cacKhoa2, 3, 4));
if (demSoKhoaDoiLoi(cacKhoa2, 3, 4) !== 9) throw new Error("3 len 4 loi phai co dung 9/12 khoa doi loi");
if (demSoKhoaDoiLoi(cacKhoa2, 3, 3) !== 0) throw new Error("so loi KHONG doi thi khong khoa nao duoc doi");
if (demSoKhoaDoiLoi([], 3, 4) !== 0) throw new Error("danh sach khoa rong thi dem phai la 0");

const cacKhoa100: string[] = [];
for (let i = 0; i < 100; i++) cacKhoa100.push("nguoidung" + i);
if (demSoKhoaDoiLoi(cacKhoa100, 3, 4) !== 77) throw new Error("100 khoa, 3 len 4 loi phai co dung 77 khoa doi loi");
```

:::hints
- kind: attention
  body: "Dieu kien dem: loi cua khoa Ở soLoiTruoc KHAC loi cua no Ở soLoiSau -- mot dong."
- kind: strategy
  body: "chonLoiChoKhoa(khoa, soLoiTruoc) !== chonLoiChoKhoa(khoa, soLoiSau)"
- kind: one-line
  body: "if (chonLoiChoKhoa(khoa, soLoiTruoc) !== chonLoiChoKhoa(khoa, soLoiSau)) {"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "9"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`75%` khoá đổi lõi — y hệt bài toán q11 ĐÃ giải Ở quy mô máy. Cách
giải Ở q11 (vòng tròn + vnode) CÓ áp dụng được Ở quy mô lõi không?
::::

::::reflect{#nghi-lai}
`chonLoiChoKhoa` (bài 4) VÀ `chonNodeNgayTho` (q11 bài 1) LÀ cùng MỘT
Ý tưởng — VÀ chúng CHIA sẻ đúng CÙNG một điểm YẾU: `% soLoi` phụ
thuộc TOÀN bộ vào CHÍNH `soLoi`, nên đổi `soLoi` LÀ xáo trộn gần hết.
q11 (bài 3-8) ĐÃ giải quyết đúng vấn đề NÀY bằng vòng tròn hash VÀ
vnode — kỹ thuật ĐÓ có ÁP dụng lại được Ở quy mô LÕI, thay vì máy,
không?
::::

::::checkpoint{mastery=0.8}
::::
</content>
