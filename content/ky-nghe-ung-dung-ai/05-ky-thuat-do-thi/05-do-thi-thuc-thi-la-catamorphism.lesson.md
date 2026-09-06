---
id: ky-nghe-ung-dung-ai.ky-thuat-do-thi.do-thi-thuc-thi-la-catamorphism
title: "Cầu nối FP — cấu trúc rẽ nhánh của đồ thị LÀ catamorphism trên cây đệ quy"
summary: "Cầu nối FP (dựa trên chapter-30c-recursive-types-folds.chapter.md, phần Tree Fold: 'tách TRAVERSAL khỏi COMPUTATION' — foldTree duyệt cây, handlers (onLa/onNhanh) quyết định NGHIỆP VỤ). CayThucThi = { loai:'la'; tenNode:string } | { loai:'nhanh'; dieuKien:boolean; neuDung:CayThucThi; neuSai:CayThucThi } biểu diễn TĨNH cấu trúc rẽ nhánh của một đồ thị (giống bài 2-3). foldCayThucThi<R>(onLa, onNhanh) LÀ catamorphism: gấp đệ quy từ LÁ lên GỐC, luôn tính CẢ HAI nhánh (ketQuaDung VÀ ketQuaSai) trước khi onNhanh quyết định cách GỘP. Hai interpretation của CÙNG một cây taoCayHaiTang(giaTri, coTheThuLai) (2 tầng rẽ nhánh, 3 lá): demSoLa (onNhanh cộng CẢ HAI nhánh) LUÔN trả về 3 bất kể dieuKien runtime là gì — demSoLa(taoCayHaiTang(5,true))=demSoLa(taoCayHaiTang(-3,true))=demSoLa(taoCayHaiTang(-3,false))=3. thucThi (onNhanh CHỌN một nhánh theo dieuKien) trả về KẾT QUẢ PHỤ THUỘC input: thucThi(taoCayHaiTang(5,true))='xuLyHopLe', thucThi(taoCayHaiTang(-3,true))='thuLai', thucThi(taoCayHaiTang(-3,false))='boCuoc' — khớp CHÍNH XÁC với node CUỐI mà chayDoThi (bài 1-3) đi tới trên cùng bộ tham số, xác nhận: diễn giải MỘT cấu trúc cây theo NHIỀU cách (đếm lá, thực thi thật) không cần viết lại traversal lần nào, chỉ đổi PHÉP TOÁN gấp (onNhanh)."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-do-thi
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [kna.do-thi-thuc-thi-la-catamorphism]
requires: [kna.checkpoint-va-resume]
concepts: [kna.do-thi-thuc-thi-la-catamorphism]
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
Bốn bài trước viết TAY logic rẽ nhánh MỖI khi cần một cách diễn giải mới
— node `"kiemTra"` (bài `2`) tự chứa `if/else`, `chayDoThi` (bài `1`) tự
chứa vòng `while` đi qua từng node. `chapter-30c-recursive-types-folds`
dạy một pattern tách RIÊNG hai việc đó: **fold (catamorphism)** — MỘT
hàm duy nhất lo việc DUYỆT (traversal) cấu trúc đệ quy, VÀ các
"handler" lo việc TÍNH TOÁN (nghiệp vụ). Bài này áp pattern đó VÀO
đúng cấu trúc rẽ nhánh của GRAPH.
::::

::::explain{#cay_thuc_thi_va_fold}
`chapter-30c` dạy `foldTree`: một hàm nhận `onLeaf` VÀ `onBranch`, tự đi
xuống LÁ, rồi CUỘN kết quả LÊN GỐC — "Cata" (đi xuống) + "morph" (biến
đổi hình thái). Ta dựng một cấu trúc TƯƠNG TỰ cho đúng hình dạng rẽ
nhánh Ở bài `2`-`3`: `CayThucThi` LÀ MỘT node lá (`"la"`, mang tên một
node THẬT của đồ thị) HOẶC MỘT node rẽ nhánh (`"nhanh"`, mang một
`dieuKien` VÀ HAI cây con `neuDung`/`neuSai`) — Y HỆT cấu trúc `kiemTra`
VÀ `xuLyLoi` Ở bài `2`-`3`, nhưng biểu diễn TĨNH dưới dạng DỮ LIỆU thay
vì hàm:

```typescript title=readonly
type CayThucThi =
  | { loai: "la"; tenNode: string }
  | { loai: "nhanh"; dieuKien: boolean; neuDung: CayThucThi; neuSai: CayThucThi };

function foldCayThucThi<R>(
  onLa: (tenNode: string) => R,
  onNhanh: (dieuKien: boolean, ketQuaDung: R, ketQuaSai: R) => R,
): (cay: CayThucThi) => R {
  const f = (cay: CayThucThi): R => {
    switch (cay.loai) {
      case "la":
        return onLa(cay.tenNode);
      case "nhanh": {
        const ketQuaDung: R = f(cay.neuDung);
        const ketQuaSai: R = f(cay.neuSai);
        return onNhanh(cay.dieuKien, ketQuaDung, ketQuaSai);
      }
    }
  };
  return f;
}

const cayDon: CayThucThi = {
  loai: "nhanh",
  dieuKien: true,
  neuDung: { loai: "la", tenNode: "a" },
  neuSai: { loai: "la", tenNode: "b" },
};

const demSoLa = foldCayThucThi<number>(
  () => 1,
  (_dieuKien, kQ, kS) => kQ + kS,
);
console.log(demSoLa(cayDon));
```

```text title=readonly
2
```

`foldCayThucThi` KHÔNG hề biết `R` LÀ kiểu gì — nó CHỈ lo việc GỌI ĐÚNG
`onLa` Ở lá, `onNhanh` Ở nhánh, VÀ luôn TÍNH CẢ HAI nhánh con
(`ketQuaDung` VÀ `ketQuaSai`) TRƯỚC KHI gọi `onNhanh` để gộp — giống hệt
`foldTree` CHƯƠNG `30c` luôn tính `leftResult` VÀ `rightResult` trước
`onBranch`. Đây LÀ ĐIỂM MẤU CHỐT: traversal (hàm `f` bên trong) VÀ
computation (`onLa`/`onNhanh`) HOÀN TOÀN tách biệt.
::::

::::example{#hai_cach_dien_giai_cung_mot_cay}
`taoCayHaiTang(giaTri, coTheThuLai)` dựng MỘT cây `2` tầng — TĨNH TẠI,
đúng `3` lá (`"xuLyHopLe"`, `"thuLai"`, `"boCuoc"`) — MÔ TẢ cấu trúc rẽ
nhánh của đồ thị `taoDoThiDayDu` Ở bài `3`. Hai cách GỌI `foldCayThucThi`
cho hai INTERPRETATION hoàn toàn khác nhau của CÙNG cây:

```typescript title=readonly
type CayThucThi =
  | { loai: "la"; tenNode: string }
  | { loai: "nhanh"; dieuKien: boolean; neuDung: CayThucThi; neuSai: CayThucThi };

function foldCayThucThi<R>(
  onLa: (tenNode: string) => R,
  onNhanh: (dieuKien: boolean, ketQuaDung: R, ketQuaSai: R) => R,
): (cay: CayThucThi) => R {
  const f = (cay: CayThucThi): R => {
    switch (cay.loai) {
      case "la":
        return onLa(cay.tenNode);
      case "nhanh": {
        const ketQuaDung: R = f(cay.neuDung);
        const ketQuaSai: R = f(cay.neuSai);
        return onNhanh(cay.dieuKien, ketQuaDung, ketQuaSai);
      }
    }
  };
  return f;
}

function taoCayHaiTang(giaTri: number, coTheThuLai: boolean): CayThucThi {
  return {
    loai: "nhanh",
    dieuKien: giaTri >= 0,
    neuDung: { loai: "la", tenNode: "xuLyHopLe" },
    neuSai: {
      loai: "nhanh",
      dieuKien: coTheThuLai,
      neuDung: { loai: "la", tenNode: "thuLai" },
      neuSai: { loai: "la", tenNode: "boCuoc" },
    },
  };
}

// Interpretation 1: DEM so la -- cong CA HAI nhanh, KHONG quan tam dieuKien.
const demSoLa = foldCayThucThi<number>(
  () => 1,
  (_dieuKien, kQ, kS) => kQ + kS,
);

// Interpretation 2: THUC THI that -- CHON dung mot nhanh theo dieuKien.
const thucThi = foldCayThucThi<string>(
  (tenNode) => tenNode,
  (dieuKien, kQ, kS) => (dieuKien ? kQ : kS),
);

const cayA = taoCayHaiTang(5, true);
const cayB = taoCayHaiTang(-3, true);
const cayC = taoCayHaiTang(-3, false);

console.log("dem la:", demSoLa(cayA), demSoLa(cayB), demSoLa(cayC));
console.log("thuc thi:", thucThi(cayA), thucThi(cayB), thucThi(cayC));
```

```text title=readonly
dem la: 3 3 3
thuc thi: xuLyHopLe thuLai boCuoc
```

`demSoLa` LUÔN trả về `3` — bất kể `giaTri` VÀ `coTheThuLai` LÀ gì, vì
`onNhanh` CỘNG cả hai nhánh (đếm CẤU TRÚC, không quan tâm ĐƯỜNG nào
runtime chọn). `thucThi` trả về BA kết quả KHÁC NHAU — vì `onNhanh` của
NÓ CHỌN đúng một nhánh dựa trên `dieuKien`. HAI interpretation này dùng
CHUNG một hàm duyệt cây (`foldCayThucThi`/`f`), KHÔNG viết lại đệ quy
LẦN nào — CHỈ khác Ở `onLa`/`onNhanh` truyền vào.
::::

::::predict{#doan-them-tang-thu-ba commitOnce}
Nếu THÊM một tầng rẽ nhánh THỨ BA vào nhánh `"boCuoc"` (đổi nó thành
`{ loai: "nhanh", dieuKien: ..., neuDung: {loai:"la",...}, neuSai:
{loai:"la",...} }` thay vì một lá đơn), thì `demSoLa` trên cây MỚI đó
sẽ đổi từ `3` thành BAO NHIÊU, VÀ `foldCayThucThi` có cần SỬA GÌ không?

:::opt{correct}
`demSoLa` tăng lên `4` (thêm MỘT tầng rẽ nhánh mới thay MỘT lá bằng HAI
lá, tăng tổng số lá thêm ĐÚNG `1`) — VÀ `foldCayThucThi` KHÔNG cần sửa
GÌ CẢ, vì nó ĐÃ xử lý được CÂY ĐỆ QUY Ở BẤT KỲ độ sâu nào (nhánh `"nhanh"`
gọi ĐỆ QUY vào CHÍNH `f`, không giới hạn số tầng)
:::
:::opt
`foldCayThucThi` phải sửa để thêm một tham số `onNhanhTangBa` xử lý
riêng tầng thứ ba
::why
Nhầm rằng MỖI tầng cần một handler RIÊNG — nhưng `foldCayThucThi` xử lý
MỌI tầng `"nhanh"` bằng ĐÚNG một `onNhanh`, gọi ĐỆ QUY vào CHÍNH nó cho
tầng SÂU hơn; số tầng của cây KHÔNG ảnh hưởng tới CHỮ KÝ (signature) của
hàm fold.

Chỗ lệch: nhánh `case "nhanh"` trong `f` gọi `f(cay.neuDung)` VÀ
`f(cay.neuSai)` — CHÍNH nó, đệ quy — nên MỘT cây SÂU `10` tầng dùng
được ĐÚNG cùng `onLa`/`onNhanh` như cây SÂU `2` tầng.
::
:::
:::opt
`demSoLa` KHÔNG đổi, vẫn LÀ `3` — thêm một tầng rẽ nhánh chỉ đổi CẤU
TRÚC BÊN TRONG một lá, không đổi TỔNG số lá
::why
Nhầm rằng "thay một lá bằng một nhánh mới" giữ NGUYÊN số lá — nhưng MỘT
lá đơn (`{loai:"la",...}`) LÀ `1` lá; thay NÓ bằng MỘT node `"nhanh"`
CHỨA hai lá con LÀ `2` lá — tổng số lá của TOÀN CÂY tăng thêm ĐÚNG `1`
(từ `3` lá cũ, MỘT lá bị thay bằng `2` lá mới, tổng thành `3 - 1 + 2 =
4`).

Chỗ lệch: `demSoLa` CỘNG số lá của CẢ HAI nhánh con Ở MỌI node `"nhanh"`
— thêm một node `"nhanh"` MỚI (thay một lá) LUÔN làm tổng lá tăng thêm
đúng `1`, không giữ nguyên.
::
:::
::::

::::code{#viet_fold_cay_thuc_thi}
Hoàn thiện `foldCayThucThi<R>` — trả về MỘT hàm đệ quy `f`: NẾU
`cay.loai` LÀ `"la"`, gọi `onLa(cay.tenNode)`; NẾU LÀ `"nhanh"`, tính
`ketQuaDung = f(cay.neuDung)` VÀ `ketQuaSai = f(cay.neuSai)` TRƯỚC, rồi
gọi `onNhanh(cay.dieuKien, ketQuaDung, ketQuaSai)`. Hoàn thiện `thucThi`
— gọi `foldCayThucThi<string>` VỚI `onLa` trả THẲNG `tenNode`, VÀ
`onNhanh` CHỌN `ketQuaDung` NẾU `dieuKien` LÀ `true`, NGƯỢC LẠI chọn
`ketQuaSai`.

```typescript title=starter
type CayThucThi =
  | { loai: "la"; tenNode: string }
  | { loai: "nhanh"; dieuKien: boolean; neuDung: CayThucThi; neuSai: CayThucThi };

function taoCayHaiTang(giaTri: number, coTheThuLai: boolean): CayThucThi {
  return {
    loai: "nhanh",
    dieuKien: giaTri >= 0,
    neuDung: { loai: "la", tenNode: "xuLyHopLe" },
    neuSai: {
      loai: "nhanh",
      dieuKien: coTheThuLai,
      neuDung: { loai: "la", tenNode: "thuLai" },
      neuSai: { loai: "la", tenNode: "boCuoc" },
    },
  };
}

function foldCayThucThi<R>(
  onLa: (tenNode: string) => R,
  onNhanh: (dieuKien: boolean, ketQuaDung: R, ketQuaSai: R) => R,
): (cay: CayThucThi) => R {
  ___
}

const demSoLa = foldCayThucThi<number>(
  () => 1,
  (_dieuKien, kQ, kS) => kQ + kS,
);

const thucThi = ___

const cayA = taoCayHaiTang(5, true);
const cayB = taoCayHaiTang(-3, true);
const cayC = taoCayHaiTang(-3, false);

console.log(
  demSoLa(cayA),
  demSoLa(cayB),
  demSoLa(cayC),
  thucThi(cayA),
  thucThi(cayB),
  thucThi(cayC),
);
```

```typescript title=solution
type CayThucThi =
  | { loai: "la"; tenNode: string }
  | { loai: "nhanh"; dieuKien: boolean; neuDung: CayThucThi; neuSai: CayThucThi };

function taoCayHaiTang(giaTri: number, coTheThuLai: boolean): CayThucThi {
  return {
    loai: "nhanh",
    dieuKien: giaTri >= 0,
    neuDung: { loai: "la", tenNode: "xuLyHopLe" },
    neuSai: {
      loai: "nhanh",
      dieuKien: coTheThuLai,
      neuDung: { loai: "la", tenNode: "thuLai" },
      neuSai: { loai: "la", tenNode: "boCuoc" },
    },
  };
}

function foldCayThucThi<R>(
  onLa: (tenNode: string) => R,
  onNhanh: (dieuKien: boolean, ketQuaDung: R, ketQuaSai: R) => R,
): (cay: CayThucThi) => R {
  const f = (cay: CayThucThi): R => {
    switch (cay.loai) {
      case "la":
        return onLa(cay.tenNode);
      case "nhanh": {
        const ketQuaDung: R = f(cay.neuDung);
        const ketQuaSai: R = f(cay.neuSai);
        return onNhanh(cay.dieuKien, ketQuaDung, ketQuaSai);
      }
    }
  };
  return f;
}

const demSoLa = foldCayThucThi<number>(
  () => 1,
  (_dieuKien, kQ, kS) => kQ + kS,
);

const thucThi = foldCayThucThi<string>(
  (tenNode) => tenNode,
  (dieuKien, kQ, kS) => (dieuKien ? kQ : kS),
);

const cayA = taoCayHaiTang(5, true);
const cayB = taoCayHaiTang(-3, true);
const cayC = taoCayHaiTang(-3, false);

console.log(
  demSoLa(cayA),
  demSoLa(cayB),
  demSoLa(cayC),
  thucThi(cayA),
  thucThi(cayB),
  thucThi(cayC),
);
```

```typescript title=test
if (demSoLa(cayA) !== 3 || demSoLa(cayB) !== 3 || demSoLa(cayC) !== 3) {
  throw new Error("demSoLa phai luon la 3 (cay tinh co dung 3 la), BAT KE dieuKien runtime");
}
if (thucThi(cayA) !== "xuLyHopLe") throw new Error("thucThi(cayA) phai la xuLyHopLe (giaTri=5 >= 0)");
if (thucThi(cayB) !== "thuLai") throw new Error("thucThi(cayB) phai la thuLai (giaTri=-3, coTheThuLai=true)");
if (thucThi(cayC) !== "boCuoc") throw new Error("thucThi(cayC) phai la boCuoc (giaTri=-3, coTheThuLai=false)");

const cayDon: CayThucThi = { loai: "la", tenNode: "chiMotLa" };
if (demSoLa(cayDon) !== 1) throw new Error("cay chi co 1 la thi demSoLa phai la 1");
if (thucThi(cayDon) !== "chiMotLa") throw new Error("thucThi tren cay 1 la phai tra ve dung ten node do");

const cayMotTang: CayThucThi = {
  loai: "nhanh",
  dieuKien: false,
  neuDung: { loai: "la", tenNode: "a" },
  neuSai: { loai: "la", tenNode: "b" },
};
if (thucThi(cayMotTang) !== "b") throw new Error("dieuKien false phai chon nhanh neuSai");
if (demSoLa(cayMotTang) !== 2) throw new Error("cay mot tang co dung 2 la");

const demDoSau = foldCayThucThi<number>(
  () => 0,
  (_dieuKien, kQ, kS) => 1 + Math.max(kQ, kS),
);
if (demDoSau(cayDon) !== 0) throw new Error("foldCayThucThi phai tong quat: interpretation KHAC (do sau) tren cay 1 la phai la 0");
if (demDoSau(cayA) !== 2) throw new Error("foldCayThucThi phai tong quat: do sau cua cayA (hai tang nhanh) phai la 2");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (foldCayThucThi): tra ve mot ham f dinh nghia BEN TRONG (dung switch tren cay.loai) -- 'la' thi goi onLa(cay.tenNode); 'nhanh' thi TINH ketQuaDung = f(cay.neuDung) va ketQuaSai = f(cay.neuSai) TRUOC, roi goi onNhanh(cay.dieuKien, ketQuaDung, ketQuaSai). Cho hai (thucThi): goi foldCayThucThi<string> voi onLa tra thang tenNode, onNhanh la mot ham (dieuKien, kQ, kS) => dieuKien ? kQ : kS."
- kind: strategy
  body: "Cho dau: const f = (cay: CayThucThi): R => { switch (cay.loai) { case \"la\": return onLa(cay.tenNode); case \"nhanh\": { const ketQuaDung: R = f(cay.neuDung); const ketQuaSai: R = f(cay.neuSai); return onNhanh(cay.dieuKien, ketQuaDung, ketQuaSai); } } }; return f; Cho hai: foldCayThucThi<string>((tenNode) => tenNode, (dieuKien, kQ, kS) => (dieuKien ? kQ : kS));"
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
  expect: "3 3 3 xuLyHopLe thuLai boCuoc"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một hàm `foldCayThucThi` duy nhất, hai `onNhanh` khác nhau — một fold
ĐẾM cấu trúc (luôn `3`), một fold THỰC THI thật (phụ thuộc input, khớp
CHÍNH XÁC với node cuối mà `chayDoThi` bài `1`-`3` từng đi tới). BOSS
khép quest ráp NGUYÊN VĂN bốn cơ chế đã học — node+edge, rẽ nhánh, edge
coverage, checkpoint/resume — trên MỘT đồ thị, đo tiết kiệm THẬT từ
resume.
::::

::::reflect{#nghi-lai}
`CayThucThi` VÀ `foldCayThucThi` không thêm một KHẢ NĂNG mới cho GRAPH —
`chayDoThi` (bài `1`) ĐÃ chạy được đúng những đường đi này bằng vòng
`while`. Giá trị nằm Ở việc TÁCH RIÊNG hai mối quan tâm: `foldCayThucThi`
LÀ traversal (luôn giống nhau, đệ quy vào CẢ HAI nhánh Ở MỌI node
`"nhanh"`), còn `onLa`/`onNhanh` LÀ nghiệp vụ (đếm lá, hay thực thi
thật, hay — như `demDoSau` Ở phần test — đo ĐỘ SÂU của cây). MASTERPLAN
gọi execution graph LÀ "Free monad + interpreter, catamorphism trên cây
node" — `CayThucThi` chính LÀ cái cây đó (biểu diễn tối giản của cấu
trúc rẽ nhánh), VÀ `foldCayThucThi` chính LÀ "interpreter": diễn giải
CÙNG một cấu trúc dữ liệu theo BAO NHIÊU cách tuỳ ý, chỉ bằng cách đổi
HAI handler, không viết lại logic duyệt cây LẦN nào.
::::

::::checkpoint{mastery=0.9}
::::
