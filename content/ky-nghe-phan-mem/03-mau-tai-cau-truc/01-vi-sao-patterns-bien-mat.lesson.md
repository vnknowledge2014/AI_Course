---
id: ky-nghe-phan-mem.mau-tai-cau-truc.vi-sao-patterns-bien-mat
title: "Vì sao GoF Patterns \"biến mất\" trong FP"
summary: "Design Patterns (GoF, 1994) giải quyết vấn đề thật — nhưng bằng infrastructure phức tạp vì C++/Smalltalk THIẾU first-class function, closure, sum type. TypeScript có đủ những thứ này — bảng ánh xạ: thiếu gì sinh pattern nào, và FP thay thế bằng khái niệm có sẵn nào."
locale: vi
track: ky-nghe-phan-mem
module: mau-tai-cau-truc
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [mau.why-patterns-vanish]
requires: [ts.adt-gate-boss]
concepts: [mau.why-patterns-vanish]
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
Track mới. Sách giáo khoa OOP dạy hàng chục "mẫu thiết kế" (design
patterns) — TypeScript có closure, hàm bậc cao, discriminated union.
Chuyện gì xảy ra với những mẫu ĐÓ?
::::

::::explain{#bang-anh-xa}
**Design Patterns** (Gang of Four, 1994) viết cho **C++**/**Smalltalk**
— hai ngôn ngữ THỜI ĐÓ **THIẾU** những thứ TypeScript có SẴN hôm nay.
Nhiều pattern KHÔNG PHẢI "sai" — chúng LÀ **GIÀN GIÁO**: cách LÁCH một
tính năng ngôn ngữ CÒN THIẾU, bằng cách xây THÊM interface/class/kế
thừa. Khi ngôn ngữ ĐÃ CÓ tính năng đó SẴN, giàn giáo TRỞ NÊN thừa:

| Ngôn ngữ CŨ thiếu | Pattern GoF sinh ra | TypeScript có SẴN |
|---|---|---|
| Hàm là "công dân hạng hai" | Strategy, Command | Hàm LÀ giá trị — gán, truyền, trả về |
| Kiểu tổng (sum type) | Visitor, State | Discriminated Union + `switch` |
| Closure | Factory | Hàm nội bộ BẮT GIỮ (capture) biến ngoài |
| Composition hàm bậc nhất | Decorator, Middleware | `pipe`/hàm gọi hàm |

Lập trình viên Peter Norvig từng nhận xét đa số mẫu GoF "biến mất hoặc
đơn giản hóa" trong ngôn ngữ có sẵn hàm bậc cao. Track này ĐI THEO
TỪNG dòng bảng trên — MỖI bài LÀ một pattern, ĐÀO SÂU cách nó ĐƠN GIẢN
HOÁ.
::::

::::example{#strategy-la-gia-tri}
Minh hoạ TRỰC TIẾP dòng ĐẦU bảng: một "chiến lược" (strategy) sắp xếp
CHỈ LÀ một **GIÁ TRỊ HÀM**, truyền thẳng làm tham số — KHÔNG cần
`interface`, KHÔNG cần `class`:

```typescript title=readonly
type SortFn = (data: readonly number[]) => readonly number[];

const tangDan: SortFn = (data) => [...data].sort((a, b) => a - b);
const giamDan: SortFn = (data) => [...data].sort((a, b) => b - a);

function apDungSapXep(data: readonly number[], sort: SortFn): readonly number[] {
  return sort(data);
}

console.log(apDungSapXep([3, 1, 2], tangDan));
console.log(apDungSapXep([3, 1, 2], giamDan));
```

```text title=readonly
[1,2,3]
[3,2,1]
```

`apDungSapXep` KHÔNG BIẾT (và KHÔNG CẦN BIẾT) `tangDan`/`giamDan`
"LÀ" cái gì — nó CHỈ nhận MỘT hàm khớp chữ ký `SortFn` VÀ GỌI nó.
Trong OOP truyền thống, ĐỔI "chiến lược" cần một `interface
SortStrategy { sort(...) }` VÀ MỘT class RIÊNG cho MỖI thuật toán —
Ở ĐÂY, "đổi chiến lược" ĐƠN GIẢN LÀ truyền MỘT giá trị hàm KHÁC. Bài
2 đào sâu ĐÚNG kỹ thuật NÀY (Strategy pattern).
::::

::::predict{#doan-gan-lai-bien-ham commitOnce}
```typescript
type CongThucGiam = (gia: number) => number;

let congThucHienTai: CongThucGiam = (gia) => gia - 20;

function apDung(gia: number): number {
  return congThucHienTai(gia);
}

console.log(apDung(100));
congThucHienTai = (gia) => gia * 0.9;
console.log(apDung(100));
```

Hai dòng `console.log` in ra gì?

:::opt{correct}
`80` rồi `90`
:::

:::opt
`80` rồi `80` — vì `apDung` đã "chụp" (capture) `congThucHienTai` NGAY
tại thời điểm ĐỊNH NGHĨA, gán LẠI biến `congThucHienTai` SAU ĐÓ không
ảnh hưởng gì tới hàm `apDung` đã định nghĩa TRƯỚC
::why
Gần đúng ở việc bạn nhớ ĐÚNG rằng CLOSURE có liên quan tới việc "bắt
giữ" một thứ gì đó từ bên ngoài — quan sát ĐÓ đúng hướng.

Chỗ lệch: closure bắt giữ **CHÍNH CÁI BIẾN** (binding), KHÔNG PHẢI
**GIÁ TRỊ HIỆN TẠI của biến đó tại thời điểm định nghĩa**. `apDung`
đọc `congThucHienTai` **MỖI LẦN NÓ ĐƯỢC GỌI**, không PHẢI một lần DUY
NHẤT lúc định nghĩa — khi `congThucHienTai` được GÁN LẠI (`let` cho
phép gán lại), LẦN GỌI TIẾP THEO của `apDung` đọc GIÁ TRỊ MỚI. Đây
CHÍNH LÀ lý do "đổi chiến lược lúc chạy" (runtime strategy swap) hoạt
động Ở CẢ OOP LẪN FP — KHÁC BIỆT chỉ LÀ FP làm điều đó với MỘT biến
hàm, OOP làm bằng MỘT field object.
::
:::

:::opt
Máy báo lỗi biên dịch — biến `congThucHienTai` được khai VỚI kiểu
`CongThucGiam` cụ thể, TypeScript KHÔNG cho GÁN LẠI một biến `let` đã
có kiểu hàm bằng MỘT giá trị hàm KHÁC, dù CÙNG chữ ký
::why
Gần đúng ở việc bạn để ý `congThucHienTai: CongThucGiam` có khai kiểu
RÕ RÀNG — một quan sát ĐÚNG về CÚ PHÁP.

Chỗ lệch: khai kiểu MỘT biến (`: CongThucGiam`) chỉ RÀNG BUỘC "giá trị
gán vào biến này PHẢI khớp chữ ký ĐÓ" — KHÔNG cấm gán LẠI, MIỄN LÀ giá
trị MỚI CŨNG khớp CHỮ KÝ (đây LÀ trường hợp: `(gia) => gia * 0.9` NHẬN
MỘT `number`, TRẢ MỘT `number`, khớp `CongThucGiam`). `let` (khác
`const`) TỒN TẠI CHÍNH LÀ để cho phép gán lại — biên dịch SẠCH.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bảng ánh xạ: thiếu hàm bậc cao → Strategy/Command; thiếu sum type →
Visitor/State; thiếu closure → Factory. Bài tiếp theo: đào sâu dòng
đầu — Strategy pattern, đầy đủ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`apDungSapXep` NHẬN một hàm làm tham số — trong OOP, việc TƯƠNG ĐƯƠNG
cần MỘT `interface` với MỘT method. Bước KẾ TIẾP để hiểu Strategy
pattern ĐẦY ĐỦ LÀ gì?
::::

::::checkpoint{mastery=0.8}
::::
