---
id: nen-tang.list-dict-set-tuple.ro-khong-xep-hang
title: Rổ không xếp hàng
summary: "Rổ vứt bỏ chỗ đứng — `ro[0]` là `TypeError` — và chính vì vứt bỏ chỗ đứng mà `in` trên rổ trả lời được ngay, không phải dò từng ô."
locale: vi
track: nen-tang
module: list-dict-set-tuple
order: 27
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.set-unordered, core.set-membership]
requires: [core.set, core.list-membership, core.list-index, core.list, core.list-append, core.len, ctrl.for-each, ctrl.if, core.fstring, err.type-error]
concepts: [core.tap-hop, core.chi-so, core.cho-chua]
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
  reviewed: true
---

::::byte{trigger=enter mood=curious pose=lean-in}
Mình bỏ mất một thứ. Và mình bỏ nó đi để đổi lấy một thứ khác.
::::

::::explain{#cai-ro-thieu-mat-gi}
Câu hỏi bài trước để lại: gõ `ro[0]` để lấy phần tử đầu của rổ thì máy nói gì?

Trước khi thử, hãy nhìn lại vì sao câu hỏi ấy nghe hợp lý đến thế. Mọi chỗ
chứa bạn từng gặp đều trả lời được nó:

- Danh sách: `so[0]` là khoản đầu, `so[-1]` là khoản cuối.
- Chuỗi: `ten[0]` là ký tự đầu.
- Tuple: `cap[0]` là ô đầu của cặp.
- Sổ tra cứu thì không tra bằng chỗ đứng, nhưng duyệt nó vẫn ra khoá theo đúng
  thứ tự bạn thêm vào — vẫn có một thứ tự để mà nói tới.

Cả bốn thứ ấy có chung một chuyện: mỗi giá trị nằm ở **một chỗ**, và chỗ ấy do
**bạn** quyết định — bạn `.append` vào cuối, bạn ghi khoá theo thứ tự này thì
nó nằm theo thứ tự ấy.

Cái rổ thì không. Bài trước bạn đã thấy dấu hiệu: bỏ vào theo một thứ tự, in
ra theo một thứ tự khác. Đó không phải máy đãng trí. Đó là vì trong một cái rổ
**không có chỗ đứng nào cả** — không ô số 0, không ô số 1, không "cái đầu
tiên". Chỉ có: giá trị này có nằm trong rổ, hay không.

Vậy `ro[0]` đang hỏi một câu mà rổ không có nghĩa nào để trả lời.
::::

::::predict{#hoi-ro-o-so-khong commitOnce}
Byte dựng một cái rổ ba nhóm rồi hỏi nó phần tử đầu tiên.

**Trước khi bấm chạy**, bạn đoán máy làm gì?

```python title=readonly
ro = {"ăn uống", "xăng xe", "học phí"}

print(ro[0])
```

:::opt{correct}
Máy dừng lại, báo `TypeError` — kiểu `set` không nhận cặp ngoặc vuông
:::

:::opt
In ra `ăn uống`, vì đó là tên vào rổ đầu tiên
::why
Gần đúng ở chỗ bạn suy từ một luật thật và đang áp nó rộng ra: danh sách,
chuỗi và tuple đều nhớ thứ tự bạn đặt vào, và sổ tra cứu cũng duyệt ra khoá
theo đúng thứ tự được thêm. Bốn trên năm chỗ chứa hành xử đúng như bạn nói.

Chỗ lệch là cái rổ nằm ngoài bốn thứ đó. Nó không ghi lại thứ tự bạn bỏ vào,
nên nó cũng chẳng có "cái đầu tiên" để mà đưa ra. Bằng chứng bạn đã thấy ở bài
trước: in rổ ra thì thứ tự không giống lúc bỏ vào.
::
:::

:::opt
In ra một trong ba tên, không biết trước là tên nào
::why
Gần đúng ở nửa quan trọng nhất: bạn nhận ra rổ **không hứa** thứ tự nào cả, và
điều đó chính xác.

Chỗ lệch là ở chữ *chọn đại*. Máy không bốc bừa một phần tử rồi đưa cho bạn.
Nó từ chối hẳn câu hỏi, và đó là chuyện tốt: nếu `ro[0]` cứ trả về đại một tên
thì cùng một chương trình chạy hai lần cho hai kết quả khác nhau mà không báo
gì — đúng loại lỗi im lặng mà hai dãy song song đã cho bạn thấy là nguy hiểm
tới đâu.
::
:::

:::opt
Báo `KeyError`, vì `0` không có trong rổ
::why
Gần đúng ở chỗ bạn nhớ đúng tên của lỗi tra một khoá không có, và cũng nhận ra
số `0` chẳng phải thứ nằm trong rổ này.

Chỗ lệch là máy chưa kịp đi tìm `0`. Nó dừng sớm hơn một bước: cặp ngoặc vuông
nghĩa là "lấy theo chỗ đứng", mà `set` không hỗ trợ phép đó chút nào. Sai **loại
việc** chứ không phải sai **nội dung** — nên lỗi thuộc họ `TypeError`, đúng như
lúc bạn thử gán vào một ô của tuple.
::
:::
::::

::::example{#thu-that}
Chạy thật, và chạy thêm hai câu nữa để thấy rổ làm được gì:

```python title=readonly
ro = {"ăn uống", "xăng xe", "học phí"}

print("ăn uống" in ro)
print("du lịch" in ro)

for nhom in ro:
    print(nhom)

print(ro[0])
```

Máy in ra:

```text title=readonly
True
False
xăng xe
ăn uống
học phí
Traceback (most recent call last):
  File "so_chi_tieu.py", line 10, in <module>
    print(ro[0])
          ~~^^^
TypeError: 'set' object is not subscriptable
```

Ba chuyện đọc ra được từ màn hình này:

- **`in` chạy ngon trên rổ.** Cùng một câu hỏi, cùng một cách viết như trên
  danh sách, cùng câu trả lời `True` / `False`. Không có gì mới phải học.
- **`for` cũng chạy ngon.** Rổ đưa ra từng phần tử một, mỗi phần tử đúng một
  lần. Chỉ có điều nó không hứa thứ tự: ba dòng ấy trên máy bạn có thể hiện ra
  theo một thứ tự khác. Nên đừng bao giờ viết chương trình dựa vào việc rổ đưa
  ra cái gì trước.
- **`ro[0]` thì không.** Máy dừng đúng tại dòng ấy. Chữ *subscriptable* trong
  câu báo lỗi nghĩa là "nhận được cặp ngoặc vuông" — và `set` thì không.

Vậy rổ đúng là thiếu một thứ mà danh sách có. Câu hỏi tiếp theo mới là câu
đáng tiền: **nó được gì khi chịu thiếu?**
::::

::::explain{#doi-cho-dung-lay-cau-tra-loi-nhanh}
Nhớ lại lời hẹn từ bài hỏi `in` trên một danh sách: *cuối track bạn sẽ gặp một
chỗ chứa trả lời cùng câu hỏi ấy theo kiểu khác hẳn*. Đây chính là chỗ chứa
đó.

Trên **danh sách**, để trả lời `"học phí" in ds` máy đứng ở ô đầu, so, chưa
khớp thì bước sang ô kế, cứ thế đi tới. Danh sách bốn mươi tên mà câu trả lời
là "không có" thì máy phải so đủ bốn mươi lần mới dám kết luận.

Trên **rổ**, máy không dò. Hãy hình dung cái rổ là một cái tủ nhiều ngăn, và
có một quy tắc cố định biến mỗi giá trị thành số hiệu một ngăn: cùng một giá
trị thì bao giờ cũng ra cùng số hiệu ấy. Bỏ `"học phí"` vào rổ nghĩa là tính
ra số hiệu rồi đặt nó vào đúng ngăn đó. Hỏi `"học phí" in ro` thì máy tính lại
số hiệu ấy — cũng từ chính giá trị ấy — rồi mở đúng một ngăn ra xem. Nó không
cần biết trong tủ có bốn mươi thứ hay bốn nghìn thứ.

Và bây giờ hai chuyện của bài này khớp vào nhau. **Chỗ nằm của một giá trị
trong rổ là do chính giá trị ấy quyết định, không do lượt bạn bỏ vào quyết
định.** Đó là một câu duy nhất, và nó kéo theo cả hai điều:

- Không còn chỗ nào để ghi "cái này vào trước cái kia", nên rổ không có thứ tự
  và không có `ro[0]`.
- Nhưng nhờ vậy, tìm một giá trị chỉ là mở đúng một ngăn, chứ không phải đi
  dọc cả tủ.

Không phải rổ tốt hơn danh sách. Rổ đã **đánh đổi**: bỏ chỗ đứng để lấy câu
trả lời nhanh cho đúng một câu hỏi — *có hay không*. Cần thứ tự, cần "cái thứ
ba", cần giữ cả những lần trùng thì danh sách vẫn là chỗ đúng.

> Chỗ dễ vấp: rổ **không** làm câu `in` trên danh sách nhanh lên. Muốn hưởng
> phần nhanh ấy thì thứ bạn đem ra hỏi phải là một cái rổ. Đổi
> `if nhom not in ds` thành rổ mà vẫn giữ `ds` là một danh sách thì chẳng có
> gì đổi khác cả.
::::

::::code{#nhom-nao-thang-nay-co}
Byte đã gom xong rổ các nhóm chi của tháng này. Sếp đưa ba cái tên và hỏi từng
cái một: tháng này có chi vào nhóm đó không?

Khung dưới đã có sẵn cái rổ, ba câu hỏi, một danh sách rỗng để ghi câu trả
lời, và đoạn in kết quả. Còn thiếu đúng **điều kiện** của câu `if`.

```python title=starter
nhom_thang_nay = {"ăn uống", "xăng xe", "học phí"}
cau_hoi = ["học phí", "du lịch", "ăn uống"]

tra_loi = []

for nhom in cau_hoi:
    if ___:
        tra_loi.append("có")
    else:
        tra_loi.append("không")

print(f"học phí: {tra_loi[0]}")
print(f"du lịch: {tra_loi[1]}")
print(f"ăn uống: {tra_loi[2]}")
```

```python title=solution
nhom_thang_nay = {"ăn uống", "xăng xe", "học phí"}
cau_hoi = ["học phí", "du lịch", "ăn uống"]

tra_loi = []

for nhom in cau_hoi:
    if nhom in nhom_thang_nay:
        tra_loi.append("có")
    else:
        tra_loi.append("không")

print(f"học phí: {tra_loi[0]}")
print(f"du lịch: {tra_loi[1]}")
print(f"ăn uống: {tra_loi[2]}")
```

```python title=test
# Ba câu hỏi cố ý không cùng một đáp án: hai câu "có" kẹp một câu "không" ở
# giữa. Một điều kiện luôn đúng thì vỡ ở dòng thứ ba, một điều kiện luôn sai
# thì vỡ ở dòng thứ hai — không có hằng số nào lọt qua được cả ba.
assert len(tra_loi) == 3, "có ba cái tên trong danh sách câu hỏi, nên mỗi lượt phải ghi đúng một câu trả lời, tổng cộng ba câu"
assert tra_loi[0] == "có", "học phí nằm trong rổ nhóm của tháng này, nên câu trả lời thứ nhất phải là có"
assert tra_loi[1] == "không", "du lịch không nằm trong rổ nhóm của tháng này, nên câu trả lời thứ hai phải là không"
assert tra_loi[2] == "có", "ăn uống nằm trong rổ nhóm của tháng này, nên câu trả lời thứ ba phải là có"
```

:::hints
- kind: attention
  body: Mỗi lượt của vòng lặp, `nhom` đang giữ một cái tên mà sếp hỏi. Câu trả lời cho tên ấy nằm trong cái rổ đã dựng sẵn ở dòng đầu, nên điều kiện phải nhắc tới cả hai — cái tên đang hỏi và cái rổ đem ra tra.
- kind: strategy
  body: Câu hỏi mỗi lượt đọc thành lời là "cái tên này có nằm trong rổ nhóm của tháng này không". Bạn đã có sẵn một phép hỏi đúng hình dạng ấy từ hồi làm việc với danh sách, và nó viết y hệt khi bên phải là một cái rổ — chỉ khác ở chỗ lần này máy không phải dò từng ô.
- kind: one-line
  body: "Viết `nhom in nhom_thang_nay` vào chỗ trống, giữ nguyên dấu hai chấm ở cuối dòng."
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 5000
- tier: output
  match: regex
  expect: ^học phí: có\ndu lịch: không\năn uống: có\s*$
- tier: static
  onFail: điều kiện phải đem cái tên của lượt này ra tra trong rổ `nhom_thang_nay`, không được là một phép so sánh tự chế
  requireAst:
  # Khung KHÔNG đọc `nhom_thang_nay` ở đâu cả (dòng đầu chỉ đặt tên ấy), và
  # cũng không đọc `nhom` (dòng `for` chỉ đặt tên). Nên mỗi luật một mình đã
  # đủ phân biệt; giữ cả hai để chặn luôn kiểu `nhom != "du lịch"`, thứ vô
  # tình cho ra đúng ba câu trả lời mà chẳng tra rổ lần nào.
  - kind: uses-name, target: nhom_thang_nay, min: 1
  - kind: uses-name, target: nhom, min: 1
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không có ô số 0 nào cả. Đổi lại, hỏi cái gì mình cũng trả lời được ngay.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Byte đem tới hai cái rổ: nhóm chi của tháng Mười, và nhóm chi của tháng Mười
Một. Cả hai đã bỏ trùng xong, mỗi tên nằm đúng một lần.

Byte hỏi: **nhóm nào tháng nào cũng có?**

Đồ nghề trong tay bạn đủ để trả lời rồi. Duyệt rổ này, mỗi nhóm hỏi rổ kia một
câu `in`, câu nào ra `True` thì nhặt sang một chỗ chứa mới — và nhờ bài này
thì mỗi câu `in` ấy rẻ.

Nhưng đọc lại đoạn ấy xem nó nói gì: *tạo một chỗ chứa rỗng · đi qua từng nhóm
· hỏi một câu · nhặt vào nếu đúng*. Bốn động tác để diễn đạt một ý duy nhất, mà
ý ấy trong tiếng Việt chỉ có ba chữ.

Duyệt rổ này rồi `in` rổ kia — hay có một phép làm gọn cả bốn động tác trong
một lượt?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
