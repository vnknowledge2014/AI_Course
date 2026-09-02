---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.ghep-applicative-va-monad-form-nhieu-buoc
title: "Ghép Applicative + Monad: form có bước ĐỘC LẬP VÀ bước PHỤ THUỘC"
summary: "map2GomLoi gom lỗi của soLuong VÀ donGia ĐỘC LẬP (Applicative); chainResult tính bước PHỤ THUỘC — tổng tiền — CHỈ SAU KHI cả hai đã hợp lệ (Monad). Một form thật cần CẢ hai kiểu ghép, không chỉ một."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 50
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [alg.combine-applicative-monad]
requires: [alg.combine-monoid-functor]
concepts: [alg.combine-applicative-monad]
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
Bài trước ghép Monoid với Functor: một chuỗi PHẲNG — `.map()` rồi
`gopTatCa`, bước sau không cần biết GIÁ TRỊ THẬT bước trước tính ra.
Một form đăng ký thật không phẳng như vậy: vài trường VALIDATE ĐỘC
LẬP, rồi một bước chỉ tính được SAU KHI tất cả đã hợp lệ.
::::

::::explain{#ghep-doc-lap-va-phu-thuoc}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function loiMot<T, E>(l: E): Result<T, E[]> { return { kind: "loi", loi: [l] }; }

function map2GomLoi<A, B, C, E>(ra: Result<A, E[]>, rb: Result<B, E[]>, f: (a: A, b: B) => C): Result<C, E[]> {
  if (ra.kind === "loi" && rb.kind === "loi") return { kind: "loi", loi: [...ra.loi, ...rb.loi] };
  if (ra.kind === "loi") return { kind: "loi", loi: ra.loi };
  if (rb.kind === "loi") return { kind: "loi", loi: rb.loi };
  return ok(f(ra.giaTri, rb.giaTri));
}

function chainResult<T, U, E>(r: Result<T, E>, f: (x: T) => Result<U, E>): Result<U, E> {
  switch (r.kind) {
    case "ok": return f(r.giaTri);
    case "loi": return loi(r.loi);
  }
}

function validateSoLuong(soLuong: number): Result<number, string[]> {
  return soLuong > 0 ? ok(soLuong) : loiMot("so luong phai lon hon 0");
}

function validateDonGia(donGia: number): Result<number, string[]> {
  return donGia > 0 ? ok(donGia) : loiMot("don gia phai lon hon 0");
}

function xuLyDonHang(soLuong: number, donGia: number): Result<number, string[]> {
  const daGhep = map2GomLoi(validateSoLuong(soLuong), validateDonGia(donGia), (sl, dg) => ({ soLuong: sl, donGia: dg }));
  return chainResult(daGhep, (don) => {
    const tongTien = don.soLuong * don.donGia;
    return tongTien > 1000000 ? loiMot("tong tien vuot qua han muc 1.000.000") : ok(tongTien);
  });
}

console.log(JSON.stringify(xuLyDonHang(5, 20000)));
console.log(JSON.stringify(xuLyDonHang(-1, -1)));
```

```text
{"kind":"ok","giaTri":100000}
{"kind":"loi","loi":["so luong phai lon hon 0","don gia phai lon hon 0"]}
```

`map2GomLoi(validateSoLuong(soLuong), validateDonGia(donGia), ...)` —
Applicative — validate CẢ HAI trường ĐỘC LẬP: `validateSoLuong` không
cần biết `donGia`, `validateDonGia` không cần biết `soLuong`. Cả hai
LUÔN chạy, dù bên nào đã sai — khi CẢ HAI đều sai (dòng 2), kết quả
gom ĐỦ hai thông điệp lỗi, không chỉ một.

`chainResult(daGhep, f)` — Monad — khác hẳn: `f` (hàm tính `tongTien`)
CHỈ chạy khi `daGhep.kind === "ok"` (nhìn `switch`: nhánh `"loi"` trả
thẳng `loi(r.loi)` ra ngoài, KHÔNG gọi `f`). `f` cần GIÁ TRỊ THẬT của
`soLuong` VÀ `donGia` — không chỉ "cả hai hợp lệ" mà chính CON SỐ đó —
một thứ `map2GomLoi` một mình không cung cấp: nó gom LỖI, không "chờ"
để đưa giá trị đã gộp sang bước tiếp theo.
::::

::::example{#mot-truong-sai-va-buoc-phu-thuoc-that-bai}
```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function loiMot<T, E>(l: E): Result<T, E[]> { return { kind: "loi", loi: [l] }; }

function map2GomLoi<A, B, C, E>(ra: Result<A, E[]>, rb: Result<B, E[]>, f: (a: A, b: B) => C): Result<C, E[]> {
  if (ra.kind === "loi" && rb.kind === "loi") return { kind: "loi", loi: [...ra.loi, ...rb.loi] };
  if (ra.kind === "loi") return { kind: "loi", loi: ra.loi };
  if (rb.kind === "loi") return { kind: "loi", loi: rb.loi };
  return ok(f(ra.giaTri, rb.giaTri));
}

function chainResult<T, U, E>(r: Result<T, E>, f: (x: T) => Result<U, E>): Result<U, E> {
  switch (r.kind) {
    case "ok": return f(r.giaTri);
    case "loi": return loi(r.loi);
  }
}

function validateSoLuong(soLuong: number): Result<number, string[]> {
  return soLuong > 0 ? ok(soLuong) : loiMot("so luong phai lon hon 0");
}

function validateDonGia(donGia: number): Result<number, string[]> {
  return donGia > 0 ? ok(donGia) : loiMot("don gia phai lon hon 0");
}

function xuLyDonHang(soLuong: number, donGia: number): Result<number, string[]> {
  const daGhep = map2GomLoi(validateSoLuong(soLuong), validateDonGia(donGia), (sl, dg) => ({ soLuong: sl, donGia: dg }));
  return chainResult(daGhep, (don) => {
    const tongTien = don.soLuong * don.donGia;
    return tongTien > 1000000 ? loiMot("tong tien vuot qua han muc 1.000.000") : ok(tongTien);
  });
}

console.log(JSON.stringify(xuLyDonHang(-3, 20000)));
console.log(JSON.stringify(xuLyDonHang(100, 20000)));
```

```text title=readonly
{"kind":"loi","loi":["so luong phai lon hon 0"]}
{"kind":"loi","loi":["tong tien vuot qua han muc 1.000.000"]}
```

Dòng đầu: `soLuong = -3` sai, `donGia = 20000` đúng — `map2GomLoi` đi
vào nhánh `if (ra.kind === "loi") return { kind: "loi", loi: ra.loi }`
— trả ĐÚNG MỘT lỗi (của `soLuong`), không đợi thêm gì từ `donGia` (dù
`donGia` đã được validate, nó hợp lệ nên không đóng góp lỗi nào). Vì
`daGhep` đã là `loi`, `chainResult` KHÔNG BAO GIỜ chạy bước tính
`tongTien`.

Dòng hai: `soLuong = 100` VÀ `donGia = 20000` ĐỀU hợp lệ — `map2GomLoi`
trả `ok`. Nhưng `100 * 20000` bằng `2000000`, vượt hạn mức `1000000`
— bước PHỤ THUỘC (`chainResult`) tự nó phát hiện một lỗi RIÊNG, chẳng
liên quan gì tới lỗi của `map2GomLoi` (vì bước gom lỗi đã "ok" từ
đầu, lỗi ở đây đến từ một QUY TẮC khác — quy tắc chỉ áp dụng được sau
khi có GIÁ TRỊ THẬT).
::::

::::predict{#doan-mot-truong-sai commitOnce}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function loiMot<T, E>(l: E): Result<T, E[]> { return { kind: "loi", loi: [l] }; }

function map2GomLoi<A, B, C, E>(ra: Result<A, E[]>, rb: Result<B, E[]>, f: (a: A, b: B) => C): Result<C, E[]> {
  if (ra.kind === "loi" && rb.kind === "loi") return { kind: "loi", loi: [...ra.loi, ...rb.loi] };
  if (ra.kind === "loi") return { kind: "loi", loi: ra.loi };
  if (rb.kind === "loi") return { kind: "loi", loi: rb.loi };
  return ok(f(ra.giaTri, rb.giaTri));
}

function chainResult<T, U, E>(r: Result<T, E>, f: (x: T) => Result<U, E>): Result<U, E> {
  switch (r.kind) {
    case "ok": return f(r.giaTri);
    case "loi": return loi(r.loi);
  }
}

function validateSoLuong(soLuong: number): Result<number, string[]> {
  return soLuong > 0 ? ok(soLuong) : loiMot("so luong phai lon hon 0");
}

function validateDonGia(donGia: number): Result<number, string[]> {
  return donGia > 0 ? ok(donGia) : loiMot("don gia phai lon hon 0");
}

function xuLyDonHang(soLuong: number, donGia: number): Result<number, string[]> {
  const daGhep = map2GomLoi(validateSoLuong(soLuong), validateDonGia(donGia), (sl, dg) => ({ soLuong: sl, donGia: dg }));
  return chainResult(daGhep, (don) => {
    const tongTien = don.soLuong * don.donGia;
    return tongTien > 1000000 ? loiMot("tong tien vuot qua han muc 1.000.000") : ok(tongTien);
  });
}

console.log(JSON.stringify(xuLyDonHang(0, 5)));
```

Dòng cuối in ra gì?

:::opt{correct}
`{"kind":"loi","loi":["so luong phai lon hon 0"]}`
:::

:::opt
`{"kind":"loi","loi":["so luong phai lon hon 0","tong tien vuot qua han muc 1.000.000"]}` — vì `soLuong` sai nên CẢ bước gom lỗi VÀ bước tính `tongTien` đều chạy, gộp lại thành hai lỗi
::why
Gần đúng ở việc bạn xác định ĐÚNG `soLuong = 0` không hợp lệ (`0`
không lớn hơn `0`) — `map2GomLoi` thật sự tạo ra một `loi` chứa
thông điệp này.

Chỗ lệch: `chainResult(daGhep, f)` chỉ gọi `f` KHI `daGhep.kind ===
"ok"` — nhìn nhánh `case "loi": return loi(r.loi)` trong
`chainResult`: TRẢ THẲNG lỗi ra ngoài, KHÔNG gọi `f`. Vì `daGhep` đã
LÀ `loi` (do `soLuong` sai), bước tính `tongTien` KHÔNG BAO GIỜ chạy
— không có lỗi thứ hai nào được thêm vào.
::
:::

:::opt
`{"kind":"ok","giaTri":5}` — vì `donGia = 5` hợp lệ, nên kết quả lấy giá trị hợp lệ đó, bỏ qua trường sai
::why
Gần đúng ở việc bạn thấy `donGia = 5` THẬT SỰ hợp lệ (`5 > 0`) —
`validateDonGia(5)` đúng là trả `ok(5)`.

Chỗ lệch: `map2GomLoi` KHÔNG "bỏ qua" trường sai để lấy trường đúng —
nhìn nhánh `if (ra.kind === "loi") return { kind: "loi", loi: ra.loi
}`: CHỈ CẦN MỘT trong hai (`ra` hoặc `rb`) là `loi` thì KẾT QUẢ CUỐI
CÙNG là `loi`, không có "ok với giá trị mặc định". `soLuong` sai
(`loi`) nên toàn bộ `map2GomLoi` trả về `loi`, dù `donGia` hợp lệ.
::
:::
::::

::::code{#xu_ly_dang_ky}
Viết `xuLyDangKy(tuoi: number, email: string): Result<number, string[]>`
— validate `tuoi` (0 đến 150) và `email` (phải chứa `@`) ĐỘC LẬP bằng
`map2GomLoi` (gom CẢ HAI lỗi nếu cả hai sai), rồi NẾU CẢ HAI hợp lệ,
dùng `chainResult` để tính bước PHỤ THUỘC: tổng `tuổi + độ dài email`
— báo lỗi nếu tổng vượt quá `100`.

```typescript title=starter
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function loiMot<T, E>(l: E): Result<T, E[]> { return { kind: "loi", loi: [l] }; }

function map2GomLoi<A, B, C, E>(ra: Result<A, E[]>, rb: Result<B, E[]>, f: (a: A, b: B) => C): Result<C, E[]> {
  if (ra.kind === "loi" && rb.kind === "loi") return { kind: "loi", loi: [...ra.loi, ...rb.loi] };
  if (ra.kind === "loi") return { kind: "loi", loi: ra.loi };
  if (rb.kind === "loi") return { kind: "loi", loi: rb.loi };
  return ok(f(ra.giaTri, rb.giaTri));
}

function chainResult<T, U, E>(r: Result<T, E>, f: (x: T) => Result<U, E>): Result<U, E> {
  switch (r.kind) {
    case "ok": return f(r.giaTri);
    case "loi": return loi(r.loi);
  }
}

function validateTuoi(tuoi: number): Result<number, string[]> {
  return tuoi >= 0 && tuoi <= 150 ? ok(tuoi) : loiMot("tuoi phai trong khoang 0 den 150");
}

function validateEmail(email: string): Result<string, string[]> {
  return email.includes("@") ? ok(email) : loiMot("email phai chua ky tu @");
}

function xuLyDangKy(tuoi: number, email: string): Result<number, string[]> {
  const daGhep = ___;
  return chainResult(daGhep, (cap) => {
    const tong = ___;
    return tong > 100 ? loiMot("tong tuoi cong do dai email vuot qua 100") : ok(tong);
  });
}

console.log(JSON.stringify(xuLyDangKy(30, "a@b.com")));
```

```typescript title=solution
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function loiMot<T, E>(l: E): Result<T, E[]> { return { kind: "loi", loi: [l] }; }

function map2GomLoi<A, B, C, E>(ra: Result<A, E[]>, rb: Result<B, E[]>, f: (a: A, b: B) => C): Result<C, E[]> {
  if (ra.kind === "loi" && rb.kind === "loi") return { kind: "loi", loi: [...ra.loi, ...rb.loi] };
  if (ra.kind === "loi") return { kind: "loi", loi: ra.loi };
  if (rb.kind === "loi") return { kind: "loi", loi: rb.loi };
  return ok(f(ra.giaTri, rb.giaTri));
}

function chainResult<T, U, E>(r: Result<T, E>, f: (x: T) => Result<U, E>): Result<U, E> {
  switch (r.kind) {
    case "ok": return f(r.giaTri);
    case "loi": return loi(r.loi);
  }
}

function validateTuoi(tuoi: number): Result<number, string[]> {
  return tuoi >= 0 && tuoi <= 150 ? ok(tuoi) : loiMot("tuoi phai trong khoang 0 den 150");
}

function validateEmail(email: string): Result<string, string[]> {
  return email.includes("@") ? ok(email) : loiMot("email phai chua ky tu @");
}

function xuLyDangKy(tuoi: number, email: string): Result<number, string[]> {
  const daGhep = map2GomLoi(validateTuoi(tuoi), validateEmail(email), (t, e) => ({ tuoi: t, email: e }));
  return chainResult(daGhep, (cap) => {
    const tong = cap.tuoi + cap.email.length;
    return tong > 100 ? loiMot("tong tuoi cong do dai email vuot qua 100") : ok(tong);
  });
}

console.log(JSON.stringify(xuLyDangKy(30, "a@b.com")));
```

```typescript title=test
const caHopLe = xuLyDangKy(30, "a@b.com");
if (caHopLe.kind !== "ok") throw new Error("ca hai truong hop le thi phai ok");
if (caHopLe.giaTri !== 37) throw new Error("gia tri phai la tuoi + do dai email = 30 + 7 = 37, nhan duoc " + caHopLe.giaTri);

const caKhongHopLe = xuLyDangKy(-5, "khongco");
if (caKhongHopLe.kind !== "loi") throw new Error("ca hai truong khong hop le thi phai loi");
if (caKhongHopLe.loi.length !== 2) throw new Error("phai gom CA HAI loi, nhan duoc " + caKhongHopLe.loi.length + " loi");
if (caKhongHopLe.loi[0] !== "tuoi phai trong khoang 0 den 150") throw new Error("loi dau tien phai la loi cua tuoi");
if (caKhongHopLe.loi[1] !== "email phai chua ky tu @") throw new Error("loi thu hai phai la loi cua email");

const chiTuoiSai = xuLyDangKy(200, "a@b.com");
if (chiTuoiSai.kind !== "loi") throw new Error("tuoi vuot qua 150 thi phai loi");
if (chiTuoiSai.loi.length !== 1) throw new Error("chi CO MOT truong sai thi chi gom MOT loi, nhan duoc " + chiTuoiSai.loi.length);
if (chiTuoiSai.loi[0] !== "tuoi phai trong khoang 0 den 150") throw new Error("loi phai la loi cua tuoi");

const buocPhuThuocSai = xuLyDangKy(90, "aaaaaaaaaaaaaaaa@b.com");
if (buocPhuThuocSai.kind !== "loi") throw new Error("ca hai truong hop le nhung tong vuot qua 100 thi van phai loi");
if (buocPhuThuocSai.loi.length !== 1) throw new Error("loi cua buoc phu thuoc chi co MOT phan tu");
if (buocPhuThuocSai.loi[0] !== "tong tuoi cong do dai email vuot qua 100") throw new Error("loi phai la loi cua buoc tinh tong phu thuoc");
```

:::hints
- kind: attention
  body: "daGhep phải gọi map2GomLoi với ĐÚNG BA đối số: validateTuoi(tuoi), validateEmail(email), và một hàm gộp hai giá trị hợp lệ thành một object { tuoi, email }. tong phải cộng cap.tuoi (số) VỚI cap.email.length (độ dài chuỗi) — thiếu một trong hai vế thì tổng sai."
- kind: strategy
  body: "daGhep: map2GomLoi(validateTuoi(tuoi), validateEmail(email), (t, e) => ({ tuoi: t, email: e })) — Applicative, LUÔN chạy CẢ HAI validate rồi gom lỗi nếu có. tong: cap.tuoi + cap.email.length — chỉ TÍNH ĐƯỢC sau khi chainResult đã xác nhận daGhep là \"ok\", lúc đó cap chứa CẢ tuoi và email THẬT (đã \"mở\" ra khỏi Result, không còn là Result nữa)."
- kind: one-line
  body: "daGhep: map2GomLoi(validateTuoi(tuoi), validateEmail(email), (t, e) => ({ tuoi: t, email: e })); tong: cap.tuoi + cap.email.length"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: '{"kind":"ok","giaTri":37}'
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`map2GomLoi` gom lỗi của những bước KHÔNG cần nhau. `chainResult` nối
bước CẦN giá trị thật của bước trước. Một form thật thường cần CẢ hai
— không phải chọn MỘT trong hai.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`xuLyDangKy` xử lý HAI trường, MỘT giá trị. Nhưng nếu có một MẢNG cả
loạt cây biểu thức (ADT đệ quy từ T4.3), mỗi cây tính ra một số — có
cách nào tính TOÀN BỘ mảng đó thành một mảng số, vẫn theo đúng khuôn
đã học ở cụm này?

Bài sau ghép Traverse với Catamorphism để trả lời.
::::

::::checkpoint{mastery=0.8}
::::
