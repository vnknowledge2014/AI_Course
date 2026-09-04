---
id: co-so-du-lieu.moi-loi-mot-vuong-quoc.cai-gia-cua-khoa
title: "Cái giá của khoá — mô phỏng tranh chấp"
summary: "tongThoiGianCho cộng dồn thời gian CHỜ của mọi yêu cầu xếp hàng trước một khoá dùng chung: yêu cầu thứ i (0-based) phải chờ i lần 'số tick mỗi yêu cầu' trước khi tới lượt mình. Với 4 yêu cầu, mỗi yêu cầu tốn 3 tick: tổng chờ là 18 tick — KHÔNG tính thời gian LÀM VIỆC của chính nó, chỉ tính lúc XẾP HÀNG. Gấp đôi số yêu cầu (4→8) không làm tổng thời gian chờ gấp đôi — nó tăng lên 84, gần gấp năm — vì thời gian chờ cộng dồn theo kiểu bậc hai (mỗi yêu cầu mới còn phải chờ MỌI yêu cầu trước nó)."
locale: vi
track: co-so-du-lieu
module: moi-loi-mot-vuong-quoc
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [db.cai-gia-cua-khoa]
requires: [db.nhieu-loi-mot-van-de]
concepts: [db.cai-gia-cua-khoa]
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
Khoá sửa được "lost update" (bài TRƯỚC) — bằng cách bắt lõi CHỜ nhau.
Cái GIÁ của việc chờ đó LỚN cỡ nào?
::::

::::explain{#tong-thoi-gian-cho}
CHỈ MỘT lõi giữ khoá tại MỘT thời điểm — mọi yêu cầu KHÁC phải xếp
HÀNG. MỖI lần giữ khoá tốn ĐÚNG `soTickMoiYeuCau` "tick" (đơn vị thời
gian mô phỏng, KHÔNG phải thời gian THẬT — quest NÀY đếm SỐ bước, không
đo đồng hồ). Yêu cầu THỨ `i` (đếm TỪ `0`) phải CHỜ đúng `i` lần
`soTickMoiYeuCau`, VÌ nó xếp SAU đúng `i` yêu cầu khác:

```typescript title=readonly
function tongThoiGianCho(soYeuCau: number, soTickMoiYeuCau: number): number {
  let tongCho = 0;
  for (let i = 0; i < soYeuCau; i++) {
    tongCho = tongCho + i * soTickMoiYeuCau;
  }
  return tongCho;
}

console.log(tongThoiGianCho(4, 3));
```

```text title=readonly
18
```

BỐN yêu cầu, MỖI yêu cầu giữ khoá `3` tick. Yêu cầu `0` KHÔNG chờ
GÌ (`0×3=0`). Yêu cầu `1` chờ ĐÚNG một yêu cầu trước NÓ (`1×3=3`). Yêu
cầu `2` chờ HAI (`2×3=6`). Yêu cầu `3` chờ BA (`3×3=9`). Tổng:
`0+3+6+9=18`.
::::

::::example{#gap-doi-khong-cho-ket-qua-gap-doi}
Gấp ĐÔI số yêu cầu (`4` lên `8`, CÙNG `3` tick MỖI yêu cầu):

```typescript title=readonly
console.log(tongThoiGianCho(8, 3));
```

```text title=readonly
84
```

`18` LÊN `84` — KHÔNG phải gấp ĐÔI (`36`), mà GẦN gấp NĂM. Lý do: yêu
cầu THỨ `7` (CUỐI cùng trong TÁM) phải chờ đủ BẢY yêu cầu trước NÓ,
nhiều HƠN hẳn yêu cầu thứ `3` (CUỐI cùng trong BỐN) chỉ chờ BA. Tổng
thời gian CHỜ tăng THEO kiểu bậc hai (tỉ lệ VỚI `soYeuCau²`), không
phải tuyến TÍNH — CÀNG nhiều lõi tranh MỘT khoá, cái GIÁ của tranh
chấp CÀNG tăng nhanh hơn hẳn số lõi.
::::

::::predict{#doan-mot-yeu-cau commitOnce}
CHỈ CÓ đúng MỘT yêu cầu (`soYeuCau = 1`), bất kể `soTickMoiYeuCau` LÀ
bao nhiêu. `tongThoiGianCho(1, 100)` trả VỀ gì?

:::opt{correct}
`0` — MỘT yêu cầu DUY nhất không CÓ yêu cầu nào TRƯỚC nó để phải CHỜ
:::

:::opt
`100` — yêu cầu ĐÓ vẫn phải giữ khoá TRONG `100` tick, nên "thời gian
liên QUAN tới khoá" của NÓ là `100`
::why
Gần đúng ở việc bạn tính ĐÚNG thời gian LÀM VIỆC (giữ khoá) CỦA chính
yêu cầu đó LÀ `100` tick — MỘT phép tính CHÍNH xác nếu câu hỏi LÀ "nó
tốn bao lâu".

Chỗ lệch: `tongThoiGianCho` đo THỜI gian CHỜ (xếp hàng TRƯỚC khi tới
lượt), KHÔNG đo thời gian LÀM việc của CHÍNH yêu cầu đó — vòng `for`
chỉ chạy đúng MỘT lần (`i=0`), VÀ số hạng cộng VÀO LÀ `0 ×
soTickMoiYeuCau = 0`. Yêu cầu duy NHẤT không có AI xếp trước NÓ, nên
thời gian CHỜ của nó LUÔN LÀ `0`, bất kể `soTickMoiYeuCau` lớn cỡ NÀO.
::
:::
::::

::::code{#viet_tong_thoi_gian_cho}
Hoàn thiện `tongThoiGianCho` — CỘNG dồn thời gian chờ CỦA yêu cầu thứ
`i` (LÀ `i` lần `soTickMoiYeuCau`) VÀO `tongCho`.

```typescript title=starter
function tongThoiGianCho(soYeuCau: number, soTickMoiYeuCau: number): number {
  let tongCho = 0;
  for (let i = 0; i < soYeuCau; i++) {
    ___
  }
  return tongCho;
}

console.log(tongThoiGianCho(4, 3));
```

```typescript title=solution
function tongThoiGianCho(soYeuCau: number, soTickMoiYeuCau: number): number {
  let tongCho = 0;
  for (let i = 0; i < soYeuCau; i++) {
    tongCho = tongCho + i * soTickMoiYeuCau;
  }
  return tongCho;
}

console.log(tongThoiGianCho(4, 3));
```

```typescript title=test
console.log(tongThoiGianCho(4, 3));
if (tongThoiGianCho(4, 3) !== 18) throw new Error("4 yeu cau, 3 tick moi yeu cau phai cho tong 18");
if (tongThoiGianCho(1, 3) !== 0) throw new Error("mot yeu cau duy nhat khong cho gi ca, phai la 0");
if (tongThoiGianCho(0, 3) !== 0) throw new Error("khong co yeu cau nao thi tong cho phai la 0");
if (tongThoiGianCho(8, 3) !== 84) throw new Error("8 yeu cau, 3 tick moi yeu cau phai cho tong 84 (khong phai gap doi 18)");
```

:::hints
- kind: attention
  body: "Cong don vao tongCho: i nhan soTickMoiYeuCau -- mot dong."
- kind: strategy
  body: "tongCho = tongCho + i * soTickMoiYeuCau;"
- kind: one-line
  body: "tongCho = tongCho + i * soTickMoiYeuCau;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "18"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Khoá CÀNG bị tranh nhiều, GIÁ càng tăng nhanh HƠN số lõi. Cách tránh
hẳn việc tranh CHẤP — không chia SẺ gì cả — trông ra sao?
::::

::::reflect{#nghi-lai}
Tổng thời gian CHỜ tăng THEO `soYeuCau²` (bậc hai) LÀ lý do cốt LÕI
khiến "MỌI lõi cùng tranh MỘT khoá" trở thành một chiến lược TỆ khi hệ
thống scale LÊN nhiều lõi: gấp đôi số lõi KHÔNG gấp đôi chi PHÍ tranh
chấp, nó gần gấp NĂM. ScyllaDB (VÀ framework Seastar nó DÙNG) né hẳn
vấn đề NÀY bằng một chiến lược KHÁC hoàn toàn: đừng chia sẻ GÌ cả.
::::

::::checkpoint{mastery=0.8}
::::
</content>
