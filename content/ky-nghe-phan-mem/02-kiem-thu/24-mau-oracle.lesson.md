---
id: ky-nghe-phan-mem.kiem-thu.mau-oracle
title: "Mẫu Oracle — so sánh với cài đặt ĐÃ TIN CẬY"
summary: "Mẫu thứ tư: so sánh cài đặt MỚI với MỘT oracle (nguồn ĐÃ tin cậy, ví dụ hàm có sẵn của ngôn ngữ) THAY VÌ chọn giá trị kỳ vọng tay. Tự viết canBacHaiCuaToi(n) (Newton's method), so sánh với Math.sqrt(n) (oracle) trên hàng chục số ngẫu nhiên, làm tròn để chấp nhận sai số nhỏ."
locale: vi
track: ky-nghe-phan-mem
module: kiem-thu
order: 24
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [kt.pattern-oracle]
requires: [kt.pattern-round-trip]
concepts: [kt.pattern-oracle]
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
Tự viết `canBacHaiCuaToi` (Newton's method) — so sánh nó VỚI
`Math.sqrt` CÓ SẴN (một "oracle" đáng tin). Mẫu tính chất này thế nào?
::::

::::explain{#oracle-so-sanh-nguon-tin-cay}
Mẫu tính chất **THỨ TƯ**: **Oracle** — so sánh cài đặt **MỚI** VỚI
MỘT **"oracle"** (nguồn ĐÃ tin cậy, VÍ DỤ hàm CÓ SẴN của ngôn ngữ)
**THAY VÌ** tự CHỌN giá trị kỳ vọng TAY:

```typescript
function canBacHaiCuaToi(n: number): number {
  if (n === 0) return 0;
  let x = n;
  for (let i = 0; i < 20; i++) {
    x = (x + n / x) / 2;
  }
  return x;
}

console.log(canBacHaiCuaToi(4), Math.sqrt(4));
console.log(canBacHaiCuaToi(2), Math.sqrt(2));
```

```text
2 2
1.414213562373095 1.4142135623730951
```

`canBacHaiCuaToi` dùng **Newton's method** (LẶP LẠI `x = (x + n/x) /
2`, MỖI vòng LẠI GẦN đúng CĂN BẬC HAI HƠN) — KHÔNG có gì "kỳ diệu",
CHỈ LÀ LẶP `20` lần. `Math.sqrt` — hàm CÓ SẴN của JavaScript, ĐÃ được
KIỂM CHỨNG kỹ, đóng vai TRÒ **oracle** (nguồn SỰ THẬT ĐÁNG tin) —
kết quả HAI hàm **RẤT gần** nhau (`4` VÀ `2` khớp CHÍNH XÁC, `2`
KHỚP gần đúng TỚI `15` chữ số thập phân).
::::

::::example{#lam-tron-de-chap-nhan-sai-so}
So sánh **KHÔNG** dùng `===` (SO chính XÁC BIT-CHO-BIT) — dùng
**làm tròn/sai số nhỏ**, vì phép TÍNH lặp KHÔNG bao giờ khớp TUYỆT
ĐỐI với `Math.sqrt` (thuật toán NỘI BỘ KHÁC nhau):

```typescript title=readonly
type BoSinh<T> = { generate: () => T };
function soNguyen(min: number, max: number): BoSinh<number> {
  return { generate: () => Math.floor(Math.random() * (max - min + 1)) + min };
}
function kiemTraTinhChat<A>(bs: BoSinh<A>, tinhChat: (a: A) => boolean, soLan: number = 100): void {
  for (let lan = 0; lan < soLan; lan++) {
    const input = bs.generate();
    if (!tinhChat(input)) throw new Error(`that bai o lan thu ${lan}: input = ${JSON.stringify(input)}`);
  }
  console.log(`[PASS] tinh chat dung tren ca ${soLan} lan`);
}
function canBacHaiCuaToi(n: number): number {
  if (n === 0) return 0;
  let x = n;
  for (let i = 0; i < 20; i++) {
    x = (x + n / x) / 2;
  }
  return x;
}

console.log(canBacHaiCuaToi(2) === Math.sqrt(2));

kiemTraTinhChat(soNguyen(0, 10000), (n) => Math.abs(canBacHaiCuaToi(n) - Math.sqrt(n)) < 0.0001, 200);
```

```text title=readonly
false
[PASS] tinh chat dung tren ca 200 lan
```

`canBacHaiCuaToi(2) === Math.sqrt(2)` LÀ `false` — HAI thuật toán
(Newton's method vs THUẬT TOÁN NỘI BỘ của `Math.sqrt`) hội TỤ tới
CÙNG giá trị nhưng **KHÔNG** cho ra CÙNG BIT-PATTERN dấu phẩy động
CHÍNH XÁC. Tính chất Ở ĐÂY dùng `Math.abs(a - b) < 0.0001` (sai số
CHẤP NHẬN được, KHÔNG PHẢI bằng TUYỆT ĐỐI) — VẪN `[PASS]` TRÊN `200`
số NGẪU NHIÊN từ `0` tới `10000`.
::::

::::predict{#doan-oracle-tren-so-lon commitOnce}
```typescript
function canBacHaiCuaToi(n: number): number {
  if (n === 0) return 0;
  let x = n;
  for (let i = 0; i < 20; i++) {
    x = (x + n / x) / 2;
  }
  return x;
}

console.log(Math.abs(canBacHaiCuaToi(1000000) - Math.sqrt(1000000)) < 0.0001);
```

Dòng cuối in ra gì?

:::opt{correct}
`true`
:::

:::opt
`false` — vì `n = 1000000` LỚN HƠN NHIỀU SO VỚI các ví dụ ĐÃ THỬ
(`4`, `2`), VÀ NEWTON'S METHOD **HỘI TỤ CHẬM HƠN** VỚI SỐ CÀNG LỚN —
`20` vòng lặp KHÔNG ĐỦ để hội tụ ĐỦ chính xác, sai số VƯỢT `0.0001`
::why
Gần đúng ở việc bạn nghĩ tới MỐI LO "số CÀNG lớn, thuật toán LẶP
CÀNG cần NHIỀU vòng hơn" — MỘT trực giác hợp lý ĐỐI VỚI NHIỀU thuật
toán số HỌC lặp (ví dụ tìm kiếm TUYẾN TÍNH).

Chỗ lệch: Newton's method có **HỘI TỤ BẬC HAI** (quadratic
convergence) — SỐ CHỮ SỐ đúng **GẦN NHƯ GẤP ĐÔI** MỖI vòng lặp, BẤT
KỂ `n` LỚN cỡ nào (chỉ CẦN điểm khởi ĐẦU HỢP LÝ, Ở ĐÂY LÀ `x = n`,
LUÔN hội tụ NHANH cho căn bậc hai). VỚI `n = 1000000`, `20` vòng lặp
LÀ THỪA THÃI để đạt độ chính XÁC XA hơn `0.0001` RẤT NHIỀU (thực tế
đạt tới GIỚI HẠN chính XÁC của kiểu `number` CHỈ SAU KHOẢNG `10`
vòng). Tính chất VẪN `[PASS]`.
::
:::

:::opt
Máy báo lỗi biên dịch — `Math.abs(canBacHaiCuaToi(1000000) -
Math.sqrt(1000000)) < 0.0001` không hợp lệ, vì so sánh HAI biểu thức
số học PHỨC TẠP TRỰC TIẾP bằng `<` đòi CẢ HAI bên PHẢI được gán VÀO
biến TRUNG GIAN trước
::why
Gần đúng ở việc bạn nghĩ tới VIỆC biểu thức Ở ĐÂY khá PHỨC TẠP (gọi
hàm LỒNG NHAU, PHÉP trừ, `Math.abs`, RỒI so sánh) — một quan sát
ĐÚNG rằng biểu thức DÀI hơn BÌNH THƯỜNG.

Chỗ lệch: TypeScript (VÀ JavaScript) KHÔNG có RÀNG BUỘC "phải gán
BIẾN trung gian TRƯỚC khi so sánh" — BẤT KỲ biểu thức nào TRẢ VỀ
`number` ĐỀU dùng được TRỰC TIẾP Ở HAI VẾ của toán tử so sánh, DÙ
LỒNG bao nhiêu LỚP hàm. Đây LÀ CÁCH viết BÌNH THƯỜNG, ĐÃ dùng NHIỀU
lần xuyên suốt track NÀY. Biên dịch VÀ chạy sạch.
::
:::
::::

::::code{#viet_canbachaicuatoi}
Tự viết PHẦN LÕI của `canBacHaiCuaToi` (Newton's method).

```typescript title=starter
type BoSinh<T> = { generate: () => T };
function soNguyen(min: number, max: number): BoSinh<number> {
  return { generate: () => Math.floor(Math.random() * (max - min + 1)) + min };
}
function kiemTraTinhChat<A>(bs: BoSinh<A>, tinhChat: (a: A) => boolean, soLan: number = 100): void {
  for (let lan = 0; lan < soLan; lan++) {
    const input = bs.generate();
    if (!tinhChat(input)) throw new Error(`that bai o lan thu ${lan}: input = ${JSON.stringify(input)}`);
  }
  console.log(`[PASS] tinh chat dung tren ca ${soLan} lan`);
}

function canBacHaiCuaToi(n: number): number {
  if (n === 0) return 0;
  let x = n;
  for (let i = 0; i < 20; i++) {
    x = ___;
  }
  return x;
}

kiemTraTinhChat(soNguyen(0, 10000), (n) => Math.abs(canBacHaiCuaToi(n) - Math.sqrt(n)) < 0.0001, 200);
```

```typescript title=solution
type BoSinh<T> = { generate: () => T };
function soNguyen(min: number, max: number): BoSinh<number> {
  return { generate: () => Math.floor(Math.random() * (max - min + 1)) + min };
}
function kiemTraTinhChat<A>(bs: BoSinh<A>, tinhChat: (a: A) => boolean, soLan: number = 100): void {
  for (let lan = 0; lan < soLan; lan++) {
    const input = bs.generate();
    if (!tinhChat(input)) throw new Error(`that bai o lan thu ${lan}: input = ${JSON.stringify(input)}`);
  }
  console.log(`[PASS] tinh chat dung tren ca ${soLan} lan`);
}

function canBacHaiCuaToi(n: number): number {
  if (n === 0) return 0;
  let x = n;
  for (let i = 0; i < 20; i++) {
    x = (x + n / x) / 2;
  }
  return x;
}

kiemTraTinhChat(soNguyen(0, 10000), (n) => Math.abs(canBacHaiCuaToi(n) - Math.sqrt(n)) < 0.0001, 200);
```

```typescript title=test
function assertGanDung(actual: number, expected: number, saiSoToiDa: number, label: string): void {
  if (Math.abs(actual - expected) >= saiSoToiDa) throw new Error(`[FAIL] ${label}: mong gan ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

assertGanDung(canBacHaiCuaToi(4), 2, 0.0001, "can bac hai cua 4 phai gan dung 2");
assertGanDung(canBacHaiCuaToi(2), Math.sqrt(2), 0.0001, "can bac hai cua 2 phai gan dung Math.sqrt(2)");
assertGanDung(canBacHaiCuaToi(1000000), 1000, 0.0001, "can bac hai cua so lon van phai hoi tu chinh xac");
assertGanDung(canBacHaiCuaToi(1), 1, 0.0001, "can bac hai cua 1 phai la 1");
```

:::hints
- kind: attention
  body: "Công thức Newton's method để xấp xỉ căn bậc hai của n, bắt đầu từ x=n: mỗi vòng lặp cập nhật x thành trung bình cộng của x và n/x."
- kind: strategy
  body: '(x + n / x) / 2'
- kind: one-line
  body: '___ = (x + n / x) / 2'
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
Bốn mẫu tính chất: Bất Biến, Idempotent, Đi-Về, Oracle. Bước tiếp
theo: khi tính chất thất bại ở một số ngẫu nhiên PHỨC TẠP — thu nhỏ
về ca lỗi TỐI GIẢN.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Tính chất thất bại Ở một input ngẫu nhiên LỚN (ví dụ số `733`) — thu
NHỎ dần về giá trị NHỎ NHẤT vẫn còn thất bại, dễ debug HƠN nhiều —
`thuNhoSoNguyen` trông thế nào?
::::

::::checkpoint{mastery=0.8}
::::
