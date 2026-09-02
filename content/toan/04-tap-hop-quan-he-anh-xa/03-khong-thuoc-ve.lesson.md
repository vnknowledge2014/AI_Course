---
id: toan.tap-hop-quan-he-anh-xa.khong-thuoc-ve
title: Không thuộc về
summary: "`∉` là ký hiệu toán của `not in` — viết tắt cho `not (x ∈ A)`, phủ định của ∈ đúng như `P`/`không P` (T2.3); phủ định của phủ định trả về đúng câu gốc."
locale: vi
track: toan
module: tap-hop-quan-he-anh-xa
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.set-not-element]
requires: [math.set-element, logic.not, core.list, core.list-append, ctrl.for-each]
concepts: [math.khong-thuoc-ve, math.phu-dinh-cua-phu-dinh]
gradingMatrix:
  web-chrome: [static, run, tests, output]
  web-firefox: [static, run, tests, output]
  macos: [static, run, tests, output]
  windows: [static, run, tests, output]
  linux: [static, run, tests, output]
  android: [static, run, tests, output]
  ios: [static, run, tests, output]
provenance:
  authoredBy: llm-assisted
  reviewed: false
---

::::byte{trigger=enter mood=curious pose=lean-in}
"Khoai tây `∈` luống_1" — Byte tra thấy sai. Viết "không có mặt" thì phải
gõ `not (... ∈ ...)` dài dòng à?
::::

::::explain{#ky-hieu-khong-thuoc}
Không cần dài dòng. **`∉`** là ký hiệu toán của `not in` — viết tắt cho
`not (x ∈ A)`, đúng như `P` và `không P` là hai câu phủ định nhau (T2.3
bài 3).

```python title=readonly
luong_1 = {"cà chua", "xà lách", "cà rốt"}

print("khoai tây" not in luong_1)
```

```text title=readonly
True
```

`khoai tây ∉ luống_1` đúng — khoai tây thật sự không nằm trong luống 1.
`∈` và `∉` LUÔN cho kết quả trái ngược nhau trên CÙNG một cặp (phần tử,
tập hợp): đúng một trong hai, không bao giờ cả hai cùng đúng, không bao
giờ cả hai cùng sai.
::::

::::example{#cung-mot-cau-hoi-hai-luong}
Cùng một loại rau, `∈` ở luống này thì `∉` ở luống kia — hai câu độc lập,
mỗi câu tự đúng hoặc tự sai:

```python title=readonly
luong_1 = {"cà chua", "xà lách", "cà rốt"}
luong_2 = {"xà lách", "cải bó xôi"}

print("cà chua" not in luong_1)
print("cà chua" not in luong_2)
```

```text title=readonly
False
True
```

`cà chua ∉ luống_1` sai (cà chua CÓ ở đó). `cà chua ∉ luống_2` đúng (cà
chua KHÔNG ở đó).
::::

::::predict{#doan-phu-dinh-hai-lan commitOnce}
Byte viết một câu có HAI chữ "không" chồng lên nhau:

```python
luong_1 = {"cà chua", "xà lách"}

print(not ("cà chua" not in luong_1))
```

Dòng cuối in ra gì?

:::opt{correct}
`True`
:::

:::opt
`False` — vì câu có HAI phủ định chồng lên nhau (`not` bọc ngoài,
`not in` ở trong), và hai lớp phủ định thì kết quả PHẢI giữ nguyên tính
"phủ định" của lớp trong cùng, tức vẫn ra `False`
::why
Gần đúng ở việc bạn ĐẾM ĐÚNG có hai chữ "không" (`not` VÀ `not in`)
trong câu — một quan sát đúng về HÌNH THỨC.

Chỗ lệch: hai lớp phủ định không "cộng dồn" thành phủ định mạnh hơn —
chúng TRIỆT TIÊU nhau, đúng luật "phủ định của phủ định trả về câu gốc"
(T2.3 bài 3, `noi-nguoc-lai-mot-cau`). Tính TỪ TRONG ra: `"cà chua" not
in luong_1` là `False` (cà chua CÓ trong luống, nên "không có mặt" là
sai). Bọc thêm `not` bên ngoài: `not False` là `True`. Hai phủ định
chồng lên đúng câu **có mặt** ban đầu — và cà chua đúng LÀ có mặt.
::
:::

:::opt
Máy báo lỗi biên dịch — Python cấm lồng `not` bên ngoài một biểu thức
đã có `not in` bên trong, vì đó là HAI toán tử phủ định chồng lên cùng
một chỗ
::why
Gần đúng ở việc bạn để ý cấu trúc lồng nhau — `not (... not in ...)` —
trông khác thường so với những câu `not`/`not in` đơn lẻ đã gặp.

Chỗ lệch: Python hoàn toàn cho phép lồng `not` bao NHIÊU lớp tuỳ ý quanh
BẤT KỲ biểu thức boolean nào, kể cả một biểu thức đã tự chứa `not`/
`not in`. Biên dịch sạch, chạy sạch — mỗi lớp `not` chỉ đơn giản lật
giá trị boolean của LỚP NGAY BÊN TRONG nó.
::
:::
::::

::::code{#viet_rau_con_thieu}
Viết `rau_con_thieu(can_trong, luong)` — `can_trong` là danh sách loại
rau CẦN có, `luong` là một tập hợp; hàm trả về danh sách những loại rau
trong `can_trong` mà `∉ luống` (còn thiếu, chưa trồng).

```python title=starter
def rau_con_thieu(can_trong, luong):
    ket_qua = []
    for rau in can_trong:
        if ___:
            ket_qua.append(rau)
    return ket_qua


luong_2 = {"xà lách", "cải bó xôi"}
can_trong = ["cà chua", "xà lách", "bí đỏ"]

print(rau_con_thieu(can_trong, luong_2))
```

```python title=solution
def rau_con_thieu(can_trong, luong):
    ket_qua = []
    for rau in can_trong:
        if rau not in luong:
            ket_qua.append(rau)
    return ket_qua


luong_2 = {"xà lách", "cải bó xôi"}
can_trong = ["cà chua", "xà lách", "bí đỏ"]

print(rau_con_thieu(can_trong, luong_2))
```

```python title=test
assert rau_con_thieu(["xà lách"], luong_2) == [], "xa lach da co san -- khong thieu gi"
assert rau_con_thieu([], luong_2) == [], "khong can trong gi thi khong thieu gi"
assert rau_con_thieu(["cà chua"], set()) == ["cà chua"], "luong rong -- moi thu can trong deu thieu"
assert rau_con_thieu(["a", "b", "c"], {"a", "b", "c"}) == [], "luong co du ca ba loai -- khong thieu gi"
assert rau_con_thieu(["a", "b", "c"], {"a"}) == ["b", "c"], "chi co a -- thieu b va c, theo dung thu tu can_trong"
```

:::hints
- kind: attention
  body: "Dieu kien if phai dung `not in`: kiem rau CO PHAI khong nam trong luong."
- kind: strategy
  body: "rau not in luong"
- kind: one-line
  body: "___ = rau not in luong"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: dieu kien phai dung DUNG toan tu `not in` de kiem rau con thieu -- dung `not (rau in luong)` cung dung ve mat logic nhung bai nay muon ban dung ky hieu ngan gon da hoc (not in), khong phai boc not ben ngoai
  requireAst:
  - kind: uses-operator, target: not in, min: 1
  - kind: uses-name, target: rau, min: 1
  - kind: uses-name, target: luong, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\['cà chua', 'bí đỏ'\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai lớp phủ định triệt tiêu nhau, đúng luật cũ. Bài sau: viết một tập hợp
mà KHÔNG liệt kê từng phần tử — nếu vườn có bảy trăm luống thì sao?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Byte có bảy luống. "Tập hợp những luống có trồng cà chua" — liệt kê tay
được, nhưng nếu vườn có bảy TRĂM luống thì sao? Có cách viết nào khác
liệt kê từng cái không?
::::

::::checkpoint{mastery=0.8}
::::
