---
id: co-so-du-lieu.dong-thuan-kieu-vsr.vi-sao-mot-ban-sao-khong-du
title: "Vì sao một bản sao không đủ"
summary: "nguongQuorum(tongSo) = floor(tongSo/2)+1 -- N=3 cần 2 phiếu để có đa số, N=5 cần 3, N=4 (chẵn) cần 3 chứ KHÔNG PHẢI 2 (một nửa không phải đa số). laPrimary(chiSo,viewNumber,tongSo) chọn 'bản sao dẫn đầu' theo round-robin viewNumber % tongSo -- N=3: view 0,1,2,3 lần lượt trao quyền primary cho replica0,1,2,0. Mọi replica còn lại LÀ backup."
locale: vi
track: co-so-du-lieu
module: dong-thuan-kieu-vsr
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 9
teaches: [db.vi-sao-mot-ban-sao-khong-du]
requires: [db.tai-hien-dung-bang-seed-da-tim]
concepts: [db.vi-sao-mot-ban-sao-khong-du]
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
q18 khép lại với MỘT máy chủ sổ cái, tiêm lỗi ĐĨA và MẠNG vào chính
nó. Nhưng NẾU máy đó chết HẲN — không phải "crash rồi khởi động
lại", mà LÀ biến mất — thì SAO? Một bản sao KHÔNG BAO GIỜ đủ.
::::

::::explain{#quorum-va-vai-tro}
`nguongQuorum(tongSo)` tính số PHIẾU tối thiểu để coi LÀ "đa số" —
`Math.floor(tongSo / 2) + 1`. `laPrimary(chiSo, viewNumber, tongSo)`
chọn "bản sao DẪN đầu" (primary) theo ROUND-ROBIN: `chiSo === viewNumber
% tongSo`. MỌI replica CÒN lại LÀ "backup":

```typescript title=readonly
function nguongQuorum(tongSo: number): number {
  return Math.floor(tongSo / 2) + 1;
}
function laPrimary(chiSoReplica: number, viewNumber: number, tongSo: number): boolean {
  return chiSoReplica === viewNumber % tongSo;
}

console.log("quorum(N=3):", nguongQuorum(3));
console.log("quorum(N=5):", nguongQuorum(5));
console.log("quorum(N=4):", nguongQuorum(4));
for (let view = 0; view < 4; view++) {
  const idPrimary = [0, 1, 2].find((i) => laPrimary(i, view, 3));
  console.log(`N=3, view=${view}: primary=replica${idPrimary}`);
}
```

```text title=readonly
quorum(N=3): 2
quorum(N=5): 3
quorum(N=4): 3
N=3, view=0: primary=replica0
N=3, view=1: primary=replica1
N=3, view=2: primary=replica2
N=3, view=3: primary=replica0
```

`N=3` cần `2` phiếu để có ĐA số (chính nó cộng MỘT backup) — đây LÀ
`f+1` trong công thức `2f+1` kinh điển của đồng thuận CHỊU lỗi
Byzantine-free (`f=1` lỗi CHỊU được, `N=2f+1=3`). `viewNumber` tăng
DẦN mỗi lần đổi primary (bài 9-12 sẽ dùng ĐIỀU này); `view % tongSo`
LUÔN cho MỘT chỉ số hợp lệ, xoay VÒNG qua tất cả replica.
::::

::::example{#tai-sao-so-le}
`nguongQuorum(4)` = `3`, KHÔNG phải `2` — MỘT nửa của `4` (`2`)
KHÔNG phải LÀ đa số THẬT, vì `2` phiếu "có" VÀ `2` phiếu "vắng" có
thể HOÀ, không quyết định được GÌ. Đây LÀ lý do VSR THẬT (VÀ
TigerBeetle) LUÔN khuyến nghị `N` LẺ (`3`, `5`, `7`): với `N` chẵn,
bạn TRẢ thêm một replica mà KHÔNG được thêm khả năng chịu lỗi nào —
`N=4` VÀ `N=5` CÙNG chịu được `f=1` lỗi, nhưng `N=5` chịu được THÊM
một tình huống mạng chia đôi (partition) MÀ `N=4` không chịu nổi.
::::

::::predict{#doan-quorum-n-1 commitOnce}
`nguongQuorum(1)` (chỉ MỘT replica DUY nhất, không có bản sao NÀO
khác) trả về BAO nhiêu?
:::opt{correct}
`1` — `Math.floor(1/2)+1 = 0+1 = 1`; một hệ THỐNG một replica LUÔN tự
nó LÀ "đa số" của chính nó, nhưng đây chính LÀ vấn đề: nó chết LÀ hệ
thống chết theo, không CÒN ai để "biểu quyết" thay
:::
:::opt
Lỗi runtime — `tongSo=1` không hợp LỆ cho một hệ thống ĐỒNG thuận
::why
Trực giác NÀY đúng về mặt Ý nghĩa THỰC tế (một hệ đồng thuận MỘT
replica vô nghĩa) — nhưng SAI về mặt CODE: `nguongQuorum` KHÔNG hề
kiểm tra `tongSo` tối thiểu, nó chỉ LÀ một phép TOÁN số học thuần.

Chỗ lệch: `Math.floor(1 / 2) + 1` tính ĐÚNG bình thường, cho ra `1`,
không NÉM lỗi. Bài học THẬT của con số NÀY: `N=1` "hợp lệ" về mặt
CÔNG thức nhưng VÔ dụng về mặt CHỊU lỗi — chính XÁC lý do q19 dùng
`N=3` xuyên suốt.
::
:::
::::

::::code{#viet_nguong_quorum}
Hoàn thiện `nguongQuorum` — trả về `Math.floor(tongSo / 2) + 1`.

```typescript title=starter
function nguongQuorum(tongSo: number): number {
  ___
}
function laPrimary(chiSoReplica: number, viewNumber: number, tongSo: number): boolean {
  return chiSoReplica === viewNumber % tongSo;
}

console.log(nguongQuorum(3));
```

```typescript title=solution
function nguongQuorum(tongSo: number): number {
  return Math.floor(tongSo / 2) + 1;
}
function laPrimary(chiSoReplica: number, viewNumber: number, tongSo: number): boolean {
  return chiSoReplica === viewNumber % tongSo;
}

console.log(nguongQuorum(3));
```

```typescript title=test
if (nguongQuorum(3) !== 2) throw new Error("nguongQuorum(3) phai la 2");
if (nguongQuorum(5) !== 3) throw new Error("nguongQuorum(5) phai la 3");
if (nguongQuorum(4) !== 3) throw new Error("nguongQuorum(4) (N chan) phai la 3, khong phai 2 -- can HON nua so N/2 moi la da so");
if (nguongQuorum(1) !== 1) throw new Error("nguongQuorum(1) phai la 1");
for (let view = 0; view < 4; view++) {
  const ai = [0, 1, 2].filter((i) => laPrimary(i, view, 3));
  if (ai.length !== 1) throw new Error("dung 1 replica la primary moi view");
  if (ai[0] !== view % 3) throw new Error("primary phai la replica co chiSo = view % tongSo");
}
```

:::hints
- kind: attention
  body: "Tra ve Math.floor(tongSo / 2) + 1 -- mot dong."
- kind: strategy
  body: "return Math.floor(tongSo / 2) + 1;"
- kind: one-line
  body: "return Math.floor(tongSo / 2) + 1;"
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
Quorum xong, primary/backup xong. Giờ cần biết MỖI replica GIỮ những
gì trong đầu nó để tham gia được vào cuộc đồng thuận NÀY.
::::

::::reflect{#nghi-lai}
`nguongQuorum` VÀ `laPrimary` LÀ hai phép TOÁN bé — nhưng CHÚNG LÀ
nền tảng của TOÀN bộ VSR: mọi quyết định (commit MỘT bút toán, bầu
MỘT primary mới) đều QUAY về câu hỏi "đã đủ phiếu CHƯA?" VÀ "ai LÀ
người dẫn đầu LÚC này?". `N=3`, `quorum=2` LÀ con số sẽ THEO suốt 13
bài còn LẠI của q19.
::::

::::checkpoint{mastery=0.8}
::::
