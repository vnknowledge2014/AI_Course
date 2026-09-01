---
id: lap-trinh-ham.adt-pattern-matching.tu-refactor-mot-tap-co-thanh-adt
title: "Tự refactor một tập cờ rời rạc thành ADT"
summary: "Cho sẵn interface DonHangCu dùng cờ rời rạc (daThanhToan, daHuy, maGiam) — tự viết một discriminated union THAY THẾ nó (không dùng cờ nữa) và một hàm xử lý đủ mọi biến thể; không khái niệm mới, đo khả năng tự tay làm lại quy trình refactor bài trước vừa trình diễn, trên một bài toán MỚI."
locale: vi
track: lap-trinh-ham
module: adt-pattern-matching
order: 23
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [ts.write-refactored-adt]
requires: [ts.refactor-real-loading-state]
concepts: [ts.write-refactored-adt]
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
Bài trước bạn thấy một SO SÁNH TRỰC TIẾP — cùng bài toán `TrangThai`,
viết hai cách (cờ cũ, union mới), đọc cách nào ngắn hơn, cách nào được
TypeScript tự kiểm tính đủ nhánh. Hôm nay không có bản mẫu để đọc nữa —
một bài toán MỚI (đơn hàng), tự bạn làm lại đúng quy trình đó.
::::

::::explain{#don-hang-co-roi-rac}
```typescript
interface DonHangCu {
  daThanhToan: boolean;
  daHuy: boolean;
  maGiam: string | null;
}

const donLoi: DonHangCu = { daThanhToan: true, daHuy: true, maGiam: null };
console.log(donLoi.daThanhToan);
console.log(donLoi.daHuy);
console.log(donLoi.maGiam);
```

```text
true
true
null
```

`DonHangCu` mô tả một đơn hàng bằng BA cờ rời rạc: `daThanhToan` (đã
thanh toán chưa), `daHuy` (đã huỷ chưa), `maGiam` (mã giảm giá đã áp
dụng, hoặc `null` nếu không có). Bài 20 đã đo đúng vấn đề này trên
`TrangThai` — giờ lặp lại Y HỆT trên đơn hàng: `donLoi` khai
`daThanhToan: true` VÀ `daHuy: true` CÙNG LÚC. TypeScript biên dịch
trôi chảy (mỗi trường đúng kiểu `boolean`/`string | null` riêng của
nó, hoàn toàn ĐỘC LẬP với nhau), chương trình chạy bình thường, in ra
`true`/`true`/`null`.

Nhưng về nghiệp vụ, "đơn hàng VỪA đã thanh toán VỪA đã huỷ" mơ hồ tới
mức vô nghĩa — không rõ nó đang ở trạng thái NÀO trong ba trạng thái
thật: chờ thanh toán, đã thanh toán, đã huỷ. Không gì trong `DonHangCu`
buộc CHỈ MỘT trong ba khả năng đó đúng tại một thời điểm — đúng lỗ
hổng bài 20 đã chỉ ra, giờ hiện lại trên một bài toán khác.
::::

::::example{#don-hang-thanh-adt}
Thay `DonHangCu` bằng MỘT discriminated union ba biến thể — mỗi biến
thể chỉ mang field nó thật sự cần:

```typescript title=readonly
type DonHang =
  | { kind: "choThanhToan" }
  | { kind: "daThanhToan"; maGiam: string | null }
  | { kind: "daHuy" };

function moTaDonHang(d: DonHang): string {
  switch (d.kind) {
    case "choThanhToan":
      return "chờ thanh toán";
    case "daThanhToan":
      if (d.maGiam === null) {
        return "đã thanh toán, không mã giảm giá";
      }
      return "đã thanh toán, mã giảm giá " + d.maGiam;
    case "daHuy":
      return "đã huỷ";
  }
}

console.log(moTaDonHang({ kind: "choThanhToan" }));
console.log(moTaDonHang({ kind: "daThanhToan", maGiam: "SALE10" }));
console.log(moTaDonHang({ kind: "daThanhToan", maGiam: null }));
console.log(moTaDonHang({ kind: "daHuy" }));
```

```text title=readonly
chờ thanh toán
đã thanh toán, mã giảm giá SALE10
đã thanh toán, không mã giảm giá
đã huỷ
```

`choThanhToan` không field nào khác — chỉ cần biết đang chờ. `daHuy`
cũng vậy — không field nào khác. Chỉ `daThanhToan` mang thêm `maGiam`
(`string` hoặc `null`), vì mã giảm giá CHỈ có ý nghĩa một khi đơn đã
thanh toán. Tổ hợp "vừa đã thanh toán vừa đã huỷ" từ ví dụ trước giờ
KHÔNG CÒN VIẾT RA ĐƯỢC nữa — thử gán
`{ kind: "daThanhToan", daHuy: true, maGiam: null }` cho `DonHang`,
TypeScript CHẶN NGAY bằng excess property check (mã TS2353, đã học ở
bài 15): field `daHuy` không thuộc biến thể `daThanhToan`. Lỗi bị đẩy
từ "phải NHỚ kiểm tra lúc chạy" sang "không có KHÔNG GIAN kiểu nào để
viết ra nó" — đúng Ý bài 21 đã đo.
::::

::::predict{#doan-mo-ta-don-hang commitOnce}
```typescript
type DonHang =
  | { kind: "choThanhToan" }
  | { kind: "daThanhToan"; maGiam: string | null }
  | { kind: "daHuy" };

function moTaDonHang(d: DonHang): string {
  switch (d.kind) {
    case "choThanhToan":
      return "chờ thanh toán";
    case "daThanhToan":
      if (d.maGiam === null) {
        return "đã thanh toán, không mã giảm giá";
      }
      return "đã thanh toán, mã giảm giá " + d.maGiam;
    case "daHuy":
      return "đã huỷ";
  }
}

console.log(moTaDonHang({ kind: "daHuy" }));
console.log(moTaDonHang({ kind: "daThanhToan", maGiam: "FREESHIP" }));
```

Hai dòng cuối in ra gì?

:::opt{correct}
`đã huỷ` rồi `đã thanh toán, mã giảm giá FREESHIP`
:::

:::opt
`đã thanh toán, mã giảm giá FREESHIP` rồi `đã huỷ` — vì đó đúng NỘI
DUNG hai nhánh `switch` sẽ khớp, chỉ đảo thứ tự
::why
Gần đúng ở việc bạn nhận ra ĐÚNG hai câu sẽ xuất hiện — một câu là
`"đã huỷ"`, một câu là `"đã thanh toán, mã giảm giá FREESHIP"` — bạn
đọc đúng NỘI DUNG hai nhánh `switch`.

Chỗ lệch: thứ tự IN RA đi theo ĐÚNG thứ tự các dòng `console.log`
trong mã nguồn, không theo thứ tự khai báo biến thể trong `type
DonHang`. Dòng `console.log` ĐẦU TIÊN gọi `moTaDonHang` với `kind:
"daHuy"` nên in `đã huỷ` TRƯỚC; dòng THỨ HAI gọi với `kind:
"daThanhToan"` nên in `đã thanh toán, mã giảm giá FREESHIP` SAU.
::
:::

:::opt
Máy báo lỗi biên dịch ở dòng `moTaDonHang({ kind: "daHuy" })` — vì
thiếu field `maGiam` so với biến thể `daThanhToan`
::why
Gần đúng ở việc bạn để ý biến thể `daThanhToan` CÓ field `maGiam` mà
object này thiếu — quan sát về CẤU TRÚC field đó đúng.

Chỗ lệch: `{ kind: "daHuy" }` khớp CHÍNH XÁC hình dạng biến thể
`daHuy` (chỉ cần field `kind`) — nó là một `DonHang` HỢP LỆ, không
phải lỗi. `DonHang` là UNION của ba hình dạng, một giá trị chỉ cần
khớp ĐÚNG MỘT trong ba, không cần mang field của các biến thể còn
lại.
::
:::
::::

::::code{#tom_tat_don_hang}
Interface `DonHangCu` (cờ rời rạc) bên dưới đã được THAY bằng
discriminated union `DonHang` — ba `case` xử lý ba biến thể, nhưng
nhánh `daThanhToan` còn thiếu một chỗ: đọc ĐÚNG field mang mã giảm giá
của chính biến thể đó.

```typescript title=starter
interface DonHangCu {
  daThanhToan: boolean;
  daHuy: boolean;
  maGiam: string | null;
}

type DonHang =
  | { kind: "choThanhToan" }
  | { kind: "daThanhToan"; maGiam: string | null }
  | { kind: "daHuy" };

function tomTatDonHang(d: DonHang): string {
  switch (d.kind) {
    case "choThanhToan":
      return "CHO_TT";
    case "daThanhToan":
      return d.maGiam === null ? "DA_TT" : "DA_TT:" + d.___;
    case "daHuy":
      return "DA_HUY";
  }
}

console.log(tomTatDonHang({ kind: "daThanhToan", maGiam: "SALE10" }));
```

```typescript title=solution
interface DonHangCu {
  daThanhToan: boolean;
  daHuy: boolean;
  maGiam: string | null;
}

type DonHang =
  | { kind: "choThanhToan" }
  | { kind: "daThanhToan"; maGiam: string | null }
  | { kind: "daHuy" };

function tomTatDonHang(d: DonHang): string {
  switch (d.kind) {
    case "choThanhToan":
      return "CHO_TT";
    case "daThanhToan":
      return d.maGiam === null ? "DA_TT" : "DA_TT:" + d.maGiam;
    case "daHuy":
      return "DA_HUY";
  }
}

console.log(tomTatDonHang({ kind: "daThanhToan", maGiam: "SALE10" }));
```

```typescript title=test
const r1 = tomTatDonHang({ kind: "choThanhToan" });
if (r1 !== "CHO_TT") throw new Error("tomTatDonHang cho choThanhToan phải trả về \"CHO_TT\" — đang là " + r1);

const r2 = tomTatDonHang({ kind: "daThanhToan", maGiam: "SALE10" });
if (r2 !== "DA_TT:SALE10") throw new Error("tomTatDonHang cho daThanhToan có mã giảm giá phải trả về \"DA_TT:SALE10\" — đang là " + r2);

const r3 = tomTatDonHang({ kind: "daThanhToan", maGiam: null });
if (r3 !== "DA_TT") throw new Error("tomTatDonHang cho daThanhToan không mã giảm giá phải trả về \"DA_TT\" — đang là " + r3);

const r4 = tomTatDonHang({ kind: "daHuy" });
if (r4 !== "DA_HUY") throw new Error("tomTatDonHang cho daHuy phải trả về \"DA_HUY\" — đang là " + r4);
```

:::hints
- kind: attention
  body: "Chỗ trống là field bạn đọc RA để ghép vào chuỗi kết quả khi đơn ĐÃ thanh toán VÀ CÓ mã giảm giá — biến thể daThanhToan của DonHang có field nào ngoài kind?"
- kind: strategy
  body: 'Biến thể { kind: "daThanhToan"; maGiam: string | null } khai field maGiam — bên trong nhánh case "daThanhToan", TypeScript đã NARROW d thành đúng biến thể đó, field còn lại để đọc là maGiam.'
- kind: one-line
  body: "Chỗ trống là: maGiam"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "DA_TT:SALE10"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cờ rời rạc, biến mất. Discriminated union, tự tay bạn viết — mỗi biến
thể chỉ mang đúng field nó cần, không còn tổ hợp mơ hồ nào viết ra
được nữa.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa tự tay LÀM lại đúng quy trình bài trước trình diễn — cờ rời
rạc thành union, hàm `switch` xử lý đủ biến thể. Nhưng đây không phải
LẦN ĐẦU bạn giải bài toán này: T4.1 (Python), bạn từng mô hình hoá một
state machine bằng NHIỀU `@dataclass(frozen=True)` riêng, rồi dùng
`match`/`case` để phân biệt.

Cùng một Ý — mỗi trạng thái một hình dạng riêng, hàm chuyển trạng thái
kiểm đủ mọi hình dạng — nhưng hai ngôn ngữ có thật sự làm CÙNG MỘT CƠ
CHẾ không, hay chỉ giống nhau ở BỀ NGOÀI cú pháp?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
