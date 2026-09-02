---
id: ky-nghe-phan-mem.mau-tai-cau-truc.visitor-la-exhaustive-switch
title: "Visitor = Exhaustive Switch — duyệt ADT không cần double dispatch"
summary: "OOP Visitor: xử lý cây cú pháp KHÔNG đồng nhất cần \"double dispatch\" — mỗi node class có accept(visitor), thêm thao tác MỚI cần sửa MỌI class. FP: cây LÀ một ADT đệ quy (BieuThuc), \"visitor\" CHỈ LÀ một hàm đệ quy với exhaustive switch. tinhGiaTri(bt): number tính giá trị BIỂU THỨC theo đúng CẤU TRÚC CÂY, không phải đọc tuyến tính."
locale: vi
track: ky-nghe-phan-mem
module: mau-tai-cau-truc
order: 13
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [mau.visitor-as-switch]
requires: [mau.gate-boss-command]
concepts: [mau.visitor-as-switch]
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
Cụm mới. Một biểu thức toán học `(2 + 3) * 4` KHÔNG PHẢI một chuỗi
phẳng — nó LÀ một CÂY. Tính giá trị của CÂY đó như thế nào?
::::

::::explain{#double-dispatch-vs-switch}
OOP Visitor: XỬ LÝ một cây cú pháp KHÔNG đồng nhất (nhiều LOẠI node)
cần "double dispatch" — MỖI node class có method `accept(visitor)`,
MỖI thao tác MỚI (tính giá trị, IN cây, tối ưu) cần SỬA **MỌI** class
node để thêm `accept` MỚI.

FP: cây LÀ một **ADT đệ quy** (`BieuThuc`, ba nhánh: số, cộng, nhân —
`cong`/`nhan` mỗi cái CHỨA HAI `BieuThuc` con) — "visitor" CHỈ LÀ MỘT
hàm **ĐỆ QUY** với exhaustive `switch`:

```typescript title=readonly
type BieuThuc =
  | { tag: "so"; giaTri: number }
  | { tag: "cong"; trai: BieuThuc; phai: BieuThuc }
  | { tag: "nhan"; trai: BieuThuc; phai: BieuThuc };

function tinhGiaTri(bt: BieuThuc): number {
  switch (bt.tag) {
    case "so":
      return bt.giaTri;
    case "cong":
      return tinhGiaTri(bt.trai) + tinhGiaTri(bt.phai);
    case "nhan":
      return tinhGiaTri(bt.trai) * tinhGiaTri(bt.phai);
  }
}

// (2 + 3) * 4
const bt: BieuThuc = {
  tag: "nhan",
  trai: { tag: "cong", trai: { tag: "so", giaTri: 2 }, phai: { tag: "so", giaTri: 3 } },
  phai: { tag: "so", giaTri: 4 },
};

console.log(tinhGiaTri(bt));
```

```text title=readonly
20
```

`tinhGiaTri` GỌI CHÍNH NÓ (đệ quy) trên `bt.trai`/`bt.phai` — với
node `"nhan"`: TÍNH `trai` (nhánh `"cong"`, ĐỆ QUY tính `2+3=5`) VÀ
`phai` (`4`), RỒI NHÂN. KHÔNG interface `Visitor`, KHÔNG method
`accept` nào cả — CHỈ MỘT hàm.
::::

::::example{#cau-truc-cay-quyet-dinh-ket-qua}
CÙNG BA con số (`2`, `3`, `4`) VÀ HAI phép toán (`+`, `*`) — NHƯNG
**CẤU TRÚC CÂY** (KHÔNG PHẢI thứ tự đọc) quyết định KẾT QUẢ:

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

// 2 * (3 + 4)
const bt: BieuThuc = {
  tag: "nhan",
  trai: { tag: "so", giaTri: 2 },
  phai: { tag: "cong", trai: { tag: "so", giaTri: 3 }, phai: { tag: "so", giaTri: 4 } },
};
console.log(tinhGiaTri(bt));
```

```text title=readonly
14
```

`nhan` có `trai = 2` (số ĐƠN) VÀ `phai` LÀ MỘT nhánh `"cong"` LỒNG
BÊN TRONG (`3 + 4`) — `tinhGiaTri` PHẢI đệ quy VÀO `phai` TRƯỚC (tính
`3+4=7`) RỒI mới nhân VỚI `2` (`2*7=14`), KHÁC HẲN ví dụ TRÊN
(`(2+3)*4=20`) dù CÙNG dùng BA số VÀ HAI phép toán.
::::

::::predict{#doan-cay-long-sau commitOnce}
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

// nhan { trai: so(2), phai: nhan { trai: so(3), phai: cong { trai: so(4), phai: so(5) } } }
const bt: BieuThuc = {
  tag: "nhan",
  trai: { tag: "so", giaTri: 2 },
  phai: {
    tag: "nhan",
    trai: { tag: "so", giaTri: 3 },
    phai: { tag: "cong", trai: { tag: "so", giaTri: 4 }, phai: { tag: "so", giaTri: 5 } },
  },
};
console.log(tinhGiaTri(bt));
```

Dòng cuối in ra gì?

:::opt{correct}
`54`
:::

:::opt
`19` — vì học đọc `bt` THEO thứ tự CÁC con số XUẤT HIỆN trong code
(`2, 3, 4, 5`) VÀ áp dụng QUY TẮC ưu tiên toán học CHUẨN (nhân TRƯỚC
cộng): `2*3 + 4 + 5`
::why
Gần đúng ở việc bạn nhớ ĐÚNG BỐN con số XUẤT HIỆN LÀ `2, 3, 4, 5` —
quan sát ĐÓ, VỀ MẶT LIỆT KÊ, chính xác.

Chỗ lệch: `tinhGiaTri` KHÔNG đọc `bt` NHƯ MỘT chuỗi tuyến tính ÁP DỤNG
quy tắc ưu tiên toán học — nó đi THEO **CẤU TRÚC LỒNG NHAU THẬT** của
object. `bt` LÀ `nhan(2, nhan(3, cong(4, 5)))` — TÍNH TỪ TRONG RA:
`cong(4,5) = 9`, RỒI `nhan(3, 9) = 27`, RỒI `nhan(2, 27) = 54`. Việc
"đọc TỪ NGOÀI VÀO cấu trúc lồng" (KHÔNG PHẢI "đọc số theo THỨ TỰ xuất
hiện Ở code") MỚI LÀ cách `tinhGiaTri` THẬT SỰ hoạt động.
::
:::

:::opt
Máy báo lỗi biên dịch — `bt.phai` được gán MỘT giá trị có `tag:
"nhan"` LỒNG BÊN TRONG một node CŨNG có `tag: "nhan"` (Ở NGOÀI CÙNG),
TypeScript KHÔNG cho HAI node CÙNG `tag` xuất hiện LỒNG nhau trong
MỘT `BieuThuc`
::why
Gần đúng ở việc bạn để ý CẤU TRÚC `bt` có HAI node CÙNG `tag: "nhan"`
LỒNG vào NHAU (một Ở NGOÀI, một Ở `phai`) — một quan sát ĐÚNG về HÌNH
DẠNG dữ liệu.

Chỗ lệch: `BieuThuc` LÀ một kiểu **ĐỆ QUY** — `trai`/`phai` của MỖI
node `cong`/`nhan` khai kiểu `BieuThuc` (CHÍNH kiểu đó), CHO PHÉP lồng
BAO NHIÊU tầng TUỲ Ý, VÀ hoàn toàn KHÔNG có quy tắc nào cấm HAI node
CÙNG `tag` lồng NHAU — đây CHÍNH LÀ cách BIỂU DIỄN một biểu thức LỚN
(`2 * (3 * (4 + 5))`) TỰ NHIÊN. Biên dịch SẠCH.
::
:::
::::

::::code{#viet_tinh_gia_tri}
Hoàn thiện `tinhGiaTri` — đệ quy TÍNH giá trị hai nhánh RỒI kết hợp
theo ĐÚNG phép toán.

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
    case "so":
      return bt.giaTri;
    case "cong":
      return ___;
    case "nhan":
      return ___;
  }
}

const bt1: BieuThuc = { tag: "so", giaTri: 7 };
assertEqual(tinhGiaTri(bt1), 7, "so don gian");

const bt2: BieuThuc = { tag: "cong", trai: { tag: "so", giaTri: 3 }, phai: { tag: "so", giaTri: 4 } };
assertEqual(tinhGiaTri(bt2), 7, "cong hai so");
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
    case "so":
      return bt.giaTri;
    case "cong":
      return tinhGiaTri(bt.trai) + tinhGiaTri(bt.phai);
    case "nhan":
      return tinhGiaTri(bt.trai) * tinhGiaTri(bt.phai);
  }
}

const bt1: BieuThuc = { tag: "so", giaTri: 7 };
assertEqual(tinhGiaTri(bt1), 7, "so don gian");

const bt2: BieuThuc = { tag: "cong", trai: { tag: "so", giaTri: 3 }, phai: { tag: "so", giaTri: 4 } };
assertEqual(tinhGiaTri(bt2), 7, "cong hai so");
```

```typescript title=test
const bt3: BieuThuc = {
  tag: "nhan",
  trai: { tag: "cong", trai: { tag: "so", giaTri: 2 }, phai: { tag: "so", giaTri: 3 } },
  phai: { tag: "so", giaTri: 4 },
};
assertEqual(tinhGiaTri(bt3), 20, "(2+3)*4");

const bt4: BieuThuc = {
  tag: "nhan",
  trai: { tag: "so", giaTri: 2 },
  phai: { tag: "cong", trai: { tag: "so", giaTri: 3 }, phai: { tag: "so", giaTri: 4 } },
};
assertEqual(tinhGiaTri(bt4), 14, "2*(3+4)");

const bt5: BieuThuc = {
  tag: "cong",
  trai: { tag: "nhan", trai: { tag: "cong", trai: { tag: "so", giaTri: 5 }, phai: { tag: "so", giaTri: 6 } }, phai: { tag: "so", giaTri: 2 } },
  phai: { tag: "so", giaTri: 1 },
};
assertEqual(tinhGiaTri(bt5), 23, "((5+6)*2)+1");
```

:::hints
- kind: attention
  body: "cong: đệ quy tính trai VÀ phai, RỒI cộng hai kết quả. nhan: y hệt nhưng nhân thay vì cộng."
- kind: strategy
  body: "tinhGiaTri(bt.trai) + tinhGiaTri(bt.phai) : tinhGiaTri(bt.trai) * tinhGiaTri(bt.phai) — gọi đệ quy trên hai nhánh, rồi kết hợp theo đúng phép toán."
- kind: one-line
  body: '___ (cong) = tinhGiaTri(bt.trai) + tinhGiaTri(bt.phai)\n___ (nhan) = tinhGiaTri(bt.trai) * tinhGiaTri(bt.phai)'
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
Visitor = hàm đệ quy + exhaustive switch trên ADT. Bài tiếp theo:
thêm MỘT thao tác MỚI (không PHẢI tính giá trị) mà KHÔNG sửa ADT.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`tinhGiaTri` LÀ MỘT hàm "duyệt" `BieuThuc`. Nếu muốn THÊM một thao
tác KHÁC — ví dụ IN biểu thức RA dạng chữ (`"(2 + 3) * 4"`) — có cần
sửa ĐỊNH NGHĨA `BieuThuc` hay `tinhGiaTri` KHÔNG?
::::

::::checkpoint{mastery=0.8}
::::
