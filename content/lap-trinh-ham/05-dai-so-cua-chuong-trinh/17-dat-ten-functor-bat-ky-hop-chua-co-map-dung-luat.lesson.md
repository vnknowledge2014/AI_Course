---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.dat-ten-functor-bat-ky-hop-chua-co-map-dung-luat
title: "Đặt tên: `Functor` — bất kỳ 'hộp chứa' có `map` đúng luật"
summary: "Option, Result, Array — ba kiểu khác hẳn nhau, CÙNG có một map cùng hình dạng. Bất kỳ kiểu nào có map như vậy được gọi là một Functor — không phải một kiểu dữ liệu CỤ THỂ, một TÍNH CHẤT nhiều kiểu cùng có."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 17
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [alg.functor-definition]
requires: [alg.array-map-same-pattern]
concepts: [alg.functor-definition]
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
`Option`, `Result`, `Array` — ba `map` cùng khuôn. Hôm nay đặt tên
chính thức cho khuôn đó.
::::

::::explain{#dinh-nghia-functor}
Track này gọi khuôn "có `map` đúng luật, biến đổi bên trong, giữ
nguyên vỏ" là một **Functor**. Không phải một KIỂU DỮ LIỆU cụ thể — là
một TÍNH CHẤT (giống `Monoid` ở cụm 1: không phải MỘT thứ, mà một
HÌNH DẠNG nhiều thứ khác nhau cùng có).

Một kiểu `F<T>` là Functor nếu nó có một hàm `map` với ĐÚNG hình dạng:

```typescript
// map: (F<T>, hàm biến T thành U) => F<U>
```

`Option<T>` là Functor (vì có `mapOption`). `Result<T, E>` là Functor
(vì có `mapResult`). `Array<T>` (hay `T[]`) là Functor (vì có
`.map()` có sẵn). Ba kiểu HOÀN TOÀN khác nhau về Ý NGHĨA (một giá trị
có/không, một phép tính thành công/lỗi, một danh sách) — nhưng CÙNG
là Functor, vì CÙNG có `map` đúng hình dạng đó.

Không phải MỌI kiểu dữ liệu đều là Functor. `number` (một số đơn) —
KHÔNG là Functor, vì không có khái niệm "bên trong" để `map` vào (một
số không "chứa" gì cả, nó CHÍNH LÀ giá trị). `interface NguoiDung {
ten: string; tuoi: number }` (T4.3's product type) — KHÔNG tự động là
Functor, trừ khi bạn TỰ viết một `mapNguoiDung` hợp lý (mà "biến đổi"
MỘT trong hai field khác nhau thì không rõ nghĩa "map" chung nào).
::::

::::example{#tu-hoi-co-phai-functor-khong}
Kiểm tra một kiểu MỚI có phải Functor không — hỏi ĐÚNG câu hỏi: "có
định nghĩa được MỘT hàm `map` áp dụng lên GIÁ TRỊ BÊN TRONG, giữ vỏ
không?"

```typescript title=readonly
type Hop<T> = { giaTri: T };

function mapHop<T, U>(h: Hop<T>, f: (x: T) => U): Hop<U> {
  return { giaTri: f(h.giaTri) };
}

console.log(mapHop({ giaTri: 5 }, (x) => x * 2));
console.log(mapHop({ giaTri: "chao" }, (s) => s.length));
```

```text title=readonly
{"giaTri":10}
{"giaTri":4}
```

`Hop<T>` (một "hộp" chứa ĐÚNG MỘT giá trị, không có biến thể "rỗng"
như `Option`) VẪN là một Functor — CÓ `mapHop`, đúng hình dạng: áp
dụng `f` lên `giaTri` bên trong, giữ nguyên "vỏ" `Hop<...>`. `Functor`
không đòi kiểu dữ liệu phải "có thể rỗng" hay "có thể lỗi" — chỉ đòi
CÓ khái niệm "bên trong" để `map` vào.
::::

::::predict{#doan-la-functor-khong commitOnce}
Cho kiểu sau — nó CÓ PHẢI một Functor hợp lệ không?

```typescript
type CapDoi<T> = { trai: T; phai: T };

function mapCapDoi<T, U>(c: CapDoi<T>, f: (x: T) => U): CapDoi<U> {
  return { trai: f(c.trai), phai: f(c.phai) };
}

console.log(mapCapDoi({ trai: 3, phai: 7 }, (x) => x * 2));
```

:::opt{correct}
Có — `mapCapDoi` đúng hình dạng: áp dụng `f` lên CẢ hai giá trị bên
trong, giữ nguyên "vỏ" `CapDoi<...>`
:::

:::opt
Không — Functor CHỈ áp dụng cho kiểu chứa ĐÚNG MỘT giá trị (như
`Option`, `Hop`), `CapDoi` chứa HAI giá trị (`trai` và `phai`) nên
không hợp lệ
::why
Gần đúng ở việc bạn để ý `CapDoi` chứa NHIỀU hơn một giá trị (`trai`
VÀ `phai`) — quan sát về CẤU TRÚC đó đúng.

Chỗ lệch: KHÔNG có giới hạn "Functor chỉ chứa đúng một giá trị" —
`Array<T>` (đã học ở bài 16) CHỨA NHIỀU giá trị (không chỉ một, không
chỉ hai — bất kỳ số lượng nào), VẪN là Functor hợp lệ. `CapDoi<T>`
CHỨA HAI giá trị, `mapCapDoi` áp dụng `f` lên CẢ HAI, giữ nguyên hình
dạng — đúng hình dạng Functor, hợp lệ.
::
:::

:::opt
Không — Functor phải có biến thể "rỗng"/"lỗi" giống `Option`/`Result`,
`CapDoi` không có biến thể nào như vậy
::why
Gần đúng ở việc bạn nhớ `Option`/`Result` (hai ví dụ ĐẦU TIÊN track đã
dạy) CÓ biến thể "rỗng"/"lỗi" — quan sát về HAI ví dụ đó đúng.

Chỗ lệch: `Array<T>` (bài 16) KHÔNG có biến thể "rỗng"/"lỗi" theo
nghĩa discriminated union — một mảng RỖNG vẫn LÀ một mảng, không phải
MỘT BIẾN THỂ khác của Functor. `CapDoi<T>` cũng KHÔNG cần biến thể nào
như vậy — chỉ cần có `map` đúng hình dạng là đủ.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Functor không phải một kiểu dữ liệu — nó là một CÂU HỎI có thể hỏi VỀ
bất kỳ kiểu dữ liệu nào: "kiểu này có `map` đúng hình dạng không?"
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Có `map` đúng HÌNH DẠNG (nhận vào, trả ra, giữ vỏ) là chưa đủ — một
hàm `map` VIẾT SAI (có tác dụng phụ lạ, đổi giá trị theo cách vô lý)
vẫn có thể khớp ĐÚNG hình dạng chữ ký đó. Có cách nào kiểm `map` hoạt
động ĐÚNG, không chỉ đúng kiểu?

Bài sau trả lời bằng LUẬT đầu tiên.
::::

::::checkpoint{mastery=0.8}
::::
