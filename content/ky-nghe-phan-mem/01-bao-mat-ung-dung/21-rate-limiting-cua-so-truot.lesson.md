---
id: ky-nghe-phan-mem.bao-mat-ung-dung.rate-limiting-cua-so-truot
title: "Rate Limiting — cửa sổ trượt đếm request theo THỜI GIAN"
summary: "Sliding window: lưu timestamp của MỖI request theo khoá (IP), lọc bỏ những dấu CŨ HƠN cửa sổ, so sánh số lượng còn lại với ngưỡng. Khác đếm THEO PHÚT CỐ ĐỊNH (dễ burst ngay lúc reset) — cửa sổ trượt liên tục theo timestamp thực, không có điểm reset đột ngột."
locale: vi
track: ky-nghe-phan-mem
module: bao-mat-ung-dung
order: 21
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [bmud.rate-limit-sliding-window]
requires: [bmud.vulnerability-spotter]
concepts: [bmud.rate-limit-sliding-window]
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
Ai đó gửi HÀNG NGHÌN request MỘT GIÂY tới endpoint đăng nhập, thử mọi
mật khẩu. Làm sao GIỚI HẠN tốc độ request từ MỘT nguồn?
::::

::::explain{#cua-so-truot}
**Sliding window** (cửa sổ trượt): LƯU timestamp của MỖI request
theo **KHOÁ** (thường LÀ IP), **LỌC BỎ** những dấu CŨ HƠN cửa sổ thời
gian, rồi **SO SÁNH** số lượng CÒN LẠI với NGƯỠNG tối đa:

```typescript
function locTrongCuaSo(dauThoiGian: number[], gioHienTai: number, doDaiCuaSoMs: number): number[] {
  return dauThoiGian.filter((t) => gioHienTai - t < doDaiCuaSoMs);
}

// Bốn request lúc 0, 1000, 2000, 3000 (mili giây) -- cửa sổ 2500ms, giờ hiện tại 3100
const cacDau = [0, 1000, 2000, 3000];
const conTrongCuaSo = locTrongCuaSo(cacDau, 3100, 2500);
console.log(conTrongCuaSo);
console.log(conTrongCuaSo.length);
```

```text
[1000,2000,3000]
3
```

Request lúc `0` bị LOẠI (`3100 - 0 = 3100`, KHÔNG nhỏ hơn `2500`) —
QUÁ CŨ, đã "trượt RA KHỎI" cửa sổ. BA request CÒN LẠI (`1000`, `2000`,
`3000`) VẪN "trong cửa sổ" (hiệu số VỚI `3100` đều NHỎ HƠN `2500`).
`locTrongCuaSo` CHÍNH LÀ bước LỌC — MỖI lần có request MỚI, gọi LẠI
hàm này để BIẾT còn BAO NHIÊU request "gần đây".
::::

::::example{#khac-dem-theo-phut-co-dinh}
Khác đếm **THEO PHÚT CỐ ĐỊNH** (reset đột ngột mỗi phút — DỄ bị
"burst" NGAY LÚC reset): cửa sổ TRƯỢT LIÊN TỤC theo timestamp THỰC,
**KHÔNG** có điểm reset đột ngột nào để khai thác:

```typescript title=readonly
function locTrongCuaSo(dauThoiGian: number[], gioHienTai: number, doDaiCuaSoMs: number): number[] {
  return dauThoiGian.filter((t) => gioHienTai - t < doDaiCuaSoMs);
}

// Đếm THEO PHÚT CỐ ĐỊNH: 100 request LÚC 0:59, RESET về 0 lúc 1:00, THÊM 100 request LÚC 1:01
// -- 200 request trong VÒNG 2 GIÂY (0:59 tới 1:01), NHƯNG "hai phút riêng biệt" đều HỢP LỆ theo bộ đếm cố định

// CỬA SỔ TRƯỢT: 200 request TRONG 2 GIÂY luôn bị PHÁT HIỆN, bất kể request rơi vào "phút" nào
const daiCuaSoTruot = locTrongCuaSo(
  Array.from({ length: 200 }, (_, i) => 59000 + i * 10), // 200 request rải trong ~2 giây (59.0s -> 61.0s)
  61000,
  60000, // cửa sổ 60 giây
);
console.log(daiCuaSoTruot.length);
```

```text title=readonly
200
```

TOÀN BỘ `200` request (rải TRONG khoảng `59.0s` tới `61.0s`, tức
KHOẢNG 2 GIÂY) VẪN nằm TRONG cửa sổ `60` GIÂY tính TỪ `61000` — cửa
sổ trượt PHÁT HIỆN được TOÀN BỘ, KHÔNG bị "chia cắt" theo ranh giới
PHÚT như bộ đếm CỐ ĐỊNH (nơi 100 request TRƯỚC `1:00` và 100 request
SAU `1:00` có thể bị tính là HAI "phút" riêng biệt, MỖI phút "hợp lệ"
riêng dù TỔNG cộng vượt xa ngưỡng).
::::

::::predict{#doan-bien-cua-so commitOnce}
```typescript
function locTrongCuaSo(dauThoiGian: number[], gioHienTai: number, doDaiCuaSoMs: number): number[] {
  return dauThoiGian.filter((t) => gioHienTai - t < doDaiCuaSoMs);
}

// Request ĐÚNG NGƯỠNG (hiệu số = độ dài cửa sổ)
const ketQua = locTrongCuaSo([1000], 3500, 2500);
console.log(ketQua.length);
```

Dòng cuối in ra gì (`3500 - 1000 = 2500`, ĐÚNG BẰNG độ dài cửa sổ)?

:::opt{correct}
`0`
:::

:::opt
`1` — vì request đúng Ở RANH GIỚI cửa sổ (hiệu số ĐÚNG BẰNG độ dài
cửa sổ) vẫn được TÍNH LÀ "còn trong cửa sổ", giống như biên GIỮ ĐƯỢC
trong nhiều bài toán biên đã gặp trước đây
::why
Gần đúng ở việc bạn liên tưởng tới các bài toán BIÊN đã gặp TRƯỚC ĐÂY
trong track (nhiều nơi biên "giữ được") — một liên tưởng HỢP LÝ, vì
quy tắc CỤ THỂ ở TỪNG nơi có thể khác nhau.

Chỗ lệch: điều kiện `filter` Ở ĐÂY LÀ `gioHienTai - t < doDaiCuaSoMs`
— dùng `<` (**NHỎ HƠN NGHIÊM NGẶT**), KHÔNG PHẢI `<=`. Với `t = 1000`,
`gioHienTai = 3500`, `doDaiCuaSoMs = 2500`: `3500 - 1000 = 2500`,
`2500 < 2500` là **`false`** — request BỊ LOẠI (KHÔNG còn "trong cửa
sổ"). Đây LÀ quy ước CỤ THỂ của `locTrongCuaSo`: request ĐÚNG NGƯỠNG
(hiệu số BẰNG độ dài cửa sổ) được coi LÀ "vừa TRƯỢT RA KHỎI cửa sổ",
KHÔNG PHẢI "vừa còn trong". Mảng KẾT QUẢ RỖNG, `length` LÀ `0`.
::
:::

:::opt
Máy báo lỗi biên dịch — `locTrongCuaSo([1000], 3500, 2500)` không hợp
lệ vì mảng đầu vào chỉ có MỘT phần tử, hàm yêu cầu ÍT NHẤT hai
timestamp để so sánh "cửa sổ trượt"
::why
Gần đúng ở việc bạn nghĩ tới việc "cửa sổ trượt" NGHE như CẦN nhiều
điểm dữ liệu để có Ý NGHĨA SO SÁNH — một trực giác dễ hiểu về mặt
KHÁI NIỆM.

Chỗ lệch: `locTrongCuaSo` nhận `dauThoiGian: number[]` — MỘT mảng
BÌNH THƯỜNG, KHÔNG có ràng buộc "PHẢI có ít nhất N phần tử" nào Ở
TẦNG KIỂU (`number[]` chấp nhận mảng RỖNG, MỘT phần tử, hay BAO NHIÊU
tuỳ ý). Hàm hoạt động HOÀN TOÀN bình thường VỚI một mảng CHỈ MỘT phần
tử — `.filter` chạy TRÊN nó như MỌI mảng khác. Biên dịch sạch.
::
:::
::::

::::code{#viet_loctrongcuaso}
Tự viết `locTrongCuaSo`.

```typescript title=starter
function locTrongCuaSo(dauThoiGian: number[], gioHienTai: number, doDaiCuaSoMs: number): number[] {
  return dauThoiGian.filter((t) => ___);
}

console.log(locTrongCuaSo([0, 1000, 2000, 3000], 3100, 2500));
```

```typescript title=solution
function locTrongCuaSo(dauThoiGian: number[], gioHienTai: number, doDaiCuaSoMs: number): number[] {
  return dauThoiGian.filter((t) => gioHienTai - t < doDaiCuaSoMs);
}

console.log(locTrongCuaSo([0, 1000, 2000, 3000], 3100, 2500));
```

```typescript title=test
if (JSON.stringify(locTrongCuaSo([0, 1000, 2000, 3000], 3100, 2500)) !== JSON.stringify([1000, 2000, 3000])) throw new Error("phải loại request quá cũ (0), giữ ba request còn lại");
if (JSON.stringify(locTrongCuaSo([], 1000, 500)) !== JSON.stringify([])) throw new Error("mảng rỗng phải ra mảng rỗng");

// Biên: ĐÚNG ngưỡng bị loại, ngưỡng-1 (mới hơn 1ms) được giữ
if (locTrongCuaSo([1000], 3500, 2500).length !== 0) throw new Error("hiệu số ĐÚNG BẰNG độ dài cửa sổ phải bị loại (dùng <, không phải <=)");
if (locTrongCuaSo([1001], 3500, 2500).length !== 1) throw new Error("hiệu số NHỎ HƠN độ dài cửa sổ dù chỉ 1ms phải được giữ");
```

:::hints
- kind: attention
  body: "Điều kiện giữ lại: hiệu số giữa gioHienTai và t (timestamp) phải NHỎ HƠN doDaiCuaSoMs — dùng < nghiêm ngặt, không phải <=."
- kind: strategy
  body: "gioHienTai - t < doDaiCuaSoMs — một biểu thức boolean duy nhất."
- kind: one-line
  body: "___ = gioHienTai - t < doDaiCuaSoMs"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "1000"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cửa sổ trượt: lọc timestamp cũ theo thời gian THỰC, không có ranh
giới đột ngột để khai thác. Bước tiếp theo: cài đặt bộ giới hạn HOÀN
CHỈNH.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`locTrongCuaSo` CHỈ lọc — chưa quyết định "cho phép hay từ chối".
Ghép nó với `Map` (lưu timestamp theo IP, trong closure) thành một bộ
giới hạn HOÀN CHỈNH trông thế nào?
::::

::::checkpoint{mastery=0.8}
::::
