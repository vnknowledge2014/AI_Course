---
id: co-so-du-lieu.mot-hop-nhieu-hinh.hanh-vi-khop-boolean
title: "Hành vi khớp: AND/OR"
summary: "WHERE tuoi > 18 AND tuoi < 28 (SurrealDB thật) trả về ĐÚNG cùng tập kết quả với danhGia (TypeScript, đệ quy trên cây điều kiện — đúng tinh thần CayAst với VaNut/HoacNut của q07) trên CÙNG dữ liệu. WHERE ... OR ... cũng khớp. Trên điều kiện GHÉP (không chỉ một phép so sánh đơn), hai bộ máy VẪN đồng thuận."
locale: vi
track: co-so-du-lieu
module: mot-hop-nhieu-hinh
order: 12
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [db.hanh-vi-khop-boolean]
requires: [db.gioi-han-mini-engine]
concepts: [db.hanh-vi-khop-boolean]
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
Bài 2 đối chiếu MỘT phép so sánh ĐƠN (`tuoi > 18`). q07's `CayAst`
CÒN hỗ trợ `VaNut`/`HoacNut` (AND/OR) — điều kiện GHÉP có khớp giữa
hai bộ máy KHÔNG?
::::

::::explain{#and-that}
Bốn người, tuổi `15, 20, 25, 30`. Lọc TUỔI trong khoảng MỞ `(18,
28)`:

```text title=readonly
SELECT ten FROM nguoi WHERE tuoi > 18 AND tuoi < 28;
```

```text title=readonly
[{ ten: "Binh" }, { ten: "Chi" }]
```

`Binh` (`20`) VÀ `Chi` (`25`) đều thoả CẢ hai điều kiện — `An` (`15`)
KHÔNG thoả điều kiện ĐẦU, `Dung` (`30`) không thoả điều kiện SAU.
::::

::::example{#or-that}
CÙNG dữ liệu, đổi `AND` thành `OR` — lấy MỌI người NGOÀI khoảng:

```text title=readonly
SELECT ten FROM nguoi WHERE tuoi < 18 OR tuoi > 28;
```

```text title=readonly
[{ ten: "An" }, { ten: "Dung" }]
```

`An` (`15`, thoả `< 18`) VÀ `Dung` (`30`, thoả `> 28`) — `OR` chỉ
cần MỘT trong hai điều kiện đúng, KHÁC hẳn `AND` (bắt buộc CẢ hai).
::::

::::predict{#doan-va-hoac-long-nhau commitOnce}
Điều kiện GHÉP ba tầng: `(tuoi > 18 AND tuoi < 28) OR tuoi = 30`.
Áp lên CÙNG bốn người (`15, 20, 25, 30`) — kết QUẢ có bao nhiêu
người?

:::opt{correct}
`3` — `Binh` (`20`), `Chi` (`25`) thoả nhánh TRÁI của `OR`; `Dung`
(`30`) thoả nhánh PHẢI
:::

:::opt
`2` — `OR` chỉ giữ lại kết QUẢ của MỘT nhánh (nhánh nào ĐÚNG trước),
không GỘP cả hai
::why
Gần đúng ở việc bạn nghĩ TỚI `OR` như một phép "CHỌN MỘT nhánh"
(giống `if/else`) — MỘT liên tưởng dễ hiểu TỪ cấu trúc điều khiển
thông thường.

Chỗ lệch: `OR` (VÀ `HoacNut` của q07) LÀ phép logic BOOLEAN, không
phải rẽ NHÁNH — nó đánh GIÁ CẢ hai vế, VÀ giữ lại MỌI hàng thoả ÍT
nhất một trong hai. `(tuoi>18 AND tuoi<28)` đúng VỚI `Binh`, `Chi`
(hai người); `tuoi=30` đúng VỚI `Dung` (một người) — GỘP lại LÀ ba
người, không phải CHỈ hai từ một nhánh DUY nhất.
::
:::
::::

::::code{#viet_danh_gia}
Hoàn thiện `danhGia` — nhánh `"va"` (AND): CHỈ đúng khi CẢ hai vế
con (`trai`, `phai`) đều đúng.

```typescript title=starter
interface Nguoi {
  ten: string;
  tuoi: number;
}

type DieuKien =
  | { loai: "so-sanh"; toanTu: ">" | "<"; nguong: number }
  | { loai: "va"; trai: DieuKien; phai: DieuKien }
  | { loai: "hoac"; trai: DieuKien; phai: DieuKien };

function danhGia(dk: DieuKien, tuoi: number): boolean {
  if (dk.loai === "so-sanh") {
    return dk.toanTu === ">" ? tuoi > dk.nguong : tuoi < dk.nguong;
  }
  if (dk.loai === "va") {
    ___
  }
  return danhGia(dk.trai, tuoi) || danhGia(dk.phai, tuoi);
}

function chonTheoDieuKien(bang: Nguoi[], dk: DieuKien): string[] {
  const ketQua: string[] = [];
  for (const hang of bang) {
    if (danhGia(dk, hang.tuoi)) {
      ketQua.push(hang.ten);
    }
  }
  return ketQua;
}

const bang: Nguoi[] = [
  { ten: "An", tuoi: 15 },
  { ten: "Binh", tuoi: 20 },
  { ten: "Chi", tuoi: 25 },
  { ten: "Dung", tuoi: 30 },
];

const dkVa: DieuKien = {
  loai: "va",
  trai: { loai: "so-sanh", toanTu: ">", nguong: 18 },
  phai: { loai: "so-sanh", toanTu: "<", nguong: 28 },
};
console.log(chonTheoDieuKien(bang, dkVa).join(","));
```

```typescript title=solution
interface Nguoi {
  ten: string;
  tuoi: number;
}

type DieuKien =
  | { loai: "so-sanh"; toanTu: ">" | "<"; nguong: number }
  | { loai: "va"; trai: DieuKien; phai: DieuKien }
  | { loai: "hoac"; trai: DieuKien; phai: DieuKien };

function danhGia(dk: DieuKien, tuoi: number): boolean {
  if (dk.loai === "so-sanh") {
    return dk.toanTu === ">" ? tuoi > dk.nguong : tuoi < dk.nguong;
  }
  if (dk.loai === "va") {
    return danhGia(dk.trai, tuoi) && danhGia(dk.phai, tuoi);
  }
  return danhGia(dk.trai, tuoi) || danhGia(dk.phai, tuoi);
}

function chonTheoDieuKien(bang: Nguoi[], dk: DieuKien): string[] {
  const ketQua: string[] = [];
  for (const hang of bang) {
    if (danhGia(dk, hang.tuoi)) {
      ketQua.push(hang.ten);
    }
  }
  return ketQua;
}

const bang: Nguoi[] = [
  { ten: "An", tuoi: 15 },
  { ten: "Binh", tuoi: 20 },
  { ten: "Chi", tuoi: 25 },
  { ten: "Dung", tuoi: 30 },
];

const dkVa: DieuKien = {
  loai: "va",
  trai: { loai: "so-sanh", toanTu: ">", nguong: 18 },
  phai: { loai: "so-sanh", toanTu: "<", nguong: 28 },
};
console.log(chonTheoDieuKien(bang, dkVa).join(","));
```

```typescript title=test
const bang2: Nguoi[] = [
  { ten: "An", tuoi: 15 },
  { ten: "Binh", tuoi: 20 },
  { ten: "Chi", tuoi: 25 },
  { ten: "Dung", tuoi: 30 },
];

const dkVa2: DieuKien = {
  loai: "va",
  trai: { loai: "so-sanh", toanTu: ">", nguong: 18 },
  phai: { loai: "so-sanh", toanTu: "<", nguong: 28 },
};
const ketQuaVa = chonTheoDieuKien(bang2, dkVa2);
console.log(ketQuaVa.join(","));
if (ketQuaVa.join(",") !== "Binh,Chi") throw new Error("dieu kien VA phai khop SurrealDB that: Binh,Chi");

const dkHoac2: DieuKien = {
  loai: "hoac",
  trai: { loai: "so-sanh", toanTu: "<", nguong: 18 },
  phai: { loai: "so-sanh", toanTu: ">", nguong: 28 },
};
const ketQuaHoac = chonTheoDieuKien(bang2, dkHoac2);
if (ketQuaHoac.join(",") !== "An,Dung") throw new Error("dieu kien HOAC phai khop SurrealDB that: An,Dung");
```

:::hints
- kind: attention
  body: "VA (AND) dung khi CA HAI ve con deu dung -- dung && de ghep hai loi goi danhGia de quy, mot dong."
- kind: strategy
  body: "return danhGia(dk.trai, tuoi) && danhGia(dk.phai, tuoi);"
- kind: one-line
  body: "return danhGia(dk.trai, tuoi) && danhGia(dk.phai, tuoi);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "Binh,Chi"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
AND/OR khớp giữa hai bộ máy. Ráp trọn một pipeline nhỏ đối chiếu —
kết hợp MỌI kỹ thuật đã xác nhận — trông ra sao?
::::

::::reflect{#nghi-lai}
`danhGia` (đệ quy TRÊN cây điều kiện, chia theo `loai`) chính LÀ
`danh_gia_ca_cay` (q08 bài 4) viết LẠI bằng TypeScript — CẤU trúc
giống ĐẾN từng nhánh (`so-sanh` ứng VỚI `SoSanh`, `va`/`hoac` ứng
VỚI `VaNut`/`HoacNut`). Kết quả khớp CHÍNH xác VỚI SurrealDB thật
trên CẢ điều kiện đơn (bài 2) LẪN điều kiện ghép (bài NÀY) xác nhận:
q07's `CayAst` KHÔNG phải một mô hình "đơn giản HOÁ" — nó nắm ĐÚNG
ngữ nghĩa boolean THẬT của SQL/SurrealQL.
::::

::::checkpoint{mastery=0.85}
::::
