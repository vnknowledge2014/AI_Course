---
id: co-so-du-lieu.chiec-hop-giay-cua-byte.ghep-truong-thanh-ban-ghi
title: Ghép nhiều trường thành một bản ghi
summary: "Nối các chuỗi byte bằng + để ghép NHIỀU trường (id, tuổi, ...) thành MỘT bản ghi; đọc lại một trường bằng cắt LÁT [bắt_đầu:bắt_đầu+độ_dài] — sai độ rộng KHÔNG báo lỗi, nó ÂM THẦM trả về giá trị SAI."
locale: vi
track: co-so-du-lieu
module: chiec-hop-giay-cua-byte
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [db.fixed-width-record]
requires: [db.int-to-bytes]
concepts: [db.fixed-width-record]
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
Một BẢN ghi thật KHÔNG chỉ có MỘT con số — id NGƯỜI dùng, tuổi, VÀ
còn NỮA. MỖI trường ĐÃ LÀ byte (bài TRƯỚC) — ghép CHÚNG lại thành
MỘT bản ghi RA sao?
::::

::::explain{#ghep-bang-cong}
Toán tử `+` TRÊN hai `bytes` — NỐI chúng LẠI, giống HỆT nối `list`:

```python title=readonly
id_byte = (7).to_bytes(2, 'big')
tuoi_byte = (25).to_bytes(1, 'big')
ban_ghi = id_byte + tuoi_byte

print(ban_ghi)
print(len(ban_ghi))
```

```text title=readonly
b'\x00\x07\x19'
3
```

`id_byte` chiếm `2` byte ĐẦU (`\x00\x07` = `7`), `tuoi_byte` chiếm
byte THỨ ba (`\x19` = `25`) — MỘT bản ghi `3` byte, NHƯNG KHÔNG có gì
"đánh dấu" ranh GIỚI giữa hai trường TRONG chính chuỗi byte đó. Người
ĐỌC (chương trình khác) PHẢI biết TRƯỚC: trường ĐẦU rộng `2` byte,
trường SAU rộng `1` byte — đúng LÀ **fixed-width record**: ĐỘ rộng
MỖI trường LÀ THOẢ thuận CỐ định, KHÔNG lưu KÈM trong dữ liệu.
::::

::::example{#cat-lat-doc-truong}
Đọc LẠI một trường bằng CẮT lát (`slice`, đã quen TỪ `list`/`str`)
rồi `int.from_bytes`:

```python title=readonly
ban_ghi = b'\x00\x07\x19'

id_lai = int.from_bytes(ban_ghi[0:2], 'big')
tuoi_lai = int.from_bytes(ban_ghi[2:3], 'big')

print(id_lai)
print(tuoi_lai)
```

```text title=readonly
7
25
```

`ban_ghi[0:2]` LẤY đúng `2` byte ĐẦU (trường `id`), `ban_ghi[2:3]`
lấy đúng byte THỨ ba (trường `tuoi`) — biên GIỚI `[0:2]` RỒI `[2:3]`
LÀ do NGƯỜI viết code TỰ tính, KHÔNG phải Python đoán GIÙM.
::::

::::predict{#doan-cat-sai-do-rong commitOnce}
Byte VIẾT nhầm — cắt CHỈ `1` byte cho trường `id` (LẼ RA phải `2`):

```python
ban_ghi = b'\x00\x07\x19'
id_sai = int.from_bytes(ban_ghi[0:1], 'big')
print(id_sai)
```

Dòng cuối in ra gì?

:::opt{correct}
`0`
:::

:::opt
`7` — vì Python NHẬN ra trường `id` THẬT SỰ cần `2` byte NÊN tự ĐỌC
thêm byte THỨ hai dù code chỉ CẮT `[0:1]`
::why
Gần đúng ở việc bạn TIN Python đủ "THÔNG minh" để hiểu Ý định thật
của người viết — một niềm TIN dễ hiểu SAU khi thấy `to_bytes` KIỂM
lỗi cẩn thận Ở bài TRƯỚC.

Chỗ lệch: `slice` KHÔNG hề biết "Ý định" — `ban_ghi[0:1]` LUÔN LUÔN
LÀ đúng `1` byte được YÊU cầu (`b'\x00'`), KHÔNG hơn. `to_bytes` kiểm
lỗi lúc MÃ HOÁ, NHƯNG lúc ĐỌC lại (`slice`), KHÔNG có gì kiểm — CẮT
sai độ rộng LÀ lỗi CỦA người viết code, Python KHÔNG tự sửa GIÙM.
::
:::

:::opt
Máy báo lỗi — vì `ban_ghi[0:1]` KHÔNG đủ `2` byte MÀ trường `id` cần
::why
Gần đúng ở việc bạn LO ngại đúng chỗ — cắt SAI độ rộng LÀ một lỗi
thật, ĐÁNG được máy BÁO ra.

Chỗ lệch: `slice` trên `bytes` KHÔNG BAO GIỜ báo lỗi MIỄN chỉ số
nằm TRONG phạm vi hợp lệ (`0` tới `len(ban_ghi)`) — `[0:1]` HOÀN
TOÀN hợp lệ, chỉ LÀ SAI Ý định. Đây chính LÀ điều NGUY hiểm nhất
của fixed-width record: cắt SAI độ rộng KHÔNG sập chương trình, nó
ÂM THẦM trả VỀ một giá trị SAI mà TRÔNG vẫn "hợp lý".
::
:::
::::

::::code{#viet_doc_truong}
Viết `doc_truong(ban_ghi, bat_dau, do_dai)` — đọc MỘT trường TỪ
`ban_ghi`, bắt đầu Ở byte `bat_dau`, rộng `do_dai` byte, trả VỀ số
nguyên tương ỨNG.

```python title=starter
def doc_truong(ban_ghi, bat_dau, do_dai):
    return ___


ban_ghi = (7).to_bytes(2, 'big') + (25).to_bytes(1, 'big')
print(doc_truong(ban_ghi, 0, 2))
```

```python title=solution
def doc_truong(ban_ghi, bat_dau, do_dai):
    return int.from_bytes(ban_ghi[bat_dau:bat_dau + do_dai], 'big')


ban_ghi = (7).to_bytes(2, 'big') + (25).to_bytes(1, 'big')
print(doc_truong(ban_ghi, 0, 2))
```

```python title=test
ban_ghi = (7).to_bytes(2, 'big') + (25).to_bytes(1, 'big')
assert doc_truong(ban_ghi, 0, 2) == 7, "truong id -- 2 byte dau"
assert doc_truong(ban_ghi, 2, 1) == 25, "truong tuoi -- byte thu ba"

ban_ghi2 = (300).to_bytes(2, 'big') + (1).to_bytes(2, 'big') + (99).to_bytes(1, 'big')
assert doc_truong(ban_ghi2, 0, 2) == 300, "truong dau -- 2 byte"
assert doc_truong(ban_ghi2, 2, 2) == 1, "truong giua -- 2 byte tiep"
assert doc_truong(ban_ghi2, 4, 1) == 99, "truong cuoi -- 1 byte"
```

:::hints
- kind: attention
  body: "Cat lat ban_ghi tu bat_dau toi bat_dau + do_dai, roi doi ve so nguyen."
- kind: strategy
  body: "int.from_bytes(ban_ghi[bat_dau:bat_dau + do_dai], 'big')"
- kind: one-line
  body: "___ = int.from_bytes(ban_ghi[bat_dau:bat_dau + do_dai], 'big')"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai cat lat ban_ghi[bat_dau:bat_dau+do_dai] roi goi int.from_bytes
  requireAst:
  - kind: uses-call, target: from_bytes, min: 1
  - kind: uses-name, target: bat_dau, min: 1
  - kind: uses-name, target: do_dai, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^7\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ghép trường, đọc trường — bằng ĐÚNG biên GIỚI đã thoả thuận TRƯỚC.
Trường `id` LÀ số, CÒN trường TÊN thì SAO — chữ CŨNG phải thành byte
ĐỘ dài cố định chứ?
::::

::::reflect{#nghi-lai}
Ghép trường bằng `+`, đọc trường bằng CẮT lát ĐÚNG biên giới đã
thoả thuận TRƯỚC — sai độ RỘNG không báo lỗi, nó ÂM thầm SAI. Trường
`id` LÀ số nguyên, CÒN một trường TÊN người (chữ) THÌ sao — LÀM sao
Ép một CHUỖI chữ VỀ đúng một độ dài BYTE cố định?
::::

::::checkpoint{mastery=0.8}
::::
