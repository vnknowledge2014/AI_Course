---
id: toan.tap-hop-quan-he-anh-xa.giao-hai-tap
title: Giao của hai tập hợp
summary: "`∩` là ký hiệu toán của `&` đã biết (T1.4) — A ∩ B gồm phần tử thuộc CẢ HAI, đúng nghĩa \"và\" (T2.3). Một tập hợp RỖNG là falsy — kiểm \"có chung gì không\" chỉ cần `if a & b:`, không cần so `== set()`."
locale: vi
track: toan
module: tap-hop-quan-he-anh-xa
order: 9
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.set-intersection]
requires: [math.set-union, core.set-intersection, core.truthiness]
concepts: [math.giao, math.tap-hop-rong-la-falsy]
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
`∪` (bài 8) gộp mọi thứ CÓ MẶT. Byte muốn hỏi câu ngược — luống 1 VÀ
luống 3 CÙNG trồng những loại rau nào?
::::

::::explain{#giao-la-gi}
**`∩`** (giao) là ký hiệu toán của `&` đã biết (T1.4 bài 28). `A ∩ B`
gồm những phần tử thuộc **CẢ HAI**, đúng nghĩa "và" của T2.3 bài 4.

```python title=readonly
luong_1 = {"cà chua", "xà lách", "cà rốt"}
luong_3 = {"xà lách", "cà rốt", "khoai lang"}

print(sorted(luong_1 & luong_3))
```

```text title=readonly
['cà rốt', 'xà lách']
```

`luống_1 ∩ luống_3` GIỮ đúng những loại rau CÓ MẶT Ở CẢ HAI luống —
`cà rốt` VÀ `xà lách`. `cà chua` (chỉ luống 1) và `khoai lang` (chỉ
luống 3) bị LOẠI, vì chúng KHÔNG "cả hai".
::::

::::example{#giao-rong-la-khong-chung-gi}
Hai luống KHÔNG chung loại rau nào — `∩` trả về tập RỖNG, không phải
lỗi:

```python title=readonly
luong_4 = {"bí đỏ"}
luong_6 = {"khoai lang"}

print(luong_4 & luong_6)
print(luong_4 & luong_6 == set())
```

```text title=readonly
set()
True
```

`luống_4 ∩ luống_6` LÀ `∅` (in ra `set()`, cách Python HIỂN THỊ một
tập rỗng — luôn AN TOÀN in trực tiếp, VÌ tập rỗng KHÔNG hề có thứ tự
nào để mà lộn xộn). So `== set()` xác nhận: đúng LÀ tập rỗng.
::::

::::predict{#doan-kiem-tra-truthy commitOnce}
Byte muốn biết NHANH hai luống CÓ chung gì không — không cần biết
CHÍNH XÁC chung LOẠI nào, chỉ cần "có hay không":

```python
luong_1 = {"cà chua", "xà lách", "cà rốt"}
luong_3 = {"xà lách", "cà rốt", "khoai lang"}
luong_4 = {"bí đỏ"}
luong_6 = {"khoai lang"}

if luong_1 & luong_3:
    print("co chung")
else:
    print("khong chung")

if luong_4 & luong_6:
    print("co chung")
else:
    print("khong chung")
```

Hai dòng cuối in ra gì?

:::opt{correct}
`co chung`, rồi `khong chung`
:::

:::opt
Máy báo lỗi biên dịch — `if luong_1 & luong_3:` đặt MỘT tập hợp NGAY
sau `if`, mà `if` chỉ chấp nhận giá trị `True`/`False`, KHÔNG chấp
nhận một tập hợp trực tiếp
::why
Gần đúng ở việc bạn nhớ ĐÚNG `if` cần một giá trị Đ/S — một quan sát
đúng về CÚ PHÁP `if` nói CHUNG.

Chỗ lệch: Python KHÔNG đòi `if` phải nhận ĐÚNG `True`/`False` — nó
chấp nhận BẤT KỲ giá trị nào, rồi TỰ hỏi giá trị đó có "rỗng/`0`/
`False`" (falsy, đã học `core.truthiness` T1.1) hay KHÔNG. Một `set`
CÓ phần tử LÀ truthy (coi NHƯ `True`), một `set` RỖNG LÀ falsy (coi
NHƯ `False`). `luống_1 & luống_3` KHÔNG rỗng → truthy → nhánh
`"co chung"` chạy. Biên dịch sạch.
::
:::

:::opt
`khong chung`, rồi `co chung` — vì `if` đọc GIÁ TRỊ ĐẦU TIÊN trong tập
hợp giao (nếu nó "nhỏ" theo thứ tự chữ cái thì hiểu LÀ falsy, "lớn"
thì hiểu LÀ truthy), và điều đó đảo ngược THỨ TỰ hai kết quả mong đợi
::why
Gần đúng ở việc bạn nghĩ TỚI khả năng `if` "nhìn vào" NỘI DUNG cụ thể
của tập hợp theo cách NÀO đó — một suy đoán có VẺ hợp lý khi chưa
chắc luật thật.

Chỗ lệch: `if` trên MỘT tập hợp KHÔNG hề nhìn vào TỪNG phần tử hay thứ
tự chữ cái — nó CHỈ hỏi ĐÚNG một câu: "tập hợp này CÓ RỖNG hay không".
`luống_1 & luống_3` có phần tử (`cà rốt`, `xà lách`) → truthy →
`"co chung"`. `luống_4 & luống_6` LÀ `∅` → falsy → `"khong chung"`.
Đúng thứ tự ĐÃ cho, không đảo.
::
:::
::::

::::code{#viet_luong_co_chung_voi}
Viết `luong_co_chung_voi(muc_tieu, vuon)` — trả về danh sách tên
những luống có `∩ muc_tieu ≠ ∅` (chung ÍT NHẤT một loại rau với
`muc_tieu`).

```python title=starter
def luong_co_chung_voi(muc_tieu, vuon):
    ket_qua = []
    for ten_luong in vuon:
        if ___:
            ket_qua.append(ten_luong)
    return ket_qua


vuon = {
    "luong_1": {"cà chua", "xà lách"},
    "luong_2": {"xà lách", "cải bó xôi"},
    "luong_3": {"khoai lang"},
}

print(sorted(luong_co_chung_voi({"xà lách"}, vuon)))
```

```python title=solution
def luong_co_chung_voi(muc_tieu, vuon):
    ket_qua = []
    for ten_luong in vuon:
        if muc_tieu & vuon[ten_luong]:
            ket_qua.append(ten_luong)
    return ket_qua


vuon = {
    "luong_1": {"cà chua", "xà lách"},
    "luong_2": {"xà lách", "cải bó xôi"},
    "luong_3": {"khoai lang"},
}

print(sorted(luong_co_chung_voi({"xà lách"}, vuon)))
```

```python title=test
assert luong_co_chung_voi({"khoai lang"}, vuon) == ["luong_3"], "chi luong_3 co khoai lang"
assert luong_co_chung_voi({"bí đỏ"}, vuon) == [], "khong luong nao co bi do"
assert luong_co_chung_voi(set(), vuon) == [], "muc tieu rong -- khong the chung voi ai"
assert sorted(luong_co_chung_voi({"xà lách", "khoai lang"}, vuon)) == ["luong_1", "luong_2", "luong_3"], "moi luong deu chung it nhat mot trong hai loai"
```

:::hints
- kind: attention
  body: "Dieu kien if kiem TRUTHY cua phep giao (&) giua muc_tieu va tap hop cua luong -- khong can so == set()."
- kind: strategy
  body: "muc_tieu & vuon[ten_luong]"
- kind: one-line
  body: "___ = muc_tieu & vuon[ten_luong]"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: dieu kien phai kiem TRUTHY cua phep giao (&) giua muc_tieu va vuon[ten_luong] -- dung == de so voi set() cung dung nhung bai nay muon ban dung cach ngan gon hon (tap hop rong la falsy)
  requireAst:
  - kind: uses-name, target: muc_tieu, min: 1
  - kind: uses-name, target: ten_luong, min: 1
  forbidAst:
  - kind: uses-operator, target: '=='
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\['luong_1', 'luong_2'\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tập rỗng là falsy — `if a & b:` là cách hỏi "có chung gì không" ngắn
gọn nhất. Bài sau: đặt tên riêng cho quan hệ "không chung gì".
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Có cặp luống nào mà phép `∩` ra `∅` không? Nếu có, hai luống ấy quan hệ
gì với nhau — và tên riêng cho quan hệ "không chung gì" đó là gì?
::::

::::checkpoint{mastery=0.8}
::::
