---
id: ky-nghe-phan-mem.kiem-thu.tu-thu-nho-dau-vao-loi
title: "Tự thu nhỏ đầu vào LỖI — shrinking, tìm ca THẤT BẠI TỐI GIẢN"
summary: "Khi tính chất thất bại ở một input ngẫu nhiên LỚN/PHỨC TẠP, thu nhỏ dần về giá trị NHỎ NHẤT/ĐƠN GIẢN NHẤT vẫn còn thất bại — debug dễ hơn nhiều với ca lỗi tối giản. thuNhoSoNguyen(batDau, tinhChat) — tìm kiếm nhị phân thu hẹp dần khoảng [đúng, sai] về tối giản."
locale: vi
track: ky-nghe-phan-mem
module: kiem-thu
order: 25
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [kt.hand-rolled-shrinking]
requires: [kt.pattern-oracle]
concepts: [kt.hand-rolled-shrinking]
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
Tính chất thất bại Ở một input ngẫu nhiên LỚN (ví dụ `733`) — thu
NHỎ dần về giá trị NHỎ NHẤT vẫn còn thất bại, dễ debug hơn nhiều?
::::

::::explain{#shrinking-tim-ca-loi-toi-gian}
KHI tính chất **THẤT BẠI** Ở một input ngẫu nhiên **LỚN/PHỨC TẠP**
(ví dụ số `733`), **THU NHỎ** dần VỀ giá trị **NHỎ NHẤT** vẫn CÒN
thất bại — debug DỄ HƠN NHIỀU với ca LỖI **TỐI GIẢN** thay VÌ số ngẫu
nhiên bất KỲ. `thuNhoSoNguyen` dùng **TÌM KIẾM NHỊ PHÂN** thu hẹp dần
khoảng `[đúng, sai]`:

```typescript
function thuNhoSoNguyen(batDau: number, tinhChat: (n: number) => boolean): number {
  let thap = 0;   // biet CHAC: tinhChat(thap) la true (PASS)
  let cao = batDau; // biet CHAC: tinhChat(cao) la false (FAIL)
  let vongLap = 0;
  while (cao - thap > 1 && vongLap < 100) {
    vongLap++;
    const giua = Math.floor((thap + cao) / 2);
    if (tinhChat(giua)) {
      thap = giua; // giua PASS -- ca loi nam O PHIA TREN giua
    } else {
      cao = giua; // giua FAIL -- ca loi CO THE thu nho HON NUA
    }
  }
  return cao; // gia tri FAIL NHO NHAT tim duoc
}

// tinh chat "n < 100" that bai voi n = 733 -- thu nho VE dau?
console.log(thuNhoSoNguyen(733, (n) => n < 100));
```

```text
100
```

BẮT ĐẦU từ `733` (MỘT ca lỗi "TÌNH CỜ" tìm được LÚC sinh ngẫu nhiên),
`thuNhoSoNguyen` THU HẸP dần khoảng `[0, 733]` — MỖI vòng KIỂM `giua`
(điểm GIỮA khoảng): NẾU `giua` VẪN PASS, ca lỗi NẰM Ở phía TRÊN (thu
hẹp `thap`); NẾU `giua` FAIL, ca lỗi CÓ THỂ NHỎ HƠN NỮA (thu hẹp
`cao`). Kết quả CUỐI: `100` — ĐÚNG NGƯỠNG nhỏ NHẤT khiến tính chất
"n < 100" thất bại, THAY VÌ `733` (số NGẪU NHIÊN, KHÔNG NÓI LÊN điều
GÌ về "TẠI SAO" thất bại).
::::

::::example{#nhieu-nguong-khac-nhau}
`thuNhoSoNguyen` HOẠT ĐỘNG với **BẤT KỲ** tính chất NÀO — KHÔNG CHỈ
`n < 100` — VÀ LUÔN thu VỀ đúng NGƯỠNG:

```typescript title=readonly
function thuNhoSoNguyen(batDau: number, tinhChat: (n: number) => boolean): number {
  let thap = 0;
  let cao = batDau;
  let vongLap = 0;
  while (cao - thap > 1 && vongLap < 100) {
    vongLap++;
    const giua = Math.floor((thap + cao) / 2);
    if (tinhChat(giua)) {
      thap = giua;
    } else {
      cao = giua;
    }
  }
  return cao;
}

console.log(thuNhoSoNguyen(500, (n) => n < 50));
console.log(thuNhoSoNguyen(10, (n) => n < 7));
console.log(thuNhoSoNguyen(1000, (n) => n < 1));
```

```text title=readonly
50
7
1
```

BA lần gọi, BA ngưỡng KHÁC nhau (`50`, `7`, `1`), BA giá trị BẮT ĐẦU
KHÁC nhau (`500`, `10`, `1000`) — `thuNhoSoNguyen` LUÔN thu VỀ ĐÚNG
ngưỡng, KHÔNG PHỤ THUỘC giá trị bắt đầu LỚN cỡ nào. Vòng lặp `vongLap
< 100` LÀ MỘT **GIỚI HẠN AN TOÀN** — thực TẾ tìm kiếm nhị phân hội tụ
RẤT nhanh (khoảng `log2(batDau)` bước), `100` vòng LÀ THỪA THÃI cho
BẤT KỲ số bắt đầu THỰC TẾ nào, NHƯNG vẫn CHẶN được vòng lặp KHÔNG hồi
kết NẾU `tinhChat` (hàm do NGƯỜI KHÁC viết, CÓ THỂ có BUG) cư XỬ THẤT
THƯỜNG.
::::

::::predict{#doan-nguong-30 commitOnce}
```typescript
function thuNhoSoNguyen(batDau: number, tinhChat: (n: number) => boolean): number {
  let thap = 0;
  let cao = batDau;
  let vongLap = 0;
  while (cao - thap > 1 && vongLap < 100) {
    vongLap++;
    const giua = Math.floor((thap + cao) / 2);
    if (tinhChat(giua)) {
      thap = giua;
    } else {
      cao = giua;
    }
  }
  return cao;
}

console.log(thuNhoSoNguyen(50, (n) => n < 30));
```

Dòng cuối in ra gì?

:::opt{correct}
`30`
:::

:::opt
`25` — vì `thuNhoSoNguyen` LUÔN thu NHỎ VỀ **ĐIỂM GIỮA** của khoảng
BẮT ĐẦU (`[0, 50]`), KHÔNG PHẢI về đúng NGƯỠNG của tính chất — điểm
GIỮA của `[0, 50]` LÀ `25`
::why
Gần đúng ở việc bạn nhớ THUẬT toán CÓ TÍNH "ĐIỂM GIỮA" (`giua =
Math.floor((thap + cao) / 2)`) LÀ TRUNG TÂM của MỖI bước LẶP — một
quan sát ĐÚNG về CƠ CHẾ bên TRONG.

Chỗ lệch: `giua` LÀ điểm KIỂM TRA Ở TỪNG BƯỚC, KHÔNG PHẢI kết QUẢ
CUỐI CÙNG — thuật toán LẶP LẠI **NHIỀU LẦN**, MỖI lần thu HẸP khoảng
`[thap, cao]` LẠI GẦN đúng NGƯỠNG THẬT của `tinhChat` (Ở ĐÂY LÀ `n <
30`, NGƯỠNG THẬT LÀ `30`), CHỨ KHÔNG dừng LẠI Ở điểm GIỮA của khoảng
BAN ĐẦU. Trace: `thap=0,cao=50`→`giua=25`(PASS,25<30)→`thap=25`;
`giua=37`(FAIL)→`cao=37`; `giua=31`(FAIL)→`cao=31`; `giua=28`(PASS)→
`thap=28`; `giua=29`(PASS)→`thap=29`; `cao-thap=31-29=2`, TIẾP:
`giua=30`(FAIL)→`cao=30`; `cao-thap=30-29=1`, DỪNG. Trả VỀ `cao=30`.
::
:::

:::opt
Máy báo lỗi biên dịch — gọi `thuNhoSoNguyen(50, (n) => n < 30)` với
`50` VÀ ngưỡng `30` KHÔNG hợp lệ, vì `thuNhoSoNguyen` đòi `tinhChat
(batDau)` PHẢI LÀ `false` (điều kiện TIÊN QUYẾT của thuật toán), VÀ
TypeScript KIỂM tra RÀNG BUỘC NÀY lúc BIÊN dịch
::why
Gần đúng ở việc bạn nhớ thuật toán CÓ MỘT GIẢ ĐỊNH quan trọng —
`tinhChat(batDau)` PHẢI LÀ `false` (điểm bắt đầu PHẢI THẬT SỰ thất
bại, nếu KHÔNG thuật toán vô nghĩa) — MỘT quan sát ĐÚNG về LOGIC của
thuật toán (VÀ Ở lời gọi NÀY, `50 < 30` LÀ `false`, giả định NÀY
ĐƯỢC thoả MÃN).

Chỗ lệch: ĐÂY LÀ MỘT **RÀNG BUỘC LOGIC/NGHIỆP VỤ** (giống MỌI ràng
buộc "đầu vào PHẢI hợp lệ" ĐÃ gặp XUYÊN SUỐT track NÀY), KHÔNG PHẢI
RÀNG BUỘC Ở TẦNG KIỂU — TypeScript KHÔNG THỂ (VÀ KHÔNG CÓ CÁCH) kiểm
tra "kết QUẢ của việc GỌI MỘT hàm VỚI MỘT giá trị CỤ THỂ" lúc BIÊN
DỊCH (đó LÀ hành VI LÚC CHẠY). Biên dịch sạch — VÀ MAY MẮN, lời gọi
NÀY THOẢ giả định (đúng NHƯ phân TÍCH Ở phương án ĐÚNG).
::
:::
::::

::::code{#viet_thunhosonguyen}
Tự viết PHẦN LÕI của `thuNhoSoNguyen`.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function thuNhoSoNguyen(batDau: number, tinhChat: (n: number) => boolean): number {
  let thap = 0;
  let cao = batDau;
  let vongLap = 0;
  while (cao - thap > 1 && vongLap < 100) {
    vongLap++;
    const giua = Math.floor((thap + cao) / 2);
    if (tinhChat(giua)) {
      thap = ___;
    } else {
      cao = ___;
    }
  }
  return cao;
}

assertEqual(thuNhoSoNguyen(733, (n) => n < 100), 100, "thu nho ve dung nguong 100");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function thuNhoSoNguyen(batDau: number, tinhChat: (n: number) => boolean): number {
  let thap = 0;
  let cao = batDau;
  let vongLap = 0;
  while (cao - thap > 1 && vongLap < 100) {
    vongLap++;
    const giua = Math.floor((thap + cao) / 2);
    if (tinhChat(giua)) {
      thap = giua;
    } else {
      cao = giua;
    }
  }
  return cao;
}

assertEqual(thuNhoSoNguyen(733, (n) => n < 100), 100, "thu nho ve dung nguong 100");
```

```typescript title=test
assertEqual(thuNhoSoNguyen(500, (n) => n < 50), 50, "nguong 50 tu diem bat dau 500");
assertEqual(thuNhoSoNguyen(10, (n) => n < 7), 7, "nguong 7 tu diem bat dau nho, 10");
assertEqual(thuNhoSoNguyen(1000, (n) => n < 1), 1, "nguong 1 -- thu nho toi da co the");
assertEqual(thuNhoSoNguyen(50, (n) => n < 30), 30, "nguong 30, diem bat dau 50");
```

:::hints
- kind: attention
  body: "Nhánh giua PASS (thap = ...): ca lỗi nằm Ở PHÍA TRÊN giua, thu hẹp thap LÊN giua. Nhánh giua FAIL (cao = ...): ca lỗi CÓ THỂ nhỏ hơn nữa, thu hẹp cao XUỐNG giua."
- kind: strategy
  body: 'giua : giua — cả hai nhánh đều gán bằng điểm giữa vừa tính, chỉ khác BIẾN nào được cập nhật.'
- kind: one-line
  body: '___ (nhanh PASS) = giua\n___ (nhanh FAIL) = giua'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "PASS"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Shrinking: thu nhỏ ca lỗi ngẫu nhiên về ngưỡng tối giản, dễ debug
hơn nhiều. Bài chốt cụm: ghép tất cả vào một bộ kiểm tính chất trả
về Result.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Viết LẠI `kiemTraTinhChat` để TRẢ VỀ `Result<{soLanDung}, {input,
viTri}>` THAY VÌ throw/console.log (nối phong cách track — Result đã
học từ T4.5) — trông thế nào?
::::

::::checkpoint{mastery=0.8}
::::
