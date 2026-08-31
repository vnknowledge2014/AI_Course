---
id: khoa-hoc-may-tinh.cau-truc-du-lieu.hang-doi-bang-mang-la-mot-cai-bay
title: "Hàng đợi xây bằng mảng là một cái bẫy"
summary: "Lấy ở đầu hàng đợi nghĩa là xoá ở chỉ số 0 — đúng chỗ tốn nhất theo bài 5. .pop(0) trông tự nhiên nhưng âm thầm bắt MỌI người còn lại trong hàng dồn chỗ, ở mỗi lần phục vụ."
locale: vi
track: khoa-hoc-may-tinh
module: cau-truc-du-lieu
order: 10
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ds.queue-array-trap]
requires: [ds.queue, ds.array-delete-shift, core.dict, core.list, core.list-index, ctrl.for-each, ctrl.for-range, ctrl.if, ctrl.comparison, core.augmented-assign, core.fstring]
concepts: [ds.queue-array-trap]
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
Có. Và cái giá đó bị trả ở MỌI lần phục vụ, không phải thỉnh thoảng như
`.append` ở bài 3.
::::

::::explain{#cai-bay-lo-ra}
Bài 5 đã đo rõ: xoá một phần tử ở giữa mảng buộc mọi phần tử ĐỨNG SAU phải
dồn lên một bước, để không còn ô trống nào giữa dãy liền kề. Xoá càng gần
ĐẦU mảng, càng nhiều phần tử phải dồn — và chỉ số 0, đầu mảng, chính là
chỗ TỐN NHẤT có thể chọn: xoá ở đó bắt TOÀN BỘ những phần tử còn lại
trong mảng phải dồn lên, không sót một ai.

`hang_doi.pop(0)` — công cụ bài trước dùng để lấy người đầu hàng — chính
là lệnh xoá ở chỉ số 0. Người mới học thường viết hàng đợi bằng đúng cách
này vì nó ĐỌC tự nhiên nhất: "lấy người đầu tiên" nghe như đúng là
`pop(0)`. Nhưng họ không biết mình vừa chọn đúng vị trí tốn kém nhất mà
bài 5 đã cảnh báo.

Đây là chỗ khác hẳn ngăn xếp: `.pop()` không tham số (bài 7) không bao
giờ phải dồn ai, vì nó chỉ động vào đuôi — chỗ rẻ nhất. `.pop(0)` thì
ngược lại hoàn toàn — nó động vào đầu, chỗ đắt nhất, và phải trả giá đó ở
MỖI LẦN gọi, không phải thỉnh thoảng như lúc `.append` phải xin vùng nhớ
mới (bài 3). Một hàng đợi càng dài, mỗi lần phục vụ một người càng phải
dồn nhiều người hơn.
::::

::::example{#dom-mot-nguoi}
Năm người xếp hàng. Byte theo dõi đúng một người — "Oanh" — trước và sau
khi người đứng đầu được phục vụ.

```python title=readonly
hang_doi = ["Lan", "Kim", "Mai", "Nga", "Oanh"]
chi_so_oanh_truoc = hang_doi.index("Oanh")

hang_doi.pop(0)

chi_so_oanh_sau = hang_doi.index("Oanh")
print(f"Oanh dồn từ chỉ số {chi_so_oanh_truoc} về {chi_so_oanh_sau}")
```

```text title=readonly
Oanh dồn từ chỉ số 4 về 3
```

Không ai đụng vào "Oanh" — không lệnh nào nhắc tên cô ấy trước dòng cuối.
Nhưng chỉ vì "Lan" ở đầu hàng bị lấy ra, "Oanh" tự động dồn lên một bước —
đúng cơ chế bài 5 đã lột trần. Và "Oanh" không phải người duy nhất: "Kim",
"Mai", "Nga" cũng dồn y hệt vậy, mọi người đứng sau "Lan" đều bị kéo theo.
::::

::::predict{#dau-hay-duoi commitOnce}
Byte thử cùng một hàng bốn người theo hai cách khác nhau, trên hai bản
sao độc lập:

```python
a = ["Vy", "Hoa", "Long", "Mai"]
a.pop(0)                  # phục vụ người ĐẦU hàng — cách hàng đợi thật làm
print(a)

b = ["Vy", "Hoa", "Long", "Mai"]
b.pop()                   # bỏ người CUỐI hàng — chỉ để so sánh, không phải cách hàng đợi làm
print(b)
```

**Trước khi chạy**, bạn đoán: trong `a`, có bao nhiêu người đổi chỉ số so
với ban đầu? Còn trong `b`?

:::opt{correct}
`a`: cả ba người còn lại (Hoa, Long, Mai) đổi chỉ số. `b`: không ai đổi
cả.
:::

:::opt
`a`: không ai đổi cả. `b`: cả ba người còn lại đổi chỉ số.
::why
Gần đúng ở việc bạn nhận ra CÓ một sự khác biệt lớn giữa hai cách — điều
đó đúng.

Chỗ lệch là bạn đảo ngược hai trường hợp. Xoá ở ĐẦU (`a.pop(0)`) mới là
chỗ tốn — mọi phần tử phía sau phải dồn lên lấp lỗ. Xoá ở ĐUÔI (`b.pop()`)
không để lại lỗ nào ở giữa dãy — không ai phía trước nó phải dồn, đúng
bài 7 đã dựng.
::
:::

:::opt
Cả hai đều 3 người đổi chỉ số.
::why
Gần đúng ở việc bạn nhớ đúng con số 3 — đúng là ba người còn lại trong cả
hai trường hợp.

Chỗ lệch là bạn nghĩ MỌI lượt xoá đều tốn như nhau, bất kể xoá ở đâu. Vị
trí bị xoá quyết định ai phải dồn: xoá đầu kéo theo TẤT CẢ phần tử phía
sau (`a`), xoá đuôi không kéo theo ai cả vì không có phần tử nào đứng sau
nó (`b`).
::
:::

:::opt
Cả hai đều 0 người đổi chỉ số — chỉ số của một tên là cố định, đâu có thể
tự đổi.
::why
Gần đúng ở việc bạn tin chỉ số gắn liền với TÊN — cảm giác "Long luôn là
Long" không sai.

Chỗ lệch là chỉ số không phải một nhãn gắn cứng vào từng người; nó chỉ là
VỊ TRÍ hiện tại trong mảng. Khi "Vy" bị xoá khỏi đầu `a`, "Hoa" — vốn ở
chỉ số 1 — giờ đứng ở chỉ số 0, dù không ai "đổi tên" nó cả. Đó chính là
việc dồn chỗ bài 5 đã nói.
::
:::
::::

::::code{#dem-so-nguoi-don-cho}
Sáu người xếp hàng: An, Binh, Chi, Dung, Em, Phuc. Byte phục vụ đúng
người đứng ĐẦU hàng, rồi đếm xem có bao nhiêu người trong số còn lại đã
phải đổi chỉ số so với lúc trước.

```python title=starter
hang_doi = ["An", "Binh", "Chi", "Dung", "Em", "Phuc"]

chi_so_cu = {}
for i in range(len(hang_doi)):
    chi_so_cu[hang_doi[i]] = i

nguoi_duoc_phuc_vu = ___                  # phục vụ người ĐẦU HÀNG hiện tại

so_nguoi_da_doi_cho = 0
for ten in hang_doi:
    if hang_doi.index(ten) != chi_so_cu[ten]:
        so_nguoi_da_doi_cho += 1

print(f"{nguoi_duoc_phuc_vu} rời hàng")
print(f"Số người phải dồn chỗ: {so_nguoi_da_doi_cho} trên tổng {len(hang_doi)} người còn lại")
print(hang_doi)
```

```python title=solution
hang_doi = ["An", "Binh", "Chi", "Dung", "Em", "Phuc"]

chi_so_cu = {}
for i in range(len(hang_doi)):
    chi_so_cu[hang_doi[i]] = i

nguoi_duoc_phuc_vu = hang_doi.pop(0)

so_nguoi_da_doi_cho = 0
for ten in hang_doi:
    if hang_doi.index(ten) != chi_so_cu[ten]:
        so_nguoi_da_doi_cho += 1

print(f"{nguoi_duoc_phuc_vu} rời hàng")
print(f"Số người phải dồn chỗ: {so_nguoi_da_doi_cho} trên tổng {len(hang_doi)} người còn lại")
print(hang_doi)
```

```python title=test
assert hang_doi == ["Binh", "Chi", "Dung", "Em", "Phuc"], f"sau khi phục vụ người đầu hàng, năm người còn lại phải đúng thứ tự cũ, trừ An — đang ra {hang_doi}"
assert nguoi_duoc_phuc_vu == "An", f"người được phục vụ phải là người ĐẦU HÀNG lúc đó — An — đang ra {nguoi_duoc_phuc_vu!r}"
assert so_nguoi_da_doi_cho == 5, f"cả năm người còn lại đều đứng SAU An, nên cả năm phải dồn chỗ — đang ra {so_nguoi_da_doi_cho}"
```

:::hints
- kind: attention
  body: Bài này không hỏi "ai bị phục vụ" — điều đó đã rõ. Nó hỏi bạn dùng ĐÚNG công cụ để phục vụ người đầu hàng, cùng công cụ mà ví dụ và predict phía trên vừa dùng.
- kind: strategy
  body: 'Người đầu hàng luôn ở chỉ số 0. Lấy người đó ra khỏi hang_doi bằng .pop(0) — đúng công cụ bài trước giới thiệu, và đúng chỗ bài này đang lột trần cái giá của nó. Đoạn đếm phía dưới đã viết sẵn, không cần sửa gì thêm — nó tự so chỉ số CŨ (chi_so_cu, ghi lại trước khi phục vụ) với chỉ số MỚI của từng người còn lại.'
- kind: one-line
  body: 'Điền `hang_doi.pop(0)` vào chỗ trống.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải thật sự gọi hang_doi.pop(0) để phục vụ đúng người đầu hàng và thật sự làm hang_doi ngắn đi — không phải gán thẳng chuỗi "An" đã biết trước, không phải hang_doi.pop() (bỏ người cuối, sai đầu), và không phải hang_doi.pop(1) hay chỉ số khác 0
  requireAst:
  # Không có cổng này, nguoi_duoc_phuc_vu = "An" (hardcode giá trị đúng, không
  # hề gọi pop) vẫn in đúng dòng đầu tiên — nhưng hang_doi không hề co lại,
  # nên so_nguoi_da_doi_cho và hai dòng in sau đều sai lệch, chỉ tests/output
  # mới bắt được, không phải static. Cổng này chặn sớm hơn: đếm thật trên lời
  # giải, .pop( xuất hiện đúng 1 lần trong toàn bộ khối.
  - kind: uses-call, target: pop, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^An rời hàng\\nSố người phải dồn chỗ: 5 trên tổng 5 người còn lại\\n\\['Binh', 'Chi', 'Dung', 'Em', 'Phuc'\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Năm trên năm người còn lại đều phải dồn chỗ — không phải một vài người
xui xẻo, mà là TOÀN BỘ hàng, ở mỗi lần phục vụ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cái bẫy đã lộ ra: hàng càng dài, mỗi lần phục vụ một người càng phải dồn
nhiều người hơn. Nhưng nhìn kỹ lại nguyên nhân: không phải "hàng đợi" tự
nó tệ — mà là cách CÀI ĐẶT nó trên một mảng liền kề, nơi mọi ô đều phải
sát nhau, buộc phải dồn khi có lỗ.

Nếu không dồn ai cả — nếu chỉ đơn giản GHI NHỚ đầu hàng hiện đang ở đâu,
thay vì luôn luôn kéo nó về chỉ số 0 — cái bẫy này có còn xảy ra không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
