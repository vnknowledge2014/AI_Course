---
id: lap-trinh-ham.cong-typescript.viet-mot-ham-co-chu-ky-day-du
title: "Viết một hàm có chữ ký đầy đủ"
summary: "Bài chốt cụm: viết một hàm có đủ kiểu tham số VÀ kiểu trả về, rồi tự gọi nó cả đúng lẫn sai để thấy TSC chặn ở đâu — không chỉ đọc lý thuyết, tự tạo ra lỗi và tự đọc thông báo."
locale: vi
track: lap-trinh-ham
module: cong-typescript
order: 10
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [ts.full-signature]
requires: [ts.arg-count-check]
concepts: [ts.full-signature]
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
Bốn bài, bốn mảnh: kiểu tham số, kiểu trả về, kiểu đối số lúc gọi, số
lượng đối số lúc gọi. Bài chốt cụm này không dạy mảnh nào mới — chỉ
đòi bạn TỰ VIẾT một hàm dùng đủ cả bốn.
::::

::::explain{#doc-chu-ky-day-du}
Gộp lại bốn bài vừa qua, một hàm TypeScript mang đúng HAI lời hứa:

1. Lời hứa về CÁI ĐI VÀO — kiểu của từng tham số (bài 6), và ĐÚNG SỐ
   LƯỢNG tham số (bài 9 áp cho phía gọi).
2. Lời hứa về CÁI ĐI RA — kiểu của giá trị `return` (bài 7).

Hai lời hứa này gộp lại gọi là CHỮ KÝ (signature) của hàm. Một hàm có
chữ ký đầy đủ trông như thế này — không thiếu dấu hai chấm nào:

```typescript
function tinhDoTuoi(namSinh: number): number {
  return 2026 - namSinh;
}
```

Và cả bốn mảnh đều bị kiểm TRƯỚC KHI CHẠY, không phải chỉ mảnh gần
dòng lỗi nhất — bài 3 đã chứng minh: một lỗi kiểu ở bất cứ đâu trong
file khiến CẢ FILE không được phép sinh mã, kể cả những dòng hoàn toàn
đúng đứng trước nó.
::::

::::example{#doc-vi-du-goi-dung-sai}
Byte gọi `tinhDoTuoi` hai lần — một lần đúng, một lần cố ý sai — để
XEM TypeScript chặn ở đâu, thay vì chỉ đọc lý thuyết:

```typescript
function tinhDoTuoi(namSinh: number): number {
  return 2026 - namSinh;
}

console.log(tinhDoTuoi(2000));
console.log(tinhDoTuoi("2000"));
```

```text title=readonly
(không in ra gì cả)

TS2345 (dòng 6, cột 24): Argument of type 'string' is not assignable
to parameter of type 'number'.
```

Đã chạy thật: dòng 5 (`tinhDoTuoi(2000)`) hoàn toàn đúng — nhưng vì
dòng 6 sai kiểu đối số, KHÔNG dòng nào được in ra, kể cả dòng 5. Đây
chính là bài 3 áp dụng lại, lần này ở cấp độ một lời gọi hàm thay vì
một phép gán biến: sai một chỗ, chặn cả file.
::::

::::predict{#chuoi-loai-loi-nao commitOnce}
Byte viết một hàm và hai lời gọi, KHÔNG chạy thử:

```typescript
function tinhTienDien(soKw: number, donGia: number): number {
  return soKw * donGia;
}

console.log(tinhTienDien(100, 3500));
console.log(tinhTienDien(100, "3500"));
```

Dòng cuối bị TypeScript từ chối. **Trước khi đọc đáp án**, đây là lỗi
LOẠI nào trong bốn mảnh vừa ôn — sai kiểu đối số, hay sai số lượng đối
số?

:::opt{correct}
Sai kiểu đối số (TS2345) — `tinhTienDien` nhận đủ hai đối số ở dòng
cuối, đúng số lượng. Nhưng `donGia` hứa kiểu `number`, còn `"3500"`
là một `string` (viết bằng nháy đơn) — kiểu không khớp, dù giá trị
"trông giống" một con số
:::

:::opt
Sai số lượng đối số (TS2554) — dòng cuối chỉ thật sự truyền được MỘT
đối số hợp lệ (`100`), vì `"3500"` không đúng kiểu nên TypeScript
không đếm nó vào tổng số đối số
::why
Gần đúng ở việc bạn nghi ngờ một đối số "không hợp lệ" có thể không
được tính.

Chỗ lệch: TypeScript đếm SỐ LƯỢNG đối số hoàn toàn độc lập với việc
từng đối số có đúng kiểu hay không — hai phép kiểm tách rời (bài 8 và
bài 9 dạy riêng biệt, không phải một luật gộp). Dòng cuối truyền đúng
hai đối số, nên TS2554 không có lý do xuất hiện. Đã thử thật: mã lỗi
xuất hiện là TS2345, không phải TS2554.
::
:::

:::opt
Sai kiểu đối số, nhưng vì hàm `tinhTienDien` bị gọi HAI LẦN liên tiếp
— gọi lại một hàm nhiều lần khiến TypeScript kiểm nghiêm khắc hơn ở
lần gọi sau
::why
Gần đúng ở việc bạn để ý `tinhTienDien` xuất hiện hai lần và nghi ngờ
điều đó có ảnh hưởng.

Chỗ lệch: số lần một hàm được gọi không làm thay đổi cách TypeScript
kiểm nó — mỗi lời gọi được kiểm ĐỘC LẬP, đúng lời hứa của riêng nó.
Dòng 5 (`tinhTienDien(100, 3500)`) hoàn toàn đúng dù đứng ngay trước
dòng sai. Đã thử thật: chỉ dòng 6 bị từ chối.
::
:::

:::opt
Sai kiểu đối số, vì `"3500"` viết bằng dấu nháy ĐƠN thay vì nháy KÉP —
TypeScript chỉ chấp nhận chuỗi viết bằng nháy kép `"..."` là `string`
hợp lệ
::why
Gần đúng ở việc bạn để ý loại dấu nháy bao quanh `3500`.

Chỗ lệch: TypeScript (và JavaScript) coi nháy đơn `'...'` và nháy kép
`"..."` là HAI CÁCH VIẾT tương đương của cùng một kiểu `string` — không
có sự phân biệt nào giữa chúng. Vấn đề không nằm ở loại dấu nháy, mà ở
việc `"3500"` — bất kể viết bằng nháy nào — vẫn LÀ một `string`, không
phải một `number`. Đã thử thật: đổi sang `"3500"` (nháy kép) lỗi vẫn y
hệt.
::
:::
::::

::::code{#tinh-chu-vi}
Chưa có hàm nào tên `tinhChuVi` cả — dòng gọi bên dưới đang gọi một
cái tên KHÔNG TỒN TẠI. Viết hàm đó, với chữ ký đầy đủ: nhận `dai` và
`rong` (đều là `number`), trả về `number` là chu vi hình chữ nhật
(`2 * (dai + rong)`).

```typescript title=starter
// Viết hàm tinhChuVi ở đây — nhận dai và rong (number), trả về
// number: chu vi hình chữ nhật = 2 * (dai + rong)


console.log(tinhChuVi(3, 4));
```

```typescript title=solution
function tinhChuVi(dai: number, rong: number): number {
  return 2 * (dai + rong);
}

console.log(tinhChuVi(3, 4));
```

```typescript title=test
if (tinhChuVi(3, 4) !== 14) throw new Error("tinhChuVi(3, 4) phải là 14 — đang là " + tinhChuVi(3, 4));
if (tinhChuVi(1, 1) !== 4) throw new Error("tinhChuVi(1, 1) phải là 4 — đang là " + tinhChuVi(1, 1));
```

:::hints
- kind: attention
  body: Dòng console.log gọi tinhChuVi(3, 4) — nhưng chưa có hàm nào tên đó. Đó là lý do chương trình hiện tại bị chặn với TS2304 (tên không tồn tại), không phải một lỗi kiểu bình thường.
- kind: strategy
  body: 'Viết một function khai đúng hai tham số kiểu number (dai, rong), kiểu trả về number, thân hàm tính 2 * (dai + rong).'
- kind: one-line
  body: 'Thêm: function tinhChuVi(dai: number, rong: number): number { return 2 * (dai + rong); }'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "14"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không phải chỉ đọc ví dụ của Byte nữa — bạn vừa tự viết một chữ ký đầy
đủ, và tự thấy TypeScript từ chối cái tên chưa tồn tại trước khi bạn
viết nó ra.
::::

::::reflect{#nghi-lai}
Tới giờ, mọi lời hứa kiểu bạn gặp đều nói về BIẾN đơn hoặc THAM SỐ/GIÁ
TRỊ TRẢ VỀ của hàm — những thứ chứa đúng MỘT giá trị. Nhưng chương
trình thật còn có MẢNG, chứa nhiều giá trị cùng lúc.

Nếu một mảng hứa "mọi phần tử đều là number" (`number[]`), thì ĐỌC một
phần tử cụ thể ra khỏi mảng đó — ví dụ phần tử đầu tiên — có LUÔN chắc
chắn nhận được một `number`, hay có trường hợp nào lời hứa ấy không
đủ? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
