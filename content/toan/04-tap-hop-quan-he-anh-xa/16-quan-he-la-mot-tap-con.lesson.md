---
id: toan.tap-hop-quan-he-anh-xa.quan-he-la-mot-tap-con
title: Quan hệ là một tập con
summary: "Quan hệ = một tập con của A × B — không phải khái niệm tách rời, mà CHÍNH LÀ một tập hợp (bài 1-13) gồm những cặp (bài 14-15) THẬT xảy ra. \"Luống nào tưới ngày nào\" là một quan hệ giữa Luống và Ngày."
locale: vi
track: toan
module: tap-hop-quan-he-anh-xa
order: 16
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.relation]
requires: [math.cartesian-product, math.subset]
concepts: [math.quan-he, math.tap-con-cua-tich]
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
49 cặp (luống, ngày) LÀ mọi khả năng. Nhưng luống 1 CHỈ tưới thứ Hai
và thứ Năm THẬT sự — hai cặp trong 49. Tập con "chỉ những cặp thật"
ấy gọi là gì?
::::

::::explain{#quan-he-la-gi}
Gọi LÀ **quan hệ**. **Quan hệ = một tập con của `A × B`** — KHÔNG
PHẢI một khái niệm hoàn toàn MỚI, mà CHÍNH LÀ một tập hợp (bài 1-13)
gồm những cặp (bài 14-15) THẬT SỰ xảy ra:

```python title=readonly
def la_tap_con(a, b):
    return all(x in b for x in a)

def tich_descartes(a, b):
    ket_qua = set()
    for x in a:
        for y in b:
            ket_qua.add((x, y))
    return ket_qua


luong = {"luong_1", "luong_2"}
ngay = {"Hai", "Ba"}
tich = tich_descartes(luong, ngay)

tuoi_nuoc = {("luong_1", "Hai"), ("luong_2", "Ba")}
print(la_tap_con(tuoi_nuoc, tich))
```

```text title=readonly
True
```

`tuoi_nuoc` (lịch tưới THẬT — CHỈ hai cặp) LÀ một tập CON của `tich`
(mọi khả NĂNG — bốn cặp). Đây chính LÀ định nghĩa: MỘT quan hệ giữa
Luống VÀ Ngày KHÔNG LÀ gì khác NGOÀI một tập con của `Luống × Ngày`.
Bạn ĐÃ có SẴN toàn bộ công cụ để kiểm điều ĐÓ — `la_tap_con` (bài 6),
`tich_descartes` (bài 15) — KHÔNG cần học thêm cơ chế mới.
::::

::::example{#quan-he-sai-bi-loai}
MỘT tập cặp KHÔNG phải tập con của `A × B` thì KHÔNG PHẢI một quan hệ
hợp lệ GIỮA `A` và `B`:

```python title=readonly
def la_tap_con(a, b):
    return all(x in b for x in a)
def tich_descartes(a, b):
    ket_qua = set()
    for x in a:
        for y in b:
            ket_qua.add((x, y))
    return ket_qua

luong = {"luong_1", "luong_2"}
ngay = {"Hai", "Ba"}
tich = tich_descartes(luong, ngay)

sai = {("luong_1", "Hai"), ("luong_3", "Ba")}
print(la_tap_con(sai, tich))
```

```text title=readonly
False
```

`sai` chứa cặp `("luong_3", "Ba")` — NHƯNG `luong_3` KHÔNG thuộc
`luong` (chỉ CÓ `luong_1`/`luong_2`) — cặp NÀY KHÔNG thể LÀ MỘT khả
NĂNG của `luong × ngày`. `sai` KHÔNG PHẢI tập con của `tich`, nên
KHÔNG PHẢI một quan hệ hợp lệ giữa `luong` và `ngay`.
::::

::::predict{#doan-quan-he-rong commitOnce}
Byte thử một "lịch tưới" TRỐNG HOÀN TOÀN — chưa tưới NGÀY nào:

```python
def la_tap_con(a, b):
    return all(x in b for x in a)
def tich_descartes(a, b):
    ket_qua = set()
    for x in a:
        for y in b:
            ket_qua.add((x, y))
    return ket_qua

luong = {"luong_1", "luong_2"}
ngay = {"Hai", "Ba"}
tich = tich_descartes(luong, ngay)

rong = set()
print(la_tap_con(rong, tich))
```

Dòng cuối in ra gì?

:::opt{correct}
`True`
:::

:::opt
`False` — vì một "quan hệ" mà KHÔNG chứa cặp NÀO thì KHÔNG THỂ tính
LÀ quan hệ hợp lệ — MỘT tập con phải CÓ ÍT NHẤT một phần tử để chứng
minh nó thật sự "liên quan" tới `A × B`
::why
Gần đúng ở việc bạn nghĩ TỚI trực giác "quan hệ" NÊN có ÍT NHẤT một
cặp để "làm gì đó" — một cách hiểu THÔNG THƯỜNG hợp lý về từ "quan
hệ" trong ĐỜI SỐNG.

Chỗ lệch: bài 6-7 ĐÃ xác lập rõ — `∅ ⊆` MỌI tập hợp, KHÔNG ngoại lệ
(chân lý rỗng: không phần tử nào để phản chứng). `rong` (`∅`) đương
NHIÊN LÀ tập con của `tich`, dù `tich` LỚN cỡ nào. Về mặt TOÁN, "lịch
tưới TRỐNG" (chưa tưới GÌ) VẪN LÀ một quan hệ hợp lệ giữa Luống và
Ngày — chỉ đơn giản LÀ quan hệ "không có cặp nào ĐANG đúng", KHÔNG
PHẢI "không hợp lệ".
::
:::

:::opt
Máy báo lỗi biên dịch — gọi `la_tap_con(rong, tich)` VỚI `rong` LÀ
`set()` HOÀN TOÀN trống đòi hỏi tham SỐ đầu phải CÓ ít nhất MỘT phần
tử để hàm `all()` BÊN TRONG có gì đó để LẶP qua
::why
Gần đúng ở việc bạn để ý `rong` KHÔNG có phần tử nào — một quan sát
đúng về DỮ LIỆU đầu vào.

Chỗ lệch: `all()` gọi TRÊN một generator KHÔNG sinh phần tử nào (đã
học bài 6: chân lý rỗng) trả VỀ `True` NGAY, KHÔNG cần Ở TRONG có gì
để lặp — đây LÀ hành vi CHUẨN, không phải lỗi. Biên dịch sạch, chạy
sạch.
::
:::
::::

::::code{#viet_la_quan_he_hop_le}
Viết `la_quan_he_hop_le(quan_he, a, b)` — kiểm tra `quan_he` có phải
MỘT tập con hợp lệ của `A × B` hay không, GHÉP LẠI `tich_descartes`
(bài 15) và `la_tap_con` (bài 6).

```python title=starter
def la_tap_con(a, b):
    return all(x in b for x in a)


def tich_descartes(a, b):
    ket_qua = set()
    for x in a:
        for y in b:
            ket_qua.add((x, y))
    return ket_qua


def la_quan_he_hop_le(quan_he, a, b):
    tich = ___
    return ___


luong = {"luong_1", "luong_2"}
ngay = {"Hai", "Ba"}

print(la_quan_he_hop_le({("luong_1", "Hai")}, luong, ngay))
```

```python title=solution
def la_tap_con(a, b):
    return all(x in b for x in a)


def tich_descartes(a, b):
    ket_qua = set()
    for x in a:
        for y in b:
            ket_qua.add((x, y))
    return ket_qua


def la_quan_he_hop_le(quan_he, a, b):
    tich = tich_descartes(a, b)
    return la_tap_con(quan_he, tich)


luong = {"luong_1", "luong_2"}
ngay = {"Hai", "Ba"}

print(la_quan_he_hop_le({("luong_1", "Hai")}, luong, ngay))
```

```python title=test
assert la_quan_he_hop_le({("luong_3", "Hai")}, luong, ngay) is False, "luong_3 khong thuoc luong -- khong hop le"
assert la_quan_he_hop_le(set(), luong, ngay) is True, "quan he rong van la tap con hop le"
assert la_quan_he_hop_le(tich_descartes(luong, ngay), luong, ngay) is True, "chinh tich descartes cung la mot quan he (moi kha nang) hop le"
```

:::hints
- kind: attention
  body: "Tinh tich = tich_descartes(a, b), roi kiem quan_he co la tap con cua tich khong bang la_tap_con."
- kind: strategy
  body: "tich_descartes(a, b) : la_tap_con(quan_he, tich)"
- kind: one-line
  body: "___ (tich) = tich_descartes(a, b)\n___ (ket qua) = la_tap_con(quan_he, tich)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai goi tich_descartes(a, b) de tinh tich TRUOC, roi goi la_tap_con(quan_he, tich) de kiem -- khong duoc bo qua buoc nao
  requireAst:
  - kind: uses-call, target: tich_descartes, min: 1
  - kind: uses-call, target: la_tap_con, min: 1
  - kind: uses-name, target: quan_he, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Quan hệ = tập con của tích Descartes — không khái niệm mới, chỉ ghép
lại cái đã có. Bài sau: nhìn quan hệ theo một cách khác — bảng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Quan hệ tưới nước viết ra là một ĐỐNG cặp rời rạc. Nhìn đống đó có DỄ
thấy "luống nào tưới NHIỀU ngày nhất" không, hay cần cách trình bày
khác?
::::

::::checkpoint{mastery=0.8}
::::
