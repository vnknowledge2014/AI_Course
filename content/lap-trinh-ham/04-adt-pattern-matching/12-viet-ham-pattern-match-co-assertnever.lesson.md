---
id: lap-trinh-ham.adt-pattern-matching.viet-ham-pattern-match-co-assertnever
title: "Viết một hàm pattern-match ĐẦY ĐỦ, có `assertNever` bảo vệ"
summary: "Bài code có chấm điểm sống: cho sẵn một discriminated union ba biến thể và hàm `chuaXuLy`, viết một hàm `switch` xử lý ĐỦ cả ba, gọi `chuaXuLy` ở `default` — test thử tính với TỪNG biến thể để đảm bảo không nhánh nào bị bỏ sót."
locale: vi
track: lap-trinh-ham
module: adt-pattern-matching
order: 12
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [ts.write-exhaustive-match]
requires: [ts.default-without-assertnever-hides-bugs]
concepts: [ts.write-exhaustive-match]
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
Bài trước: `default: return 0;` không gọi `chuaXuLy` VẪN biên dịch được —
và ÂM THẦM che giấu lỗi, chỉ khác bài 7 ở chỗ trông như đã xử lý đủ.
Hôm nay đến lượt bạn: tự viết một hàm `switch` ĐẦY ĐỦ, với `chuaXuLy`
đứng ĐÚNG vị trí bảo vệ.
::::

::::explain{#switch-day-du-co-chuaxuly}
```typescript
interface Vuong {
  kind: "vuong";
  canh: number;
}

interface Tron {
  kind: "tron";
  banKinhTron: number;
}

interface TamGiac {
  kind: "tamGiac";
  canhTamGiac: number;
}

type Hinh = Vuong | Tron | TamGiac;

function chuaXuLyHinh(x: never): never {
  throw new Error("Chưa xử lý hình: " + JSON.stringify(x));
}

function moTa(h: Hinh): string {
  switch (h.kind) {
    case "vuong":
      return "hình vuông cạnh " + h.canh;
    case "tron":
      return "hình tròn bán kính " + h.banKinhTron;
    case "tamGiac":
      return "tam giác đều cạnh " + h.canhTamGiac;
    default:
      return chuaXuLyHinh(h);
  }
}

console.log(moTa({ kind: "vuong", canh: 4 }));
console.log(moTa({ kind: "tron", banKinhTron: 3 }));
console.log(moTa({ kind: "tamGiac", canhTamGiac: 5 }));
```

```text
hình vuông cạnh 4
hình tròn bán kính 3
tam giác đều cạnh 5
```

Ba `case` xử lý ĐỦ ba biến thể của `Hinh` — cú pháp này bạn đã viết từ
bài 4/6, không có gì mới. Cái MỚI là dòng `default`: vì `case
"vuong"`/`"tron"`/`"tamGiac"` đã bao phủ CẢ BA giá trị literal mà
`h.kind` có thể mang, TypeScript thu hẹp kiểu của `h` tại `default`
xuống còn `never` (bài 8 đã đo). Hàm `chuaXuLyHinh` chỉ nhận tham số
kiểu `never` — nên `chuaXuLyHinh(h)` biên dịch được CHÍNH VÌ switch đã
ĐẦY ĐỦ, không phải ngẫu nhiên.

Đây mới ĐÚNG là "pattern-match ĐẦY ĐỦ" trong tiêu đề bài: không chỉ ba
`case` trả đúng kết quả (điều đó bài 6 đã đo), mà còn có `chuaXuLy`
đứng gác ở `default` — một hàm KHÔNG BAO GIỜ được gọi lúc chạy với dữ
liệu hợp lệ hôm nay, nhưng sẽ LÀM LỘ lỗi ngay lúc biên dịch nếu ai đó
mở rộng `Hinh` thêm một biến thể mà quên sửa `switch` (đúng cơ chế bài
9, giờ do CHÍNH BẠN dựng nên từ đầu).
::::

::::example{#kiem-tung-bien-the}
Vòng lặp gọi `moTa` với CẢ BA biến thể — không biến thể nào rơi vào
`default`:

```typescript title=readonly
interface Vuong {
  kind: "vuong";
  canh: number;
}

interface Tron {
  kind: "tron";
  banKinhTron: number;
}

interface TamGiac {
  kind: "tamGiac";
  canhTamGiac: number;
}

type Hinh = Vuong | Tron | TamGiac;

function chuaXuLyHinh(x: never): never {
  throw new Error("Chưa xử lý hình: " + JSON.stringify(x));
}

function moTa(h: Hinh): string {
  switch (h.kind) {
    case "vuong":
      return "hình vuông cạnh " + h.canh;
    case "tron":
      return "hình tròn bán kính " + h.banKinhTron;
    case "tamGiac":
      return "tam giác đều cạnh " + h.canhTamGiac;
    default:
      return chuaXuLyHinh(h);
  }
}

const cacBienThe: Hinh[] = [
  { kind: "vuong", canh: 4 },
  { kind: "tron", banKinhTron: 3 },
  { kind: "tamGiac", canhTamGiac: 5 },
];

let soLuongDaKiem = 0;
for (const h of cacBienThe) {
  moTa(h);
  soLuongDaKiem = soLuongDaKiem + 1;
}
console.log(
  "đã kiểm " + soLuongDaKiem + " biến thể, không biến thể nào rơi vào default"
);
```

```text title=readonly
đã kiểm 3 biến thể, không biến thể nào rơi vào default
```

Đây CHÍNH LÀ cách bài toán này được CHẤM: không chỉ gọi hàm MỘT lần
rồi so kết quả, mà LẶP qua TỪNG biến thể — nếu bạn (lúc viết `switch`)
lỡ quên một `case`, biến thể đó sẽ rơi vào `default`, `chuaXuLyHinh`
NÉM một `Error` ngay lập tức, và vòng lặp KHÔNG chạy hết — test thất
bại RÕ RÀNG, khác hẳn lỗi ÂM THẦM ở bài 7 (hàm trả `undefined`, không
ai biết).
::::

::::predict{#doan-ket-qua-switch-day-du commitOnce}
```typescript
interface Loi {
  kind: "loi";
  thongDiepLoi: string;
}

interface CanhBao {
  kind: "canhBao";
  thongDiepCanhBao: string;
}

interface ThanhCong {
  kind: "thanhCong";
  ketQua: string;
}

type ThongBao = Loi | CanhBao | ThanhCong;

function chuaXuLyThongBao(x: never): never {
  throw new Error("Chưa xử lý thông báo: " + JSON.stringify(x));
}

function hienThi(t: ThongBao): string {
  switch (t.kind) {
    case "loi":
      return "LỖI: " + t.thongDiepLoi;
    case "canhBao":
      return "CẢNH BÁO: " + t.thongDiepCanhBao;
    case "thanhCong":
      return "OK: " + t.ketQua;
    default:
      return chuaXuLyThongBao(t);
  }
}

console.log(hienThi({ kind: "canhBao", thongDiepCanhBao: "sắp hết dung lượng" }));
console.log(hienThi({ kind: "thanhCong", ketQua: "đã lưu" }));
```

Hai dòng cuối in ra gì?

:::opt{correct}
`CẢNH BÁO: sắp hết dung lượng` rồi `OK: đã lưu`
:::

:::opt
`sắp hết dung lượng` rồi `đã lưu` — vì đó là đúng giá trị field ứng
với từng biến thể, hàm chỉ trả về NGUYÊN VĂN field đó
::why
Gần đúng ở việc bạn đọc đúng FIELD ứng với từng biến thể —
`thongDiepCanhBao` đúng là `"sắp hết dung lượng"`, `ketQua` đúng là
`"đã lưu"`.

Chỗ lệch: mỗi `case` trong `hienThi` KHÔNG trả nguyên văn field — nó
NỐI THÊM một tiền tố (`"CẢNH BÁO: "`, `"OK: "`) trước giá trị field
bằng toán tử `+`. Output PHẢI có tiền tố đó, không phải giá trị field
trần trụi.
::
:::

:::opt
Máy ném lỗi lúc chạy — vì `switch` rơi vào nhánh `default`, gọi
`chuaXuLyThongBao`
::why
Gần đúng ở việc bạn nhớ ĐÚNG cấu trúc code: `default` trong `hienThi`
THẬT SỰ có gọi `chuaXuLyThongBao` — quan sát về code đúng.

Chỗ lệch: `default` CHỈ chạy khi `t.kind` KHÔNG khớp bất kỳ `case` nào
phía trên. Ở đây `"canhBao"` khớp đúng `case "canhBao"`, `"thanhCong"`
khớp đúng `case "thanhCong"` — switch không bao giờ CHẠM tới `default`
với hai lời gọi này. `chuaXuLyThongBao` chỉ chạy khi có một biến thể
KHÔNG được `case` nào xử lý.
::
:::
::::

::::code{#tom_tat_hinh}
Ba `case` bên dưới đã xử lý ĐỦ ba biến thể của `Hinh`. Việc còn lại:
hoàn thành nhánh `default`, gọi ĐÚNG hàm `chuaXuLyHinh` với tham số
`h` — đúng mẫu `assertNever` đã học ở bài 8.

```typescript title=starter
interface Vuong {
  kind: "vuong";
  canh: number;
}

interface Tron {
  kind: "tron";
  banKinhTron: number;
}

interface TamGiac {
  kind: "tamGiac";
  canhTamGiac: number;
}

type Hinh = Vuong | Tron | TamGiac;

function chuaXuLyHinh(x: never): never {
  throw new Error("Chưa xử lý hình: " + JSON.stringify(x));
}

function tomTat(h: Hinh): string {
  switch (h.kind) {
    case "vuong":
      return "VUONG(" + h.canh + ")";
    case "tron":
      return "TRON(" + h.banKinhTron + ")";
    case "tamGiac":
      return "TAMGIAC(" + h.canhTamGiac + ")";
    default:
      return ___;
  }
}

console.log(tomTat({ kind: "vuong", canh: 4 }));
```

```typescript title=solution
interface Vuong {
  kind: "vuong";
  canh: number;
}

interface Tron {
  kind: "tron";
  banKinhTron: number;
}

interface TamGiac {
  kind: "tamGiac";
  canhTamGiac: number;
}

type Hinh = Vuong | Tron | TamGiac;

function chuaXuLyHinh(x: never): never {
  throw new Error("Chưa xử lý hình: " + JSON.stringify(x));
}

function tomTat(h: Hinh): string {
  switch (h.kind) {
    case "vuong":
      return "VUONG(" + h.canh + ")";
    case "tron":
      return "TRON(" + h.banKinhTron + ")";
    case "tamGiac":
      return "TAMGIAC(" + h.canhTamGiac + ")";
    default:
      return chuaXuLyHinh(h);
  }
}

console.log(tomTat({ kind: "vuong", canh: 4 }));
```

```typescript title=test
const v = tomTat({ kind: "vuong", canh: 4 });
if (v !== "VUONG(4)") throw new Error("tomTat với Vuong phải trả về \"VUONG(4)\" — đang là " + v);

const t = tomTat({ kind: "tron", banKinhTron: 3 });
if (t !== "TRON(3)") throw new Error("tomTat với Tron phải trả về \"TRON(3)\" — đang là " + t);

const g = tomTat({ kind: "tamGiac", canhTamGiac: 5 });
if (g !== "TAMGIAC(5)") throw new Error("tomTat với TamGiac phải trả về \"TAMGIAC(5)\" — đang là " + g);

let daNem = false;
try {
  tomTat({ kind: "khac" } as any);
} catch (e) {
  daNem = true;
}
if (!daNem) throw new Error("tomTat với một biến thể KHÔNG xác định phải NÉM lỗi qua chuaXuLyHinh — chỗ trống không được là một giá trị cố định nào khác");
```

:::hints
- kind: attention
  body: "Chỗ trống là toàn bộ biểu thức sau return trong nhánh default — phải gọi chuaXuLyHinh, truyền đúng biến h (không phải h.kind hay một chuỗi tự viết)."
- kind: strategy
  body: "Ba case phía trên đã xử lý đủ \"vuong\"/\"tron\"/\"tamGiac\" — nên tại default, TypeScript đã thu hẹp kiểu của h xuống never (bài 8). chuaXuLyHinh nhận tham số kiểu never, nên chuaXuLyHinh(h) biên dịch được."
- kind: one-line
  body: "Chỗ trống là: chuaXuLyHinh(h)"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "VUONG(4)"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bạn vừa tự tay dựng một hàm pattern-match ĐẦY ĐỦ — ba `case`, một
`chuaXuLy` đứng gác ở `default`. Không phải đọc ví dụ có sẵn nữa — lần
này CHÍNH BẠN viết ra kỷ luật đó.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hôm nay `Hinh` có BA biến thể, bạn viết ĐỦ ba `case` cộng một
`chuaXuLy`. Nếu `Hinh` có BỐN biến thể — hay một union HOÀN TOÀN khác,
không phải hình học — thì kỷ luật này còn đứng vững không?

Bài sau đo lại TOÀN BỘ kỹ năng exhaustiveness trên một union lớn hơn,
khép cụm 2.
::::

::::checkpoint{mastery=0.8}
::::
