---
id: lap-trinh-ham.adt-pattern-matching.do-tong-hop-illegal-states
title: "Đo tổng hợp: illegal states unrepresentable"
summary: "Bài chốt cụm 4, code có chấm điểm sống: cho một mô tả nghiệp vụ MỚI (chưa từng dùng trong track) có nhiều cờ rời rạc, tự thiết kế discriminated union thay thế và một hàm xử lý đủ biến thể — không khái niệm mới, đo khả năng NHẬN RA khi nào cờ boolean/optional đang che giấu một sum type."
locale: vi
track: lap-trinh-ham
module: adt-pattern-matching
order: 25
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [ts.review-illegal-states]
requires: [ts.compare-python-dataclass-vs-ts-union]
concepts: [ts.review-illegal-states]
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
Bài trước bạn so sánh Python's `match` và TS's `switch` — cùng một Ý,
khác cơ chế. Cụm này khép lại hôm nay: một mô tả nghiệp vụ HOÀN TOÀN
MỚI, cờ rời rạc kiểu cũ — bạn có tự NHẬN RA nó đang che một sum type,
và tự sửa được không?
::::

::::explain{#on-lai-co-va-union}
Một ví dụ MỚI, chưa dùng ở đâu trong track: trạng thái một GHẾ trong
rạp chiếu phim, mô tả bằng cờ rời rạc quen thuộc.

```typescript
interface Ghe {
  daDat: boolean;
  tenNguoiDat: string | null;
  daBan: boolean;
  maVe: string | null;
}

const g: Ghe = {
  daDat: true,
  tenNguoiDat: "An",
  daBan: true,
  maVe: "VE-9",
};

console.log("daDat=" + g.daDat + " daBan=" + g.daBan);
```

```text
daDat=true daBan=true
```

`Ghe` có bốn field độc lập: `daDat`/`tenNguoiDat` (đã ĐẶT chỗ chưa,
đặt bởi ai) và `daBan`/`maVe` (đã BÁN chưa, mã vé nào). TypeScript
biên dịch `g` ở trên HOÀN TOÀN BÌNH THƯỜNG — dù tổ hợp đó VÔ NGHĨA về
nghiệp vụ: một ghế không thể VỪA đang được một người GIỮ CHỖ (chưa trả
tiền) VỪA đã BÁN xong (đã trả tiền, có vé). Đúng cơ chế cụm này đã chỉ
ra ở bài 20: cờ boolean/optional rời rạc không hề NGĂN một tổ hợp phi
lý.

Thay bốn field rời rạc đó bằng MỘT discriminated union ba biến thể —
`Trong` (trống, không ai giữ), `DaDat` (chỉ mang `tenNguoiDat`),
`DaBan` (chỉ mang `maVe`) — thì tổ hợp "vừa đặt vừa bán" KHÔNG CÒN
VIẾT RA ĐƯỢC nữa: biến thể `DaDat` không khai field `maVe`, gán nó vào
bị TypeScript CHẶN NGAY lúc biên dịch, mã `TS2353` — cơ chế bài 21 đã
dùng ("excess property check": một object literal gán TRỰC TIẾP vào
một vị trí đã khai kiểu không được có field THỪA so với đúng biến thể
nó khớp).
::::

::::example{#khong-con-viet-duoc-trang-thai-vo-nghia}
Viết lại `Ghe` bằng union, và một hàm xử lý ĐỦ cả ba biến thể:

```typescript title=readonly
interface Trong {
  kind: "trong";
}

interface DaDat {
  kind: "daDat";
  tenNguoiDat: string;
}

interface DaBan {
  kind: "daBan";
  maVe: string;
}

type Ghe = Trong | DaDat | DaBan;

function moTaGhe(g: Ghe): string {
  switch (g.kind) {
    case "trong":
      return "ghế trống";
    case "daDat":
      return "ghế đã đặt bởi " + g.tenNguoiDat;
    case "daBan":
      return "ghế đã bán, vé " + g.maVe;
  }
}

console.log(moTaGhe({ kind: "trong" }));
console.log(moTaGhe({ kind: "daDat", tenNguoiDat: "An" }));
console.log(moTaGhe({ kind: "daBan", maVe: "VE-9" }));
```

```text title=readonly
ghế trống
ghế đã đặt bởi An
ghế đã bán, vé VE-9
```

Giờ thử viết CHÍNH cái tổ hợp vô nghĩa lúc nãy — một ghế vừa đã đặt
vừa có mã vé — nhưng bằng union mới:

```typescript title=readonly
interface Trong {
  kind: "trong";
}

interface DaDat {
  kind: "daDat";
  tenNguoiDat: string;
}

interface DaBan {
  kind: "daBan";
  maVe: string;
}

type Ghe = Trong | DaDat | DaBan;

const gSai: Ghe = { kind: "daDat", tenNguoiDat: "An", maVe: "VE-9" };
console.log(gSai);
```

```text title=readonly
(không in ra gì cả)

TS2353 (dòng 17, cột 55): Object literal may only specify known
properties, and 'maVe' does not exist in type 'DaDat'.
```

Không phải TypeScript "phát hiện" tổ hợp phi lý này LÚC CHẠY — không
có dòng nào chạy tới `console.log`, chương trình không hề khởi động.
Nó bị CHẶN NGAY từ lúc BIÊN DỊCH, vì đơn giản KHÔNG CÓ chỗ nào trong
kiểu `DaDat` để viết field `maVe` vào. Lỗi bị đẩy từ "phải nhớ kiểm
tra lúc chạy" sang "không có không gian kiểu nào để viết ra nó" — đúng
Ý cốt lõi mà track này gọi là "illegal states unrepresentable".
::::

::::predict{#doan-tin-nhan-vua-gui-vua-loi commitOnce}
```typescript
interface DangGui {
  kind: "dangGui";
}

interface DaGui {
  kind: "daGui";
  thoiGianGui: string;
}

interface GuiThatBai {
  kind: "guiThatBai";
  lyDoLoi: string;
}

type TrangThaiTinNhan = DangGui | DaGui | GuiThatBai;

const tn: TrangThaiTinNhan = {
  kind: "daGui",
  thoiGianGui: "10:30",
  lyDoLoi: "mất mạng",
};
console.log(tn);
```

Dòng cuối in ra gì?

:::opt{correct}
Không dòng nào được in ra — TypeScript từ chối BIÊN DỊCH ngay tại dòng
gán `tn`, báo lỗi TS2353 (`lyDoLoi` không tồn tại trên kiểu `DaGui`),
chương trình không chạy tới `console.log`
:::

:::opt
`{ kind: "daGui", thoiGianGui: "10:30", lyDoLoi: "mất mạng" }` — object
vẫn được tạo ra bình thường, dư field nào cũng được vì TypeScript chỉ
cấm THIẾU field, không cấm THỪA field
::why
Gần đúng ở việc bạn nhớ đúng một điều CÓ THẬT: TypeScript nói chung CHO
PHÉP một giá trị có nhiều field hơn kiểu yêu cầu, khi giá trị đó đi qua
một biến trung gian đã có kiểu suy sẵn (structural typing, không phải
nominal typing) — quan sát đó đúng trong nhiều tình huống khác.

Chỗ lệch: đây là gán TRỰC TIẾP một OBJECT LITERAL (viết ngay tại chỗ,
không qua biến trung gian) vào một vị trí ĐÃ KHAI kiểu (`tn:
TrangThaiTinNhan`). Ở đúng tình huống này, TypeScript bật thêm một lớp
kiểm gọi là "excess property check": field THỪA so với đúng biến thể
khớp (`DaGui`, khớp qua `kind: "daGui"`) bị chặn ngay, mã TS2353 —
không có ngoại lệ "dư field nào cũng được" ở đây.
::
:::

:::opt
Máy báo lỗi biên dịch — nhưng ở dòng khai `type TrangThaiTinNhan =
DangGui | DaGui | GuiThatBai;`, vì `DaGui` và `GuiThatBai` không được
phép có field khác tên nhau trong cùng một union
::why
Gần đúng ở việc bạn đoán ĐÚNG là CÓ lỗi biên dịch — quan sát đó đúng.

Chỗ lệch: khai báo UNION không bao giờ tự nó gây lỗi — mỗi biến thể là
một `interface` ĐỘC LẬP, tên field trùng hay khác nhau giữa các biến
thể không quan trọng gì (`Ghe` ở trên y hệt: `DaDat`/`DaBan` có field
khác tên nhau, dòng `type Ghe = ...` biên dịch sạch). Lỗi thật nằm ở
dòng GÁN `tn`, nơi object literal có field `lyDoLoi` THỪA so với biến
thể `DaGui` mà nó khớp qua `kind`.
::
:::
::::

::::code{#mo_ta_cuoc_hen}
Trước đây, `CuocHen` từng là một `interface` với cờ rời rạc
(`daXacNhan: boolean`, `ngayHen: string | null`, `daHuy: boolean`,
`lyDoHuy: string | null`) — cho phép viết được một cuộc hẹn "vừa đã
xác nhận vừa đã huỷ". Union dưới đây đã thay thế nó, đúng như cụm này
đã làm nhiều lần. Hoàn thiện nốt nhánh `daXacNhan` — đọc đúng field
ngày hẹn của biến thể đó.

```typescript title=starter
interface ChoXacNhan {
  kind: "choXacNhan";
  ten: string;
}

interface DaXacNhan {
  kind: "daXacNhan";
  ten: string;
  ngayHen: string;
}

interface DaHuy {
  kind: "daHuy";
  ten: string;
  lyDoHuy: string;
}

type CuocHen = ChoXacNhan | DaXacNhan | DaHuy;

function moTaCuocHen(c: CuocHen): string {
  switch (c.kind) {
    case "choXacNhan":
      return "cuộc hẹn của " + c.ten + " đang chờ xác nhận";
    case "daXacNhan":
      return "cuộc hẹn của " + c.ten + " đã xác nhận, ngày " + c.___;
    case "daHuy":
      return "cuộc hẹn của " + c.ten + " đã huỷ, lý do: " + c.lyDoHuy;
  }
}

console.log(moTaCuocHen({ kind: "choXacNhan", ten: "Lan" }));
```

```typescript title=solution
interface ChoXacNhan {
  kind: "choXacNhan";
  ten: string;
}

interface DaXacNhan {
  kind: "daXacNhan";
  ten: string;
  ngayHen: string;
}

interface DaHuy {
  kind: "daHuy";
  ten: string;
  lyDoHuy: string;
}

type CuocHen = ChoXacNhan | DaXacNhan | DaHuy;

function moTaCuocHen(c: CuocHen): string {
  switch (c.kind) {
    case "choXacNhan":
      return "cuộc hẹn của " + c.ten + " đang chờ xác nhận";
    case "daXacNhan":
      return "cuộc hẹn của " + c.ten + " đã xác nhận, ngày " + c.ngayHen;
    case "daHuy":
      return "cuộc hẹn của " + c.ten + " đã huỷ, lý do: " + c.lyDoHuy;
  }
}

console.log(moTaCuocHen({ kind: "choXacNhan", ten: "Lan" }));
```

```typescript title=test
const r1 = moTaCuocHen({ kind: "choXacNhan", ten: "Lan" });
if (r1 !== "cuộc hẹn của Lan đang chờ xác nhận") throw new Error("case choXacNhan sai — đang là " + r1);

const r2 = moTaCuocHen({ kind: "daXacNhan", ten: "Minh", ngayHen: "2026-09-10" });
if (r2 !== "cuộc hẹn của Minh đã xác nhận, ngày 2026-09-10") throw new Error("case daXacNhan sai — đang là " + r2);

const r3 = moTaCuocHen({ kind: "daHuy", ten: "Hoa", lyDoHuy: "bận việc" });
if (r3 !== "cuộc hẹn của Hoa đã huỷ, lý do: bận việc") throw new Error("case daHuy sai — đang là " + r3);
```

:::hints
- kind: attention
  body: "Chỗ trống là field bạn đọc RA để lấy ngày hẹn — interface DaXacNhan có field nào ngoài kind và ten?"
- kind: strategy
  body: 'interface DaXacNhan khai kind: "daXacNhan", ten: string và ngayHen: string — bên trong nhánh case "daXacNhan", TypeScript đã NARROW c thành DaXacNhan, field còn lại cần đọc là ngayHen.'
- kind: one-line
  body: "Chỗ trống là: ngayHen"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "cuộc hẹn của Lan đang chờ xác nhận"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cờ rời rạc cho phép trạng thái vô nghĩa BIÊN DỊCH ĐƯỢC. Union đúng
thiết kế thì KHÔNG. Cụm 4 khép lại ở đây — bạn vừa tự đi lại toàn bộ
đường đó, trên một bài toán bạn chưa từng thấy.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Union vừa giải quyết bài toán "nhiều cờ rời rạc che giấu nhiều trạng
thái". Nhưng có một bài toán KHÁC, không liên quan gì tới nhiều biến
thể: nếu một hàm nhận HAI tham số CÙNG kiểu `string` (ví dụ một email,
một tên người dùng) — trình biên dịch có phân biệt được truyền NHẦM
thứ tự hai tham số đó không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
