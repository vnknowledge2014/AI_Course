---
id: co-so-du-lieu.moi-loi-mot-vuong-quoc.share-nothing-chia-theo-loi
title: "Share-nothing — chia dữ liệu theo lõi"
summary: "taoKhoChoMoiLoi(soLoi) tạo MỘT Map ĐỘC LẬP cho mỗi lõi (thay vì một Map DUY NHẤT dùng chung, bài 1-2). Hai lõi cùng tăng 'x' trên KHO RIÊNG của mình — dù đọc/ghi xen kẽ y hệt kịch bản 'lost update' — không hề mất cập nhật nào, vì không lõi nào chạm bộ nhớ của lõi khác. Đây là 'share-nothing': loại bỏ hẳn tranh chấp bằng cách không chia sẻ gì, thay vì kiểm soát chia sẻ bằng khoá."
locale: vi
track: co-so-du-lieu
module: moi-loi-mot-vuong-quoc
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [db.share-nothing-chia-theo-loi]
requires: [db.cai-gia-cua-khoa]
concepts: [db.share-nothing-chia-theo-loi]
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
Khoá SỬA "lost update" NHƯNG tốn giá bậc HAI (bài 2). Có cách NÀO
tránh HẲN vấn đề, không CẦN khoá gì cả?
::::

::::explain{#kho-rieng-moi-loi}
`taoKhoChoMoiLoi` tạo MỘT `Map` riêng CHO mỗi lõi — KHÔNG phải một
`Map` DUY nhất dùng chung (bài 1-2):

```typescript title=readonly
function taoKhoChoMoiLoi(soLoi: number): Map<string, number>[] {
  const khoCacLoi: Map<string, number>[] = [];
  for (let i = 0; i < soLoi; i++) {
    khoCacLoi.push(new Map());
  }
  return khoCacLoi;
}

const khoCacLoi = taoKhoChoMoiLoi(2);
console.log(khoCacLoi.length);
```

```text title=readonly
2
```

MỖI lần lặp gọi `new Map()` — tạo một Đối tượng `Map` HOÀN toàn MỚI,
KHÔNG liên quan gì Đến các `Map` ĐÃ tạo trước ĐÓ. Mảng trả VỀ CÓ đúng
`soLoi` phần tử, MỖI phần tử LÀ một kho ĐỘC lập.
::::

::::example{#khong-con-lost-update}
CHẠY LẠI kịch bản "lost update" (bài 1) — NHƯNG lần NÀY mỗi lõi tăng
`"x"` TRÊN kho RIÊNG của chính nó, đọc/ghi ĐAN xen y HỆT trước:

```typescript title=readonly
khoCacLoi[0]!.set("x", 0);
khoCacLoi[1]!.set("x", 0);

const doc0 = khoCacLoi[0]!.get("x")!;
const doc1 = khoCacLoi[1]!.get("x")!;
khoCacLoi[0]!.set("x", doc0 + 1);
khoCacLoi[1]!.set("x", doc1 + 1);

console.log(khoCacLoi[0]!.get("x"));
console.log(khoCacLoi[1]!.get("x"));
```

```text title=readonly
1
1
```

CẢ hai đều LÀ `1` — ĐÚNG, KHÔNG mất cập nhật NÀO, dù thứ tự ĐỌC/ghi
đan XEN CHÍNH xác như kịch bản GÂY lỗi Ở bài 1. Lõi `0` không hề CHẠM
tới `khoCacLoi[1]`, lõi `1` không hề chạm TỚI `khoCacLoi[0]` — không
CÓ gì để "tranh CHẤP" cả, nên thứ tự XEN kẽ không còn quan TRỌNG.
::::

::::predict{#doan-nham-dung-chung-mot-kho commitOnce}
Một phiên bản `taoKhoChoMoiLoi` viết SAI: tạo `const mDuyNhat = new
Map();` MỘT lần Ở NGOÀI vòng lặp, RỒI `push(mDuyNhat)` LẶP LẠI (CÙNG
MỘT đối tượng, không phải đối tượng MỚI). Mảng trả VỀ vẫn CÓ đúng
`soLoi` phần tử. Lõi `0` gọi `khoSai[0].set("x", 5)`. Lõi `1` gọi
`khoSai[1].get("x")` — kết QUẢ LÀ gì?

:::opt{correct}
`5` — VÌ `khoSai[0]` VÀ `khoSai[1]` thực ra LÀ CÙNG một đối tượng
`Map`, ghi VÀO một bên cũng LÀ ghi VÀO bên kia
:::

:::opt
`undefined` — MỖI CHỈ số mảng vẫn LÀ một "lõi" riêng biệt VỀ mặt LOGIC,
nên dữ liệu của lõi `0` không hề LỘ sang lõi `1`
::why
Gần đúng ở việc bạn nghĩ TỚI "chỉ số mảng KHÁC nhau" NHƯ một ranh
giới cách LY — ĐÚNG khi mỗi Ô mảng trỏ TỚI một `Map` THẬT sự khác
nhau (như `taoKhoChoMoiLoi` ĐÚNG Ở bài NÀY).

Chỗ lệch: `khoSai[0]` VÀ `khoSai[1]` LÀ hai chỉ SỐ mảng khác nhau,
NHƯNG cả hai đều trỏ TỚI cùng MỘT đối tượng `Map` (`mDuyNhat`) — độ
DÀI mảng đúng KHÔNG hề chứng minh các phần tử ĐỘC lập với nhau. Ghi
VÀO `khoSai[0]` chính LÀ ghi vào `mDuyNhat`, VÀ đọc TỪ `khoSai[1]`
cũng LÀ đọc từ CHÍNH `mDuyNhat` ĐÓ — kết quả LÀ `5`, y hệt vấn đề
"chia sẻ" mà share-nothing MUỐN tránh, chỉ LÀ nguỵ trang dưới một
mảng CÓ độ dài đúng.
::
:::
::::

::::code{#viet_tao_kho_cho_moi_loi}
Hoàn thiện `taoKhoChoMoiLoi` — MỖI lần lặp, tạo một `Map` MỚI VÀ đẩy
VÀO mảng.

```typescript title=starter
function taoKhoChoMoiLoi(soLoi: number): Map<string, number>[] {
  const khoCacLoi: Map<string, number>[] = [];
  for (let i = 0; i < soLoi; i++) {
    ___
  }
  return khoCacLoi;
}

const khoCacLoi = taoKhoChoMoiLoi(2);
console.log(khoCacLoi.length);
```

```typescript title=solution
function taoKhoChoMoiLoi(soLoi: number): Map<string, number>[] {
  const khoCacLoi: Map<string, number>[] = [];
  for (let i = 0; i < soLoi; i++) {
    khoCacLoi.push(new Map());
  }
  return khoCacLoi;
}

const khoCacLoi = taoKhoChoMoiLoi(2);
console.log(khoCacLoi.length);
```

```typescript title=test
const kho2 = taoKhoChoMoiLoi(3);
console.log(kho2.length);
if (kho2.length !== 3) throw new Error("taoKhoChoMoiLoi(3) phai tao dung 3 kho");

kho2[0]!.set("x", 5);
if (kho2[1]!.get("x") !== undefined) throw new Error("kho cua loi 1 phai DOC LAP voi kho cua loi 0 -- khong duoc thay gia tri loi 0 vua ghi");
if (kho2[2]!.get("x") !== undefined) throw new Error("kho cua loi 2 cung phai doc lap");

const khoRong = taoKhoChoMoiLoi(0);
if (khoRong.length !== 0) throw new Error("0 loi thi mang kho phai rong");
```

:::hints
- kind: attention
  body: "Moi lan lap, tao MOT Map moi (new Map()) roi push vao khoCacLoi -- mot dong."
- kind: strategy
  body: "khoCacLoi.push(new Map());"
- kind: one-line
  body: "khoCacLoi.push(new Map());"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "2"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không CÒN tranh chấp — mỗi lõi CÓ kho RIÊNG. Nhưng "kho NÀO chứa khoá
NÀO" phải được QUYẾT định trước — bằng CÁCH nào?
::::

::::reflect{#nghi-lai}
"Share-nothing" KHÔNG phải một kỹ thuật ĐỒNG bộ hoá tinh VI hơn khoá
— nó LÀ việc TỪ chối chia sẻ NGAY từ đầu. Không CÓ tài nguyên chung
nghĩa LÀ không CÓ gì để tranh CHẤP, VÀ không có gì để tranh chấp
nghĩa LÀ không CẦN khoá — chi phí bậc HAI của bài 2 biến MẤT hoàn
TOÀN, không phải giảm ĐI. Nhưng chia dữ liệu RA nhiều kho ĐỘC lập chỉ
GIẢI quyết được nửa BÀI toán: một hệ THẬT cần biết CHÍNH xác "khoá
NÀY thuộc kho NÀO" — quyết định ĐÓ dựa VÀO điều gì?
::::

::::checkpoint{mastery=0.8}
::::
</content>
