---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.bo-xu-ly-goi-lai-tung-bien-the-lam-gi
title: "`BoXuLy` — một object nói TỪNG biến thể LÀM GÌ, không TỰ đệ quy"
summary: "interface BoXuLyExpr<R> { so: (giaTri: number) => R; cong: (trai: R, phai: R) => R; nhan: (trai: R, phai: R) => R; } — cong/nhan nhận trai: R, phai: R (KẾT QUẢ đã tính của nhánh con), không phải Expr thô. BoXuLy tự nó không hề gọi lại chính mình."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 43
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 9
teaches: [alg.handler-object-per-variant]
requires: [alg.recursion-pattern-repeats]
concepts: [alg.handler-object-per-variant]
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
Bài trước hỏi: viết một hàm đệ quy MỚI trên `Expr` cứ phải chép lại
y hệt phần đệ quy — có cách nào tách phần đó ra, viết một lần không?
Câu trả lời KHÔNG bắt đầu từ phần đệ quy. Nó bắt đầu từ phần NGƯỢC
LẠI: gói phần TÍNH TOÁN — thứ thật sự khác nhau giữa `tinh` và
`demNode` — vào một object riêng.
::::

::::explain{#dinh-nghia-bo-xu-ly}
```typescript
interface BoXuLyExpr<R> {
  so: (giaTri: number) => R;
  cong: (trai: R, phai: R) => R;
  nhan: (trai: R, phai: R) => R;
}

const boXuLyTinh: BoXuLyExpr<number> = {
  so: (giaTri) => giaTri,
  cong: (trai, phai) => trai + phai,
  nhan: (trai, phai) => trai * phai,
};

console.log(boXuLyTinh.so(5));
console.log(boXuLyTinh.cong(3, 4));
console.log(boXuLyTinh.nhan(3, 4));
```

```text
5
7
12
```

`interface BoXuLyExpr<R>` có ĐÚNG ba thuộc tính — khớp ĐÚNG ba biến
thể của `Expr` (bài 41): `so` ứng với node lá `"so"`, `cong`/`nhan`
ứng với hai node nhánh `"cong"`/`"nhan"`. Nhưng nhìn kỹ CHỮ KÝ: `so`
nhận `giaTri: number` — con số THÔ nằm ngay tại node lá. `cong` và
`nhan` lại nhận `trai: R`, `phai: R` — không phải `Expr` của nhánh
con, mà là KẾT QUẢ kiểu `R` đã được tính XONG cho nhánh đó rồi.

Để ý: cả ba hàm trong `BoXuLyExpr<R>` không hề nhắc tới `Expr` trong
chữ ký của mình — chỉ nhắc `number` (ở `so`) và `R` (ở `cong`/`nhan`).
`boXuLyTinh.so(5)`, `boXuLyTinh.cong(3, 4)`, `boXuLyTinh.nhan(3, 4)`
gọi được ngay với số trần, KHÔNG có một `Expr` nào ở đây cả. `BoXuLy`
không tự đi xuống cây, không tự gọi lại chính mình — nó chỉ là một
object trả lời hai câu: "gặp một SỐ thì làm gì" và "gặp HAI KẾT QUẢ
CON đã tính xong thì gộp ra sao".
::::

::::example{#hai-bo-xu-ly-khac-nhau}
CÙNG một `interface BoXuLyExpr<R>`, hai object khác nhau — khác cả
`R` lẫn cách ứng xử:

```typescript title=readonly
interface BoXuLyExpr<R> {
  so: (giaTri: number) => R;
  cong: (trai: R, phai: R) => R;
  nhan: (trai: R, phai: R) => R;
}

const boXuLyDemNode: BoXuLyExpr<number> = {
  so: () => 1,
  cong: (trai, phai) => trai + phai + 1,
  nhan: (trai, phai) => trai + phai + 1,
};

const boXuLyChuoi: BoXuLyExpr<string> = {
  so: (giaTri) => String(giaTri),
  cong: (trai, phai) => "(" + trai + " + " + phai + ")",
  nhan: (trai, phai) => "(" + trai + " * " + phai + ")",
};

console.log(boXuLyDemNode.so(999));
console.log(boXuLyDemNode.cong(boXuLyDemNode.so(1), boXuLyDemNode.so(2)));
console.log(boXuLyChuoi.cong(boXuLyChuoi.so(3), boXuLyChuoi.so(4)));
```

```text title=readonly
1
3
(3 + 4)
```

`boXuLyDemNode.so` BỎ QUA tham số hoàn toàn — chữ ký `() => 1` không
nhận `giaTri`, luôn trả `1` dù gọi `.so(999)` hay `.so(0)`. Đếm số
NODE không cần biết con số đó LÀ BAO NHIÊU, chỉ cần biết "có một node
lá ở đây". `boXuLyDemNode.cong(boXuLyDemNode.so(1), boXuLyDemNode.so(2))`
— dòng này CHÍNH TAY lồng hai lời gọi `.so(...)` vào bên trong một
lời gọi `.cong(...)`, mô phỏng hình dạng một `Expr` nhỏ (một node
`"cong"` với hai nhánh lá). Nhưng chính người viết mã quyết định lồng
thế nào — `boXuLyDemNode` không hề biết nó vừa "đếm" một cây gì.
`boXuLyChuoi` khớp CÙNG interface với `R = string`, gộp bằng cách
NỐI chuỗi thay vì cộng số — cùng khuôn, khác hoàn toàn hành vi.
::::

::::predict{#doan-goi-bo-xu-ly-truc-tiep commitOnce}
```typescript
interface BoXuLyExpr<R> {
  so: (giaTri: number) => R;
  cong: (trai: R, phai: R) => R;
  nhan: (trai: R, phai: R) => R;
}

const boXuLyDemNode: BoXuLyExpr<number> = {
  so: () => 1,
  cong: (trai, phai) => trai + phai + 1,
  nhan: (trai, phai) => trai + phai + 1,
};

console.log(boXuLyDemNode.so(999));
console.log(boXuLyDemNode.cong(boXuLyDemNode.so(1), boXuLyDemNode.so(2)));
```

Hai dòng in ra gì?

:::opt{correct}
`1` rồi `3`
:::

:::opt
`999` rồi `3` — vì `so` nhận tham số `giaTri: number`, nên nó phải
TRẢ VỀ đúng con số được truyền vào
::why
Gần đúng ở việc bạn nhớ ĐÚNG: `boXuLyTinh` (bài học phía trên)
THẬT SỰ có `so: (giaTri) => giaTri` — dùng đúng tham số truyền vào.

Chỗ lệch: `boXuLyDemNode` là một object KHÁC, với `so: () => 1` —
dấu ngoặc RỖNG `()`, không nhận tham số nào cả, LUÔN trả về `1` bất
kể gọi `.so(999)` hay `.so(1)`. Đếm số node không quan tâm giá trị
số là bao nhiêu. Dòng đầu in `1`, không phải `999`.
::
:::

:::opt
`1` rồi `2` — vì `cong(1, 1)` chỉ cộng hai nhánh con lại, `1 + 1 = 2`
::why
Gần đúng ở việc bạn tính đúng CẢ HAI lời gọi `.so(...)` bên trong đều
ra `1` (không phụ thuộc tham số) — quan sát đó đúng.

Chỗ lệch: `boXuLyDemNode.cong` là `(trai, phai) => trai + phai + 1`
— có thêm `+ 1` ở cuối, đếm luôn CHÍNH node `"cong"` này, không chỉ
cộng hai nhánh con. `cong(1, 1)` ra `1 + 1 + 1 = 3`, không phải `2`.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`BoXuLy` trả lời ĐÚNG một điều cho từng biến thể: gặp một SỐ thì làm
gì, gặp HAI KẾT QUẢ CON đã tính xong thì gộp ra sao. Nó chưa hề chạm
vào một `Expr` thật.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Ở ví dụ trên, CHÍNH BẠN phải tự tay lồng `.so(...)` vào bên trong
`.cong(...)` để mô phỏng hình dạng một `Expr` nhỏ — `BoXuLyExpr` không
tự đi xuống `trai`/`phai` của một `Expr` thật, nó không hề biết `Expr`
tồn tại. Ai sẽ làm việc ĐÓ — nhận một `Expr` thật, tự gọi xuống đúng
hai nhánh con, rồi đưa đúng kết quả vào `.cong`/`.nhan`?

Bài sau viết đúng hàm đó — dùng lại được cho BẤT KỲ `BoXuLyExpr` nào.
::::

::::checkpoint{mastery=0.8}
::::
