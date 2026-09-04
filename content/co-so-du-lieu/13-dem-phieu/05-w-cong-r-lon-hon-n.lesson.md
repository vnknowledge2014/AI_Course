---
id: co-so-du-lieu.dem-phieu.w-cong-r-lon-hon-n
title: "W cộng R lớn hơn N"
summary: "demSoCapKhongGiao liệt kê HẾT mọi cặp (tập ghi kích thước W, tập đọc kích thước R) trong N owner, đếm số cặp KHÔNG giao nhau. Với N=3,W=2,R=2 (W+R=4>N): 0/9 cặp không giao nhau — LUÔN đảm bảo. Với N=3,W=2,R=1 (W+R=3, KHÔNG lớn hơn N): 3/9 cặp không giao nhau — KHÔNG đảm bảo, dù W+R=N vừa đúng. Chỉ W+R>N (nghiêm ngặt lớn hơn) mới chắc chắn tập ghi và tập đọc luôn dùng chung ít nhất một node."
locale: vi
track: co-so-du-lieu
module: dem-phieu
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [db.w-cong-r-lon-hon-n]
requires: [db.doc-can-bao-nhieu-phieu-r]
concepts: [db.w-cong-r-lon-hon-n]
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
Bài 4 kết THÚC bằng một câu hỏi: `W` VÀ `R` được chọn ĐỘC lập, làm
sao chắc TẬP ghi và tập đọc LUÔN có điểm CHUNG? Đếm HẾT mọi khả năng
để trả LỜI, thay vì đoán.
::::

::::explain{#dem-so-cap-khong-giao}
`demSoCapKhongGiao(n, w, r)` liệt KÊ hết mọi tập con kích thước `w`
(tập GHI có THỂ) VÀ mọi tập con kích thước `r` (tập ĐỌC có thể)
trong `n` owner, RỒI đếm số CẶP (tập ghi, tập đọc) hoàn toàn KHÔNG
giao nhau:

```typescript title=readonly
function taoToHop(n: number, k: number): number[][] {
  const ketQua: number[][] = [];
  function duyet(batDau: number, hienTai: number[]) {
    if (hienTai.length === k) { ketQua.push([...hienTai]); return; }
    for (let i = batDau; i < n; i++) {
      hienTai.push(i);
      duyet(i + 1, hienTai);
      hienTai.pop();
    }
  }
  duyet(0, []);
  return ketQua;
}

function demSoCapKhongGiao(n: number, w: number, r: number): number {
  const dsW = taoToHop(n, w);
  const dsR = taoToHop(n, r);
  let dem = 0;
  for (const tapW of dsW) {
    for (const tapR of dsR) {
      const giaoNhau = tapW.some((x) => tapR.includes(x));
      if (!giaoNhau) dem++;
    }
  }
  return dem;
}

console.log("N=3,W=2,R=2 (W+R=4>3):", demSoCapKhongGiao(3, 2, 2));
console.log("N=3,W=1,R=1 (W+R=2):", demSoCapKhongGiao(3, 1, 1));
console.log("N=3,W=2,R=1 (W+R=3):", demSoCapKhongGiao(3, 2, 1));
```

```text title=readonly
N=3,W=2,R=2 (W+R=4>3): 0
N=3,W=1,R=1 (W+R=2): 6
N=3,W=2,R=1 (W+R=3): 3
```

VỚI `N=3, W=2, R=2` (`W+R=4`, LỚN hơn `N=3`): trong ĐÚNG `9` cặp có
thể (`3` tập ghi × `3` tập đọc), KHÔNG cặp NÀO thiếu giao nhau —
`0/9`. Đây KHÔNG phải may mắn — LÀ hệ quả của nguyên lý "chuồng bồ
CÂU": hai tập con kích THƯỚC `2` trong MỘT tập `3` phần tử LUÔN phải
chồng LẤN Ở ít nhất một phần tử, vì `2+2=4` VƯỢT quá `3` chỗ CÓ thể
xếp riêng.
::::

::::example{#w-cong-r-bang-n-khong-du}
`N=3, W=2, R=1` (`W+R=3`, ĐÚNG bằng `N`, KHÔNG lớn hơn): `3` TRÊN
`9` cặp (`33%`) KHÔNG giao nhau. VÍ dụ: tập ghi `{alpha, gamma}`
(owner thứ nhất VÀ thứ ba), tập đọc CHỈ hỏi `{beta}` (owner thứ
hai) — không có điểm CHUNG. `W+R=N` (bằng NHAU, không hơn) đủ CHỖ để
hai tập con tách RỜI hoàn toàn — chỉ KHI `W+R` thực sự VƯỢT quá `N`
mới hết chỗ ĐỂ tách rời.
::::

::::predict{#doan-n5 commitOnce}
`N=5` (RF=5, cụm LỚN hơn). So sánh `demSoCapKhongGiao(5, 3, 3)`
(`W+R=6>5`) VÀ `demSoCapKhongGiao(5, 2, 2)` (`W+R=4`, KHÔNG lớn hơn
`5`) — kết quả NÀO đúng?

:::opt{correct}
`demSoCapKhongGiao(5,3,3) = 0` (đảm bảo TUYỆT đối), CÒN
`demSoCapKhongGiao(5,2,2) = 30` (`30` TRÊN `100` cặp CÓ thể không
giao nhau) — quy TẮC "chỉ `W+R>N` mới đảm bảo" áp DỤNG y hệt Ở quy
mô LỚN hơn, không riêng `N=3`
:::

:::opt
Cả hai đều CHO `0` — cụm CÀNG lớn (`N=5`) CÀNG khó CHO hai tập con
tách rời HOÀN toàn, bất kể `W,R` LÀ bao nhiêu
::why
Trực giác "cụm lớn HƠN thì an toàn hơn" ĐÚNG cho một số khía cạnh
khác (VÍ dụ chịu được nhiều node sập HƠN) — nhưng KHÔNG áp dụng cho
CHÍNH câu hỏi "hai tập con CÓ BẮT buộc giao nhau không".

Chỗ lệch: điều kiện đảm BẢO giao nhau CHỈ phụ thuộc và HIỆU
`(w+r) - n`, KHÔNG phụ thuộc và độ LỚN tuyệt đối của `n`. VỚI
`n=5, w=2, r=2`: `w+r=4 < 5`, VẪN còn dư `1` "chỗ TRỐNG" để xếp hai
tập con hoàn toàn tách rời (VÍ dụ tập ghi chiếm `2` owner ĐẦU, tập
đọc chiếm `2` owner CUỐI, dư đúng `1` owner Ở giữa không thuộc bên
nào) — CHÍNH xác `30` trên `100` cặp làm được điều NÀY.
::
:::
::::

::::code{#viet_dem_so_cap_khong_giao}
Hoàn thiện `demSoCapKhongGiao` — VỚI mỗi cặp (tập ghi, tập đọc), nếu
KHÔNG giao nhau thì đếm.

```typescript title=starter
function taoToHop(n: number, k: number): number[][] {
  const ketQua: number[][] = [];
  function duyet(batDau: number, hienTai: number[]) {
    if (hienTai.length === k) { ketQua.push([...hienTai]); return; }
    for (let i = batDau; i < n; i++) {
      hienTai.push(i);
      duyet(i + 1, hienTai);
      hienTai.pop();
    }
  }
  duyet(0, []);
  return ketQua;
}

function demSoCapKhongGiao(n: number, w: number, r: number): number {
  const dsW = taoToHop(n, w);
  const dsR = taoToHop(n, r);
  let dem = 0;
  for (const tapW of dsW) {
    for (const tapR of dsR) {
      const giaoNhau = tapW.some((x) => tapR.includes(x));
      ___
    }
  }
  return dem;
}

console.log(demSoCapKhongGiao(3, 2, 2));
```

```typescript title=solution
function taoToHop(n: number, k: number): number[][] {
  const ketQua: number[][] = [];
  function duyet(batDau: number, hienTai: number[]) {
    if (hienTai.length === k) { ketQua.push([...hienTai]); return; }
    for (let i = batDau; i < n; i++) {
      hienTai.push(i);
      duyet(i + 1, hienTai);
      hienTai.pop();
    }
  }
  duyet(0, []);
  return ketQua;
}

function demSoCapKhongGiao(n: number, w: number, r: number): number {
  const dsW = taoToHop(n, w);
  const dsR = taoToHop(n, r);
  let dem = 0;
  for (const tapW of dsW) {
    for (const tapR of dsR) {
      const giaoNhau = tapW.some((x) => tapR.includes(x));
      if (!giaoNhau) dem++;
    }
  }
  return dem;
}

console.log(demSoCapKhongGiao(3, 2, 2));
```

```typescript title=test
if (demSoCapKhongGiao(3, 2, 2) !== 0) throw new Error("N=3,W=2,R=2 (W+R=4>3) phai dam bao 0 cap khong giao nhau");
if (demSoCapKhongGiao(3, 1, 1) !== 6) throw new Error("N=3,W=1,R=1 (W+R=2) phai co dung 6/9 cap khong giao nhau");
if (demSoCapKhongGiao(3, 2, 1) !== 3) throw new Error("N=3,W=2,R=1 (W+R=3, KHONG lon hon N) phai co dung 3/9 cap khong giao nhau -- bang khong du");
if (demSoCapKhongGiao(5, 3, 3) !== 0) throw new Error("N=5,W=3,R=3 (W+R=6>5) phai dam bao 0 cap khong giao nhau");
if (demSoCapKhongGiao(5, 2, 2) !== 30) throw new Error("N=5,W=2,R=2 (W+R=4, khong lon hon 5) phai co dung 30/100 cap khong giao nhau");
```

:::hints
- kind: attention
  body: "Neu KHONG giao nhau (giaoNhau la false) thi dem tang len 1 -- mot dong."
- kind: strategy
  body: "if (!giaoNhau) dem++;"
- kind: one-line
  body: "if (!giaoNhau) dem++;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "0"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`W+R>N` bảo đảm giao NHAU — nhưng "giao nhau" mới chỉ nghĩa LÀ CÙNG
một node phản hồi CẢ hai phía. Nếu owner ĐÓ giữ nhiều BẢN ghi khác
thời điểm nhau, cái nào LÀ "đúng"?
::::

::::reflect{#nghi-lai}
`demSoCapKhongGiao` không dùng BẤT kỳ hàm nào từ preference list HAY
quorum Ở các bài trước — nó CHỨNG minh bằng cách liệt KÊ hết mọi khả
năng (`taoToHop`), ĐÚNG tinh thần "đếm thay VÌ đoán" đã dùng xuyên
suốt các bài toán RỜI rạc Ở đầu khoá. Kết luận rút RA (`W+R>N` mới
đảm bảo giao nhau) LÀ nền tảng LÝ thuyết cho MỌI lựa chọn `W`, `R`
thực TẾ — nhưng chỉ đảm bảo "CÓ ít nhất một node CHUNG", chưa nói gì
về việc node đó giữ giá trị NÀO nếu có NHIỀU phiên bản khác nhau.
::::

::::checkpoint{mastery=0.85}
::::
