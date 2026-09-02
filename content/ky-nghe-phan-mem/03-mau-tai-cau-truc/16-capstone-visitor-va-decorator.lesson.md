---
id: ky-nghe-phan-mem.mau-tai-cau-truc.capstone-visitor-va-decorator
title: "Capstone: Kết hợp Visitor (duyệt cây) + Decorator (đếm lượt gọi)"
summary: "Bài chốt cụm 4: bọc tinhGiaTri (bài 13) bằng voiDemLuotGoi (đếm SỐ LẦN hàm được gọi qua closure) — kiểm CẢ kết quả tính toán ĐÚNG (visitor) LẪN số đếm TĂNG đúng. Caveat thành thật: bộ đếm CHỈ tính lượt gọi QUA wrapper (.ham), KHÔNG tính các lần tinhGiaTri tự gọi ĐỆ QUY chính nó bên trong (những lần đó bỏ qua wrapper hoàn toàn)."
locale: vi
track: ky-nghe-phan-mem
module: mau-tai-cau-truc
order: 16
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [mau.gate-boss-visitor-decorator]
requires: [mau.decorator-as-wrapper]
concepts: [mau.gate-boss-visitor-decorator]
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
Bài chốt cụm 4. Muốn biết `tinhGiaTri` (bài 13) được gọi BAO NHIÊU
LẦN — bọc nó bằng một decorator ĐẾM, KHÔNG sửa THÂN hàm.
::::

::::explain{#boc-visitor-bang-decorator}
Ghép TOÀN BỘ cụm 4: `voiDemLuotGoi<A,B>(f)` (decorator, bài 15's kỹ
thuật) TRẢ VỀ một OBJECT nhóm HAI thứ (giống `taoKhoHang` bài 3) —
`ham` (bản BỌC của `f`, tăng bộ đếm MỖI lần gọi) VÀ `laySoLuotGoi`
(đọc bộ đếm) — bọc QUANH `tinhGiaTri` (visitor, bài 13), HOÀN TOÀN
KHÔNG sửa `tinhGiaTri` hay `BieuThuc`:

```typescript title=readonly
type BieuThuc =
  | { tag: "so"; giaTri: number }
  | { tag: "cong"; trai: BieuThuc; phai: BieuThuc }
  | { tag: "nhan"; trai: BieuThuc; phai: BieuThuc };

function tinhGiaTri(bt: BieuThuc): number {
  switch (bt.tag) {
    case "so": return bt.giaTri;
    case "cong": return tinhGiaTri(bt.trai) + tinhGiaTri(bt.phai);
    case "nhan": return tinhGiaTri(bt.trai) * tinhGiaTri(bt.phai);
  }
}

function voiDemLuotGoi<A, B>(f: (a: A) => B): { ham: (a: A) => B; laySoLuotGoi: () => number } {
  let soLuotGoi = 0;
  return {
    ham: (a: A) => {
      soLuotGoi += 1;
      return f(a);
    },
    laySoLuotGoi: () => soLuotGoi,
  };
}

const tinhGiaTriDemLuot = voiDemLuotGoi(tinhGiaTri);
const bt1: BieuThuc = { tag: "so", giaTri: 5 };
const bt2: BieuThuc = { tag: "cong", trai: bt1, phai: { tag: "so", giaTri: 3 } };

console.log(tinhGiaTriDemLuot.ham(bt1));
console.log(tinhGiaTriDemLuot.ham(bt2));
console.log(tinhGiaTriDemLuot.laySoLuotGoi());
```

```text title=readonly
5
8
2
```

Kết quả TÍNH TOÁN (`5`, `8`) HOÀN TOÀN ĐÚNG (visitor VẪN hoạt động Y
HỆT bài 13) — VÀ `laySoLuotGoi()` trả về `2` (đúng SỐ LẦN gọi
`.ham(...)`). Hai kỹ thuật (Visitor + Decorator) KẾT HỢP TỰ NHIÊN vì
CẢ HAI đều LÀ "chỉ là hàm".
::::

::::example{#dem-khong-tinh-de-quy-noi-bo}
GIỚI HẠN THÀNH THẬT: bộ đếm CHỈ tính lượt gọi **QUA** `.ham(...)` —
KHÔNG tính các lần `tinhGiaTri` TỰ gọi CHÍNH NÓ (đệ quy BÊN TRONG,
bỏ qua wrapper HOÀN TOÀN):

```typescript title=readonly
type BieuThuc =
  | { tag: "so"; giaTri: number }
  | { tag: "cong"; trai: BieuThuc; phai: BieuThuc }
  | { tag: "nhan"; trai: BieuThuc; phai: BieuThuc };
function tinhGiaTri(bt: BieuThuc): number {
  switch (bt.tag) {
    case "so": return bt.giaTri;
    case "cong": return tinhGiaTri(bt.trai) + tinhGiaTri(bt.phai);
    case "nhan": return tinhGiaTri(bt.trai) * tinhGiaTri(bt.phai);
  }
}
function voiDemLuotGoi<A, B>(f: (a: A) => B): { ham: (a: A) => B; laySoLuotGoi: () => number } {
  let soLuotGoi = 0;
  return {
    ham: (a: A) => { soLuotGoi += 1; return f(a); },
    laySoLuotGoi: () => soLuotGoi,
  };
}

const bt: BieuThuc = {
  tag: "nhan",
  trai: { tag: "cong", trai: { tag: "so", giaTri: 2 }, phai: { tag: "so", giaTri: 3 } },
  phai: { tag: "so", giaTri: 4 },
};
const demo = voiDemLuotGoi(tinhGiaTri);
console.log(demo.ham(bt));
console.log(demo.laySoLuotGoi());
```

```text title=readonly
20
1
```

`bt` CÓ NĂM node (`nhan`, `cong`, ba `so`) — TÍNH `tinhGiaTri(bt)`
THẬT SỰ chạy `tinhGiaTri` **NĂM LẦN** (đệ quy VÀO từng node), NHƯNG
`laySoLuotGoi()` CHỈ trả `1` — vì BỐN trong NĂM lần gọi ĐÓ LÀ
`tinhGiaTri` GỌI **CHÍNH NÓ TRỰC TIẾP** (`return tinhGiaTri(bt.trai)
+ ...`), KHÔNG ĐI QUA `demo.ham`. CHỈ lần gọi ĐẦU TIÊN (`demo.ham(bt)`)
mới ĐƯỢC đếm. Đây LÀ giới hạn THẬT của decorator bọc hàm ĐỆ QUY: nó
CHỈ thấy được "cổng vào", KHÔNG thấy được những gì XẢY RA BÊN TRONG.
::::

::::predict{#doan-dem-khong-tang-theo-do-sau commitOnce}
```typescript
type BieuThuc =
  | { tag: "so"; giaTri: number }
  | { tag: "cong"; trai: BieuThuc; phai: BieuThuc }
  | { tag: "nhan"; trai: BieuThuc; phai: BieuThuc };
function tinhGiaTri(bt: BieuThuc): number {
  switch (bt.tag) {
    case "so": return bt.giaTri;
    case "cong": return tinhGiaTri(bt.trai) + tinhGiaTri(bt.phai);
    case "nhan": return tinhGiaTri(bt.trai) * tinhGiaTri(bt.phai);
  }
}
function voiDemLuotGoi<A, B>(f: (a: A) => B): { ham: (a: A) => B; laySoLuotGoi: () => number } {
  let soLuotGoi = 0;
  return {
    ham: (a: A) => { soLuotGoi += 1; return f(a); },
    laySoLuotGoi: () => soLuotGoi,
  };
}

const btSau: BieuThuc = {
  tag: "cong",
  trai: { tag: "nhan", trai: { tag: "cong", trai: { tag: "so", giaTri: 1 }, phai: { tag: "so", giaTri: 2 } }, phai: { tag: "so", giaTri: 3 } },
  phai: { tag: "so", giaTri: 4 },
};
const demo = voiDemLuotGoi(tinhGiaTri);
demo.ham(btSau);
demo.ham(btSau);
console.log(demo.laySoLuotGoi());
```

`btSau` LỒNG SÂU BỐN tầng, VÀ được gọi HAI LẦN QUA `.ham`. Dòng cuối
in ra gì?

:::opt{correct}
`2`
:::

:::opt
`8` — vì CÂY `btSau` lồng SÂU (nhiều node HƠN), NÊN MỖI lần gọi
`.ham` sẽ đếm THEO ĐỘ SÂU của cây (ĐỘ SÂU CÀNG lớn, bộ đếm CÀNG tăng
NHIỀU MỖI lần gọi)
::why
Gần đúng ở việc bạn nhớ ĐÚNG `btSau` LỒNG SÂU HƠN ví dụ TRƯỚC (bốn
tầng thay VÌ hai) — quan sát ĐÓ về CẤU TRÚC dữ liệu chính xác.

Chỗ lệch: ĐỘ SÂU của cây **KHÔNG ẢNH HƯỞNG** tới bộ đếm — `soLuotGoi`
CHỈ tăng Ở ĐÚNG MỘT chỗ (`ham: (a) => { soLuotGoi += 1; ... }`), VÀ
chỗ ĐÓ CHỈ chạy khi `.ham(...)` được gọi TRỰC TIẾP TỪ NGOÀI. MỌI lần
đệ quy BÊN TRONG `tinhGiaTri` (dù cây SÂU bao nhiêu tầng) ĐỀU gọi
`tinhGiaTri` TRỰC TIẾP, KHÔNG QUA `.ham`. Gọi `.ham(btSau)` **HAI
LẦN** (BẤT KỂ `btSau` sâu CỠ NÀO) → bộ đếm CHỈ LÀ `2`.
::
:::

:::opt
Máy báo lỗi lúc chạy — `voiDemLuotGoi(tinhGiaTri)` được gọi để tạo
`demo`, RỒI `demo.ham` được gọi VỚI CÙNG một `btSau` HAI LẦN LIÊN
TIẾP, TypeScript/JavaScript coi việc gọi CÙNG một hàm bọc VỚI CÙNG
đối số NHIỀU LẦN LÀ thao tác KHÔNG hợp lệ (trùng lặp)
::why
Gần đúng ở việc bạn để ý `demo.ham(btSau)` được gọi HAI LẦN LIÊN TIẾP
VỚI CÙNG một giá trị — một quan sát ĐÚNG về CẤU TRÚC code.

Chỗ lệch: KHÔNG CÓ quy tắc nào cấm gọi MỘT hàm NHIỀU LẦN VỚI CÙNG đối
số — đây LÀ thao tác BÌNH THƯỜNG, PHỔ BIẾN (ví dụ: TÍNH LẠI cùng một
biểu thức nhiều lần Ở nhiều chỗ khác NHAU trong chương trình). Chạy
SẠCH, không lỗi nào cả — VÀ ĐÂY CHÍNH LÀ lý do bộ đếm HỮU ÍCH: để
BIẾT MỘT hàm ĐANG bị gọi LẠI (có thể LÃNG PHÍ) bao nhiêu LẦN.
::
:::
::::

::::code{#viet_capstone_visitor_decorator}
Hoàn thiện `voiDemLuotGoi` — `ham` tăng bộ đếm RỒI gọi `f`, `laySoLuotGoi`
đọc bộ đếm HIỆN TẠI.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type BieuThuc =
  | { tag: "so"; giaTri: number }
  | { tag: "cong"; trai: BieuThuc; phai: BieuThuc }
  | { tag: "nhan"; trai: BieuThuc; phai: BieuThuc };

function tinhGiaTri(bt: BieuThuc): number {
  switch (bt.tag) {
    case "so": return bt.giaTri;
    case "cong": return tinhGiaTri(bt.trai) + tinhGiaTri(bt.phai);
    case "nhan": return tinhGiaTri(bt.trai) * tinhGiaTri(bt.phai);
  }
}

function voiDemLuotGoi<A, B>(f: (a: A) => B): { ham: (a: A) => B; laySoLuotGoi: () => number } {
  let soLuotGoi = 0;
  return {
    ham: (a: A) => {
      ___;
      return f(a);
    },
    laySoLuotGoi: () => ___,
  };
}

const bt1: BieuThuc = { tag: "so", giaTri: 5 };
const demo1 = voiDemLuotGoi(tinhGiaTri);
assertEqual(demo1.ham(bt1), 5, "ket qua dung");
assertEqual(demo1.laySoLuotGoi(), 1, "dem dung 1 lan goi");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type BieuThuc =
  | { tag: "so"; giaTri: number }
  | { tag: "cong"; trai: BieuThuc; phai: BieuThuc }
  | { tag: "nhan"; trai: BieuThuc; phai: BieuThuc };

function tinhGiaTri(bt: BieuThuc): number {
  switch (bt.tag) {
    case "so": return bt.giaTri;
    case "cong": return tinhGiaTri(bt.trai) + tinhGiaTri(bt.phai);
    case "nhan": return tinhGiaTri(bt.trai) * tinhGiaTri(bt.phai);
  }
}

function voiDemLuotGoi<A, B>(f: (a: A) => B): { ham: (a: A) => B; laySoLuotGoi: () => number } {
  let soLuotGoi = 0;
  return {
    ham: (a: A) => {
      soLuotGoi += 1;
      return f(a);
    },
    laySoLuotGoi: () => soLuotGoi,
  };
}

const bt1: BieuThuc = { tag: "so", giaTri: 5 };
const demo1 = voiDemLuotGoi(tinhGiaTri);
assertEqual(demo1.ham(bt1), 5, "ket qua dung");
assertEqual(demo1.laySoLuotGoi(), 1, "dem dung 1 lan goi");
```

```typescript title=test
assertEqual(demo1.ham(bt1), 5, "goi lan hai van dung ket qua");
assertEqual(demo1.laySoLuotGoi(), 2, "dem tang len 2 sau lan goi thu hai");

const bt2: BieuThuc = {
  tag: "nhan",
  trai: { tag: "cong", trai: { tag: "so", giaTri: 2 }, phai: { tag: "so", giaTri: 3 } },
  phai: { tag: "so", giaTri: 4 },
};
const demo2 = voiDemLuotGoi(tinhGiaTri);
assertEqual(demo2.ham(bt2), 20, "tinh dung gia tri cay long nhau");
assertEqual(demo2.laySoLuotGoi(), 1, "chi dem MOT lan goi qua wrapper, khong dem cac lan de quy noi bo");
assertEqual(demo1.laySoLuotGoi(), 2, "demo1 khong bi anh huong boi demo2 -- doc lap");
```

:::hints
- kind: attention
  body: "ham: tăng soLuotGoi lên 1 (dùng +=), RỒI gọi f(a). laySoLuotGoi: trả VỀ soLuotGoi hiện tại."
- kind: strategy
  body: "soLuotGoi += 1 : soLuotGoi — tăng biến đếm nội bộ (closure), và đọc giá trị hiện tại của nó."
- kind: one-line
  body: '___ (ham) = soLuotGoi += 1\n___ (laySoLuotGoi) = soLuotGoi'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "PASS"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cụm 4 hoàn tất: Visitor=switch, thêm thao tác không sửa ADT, Decorator
= wrapper, capstone kết hợp cả hai. Cụm tiếp theo: Middleware &
Tái cấu trúc.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`voiGhiLog`/`voiDemLuotGoi` (bài 15-16) MỖI cái bọc MỘT hàm. Nếu CẦN
BỌC một giá trị QUA **NHIỀU** decorator LIÊN TIẾP (validate → log →
xử lý, giống middleware Express) — làm sao GHÉP chúng lại?
::::

::::checkpoint{mastery=0.8}
::::
