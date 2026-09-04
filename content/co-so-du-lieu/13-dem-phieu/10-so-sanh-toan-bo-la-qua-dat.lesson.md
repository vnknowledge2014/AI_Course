---
id: co-so-du-lieu.dem-phieu.so-sanh-toan-bo-la-qua-dat
title: "So sánh toàn bộ là quá đắt"
summary: "soKhoaLechBangSoSanhTung duyệt HỢP của mọi khoá giữa hai bản sao, so sánh TỪNG cặp giá trị -- với 1000 khoá và CHỈ 3 khoá thực sự lệch, hàm vẫn phải chạm ĐỦ 1000 khoá để biết được điều đó. Read repair (bài 9) chỉ sửa khoá CÓ ai đọc; một tiến trình anti-entropy định kỳ cần so sánh HAI bản sao TOÀN BỘ để bắt phần còn sót -- nhưng chi phí O(n) này không phụ thuộc số khoá lệch, chỉ phụ thuộc tổng kích thước dữ liệu."
locale: vi
track: co-so-du-lieu
module: dem-phieu
order: 10
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [db.so-sanh-toan-bo-la-qua-dat]
requires: [db.doc-thay-khac-nhau-read-repair]
concepts: [db.so-sanh-toan-bo-la-qua-dat]
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
Read repair (bài trước) chỉ sửa khoá CÓ ai đọc lại. Nếu MUỐN chủ
động quét TÌM mọi khoá lệch giữa hai bản sao — kể cả khoá CHẲNG ai
đụng tới — cách đơn giản nhất TỐN bao nhiêu công?
::::

::::explain{#so-khoa-lech-bang-so-sanh-tung}
`soKhoaLechBangSoSanhTung` duyệt HỢP của mọi khoá xuất hiện Ở HAI
bản sao, so sánh TỪNG cặp giá trị — đếm số khoá KHÁC nhau (hoặc chỉ
tồn tại Ở một BÊN):

```typescript title=readonly
function soKhoaLechBangSoSanhTung(khoA: Map<string, number>, khoB: Map<string, number>): number {
  const tatCaKhoa = new Set([...khoA.keys(), ...khoB.keys()]);
  let dem = 0;
  for (const k of tatCaKhoa) {
    if (khoA.get(k) !== khoB.get(k)) dem++;
  }
  return dem;
}

const khoA = new Map<string, number>();
const khoB = new Map<string, number>();
for (let i = 0; i < 1000; i++) { khoA.set("khoa" + i, i); khoB.set("khoa" + i, i); }
khoB.set("khoa5", 9999);   // gia tri khac
khoB.set("khoa500", 8888); // gia tri khac
khoB.delete("khoa999");    // chi con o mot ben

console.log("tong so khoa (hop):", new Set([...khoA.keys(), ...khoB.keys()]).size);
console.log("so khoa lech:", soKhoaLechBangSoSanhTung(khoA, khoB));
```

```text title=readonly
tong so khoa (hop): 1000
so khoa lech: 3
```

CHỈ CÓ `3` khoá thực SỰ lệch trong `1000` khoá — NHƯNG để BIẾT được
con số `3` đó, hàm phải chạm ĐỦ cả `1000` khoá (vòng `for` duyệt
`tatCaKhoa`, KHÔNG có cách nào bỏ QUA phần "chắc chắn giống nhau" mà
không kiểm TRA). Chi phí LÀ `O(n)` VỚI `n` LÀ tổng số khoá — KHÔNG
phụ thuộc và SỐ khoá thực sự lệch.
::::

::::example{#khong-lien-quan-so-luong-lech}
Nếu chỉ `1` khoá lệch giữa `1000000` khoá, `soKhoaLechBangSoSanhTung`
VẪN phải duyệt qua CẢ triệu khoá đó để tìm RA nó — chi phí không hề
"tự động rẻ hơn" chỉ VÌ phần lớn dữ liệu giống NHAU. Đây chính LÀ vấn
đề cần giải quyết: MỘT tiến trình anti-entropy chạy ĐỊNH kỳ (mỗi vài
phút, giữa MỖI cặp bản sao) mà TỐN `O(n)` mỗi lần LÀ không khả thi Ở
quy mô hàng triệu khoá.
::::

::::predict{#doan-hai-kho-giong-het commitOnce}
Hai bản sao GIỐNG HỆT nhau hoàn TOÀN (`1000000` khoá, không lệch một
khoá NÀO). Gọi `soKhoaLechBangSoSanhTung` — chi phí (số lần SO sánh
thực hiện) CÓ rẻ hơn trường hợp CÓ vài khoá lệch KHÔNG?

:::opt{correct}
KHÔNG — vẫn phải duyệt VÀ so sánh đủ `1000000` khoá để XÁC nhận
"giống hệt", chi phí GIỐNG hệt trường hợp có vài khoá lệch; kết quả
(`0`) rẻ để BÁO cáo, nhưng quá trình ĐỂ đi tới kết quả đó không rẻ
hơn
:::

:::opt
CÓ — nếu hai `Map` giống hệt, thuật TOÁN có thể dừng sớm ngay khi
phát hiện không CÓ khác biệt nào
::why
Trực giác NÀY hợp lý cho MỘT số thuật toán so sánh KHÁC (VÍ dụ so
sánh THAM chiếu `===` cho hai object giống hệt Ở bộ NHỚ) — nhưng
`soKhoaLechBangSoSanhTung` không hề CÀI đặt kiểu "dừng sớm" đó.

Chỗ lệch: vòng lặp `for (const k of tatCaKhoa)` duyệt hết TOÀN bộ
tập hợp khoá, KHÔNG có điều kiện thoát SỚM nào cả — kể cả khi TẤT cả
các lần so sánh TRƯỚC đó đều "bằng nhau", vòng lặp vẫn TIẾP tục kiểm
tra khoá tiếp THEO. Không có cách NÀO biết "phần còn lại chắc chắn
giống nhau" MÀ không thực sự kiểm tra từng khoá MỘT — đây chính LÀ
lý do cần một cấu trúc dữ liệu THÔNG minh hơn (bài sau).
::
:::
::::

::::code{#viet_so_khoa_lech_bang_so_sanh_tung}
Hoàn thiện `soKhoaLechBangSoSanhTung` — đếm số khoá CÓ giá trị khác
nhau giữa hai bản sao.

```typescript title=starter
function soKhoaLechBangSoSanhTung(khoA: Map<string, number>, khoB: Map<string, number>): number {
  const tatCaKhoa = new Set([...khoA.keys(), ...khoB.keys()]);
  let dem = 0;
  for (const k of tatCaKhoa) {
    ___
  }
  return dem;
}

const khoA = new Map<string, number>();
const khoB = new Map<string, number>();
for (let i = 0; i < 1000; i++) { khoA.set("khoa" + i, i); khoB.set("khoa" + i, i); }
khoB.set("khoa5", 9999);
khoB.set("khoa500", 8888);
khoB.delete("khoa999");
console.log(soKhoaLechBangSoSanhTung(khoA, khoB));
```

```typescript title=solution
function soKhoaLechBangSoSanhTung(khoA: Map<string, number>, khoB: Map<string, number>): number {
  const tatCaKhoa = new Set([...khoA.keys(), ...khoB.keys()]);
  let dem = 0;
  for (const k of tatCaKhoa) {
    if (khoA.get(k) !== khoB.get(k)) dem++;
  }
  return dem;
}

const khoA = new Map<string, number>();
const khoB = new Map<string, number>();
for (let i = 0; i < 1000; i++) { khoA.set("khoa" + i, i); khoB.set("khoa" + i, i); }
khoB.set("khoa5", 9999);
khoB.set("khoa500", 8888);
khoB.delete("khoa999");
console.log(soKhoaLechBangSoSanhTung(khoA, khoB));
```

```typescript title=test
if (soKhoaLechBangSoSanhTung(khoA, khoB) !== 3) throw new Error("phai dem dung 3 khoa lech (2 gia tri khac, 1 chi co o mot ben)");

const khoGiongHet = new Map(khoA);
if (soKhoaLechBangSoSanhTung(khoA, khoGiongHet) !== 0) throw new Error("hai kho giong het nhau phai cho 0 lech");

const khoRong = new Map<string, number>();
if (soKhoaLechBangSoSanhTung(khoRong, khoRong) !== 0) throw new Error("hai kho rong phai cho 0 lech");

const khoChiA = new Map<string, number>([["x", 1]]);
const khoChiB = new Map<string, number>([["y", 2]]);
if (soKhoaLechBangSoSanhTung(khoChiA, khoChiB) !== 2) throw new Error("khong co khoa chung nao thi ca 2 khoa deu tinh la lech");

const khoMotKhoaKhac = new Map<string, number>([["x", 1], ["y", 2]]);
const khoMotKhoaKhac2 = new Map<string, number>([["x", 1], ["y", 3]]);
if (soKhoaLechBangSoSanhTung(khoMotKhoaKhac, khoMotKhoaKhac2) !== 1) throw new Error("chi 1 gia tri khac nhau (y: 2 vs 3) thi phai dem dung 1");
```

:::hints
- kind: attention
  body: "Neu gia tri cua k O hai kho khac nhau (ke ca undefined) thi dem tang -- mot dong."
- kind: strategy
  body: "if (khoA.get(k) !== khoB.get(k)) dem++;"
- kind: one-line
  body: "if (khoA.get(k) !== khoB.get(k)) dem++;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "3"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
So sánh từng khoá LÀ đúng nhưng đắt. Có cách NÀO so sánh HAI bản sao
mà KHÔNG phải chạm từng khoá — trừ khi thật SỰ cần?
::::

::::reflect{#nghi-lai}
`soKhoaLechBangSoSanhTung` không SAI — nó cho kết quả CHÍNH xác
tuyệt đối. Vấn đề DUY nhất LÀ chi phí: `O(n)` cho MỖI lần so sánh,
BẤT kể có bao nhiêu khoá thực sự LỆCH. Với một hệ THỐNG chạy anti-
entropy định kỳ GIỮA nhiều cặp bản sao, chi phí NÀY cộng dồn rất
nhanh. Câu hỏi cần trả LỜI: LÀM sao biết "PHẦN nào của dữ liệu chắc
chắn giống nhau" MÀ không cần so sánh từng khoá MỘT trong phần đó?
::::

::::checkpoint{mastery=0.8}
::::
