---
id: lap-trinh-ham.adt-pattern-matching.default-khong-co-assertnever-che-giau-loi
title: "`default` KHÔNG có `assertNever` — che giấu lỗi, không bắt nó"
summary: "`default: return 0` (không gọi `chuaXuLy`) vẫn biên dịch được khi thêm biến thể mới — nhưng hàm giờ ÂM THẦM trả `0` cho biến thể chưa xử lý, y hệt lỗi bài 7, chỉ khác chỗ có `default` khiến người đọc TƯỞNG NHẦM là đã xử lý đủ. `assertNever` là một KỶ LUẬT tự áp, chỉ có tác dụng nếu gọi ĐÚNG vào nhánh đó."
locale: vi
track: lap-trinh-ham
module: adt-pattern-matching
order: 11
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [ts.default-without-assertnever-hides-bugs]
requires: [ts.predict-exhaustiveness]
concepts: [ts.default-without-assertnever-hides-bugs]
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
Bài trước bạn đoán đúng: thêm biến thể mới vào một union CÓ `assertNever`
bảo vệ — trình biên dịch báo lỗi NGAY. Nhưng nếu `default` không hề gọi
`chuaXuLy`, chỉ lặng lẽ `return 0`, thì sao?
::::

::::explain{#default-khong-goi-chuaxuly}
```typescript
interface Vuong {
  kind: "vuong";
  canh: number;
}

interface Tron {
  kind: "tron";
  banKinh: number;
}

interface TamGiacDeu {
  kind: "tamGiac";
  canhTamGiac: number;
}

type Hinh = Vuong | Tron | TamGiacDeu;

function dienTich(h: Hinh): number {
  switch (h.kind) {
    case "vuong":
      return h.canh * h.canh;
    case "tron":
      return 3.14 * h.banKinh * h.banKinh;
    default:
      return 0;
  }
}

console.log(dienTich({ kind: "vuong", canh: 4 }));
console.log(dienTich({ kind: "tamGiac", canhTamGiac: 5 }));
```

```text
16
0
```

`Hinh` có BA biến thể — `vuong`, `tron`, `tamGiac` — nhưng `switch` chỉ
viết `case` cho `vuong` và `tron`. Nhánh `default` KHÔNG gọi `chuaXuLy`
(mẫu bài 8 đã dạy) — nó chỉ viết `return 0;`. Mã này BIÊN DỊCH SẠCH,
không một cảnh báo nào.

Gọi `dienTich` với một `Vuong` cho ra `16` — đúng, `case "vuong"` tính
thật. Gọi với một `TamGiacDeu` cho ra `0` — SAI (một tam giác cạnh 5
diện tích thật không thể là `0`), nhưng không có lỗi biên dịch, không
có lỗi lúc chạy — chỉ một con số sai lặng lẽ trôi ra `console.log`.

Đây CHÍNH LÀ lỗi bài 7 đã nêu (một nhánh bị bỏ sót, hàm trả sai mà
không báo động gì) — chỉ khác MỘT điểm: có `default`. Điểm khác đó
nguy hiểm HƠN, không an toàn hơn. Nhìn vào `switch` này, một người đọc
thấy có nhánh `default` liền yên tâm — "có nhánh dự phòng rồi, chắc đã
xử lý đủ". Sự yên tâm đó SAI. `default` chỉ đảm bảo hàm LUÔN trả về
MỘT giá trị nào đó (không rơi qua `undefined` như bài 7) — nó không hề
đảm bảo giá trị đó ĐÚNG. Nếu nhánh này gọi `chuaXuLy(h)` thay vì
`return 0` (đã đo thật), TypeScript sẽ báo TS2345 ngay dòng đó — nhưng
nó không gọi, nên không có gì bắt lỗi cả.
::::

::::example{#hai-bien-the-cung-lot}
Thêm biến thể THỨ TƯ (`NguGiacDeu`) — `switch` VẪN không sửa gì, cả hai
biến thể mới đều rơi vào cùng một `default`:

```typescript title=readonly
interface Vuong {
  kind: "vuong";
  canh: number;
}

interface Tron {
  kind: "tron";
  banKinh: number;
}

interface TamGiacDeu {
  kind: "tamGiac";
  canhTamGiac: number;
}

interface NguGiacDeu {
  kind: "nguGiac";
  canhNguGiac: number;
}

type Hinh = Vuong | Tron | TamGiacDeu | NguGiacDeu;

function dienTich(h: Hinh): number {
  switch (h.kind) {
    case "vuong":
      return h.canh * h.canh;
    case "tron":
      return 3.14 * h.banKinh * h.banKinh;
    default:
      return 0;
  }
}

console.log(dienTich({ kind: "tamGiac", canhTamGiac: 5 }));
console.log(dienTich({ kind: "nguGiac", canhNguGiac: 5 }));
```

```text title=readonly
0
0
```

Một tam giác VÀ một ngũ giác — hai hình dạng hoàn toàn khác nhau — đều
cho ra CÙNG một con số sai: `0`. Không gì phân biệt được hai lỗi này,
không gì báo hiệu có TỚI HAI biến thể chưa được xử lý thật. So với
`chuaXuLy` (bài 8, 9): mỗi biến thể MỚI thêm vào mà chưa có `case` sẽ
làm trình biên dịch báo lỗi RIÊNG, NGAY tại dòng `chuaXuLy(h)`, ngay
lúc biên dịch — trước khi chương trình chạy dù chỉ một lần. `default:
return 0` không có khả năng đó, bất kể bạn thêm bao nhiêu biến thể.
`assertNever` không phải cú pháp `switch` bắt buộc phải có — nó là một
KỶ LUẬT bạn TỰ áp đặt lên chính mình, và kỷ luật đó chỉ có tác dụng nếu
`default` thật sự GỌI nó.
::::

::::predict{#doan-default-tra-gi commitOnce}
```typescript
interface DonChoXuLy {
  kind: "choXuLy";
  ma: string;
}

interface DonDaGiao {
  kind: "daGiao";
  ma: string;
}

interface DonDaHuy {
  kind: "daHuy";
  ma: string;
  lyDoHuy: string;
}

type Don = DonChoXuLy | DonDaGiao | DonDaHuy;

function phiXuLy(d: Don): number {
  switch (d.kind) {
    case "choXuLy":
      return 10;
    case "daGiao":
      return 5;
    default:
      return 0;
  }
}

console.log(phiXuLy({ kind: "daHuy", ma: "DH-9", lyDoHuy: "khach huy" }));
```

Dòng cuối in ra gì?

:::opt{correct}
`0`
:::

:::opt
Máy báo lỗi biên dịch — thiếu `case "daHuy"` trong `switch`
::why
Gần đúng ở việc bạn để ý `switch` này THẬT SỰ thiếu một `case` riêng
cho `"daHuy"` — quan sát về cấu trúc đó đúng.

Chỗ lệch: nhánh `default` ở đây chỉ viết `return 0;`, không gọi
`chuaXuLy` — không có gì thu hẹp kiểu `d` về `never` để buộc trình biên
dịch kiểm tính đủ nhánh. `switch` không TỰ ĐỘNG đòi xử lý hết mọi biến
thể (bài 7 đã nêu) — thiếu `case` chỉ gây lỗi biên dịch NẾU có cơ chế
như `assertNever` bắt nó, mà mã này không có. Mã này biên dịch sạch,
không cảnh báo gì.
::
:::

:::opt
Chương trình NÉM lỗi lúc chạy (`throw`) vì `"daHuy"` chưa được xử lý
::why
Gần đúng ở việc bạn nhận ra `"daHuy"` THẬT SỰ chưa được một `case`
riêng nào xử lý — quan sát đó đúng.

Chỗ lệch: `default: return 0;` là một câu lệnh `return` BÌNH THƯỜNG,
không phải `throw`. Nó không ném lỗi gì — nó trả về `0` rồi hàm kết
thúc êm re, y hệt mọi nhánh khác. Muốn có `throw` ở đây, `default`
phải GỌI một hàm như `chuaXuLy` (bài 8) — mã này không gọi hàm nào cả.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Có `default` không có nghĩa là đã xử lý đủ. `assertNever` không phải
cú pháp `switch` bắt buộc — nó là một kỷ luật bạn tự áp, và chỉ có tác
dụng nếu `default` gọi ĐÚNG vào nó.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn đã thấy `default` một mình — dù có mặt hay không, dù `return 0`
hay im lặng — đều không tự bảo vệ được gì. Cái thật sự bảo vệ là việc
GỌI `chuaXuLy` đúng chỗ.

Nếu phải tự tay viết một hàm `switch` xử lý ĐỦ mọi biến thể, gọi đúng
`chuaXuLy` ở `default` — không chỉ đọc hiểu ví dụ có sẵn nữa, mà tự
dựng từ đầu — bạn sẽ viết thế nào?
::::

::::checkpoint{mastery=0.8}
::::
