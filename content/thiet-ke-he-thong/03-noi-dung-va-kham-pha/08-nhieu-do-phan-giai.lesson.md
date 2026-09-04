---
id: thiet-ke-he-thong.noi-dung-va-kham-pha.nhieu-do-phan-giai
title: "Nhiều độ phân giải: chọn bản vừa băng thông"
summary: "chonDoPhanGiaiPhuHop chon ban co bitrate LON NHAT con <= bang thong (4 muc: 240p/400, 480p/1000, 720p/2500, 1080p/5000). Bang thong 999 (ngay duoi 1000) lui han ve 240p (khong phai 480p, vi 480p doi hoi dung 1000). Bang thong DUNG bang mot muc (2500) van duoc chon muc do (720p, dung <=). Bang thong qua thap (300, duoi ca 240p) van fallback ve 240p thay vi bo trong."
locale: vi
track: thiet-ke-he-thong
module: noi-dung-va-kham-pha
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 9
teaches: [sd.nhieu-do-phan-giai]
requires: [sd.hang-doi-transcode]
concepts: [sd.nhieu-do-phan-giai]
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
Video transcode xong (bài trước) không hề sinh RA đúng MỘT file — nó
sinh RA nhiều bản, mỗi bản MỘT độ phân giải. Client cần chọn ĐÚNG bản
phù hợp với đường TRUYỀN hiện có, không phải LUÔN lấy bản đẹp nhất.
::::

::::explain{#chon-do-phan-giai}
Mỗi bản gắn MỘT `bitrateKbps` (tốc độ dữ liệu cần THIẾT để phát mượt).
`chonDoPhanGiaiPhuHop` chọn bản có bitrate LỚN nhất mà VẪN nhỏ hơn hoặc
bằng băng thông hiện CÓ — bản "đẹp" hơn nhưng ĐÒI hỏi nhiều hơn băng
thông sẽ bị BỎ qua, vì phát nó sẽ GIẬT:

```typescript title=readonly
interface BienTheDoPhanGiai { ten: string; bitrateKbps: number; }

const CAC_BIEN_THE: BienTheDoPhanGiai[] = [
  { ten: "240p", bitrateKbps: 400 },
  { ten: "480p", bitrateKbps: 1000 },
  { ten: "720p", bitrateKbps: 2500 },
  { ten: "1080p", bitrateKbps: 5000 },
];

function chonDoPhanGiaiPhuHop(bangThongKbps: number, cacBienThe: BienTheDoPhanGiai[]): string {
  let ketQua: BienTheDoPhanGiai | undefined;
  for (const bt of cacBienThe) {
    if (bt.bitrateKbps <= bangThongKbps) {
      if (ketQua === undefined || bt.bitrateKbps > ketQua.bitrateKbps) ketQua = bt;
    }
  }
  if (ketQua !== undefined) return ketQua.ten;

  let thapNhat = cacBienThe[0]!;
  for (const bt of cacBienThe) if (bt.bitrateKbps < thapNhat.bitrateKbps) thapNhat = bt;
  return thapNhat.ten;
}

console.log("bang thong 3000 kbps ->", chonDoPhanGiaiPhuHop(3000, CAC_BIEN_THE));
console.log("bang thong 5000 kbps (dung bitrate 1080p) ->", chonDoPhanGiaiPhuHop(5000, CAC_BIEN_THE));
console.log("bang thong 300 kbps (qua thap, ca 240p cung khong vua) ->", chonDoPhanGiaiPhuHop(300, CAC_BIEN_THE));
```

```text title=readonly
bang thong 3000 kbps -> 720p
bang thong 5000 kbps (dung bitrate 1080p) -> 1080p
bang thong 300 kbps (qua thap, ca 240p cung khong vua) -> 240p
```

`3000 kbps` VƯỢT `bitrate` của `720p` (`2500`) nhưng KHÔNG đủ cho
`1080p` (`5000`) — NÊN `720p` được chọn, không phải bản đẹp NHẤT.
`300 kbps` không đủ cho CẢ `240p` (`400`) — nhưng thay VÌ trả về rỗng,
hàm vẫn FALLBACK về bản THẤP nhất để tránh dừng hình HOÀN toàn.
::::

::::example{#nhay-bac-theo-nguong}
Độ phân giải được chọn KHÔNG tăng đều theo băng thông — nó NHẢY bậc
đúng TẠI từng ngưỡng bitrate, VÀ băng thông chỉ THIẾU một chút so VỚI
một ngưỡng sẽ LUI hẳn về mức THẤP hơn TRƯỚC đó, không phải mức "gần
đúng nhất":

```typescript title=readonly
interface BienTheDoPhanGiai { ten: string; bitrateKbps: number; }

const CAC_BIEN_THE: BienTheDoPhanGiai[] = [
  { ten: "240p", bitrateKbps: 400 },
  { ten: "480p", bitrateKbps: 1000 },
  { ten: "720p", bitrateKbps: 2500 },
  { ten: "1080p", bitrateKbps: 5000 },
];

function chonDoPhanGiaiPhuHop(bangThongKbps: number, cacBienThe: BienTheDoPhanGiai[]): string {
  let ketQua: BienTheDoPhanGiai | undefined;
  for (const bt of cacBienThe) {
    if (bt.bitrateKbps <= bangThongKbps) {
      if (ketQua === undefined || bt.bitrateKbps > ketQua.bitrateKbps) ketQua = bt;
    }
  }
  if (ketQua !== undefined) return ketQua.ten;

  let thapNhat = cacBienThe[0]!;
  for (const bt of cacBienThe) if (bt.bitrateKbps < thapNhat.bitrateKbps) thapNhat = bt;
  return thapNhat.ten;
}

const cacMoc = [999, 1000, 2499, 2500];
for (const bt of cacMoc) {
  console.log(`bang thong ${bt} kbps -> ${chonDoPhanGiaiPhuHop(bt, CAC_BIEN_THE)}`);
}
```

```text title=readonly
bang thong 999 kbps -> 240p
bang thong 1000 kbps -> 480p
bang thong 2499 kbps -> 480p
bang thong 2500 kbps -> 720p
```

`999 kbps` chỉ THIẾU đúng `1` so với `bitrate` của `480p` (`1000`) —
NHƯNG kết quả LUI thẳng về `240p`, không hề CÓ mức "480p gần đúng" nào
cả, vì `480p` đòi hỏi ĐỦ `1000`. `1000 kbps` (đúng NGƯỠNG) đã đủ cho
`480p` ngay LẬP tức.
::::

::::predict{#doan-dung-bitrate commitOnce}
Băng thông hiện CÓ đúng bằng `2500` — CHÍNH XÁC bằng `bitrateKbps` của
`720p`, không hơn không kém. `chonDoPhanGiaiPhuHop(2500, CAC_BIEN_THE)`
trả về bản NÀO?

:::opt{correct}
`"720p"` — điều kiện dùng `bt.bitrateKbps <= bangThongKbps`, VÀ `2500 <=
2500` LÀ đúng, nên `720p` VẪN được coi LÀ "vừa" với băng thông hiện có
:::
:::opt
`"480p"` — dùng ĐÚNG hết băng thông hiện có (không CHỪA margin an toàn)
LÀ rủi ro, nên hệ thống nên LUI xuống một mức để đề PHÒNG
::why
Nhầm một chiến lược "AN TOÀN" (chừa margin) VỚI cách `chonDoPhanGiaiPhuHop`
THẬT sự so sánh — nhưng hàm này không hề chừa margin NÀO cả, nó so
sánh CHÍNH XÁC bằng `<=`.

Chỗ lệch: `if (bt.bitrateKbps <= bangThongKbps)` dùng `<=`, KHÔNG phải
`<`. Với `bt.bitrateKbps === 2500` VÀ `bangThongKbps === 2500`, biểu
thức `2500 <= 2500` LÀ `true`, NÊN `720p` được coi LÀ một ứng viên hợp
lệ — VÀ vì nó có `bitrateKbps` lớn NHẤT trong số các ứng viên hợp lệ
(`240p`, `480p`, `720p` đều `<= 2500`), nó được chọn.
::
:::
::::

::::code{#viet_chon_do_phan_giai}
Hoàn thiện vòng lặp CHÍNH trong `chonDoPhanGiaiPhuHop` — với MỖI bản,
nếu bitrate của nó CÒN vừa băng thông, VÀ nó lớn hơn ứng viên tốt nhất
hiện có, cập nhật ứng viên.

```typescript title=starter
interface BienTheDoPhanGiai { ten: string; bitrateKbps: number; }

const CAC_BIEN_THE: BienTheDoPhanGiai[] = [
  { ten: "240p", bitrateKbps: 400 },
  { ten: "480p", bitrateKbps: 1000 },
  { ten: "720p", bitrateKbps: 2500 },
  { ten: "1080p", bitrateKbps: 5000 },
];

function chonDoPhanGiaiPhuHop(bangThongKbps: number, cacBienThe: BienTheDoPhanGiai[]): string {
  let ketQua: BienTheDoPhanGiai | undefined;
  for (const bt of cacBienThe) {
    ___
  }
  if (ketQua !== undefined) return ketQua.ten;

  let thapNhat = cacBienThe[0]!;
  for (const bt of cacBienThe) if (bt.bitrateKbps < thapNhat.bitrateKbps) thapNhat = bt;
  return thapNhat.ten;
}

console.log(chonDoPhanGiaiPhuHop(3000, CAC_BIEN_THE));
```

```typescript title=solution
interface BienTheDoPhanGiai { ten: string; bitrateKbps: number; }

const CAC_BIEN_THE: BienTheDoPhanGiai[] = [
  { ten: "240p", bitrateKbps: 400 },
  { ten: "480p", bitrateKbps: 1000 },
  { ten: "720p", bitrateKbps: 2500 },
  { ten: "1080p", bitrateKbps: 5000 },
];

function chonDoPhanGiaiPhuHop(bangThongKbps: number, cacBienThe: BienTheDoPhanGiai[]): string {
  let ketQua: BienTheDoPhanGiai | undefined;
  for (const bt of cacBienThe) {
    if (bt.bitrateKbps <= bangThongKbps) {
      if (ketQua === undefined || bt.bitrateKbps > ketQua.bitrateKbps) ketQua = bt;
    }
  }
  if (ketQua !== undefined) return ketQua.ten;

  let thapNhat = cacBienThe[0]!;
  for (const bt of cacBienThe) if (bt.bitrateKbps < thapNhat.bitrateKbps) thapNhat = bt;
  return thapNhat.ten;
}

console.log(chonDoPhanGiaiPhuHop(3000, CAC_BIEN_THE));
```

```typescript title=test
if (chonDoPhanGiaiPhuHop(3000, CAC_BIEN_THE) !== "720p") throw new Error("3000 kbps phai chon 720p (bitrate 2500, gan nhat khong vuot qua)");
if (chonDoPhanGiaiPhuHop(5000, CAC_BIEN_THE) !== "1080p") throw new Error("5000 kbps (dung bitrate 1080p) phai chon 1080p");
if (chonDoPhanGiaiPhuHop(9000, CAC_BIEN_THE) !== "1080p") throw new Error("bang thong vuot xa nhat cung chi co 1080p de chon");
if (chonDoPhanGiaiPhuHop(300, CAC_BIEN_THE) !== "240p") throw new Error("bang thong qua thap (300 < 400) van phai fallback ve ban THAP NHAT (240p)");
if (chonDoPhanGiaiPhuHop(999, CAC_BIEN_THE) !== "240p") throw new Error("999 kbps (ngay duoi bitrate 480p la 1000) phai lui ve 240p");
if (chonDoPhanGiaiPhuHop(1000, CAC_BIEN_THE) !== "480p") throw new Error("dung bang bitrate 480p (1000) phai duoc chon 480p, khong roi xuong 240p");
if (chonDoPhanGiaiPhuHop(2500, CAC_BIEN_THE) !== "720p") throw new Error("dung bang bitrate 720p (2500) phai duoc chon 720p, KHONG lui ve 480p");
if (chonDoPhanGiaiPhuHop(2499, CAC_BIEN_THE) !== "480p") throw new Error("2499 (ngay duoi 2500) chua du cho 720p, phai chon 480p");
```

:::hints
- kind: attention
  body: "Voi tung bien the bt, chi xet no la ung vien NEU bitrateKbps cua no <= bangThongKbps; trong so cac ung vien do, giu lai ung vien co bitrate LON NHAT."
- kind: strategy
  body: "if (bt.bitrateKbps <= bangThongKbps) { if (ketQua === undefined || bt.bitrateKbps > ketQua.bitrateKbps) ketQua = bt; }"
- kind: one-line
  body: "if (bt.bitrateKbps <= bangThongKbps) { if (ketQua === undefined || bt.bitrateKbps > ketQua.bitrateKbps) ketQua = bt; }"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 5000
- tier: output
  match: contains
  expect: "720p"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Client giờ chọn đúng bản theo băng thông thực TẾ. Nhưng phục vụ video
phổ biến từ MÁY chủ gốc cho hàng triệu người xem lại LÀ một bài toán
khác.
::::

::::reflect{#nghi-lai}
`chonDoPhanGiaiPhuHop` không hề "làm TRÒN" hay "tìm gần ĐÚNG nhất" — nó
chỉ lọc CÁC ứng viên hợp lệ (`<=` băng thông) RỒI lấy ứng viên lớn nhất
TRONG số đó. Kết quả "nhảy bậc" thay VÌ mượt LÀ hệ quả trực TIẾP của
việc chỉ có một tập hữu HẠN các mức rời rạc để CHỌN, không phải lỗi
thiết kế.
::::

::::checkpoint{mastery=0.79}
::::
