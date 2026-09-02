---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.ap-dung-validate-mot-form-nhieu-truong
title: "Áp dụng: validate một form NHIỀU trường, gom hết lỗi"
summary: "kiemTen/kiemTuoi/kiemEmail ghép qua map2GomLoi lồng hai lần — form SAI CẢ BA trường phải báo ĐỦ BA lỗi trong MỘT lần chạy, không phải sửa lỗi 1, chạy lại, thấy lỗi 2, sửa, chạy lại thấy lỗi 3."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 26
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [alg.apply-form-validation]
requires: [alg.write-map2-collect]
concepts: [alg.apply-form-validation]
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
`map2GomLoi` ghép được HAI `Result`. Nhưng một form THẬT thường có BA
trường trở lên — làm sao ghép hơn hai?
::::

::::explain{#long-map2gomloi}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loiMot<T, E>(l: E): Result<T, E[]> { return { kind: "loi", loi: [l] }; }
function map2GomLoi<A, B, C, E>(ra: Result<A, E[]>, rb: Result<B, E[]>, f: (a: A, b: B) => C): Result<C, E[]> {
  if (ra.kind === "loi" && rb.kind === "loi") return { kind: "loi", loi: [...ra.loi, ...rb.loi] };
  if (ra.kind === "loi") return { kind: "loi", loi: ra.loi };
  if (rb.kind === "loi") return { kind: "loi", loi: rb.loi };
  return ok(f(ra.giaTri, rb.giaTri));
}

function kiemTen(ten: string): Result<string, string[]> {
  return ten.length > 0 ? ok(ten) : loiMot("tên không được rỗng");
}
function kiemTuoi(tuoi: number): Result<number, string[]> {
  return tuoi >= 0 && tuoi <= 150 ? ok(tuoi) : loiMot("tuổi phải trong khoảng 0-150");
}
function kiemEmail(email: string): Result<string, string[]> {
  return email.includes("@") ? ok(email) : loiMot("email phải có @");
}

const buoc1 = map2GomLoi(kiemTen(""), kiemTuoi(200), (ten, tuoi) => ({ ten, tuoi }));
const ketQua = map2GomLoi(buoc1, kiemEmail("khong-hop-le"), (nguoiDung, email) => ({ ...nguoiDung, email }));

console.log(ketQua);
```

```text
{"kind":"loi","loi":["tên không được rỗng","tuổi phải trong khoảng 0-150","email phải có @"]}
```

Form SAI CẢ BA trường (`ten` rỗng, `tuoi` `200` ngoài khoảng, `email`
thiếu `@`) — kết quả BÁO ĐỦ CẢ BA lỗi, trong MỘT LẦN CHẠY DUY NHẤT.
Cách làm: LỒNG `map2GomLoi` — `buoc1` ghép `ten` với `tuoi` (gom lỗi
của CẢ HAI nếu có), rồi `ketQua` ghép `buoc1` (đã MANG lỗi của ten+tuoi
nếu có) VỚI `email`. `map2GomLoi` bên NGOÀI KHÔNG cần biết `buoc1` gồm
mấy trường — nó chỉ thấy `buoc1` là MỘT `Result`, gộp mảng lỗi ĐÚNG
như mọi lần khác.

So sánh trải nghiệm: KHÔNG PHẢI sửa lỗi tên, gửi lại, thấy lỗi tuổi,
sửa, gửi lại, thấy lỗi email — NGƯỜI DÙNG thấy ĐỦ cả ba lỗi NGAY LẦN
ĐẦU, sửa được TẤT CẢ trong một lượt.
::::

::::example{#form-hop-le-khong-loi}
Ngược lại — form HỢP LỆ, `map2GomLoi` lồng nhau vẫn cho ra kết quả
ĐÚNG, gộp cả BA giá trị lại:

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loiMot<T, E>(l: E): Result<T, E[]> { return { kind: "loi", loi: [l] }; }
function map2GomLoi<A, B, C, E>(ra: Result<A, E[]>, rb: Result<B, E[]>, f: (a: A, b: B) => C): Result<C, E[]> {
  if (ra.kind === "loi" && rb.kind === "loi") return { kind: "loi", loi: [...ra.loi, ...rb.loi] };
  if (ra.kind === "loi") return { kind: "loi", loi: ra.loi };
  if (rb.kind === "loi") return { kind: "loi", loi: rb.loi };
  return ok(f(ra.giaTri, rb.giaTri));
}

function kiemTen(ten: string): Result<string, string[]> {
  return ten.length > 0 ? ok(ten) : loiMot("tên không được rỗng");
}
function kiemTuoi(tuoi: number): Result<number, string[]> {
  return tuoi >= 0 && tuoi <= 150 ? ok(tuoi) : loiMot("tuổi phải trong khoảng 0-150");
}
function kiemEmail(email: string): Result<string, string[]> {
  return email.includes("@") ? ok(email) : loiMot("email phải có @");
}

const buoc1 = map2GomLoi(kiemTen("An"), kiemTuoi(30), (ten, tuoi) => ({ ten, tuoi }));
const ketQua = map2GomLoi(buoc1, kiemEmail("an@example.com"), (nguoiDung, email) => ({ ...nguoiDung, email }));

console.log(ketQua);
```

```text title=readonly
{"kind":"ok","giaTri":{"ten":"An","tuoi":30,"email":"an@example.com"}}
```

CẢ BA trường hợp lệ — kết quả là `"ok"`, MANG một object GỘP ĐỦ CẢ BA
giá trị (`ten`, `tuoi`, `email`). Không lỗi nào bị báo, không mảng lỗi
nào xuất hiện — đúng đối xứng với trường hợp lỗi: `map2GomLoi` gộp
GIÁ TRỊ khi tất cả `ok`, gộp LỖI khi có bất kỳ `loi` nào.
::::

::::predict{#doan-mot-truong-sai commitOnce}
Cùng ba hàm kiểm ở trên. Nếu CHỈ `tuoi` SAI (`ten` và `email` hợp lệ),
mảng lỗi cuối cùng sẽ CÓ MẤY phần tử?

:::opt{correct}
Một phần tử — chỉ lỗi của `tuoi`
:::

:::opt
Ba phần tử — vì `map2GomLoi` LUÔN báo lỗi cho CẢ BA trường một khi
CÓ BẤT KỲ trường nào sai, để nhắc người dùng kiểm tra lại toàn bộ
::why
Gần đúng ở việc bạn nghĩ tới việc "nhắc kiểm tra lại toàn bộ form" —
một cách thiết kế UX có thể gặp ở một số hệ thống khác (nhưng KHÔNG
phải cách `map2GomLoi` hoạt động).

Chỗ lệch: `map2GomLoi` CHỈ gộp lỗi từ những phần TỰ NÓ đã lỗi (`ra`
lỗi thì lấy lỗi của `ra`, `rb` lỗi thì lấy lỗi của `rb`, cả hai lỗi
thì gộp cả hai) — nó KHÔNG tạo ra lỗi MỚI cho những trường VẪN hợp lệ.
`kiemTen`/`kiemEmail` (đều hợp lệ ở đây) không đóng góp gì vào mảng
lỗi — chỉ CÓ ĐÚNG lỗi của `tuoi`.
::
:::

:::opt
Máy báo lỗi biên dịch — không thể GHÉP hai `map2GomLoi` LỒNG NHAU khi
chỉ MỘT trong ba trường lỗi
::why
Gần đúng ở việc bạn cân nhắc CẤU TRÚC lồng nhau có thể gây khó khăn gì
đó khi chỉ MỘT phần bị lỗi — một mối lo hợp lý khi mới thấy `map2GomLoi`
lồng hai lần.

Chỗ lệch: `map2GomLoi` hoạt động HOÀN TOÀN BÌNH THƯỜNG bất kể BAO
NHIÊU trường (không, một, hai, hay cả ba) bị lỗi — không có "trường
hợp đặc biệt" nào gây lỗi biên dịch. `buoc1`/`ketQua` LUÔN LÀ một
`Result` hợp lệ, dù mang giá trị hay mang lỗi.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Lồng `map2GomLoi` — ghép được BAO NHIÊU trường tuỳ ý, gom ĐỦ mọi lỗi
trong MỘT lần chạy, đúng trải nghiệm form THẬT cần.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn đã ghép được HAI trường bằng `map2GomLoi`. Nếu cần BA trường ĐỘC
LẬP cùng lúc (không lồng hai lần), có cách viết `map3GomLoi` trực
tiếp không?

Bài sau chốt cụm — tự mở rộng khuôn `map2` lên BA đối số.
::::

::::checkpoint{mastery=0.8}
::::
