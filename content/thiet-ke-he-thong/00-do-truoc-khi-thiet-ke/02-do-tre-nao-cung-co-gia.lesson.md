---
id: thiet-ke-he-thong.do-truoc-khi-thiet-ke.do-tre-nao-cung-co-gia
title: "Độ trễ nào cũng có giá"
summary: "DO_TRE_NS gom bảy con số độ trễ 2020 (L1=0.5ns, L2=7ns, RAM=100ns, SSD đọc ngẫu nhiên=150.000ns, HDD seek ngẫu nhiên=10.000.000ns, mạng cùng datacenter=500.000ns, mạng khác vùng=150.000.000ns) trên CÙNG một đơn vị (ns) để so được trực tiếp. soLanCham(nhanh, cham) tính tỉ lệ: mạng cùng DC nhanh hơn HDD seek đúng 20 lần, mạng khác vùng chậm hơn RAM đúng 1.500.000 lần -- những con số này không cần nhớ, chỉ cần tính đúng thứ tự độ lớn."
locale: vi
track: thiet-ke-he-thong
module: do-truoc-khi-thiet-ke
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 9
teaches: [sd.do-tre-nao-cung-co-gia]
requires: [sd.don-vi-cua-quy-mo]
concepts: [sd.do-tre-nao-cung-co-gia]
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
Dung lượng có đơn vị RỒI (bài trước). Nhưng "chứa được bao nhiêu"
không nói GÌ về "lấy RA nhanh cỡ nào" — VÀ mỗi tầng lưu trữ (cache,
RAM, đĩa, mạng) có một cái GIÁ độ trễ hoàn toàn khác nhau.
::::

::::explain{#bang-do-tre}
Bảy con số độ trễ (2020) — MỘT thao tác đơn giản Ở mỗi tầng, quy hết
VỀ chung một đơn vị (nano-giây, `ns`) để so trực TIẾP:

```typescript title=readonly
const DO_TRE_NS: Record<string, number> = {
  L1: 0.5,
  L2: 7,
  RAM: 100,
  SSD_DOC_NGAU_NHIEN: 150_000,
  HDD_SEEK_NGAU_NHIEN: 10_000_000,
  MANG_CUNG_DATACENTER: 500_000,
  MANG_KHAC_VUNG: 150_000_000,
};

function soLanCham(tenNhanh: string, tenCham: string): number {
  return DO_TRE_NS[tenCham]! / DO_TRE_NS[tenNhanh]!;
}

console.log("HDD seek / mang cung DC:", soLanCham('MANG_CUNG_DATACENTER', 'HDD_SEEK_NGAU_NHIEN'));
console.log("mang khac vung / RAM:", soLanCham('RAM', 'MANG_KHAC_VUNG'));
console.log("L2 / L1:", soLanCham('L1', 'L2'));
```

```text title=readonly
HDD seek / mang cung DC: 20
mang khac vung / RAM: 1500000
L2 / L1: 14
```

`soLanCham(nhanh, cham)` LUÔN trả về một số `>= 1` — "thao tác CHẬM
mất gấp bao nhiêu lần thao tác NHANH". `MANG_CUNG_DATACENTER` (round-
trip TRONG một trung tâm dữ liệu, `500.000 ns`) nhanh hơn
`HDD_SEEK_NGAU_NHIEN` (`10.000.000 ns`) đúng `20` lần — MỘT round-
trip mạng NỘI bộ rẻ hơn hẳn một cú seek đĩa quay.
::::

::::example{#thu-tu-do-lon}
So RAM VỚI mạng liên vùng (`inter-region`, `150.000.000 ns`):
`MANG_KHAC_VUNG` chậm hơn `RAM` đúng `1.500.000` lần — MỘT round-trip
sang datacenter Ở CHÂU lục khác chậm hơn một triệu rưỡi LẦN so VỚI
đọc RAM tại chỗ. Đây LÀ lý do hệ thống PHÂN tán tránh gọi liên vùng
Ở đường "hot path" (đường xử lý MỖI request): chi phí không nằm Ở vài
phần trăm, mà Ở NHIỀU bậc độ lớn.

```typescript title=readonly
const DO_TRE_NS: Record<string, number> = {
  L1: 0.5,
  L2: 7,
  RAM: 100,
  SSD_DOC_NGAU_NHIEN: 150_000,
  HDD_SEEK_NGAU_NHIEN: 10_000_000,
  MANG_CUNG_DATACENTER: 500_000,
  MANG_KHAC_VUNG: 150_000_000,
};
function soLanCham(tenNhanh: string, tenCham: string): number {
  return DO_TRE_NS[tenCham]! / DO_TRE_NS[tenNhanh]!;
}

console.log("mang khac vung / mang cung DC:", soLanCham('MANG_CUNG_DATACENTER', 'MANG_KHAC_VUNG'));
console.log("SSD / RAM:", soLanCham('RAM', 'SSD_DOC_NGAU_NHIEN'));
```

```text title=readonly
mang khac vung / mang cung DC: 300
SSD / RAM: 1500
```

Ngay cả HAI cuộc gọi mạng (cùng DC VÀ khác vùng) cũng lệch NHAU
`300` lần — "gọi mạng" không phải MỘT con số duy nhất, mà một DẢI
rộng tuỳ khoảng cách vật LÝ.
::::

::::predict{#doan-ssd-vs-mang commitOnce}
`SSD_DOC_NGAU_NHIEN` LÀ `150.000 ns`. `MANG_CUNG_DATACENTER` (round-
trip TRONG cùng một trung tâm dữ liệu) LÀ `500.000 ns`. Thao TÁC nào
NHANH hơn?

:::opt{correct}
`SSD_DOC_NGAU_NHIEN` nhanh hơn — `150.000 < 500.000`, đọc SSD ngẫu
nhiên tại chỗ nhanh hơn CẢ một round-trip mạng NỘI bộ
:::
:::opt
`MANG_CUNG_DATACENTER` nhanh hơn — mạng hiện đại LUÔN nhanh hơn đĩa,
vì mạng dùng cáp quang còn đĩa CÓ bộ phận cơ khí
::why
Trực giác "mạng LUÔN nhanh hơn đĩa" đúng KHI so VỚI đĩa cơ (HDD,
`10.000.000 ns`) — nhưng SAI khi đối THỦ LÀ SSD.

Chỗ lệch: SSD KHÔNG có bộ phận cơ khí (không seek VẬT lý như HDD) —
đọc ngẫu nhiên trên SSD (`150.000 ns`) NHANH hơn hẳn round-trip mạng
NỘI bộ (`500.000 ns`), dù cả hai đều "nhanh" so VỚI HDD. So sánh
ĐÚNG luôn cần nhìn đúng CẶP con số, không dựa và một quy tắc chung
chung "mạng nhanh hơn đĩa".
::
:::
::::

::::code{#viet_so_lan_cham}
Hoàn thiện `soLanCham` — tính tỉ lệ ĐỘ trễ CỦA thao tác chậm SO với
thao tác nhanh.

```typescript title=starter
const DO_TRE_NS: Record<string, number> = {
  L1: 0.5,
  L2: 7,
  RAM: 100,
  SSD_DOC_NGAU_NHIEN: 150_000,
  HDD_SEEK_NGAU_NHIEN: 10_000_000,
  MANG_CUNG_DATACENTER: 500_000,
  MANG_KHAC_VUNG: 150_000_000,
};

function soLanCham(tenNhanh: string, tenCham: string): number {
  return ___;
}

console.log(soLanCham('MANG_CUNG_DATACENTER', 'HDD_SEEK_NGAU_NHIEN'));
```

```typescript title=solution
const DO_TRE_NS: Record<string, number> = {
  L1: 0.5,
  L2: 7,
  RAM: 100,
  SSD_DOC_NGAU_NHIEN: 150_000,
  HDD_SEEK_NGAU_NHIEN: 10_000_000,
  MANG_CUNG_DATACENTER: 500_000,
  MANG_KHAC_VUNG: 150_000_000,
};

function soLanCham(tenNhanh: string, tenCham: string): number {
  return DO_TRE_NS[tenCham]! / DO_TRE_NS[tenNhanh]!;
}

console.log(soLanCham('MANG_CUNG_DATACENTER', 'HDD_SEEK_NGAU_NHIEN'));
```

```typescript title=test
if (soLanCham('MANG_CUNG_DATACENTER', 'HDD_SEEK_NGAU_NHIEN') !== 20) throw new Error("HDD seek phai cham hon mang cung DC dung 20 lan");
if (soLanCham('RAM', 'MANG_KHAC_VUNG') !== 1_500_000) throw new Error("mang khac vung phai cham hon RAM dung 1.500.000 lan");
if (soLanCham('L1', 'L2') !== 14) throw new Error("L2 phai cham hon L1 dung 14 lan");
if (soLanCham('L1', 'RAM') !== 200) throw new Error("RAM phai cham hon L1 dung 200 lan");
if (soLanCham('RAM', 'SSD_DOC_NGAU_NHIEN') !== 1500) throw new Error("SSD phai cham hon RAM dung 1500 lan");
if (soLanCham('MANG_CUNG_DATACENTER', 'MANG_KHAC_VUNG') !== 300) throw new Error("mang khac vung phai cham hon mang cung DC dung 300 lan");
if (soLanCham('HDD_SEEK_NGAU_NHIEN', 'HDD_SEEK_NGAU_NHIEN') !== 1) throw new Error("so voi chinh no phai la 1 lan");
```

:::hints
- kind: attention
  body: "Chia do tre cua thao tac cham cho do tre cua thao tac nhanh -- mot dong."
- kind: strategy
  body: "DO_TRE_NS[tenCham]! / DO_TRE_NS[tenNhanh]!"
- kind: one-line
  body: "return DO_TRE_NS[tenCham]! / DO_TRE_NS[tenNhanh]!;"
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
Độ trễ đã có thứ tự độ lớn RÕ ràng. Nhưng "chậm bao nhiêu" chưa nói
"hệ thống SỐNG được bao lâu khi có sự cố" — đó LÀ một trục đo khác.
::::

::::reflect{#nghi-lai}
`soLanCham` chỉ LÀ một phép CHIA — nhưng đưa được BẢY con số rất khác
đơn vị (nano-giây, micro-giây, mili-giây trong đề bài GỐC) VỀ CHUNG
một thang đo (`ns`) LÀ bước ẩn quan trọng NHẤT: không quy đổi đúng
đơn vị TRƯỚC, mọi phép SO sánh sau đều vô nghĩa.
::::

::::checkpoint{mastery=0.75}
::::
