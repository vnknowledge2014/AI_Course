---
id: co-so-du-lieu.dem-phieu.khi-cac-ban-sao-khong-dong-y
title: "Khi các bản sao không đồng ý"
summary: "banMoiNhat chọn bản ghi có thoiGian LỚN nhất trong một mảng BanGhi{giaTri,thoiGian}. So sánh dùng '>' NGHIÊM ngặt (không phải '>='), nên khi HAI bản ghi hoà thoiGian, kết quả LÀ bản gặp ĐẦU tiên trong mảng — đảo thứ tự mảng thì kết quả cũng đổi THEO. Đây LÀ hạ tầng cho read repair (bài 9): node CHUNG (đảm bảo tồn tại nhờ W+R>N, bài 5) có THỂ giữ bản CŨ hoặc MỚI tuỳ thời điểm nó nhận ghi — banMoiNhat LÀ quy tắc quyết định bản NÀO 'thắng'."
locale: vi
track: co-so-du-lieu
module: dem-phieu
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 9
teaches: [db.khi-cac-ban-sao-khong-dong-y]
requires: [db.w-cong-r-lon-hon-n]
concepts: [db.khi-cac-ban-sao-khong-dong-y]
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
`W+R>N` (bài trước) đảm bảo tập ghi VÀ tập đọc dùng CHUNG ít nhất một
node. NHƯNG node đó CÓ thể đã nhận NHIỀU lần ghi khác nhau CHO cùng
một khoá — bản NÀO là "đúng"?
::::

::::explain{#ban-moi-nhat}
`banMoiNhat` nhận một mảng `BanGhi { giaTri, thoiGian }`, trả VỀ bản
ghi CÓ `thoiGian` lớn NHẤT — "bản mới nhất luôn THẮNG" (last-write-
wins theo THỜI gian):

```typescript title=readonly
interface BanGhi { giaTri: number; thoiGian: number; }

function banMoiNhat(cacBanGhi: BanGhi[]): BanGhi {
  let ketQua = cacBanGhi[0]!;
  for (const bg of cacBanGhi) {
    if (bg.thoiGian > ketQua.thoiGian) ketQua = bg;
  }
  return ketQua;
}

console.log(banMoiNhat([{ giaTri: 10, thoiGian: 5 }, { giaTri: 20, thoiGian: 8 }, { giaTri: 15, thoiGian: 3 }]));
```

```text title=readonly
{ giaTri: 20, thoiGian: 8 }
```

`thoiGian: 8` LÀ lớn nhất trong BA bản ghi — `banMoiNhat` trả VỀ
đúng bản ĐÓ (`giaTri: 20`), bỏ QUA hai bản còn LẠI dù chúng ĐẾN
trước hay sau trong MẢNG.
::::

::::example{#hoa-thoi-gian}
So sánh dùng `>` NGHIÊM ngặt, KHÔNG phải `>=`. Nếu HAI bản ghi hoà
`thoiGian`, `ketQua` chỉ đổi KHI gặp một bản CÓ `thoiGian` LỚN hơn
STRICT — bản HOÀ đến SAU không thay THẾ được bản hoà đến TRƯỚC. Kết
quả LÀ: khi hoà, `banMoiNhat` giữ bản GẶP ĐẦU TIÊN trong mảng CÓ
`thoiGian` lớn nhất đó — đảo thứ TỰ mảng thì kết quả cũng đổi THEO,
vì "gặp đầu tiên" phụ THUỘC thứ tự.
::::

::::predict{#doan-hoa-doi-thu-tu commitOnce}
Gọi `banMoiNhat` VỚI mảng
`[{giaTri:15,thoiGian:8}, {giaTri:20,thoiGian:8}, {giaTri:10,thoiGian:5}]`
— HAI bản ĐẦU hoà `thoiGian=8`. Kết quả LÀ bản NÀO?

:::opt{correct}
`{giaTri:15, thoiGian:8}` — bản GẶP ĐẦU TIÊN trong mảng CÓ
`thoiGian` lớn nhất; so sánh `>` nghiêm ngặt KHÔNG cho bản thứ hai
(hoà, đến SAU) thay thế bản ĐẦU
:::

:::opt
`{giaTri:20, thoiGian:8}` — giữa hai bản HOÀ thời gian, `banMoiNhat`
ưu tiên `giaTri` LỚN hơn để phá thế hoà
::why
Trực giác NÀY hợp lý Ở NHIỀU hệ thống thật (dùng thêm MỘT tiêu chí
phụ, VÍ dụ ID node, để phá thế hoà một CÁCH tất định) — nhưng
`banMoiNhat` Ở bài NÀY không hề CÀI đặt quy tắc phụ đó.

Chỗ lệch: VÒNG lặp `for` chỉ so sánh `bg.thoiGian > ketQua.thoiGian`
— hoàn TOÀN không nhìn tới `giaTri`. Khi gặp bản THỨ hai
(`giaTri:20, thoiGian:8`), điều kiện LÀ `8 > 8` (SAI), nên `ketQua`
KHÔNG đổi, VẪN giữ bản đầu TIÊN (`giaTri:15`) đã gán TỪ trước đó.
::
:::
::::

::::code{#viet_ban_moi_nhat}
Hoàn thiện `banMoiNhat` — SO sánh `thoiGian`, giữ LẠI bản ghi có
`thoiGian` lớn HƠN.

```typescript title=starter
interface BanGhi { giaTri: number; thoiGian: number; }

function banMoiNhat(cacBanGhi: BanGhi[]): BanGhi {
  let ketQua = cacBanGhi[0]!;
  for (const bg of cacBanGhi) {
    ___
  }
  return ketQua;
}

console.log(banMoiNhat([{ giaTri: 10, thoiGian: 5 }, { giaTri: 20, thoiGian: 8 }, { giaTri: 15, thoiGian: 3 }]));
```

```typescript title=solution
interface BanGhi { giaTri: number; thoiGian: number; }

function banMoiNhat(cacBanGhi: BanGhi[]): BanGhi {
  let ketQua = cacBanGhi[0]!;
  for (const bg of cacBanGhi) {
    if (bg.thoiGian > ketQua.thoiGian) ketQua = bg;
  }
  return ketQua;
}

console.log(banMoiNhat([{ giaTri: 10, thoiGian: 5 }, { giaTri: 20, thoiGian: 8 }, { giaTri: 15, thoiGian: 3 }]));
```

```typescript title=test
const kq1 = banMoiNhat([{ giaTri: 10, thoiGian: 5 }, { giaTri: 20, thoiGian: 8 }, { giaTri: 15, thoiGian: 3 }]);
if (kq1.giaTri !== 20 || kq1.thoiGian !== 8) throw new Error("phai chon ban ghi co thoiGian lon nhat (20,8)");

const kqTie = banMoiNhat([{ giaTri: 10, thoiGian: 5 }, { giaTri: 20, thoiGian: 8 }, { giaTri: 15, thoiGian: 8 }]);
if (kqTie.giaTri !== 20) throw new Error("hoa thoiGian: phai giu ban ghi GAP DAU TIEN trong mang co thoiGian lon nhat do (20), khong phai ban sau (15)");

const kqDao = banMoiNhat([{ giaTri: 15, thoiGian: 8 }, { giaTri: 20, thoiGian: 8 }, { giaTri: 10, thoiGian: 5 }]);
if (kqDao.giaTri !== 15) throw new Error("doi thu tu mang thi ban GAP DAU TIEN co thoiGian max thay doi theo -- phai la 15 lan nay");

const kqMot = banMoiNhat([{ giaTri: 42, thoiGian: 1 }]);
if (kqMot.giaTri !== 42) throw new Error("mang mot phan tu phai tra ve chinh phan tu do");
```

:::hints
- kind: attention
  body: "So sanh thoiGian cua bg voi thoiGian cua ketQua, gan lai neu lon hon -- mot dong."
- kind: strategy
  body: "if (bg.thoiGian > ketQua.thoiGian) ketQua = bg;"
- kind: one-line
  body: "if (bg.thoiGian > ketQua.thoiGian) ketQua = bg;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "20"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Biết CÁCH chọn "bản thắng" giữa NHIỀU phiên bản — bây giờ: nếu MỘT
owner sập ngay lúc ĐANG ghi, làm sao ghi KHÔNG bị mất, mà KHÔNG cần
chờ owner đó sống LẠI?
::::

::::reflect{#nghi-lai}
`banMoiNhat` LÀ một hàm rất nhỏ — MỘT vòng lặp, một PHÉP so sánh —
nhưng NÓ chính LÀ quy tắc quyết định "đúng LÀ gì" mỗi khi các bản sao
BẤT đồng. `W+R>N` (bài 5) chỉ đảm bảo LUÔN có ít nhất một node CHUNG
Ở lần đọc — `banMoiNhat` LÀ bước tiếp THEO: khi node chung đó (VÀ
các owner khác được hỏi) trả VỀ NHIỀU bản khác nhau, đây LÀ cách
chọn RA một bản DUY nhất để trả lời.
::::

::::checkpoint{mastery=0.8}
::::
