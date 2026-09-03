---
id: toan.tap-hop-quan-he-anh-xa.hieu-hai-tap
title: Hiệu của hai tập hợp
summary: "`−` là ký hiệu toán của `-` đã biết (T1.4) — A − B gồm phần tử thuộc A mà KHÔNG thuộc B. Khác `∪`/`∩`: KHÔNG đối xứng — `A − B` và `B − A` thường là hai tập KHÁC nhau."
locale: vi
track: toan
module: tap-hop-quan-he-anh-xa
order: 11
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.set-difference]
requires: [math.set-disjoint, core.set-difference, ctrl.comparison]
concepts: [math.hieu, math.khong-doi-xung]
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
Byte muốn biết luống nào trồng cà chua mà KHÔNG trồng xà lách. `∪`
gộp, `∩` giữ chung — phép NÀO lấy đúng "có cái này, không cái kia"?
::::

::::explain{#hieu-la-gi}
**`−`** (hiệu) — ký hiệu toán của `-` đã biết (T1.4 bài 29). `A − B`
gồm những phần tử thuộc A **NHƯNG KHÔNG** thuộc B:

```python title=readonly
luong_1 = {"cà chua", "xà lách"}
luong_3 = {"xà lách", "cà rốt"}

print(sorted(luong_1 - luong_3))
```

```text title=readonly
['cà chua']
```

`luống_1 − luống_3` giữ lại đúng `cà chua` — phần tử CÓ Ở `luống_1`
mà KHÔNG có Ở `luống_3`. `xà lách` (có Ở CẢ HAI) bị LOẠI, đúng nghĩa
"riêng của A".
::::

::::example{#khong-doi-xung}
Đảo chiều `−` — kết quả KHÁC hẳn, KHÔNG như `∪`/`∩` (bài 8-9 đối
xứng):

```python title=readonly
luong_1 = {"cà chua", "xà lách"}
luong_3 = {"xà lách", "cà rốt"}

print(sorted(luong_3 - luong_1))
```

```text title=readonly
['cà rốt']
```

`luống_3 − luống_1` giữ `cà rốt` (riêng của `luống_3`), KHÔNG PHẢI
`cà chua` (kết quả của `luống_1 − luống_3` Ở TRÊN). Hai phép trừ, hai
kết quả HOÀN TOÀN khác nhau.
::::

::::predict{#doan-hieu-khong-doi-xung commitOnce}
Byte kiểm tra CHÍNH XÁC `−` có "đối xứng" như `∪`/`∩` không:

```python
luong_1 = {"cà chua", "xà lách"}
luong_3 = {"xà lách", "cà rốt"}

print((luong_1 - luong_3) == (luong_3 - luong_1))
```

Dòng cuối in ra gì?

:::opt{correct}
`False`
:::

:::opt
`True` — vì cả HAI phép trừ đều LOẠI BỎ đúng phần TỬ CHUNG (`xà lách`)
khỏi phép tính, nên phần CÒN LẠI sau khi trừ PHẢI giống nhau, bất kể
trừ theo chiều nào
::why
Gần đúng ở việc bạn nhận RA `xà lách` (phần TỬ CHUNG) bị LOẠI khỏi CẢ
HAI phép tính — một quan sát đúng về VAI TRÒ của phần tử chung.

Chỗ lệch: "loại bỏ phần chung" KHÔNG PHẢI TOÀN BỘ câu chuyện — `A − B`
GIỮ LẠI phần RIÊNG của A (`cà chua`), CÒN `B − A` GIỮ LẠI phần RIÊNG
của B (`cà rốt`) — HAI phần RIÊNG hoàn toàn KHÁC NHAU, không phải
CÙNG một "phần còn lại". `{cà chua} == {cà rốt}` LÀ `False`.
::
:::

:::opt
Máy báo lỗi biên dịch — so sánh `==` giữa HAI biểu thức trừ tập hợp
(`luong_1 - luong_3` VÀ `luong_3 - luong_1`) đòi hỏi phải TÍNH riêng
mỗi vế ra một BIẾN trước, KHÔNG được viết trực tiếp bên trong `==`
::why
Gần đúng ở việc bạn để ý biểu thức Ở CẢ HAI vế của `==` LÀ phép TÍNH
(trừ tập hợp), KHÔNG PHẢI một biến đơn — một quan sát đúng về HÌNH
THỨC biểu thức.

Chỗ lệch: Python hoàn toàn cho phép SO SÁNH TRỰC TIẾP hai BIỂU THỨC
BẤT KỲ bằng `==`, không cần gán ra biến trước (`(3 + 4) == (2 + 5)`
cũng hợp lệ tương tự). Biên dịch sạch, chạy sạch — mỗi vế được TÍNH
xong RỒI mới ĐEM so sánh.
::
:::
::::

::::code{#viet_rau_doc_quyen}
Viết `rau_doc_quyen(vuon, ten_muc_tieu)` — trả về tập hợp loại rau MÀ
CHỈ luống `ten_muc_tieu` có, KHÔNG luống nào khác trong `vuon` có.

```python title=starter
def rau_doc_quyen(vuon, ten_muc_tieu):
    ket_qua = vuon[ten_muc_tieu]
    for ten_luong in vuon:
        if ten_luong != ten_muc_tieu:
            ket_qua = ___
    return ket_qua


vuon = {
    "luong_1": {"cà chua", "xà lách"},
    "luong_2": {"xà lách", "cải bó xôi"},
    "luong_3": {"cà rốt"},
}

print(sorted(rau_doc_quyen(vuon, "luong_1")))
```

```python title=solution
def rau_doc_quyen(vuon, ten_muc_tieu):
    ket_qua = vuon[ten_muc_tieu]
    for ten_luong in vuon:
        if ten_luong != ten_muc_tieu:
            ket_qua = ket_qua - vuon[ten_luong]
    return ket_qua


vuon = {
    "luong_1": {"cà chua", "xà lách"},
    "luong_2": {"xà lách", "cải bó xôi"},
    "luong_3": {"cà rốt"},
}

print(sorted(rau_doc_quyen(vuon, "luong_1")))
```

```python title=test
assert rau_doc_quyen(vuon, "luong_3") == {"cà rốt"}, "ca rot chi co o luong_3"
assert rau_doc_quyen(vuon, "luong_2") == {"cải bó xôi"}, "cai bo xoi chi co o luong_2 (xa lach bi tru vi luong_1 cung co)"
vuon2 = {"a": {"x", "y"}, "b": {"x"}}
assert rau_doc_quyen(vuon2, "a") == {"y"}, "x bi tru vi b cung co, chi con y"
assert rau_doc_quyen(vuon2, "b") == set(), "moi thu b co (x) deu bi a chia se -- khong con gi doc quyen"
```

:::hints
- kind: attention
  body: "Moi luot lap qua luong KHAC (khong phai ten_muc_tieu), TRU di tap hop cua luong do khoi ket_qua."
- kind: strategy
  body: "ket_qua - vuon[ten_luong]"
- kind: one-line
  body: "___ = ket_qua - vuon[ten_luong]"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: moi luot lap qua LUONG KHAC phai TRU tap hop cua luong do khoi ket_qua (dung phep -) -- thieu ket_qua cu se lam mat ket qua cac luot truoc
  requireAst:
  - kind: uses-operator, target: '-', min: 1
  - kind: uses-name, target: ket_qua, min: 2
  - kind: uses-name, target: ten_luong, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\['cà chua'\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`−` không đối xứng — riêng của A khác hẳn riêng của B. Bài sau: chốt
một "vũ trụ" để hỏi "cái gì KHÔNG có trong tập này, TRONG TOÀN vườn?"
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cả vườn Byte trồng đúng bảy loại rau. "Phần bù của luống 1" là những
loại rau CẢ VƯỜN có mà luống 1 không trồng. Đổi "vũ trụ" từ "cả vườn
Byte" sang "cả vườn của Lan" — phần bù của luống 1 có đổi theo không?
::::

::::checkpoint{mastery=0.8}
::::
