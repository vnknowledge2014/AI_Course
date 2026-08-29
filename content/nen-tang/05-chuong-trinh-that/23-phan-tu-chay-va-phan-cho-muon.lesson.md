---
id: nen-tang.chuong-trinh-that.phan-tu-chay-va-phan-cho-muon
title: Phần tự chạy và phần cho mượn
summary: "Python đặt sẵn trong mỗi file một cái tên `__name__` — bằng `__main__` khi file được gọi thẳng, bằng tên module khi file bị mượn — và đó là chỗ tách phần thử khỏi phần cho mượn."
locale: vi
track: nen-tang
module: chuong-trinh-that
order: 23
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.main-guard]
requires: [core.import-runs-file, core.import-module, core.own-module, mod.import, mod.dotted-access, core.function-def, core.variable, core.assignment, core.string-literal, core.fstring, core.output, core.boolean, core.print-variable, ctrl.if, ctrl.comparison]
concepts: [core.vai-cua-file, core.le-trai, core.dieu-kien]
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
Trước khi chạy một file, Python ghé tai nói cho file ấy biết nó đang đóng vai nào.
::::

::::explain{#may-noi-cho-file-biet-no-dang-dong-vai-nao}
Bài trước để lại một thế kẹt có thật. Mọi dòng ở lề trái của `so_sach.py` đều
chạy khi ai đó import nó — kể cả mấy dòng bạn viết chỉ để thử cho nhanh. Bỏ
chúng đi thì mất chỗ thử; để nguyên thì `main.py` phải gánh chúng mỗi lần chạy.

Muốn thoát khỏi thế kẹt này, `so_sach.py` phải trả lời được một câu: *lần này
tôi đang đóng vai nào — file được gọi thẳng, hay file bị mượn?* Hai vai ấy khác
nhau thật, và người duy nhất biết chắc là Python: chính nó vừa quyết định chạy
file này vì lý do gì.

Và nó có nói. Trước khi chạy một file, Python đặt sẵn vào bên trong file ấy một
cái tên, tên là `__name__` — hai dấu gạch dưới ở mỗi đầu. Cái tên ấy đã có giá
trị từ trước dòng đầu tiên của bạn, và giá trị của nó tuỳ theo vai:

- File **bạn gọi thẳng** (`python main.py`) nhận `__name__` bằng chuỗi
  `"__main__"`.
- File **bị mượn** qua `import` nhận `__name__` bằng chính tên module của nó —
  file `so_sach.py` nhận chuỗi `"so_sach"`.

Chú ý một chuyện, vì nó dễ nhầm: luật của bài trước không bị bác bỏ chỗ nào cả.
Cả file được mượn **vẫn chạy** hết, từ đầu tới cuối, đúng như bài 22 nói. Cái
thay đổi giữa hai vai không phải là "chạy hay không chạy", mà là câu hỏi
`__name__ đang bằng gì?` cho ra câu trả lời khác nhau.

Có câu hỏi ấy rồi thì phần còn lại là một câu `if` bình thường, đặt ở lề trái:

```python title=readonly
if __name__ == "__main__":
    print("THỬ NHANH: sổ có", SO_KHOAN, "khoản")
```

Đọc thành tiếng Việt: *nếu tôi đang là file được gọi thẳng thì chạy khối này.*
Gọi thẳng thì `__name__` bằng `"__main__"`, câu hỏi cho `True`, khối thụt vào
chạy. Bị mượn thì `__name__` bằng `"so_sach"`, câu hỏi cho `False`, khối thụt
vào bị bỏ qua — trong khi mọi dòng lề trái khác vẫn chạy như thường.

Một file viết theo lối này có hai phần rõ ràng: **phần cho mượn** ở lề trái
(các `def`, các hằng số) và **phần tự chạy** nằm thụt vào trong câu `if` ấy.
::::

::::example{#mot-file-hai-vai}
Vẫn hai file ấy, nhưng rút gọn hẳn để nhìn cho rõ đúng một chuyện: `doc_so`
lần này giữ sẵn ba dòng sổ trong `CAC_DONG` thay vì mở file, và `so_sach.py`
được thêm ba dòng cuối. Cuốn sổ thật thì vẫn đợi ở bài sau — ở đây file nào
mở file nào không, không phải chuyện đang bàn.

```python title=readonly
# ── so_sach.py ──────────────────────────────────────────────────────
print("so_sach.py: đang mở sổ...")

CAC_DONG = ["cà phê,25000", "bún bò,40000", "gửi xe,10000"]
SO_KHOAN = len(CAC_DONG)


def doc_so():
    print("so_sach.py: doc_so vừa được gọi")
    return CAC_DONG


if __name__ == "__main__":
    print("THỬ NHANH: sổ có", SO_KHOAN, "khoản")
```

```python title=readonly
# ── main.py ─────────────────────────────────────────────────────────
import so_sach

print("main.py: bắt đầu")
print("main.py: sổ có", so_sach.SO_KHOAN, "khoản")
```

Gõ `python so_sach.py` — vai thứ nhất, gọi thẳng:

```text title=readonly
so_sach.py: đang mở sổ...
THỬ NHANH: sổ có 3 khoản
```

Gõ `python main.py` — vai thứ hai, bị mượn:

```text title=readonly
so_sach.py: đang mở sổ...
main.py: bắt đầu
main.py: sổ có 3 khoản
```

Đặt hai màn hình cạnh nhau thì thấy đúng một chỗ khác:

- **Câu chào vẫn hiện ở cả hai lần.** Nó nằm ở lề trái, không nằm trong câu
  `if`, nên bài 22 vẫn đúng nguyên: mượn một file là chạy cả file đó.
- **Câu "THỬ NHANH" chỉ hiện ở lần đầu.** Lần sau, `so_sach.py` đang đóng vai
  file bị mượn, `__name__` của nó bằng `"so_sach"` chứ không bằng `"__main__"`,
  nên khối thụt vào bị bỏ qua.
- **`main.py` không phải sửa một chữ nào.** Chỗ tách hai vai nằm hẳn bên trong
  file cho mượn, và mọi file mượn nó đều được lợi.
::::

::::predict{#doan-hai-cai-ten commitOnce}
Byte muốn nhìn tận mắt hai giá trị ấy, nên đặt vào mỗi file một dòng in ra
chính `__name__` của nó.

**Trước khi bấm chạy** `python main.py`, bạn đoán màn hình hiện ra gì?

```python title=readonly
# ── so_sach.py ──────────────────────────────────────────────────────
print("so_sach.py thấy __name__ =", __name__)
```

```python title=readonly
# ── main.py ─────────────────────────────────────────────────────────
import so_sach

print("main.py thấy __name__ =", __name__)
```

:::opt{correct}
`so_sach.py thấy __name__ = so_sach` rồi `main.py thấy __name__ = __main__`
:::

:::opt
Cả hai dòng đều `__main__`
::why
Gần đúng ở chỗ bạn nhớ đúng một nửa sự thật: `__main__` đúng là cái tên dành
cho file đang chạy, và cả hai file thì đều đang chạy thật — bài 22 vừa chứng
minh điều đó.

Chỗ lệch nằm ở nghĩa của chữ "main" ở đây. Nó không có nghĩa là *"đang chạy"*,
nó có nghĩa là *"được gọi thẳng"*. Trong một lần gõ lệnh chỉ có đúng một file
được gọi thẳng — file bạn viết tên ra sau chữ `python`. Mọi file khác đều vào
theo đường `import`, và chúng nhận tên module của mình.
::
:::

:::opt
Cả hai dòng đều in ra tên file của chính nó: `so_sach` rồi `main`
::why
Gần đúng ở chỗ bạn suy ra một luật rất gọn từ nửa sự thật đã thấy: mỗi file
mang tên của chính nó. Với `so_sach.py` thì luật ấy cho đúng đáp án.

Chỗ lệch là ở file được gọi thẳng. Nếu nó cũng mang tên `"main"` thì không câu
`if` nào phân biệt nổi hai vai nữa — muốn hỏi "tôi có được gọi thẳng không",
bạn sẽ phải gõ sẵn tên file của mình vào code, và đổi tên file một cái là hỏng.
Python tránh chuyện đó bằng cách dùng chung một chuỗi cố định `"__main__"` cho
vai được gọi thẳng, dù file tên gì.
::
:::

:::opt
Chỉ có `main.py thấy __name__ = __main__`, vì `so_sach.py` chưa được gọi hàm nào
::why
Gần đúng ở chỗ bạn cẩn thận đúng chỗ đáng cẩn thận: `main.py` quả thật không
gọi hàm nào của `so_sach` cả.

Chỗ lệch là dòng `print` trong `so_sach.py` nằm ở **lề trái**, không nằm trong
thân một hàm nào. Bài 22 đã cho thấy đúng chuyện này: `import` chạy cả file
được mượn, nên mọi dòng lề trái của nó đều chạy — kể cả khi bên kia chưa gọi gì.
::
:::
::::

::::code{#hoi-may-toi-dang-la-ai}
Đến lượt bạn hỏi máy.

Khung tập này là một file, và nó là file **được gọi thẳng** khi bạn bấm chạy.
Còn để nhìn thấy vế bên kia — cái tên mà một file bị mượn nhận được — bạn hỏi
thẳng một hộp mình đã mượn từ bài 15: hộp `os`. Nó cũng là một file Python nằm
đâu đó trong máy, và nó vào chương trình này qua đường `import`.

Hai chỗ trống:

- chỗ thứ nhất lấy ra cái tên mà Python vừa đặt cho **chính file này**;
- chỗ thứ hai là câu hỏi mở cửa cho phần thử nhanh.

```python title=starter
import os

# Python đã đặt sẵn một cái tên vào file này trước cả dòng đầu tiên.
# Lấy cái tên ấy ra.
ten_file_dang_chay = ___

# Hộp `os` thì vào đây bằng đường import, không phải bằng lệnh gọi thẳng.
# Hỏi xem Python đặt cho nó cái tên nào.
ten_file_duoc_muon = os.__name__

print(f"File đang chạy mang tên: {ten_file_dang_chay}")
print(f"File được mượn mang tên: {ten_file_duoc_muon}")

# Phần thử nhanh: chỉ được chạy khi file này là file được gọi thẳng.
da_chay_phan_thu = False
if ___:
    da_chay_phan_thu = True
    print("PHẦN THỬ NHANH: chạy vì file này được gọi thẳng")
```

```python title=solution
import os

# Python đã đặt sẵn một cái tên vào file này trước cả dòng đầu tiên.
# Lấy cái tên ấy ra.
ten_file_dang_chay = __name__

# Hộp `os` thì vào đây bằng đường import, không phải bằng lệnh gọi thẳng.
# Hỏi xem Python đặt cho nó cái tên nào.
ten_file_duoc_muon = os.__name__

print(f"File đang chạy mang tên: {ten_file_dang_chay}")
print(f"File được mượn mang tên: {ten_file_duoc_muon}")

# Phần thử nhanh: chỉ được chạy khi file này là file được gọi thẳng.
da_chay_phan_thu = False
if __name__ == "__main__":
    da_chay_phan_thu = True
    print("PHẦN THỬ NHANH: chạy vì file này được gọi thẳng")
```

```python title=test
# Chỗ trống 1 bị soi bởi câu này: file đang chạy phải nhận đúng chuỗi
# dành cho vai được gọi thẳng, không phải tên nào khác.
assert ten_file_dang_chay == "__main__", "file này là file được gọi thẳng, nên cái tên Python đặt cho nó phải là chuỗi __main__"
# Câu này không có chỗ trống, nhưng nó là vế đối chứng: một file bị mượn
# nhận tên module của nó, ở đây là os.
assert ten_file_duoc_muon == "os", "hộp os vào chương trình bằng đường import, nên cái tên Python đặt cho nó là chuỗi os"
# Chỗ trống 2 bị soi bởi câu này. Câu hỏi phải cho True trong lần chạy
# này; viết ngược dấu, hay so với một chuỗi khác, thì khối thụt vào không
# chạy và da_chay_phan_thu còn nguyên False.
assert da_chay_phan_thu is True, "trong lần chạy này file được gọi thẳng, nên phần thử nhanh phải chạy và da_chay_phan_thu phải thành True"
# Và hai vai phải khác nhau — đó là toàn bộ lý do câu hỏi kia có nghĩa.
assert ten_file_dang_chay != ten_file_duoc_muon, "hai cái tên phải khác nhau: file được gọi thẳng nhận __main__, file bị mượn nhận tên module của nó"
```

:::hints
- kind: attention
  body: "Cái tên Python đặt sẵn vào mỗi file có hai dấu gạch dưới ở mỗi đầu, và bạn đã thấy nó ở dòng ngay dưới chỗ trống thứ nhất — chỉ khác là ở đó nó được hỏi qua một cái hộp, còn ở đây bạn hỏi thẳng file mình đang viết."
- kind: strategy
  body: "Chỗ trống thứ nhất chỉ cần đúng cái tên ấy viết trần, không dấu chấm, không hộp nào phía trước. Chỗ trống thứ hai là một câu hỏi đúng-sai: so cái tên ấy với chuỗi mà Python dành riêng cho vai được gọi thẳng — chuỗi ấy được viết ra đầy đủ trong phần giải thích ở đầu bài, và nó cũng có hai dấu gạch dưới ở mỗi đầu."
- kind: one-line
  body: "Chỗ trống thứ nhất viết `__name__`, chỗ thứ hai viết `__name__ == \"__main__\"`."
:::

:::validate
- tier: static
  onFail: cả hai chỗ trống phải hỏi tới cái tên `__name__` viết trần, và câu hỏi phải so nó với chuỗi `__main__`
  requireAst:
  # `min: 2` vì có hai chỗ trống. Khung chưa đọc `__name__` trần lần nào —
  # `os.__name__` là lấy đồ trong hộp, không tính — nên luật này chặn cả đáp
  # án gõ bừa lẫn đáp án chép cứng chuỗi vào chỗ thứ nhất.
  - kind: uses-name, target: __name__, min: 2
  # Câu hỏi phải so với đúng chuỗi ấy, không so với "os" hay một tên file.
  - kind: has-literal, target: __main__, min: 1
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 5000
- tier: output
  expect: "File đang chạy mang tên: __main__"
- tier: output
  expect: "File được mượn mang tên: os"
- tier: output
  expect: "PHẦN THỬ NHANH: chạy vì file này được gọi thẳng"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một file, hai vai, và đúng một dòng để hỏi lần này là vai nào.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Sổ đã nằm trên đĩa, chương trình đã nhận tên file từ dòng lệnh, module đã tách
được phần thử khỏi phần cho mượn. Bạn gõ:

```text title=readonly
python main.py xem so.txt
```

Không một dòng báo lỗi nào. Không traceback, không tên lỗi, không số dòng.
Chương trình chạy êm từ đầu tới cuối rồi in ra:

```text title=readonly
Tổng chi: 0 đồng
```

Trong khi cuốn sổ có 12 khoản, và bạn vừa mở file ra nhìn tận mắt.

Từ Realm 0 tới giờ, hễ có gì sai thì máy chỉ thẳng vào dòng cho bạn — bài 17
của Realm 0 dạy hẳn cách đọc lời chỉ ấy từ dòng cuối lên. Lần này máy không
chỉ gì cả. Nó không nghĩ là có chuyện gì sai.

Vậy hỏi ai?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
