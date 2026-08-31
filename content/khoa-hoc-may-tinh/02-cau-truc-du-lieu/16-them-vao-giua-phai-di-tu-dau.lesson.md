---
id: khoa-hoc-may-tinh.cau-truc-du-lieu.them-vao-giua-phai-di-tu-dau
title: "Thêm vào giữa: phải đi bộ từ đầu"
summary: "Cái giá danh sách liên kết phải trả: không có địa chỉ tính thẳng như mảng, muốn tới một nút giữa dây phải ĐI QUA hết mọi nút đứng trước nó."
locale: vi
track: khoa-hoc-may-tinh
module: cau-truc-du-lieu
order: 16
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ds.linked-insert-middle]
requires: [ds.linked-insert-head]
concepts: [ds.linked-insert-middle]
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
Thêm vào đầu thì rẻ. Thêm vào giữa lại đòi một việc mảng chưa từng đòi:
phải ĐI TỚI đó trước đã.
::::

::::explain{#phai-di-bo-truoc}
Bài 2 dạy: địa chỉ ô thứ i của một MẢNG tính thẳng được — một phép nhân,
không phải một cuộc tìm kiếm. Danh sách liên kết không có phép tính đó.
Không ai biết "bài hát đứng ngay sau Hạ trắng" nằm ở đâu trong bộ nhớ, cho
tới khi thật sự ĐI tới `nut` đang giữ "Hạ trắng" và đọc trường `"tiep"`
của nó.

Vậy chèn vào giữa cần hai giai đoạn, không phải một:

1. **Đi bộ** từ `dau`, kiểm từng nút một, cho tới khi tìm đúng nút cần
   chèn NGAY SAU nó.
2. Nối nút mới vào đúng chỗ đó — giống hệt cách bài 15 nối vào đầu, chỉ
   khác nút "đứng trước" giờ không còn luôn luôn là `dau` nữa.

Giai đoạn hai, viết ra, trông rất giống bài 15:

```python title=readonly
nut_moi = {"gia_tri": gia_tri_moi, "tiep": nut_truoc["tiep"]}
nut_truoc["tiep"] = nut_moi
```

Nhưng THỨ TỰ hai dòng này không phải chuyện tuỳ ý. Dòng đầu ĐỌC
`nut_truoc["tiep"]` — số nhà của nút đang đứng sau `nut_truoc` — trước
khi dòng thứ hai GHI ĐÈ chính trường đó. Đảo ngược hai dòng, và dòng thứ
nhất sẽ đọc phải giá trị đã bị dòng thứ hai ghi đè mất rồi.
::::

::::example{#di-bo-tim-cho-chen}
Byte muốn chèn "Mưa hồng" vào ngay sau "Hạ trắng". Trước khi chèn được,
phải ĐI TỚI đúng nút giữ "Hạ trắng" trước — không có cách nào nhảy thẳng
tới đó.

```python title=readonly
nut3 = {"gia_tri": "Diễm xưa", "tiep": None}
nut2 = {"gia_tri": "Hạ trắng", "tiep": nut3}
nut1 = {"gia_tri": "Cát bụi", "tiep": nut2}

hien = nut1
so_buoc = 0
while hien is not None and hien["gia_tri"] != "Hạ trắng":
    hien = hien["tiep"]
    so_buoc += 1

print("số bước phải đi:", so_buoc)

nut_moi = {"gia_tri": "Mưa hồng", "tiep": hien["tiep"]}
hien["tiep"] = nut_moi

ten_bai = []
h = nut1
while h is not None:
    ten_bai.append(h["gia_tri"])
    h = h["tiep"]
print(ten_bai)
```

```text title=readonly
số bước phải đi: 1
['Cát bụi', 'Hạ trắng', 'Mưa hồng', 'Diễm xưa']
```

Chèn "Mưa hồng" vào ngay sau "Diễm xưa" (nút CUỐI) thì số bước phải đi sẽ
là 2, không phải 1 — càng chèn xa về cuối, càng phải đi bộ nhiều hơn.
Không có phép tính nào rút ngắn quãng đường đó; chỉ có cách đi qua từng
nút một.
::::

::::predict{#dao-thu-tu-hai-dong commitOnce}
Byte viết hàm chèn, nhưng lỡ đảo NGƯỢC thứ tự hai dòng đã học ở phần
`explain` — ghi trước, đọc sau:

```python
nut_moi = {"gia_tri": gia_tri_moi, "tiep": None}
nut_truoc["tiep"] = nut_moi              # ghi TRƯỚC
nut_moi["tiep"] = nut_truoc["tiep"]      # đọc SAU
```

**Không chạy code**, chỉ đọc ba dòng trên: sau khi chạy xong, `nut_moi["tiep"]`
đang trỏ tới đâu?

:::opt{correct}
Trỏ ngược lại vào chính `nut_moi` — một vòng nối vào chính nó
:::

:::opt
Trỏ đúng tới nút đứng ngay sau `nut_truoc` trong dây gốc, không có gì khác
::why
Gần đúng ở chỗ đó CHÍNH LÀ điều đoạn mã ĐÁNG LẼ phải làm — ý định đúng,
và nếu hai dòng viết ĐÚNG thứ tự thì kết quả sẽ đúng như vậy.

Chỗ lệch là dòng thứ hai (`nut_truoc["tiep"] = nut_moi`) đã CHẠY TRƯỚC và
ghi đè `nut_truoc["tiep"]`. Tới lúc dòng thứ ba đọc `nut_truoc["tiep"]`,
giá trị cũ (nút đứng sau trong dây gốc) đã biến mất — thứ còn lại để đọc
chính là `nut_moi` vừa được gán vào đó.
::
:::

:::opt
Trỏ tới `None`, giống hệt lúc `nut_moi` vừa được tạo ra
::why
Gần đúng ở chỗ `None` đúng là giá trị `"tiep"` mang lúc `nut_moi` MỚI
được tạo, ở dòng đầu tiên.

Chỗ lệch là dòng thứ ba GÁN LẠI trường đó — `nut_moi["tiep"] = ...` — nên
`None` không còn đứng yên nữa. Nó bị ghi đè bằng bất cứ giá trị nào biểu
thức bên phải tính ra, mà ở đây biểu thức đó tính ra chính `nut_moi`.
::
:::

:::opt
Máy dừng lại báo lỗi, vì đọc một trường vừa mới bị ghi đè là không hợp lệ
::why
Gần đúng ở chỗ nghe có gì đó nguy hiểm khi vừa ghi vừa đọc cùng một chỗ —
phản xạ cảnh giác đó không sai.

Chỗ lệch là Python không hề cấm việc đó. Đọc lại một trường vừa gán là
hợp lệ hoàn toàn — nó chỉ đọc ra đúng giá trị MỚI vừa được ghi vào, không
phải giá trị cũ. Không có luật nào chặn, chỉ có logic sai nằm ở người
viết thứ tự hai dòng.
::
:::
::::

::::code{#chen-sau-mot-bai-hat}
Viết hàm `chen_sau_gia_tri(dau, gia_tri_tim, gia_tri_moi)`: đi bộ từ
`dau` tới khi tìm được nút có `"gia_tri"` bằng `gia_tri_tim`, rồi chèn
một nút mới giữ `gia_tri_moi` ngay SAU nút đó. Phần đi bộ và phần trả về
`False` khi không tìm thấy đã có sẵn — bạn chỉ viết phần CHÈN.

```python title=starter
def chen_sau_gia_tri(dau, gia_tri_tim, gia_tri_moi):
    hien = dau
    while hien is not None and hien["gia_tri"] != gia_tri_tim:
        hien = hien["tiep"]
    if hien is None:
        return False
    ___
    return True

nut3 = {"gia_tri": "Diễm xưa", "tiep": None}
nut2 = {"gia_tri": "Hạ trắng", "tiep": nut3}
nut1 = {"gia_tri": "Cát bụi", "tiep": nut2}

chen_sau_gia_tri(nut1, "Hạ trắng", "Mưa hồng")

hien = nut1
ten_bai = []
while hien is not None:
    ten_bai.append(hien["gia_tri"])
    hien = hien["tiep"]
print(ten_bai)
```

```python title=solution
def chen_sau_gia_tri(dau, gia_tri_tim, gia_tri_moi):
    hien = dau
    while hien is not None and hien["gia_tri"] != gia_tri_tim:
        hien = hien["tiep"]
    if hien is None:
        return False
    nut_moi = {"gia_tri": gia_tri_moi, "tiep": hien["tiep"]}
    hien["tiep"] = nut_moi
    return True

nut3 = {"gia_tri": "Diễm xưa", "tiep": None}
nut2 = {"gia_tri": "Hạ trắng", "tiep": nut3}
nut1 = {"gia_tri": "Cát bụi", "tiep": nut2}

chen_sau_gia_tri(nut1, "Hạ trắng", "Mưa hồng")

hien = nut1
ten_bai = []
while hien is not None:
    ten_bai.append(hien["gia_tri"])
    hien = hien["tiep"]
print(ten_bai)
```

```python title=test
assert ten_bai == ["Cát bụi", "Hạ trắng", "Mưa hồng", "Diễm xưa"], "sai vị trí chèn: " + str(ten_bai)

a3 = {"gia_tri": "Diễm xưa", "tiep": None}
a2 = {"gia_tri": "Hạ trắng", "tiep": a3}
a1 = {"gia_tri": "Cát bụi", "tiep": a2}
ok = chen_sau_gia_tri(a1, "Diễm xưa", "Một cõi đi về")
gom = []
h = a1
buoc = 0
while h is not None and buoc < 8:
    gom.append(h["gia_tri"]); h = h["tiep"]; buoc += 1
assert ok is True, "phải trả về True khi tìm thấy gia_tri_tim"
assert gom == ["Cát bụi", "Hạ trắng", "Diễm xưa", "Một cõi đi về"], "chèn sau phần tử CUỐI phải thêm được đúng vào cuối dây, không được làm rơi mất nút cuối cũ: " + str(gom)

b3 = {"gia_tri": "Diễm xưa", "tiep": None}
b2 = {"gia_tri": "Hạ trắng", "tiep": b3}
b1 = {"gia_tri": "Cát bụi", "tiep": b2}
assert chen_sau_gia_tri(b1, "Không có bài này", "x") is False, "không tìm thấy gia_tri_tim thì phải trả về False, không được chèn bừa"
```

:::hints
- kind: attention
  body: Tới đúng chỗ trống, `hien` ĐANG LÀ nút mà bạn cần chèn nút mới vào NGAY SAU. Chỗ trống cần hai việc — dựng nút mới, và nối nó vào dây — theo đúng thứ tự đã học ở phần giải thích.
- kind: strategy
  body: 'Dựng nút mới với tiep TRỎ TỚI hien["tiep"] hiện tại (chỗ hien đang trỏ đi, trước khi bị đổi) — đây phải là dòng ĐẦU. Rồi mới gán hien["tiep"] bằng nút mới đó — dòng SAU. Đảo ngược hai dòng này chính là lỗi bài predict vừa cho bạn thấy.'
- kind: one-line
  body: 'Điền `nut_moi = {"gia_tri": gia_tri_moi, "tiep": hien["tiep"]}` rồi `hien["tiep"] = nut_moi`, đúng theo thứ tự đó.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^\['Cát bụi', 'Hạ trắng', 'Mưa hồng', 'Diễm xưa'\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đi bộ để tìm chỗ, rồi nối một dây — đúng thứ tự, không nút nào bị mất.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn giờ biết đủ ba việc: thêm vào đầu (bài 15, rẻ), thêm vào giữa (bài
này, phải đi bộ), và đọc hết cả dây (bài 14). Mảng thì ngược hẳn: tới
thẳng một ô bất kỳ bằng chỉ số (bài 2), nhưng chèn/xoá giữa lại phải dời
cả dãy (bài 4, 5).

Nếu phải chọn một trong hai cấu trúc cho một công việc cụ thể, bạn dựa
vào đâu để chọn?

Bài sau xếp cả hai cạnh nhau và trả lời thẳng câu đó.
::::

::::checkpoint{mastery=0.8}
::::
