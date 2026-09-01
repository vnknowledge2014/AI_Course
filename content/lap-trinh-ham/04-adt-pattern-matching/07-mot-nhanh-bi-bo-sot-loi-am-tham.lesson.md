---
id: lap-trinh-ham.adt-pattern-matching.mot-nhanh-bi-bo-sot-loi-am-tham
title: "Quên một nhánh trong `switch` — lỗi ÂM THẦM lúc chạy"
summary: "Một switch trên Hinh (ba biến thể) chỉ xử lý vuong/tron, quên tamGiac, không có default — TypeScript biên dịch được (không lỗi gì!), nhưng gọi hàm với một tamGiac thật thì hàm trả về undefined lúc chạy, không báo lỗi rõ ràng nào."
locale: vi
track: lap-trinh-ham
module: adt-pattern-matching
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [ts.missing-case-silent-bug]
requires: [ts.review-discriminated-union]
concepts: [ts.missing-case-silent-bug]
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
Bài trước, bạn tự tay viết một `switch` xử lý ĐỦ cả ba biến thể — không
sai chỗ nào. Nhưng nếu, một ngày nào đó, bạn (hay một đồng nghiệp)
QUÊN mất một nhánh thì sao? Có gì lên tiếng báo cho biết không?
::::

::::explain{#switch-bo-sot-nhanh}
```typescript
interface Vuong {
  kind: "vuong";
  canh: number;
}

interface Tron {
  kind: "tron";
  banKinh: number;
}

interface TamGiac {
  kind: "tamGiac";
  canhTamGiac: number;
}

type Hinh = Vuong | Tron | TamGiac;

function moTa(h: Hinh) {
  switch (h.kind) {
    case "vuong":
      return "hình vuông cạnh " + h.canh;
    case "tron":
      return "hình tròn bán kính " + h.banKinh;
  }
}

const v: Hinh = { kind: "vuong", canh: 4 };
const tg: Hinh = { kind: "tamGiac", canhTamGiac: 6 };

console.log(moTa(v));
console.log("kết quả cho tam giác: " + moTa(tg));
```

```text
hình vuông cạnh 4
kết quả cho tam giác: undefined
```

`Hinh` có BA biến thể (`vuong`/`tron`/`tamGiac`) — nhưng `switch` bên
trong `moTa` chỉ có `case "vuong"` và `case "tron"`. KHÔNG có
`case "tamGiac"`. KHÔNG có `default`. Đọc lại đoạn mã: không dòng nào
trông như lỗi.

Và đúng là KHÔNG có lỗi — TypeScript BIÊN DỊCH đoạn mã này SẠCH SẼ,
không một cảnh báo. Bạn đã quen `switch` trên discriminant tự NARROW
kiểu (bài 3, bài 4) — nhưng NARROW không giống ĐÒI ĐỦ. TypeScript chỉ
thu hẹp kiểu ở những nhánh BẠN ĐÃ VIẾT; nó không hề kiểm xem bạn có
viết ĐỦ nhánh cho MỌI biến thể của `Hinh` hay không.

Hậu quả lộ ra lúc CHẠY, không phải lúc biên dịch: gọi `moTa(v)` (một
`Vuong` thật) in đúng "hình vuông cạnh 4". Nhưng gọi `moTa(tg)` (một
`TamGiac` thật) — `switch` không khớp nhánh nào, thân hàm chạy tới
cuối mà không gặp `return` nào, nên hàm ÂM THẦM trả về `undefined`.
Không có `throw`, không có exception, không có dòng nào đỏ trong
console — chỉ một giá trị SAI lặng lẽ trôi tiếp vào phần còn lại của
chương trình.
::::

::::example{#thieu-nhanh-tinh-phi}
Cùng lỗi đó, một ngữ cảnh khác: tính phí vận chuyển cho MỘT trong ba
trạng thái đơn hàng, nhưng hàm quên mất trạng thái `"daHuy"`:

```typescript title=readonly
interface DonChoXuLy {
  kind: "choXuLy";
  phiVanChuyen: number;
}

interface DonDaGiao {
  kind: "daGiao";
  phiVanChuyen: number;
}

interface DonDaHuy {
  kind: "daHuy";
  phiVanChuyen: number;
}

type Don = DonChoXuLy | DonDaGiao | DonDaHuy;

function layPhi(d: Don) {
  switch (d.kind) {
    case "choXuLy":
      return d.phiVanChuyen;
    case "daGiao":
      return d.phiVanChuyen;
  }
}

const d1: Don = { kind: "choXuLy", phiVanChuyen: 20 };
const d2: Don = { kind: "daHuy", phiVanChuyen: 15 };

console.log(layPhi(d1));
console.log(layPhi(d2) === undefined);
```

```text title=readonly
20
true
```

`layPhi(d1)` (một đơn `choXuLy`) trả đúng `20`. `layPhi(d2)` (một đơn
`daHuy`) — `switch` không khớp nhánh nào, hàm trả `undefined`, dòng
kiểm `=== undefined` xác nhận đúng là vậy. TypeScript biên dịch đoạn
mã này KHÔNG một lỗi nào, dù `DonDaHuy` là một biến thể HOÀN TOÀN hợp
lệ của `Don`. Ba biến thể trong khai báo kiểu, nhưng chỉ hai nhánh
trong `switch` — khoảng cách đó không hiện ra ở đâu cả cho tới khi một
đơn `daHuy` THẬT đi qua hàm.
::::

::::predict{#doan-ket-qua-thieu-nhanh commitOnce}
```typescript
interface HangDong {
  kind: "dong";
  diem: number;
}

interface HangBac {
  kind: "bac";
  diem: number;
}

interface HangVang {
  kind: "vang";
  diem: number;
}

type Hang = HangDong | HangBac | HangVang;

function moTaHang(h: Hang) {
  switch (h.kind) {
    case "dong":
      return "hạng đồng";
    case "bac":
      return "hạng bạc";
  }
}

const hv: Hang = { kind: "vang", diem: 500 };
console.log("Kết quả: " + moTaHang(hv));
```

Dòng cuối in ra gì?

:::opt{correct}
`Kết quả: undefined`
:::

:::opt
Máy báo lỗi biên dịch ngay tại `switch (h.kind)` — vì thiếu nhánh xử
lý `"vang"`
::why
Gần đúng ở việc bạn để ý ĐÚNG là `switch` này thật sự thiếu một nhánh
cho `"vang"` — quan sát về cấu trúc code đó chính xác.

Chỗ lệch: `switch` thường (không có cơ chế `assertNever` ở nhánh
`default` — bài sau mới học cơ chế đó) KHÔNG tự đòi xử lý đủ mọi biến
thể. TypeScript coi việc bỏ sót một `case` là hợp lệ về mặt cú pháp,
không kiểm tra tính đầy đủ đó lúc biên dịch. Mã biên dịch sạch, không
một cảnh báo nào.
::
:::

:::opt
Chương trình bị crash lúc chạy — ném lỗi kiểu "không nhánh nào khớp
kind vang"
::why
Gần đúng ở việc bạn nhận ra `hv` KHÔNG khớp `case` nào trong `switch`
— quan sát đó đúng.

Chỗ lệch: `switch` trong JavaScript/TypeScript không tự ném lỗi khi
không nhánh nào khớp — nó chỉ ÂM THẦM bỏ qua toàn bộ khối `switch`,
hàm chạy tới cuối thân hàm mà không gặp `return` nào, nên trả về
`undefined` chứ không `throw` gì. Ghép với `"Kết quả: " + undefined`
(nối chuỗi với giá trị không phải chuỗi tự động chuyển thành chữ
"undefined"), dòng in ra là `Kết quả: undefined` chứ không phải một
thông báo lỗi.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`switch` bỏ sót một nhánh vẫn biên dịch, vẫn chạy, và âm thầm trả về
một giá trị SAI mà không kêu ca gì. Vấn đề đã rõ — giờ cần một cách để
TRÌNH BIÊN DỊCH tự phát hiện chỗ thiếu, thay vì đợi ai đó chạy đúng
trường hợp bị bỏ sót rồi mới biết.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa thấy: không có gì buộc `switch` phải xử lý ĐỦ mọi biến thể của
một union — thiếu một nhánh vẫn biên dịch sạch, lỗi chỉ lộ ra lúc CHẠY,
và chỉ với ĐÚNG trường hợp bị bỏ sót.

Nếu có một cách viết khiến TypeScript TỰ CHẶN ngay lúc biên dịch, ngay
cả khi bạn chưa từng chạy thử trường hợp bị thiếu — thì sao?
::::

::::checkpoint{mastery=0.8}
::::
